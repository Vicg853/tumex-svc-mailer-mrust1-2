use rocket::{
   response::{content::RawJson, status::Custom},
   http::{Status as HttpStatus}, 
   State,
   serde::json::serde_json::json
};
use serde_json::Value as SerdeVal;

use crate::{
   mongo::MessageCmsDb,
   guards::{Auth},
   auth::auth0_perms::{check_perms, Permissions, PermCheckOptions}
};
mod msgs_filter_params {
   #[derive(FromForm)]
   pub struct ReadFilter(pub bool);

   #[derive(FromForm)]
   pub struct SenderFilter(pub Vec<String>);
   
   #[derive(FromForm)]
   pub struct ArchivedFilter(pub bool);
   
   #[derive(FromForm)]
   pub struct DateFilter {
      pub before: Option<String>,
      pub after: Option<String>,
      pub within: Option<(String, String)>
   }
}

#[get("/?<read>&<date>&<archived>&<sender>")]
pub async fn get_msgs(cms_db: &State<MessageCmsDb>, auth: Auth, 
   read: Option<ReadFilter>, date: Option<DateFilter>, archived: Option<ArchivedFilter>,
   sender: Option<SenderFilter>
) -> Custom<RawJson<String>> {
   let perms = auth.decoded_payload.raw_permissions
      .unwrap_or(vec![]);
   let req_perms = vec![Permissions::MAILER_WEBP_MSGS_READ.as_string()];

   if !check_perms(
      &perms, 
      Some(PermCheckOptions::All(&req_perms.iter().map(|p| p.as_str()).collect())),
      false, false) {
      return Custom(
         HttpStatus::new(403), 
         RawJson(json!({
            "error": "Yoy don't meet the required permissions!"
         }).to_string())
      );
   }

   match cms_db.get_msg_col().find(None, None).await {
      Err(err) => {
         warn!("Failed retrieving messages. Error: {:?}", err);

         Custom(
            HttpStatus::new(500), 
            RawJson(json!({
               "error": "Failed retrieving messages. Don't worry this is a fault on our side!"
            }).to_string())
         )
      },
      Ok(mut cursor) => async {
         let mut msgs_res: Vec<SerdeVal> = Vec::new();

         loop {
            let doc = cursor.advance().await;
            if doc.is_err() {
               warn!("Failed to retrieve a doc from MongoDB. Error: {:?}", doc.err().unwrap());
               continue;
            }
            if !doc.unwrap() {
               break;
            }

            match cursor.deserialize_current() {
               Err(err) => {
                  warn!("Failed to deserialize a doc from MongoDB. Error: {:?}", err);
                  continue;
               },
               Ok(msg) => msgs_res.push(json!({
                  "id": msg.id.unwrap().to_string(),
                  "subject": msg.subject,
                  "body": msg.message,
                  "sender": msg.name,
                  "email": msg.from,
                  "sent_at": msg.created_at.unwrap().to_chrono().to_rfc3339().to_string(),
               }))
            }
         }

         Custom(
            HttpStatus::new(200), 
            RawJson(json!({
               "msgs": msgs_res
            }).to_string())
         )
      }.await
   }
}
