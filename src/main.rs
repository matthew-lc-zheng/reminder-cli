mod reminder;
mod args;

use clap::{Arg, Command};
use reminder::add_event;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}




fn main() {
    let args:Vec<String> =env::args().collect();
    let cmd=args::parse_command(&args);
    match cmd{
        args::Command::help=>args::print_help(),
        args::Command::add=>reminder::add_event(),
        args::Command::events=>reminder::all_events(),
        args::Command::finish=>reminder::finish_event(),
        args::Command::lists=>reminder::all_lists(),
        args::Command::load=>reminder::load_list(),
        args::Command::new=>reminder::create_list(),
        _=>eprintln!("Invalid command, please check the usage with 'remind -h'."),
    }
}