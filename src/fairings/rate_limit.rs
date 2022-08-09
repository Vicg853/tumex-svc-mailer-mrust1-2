mod state {
   use chrono::{DateTime, Utc};

   struct ClientRecord {
      ip: String,
      count: u32,
      last_request: DateTime<Utc>,
      first_request: DateTime<Utc>
   }

   enum RateType {
      PerSecond(u32),
      PerMinute(u32),
      PerHour(u32),
      PerDay(u32)
   }

   pub struct RateLimitState{
      clients: Vec<ClientRecord>,
      limit: RateType,

   }

   impl RateLimitState {
      pub fn new(rate: RateType) -> Self {
         Self {
            clients: Vec::new(),
            limit: rate
         }
      }

      pub fn add_client(&mut self, ip: String) {
         let mut found = false;
         let now = Utc::now();

         for client in self.clients.iter_mut() {
            if client.ip == ip {
               client.count += 1;
               client.last_request = now;
               found = true;
               return;
            }
         }

         if !found {
            self.clients.push(ClientRecord {
               ip,
               count: 1,
               last_request: now,
               first_request: now
            });
         }
      }

      pub fn get_client(&self, ip: &str) -> Option<&ClientRecord> {
         for client in self.clients.iter() {
            if client.ip == ip {
               return Some(client);
            }
         }

         None
      }

      pub fn on_the_limit(&self, client: &ClientRecord) -> bool {
         let now = Utc::now();
         let diff = now - client.last_request;

         use RateType::*;
         match self.limit {
            PerSecond(limit) => {
               if diff.num_seconds() <= 1 && client.count >= limit {
                  return false;
               }
            },
            PerMinute(limit) => {
               if diff.num_minutes() <= 1 && client.count >= limit {
                  return false;
               }
            },
            PerHour(limit) => {
               if diff.num_hours() <= 1 && client.count >= limit {
                  return false;
               }
            },
            PerDay(limit) => {
               if diff.num_days() <= 1 && client.count >= limit {
                  return false;
               }
            }
         }
         
         true
      }
   }
}
