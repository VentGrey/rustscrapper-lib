use colored::Colorize;
use scanln::scanln;

use std::env;

mod system;
mod deps;

/* TODO: Refactor, this will be only an entry point */

fn check_platfom() {
    // FIXME: This should try to cover all unix-like
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
    check_platfom();
    /* Arguments parsing section */
    let args: Vec<String> = env::args().collect();

    /* Arguments count */
    println!("{}", args.len());

    match args.len() {
        1 => {
            system::init()
        },
        2 => {
            match args[1].as_str() {
                "-h" | "--help" => {
                    println!(" Usage: rustscrapper [options] [quickruns]");
                    println!(" Options:");
                    println!(" -h, --help\t show this help message and exit");
                    println!(" -l, --list\t list available quickruns");
                    println!(" -q, --quick\t perform a (safe) quick cleanup");
                    println!(" -s, --system\t show basic system information");
                    return;
                },
                // TODO: Call functions here
                _ => {
                    println!("Invalid argument, run 'rustscrapper --help to see available options'");
                    std::process::exit(1);
                }
            }
        },

        _ => panic!("Invalid arguments or length")
    }



    println!("Welcome to {}", "RustScrapper".red());
    println!("Please input your choice from our menu");
    let option = menu();

    match option {
        1 => system::mainsys(),
        2 => deps::check_deps(),
        5 => {},
        _ => panic!("{}", "Invalid Value, Aborting".red()),
    }
}
