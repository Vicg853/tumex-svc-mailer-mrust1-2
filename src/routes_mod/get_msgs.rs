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
use msgs_filter_params::*;
use get_msgs_filtering::{get_filter, FilterErr};

mod get_msgs_filtering {
   use std::str::FromStr;
   use mongodb::bson::{Document, doc, DateTime as BsonDateTime, Bson};
   use chrono::{DateTime, Utc};
   use super::msgs_filter_params::*;
   
   #[derive(Debug)]
   pub struct FilterErr {
      pub msg: String,
      pub unexpected: bool
   }

   pub fn get_filter(read_filter: Option<ReadFilter>, date_filter: Option<DateFilter>, archived_filter: Option<ArchivedFilter>, sender_filter: Option<SenderFilter>) -> Result<Document, FilterErr> {
      let mut mongo_query_filters = Document::new();

      if date_filter.is_some() {
         let date_filter = date_filter.unwrap();
         
         if (date_filter.before.is_some() && date_filter.after.is_some()) 
         || date_filter.within.is_some() && (date_filter.before.is_some() || date_filter.after.is_some()) {
            return Err(FilterErr {
               msg: "Only one of \"before\" or \"after\" or \"within\" can be specified".to_string(),
               unexpected: false
            });
         }

         let before_date_filter = date_filter.before.and_then(
            |b| DateTime::from_str(&b)
               .map_or_else(|e| {
                  warn!("Failed parsing \"before\" date filter. Error: {:?}", e);
                  Some(Err(FilterErr { msg: "Failed parsing \"before\" date filter.".to_owned(), unexpected: true }))
               }, |d: DateTime<Utc>| Some(Ok(d)))
         );

         let after_date_filter = date_filter.after.and_then(
            |b| DateTime::from_str(&b)
               .map_or_else(|e| {
                  warn!("Failed parsing \"after\" date filter. Error: {:?}", e);
                  Some(Err(FilterErr { msg: "Failed parsing \"after\" date filter.".to_owned(), unexpected: true }))
               }, |d: DateTime<Utc>| Some(Ok(d)))
         );

         let within_date_filter = date_filter.within.and_then(
            |b| { 
               let i_0 = DateTime::from_str(&b.0)
               .map_or_else(|e| Err(e), |d: DateTime<Utc>| Ok(d));
               let i_1 = DateTime::from_str(&b.1)
               .map_or_else(|e| Err(e), |d: DateTime<Utc>| Ok(d));
               if i_0.is_err() || i_1.is_err() {
                  warn!("Failed parsing \"within\" date filter. \n Start {:?} \n End: {:?}", i_0.is_err(), i_1.is_err());
                  Some(Err(FilterErr { msg: "Failed parsing \"within\" date filter.".to_owned(), unexpected: true }))
               } else {
                  Some(Ok((i_0.unwrap(), i_1.unwrap())))
               }
            }
         );

         match (before_date_filter, after_date_filter, within_date_filter) {
            (Some(Err(e)), _, _) => return Err(e),
            (_, Some(Err(e)), _) => return Err(e),
            (_, _, Some(Err(e))) => return Err(e),
            (Some(Ok(b)), _, _) => {
               println!("{}", b.to_string());
               mongo_query_filters.insert("createdAt", doc!{ "$lt": BsonDateTime::from_chrono(b) }); 
            },
            (_, Some(Ok(b)), _) => {
               println!("{}", b.to_string());
               mongo_query_filters.insert("createdAt", doc!{ "$gt": BsonDateTime::from_chrono(b) });
            },
            (_, _, Some(Ok((b, e)))) => {
               println!("{}", b.to_string());
               mongo_query_filters.insert("createdAt", doc!{ "$gte": BsonDateTime::from_chrono(b), "$lte": BsonDateTime::from_chrono(e) });
            },
            (_, _, _) => {}
         }
      }

      if read_filter.is_some() {
         let read_filter = read_filter.unwrap();
         mongo_query_filters.insert("read", doc! { "$eq": read_filter.0 });
      }
      
      if archived_filter.is_some() {
         let archived_filter = archived_filter.unwrap();
         mongo_query_filters.insert("read", doc! { "$eq": archived_filter.0 });
      }

      if sender_filter.is_some() {
         let sender_filter = sender_filter.unwrap();
         let bson_vec = Bson::from(sender_filter.0); 

         mongo_query_filters.insert("from", doc! { "$in": bson_vec });
      }

      Ok(mongo_query_filters)
   }
}

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

   let filter = get_filter(read, date, archived, sender);
   if filter.is_err() {
      match filter.unwrap_err() {
         FilterErr { msg, unexpected: true } => {
            return Custom(
               HttpStatus::new(500), 
               RawJson(json!({
                  "error": msg
               }).to_string())
            );
         }
         FilterErr { msg, unexpected: false } => {
            return Custom(
               HttpStatus::new(400), 
               RawJson(json!({
                  "error": msg
               }).to_string())
            );
         }
      }
   }
   let filter = filter.unwrap();

   match cms_db.get_msg_col().find(Some(filter), None).await {
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
