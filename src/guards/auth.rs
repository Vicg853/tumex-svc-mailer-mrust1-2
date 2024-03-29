use serde_json::Value as SerdeVal;
use std::env;
use jsonwebtokens::{
   error::Error as JwtErr, 
   AlgorithmID, Algorithm,
   Verifier
};
use rocket::{
   http::Status as HttpStatus, log::private::warn,
   request::{FromRequest, Outcome},
   async_trait
};

use crate::auth::{
   auth0_key_components::{Exponent, Modulus},
   auth0_token_related::{Auth0TokenFields, PermCheckOpt},
   auth0_perm_claims::{IsPerm, ScopePerm},
   PublicKeys
};

//* Env and related
const TOKEN_TYPE: &str = "Bearer ";
fn this_aud() -> String {
   match env::var("CURR_AUDIENCE") {
      Ok(val) => val,
      Err(_) => panic!("CURR_AUDIENCE environment must be set")
   }
}

pub struct Auth{
   pub raw_token: String,
   pub decoded_payload: Auth0TokenFields,
}

#[derive(Debug)]
pub enum AuthOutcomeErr  {
   Unauthorized(String),
   InvalidToken(String),
   Forbidden(String),
   Unexpected
}

#[async_trait]
impl<'r> FromRequest<'r> for Auth {
   type Error = AuthOutcomeErr;

   async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
      let token = request.headers().get_one("Authorization");

      if token.is_none() 
         || (token.unwrap().len() == 0) 
         || (!token.unwrap().starts_with(TOKEN_TYPE)) {
         return Outcome::Failure((
            HttpStatus::new(401),
            AuthOutcomeErr::Unauthorized("Either no token is present on the header or is of invalid type!".to_owned())));
      }

      let token = token
         .unwrap().strip_prefix(TOKEN_TYPE).unwrap();
      
      //* Building the JWT verifier
      let aud: String = this_aud().into();
      let verifier = Verifier::create()
         .audience(aud)
         .build().unwrap();

      //* Retrieving pub keys from auth0
      let jwks = request.rocket().state::<PublicKeys>();
      if jwks.is_none() {
         warn!("Public keys state fetch failed");
         return Outcome::Failure((HttpStatus::new(500), AuthOutcomeErr::Unexpected));
      }
      let jwks = jwks.unwrap();

      let jwks_vec = jwks.0.read().await;
      let the_jwk = PublicKeys::get_components_by_kid(&jwks_vec, &token);
      if the_jwk.is_none() {
         return Outcome::Failure((
            HttpStatus::new(500), 
            AuthOutcomeErr::Unexpected
         ));
      }
      let the_jwk = the_jwk.unwrap().to_owned();

      //* Verify closure for re-use in case of jwt failure
      let verify = |token: &str, modulus: &Modulus, exponent: &Exponent, comps_kid: &str| -> Result<SerdeVal, JwtErr> {
         let algo = match Algorithm::new_rsa_n_e_b64_verifier(
            AlgorithmID::RS256, 
            &modulus.0, 
            &exponent.0
         ) {
            Ok(mut algo) => {
               algo.set_kid(comps_kid);
               algo
            },
            Err(err) => {
               warn!("Failed to create RSA verifier. Error: {}", err);
               return Err(err);
            }
         };
         
         match verifier.verify(token, &algo) {
            Ok(token_data) => Ok(token_data),
            Err(err) => {
               warn!("Failed to verify token. Error: {}", err);
               Err(err)
            }
         }
      }; 

      //* Token decode and processing closure for re-use
      let get_auth_data = |tkn: &str, verified_tkn: SerdeVal| {
         let token_obj = Auth0TokenFields::from_serde_val(verified_tkn).unwrap();

         Auth {
            raw_token: tkn.to_owned(),
            decoded_payload: token_obj,
         }
      };

      let has_min_perms = |auth_data: &Auth| -> bool {
         let req_perms = vec![ IsPerm::SUDO_HIGH ];
         
         let is_check = auth_data.decoded_payload.check_perm(
            Some(PermCheckOpt::All(req_perms)),
            true, true
         );
         
         let req_perms = vec![ ScopePerm::MAILER_BASE_ACCESS ];
         let scope_check = auth_data.decoded_payload.check_perm(
            Some(PermCheckOpt::All(req_perms)),
            true, true
         );

         is_check || scope_check
      };
      

      //* Finally, verify the token
      let e = &the_jwk.exponent;
      let n = &the_jwk.modulus;
      let kid = &the_jwk.kid;
      match verify(&token, n, e, kid) {
         Ok(res) => {
            drop(jwks_vec);
            let auth_data = get_auth_data(&token, res);
            
            if !has_min_perms(&auth_data) {
               Outcome::Failure((
                  HttpStatus::new(403), 
                  AuthOutcomeErr::Forbidden("User does not have sufficient permissions!".to_owned())
               ))
            } else {
               Outcome::Success(auth_data)
            }
         },
         Err(_) => {
            //TODO Make this safer by creating a function wrapper that autmatically drops the lock
            // ! and also uses RwLock to ensure additional safety by keeping track of read/write refs
            drop(jwks_vec);

            //* Try refetching public keys from auth0 and try again
            match jwks.refetch_keys().await {
               Err(_) => return Outcome::Failure((
                  HttpStatus::new(500), 
                  AuthOutcomeErr::Unexpected
               )),
               Ok(_) => {}
            }

            let jwks_vec = jwks.0.read().await;
            let the_jwk = PublicKeys::get_components_by_kid(&jwks_vec,&token);
            if the_jwk.is_none() {
               return Outcome::Failure((
                  HttpStatus::new(500), 
                  AuthOutcomeErr::Unexpected
               ));
            }
            let the_jwk = the_jwk.unwrap().to_owned();
            
            let e = &the_jwk.exponent;
            let n = &the_jwk.modulus;
            let kid = &the_jwk.kid;

            //* Second attempt to verify the token, with possibly new keys
            match verify(&token, n, e, kid) {
               Ok(res) => {
                  drop(jwks_vec);
                  let auth_data = get_auth_data(&token, res);

                  if !has_min_perms(&auth_data) {
                     Outcome::Failure((
                        HttpStatus::new(403), 
                        AuthOutcomeErr::Forbidden("User does not have sufficient permissions!".to_owned())
                     ))
                  } else {
                     Outcome::Success(auth_data)
                  }
               },
               Err(_) => {
                  drop(jwks_vec);
                  Outcome::Failure((
                     HttpStatus::new(401),
                     AuthOutcomeErr::InvalidToken("We we unable to verify your identity or your token is invalid!".to_owned())
                  ))
               }
            }
         }
      }
   }
}
