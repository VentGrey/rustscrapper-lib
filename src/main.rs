use std::process::Command;
use colored::*;

fn check_platfom() -> char {
    let output = if cfg!(target_os = "linux") {

    } else if cfg!(target_os = "windows") {
        panic!("{}", "Configuration tool is not supported on this OS!".red());
    };
}

fn main() {

}
