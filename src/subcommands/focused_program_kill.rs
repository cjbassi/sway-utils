use std::process::Command;

use crate::subcommands::get_focused_program_pid;

pub fn focused_program_kill() {
    let pid = get_focused_program_pid();
    Command::new("kill")
        .args(&[&pid.to_string()])
        .spawn()
        .unwrap();
}
