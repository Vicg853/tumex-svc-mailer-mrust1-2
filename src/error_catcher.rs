use rocket::{
   response::{
      content::RawJson, 
      status::Custom
   }, 
   http::Status as HttpStatus
};

#[catch(404)]
pub fn not_found() -> Custom<RawJson<String>> {
   Custom(
      HttpStatus::new(404),
      RawJson(
         r#"{
            "error": "Sorry, this route could not be found", 
            "http_cat": "https://http.cat/404"
         }"#.to_string()
      )
   )
}

#[catch(500)]
pub fn internal_server_error() -> Custom<RawJson<String>> {
   Custom(
      HttpStatus::new(500),
      RawJson(
         r#"{
            "error": "Sorry, something went wrong. Don't worry this is our fault: probably the dinosaurs are chewing on some cables again...",
            "http_cat": "https://http.cat/500"
         }"#.to_string()
      )
   )
}

#[catch(401)]
pub fn unauthorized() -> Custom<RawJson<String>> {
   Custom(
      HttpStatus::new(401),
      RawJson(
         r#"{
            "error": "You aren't authorized to access this resource!",
            "http_cat": "https://http.cat/401"
         }"#.to_string()
      )
   )
}

#[catch(403)]
pub fn forbidden() -> Custom<RawJson<String>> {
   Custom(
      HttpStatus::new(403),
      RawJson(
         r#"{
            "error": "You do not meet the required authorization levels to access this resource!",
            "http_cat": "https://http.cat/403"
         }"#.to_string()
      )
   )
}

#[catch(420)]
pub fn enhance_calm() -> Custom<RawJson<String>> {
   Custom(
      HttpStatus::new(429),
      RawJson(
         r#"{
            "error": "Enhance your calm bro. A.k.a.: You're being rate limited!",
            "http_cat": "https://http.cat/429"
         }"#.to_string()
      )
   )
}
#[catch(429)]
pub fn enhance_calm2() -> Custom<RawJson<String>> {
   Custom(
      HttpStatus::new(429),
      RawJson(
         r#"{
            "error": "Enhance your calm bro. A.k.a.: You're being rate limited!",
            "http_cat": "https://http.cat/429"
         }"#.to_string()
      )
   )
}

#[catch(400)]
pub fn bad_request() -> Custom<RawJson<String>> {
   Custom(
      HttpStatus::new(400),
      RawJson(
         r#"{
            "error": "There something wrong with your request!",
            "http_cat": "https://http.cat/400"
         }"#.to_string()
      )
   )
}