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


fn check_software(kind: u8) -> bool {
    match kind {
        1 => {
            // 1 is used to check disk-reffered software
            let result = Command::new("which")
                .arg("df")
                .output()
                .expect("Failed to execute which");

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
   
}
