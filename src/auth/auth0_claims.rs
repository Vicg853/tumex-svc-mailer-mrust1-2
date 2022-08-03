#[allow(non_camel_case_types)]

pub mod auth0_perms {
    use std::ops::Deref;

   pub trait ClaimsToEnumConstructors where Self: Sized {
      fn from_perms(_perms: &Vec<String>) -> Vec<Self> { Vec::new() }
   }

   #[derive(Eq, PartialEq, Debug)]
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
            IsClaims::TUMEX => "is:tumex".to_owned(),
            IsClaims::FRIENDS_NORMAL => "is:friends:normal".to_owned(),
            IsClaims::FRIENDS_CLOSE => "is:friends:close".to_owned(),
            IsClaims::FRIENDS_BFF => "is:friends:bff".to_owned(),
            IsClaims::FAMILY_FIRST => "is:family:first-deg".to_owned(),
            IsClaims::FAMILY_SECOND => "is:family:second-deg".to_owned(),
            IsClaims::FAMILY_THIRD => "is:family:third-deg".to_owned(),
            IsClaims::SUDO_LOW => "is:sudo:low".to_owned(),
            IsClaims::SUDO_HIGH => "is:sudo:high".to_owned()
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
   
   #[derive(Eq, PartialEq, Debug)]
   pub enum Permissions {
      MAILER_BASE_ACCESS,
      MAILER_WEBP_MSGS_READ,
      MAILER_WEBP_MSGS_DEL
   }
   
   impl Permissions {
      pub fn as_string(&self) -> String {
         match self {
            Permissions::MAILER_BASE_ACCESS => "mailers:baseaccess".to_owned(),
            Permissions::MAILER_WEBP_MSGS_READ => "mailer:webp:messages:read".to_owned(),
            Permissions::MAILER_WEBP_MSGS_DEL => "mailer:webp:messages:delete".to_owned()
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

   pub enum PermCheckOptions<'a>{
      AtLeastOne(&'a Vec<&'a str>),
      All(&'a Vec<&'a str>),
      None(&'a Vec<&'a str>)
   }

   pub fn check_perms(
      usr_perms: &Vec<String>,
      req_perms: Option<PermCheckOptions>,
      check_min: bool,
      check_tumex: bool) -> bool {
      let min_perm = "mailer:baseaccess";
      let is_tumex = "is:tumex";
      let mut min_perms_check = false;

      if check_tumex || check_min {
         for perm in usr_perms.deref().iter() {
            if *perm == *min_perm && check_min {
               min_perms_check = true;
            }
            if *perm == *is_tumex && check_tumex {
               return true;
            }
         }
      }

      match req_perms {
         Some(PermCheckOptions::All(req_perms)) => {
            for perm in req_perms {
               if usr_perms.contains(&perm.deref().to_owned()) {
                  return false;
               }
            }

            return true;
         },
         Some(PermCheckOptions::AtLeastOne(req_perms) ) => {
            for perm in req_perms {
               if usr_perms.contains(&perm.deref().to_owned()) {
                  return true;
               }
            }

            return false;
         },
         Some(PermCheckOptions::None(req_perms) ) => {
            for perm in req_perms {
               if usr_perms.contains(&perm.deref().to_owned()) {
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
            permissions: token.get("permissions").and_then(|perms| {
               let iterable = perms.as_array();
               let mut res: Option<Vec<Permissions>> = None;                  
            
               if iterable.is_none() {
                  let iterable: &Vec<String> = &perms.to_string()
                     .split(" ")
                     .map(|perm| perm.to_string())
                     .collect();

                  if iterable.len() > 0 {
                     res = Some(Permissions::from_perms(iterable));
                  }
               } else {
                  res = Some(Permissions::from_perms(
                     iterable.unwrap()
                        .iter()
                        .map(|perm| perm.as_str())
                        .filter(|perm| perm.is_some())
                        .map(|perm| perm.unwrap().to_string())
                        .collect::<Vec<String>>()
                        .as_ref()
                  ));
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
                  let iterable: &Vec<String> = &perms.to_string()
                     .split(" ")
                     .map(|perm| perm.to_string())
                     .collect();

                  if iterable.len() > 0 {
                     res = Some(IsClaims::from_perms(iterable));
                  }
               } else {
                  res = Some(IsClaims::from_perms(
                     iterable.unwrap()
                        .iter()
                        .map(|perm| perm.as_str())
                        .filter(|perm| perm.is_some())
                        .map(|perm| perm.unwrap().to_string())
                        .collect::<Vec<String>>()
                        .as_ref()
                  ));
               }
               
               res
            }),
            role: token.get("role").and_then(|role| 
               Some(role.to_string().split(" ").map(|val| val.to_string()).collect())
            ),
         })
      }
   }
}
