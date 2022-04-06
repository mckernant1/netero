#![allow(dead_code)]
extern crate clap;

use crate::clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::fs;
use crate::args::args::Cli;

#[path = "src/args/mod.rs"]
mod args;

fn main() {
    let mut command = Cli::command();
    fs::create_dir_all("completions").unwrap();
    generate_to(Shell::Zsh, &mut command, "netero", "completions").unwrap();
    generate_to(Shell::Bash, &mut command, "netero", "completions").unwrap();
}
