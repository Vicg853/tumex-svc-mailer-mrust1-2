use chrono::{Utc};
use regex::Regex;
use mongodb::{bson::{doc, DateTime}};
use rocket::{
    response::{content, status},
    http::{Status as HttpStatus}, 
    State,
    warn,
    serde::{Deserialize, json::{Json, serde_json}}
};

use crate::{
    MessageCmsDb,
    models::message::Message
};

#[derive(Deserialize)]
pub struct NewMessagePayload {
   pub from: String,
   pub name: String,
   pub subject: String,
   pub message: String
}

struct ValidError<'c> {
    message: &'c str,
    code: u16
}


impl<'c> NewMessagePayload {
    fn is_valid(&self) -> Result<(), ValidError<'c>> {
        const EMAIL_RGX: &str = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9]+(\.[a-zA-Z0-9-]{0,61})+$";

        if Regex::new(EMAIL_RGX).unwrap().is_match(&self.from) {
            Ok(())
        } else {
            Err(ValidError {
                message: "Invalid email address",
                code: 400
            })
        }
    }
}

//TODO Add something to prevent xss and other attacks (e.g.: "sanitize-html")
//TODO Find a way to prevent sql injection scripts
//TODO + other sec shit

#[post("/send", format = "application/json", data = "<message>")]
pub async fn send_message(cms_db: &State<MessageCmsDb>, message: Json<NewMessagePayload>) -> status::Custom<content::RawJson<String>> {
    let message = message.into_inner();
    let validated = message.is_valid();

    if validated.is_err() {
        let valid_err = validated.unwrap_err();
        
        let json_response = serde_json::json!({
            "message": valid_err.message 
        });
    
        return status::Custom(
            HttpStatus::new(valid_err.code), 
            content::RawJson(json_response.to_string()))
    }
    
    let msg_doc = Message {
        id: None,
        created_at: Some(DateTime::from(Utc::now())),
        from: message.from,
        name: message.name,
        subject: message.subject,
        message: message.message,
        read: false
    };
    
    match cms_db.get_msg_col().insert_one(msg_doc, None).await {
        Ok(_) => status::Custom(
            HttpStatus::new(200), 
            content::RawJson(String::from("Your message has been sent!"))),
        Err(err) => {
            warn!("Failed to insert new message into CMS MSG DB: {}", err);
            status::Custom(
                HttpStatus::new(500), 
                content::RawJson(String::from("Sorry, something went wrong when sending your message. Please try again.")))
        }
    }
}