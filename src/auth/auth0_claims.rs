pub mod Auth0Perms {
   pub enum IsClaims {
      TUMEX,
      FRIENDS_NORMAL,
      FRIENDS_CLOSE,
      FRIENDS_BFF,
      FAMILY_FIRST,
      FAMILY_SECOND,
      FAMILY_THIRD,
      SUDO_LOW,
      SUDO_HIGH,
   }
   
   impl IsClaims {
      pub fn as_str(&self) -> &'static str {
         match self {
            IsClaims::TUMEX => "is:tumex",
            IsClaims::FRIENDS_NORMAL => "is:friends:normal",
            IsClaims::FRIENDS_CLOSE => "is:friends:close",
            IsClaims::FRIENDS_BFF => "is:friends:bff",
            IsClaims::FAMILY_FIRST => "is:family:first-deg",
            IsClaims::FAMILY_SECOND => "is:family:second-deg",
            IsClaims::FAMILY_THIRD => "is:family:third-deg",
            IsClaims::SUDO_LOW => "is:sudo:low",
            IsClaims::SUDO_HIGH => "is:sudo:high"
         }
      }

   }
   impl IsClaims {
      pub fn from_perms(perms: &Vec<String>) -> Vec<Self>  {
         let mut claims: Vec<IsClaims> = Vec::new();
         
         for perm in perms {
            match perm.as_str() {
               "is:tumex" => claims.push(IsClaims::TUMEX),
               "is:friends:normal" => claims.push(IsClaims::FRIENDS_NORMAL),
               "is:friends:close" => claims.push(IsClaims::FRIENDS_CLOSE),
               "is:friends:bff" => claims.push(IsClaims::FRIENDS_BFF),
               "is:family:first-deg" => claims.push(IsClaims::FAMILY_FIRST),
               "is:family:second-deg" => claims.push(IsClaims::FAMILY_SECOND),
               "is:family:third-deg" => claims.push(IsClaims::FAMILY_THIRD),
               "is:sudo:low" => claims.push(IsClaims::SUDO_LOW),
               "is:sudo:high" => claims.push(IsClaims::SUDO_HIGH),
               _ => {}
            }
         }
         claims
      }
   }
   
   pub enum Permissions {
      MAILER_BASE_ACCESS,
      MAILER_WEBP_MSGS_READ,
      MAILER_WEBP_MSGS_DEL
   }
   
   impl Permissions {
      pub fn as_str(&self) -> &'static str {
         match self {
            Permissions::MAILER_BASE_ACCESS => "mailer:baseaccess",
            Permissions::MAILER_WEBP_MSGS_READ => "mailer:webp:messages:read",
            Permissions::MAILER_WEBP_MSGS_DEL => "mailer:webp:messages:delete",
         }
      }
   }
}


pub mod Auth0TokenRelated {
   use std::clone::Clone;
   use serde_json::Value;

   #[derive(Clone)]
   pub struct AudienceIdentifier(pub String);

   #[derive(Clone)]
   pub struct AudienceUri(pub String);

   #[derive(Clone)]
   pub struct Auth0TokenFields {
      pub iss: String,
      pub sub: String,
      pub aud: Option<(AudienceIdentifier, AudienceUri)>,
      pub azp: String,
      pub exp: usize,
      pub iat: usize,
      pub scope: Vec<String>,
      pub permissions: Vec<String>,
      pub role: Option<Vec<String>>,
   }

   impl Auth0TokenFields {
      pub fn from_serde_val(token: &Value) -> Result<Self, ()> {
         Ok(Auth0TokenFields {
            iss: token["iss"].as_str().unwrap().to_string(),
            sub: token["sub"].as_str().unwrap().to_string(),
            aud: Some((
              AudienceIdentifier(token["aud"][0].as_str().unwrap().to_string()),
              AudienceUri(token["aud"][1].as_str().unwrap().to_string()),
            )),
            azp: token["azp"].as_str().unwrap().to_string(),
            exp: token["exp"].as_i64().unwrap() as usize,
            iat: token["iat"].as_u64().unwrap() as usize,
            scope: token["scope"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect(),
            permissions: token["permissions"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect(),
            role: match token["role"].is_null() {
               true => Some(token["role"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect()),
               false => None,
            }
         })
      }
   }
}