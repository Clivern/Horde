// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

mod cmd;
mod controller;
mod db;
mod util;

use clap::Command;

#[macro_use]
extern crate rocket;

fn main() {
    let matches = Command::new("🐺 Horde")
        .about("A Disposable, Secure, Anonymous Email Server, Set up in Minutes")
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
        .subcommand(
            Command::new("migrate")
                .short_flag('m')
                .long_flag("migrate")
                .about("Migrate the database"),
        )
        .get_matches();

    match matches.subcommand() {
        // serve command
        Some(("serve", _sub_matches)) => {
            tokio::runtime::Runtime::new().unwrap().block_on(cmd::app::serve());
        },

        // migrate command
        Some(("migrate", _sub_matches)) => cmd::migrate::migrate(),
        _ => unreachable!(),
    }
}
