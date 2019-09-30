use std::process::Command;
use colored::Colorize;

fn check_platfom() {
    if cfg!(target_os = "linux") {

    } else if cfg!(target_os = "windows") {
        panic!("{}", "Configuration tool is not supported on this OS!".red());
    } else {
        panic!("{}", "Could not determine your current OS".red());
    }


}

fn main() {
    println!("");
    println!("Welcome to <Name Pending>:");
}
