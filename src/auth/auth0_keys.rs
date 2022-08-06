use rocket::{warn, log::private::info};
use std::{vec::Vec, env};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use reqwest::{get, Error as ReqwestErr};
use jsonwebtokens::raw::decode_header_only;

fn tennant_endpoint() -> String {
   match env::var("TENNANT_ENDPOINT") {
      Ok(val) => val,
      Err(_) => panic!("TENNANT_ENDPOINT environment must be set")
   }
}
const PUB_KEYS_PATH: &str = "/.well-known/jwks.json";

pub mod auth0_key_components {
   use serde::Deserialize;
   
   #[derive(Debug, Deserialize)]
   pub struct Modulus(pub String);
   #[derive(Debug, Deserialize)]
   pub struct Exponent(pub String);
}
mod auth0_jwk_set {
    use serde::Deserialize;
    use super::auth0_key_components::{Modulus, Exponent};

    #[derive(Deserialize)]
   pub struct TenantKey {
      pub alg: String,
      pub kty: String,
      pub r#use: String,
      pub x5c: Vec<String>,
      pub n: Modulus,
      pub e: Exponent,
      pub kid: String,
      pub x5t: String
   }

   #[derive(Deserialize)]
   pub struct TenantKeysResponse {
      pub keys: Vec<TenantKey>
   }
}

use auth0_key_components::{Exponent, Modulus};
#[derive(Debug)]
pub struct KeyComponents {
   pub modulus: Modulus,
   pub exponent: Exponent,
   pub kid: String,
}

#[derive(Debug)]
pub struct PublicKeys(pub RwLock<Vec<KeyComponents>>);

async fn fetch_components() -> Result<Vec<KeyComponents>, ReqwestErr> {
   info!("Fetching public keys from {}", tennant_endpoint());
   let endpt = tennant_endpoint();
   let url = format!("{}{}", endpt.to_owned(), PUB_KEYS_PATH);

   match get(&url).await {
      Err(err) => {
         warn!("Failed to fetch public x509 keys from auth0. The following error was encountered: {}", err);

         Err(err)
      },
      Ok(res) => {
         let json = res.json
            ::<auth0_jwk_set::TenantKeysResponse>().await;

         if json.is_err() {
            warn!("Failed to parse public JWT key set from auth0. The following error was encountered: {}", json.as_ref().err().unwrap());
            return Err(json.err().unwrap());
         } 

         let json = json.unwrap();
         let mut keys_vec: Vec<KeyComponents> = Vec::new();
         for key in json.keys.into_iter() {
            keys_vec.push(KeyComponents {
               modulus: key.n,
               exponent: key.e,
               kid: key.kid
            })
         }

         Ok(keys_vec)
      }
   }
}

impl PublicKeys {
   pub async fn new() -> Result<Self, ReqwestErr> {
      let keys = fetch_components().await;

      if keys.is_err() {
         warn!("Failed to fetch public JWT key set components from auth0. The following error was encountered: {:?}", keys.as_ref().err().unwrap());
         return Err(keys.err().unwrap())
      }

      Ok(PublicKeys(RwLock::new(keys.unwrap())))
   }

   pub async fn refetch_keys(&self) -> Result<(), ReqwestErr> {
      let keys = fetch_components().await;

      if keys.is_err() {
         warn!("Failed to fetch public JWT key set components from auth0. The following error was encountered: {:?}", keys.as_ref().err().unwrap());
         return Err(keys.err().unwrap())
      }

      let mut prev_keys = self.0.write().await;
      prev_keys.clear();
      prev_keys.extend(keys.unwrap());

      drop(prev_keys);
      Ok(())
   }

   pub fn get_components<'guard>(locked_components: &'guard RwLockReadGuard<'guard, Vec<KeyComponents>>, kid: &str) -> Option<&'guard KeyComponents> {
      locked_components.iter().find(|&key| {
         *key.kid == *kid
      })
   }

   pub fn get_components_by_kid<'guard>(locked_components: &'guard RwLockReadGuard<'guard, Vec<KeyComponents>>, jwt: &str) -> Option<&'guard KeyComponents> {
      let mut kid = String::new();
      let decoded_token_head = decode_header_only(&jwt);

      if decoded_token_head.is_ok() {
         let decoded_token = decoded_token_head.unwrap();

         let opt_kid = decoded_token.get("kid");

         if opt_kid.is_none() {
            return None;
         } else {
            kid.push_str(opt_kid.unwrap().as_str().unwrap());
         }
      } else {
         warn!("Failed to decode token: {}", 
            decoded_token_head.err().as_ref().unwrap());
         return None;
      }

      match Self::get_components(locked_components, kid.as_str()) {
         Some(jwt) => Some(jwt),
         None => {
            warn!("Failed to find kid in jwks. KID: {}", kid.as_str());
            None
         }
      }
   }
   pub async fn safe_read_lock_exec<'guard, Res>(
      &'guard self, 
      f: impl FnOnce(&RwLockReadGuard<'guard, Vec<KeyComponents>>) -> Res
   ) -> Res {
      let lock = self.0.read().await;

      let f_res = f(&lock);

      drop(lock);
      f_res
   }

   pub async fn safe_write_lock_exec<'guard, Res>(
      &'guard self, 
      f: impl FnOnce(&mut RwLockWriteGuard<'guard, Vec<KeyComponents>>) -> Res
   ) -> Res {
      let mut lock = self.0.write().await;

      let f_res = f(&mut lock);

      drop(lock);
      f_res
   }
}