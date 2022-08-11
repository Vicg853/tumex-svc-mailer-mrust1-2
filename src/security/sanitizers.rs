use ammonia;

pub fn message_sanitizing(msg: String) -> String {
   ammonia::Builder::empty()
      .clean(&msg)
      .to_string()
}