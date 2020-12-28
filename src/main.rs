// This is the main module for rustscrapper
use cmd_lib::run_fun;
use cmd_lib::FunResult;
use cmd_lib::run_cmd;

pub fn bin_exists(bin: &str) -> FunResult {
    let result = run_fun!("which {}", bin);
    result
}

pub fn exec_status(process: &str) -> i32 {
    let result = run_cmd!(process);

    let result = match result {
        Ok(_) => {
            0
        }
        Err(e) => {
            println!("Process failed to execute at {}", e);
            1
        }
    };

    // If matching fails we'll assume everyting went ok
    0

}
