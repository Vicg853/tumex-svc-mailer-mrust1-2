use chrono::{DateTime, Duration, Utc};

pub struct ClientRecord {
    ip: String,
    count: u32,
    last_request: DateTime<Utc>,
    first_request: DateTime<Utc>,
}

pub struct RateType(Duration, u32);

impl RateType {
    pub fn new(duration: Duration, req_limit: u32) -> Self {
        Self(duration, req_limit)
    }
}

pub struct RateLimitState {
    clients: Vec<ClientRecord>,
    limit: RateType,
    reset_timeout: Duration,
}

impl RateLimitState {
    pub fn new(rate: RateType, reset_timeout: Duration) -> Self {
        Self {
            clients: Vec::new(),
            limit: rate,
            reset_timeout,
        }
    }

    pub fn add_client(&mut self, ip: String) {
        let now = Utc::now();

        for client in self.clients.iter_mut() {
            if client.ip == ip {
                client.count += 1;
                client.last_request = now;
                return;
            }
        }

        self.clients.push(ClientRecord {
            ip,
            count: 1,
            last_request: now,
            first_request: now,
        });
    }

    fn get_client(&self, ip: &str) -> Option<&ClientRecord> {
        for client in self.clients.iter() {
            if client.ip == ip {
                return Some(client);
            }
        }

        None
    }

    pub fn reset_client<'s>(&'s mut self, ip: &str) -> Option<&'s ClientRecord> {
        for client in self.clients.iter_mut() {
            if client.ip == ip {
                client.count = 0;
                client.last_request = Utc::now();
                client.first_request = Utc::now();
                return Some(client);
            }
        }

        None
    }

    pub fn on_the_limit(&mut self, ip: &str) -> bool {
        let client = self.get_client(ip);
        if client.is_none() {
            return false;
        }
        let client = client.unwrap();

        if (Utc::now().timestamp() - client.last_request.timestamp()) <= self.limit.0.num_seconds()
            && client.count >= self.limit.1
        {
            false
        } else {
            true
        }
    }

    pub fn passed_reset_timeout(&self, ip: &str) -> bool {
        let client = self.get_client(ip);
        if client.is_none() {
            return false;
        }
        let client = client.unwrap();

        (Utc::now().timestamp() - client.last_request.timestamp())
            > self.reset_timeout.num_seconds()
    }

    pub fn full_check_on_the_limit(&mut self, ip: String) -> bool {
        if self.passed_reset_timeout(&ip) {
            self.reset_client(&ip);
            return true;
        }

        self.add_client(ip.clone());
        self.on_the_limit(&ip)
    }
}

pub struct ServerLimit {
    limit: RateType,
    current_count: u32,
    last_reset: DateTime<Utc>,
    reset_timeout: Duration,
}

impl ServerLimit {
    pub fn new(limit: RateType, reset_timeout: Duration) -> Self {
        Self {
            limit,
            reset_timeout,
            current_count: 0,
            last_reset: Utc::now(),
        }
    }

    pub fn reset(&mut self) {
        self.current_count = 0;
        self.last_reset = Utc::now();
    }

    pub fn increment(&mut self) {
        self.current_count += 1;
    }

    pub fn check(&mut self) -> bool {
        let now = Utc::now();
        let diff = now.timestamp() - self.last_reset.timestamp();

        if diff > self.reset_timeout.num_seconds() {
            self.reset();
            return true;
        }

        if diff < self.limit.0.num_seconds() && self.current_count >= self.limit.1 {
            false
        } else {
            true
        }
    }
}
