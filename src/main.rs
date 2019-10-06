use std::process::Command;
use colored::Colorize;

fn check_platfom() {
    if cfg!(target_os = "linux") {
        return;
    } else if cfg!(target_os = "windows") {
        panic!("{}", "Configuration tool is not supported on this OS!".red());
    } else {
        panic!("{}", "Could not determine your current OS".red());
    }


}

fn main() {
    println!("Initializing...");
    println!("{}","Detecting your current platform...".blink());
    check_platfom();
    println!("Welcome to {} (Hazardous Server Protection)", "H.S.P".green());
    println!("Please input your choice from our menu");
}
