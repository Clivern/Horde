// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use rusqlite::Connection;
use std::process;

use crate::util;

pub fn tables() {
    let rocket_config =
        util::config::get_env("ROCKET_CONFIG", util::config::get_config_path().as_str());
    let config = util::config::get_configs(rocket_config.to_string());
    let conn = Connection::open(&config.global.db).expect("Failed to open database");

    let sql = "
        CREATE TABLE IF NOT EXISTS emails (
            id                   INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid                 TEXT NOT NULL UNIQUE,
            email                VARCHAR(60) NOT NULL,
            expire               BOOLEAN DEFAULT 1,
            expire_at            DATETIME,
            inserted_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at           DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS emails_meta (
            id                   INTEGER PRIMARY KEY AUTOINCREMENT,
            key                  VARCHAR(255) NOT NULL,
            value                TEXT NOT NULL,
            email_id             INTEGER NOT NULL,
            inserted_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at           DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS messages (
            id                   INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid                 TEXT NOT NULL UNIQUE,
            `from`               VARCHAR(255) NOT NULL,
            subject              VARCHAR(255) NOT NULL,
            content              TEXT NOT NULL,
            email_id             INTEGER NOT NULL,
            inserted_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at           DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS messages_meta (
            id                   INTEGER PRIMARY KEY AUTOINCREMENT,
            key                  VARCHAR(255) NOT NULL,
            value                TEXT NOT NULL,
            message_id           INTEGER NOT NULL,
            inserted_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at           DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS attachments (
            id                   INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid                 TEXT NOT NULL UNIQUE,
            filename             VARCHAR(255) NOT NULL,
            content_type         VARCHAR(100) NOT NULL,
            size                 INTEGER NOT NULL,
            data                 BLOB NOT NULL,
            message_id           INTEGER NOT NULL,
            inserted_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at           DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS attachments_meta (
            id                   INTEGER PRIMARY KEY AUTOINCREMENT,
            key                  VARCHAR(255) NOT NULL,
            value                TEXT NOT NULL,
            attachment_id        INTEGER NOT NULL,
            inserted_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at           DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (attachment_id) REFERENCES attachments(id) ON DELETE CASCADE
        );
    ";

    match conn.execute_batch(sql) {
        Ok(_) => {
            println!("Database migrated successfully!");
        }
        Err(err) => {
            println!("Error while migration: {:?}", err);
            process::exit(1);
        }
    }
}
