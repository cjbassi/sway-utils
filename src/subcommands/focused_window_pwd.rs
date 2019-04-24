use std::fs;
use std::path::PathBuf;
use std::process::Command;

use dirs::home_dir;
use i3ipc::{reply::Node, I3Connection};

fn search_focused_node(node: Node) -> Option<Node> {
    if node.focused {
        Some(node)
    } else {
        node.nodes
            .into_iter()
            .filter_map(search_focused_node)
            .collect::<Vec<Node>>()
            .get(0)
            .map(|node| node.to_owned())
    }
}

// https://www.reddit.com/r/swaywm/comments/bdseiq/modreturn_to_open_new_termite_window_in_the_same/
pub fn focused_window_pwd(terminal: &str) {
    let home_dir = home_dir().unwrap();
    let mut connection = I3Connection::connect().unwrap();
    let tree = connection.get_tree().unwrap();
    let pid = search_focused_node(tree).unwrap().pid.unwrap();
    let bytes_output = Command::new("ps")
        .args(&["-p", &pid.to_string(), "-o", "comm="])
        .output()
        .unwrap()
        .stdout;
    let pname = String::from_utf8_lossy(&bytes_output);
    let cwd = if pname.trim() == terminal {
        let bytes_output = Command::new("pgrep")
            .args(&["--newest", "--parent", &pid.to_string()])
            .output()
            .unwrap()
            .stdout;
        let ppid = String::from_utf8_lossy(&bytes_output);
        fs::read_link(
            PathBuf::from("/proc")
                .join(ppid.trim().to_string())
                .join("cwd"),
        )
        .unwrap_or(home_dir)
    } else {
        home_dir
    };
    println!("{}", cwd.display());
}
