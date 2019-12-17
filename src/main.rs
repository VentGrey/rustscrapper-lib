use colored::Colorize;
use scanln::scanln;

mod system;
mod deps;


fn check_platfom() {
    if cfg!(target_os = "linux") {
        
    } else if cfg!(target_os = "windows") {
        panic!(
            "{}",
            "Configuration tool is not supported on this OS!".red()
        );
    } else {
        panic!("{}", "Could not determine your current OS".red());
    }
}

fn menu() -> u8 {
    println!("\t MENU 1 \t");
    println!("\t Please choose what kind of operation you wish to perform");
    println!("\t {}", "1- System Checking".green());
    println!("\t {}", "2- Dependency Check".blue());
    println!("\t {}", "3- DevOps".yellow());
    println!("\t {}", "4- User Database".yellow());
    println!("\t {}", "5- Exit".red());
    let input = scanln!("> ");
    let input: u8 = input.parse().unwrap();

    input
}

fn main() {
    println!("Initializing...");
    check_platfom();
    println!(
        " λ Welcome to {} (Hazardous Conditions Server Protection System) λ",
        "H.C.S.P.S".green()
    );
    println!("Please input your choice from our menu");
    let option = menu();

    match option {
        1 => system::mainsys(),
        2 => deps::check_deps(),
        5 => {},
        _ => panic!("{}", "Invalid Value, Aborting".red()),
    }
}
