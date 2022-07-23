use rocket::{
   response::{content, status},
   http::{Status as HttpStatus}, 
   State,
   serde::json::serde_json::json
};

use crate::{
   mongo::MessageCmsDb,
   guards::{Auth}
};

#[get("/")]
pub async fn get_msgs(cms_db: &State<MessageCmsDb>, auth: Auth) -> status::Custom<content::RawJson<String>> { 
   status::Custom(
      HttpStatus::new(200), 
      content::RawJson(json!({
         "message": "Hello World"
      }).to_string())
   )
}