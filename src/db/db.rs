// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use postgres::{Client, NoTls};
use std::process;

use crate::util;

pub fn tables() {
    let rocket_config =
        util::config::get_env("ROCKET_CONFIG", util::config::get_config_path().as_str());
    let config = util::config::get_configs(rocket_config.to_string());
    let client = Client::connect(config.global.db.as_str(), NoTls);

    match client.expect("failed to migrate").batch_execute(
        "
        CREATE  TABLE IF NOT EXISTS ho_project (
            id                   integer  NOT NULL,
            name                 varchar(100)  NOT NULL,
            vers                 varchar(100)  NOT NULL,
            created_at           timestamp  NOT NULL,
            updated_at           timestamp  NOT NULL,
            username             varchar(100)  NOT NULL,
            secret               varchar(100)  NOT NULL,
            CONSTRAINT pk_ho_project PRIMARY KEY ( id )
         );

        CREATE  TABLE IF NOT EXISTS ho_state (
            id                   integer  NOT NULL,
            name                 varchar(100),
            val                  text  NOT NULL,
            created_at           timestamp,
            updated_at           timestamp,
            project_id           integer  NOT NULL,
            CONSTRAINT state_pkey PRIMARY KEY ( id )
         );

        CREATE  TABLE IF NOT EXISTS ho_lock (
            id                   integer  NOT NULL,
            project_id           integer  NOT NULL,
            created_at           timestamp  NOT NULL,
            updated_at           timestamp  NOT NULL,
            CONSTRAINT pk_ho_lock PRIMARY KEY ( id )
         );
    ",
    ) {
        Ok(_) => {
            println!("Database migrated successfully!");
        }
        Err(err) => {
            println!("Error while migration: {:?}", err);
            process::exit(1);
        }
    }
}

pub fn changes() {
    let rocket_config =
        util::config::get_env("ROCKET_CONFIG", util::config::get_config_path().as_str());
    let config = util::config::get_configs(rocket_config.to_string());
    let client = Client::connect(config.global.db.as_str(), NoTls);

    match client.expect("failed to migrate").batch_execute(
        "
        ALTER TABLE ho_lock ADD CONSTRAINT fk_ho_lock_ho_project FOREIGN KEY ( project_id ) REFERENCES ho_project( id ) ON DELETE CASCADE ON UPDATE CASCADE;
        ALTER TABLE ho_state ADD CONSTRAINT fk_ho_state_ho_project FOREIGN KEY ( project_id ) REFERENCES ho_project( id ) ON DELETE CASCADE ON UPDATE CASCADE;
    ",
    ) {
        Ok(_) => {}
        Err(_) => {}
    }
}
