// Copyright 2025 Clivern. All rights reserved.
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
            Command::new("app")
                .short_flag('a')
                .long_flag("app")
                .about("Run the application"),
        )
        .subcommand(
            Command::new("migrate")
                .short_flag('m')
                .long_flag("migrate")
                .about("Migrate the database"),
        )
        .subcommand(
            Command::new("smtp")
                .short_flag('s')
                .long_flag("smtp")
                .about("Run the SMTP server"),
        )
        .get_matches();

    match matches.subcommand() {
        // app command
        Some(("app", _sub_matches)) => {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(cmd::app::run());
        }

        // migrate command
        Some(("migrate", _sub_matches)) => cmd::migrate::migrate(),

        // smtp command
        Some(("smtp", _sub_matches)) => cmd::smtp::run(),
        _ => unreachable!(),
    }
}
