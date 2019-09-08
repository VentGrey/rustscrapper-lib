use std::process::Command;
use colored::*;

fn check_os() {
    let output = if cfg!(target_os = "linux") {
        println!("Checking your Linux Distribution");
        Command::new("");
    } else if cfg!(target_os = "windows") {
        panic!("{}", "Configuration tool is not supported on this OS!".red());
    };
}

fn main() {

}
