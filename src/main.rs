use cmd_lib::{FunResult, run_cmd, run_fun};

/* System libraries */
use std::{env, process::exit};

/* self modules */
mod apt;

fn main() {
    /* TODO: Handle arguments passed in order */
    let args: Vec<String> = env::args().collect();

}
