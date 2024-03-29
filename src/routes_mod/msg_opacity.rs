use std::str::FromStr;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::{
   response::{status::Custom , content::RawJson},
   http::Status as HttpStatus,
   State
};
use serde_json::json;

use crate::{
   auth::{
      auth0_token_related::PermCheckOpt,
      auth0_perm_claims::ScopePerm,
    },
   mongo::MessageCmsDb, 
   guards::Auth,
};

#[post("/toggle?<toggle_type>&<id>&<value>")]
pub async fn toggle_read_archive(db: &State<MessageCmsDb>, auth: Auth, 
   toggle_type: Option<String>, id: Option<String>, value: Option<bool>
) -> Custom<RawJson<String>> {
   if toggle_type.is_none() || id.is_none() || value.is_none() {
      return Custom(
         HttpStatus::new(400),
         RawJson(json!({
            "error": "Invalid request. You must first specify a toggle type (archive or read), a message id, and then a set value (true or false)."
         }).to_string())
      );
   }
   let toggle_type = toggle_type.unwrap();
   let id = id.unwrap();
   let value = value.unwrap();

   let mut req_perms = Vec::<ScopePerm>::new();

   if toggle_type.eq("archive") {
      req_perms.push(ScopePerm::MAILER_WEBP_MSGS_READ);
   } else if toggle_type.eq("read") {
      req_perms.push(ScopePerm::MAILER_WEBP_MSGS_READ);
   } else {
      return Custom(
         HttpStatus::new(400),
         RawJson(json!({
            "error": "Invalid request. Toggle type must be: 'archive' or 'read'."
         }).to_string())
      );
   }

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
   
   let update_data = match toggle_type.as_str() {
      "archive" => {
         doc! { "$set": { "archived": value } }
      },
      "read" => {
         doc! { "$set": { "read": value } }
      },
      _ => {
         return Custom(
            HttpStatus::new(400),
            RawJson(json!({
               "error": "Invalid request. Toggle type must be: 'archive' or 'read'."
            }).to_string())
         );
      }
   };

   let query = doc! { "_id": { "$eq": msg_oid } };
   match db.get_msg_col().update_one(query, update_data, None).await {
      Ok(_) => {
         Custom(
            HttpStatus::new(200),
            RawJson(json!({
               "success": format!(
                  "Toggled message {} status successfully!", 
                  if toggle_type.eq("archive") { "archived" } else { "read" }
               )
            }).to_string())
         )
      },
      Err(e) => {
         warn!("Error toggling message read status: {}", e);
         Custom(
            HttpStatus::new(500),
            RawJson(json!({
               "error": format!(
                  "Error toggling message {} status",
                  if toggle_type.eq("archive") { "archived" } else { "read" }
               )
            }).to_string())
         )
      }
   }
}