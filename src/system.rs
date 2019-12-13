use cmd_lib::*;
use colored::Colorize;
use scanln::scanln;

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

        2 => {
            if let Ok(_result) = run_cmd!("which mpstat") {
                info!("{}", "mpstat is installed".green());
            } else if let Err(_result) = run_cmd!("which mpstat") {
                die!("{}", "mpstat is not present!".red());
            }
        }

        _ => panic!("{}", "Invalid Value, Aborting".red()),
    }
}

pub fn mainsys() {
    println!("\t System Checking Menu \t");
    let option: u8 = sysmenu();

    match option {
        1 => dsk_usg(),
        2 => cpu_usg(),
        _ => panic!("{}", "Invalid Value, Aborting".red()),
    }
}

fn cpu_usg() {
    check_software(2);
    println!(
        "{}",
        "All cpu required software is present, proceeding to check the current cpu load.".green()
    );

    let cpu_usg = run_fun!("mpstat | awk '$12 ~ /[0-9.]+/ { print 100 - $12 }'");
    match cpu_usg {
        Ok(ok_command) => match ok_command.parse::<u8>().expect("Error at type conversion") {
            0..=30 => println!("{}: {}", "CPU usage is".yellow(), "Ok".green()),
            31..=40 => println!("{}: {}", "CPU usage is".yellow(), "Mildly used".yellow()),
            41..=60 => println!(
                "{}: {}\n Hint: Consider running a cleanup function after this test",
                "Disk usage is".yellow(),
                "Mostly used".yellow()
            ),
            61..=100 => println!(
                "{}: {}\n Hint: Consider terminating some processes after this test",
                "Disk usage is".yellow(),
                "Highly used".red()
            ),
        },
        Err(e) => {
            eprint!("Error in executing command, failed at: {}", e);
        }
    }
}

fn dsk_usg() {
    check_software(1);
    println!(
        "{}",
        "All disk required software is present, proceeding to check the \
         disk type."
            .green()
    );

    // This is hardcoded for now to use sda3
    let dsk_usg = run_fun!("df --output=pcent /dev/sda3 | tr -dc '0-9'");
    match dsk_usg {
        Ok(ok_command) => match ok_command.parse::<u8>().expect("Error at type conversion") {
            0..=30 => println!("{}: {}", "Disk usage is".yellow(), "Ok".green()),
            31..=40 => println!("{}: {}", "Disk usage is".yellow(), "Mildly used".yellow()),
            41..=60 => println!(
                "{}: {}\n Hint: Consider running a cleanup function after this test",
                "Disk usage is".yellow(),
                "Mostly used".yellow()
            ),
            61..=70 => println!(
                "{}: {}\n Hint: Consider running a cleanup function after this test",
                "Disk usage is".yellow(),
                "Highly used".red()
            ),
            71..=80 => println!(
                "{}: {}\n Hint: Consider running a cleanup function after this test",
                "Disk usage is".yellow(),
                "Low capacity".red()
            ),
            81..=90 => println!(
                "{}: {}\n Hint: Consider running a cleanup function after this test",
                "Disk usage is".yellow(),
                "Almost full, writing may fail at this point".red()
            ),
            91..=100 => println!(
                "{}: {}\n Hint: Consider running a cleanup function after this test",
                "Disk usage is".yellow(),
                "Almost full, writing will fail at this point".red()
            ),
            _ => println!("{}", "Unknown Error".red()),
        },
        Err(e) => {
            eprint!("Error in executing command, failed at: {}", e);
        }
    }
}
