use serde_json::Value as SerdeVal;
use std::{env, vec::Vec, rc::Rc};
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
   Auth0TokenRelated::{Auth0TokenFields},
   Auth0Perms::{IsClaims}, 
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
   pub user_id: String,
   pub decoded_payload: Auth0TokenFields,
   pub user_permissions: Vec<String>,
   pub user_roles: Option<Vec<String>>,
   pub is_claims: Vec<IsClaims>
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

      let jwks_vec = jwks.0.lock().await;
      let the_jwk = PublicKeys::get_components_by_kid(&jwks_vec, &token);
      if the_jwk.is_none() {
         return Outcome::Failure((
            HttpStatus::new(500), 
            AuthOutcomeErr::Unexpected
         ));
      }
      let the_jwk = the_jwk.unwrap();
      

      //* Verify closure for re-use in case of jwt failure
      let verify = |token: &str, n: &Modulus, e: &Exponent| -> Result<SerdeVal, JwtErr> {
         let algo = match Algorithm::new_rsa_n_e_b64_verifier(
            AlgorithmID::RS256, 
            &n.0, 
            &e.0
         ) {
            Ok(algo) => algo,
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
      

      //* Finally, verify the token
      let e = &the_jwk.exponent;
      let n = &the_jwk.modulus;
      match verify(&token, n, e) {
         Ok(res) => {
            let token_obj = Auth0TokenFields::from_serde_val(&res).unwrap();

            Outcome::Success(Auth {
               raw_token: token.to_owned(),
               decoded_payload: token_obj.clone(),
               user_id: token_obj.sub.clone(),
               user_permissions: token_obj.permissions.clone(),
               user_roles: token_obj.role,
               is_claims: IsClaims::from_perms(&token_obj.permissions)
            })
         },
         Err(_) => {
            //* Try refetching public keys from auth0 and try again

            match jwks.refetch_keys().await {
               Err(_) => return Outcome::Failure((
                  HttpStatus::new(500), 
                  AuthOutcomeErr::Unexpected
               )),
               Ok(_) => {}
            }

            drop(jwks_vec);
            let jwks_vec = jwks.0.lock().await;
            let the_jwk = PublicKeys::get_components_by_kid(&jwks_vec,&token);
            if the_jwk.is_none() {
               return Outcome::Failure((
                  HttpStatus::new(500), 
                  AuthOutcomeErr::Unexpected
               ));
            }
            let the_jwk = the_jwk.unwrap();
            
            let e = &the_jwk.exponent;
            let n = &the_jwk.modulus;

            //* Second attempt to verify the token, with possibly new keys
            match verify(&token, n, e) {
               Ok(res) => {
                  let token_obj = Auth0TokenFields::from_serde_val(&res).unwrap();

                  Outcome::Success(Auth {
                     raw_token: token.to_owned(),
                     decoded_payload: token_obj.clone(),
                     user_id: token_obj.sub.clone(),
                     user_permissions: token_obj.permissions.clone(),
                     user_roles: token_obj.role,
                     is_claims: IsClaims::from_perms(&token_obj.permissions)
                  })
               },
               Err(_) => Outcome::Failure((
                  HttpStatus::new(401),
                  AuthOutcomeErr::InvalidToken("We we unsable to verify your identity, your token is invalid!".to_owned())
               ))
            }
         }
      }
   }
}