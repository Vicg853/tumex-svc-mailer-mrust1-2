use std::str::FromStr;

use serde_json::json;
use rocket::{
   response::{status::Custom, content::RawJson},
   http::Status as HttpStatus,
   State, log::private::warn
};
use mongodb::bson::{doc, oid::ObjectId};

use crate::{
   MessageCmsDb,
   guards::Auth,
   auth::{
      auth0_token_related::PermCheckOpt,
      auth0_perm_claims::ScopePerm,
   },
};

#[get("/get/<id>")]
pub async fn get_msg(db: &State<MessageCmsDb>, auth: Auth, id: String) -> Custom<RawJson<String>> {
   let req_perms = vec![ ScopePerm::MAILER_WEBP_MSGS_READ ];

  if !auth.decoded_payload.check_perm(Some(PermCheckOpt::All(req_perms)), false, true) {
    return Custom(
      HttpStatus::new(403),
      RawJson(json!({
        "error": "Not authorized: insufficient permissions for this token"
      }).to_string())
    );
  }

   let msg_oid = ObjectId::from_str(&id).or(Err(Custom(
      HttpStatus::new(400),
      RawJson(json!({
         "error": "Invalid message id"
      }).to_string())
   )));

   if msg_oid.is_err() {
      return msg_oid.unwrap_err();
   }
   let msg_oid = msg_oid.unwrap();

   let filter = doc! { "_id": { "$eq": msg_oid } };
   let update_data = doc! { "$set": { "read": true } };
   match db.get_msg_col().find_one_and_update(filter, update_data, None).await {
      Ok(Some(msg)) => {
         
         let msg_data = json!({
            "id": msg.id.unwrap().to_string(),
            "subject": msg.subject,
            "message": msg.message,
            "from": msg.from,
            "name": msg.name,
            "read": true,
            "archived": msg.archived,
         }).to_string();

         Custom(
            HttpStatus::new(200),
            RawJson(msg_data)
         )
      },
      Ok(None) => Custom(
         HttpStatus::NotFound,
         RawJson(json!({
            "erro": "Message counldn't be found!"
         }).to_string())         
      ),
      Err(e) => {
         warn!("There was an error while fetching message from MongoDB! Err: {:?}", e);
         Custom(
            HttpStatus::new(500),
            RawJson(json!({
               "error": "There was an error while retrieving the message! Don't worry this is our fault!"
            }).to_string())
         )
      }
   }
}

#[get("/get")]
pub fn get_msg_no_id() -> Custom<RawJson<String>> {
   Custom(
      HttpStatus::new(400),
      RawJson(
         json!({
            "message": "No id was provided"
         }).to_string()
      )
   )
}
