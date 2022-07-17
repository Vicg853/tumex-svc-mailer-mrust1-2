pub enum IsClaims {
   TUMEX,
   FRIENDS_NORMAL,
   FRIENDS_CLOSE,
   FRIENDS_BFF,
   FAMILY_FIRST,
   FAMILY_SECOND,
   FAMILY_THIRD,
   SUDO_LOW,
   SUDO_HIGH,
}

impl IsClaims {
   pub fn as_str(&self) -> &'static str {
      match self {
         IsClaims::TUMEX => "is:tumex",
         IsClaims::FRIENDS_NORMAL => "is:friends:normal",
         IsClaims::FRIENDS_CLOSE => "is:friends:close",
         IsClaims::FRIENDS_BFF => "is:friends:bff",
         IsClaims::FAMILY_FIRST => "is:family:first-deg",
         IsClaims::FAMILY_SECOND => "is:family:second-deg",
         IsClaims::FAMILY_THIRD => "is:family:third-deg",
         IsClaims::SUDO_LOW => "is:sudo:low",
         IsClaims::SUDO_HIGH => "is:sudo:high"
      }
   }
}

pub enum Permissions {
   MAILER_BASE_ACCESS,
   MAILER_WEBP_MSGS_READ,
   MAILER_WEBP_MSGS_DEL
}

impl Permissions {
   pub fn as_str(&self) -> &'static str {
      match self {
         Permissions::MAILER_BASE_ACCESS => "mailer:baseaccess",
         Permissions::MAILER_WEBP_MSGS_READ => "mailer:webp:messages:read",
         Permissions::MAILER_WEBP_MSGS_DEL => "mailer:webp:messages:delete",
      }
   }
}