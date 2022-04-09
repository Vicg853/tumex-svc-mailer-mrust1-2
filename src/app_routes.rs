use rocket::response::status;
use rocket::serde::{Serialize, Deserialize};
use rocket::form::{FromForm, Form};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MailMessage {
   pub from: String,
   pub name: String,
   pub subject: String,
   pub message: String
}

#[post("/send", data = "<form>")]
pub fn send_message(form: Form<MailMessage>) -> status::Accepted<String> {
   let message = form.into_inner();
   let response = format!(
      "From: {}\nName: {}\nSubject: {}\nMessage: {}", 
      message.from, 
      message.name, 
      message.subject, 
      message.message);

   status::Accepted(Some(response))
}