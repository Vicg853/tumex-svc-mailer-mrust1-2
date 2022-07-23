mod send_msg;
mod get_msgs;

pub use send_msg::send_message as sd_msg_route;
pub use get_msgs::get_msgs as gt_msg_route;