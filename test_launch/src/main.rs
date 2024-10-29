use fls_launch;

fn main() {
    let mut pub_int = fls_launch::LaunchNode::new("pub_int");
    pub_int.set_config_file("test.yaml");
    let sub_int = fls_launch::LaunchNode::new("sub_int");

    let mut launcher = fls_launch::Launcher::new();

    launcher.add(pub_int);
    launcher.add(sub_int);

    launcher.launch();
}
