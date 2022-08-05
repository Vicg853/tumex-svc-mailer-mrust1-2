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
   auth::auth0_perms::{PermCheckOptions, check_perms, Permissions},
};

#[get("/get/<id>")]
pub async fn get_msg(db: &State<MessageCmsDb>, auth: Auth, id: String) -> Custom<RawJson<String>> {
   let req_perms = vec![
      Permissions::MAILER_WEBP_MSGS_READ.as_string(),
   ];

   if auth.decoded_payload.permissions.is_none() {
      return Custom(
         HttpStatus::new(403),
         RawJson(json!({
            "error": "Not authorized: no permissions for this token"
         }).to_string())
      )
   }

   if !check_perms(
   auth.decoded_payload.raw_permissions.as_ref().unwrap(), 
   Some(PermCheckOptions::All(&req_perms.iter().map(|p| p.as_str()).collect())), 
   false, true) {
      return Custom(
         HttpStatus::new(403),
         RawJson(json!({
            "error": "Unauthorized: You do not meet the requirements for to access this resource."
         }).to_string())
      )
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
