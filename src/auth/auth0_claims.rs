#[allow(non_camel_case_types)]

pub mod auth0_perms {
   pub trait ClaimsToEnumConstructors where Self: Sized {
      fn from_perms(_perms: &Vec<String>) -> Vec<Self> { Vec::new() }
   }

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
       pub fn as_string(&self) -> String {
         match self {
            IsClaims::TUMEX => "is:tumex".to_string(),
            IsClaims::FRIENDS_NORMAL => "is:friends:normal".to_string(),
            IsClaims::FRIENDS_CLOSE => "is:friends:close".to_string(),
            IsClaims::FRIENDS_BFF => "is:friends:bff".to_string(),
            IsClaims::FAMILY_FIRST => "is:family:first-deg".to_string(),
            IsClaims::FAMILY_SECOND => "is:family:second-deg".to_string(),
            IsClaims::FAMILY_THIRD => "is:family:third-deg".to_string(),
            IsClaims::SUDO_LOW => "is:sudo:low".to_string(),
            IsClaims::SUDO_HIGH => "is:sudo:high".to_string()
         }
      }

   }
   impl ClaimsToEnumConstructors for IsClaims {
      fn from_perms(perms: &Vec<String>) -> Vec<Self>  {
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
      pub fn as_string(&self) -> String {
         match self {
            Permissions::MAILER_BASE_ACCESS => "mailer:baseaccess".to_string(),
            Permissions::MAILER_WEBP_MSGS_READ => "mailer:webp:messages:read".to_string(),
            Permissions::MAILER_WEBP_MSGS_DEL => "mailer:webp:messages:delete".to_string(),
         }
      }
   }
   
   impl ClaimsToEnumConstructors for Permissions {
      fn from_perms(perms: &Vec<String>) -> Vec<Self>  {
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

   pub enum PermCheckOptions {
      AtLeastOne(Vec<String>),
      All(Vec<String>),
      None(Vec<String>)
   }

   pub fn check_perms(
      usr_perms: &Vec<String>, 
      req_perms: Option<&PermCheckOptions>, 
      check_min: bool, 
      check_tumex: bool) -> bool {
      let min_perm = "mailer:baseaccess";
      let is_tumex = "is:tumex";
      let mut min_perms_check = false;
      
      if check_tumex || check_min {
         for perm in usr_perms {
            if perm == min_perm && check_min {
               min_perms_check = true;
            }
            if perm == is_tumex && check_tumex {
               return true;
            }
         }
      }

      match req_perms {
         Some(req_perms ) 
         if let PermCheckOptions::All(req_perms) = req_perms => {
            for perm in req_perms {
               if usr_perms.contains(perm) {
                  return false;
               }
            }

            return true;
         },
         Some(req_perms ) 
         if let PermCheckOptions::AtLeastOne(req_perms) = req_perms => {
            for perm in req_perms {
               if usr_perms.contains(perm) {
                  return true;
               }
            }

            return false;
         },
         Some(req_perms ) 
         if let PermCheckOptions::None(req_perms) = req_perms => {
            for perm in req_perms {
               if usr_perms.contains(perm) {
                  return false;
               }
            }

            return true;
         },
         _ => min_perms_check
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