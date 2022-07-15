use jsonwebtokens::{Verifier, Algorithm, TokenData, AlgorithmID};
use std::vec::Vec;
use rocket::request::{FromRequest, Outcome};

use crate::auth::{IsClaims, TokenPayload};

//* Env and related
const TOKEN_TYPE: &srt = "Bearer ";
const this_aud: Fn = || {
   match env::var("THIS_AUDIENCE") {
      Ok(val) => val,
      Err(_) => panic!("THIS_AUDIENCE environment must be set")
   }
};

pub struct AuthedUser {
   pub raw_token: String,
   pub user_id: String,
   pub decoded_payload: TokenPayload,
   pub user_permissions: Option<Vec<String>>,
   pub user_roles: Option<Vec<String>>,
   pub is_claims: Vec<IsClaims>
}

pub enum Auth {
   Authed(AuthedUser),
   Unauthed
}

impl FromRequest for Auth {
   async fn from_request(request: & 'r rocket::Request< '_>) -> rocket::request::Outcome<Self,Self::Error> {
      let token = request.headers().get_one("Authorization");

      if token.is_none() 
         | token.unwrap().len() == 0 
         | !token.unwrap().starts_with(TOKEN_TYPE) {
         return Outcome::Failure(());
      }

      let token = token
         .unwrap().strip_prefix(TOKEN_TYPE).unwrap();
      
      let aud: String = this_aud().into();
      let verifier = Verifier::create()
      .audience(aud);

      let x509_keys = request.rocket().state::<>();
      
      const verif: Fn<&str> = |token: &str| -> Result<TokenData, jsonwebtokens::Error> {
         let algo = Algorithm::new_rsa_pem_verifier(AlgorithmID::RS256, key)

      };

         
   }
}