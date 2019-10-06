use std::process::Command;
use colored::Colorize;
use scanln::scanln;

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
    menu();
}

fn menu() -> u8 {
    println!("\t MENU 1 \t");
    println!("\t Please choose what kind of operation you wish to perform");
    println!("\t {}", "1- System Checking".yellow());
    println!("\t {}", "2- Dependency Check".yellow());
    println!("\t {}", "3- DevOps".yellow());
    println!("\t {}", "4- User Database".yellow());
    println!("\t {}", "5- Exit".yellow());
    let input = scanln!("> ");
    let input:u8 = input.parse().unwrap();

    input
}
