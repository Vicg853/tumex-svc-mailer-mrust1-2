use rocket::{
   Request, 
   Data, 
   futures::future::BoxFuture, http::uri::Origin,
   uri
};
use tokio::sync::RwLock;
use super::super::security::RateLimitState;

pub struct PerMinRateLimit(pub RwLock<RateLimitState>);

pub fn rate_limiter<'a>(req: &'a mut Request<'_>, _data: &'a Data<'_>) -> BoxFuture<'a, ()> {
   Box::pin(async move {
      let ip = req.client_ip();
      if ip.is_none() {
         req.set_uri(Origin::from(uri!("/420")));
         return;
      }
      let ip = ip.unwrap().to_string();

      let rate_state = req.rocket().state::<PerMinRateLimit>();
      if rate_state.is_none() {
         req.set_uri(Origin::from(uri!("/500")));
         return;
      }
      let mut rate_state = rate_state.unwrap().0.write().await;
      
      let on_the_limit = rate_state.full_check_on_the_limit(ip);
      if !on_the_limit {
         req.set_uri(Origin::from(uri!("/420")));
      }

      return;
   })
}