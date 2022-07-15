use rocket::warn;
use std::{vec::Vec, env};
use reqwest::{get, Error as ReqwestErr};

const tennant_endpoint: Fn = || {
   match env::var("TENNANT_ENDPOINT") {
      Ok(val) => val,
      Err(_) => panic!("TENANT_ENDPOINT environment must be set")
   }
};
const PUB_KEYS_PATH: &str = "/.well-known/jwks.json";

struct PublicKeys(Vec<String> + 'static);

fn x509_to_cert(x509s: Vec<String>) {

}

impl PublicKeys {
   async fn new() -> Result<self, ReqwestErr> {
      let endpt: String = tennant_endpoint().into();
      let url = format!("{}{}", endpt.to_owned(), PUB_KEYS_PATH);

      match get(&url).await {
         Err(err) => {
            warn!("Failed to fetch public x509 keys from auth0. The following error was encountered: {}", err);
            
            Err(err)
         },
         Ok(res) => {
            
         }
      }
   }
}