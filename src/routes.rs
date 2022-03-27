use rocket::http::RawStr;
use rocket::request::Form;
use rocket::request::FromFormValue;
use rocket::routes;
use regex::Regex;

struct MailAddress(String);

impl<'v> FromFormValue<'v> for MailAddress {
   type Error = &'v RawStr;

   fn from_form_value(form_value: &'v RawStr) -> Result<MailAddress, &'v RawStr> {
      let mail_Regex: Regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
      
      match form_value.parse::<String>() {
          Ok(mail) if mail.len() > 0 && mail_Regex.is_match(&mail) => Ok(MailAddress(mail)),
          _ => Err(form_value)
      }
   }
}

#[derive(FromForm)]
struct MailMessage {
   from: MailAddress,
   name: String,
   subject: String,
   message: String
}

#[post("/send", data = "<message>", format = "application/json")]
pub fn send_message(message: Form<MailMessage>) -> String {
   let message = message.into_inner();
   let message = format!("From: {}\nName: {}\nSubject: {}\nMessage: {}", message.from.0, message.name, message.subject, message.message);

   return message.to_string();
}