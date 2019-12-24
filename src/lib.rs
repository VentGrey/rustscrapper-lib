// This is the main module for rustscrapper
use cmd_lib::run_fun;
use cmd_lib::FunResult;

pub fn bin_exists(bin: &str) -> FunResult {
    let result = run_fun!("which {}", bin);
    result
}
