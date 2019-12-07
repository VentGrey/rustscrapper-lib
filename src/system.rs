use cmd_lib::*;
use scanln::scanln;
use colored::Colorize;

/*
 * This function spawns a small system menu to perform several checks on the
 * current system that the script is running.
 */
fn sysmenu() -> u8 {
    println!("\t Please input your choice");
    println!("{}", "\t 1- View Disk Usage".yellow());
    println!("{}", "\t 2- View CPU Usage".yellow());
    let input = scanln!("> ");
    let input: u8 = input.parse().unwrap();

    input
}

/*
 * This function is used to check software in order to ensure that all the
 * required software (or it's alternatives) are installed and fully working.
 */
fn check_software(kind: u8) {
    match kind {
        1 => {
            if let Ok(_result) = run_cmd!("which df") {
                info!("{}", "df command is installed.".green());
            } else if let Err(_result) = run_cmd!("which df") {
                die!("{}", "df was not found!".red());
            }                      
        }

        _ => panic!("{}", "Invalid Value, Aborting".red())
    }
}


pub fn mainsys() {
    println!("\t System Checking Menu \t");
    let option: u8 = sysmenu();

    match option {
        1 => dsk_usg(),
        _ => panic!("{}", "Invalid Value, Aborting".red())
    }
}

fn dsk_usg() {
    check_software(1);
    println!("{}", "All disk management software is present, proceeding to check the \
              disk type.".green());
}
