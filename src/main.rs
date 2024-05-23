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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
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
            Command::new("truncate")
                .short_flag('t')
                .long_flag("truncate")
                .about("Truncate the database"),
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
            cmd::app::run().await;
        }

        // migrate command
        Some(("migrate", _sub_matches)) => cmd::migrate::migrate(),

        // truncate command
        Some(("truncate", _sub_matches)) => cmd::truncate::truncate(),

        // smtp command
        Some(("smtp", _sub_matches)) => cmd::smtp::run(),
        _ => unreachable!(),
    }
    Ok(())
}
