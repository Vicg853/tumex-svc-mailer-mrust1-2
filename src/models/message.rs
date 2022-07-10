use serde::{Deserialize, Serialize};
use mongodb::bson::{
   oid::{ObjectId}, 
   DateTime
};

#[derive(Serialize, Deserialize)]
pub struct Message {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<ObjectId>,
   #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
   pub created_at: Option<DateTime>,
   pub from: String,
   pub name: String,
   pub subject: String,
   pub message: String,
   pub read: bool
}