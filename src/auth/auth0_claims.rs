#[allow(non_camel_case_types)]

pub mod auth0_perm_claims {
   use std::string::ToString;

   pub trait NewAuth0Perms: Sized {
      fn from_perm_string(perms_string: &str) -> Option<Self>;
   }

   pub enum IsPerm {
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

   pub enum ScopePerm {
      MAILER_BASE_ACCESS,
      MAILER_WEBP_MSGS_READ,
      MAILER_WEBP_MSGS_DEL
   }

   impl NewAuth0Perms for IsPerm {
      fn from_perm_string(perms_string: &str) -> Option<Self> {
         match perms_string {
            "is:tumex" => Some(IsPerm::TUMEX),
            "is:friends:normal" => Some(IsPerm::FRIENDS_NORMAL),
            "is:friends:close" => Some(IsPerm::FRIENDS_CLOSE),
            "is:friends:bff" => Some(IsPerm::FRIENDS_BFF),
            "is:family:first-deg" => Some(IsPerm::FAMILY_FIRST),
            "is:family:second-deg" => Some(IsPerm::FAMILY_SECOND),
            "is:family:third-deg" => Some(IsPerm::FAMILY_THIRD),
            "is:sudo:low" => Some(IsPerm::SUDO_LOW),
            "is:sudo:high" => Some(IsPerm::SUDO_HIGH),
            _ => None,
         }
      }
   }

   impl NewAuth0Perms for ScopePerm {
      fn from_perm_string(perms_string: &str) -> Option<Self> {
         match perms_string {
            "mailer:baseaccess" => Some(ScopePerm::MAILER_BASE_ACCESS),
            "mailer:webp:messages:read" => Some(ScopePerm::MAILER_WEBP_MSGS_READ),
            "mailer:webp:messages:delete" => Some(ScopePerm::MAILER_WEBP_MSGS_DEL),
            _ => None,
         }
      }
   }

   impl ToString for IsPerm {
      fn to_string(&self) -> String {
         match self {
            IsPerm::TUMEX => "is:tumex".to_string(),
            IsPerm::FRIENDS_NORMAL => "is:friends:normal".to_string(),
            IsPerm::FRIENDS_CLOSE => "is:friends:close".to_string(),
            IsPerm::FRIENDS_BFF => "is:friends:bff".to_string(),
            IsPerm::FAMILY_FIRST => "is:family:first-deg".to_string(),
            IsPerm::FAMILY_SECOND => "is:family:second-deg".to_string(),
            IsPerm::FAMILY_THIRD => "is:family:third-deg".to_string(),
            IsPerm::SUDO_LOW => "is:sudo:low".to_string(),
            IsPerm::SUDO_HIGH => "is:sudo:high".to_string(),
         }
      }
   }

   impl ToString for ScopePerm {
      fn to_string(&self) -> String {
         match self {
            ScopePerm::MAILER_BASE_ACCESS => "mailer:baseaccess".to_string(),
            ScopePerm::MAILER_WEBP_MSGS_READ => "mailer:webp:messages:read".to_string(),
            ScopePerm::MAILER_WEBP_MSGS_DEL => "mailer:webp:messages:delete".to_string(),
         }
      }
   }

   impl IsPerm {
      #[allow(dead_code)]
      pub fn as_str(&self) -> &str {
         match self {
            IsPerm::TUMEX => "is:tumex",
            IsPerm::FRIENDS_NORMAL => "is:friends:normal",
            IsPerm::FRIENDS_CLOSE => "is:friends:close",
            IsPerm::FRIENDS_BFF => "is:friends:bff",
            IsPerm::FAMILY_FIRST => "is:family:first-deg",
            IsPerm::FAMILY_SECOND => "is:family:second-deg",
            IsPerm::FAMILY_THIRD => "is:family:third-deg",
            IsPerm::SUDO_LOW => "is:sudo:low",
            IsPerm::SUDO_HIGH => "is:sudo:high",
         }
      }
   }

   impl ScopePerm {
      #[allow(dead_code)]
      pub fn as_str(&self) -> &str {
         match self {
            ScopePerm::MAILER_BASE_ACCESS => "mailer:baseaccess",
            ScopePerm::MAILER_WEBP_MSGS_READ => "mailer:webp:messages:read",
            ScopePerm::MAILER_WEBP_MSGS_DEL => "mailer:webp:messages:delete",
         }
      }
   }

   impl NewAuth0Perms for IsPermVec {
      fn from_perm_string(perms_string: &str) -> Option<Self> {
         let mut perms = Vec::new();
         for perm in perms_string.split(',') {
            if let Some(perm) = IsPerm::from_perm_string(perm) {
               perms.push(perm);
            }
         }
         if perms.is_empty() {
            None
         } else {
            Some(IsPermVec(perms))
         }
      }
   }

   impl NewAuth0Perms for ScopePermVec {
      fn from_perm_string(perms_string: &str) -> Option<Self> {
         let mut perms = Vec::new();
         for perm in perms_string.split(',') {
            if let Some(perm) = ScopePerm::from_perm_string(perm) {
               perms.push(perm);
            }
         }
         if perms.is_empty() {
            None
         } else {
            Some(ScopePermVec(perms))
         }
      }
   }

   pub struct IsPermVec(pub Vec<IsPerm>);
   pub struct ScopePermVec(pub Vec<ScopePerm>);

   impl ToString for IsPermVec {
      fn to_string(&self) -> String {
         self.0.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",")
      }
   }

   impl ToString for ScopePermVec {
      fn to_string(&self) -> String {
         self.0.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",")
      }
   }

   impl IsPermVec {
      #[allow(dead_code)]
      pub fn as_string_vec(&self) -> Vec<String> {
         self.0.iter().map(|p| p.to_string()).collect::<Vec<String>>()
      }
   }

   impl ScopePermVec {
      #[allow(dead_code)]
      pub fn as_string_vec(&self) -> Vec<String> {
         self.0.iter().map(|p| p.to_string()).collect::<Vec<String>>()
      }
   }
}

pub mod auth0_token_related {
   use serde_json::Value;
   use super::auth0_perm_claims::{
      NewAuth0Perms,
      IsPermVec,
      ScopePermVec
   };

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
      pub permissions: Option<ScopePermVec>,
      pub raw_permissions: Option<Vec<String>>,
      pub is_claims: Option<IsPermVec>,
      pub role: Option<Vec<String>>,
   }

   pub enum PermCheckOpt<T> {
      All(Vec<T>),
      Any(Vec<T>),
      None(Vec<T>)
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
            permissions: token.get("permissions").and_then(|perms| {
               let iterable = perms.as_array();
               let mut res: Option<ScopePermVec> = None;                  
            
               if iterable.is_none() {
                  let iterable = perms.to_string()
                     .split(" ")
                     .collect::<Vec<&str>>()
                     .join(",");

                  if iterable.len() > 0 {
                     res = ScopePermVec::from_perm_string(&iterable);
                  }
               } else {
                  res = ScopePermVec::from_perm_string(
                     &iterable.unwrap()
                        .iter()
                        .map(|perm| perm.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                  );
               }
             
               res
            }),
            raw_permissions: token.get("permissions").and_then(|perms| {
               let iterable = perms.as_array();
               let mut res = None;
              
               if iterable.is_none() {
                  let iterable: Vec<String> = perms.to_string()
                     .split(" ")
                     .map(|perm| perm.to_string())
                     .collect();

                  if iterable.len() > 0 {
                     res = Some(iterable);
                  }
               } else {
                  res = Some(
                     iterable.unwrap()
                     .iter()
                     .map(|perm| perm.as_str())
                     .filter(|perm| perm.is_some())
                     .map(|perm| perm.unwrap().to_string())
                     .collect()
                  );
               }
               
               res
            }),
            is_claims: token.get("permissions").and_then(|perms| {
               let iterable = perms.as_array();
               let mut res = None;

               if iterable.is_none() {
                  let iterable = perms.to_string()
                     .split(" ")
                     .collect::<Vec<&str>>()
                     .join(",");

                  if iterable.len() > 0 {
                     res = IsPermVec::from_perm_string(&iterable);
                  }
               } else {
                  res = IsPermVec::from_perm_string(
                     &iterable.unwrap()
                        .iter()
                        .map(|perm| perm.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                  );
               }

               res
            }),
            role: token.get("role").and_then(|role| 
               Some(role.to_string().split(" ").map(|val| val.to_string()).collect())
            ),
         })
      }

      pub fn check_perm(&self, req_perm: Option<PermCheckOpt<impl NewAuth0Perms + ToString>>, check_min: impl Into<Option<bool>>, check_tumex: impl Into<Option<bool>>) -> bool {
         let check_min = check_min.into().unwrap_or(false);
         let check_tumex = check_tumex.into().unwrap_or(true);

         const MIN_PERM: &str = "mailer:baseaccess";
         #[allow(non_upper_case_globals)]
         const IS_TUMEX: &str = "is:tumex";

         if self.raw_permissions.as_ref().is_none() {
            return false;
         }
         let raw_perms = self.raw_permissions.as_ref().unwrap();

         if raw_perms.contains(&IS_TUMEX.to_string()) && check_tumex {
            return true;
         }
         if !raw_perms.contains(&MIN_PERM.to_string()) && check_min {
            return false;
         }
         
         match req_perm {
            None => true,
            Some(PermCheckOpt::All(req_perms)) => {
               for perm in req_perms {
                  let perm = perm.to_string();
                  if !raw_perms.contains(&perm) {
                     return false;
                  }
               }

               return true;
            },
            Some(PermCheckOpt::Any(req_perms) ) => {
               for perm in req_perms {
                  let perm = perm.to_string();
                  if raw_perms.contains(&perm) {
                     return true;
                  }
               }

               return false;
            },
            Some(PermCheckOpt::None(req_perms) ) => {
               for perm in req_perms {
                  let perm = perm.to_string();
                  if raw_perms.contains(&perm) {
                     return false;
                  }
               }

               return true;
            }
         }
      }
   }
}
