use std::process::Command;
use colored::*;

<<<<<<< HEAD
fn check_os() {
    let output = if cfg!(target_os = "linux") {
        println!("Checking your Linux Distribution");
        Command::new("");
=======
fn check_platfom() -> char {
    let output = if cfg!(target_os = "linux") {

>>>>>>> d551025569a281be061d1f15564c06e85027f9b4
    } else if cfg!(target_os = "windows") {
        panic!("{}", "Configuration tool is not supported on this OS!".red());
    };
}

fn main() {

}
