use std::str::FromStr;

use rocket_cors::{
    AllowedOrigins,
    Method,
    AllowedHeaders,
    CorsOptions
};
use rocket::{
    futures::future::BoxFuture,
    http::Header,
    Response, Request,
};

pub struct HeaderFairings;

impl HeaderFairings {
    pub fn header_res_filter<'r, 's, 'b> (
        _req: &'r Request<'s>, 
        res: &'b mut Response<'r>
    ) -> BoxFuture<'b, ()> {
        Box::pin(async {
            res.remove_header("X-Powered-By");

            if res.headers().get_one("X-Content-Type-Options").is_some() {
                res.remove_header("X-Content-Type-Options");
            }
            let no_sniff = Header::new("X-Content-Type-Options", "nosniff");
            res.adjoin_header(no_sniff);

            let hsts = Header::new(
                "Strict-Transport-Security",
                "max-age=31536000; includeSubDomains",
            );
            res.adjoin_header(hsts);

            let xss_prevention = Header::new("X-XSS-Protection", "1; mode=block");
            res.adjoin_header(xss_prevention);
        })
    }

    pub fn rocket_cors_config() -> CorsOptions {
        CorsOptions {
            allowed_origins: AllowedOrigins::some(&["https://victorgomez.dev"], &[r"^https://(.*\.)*victorgomez.dev$"]),
            allowed_methods: vec![Method::from_str("GET").unwrap(), Method::from_str("POST").unwrap()].into_iter().map(From::from).collect(),
            allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
            ..Default::default()
        }
    }
}
