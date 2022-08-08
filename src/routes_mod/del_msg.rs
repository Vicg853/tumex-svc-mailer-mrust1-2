use std::str::FromStr;

use mongodb::bson::{doc, oid::ObjectId, Bson};
use rocket::{
  response::{content::RawJson, status::Custom},
  request::FromParam,
  http::{Status as HttpStatus}, 
  State,
  serde::json::serde_json::json, log::private::warn
};
use regex::Regex;

use crate::{
  auth::{
    auth0_token_related::PermCheckOpt,
    auth0_perm_claims::ScopePerm,
  },
  guards::Auth,
  mongo::MessageCmsDb
};

pub struct Ids(pub Vec<String>);

impl<'r> FromParam<'r> for Ids {
  type Error = RawJson<String>;

  fn from_param(param: &'r str) -> Result<Self, Self::Error> {
    let ids_rgx = Regex::new(r"^([a-zA-Z0-9]+)((,[a-zA-Z0-9]+)+$|$)").unwrap();

    let ids: Vec<String> = match ids_rgx.is_match(&param) {
      true => param.split(",").map(|id| id.to_string()).collect(),
      false => return Err(
        RawJson(json!({
        "error": "Invalid request. You must first specify a message id(s) and separate them by commas."
        }).to_string())
      )
    };

    Ok(Ids(ids))
  }
}

#[post("/del/<ids>")]
pub async fn del_msg(db: &State<MessageCmsDb>, auth: Auth, ids: Ids) -> Custom<RawJson<String>> {
  let req_perms = vec![ ScopePerm::MAILER_WEBP_MSGS_DEL ];

  if !auth.decoded_payload.check_perm(Some(PermCheckOpt::All(req_perms)), false, true) {
    return Custom(
      HttpStatus::new(403),
      RawJson(json!({
        "error": "Not authorized: insufficient permissions for this token"
      }).to_string())
    );
  }
  
  let oids_vec = ids.0.iter()
    .map(|id| ObjectId::from_str(id).to_owned());

  let mut oids = Vec::<ObjectId>::new();
  for oid in oids_vec {
    if oid.as_ref().is_err() {
      return Custom(
        HttpStatus::new(409),
        RawJson(json!({
          "error": "Error parsing message id(s)"
        }).to_string())
      );
    } else {
      oids.push(oid.unwrap());
    }
  }

  let oids = Bson::from_iter(oids);

  let delete_filter = doc! {
    "_id": {
      "$in": oids
    }
  };

  println!("{:?}", &delete_filter.to_string());
  match db.get_msg_col().delete_many(delete_filter, None).await {
    Ok(res) 
    if res.deleted_count.to_be_bytes() != ids.0.len().to_be_bytes() => Custom(
      HttpStatus::new(412),
      RawJson(json!({
        "error": "Some messages could not be deleted as they do not even exist."
      }).to_string())
    ),
    Ok(_) => Custom(
      HttpStatus::new(200), 
      RawJson(json!({
        "success": "Messages deleted successfully!"
      }).to_string())
    ),
    Err(err) => {
      warn!("Error deleting messages: {}", err);

      Custom(
        HttpStatus::new(500),
        RawJson(json!({
          "error": "Internal server error. Don't worry, this is our fault."
        }).to_string())
      )
    }
  }
}

#[post("/del")]
pub fn del_msg_no_id() -> Custom<RawJson<String>> {
  Custom(
    HttpStatus::new(400),
    RawJson(json!({
      "error": "Invalid request. You must first specify a message id."
    }).to_string())
  )
}