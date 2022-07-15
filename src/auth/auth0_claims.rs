pub struct TokenPayload {
   
}

pub enum IsClaims {
   TUMEX = "is:tumex",
   FRIENDS_NORMAL = "is:friends:normal",
   FRIENDS_CLOSE = "is:friends:close",
   FRIENDS_BFF = "is:friends:bff",
   FAMILY_FIRST = "is:family:first-deg",
   FAMILY_SECOND = "is:family:second-deg",
   FAMILY_THIRD = "is:family:third-deg",
   SUDO_LOW = "is:sudo:low",
   SUDO_HIGH = "is:sudo:high",
}

pub enum Permissions {
   MAILER_BASE_ACCESS = "mailer:baseaccess",
   MAILER_WEBP_MSGS_READ = "mailer:webp:messages:read",
   MAILER_WEBP_MSGS_DEL = "mailer:webp:messages:delete",
}