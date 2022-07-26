#[allow(non_camel_case_types)]

pub mod auth0_perms {
   #[derive(Eq, PartialEq)]
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
   
   #[derive(Eq, PartialEq)]
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

      pub fn from_perms(perms: &Vec<String>) -> Vec<Self>  {
         let mut claims: Vec<Permissions> = Vec::new();
         
         for perm in perms {
            match perm.as_str() {
               "mailer:baseaccess" => claims.push(Permissions::MAILER_BASE_ACCESS),
               "mailer:webp:messages:read" => claims.push(Permissions::MAILER_WEBP_MSGS_READ),
               "mailer:webp:messages:delete" => claims.push(Permissions::MAILER_WEBP_MSGS_DEL),
               _ => {}
            }
         }
         claims
      }
   }
}


pub mod auth0_token_related {
   use serde_json::Value;
   use super::auth0_perms::*;

   pub struct AudienceIdentifier(pub String);

   pub struct AudienceUri(pub String);

   pub struct Auth0TokenFields {
      pub iss: Option<String>,
      pub sub: Option<String>,
      pub aud: Option<(AudienceIdentifier, AudienceUri)>,
      pub azp: Option<String>,
      pub exp: Option<u64>,
      pub iat: Option<u64>,
      pub scope: Option<Vec<String>>,
      pub permissions: Option<Vec<Permissions>>,
      pub raw_permissions: Option<Vec<String>>,
      pub is_claims: Option<Vec<IsClaims>>,
      pub role: Option<Vec<String>>,
   }

   impl Auth0TokenFields {
      pub fn from_serde_val(token: Value) -> Result<Self, ()> {
         Ok(Auth0TokenFields {
            iss: token.get("iss").and_then(|x| Some(x.to_string())),
            sub: token.get("sub").and_then(|x| Some(x.to_string())),
            aud: match token.get("aud").and_then(|x| x.as_array()) {
               Some(aud) => {
                  if aud[0].is_null() || aud[1].is_null() {
                     None
                  } else {
                     Some((AudienceIdentifier(aud[0].to_string()), AudienceUri(aud[1].to_string())))
                  }
               },
               _ => None
            },
            azp: token.get("azp").and_then(|x| Some(x.to_string())),
            exp: token.get("exp").and_then(|x| x.as_u64()),
            iat: token.get("iat").and_then(|x| x.as_u64()),
            scope: token.get("scope").and_then(|scope| scope.as_array()
               .and_then(|vec| Some(
                  vec.iter().map(|x| x.to_string()).collect()
               ))
            ),
            permissions: token.get("permissions").and_then(|perms| Some(
               Permissions::from_perms(&perms.to_string().split(" ").map(|x| x.to_string()).collect())
            )),
            raw_permissions: token.get("raw_permissions").and_then(|perms| Some(
               perms.to_string().split(" ").map(|val| val.to_string()).collect()
            )),
            is_claims: token.get("is_claims").and_then(|perms| Some(
               IsClaims::from_perms(&perms.to_string().split(" ").map(|x| x.to_string()).collect())
            )),
            role: token.get("role").and_then(|role| 
               Some(role.to_string().split(" ").map(|val| val.to_string()).collect())
            ),
         })
      }
   }
}