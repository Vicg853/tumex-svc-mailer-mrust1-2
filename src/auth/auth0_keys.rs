use rocket::warn;
use std::{vec::Vec, env};
use reqwest::{get, Error as ReqwestErr};
use openssl::{
   error::ErrorStack as RsaErrStack, 
   rsa::Rsa
};

fn tennant_endpoint() -> String {
   match env::var("TENNANT_ENDPOINT") {
      Ok(val) => val,
      Err(_) => panic!("TENANT_ENDPOINT environment must be set")
   }
}
const PUB_KEYS_PATH: &str = "/.well-known/jwks.json";

mod auth0_jwk_set {
   pub struct TenantKey {
      pub alg: String,
      pub kty: String,
      pub r#use: String,
      pub x5c: Vec<String>,
      pub n: String,
      pub e: String,
      pub kid: String,
      pub x5t: String
   }
   pub struct TenantKeysResponse {
      pub keys: Vec<TenantKey>
   }
}

struct KeyComponents {
   pub modulus: String,
   pub exponent: String,
   pub kid: String,
}
pub struct PublicKeys(Vec<KeyComponents>);

async fn fetch_components() -> Result<Vec<KeyComponents>, ReqwestErr> {
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
            warn!("Failed to parse public JWT key set from auth0. The following error was encountered: {}", json.err().unwrap());
            Err(json.err().unwrap())
         } 

         let json = json.unwrap();
         let keys_vec = Vec::new();
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
         warn!("Failed to fetch public JWT key set components from auth0. The following error was encountered: {}", keys.err().unwrap());
         Err(keys.err().unwrap())
      }

      Ok(PublicKeys(keys.unwrap()))
   }

   pub async fn refetch_keys(&mut self) -> Result<(), ReqwestErr> {
      let keys = fetch_components().await;

      if keys.is_err() {
         warn!("Failed to fetch public JWT key set components from auth0. The following error was encountered: {}", keys.err().unwrap());
         Err(keys.err().unwrap())
      }

      self.0 = keys.unwrap();
      Ok(())
   }

   pub fn get_components(&self, kid: &str) -> Option<&KeyComponents> {
      self.0.iter().find(|key| {
         **key.kid == kid
      })
   }

   pub fn rsa_from_components(&self, kid: &str) -> Option<Result<String, RsaErrStack>> {
      let components = self.get_components(kid)?;

      match Rsa::from_public_components(
         components.modulus.parse::<u64>().unwrap(),
         components.exponent.parse::<u64>().unwrap()
      ) {
         Ok(rsa) => {
            let pem = rsa.public_key_to_pem_pkcs1();

            if pem.is_err() {
               warn!("Failed to convert public key to PEM format. The following error was encountered: {}", pem.err().unwrap());
               Some(Err(pem.err()))
            } else {
               Some(pem.unwrap())
            }
         }
         Err(err) => {
            warn!("Failed to convert public key to RSA format. The following error was encountered: {}", err);
            Some(Err(err))
         }
      }


   }
}