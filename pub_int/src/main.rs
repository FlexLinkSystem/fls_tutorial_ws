use fls::node::Node;
use fls::param::Param;
use std_msgs;

fn main() {
    let mut node = Node::new("Publisher".to_string());

    let param = Param::new().unwrap();

    let send_value = param.get_i64_parameter("send_value").unwrap();

    let mut pu = node.create_publisher::<std_msgs::msg::Int32>("test".to_string());

    loop{
        let mut new_msg = std_msgs::msg::Int32::new();
        new_msg.data = send_value as i32;

        pu.publish(new_msg);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
