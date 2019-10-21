use cmd_lib::*;
use scanln::scanln;
use colored::Colorize;
use std::process::Command;

fn sysmenu() -> u8 {
    println!("\t Please input your choice");
    println!("\t 1- View Disk Usage");
    println!("\t 2- View CPU Usage");
    let input = scanln!("> ");
    let input: u8 = input.parse().unwrap();

    return input
}


fn check_software(kind: u8) {
    match kind {
        1 => {
            if let Ok(result) = run_cmd("which df") {
                info!("df command is installed.");
            } else if let Err(result) = run_cmd("which df") {
                die!("df was not found!");
            }
        },

        2 => {

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
}
