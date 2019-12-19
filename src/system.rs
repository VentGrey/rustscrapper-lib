use cmd_lib::*;
use colored::Colorize;
use scanln::scanln;

pub fn clear() {
    print!("\x1B[2J");
}

/*
 * This function spawns a small system menu to perform several checks on the
 * current system that the script is running.
 */
fn sysmenu() -> u8 {
    println!("\t Please input your choice");
    println!("{}", "\t 1- View Disk Usage".yellow());
    println!("{}", "\t 2- View CPU Usage".yellow());
    println!("{}", "\t 3- View RAM Usage".yellow());
    println!("{}", "\t ----- CLEANUP -----".blue().bold());
    let input = scanln!("> ");
    let input: u8 = input.parse().unwrap();

    input
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

/*
 * This function is used to check software in order to ensure that all the
 * required software (or it's alternatives) are installed and fully working.
 */
fn check_software(kind: u8) {
    match kind {
        1 => {
            if let Ok(_result) = run_cmd!("which df") {
                clear();
                info!("{}", "df command is installed.".green());
            } else if let Err(_result) = run_cmd!("which df") {
                clear();
                die!("{}", "df was not found!".red());
            }
        }

        2 => {
            if let Ok(_result) = run_cmd!("which mpstat") {
                clear();
                info!("{}", "mpstat is installed".green());
            } else if let Err(_result) = run_cmd!("which mpstat") {
                clear();
                die!("{}", "mpstat is not present!".red());
            }
        }

        _ => panic!("{}", "Invalid Value, Aborting".red()),
    }
}

pub fn init() {

}

pub fn mainsys() {
    println!("\t System Checking Menu \t");
    let option: u8 = sysmenu();

    match option {
        1 => dsk_usg(),
        2 => cpu_usg(),
        3 => mem_usg(),
        _ => panic!("{}", "Invalid Value, Aborting".red()),
    }
}

fn mem_usg() {
    println!("{}", "Not yet available.");
}

fn cpu_usg() {
    check_software(2);
    println!(
        "{}",
        "All cpu required software is present, proceeding to check the current cpu load.\n\n'".green()
    );

    let order: &str = "mpstat | awk '$12 ~ /[0-9.]+/ { print 100 - $12 }'";
    let cpu_usg = run_fun!("{}", order);
    match cpu_usg {
        Ok(ok_command) => match ok_command.parse::<f64>().expect("Error at type conversion") {
            0.0..=30.0 => println!("{}: {}", "CPU usage is".yellow(), "Ok".green()),
            31.0..=40.0 => println!("{}: {}", "CPU usage is".yellow(), "Mildly used".yellow()),
            41.0..=60.0 => println!(
                "{}: {}\n Hint: Consider running a cleanup function after this test",
                "Disk usage is".yellow(),
                "Mostly used".yellow()
            ),
            61.0..=100.0 => println!(
                "{}: {}\n Hint: Consider terminating some processes after this test",
                "Disk usage is".yellow(),
                "Highly used".red()
            ),
            _ => println!("{}", "Unknown error".red())
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
         disk type.\n\n\n"
            .green()
    );

    // This is hardcoded for now to use sda3
    let dsk_usg = run_fun!("df --output=pcent /dev/sda3 | tr -dc '0-9'");
    match dsk_usg {
        Ok(ok_command) => match ok_command.parse::<u8>().expect("Error at type conversion") {
            0..=30 => println!("\n\n{}: {}", "Disk usage is".yellow(), "Ok".green()),
            31..=40 => println!("\n\n{}: {}", "Disk usage is".yellow(), "Mildly used".yellow()),
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
