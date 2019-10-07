use scanln::scanln;
use colored::Colorize;

fn sysmenu() -> u8 {
    println!("\t Please input your choice");
    println!("\t 1- View Disk Usage");
    println!("\t 2- View CPU Usage");
    let input = scanln!("> ");
    let input: u8 = input.parse().unwrap();

    return input
}

fn dsk_usg() {
    println!("{}","Checking for needed software...".blink())
}

fn check_software(kind: u8) {

}



pub fn mainsys() {
    println!("\t System Checking Menu \t");
    let option: u8 = sysmenu();

    match option {
        1 => dsk_usg(),
        _ => panic!("{}", "Invalid Value, Aborting".red())
    }
}
