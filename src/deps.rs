use cmd_lib::*;
use colored::Colorize;
use scanln::scanln;

use std::io::{Error, ErrorKind};


pub fn check_deps() {
    let check_cargo = run_fun!("which cargo");

    match check_cargo {
        Ok(cargo_found) => {
            println!("{} : {}", "Cargo was found on path".green(), cargo_found);
        },

        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("{}", "Cargo does not appear to be instaled!".red());
            }
        }
    }
}
