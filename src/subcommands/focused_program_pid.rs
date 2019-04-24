use i3ipc::{reply::Node, I3Connection};

fn search_for_focused_node(node: Node) -> Option<Node> {
    if node.focused {
        Some(node)
    } else {
        node.nodes
            .into_iter()
            .filter_map(search_for_focused_node)
            .collect::<Vec<Node>>()
            .get(0)
            .map(|node| node.to_owned())
    }
}

pub fn get_focused_program_pid() -> i32 {
    let mut connection = I3Connection::connect().unwrap();
    let tree = connection.get_tree().unwrap();
    search_for_focused_node(tree).unwrap().pid.unwrap()
}

pub fn focused_program_pid() {
    println!("{}", get_focused_program_pid());
}
