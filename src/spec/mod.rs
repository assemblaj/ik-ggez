use std::fs;

pub mod cmd;
pub mod cns;
pub mod constants;
pub mod controllers;
pub(crate) mod def;
pub mod state;
pub mod triggers;

use std::collections::HashMap;

pub(crate) fn test() {
    let data = std::fs::read_to_string("./resources/kfm720.cmd").expect("error");
    // let ini = parse_cns(data.as_str());

    // dbg!(ini);
    /*let command_str = "~60$B, F, z";
    match command_str.parse::<cmd::Command>() {
        Ok(command) => println!("{:?}", command),
        Err(err) => println!("Failed to parse command: {}", err),
    }*/
}
