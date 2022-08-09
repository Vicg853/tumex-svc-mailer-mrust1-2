mod state {
   use chrono::{DateTime, Utc};

   struct ClientRecord {
      ip: String,
      count: u32,
      last_request: DateTime<Utc>,
      first_request: DateTime<Utc>
   }

   pub enum RateType {
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

      pub fn reset_client(&mut self, ip: &str) -> Option<&ClientRecord> {
         for client in self.clients.iter_mut() {
            if client.ip == ip {
               client.count = 0;
               client.last_request = Utc::now();
               client.first_request = Utc::now();
               return Some(&client);
            }
         }

         None
      }

      pub fn on_the_limit(&mut self, client: &ClientRecord) -> bool {
         let now = Utc::now();
         let diff = now - client.last_request;

         let mut should_rst = false;

         use RateType::*;
         match self.limit {
            PerSecond(limit) => {
               let smaller_than = diff.num_seconds() <= 1;
               if !smaller_than {
                  should_rst = true;6
               } else if smaller_than && client.count >= limit {
                  return false;
               }
            },
            PerMinute(limit) => {
               let smaller_than = diff.num_minutes() <= 1;
               if !smaller_than {
                  should_rst = true;
               } else if smaller_than && client.count >= limit {
                  return false;
               }
            },
            PerHour(limit) => {
               let smaller_than = diff.num_hours() <= 1;
               if !smaller_than {
                  should_rst = true;
               } else if smaller_than && client.count >= limit {
                  return false;
               }
            },
            PerDay(limit) => {
               let smaller_than = diff.num_days() <= 1;
               if !smaller_than {
                  should_rst = true;
               } else if smaller_than && client.count >= limit {
                  return false;
               }
            }
         }
         
         if should_rst {
            self.reset_client(&client.ip); 
         }

         true
      }

      pub fn full_run(self, ip: String) -> (Self, bool) {
         self.add_client(ip);
         let client = self.get_client(&ip).unwrap();

         let on_the_limit = self.on_the_limit(client);
         (self, on_the_limit)
      }
   }

   
   pub struct ServerLimit{
      limit: RateType,
      current_count: u32,
      last_reset: DateTime<Utc>
   }

   impl ServerLimit {
      pub fn new(limit: RateType) -> Self {
         Self {
            limit,
            current_count: 0,
            last_reset: Utc::now()
         }
      }

      pub fn reset(&mut self) {
         self.current_count = 0;
         self.last_reset = Utc::now();
      }

      pub fn increment(&mut self) {
         self.current_count += 1;
      }

      pub fn check(&self) -> bool {
         let now = Utc::now();
         let diff = now - self.last_reset;

         use RateType::*;
         match self.limit {
            PerSecond(limit) => {
               if diff.num_seconds() <= 1 && self.current_count >= limit {
                  return false;
               }
            },
            PerMinute(limit) => {
               if diff.num_minutes() > 1 && self.current_count > limit {
                  self.reset();
                  return false;
               }
            },
            PerHour(limit) => {
               if diff.num_hours() > 1 && self.current_count > limit {
                  self.reset();
                  return false;
               }
            },
            PerDay(limit) => {
               if diff.num_days() > 1 && self.current_count > limit {
                  self.reset();
                  return false;
               }
            }
         }

         true
      }
   }
}
