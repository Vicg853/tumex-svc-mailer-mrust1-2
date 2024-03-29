use std::{env, panic};
use mongodb::{
   options::ClientOptions,
   Collection,
   Client,
   self,
   error::Error as MongoError,
};

use crate::models::message::Message;

pub struct MessageCmsDb {
   client: Client,
   msg_col: Collection<Message>
}

pub enum ConnCheck {
   Ok, 
   Issue(MongoError)
}

impl MessageCmsDb {
   pub async fn init() -> Self {
      #[allow(non_snake_case)]
      let CMS_DB_CLUST_URI = match env::var("CMS_DB_CLUST_URI") {
         Ok(val) => val,
         Err(_) => panic!("CMS_DB_CLUST_URI environment must be set")
      };

      #[allow(non_snake_case)]
      let CMS_MSG_DB_NAME = match env::var("CMS_MSG_DB_NAME") {
         Ok(val) => val,
         Err(_) => panic!("CMS_MSG_DB_NAME environment must be set")
      };

      //TODO - finish tls setup
      //let tls_opts = mongodb::options::TlsOptions {
      //   ..Default::default()
      //};

      let client_opts = ClientOptions::parse(CMS_DB_CLUST_URI)
         .await.expect("Failed to parse mongodb CMS DB's connection URI");

      match Client::with_options(client_opts) {
         Ok(client) => {
            let msg_col = client.database(CMS_MSG_DB_NAME.as_str())
            .collection("messages");

            MessageCmsDb {
               client,
               msg_col
            }
         },
         Err(err) => panic!("Failed to connect to CMS DB Cluster: {}", err)
      }
   }
   pub fn get_msg_col(&self) -> &Collection<Message> {
      &self.msg_col
   }
   pub async fn check_conn(&self) -> ConnCheck {
      match self.client.list_database_names(None, None).await {
         Ok(_) => ConnCheck::Ok,
         Err(err) => ConnCheck::Issue(err)
      }
   }
}