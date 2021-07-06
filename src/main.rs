// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

#![feature(proc_macro_hygiene, decl_macro)]

mod cmd;
mod controller;

use clap::Command;

#[macro_use]
extern crate rocket;

fn main() {
    let matches = Command::new("🐺 Horde")
        .about("A Fast, Secure and Reliable Terraform Backend, Set up in Minutes")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Clivern")
        .subcommand(
            Command::new("serve")
                .short_flag('s')
                .long_flag("serve")
                .about("Serve the application"),
        )
        .get_matches();

    match matches.subcommand() {
        // serve command
        Some(("serve", _sub_matches)) => cmd::serve::serve(),
        _ => unreachable!(),
    }
}
