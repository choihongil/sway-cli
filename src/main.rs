mod error;
mod launcher;
mod sway_ipc;

fn main() {
    let node = launcher::node_tree().unwrap();
    launcher::switch_or_launch_application("firefox", &node).unwrap();
}
