use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::subcommands::get_focused_program_pid;

pub fn focused_program_cwd() {
    let pid = get_focused_program_pid();
    let bytes_output = Command::new("pgrep")
        .args(&["--newest", "--parent", &pid.to_string()])
        .output()
        .unwrap()
        .stdout;
    let ppid = String::from_utf8_lossy(&bytes_output);
    let cwd = fs::read_link(
        PathBuf::from("/proc")
            .join(ppid.trim().to_string())
            .join("cwd"),
    )
    .unwrap();
    println!("{}", cwd.display());
}
