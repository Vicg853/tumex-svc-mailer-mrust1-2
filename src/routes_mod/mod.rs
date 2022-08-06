mod send_msg;
mod get_msgs;
mod health;
mod read_message;
mod msg_opacity;

pub use msg_opacity::toggle_read_archive as toggle_read_archive_route;
pub use read_message::{get_msg as get_msg_route, get_msg_no_id as get_msg_no_id_route};
pub use health::check_health as check_health_route;
pub use send_msg::send_message as sd_msg_route;
pub use get_msgs::get_msgs as gt_msg_route;
