use rocket::{
  response::{content::RawJson, status::Custom},
  http::{Status as HttpStatus}, 
  State,
  serde::json::serde_json::json
};
use crate::mongo::{MessageCmsDb, ConnCheck};

#[get("/")]
pub async fn check_health(cms_db: &State<MessageCmsDb>) -> Custom<RawJson<String>> {
  info!("Health check requested!...");
  
  match cms_db.check_conn().await {
    ConnCheck::Ok => Custom(
      HttpStatus::new(200), 
      RawJson(json!({
         "status": {
            "server": {
                "status_msg": "OK",
                "is_ok": true
            },
            "db": {
                "status_msg": "OK",
                "is_ok": true
            }
         }
      }).to_string())
    ),
    ConnCheck::Issue(err) => {
      error!("Failed to connect to CMS DB Cluster: {}", err);
      Custom(
        HttpStatus::new(200), 
        RawJson(json!({
           "status": {
              "server": {
                  "status_msg": "OK",
                  "is_ok": true
              },
              "db": {
                  "status_msg": "There seems to be an issue between the server's DB connection.",
                  "is_ok": false
              }
           }
        }).to_string())
      )
    }
  }
}