// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use rusqlite::Connection;
use std::process;

/// Migrate the database
pub fn up(db_path: &str) {
    let conn = Connection::open(db_path).expect("Failed to open database");

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
            path                 TEXT NOT NULL,
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

/// Downgrade the database
pub fn down(db_path: &str) {
    let conn = Connection::open(db_path).expect("Failed to open database");

    let sql = "DROP TABLE IF EXISTS emails;
        DROP TABLE IF EXISTS emails_meta;
        DROP TABLE IF EXISTS messages;
        DROP TABLE IF EXISTS messages_meta;
        DROP TABLE IF EXISTS attachments;
        DROP TABLE IF EXISTS attachments_meta;
    ";

    match conn.execute_batch(sql) {
        Ok(_) => {
            println!("Database downgraded successfully!");
        }
        Err(err) => {
            println!("Error while downgrading: {:?}", err);
            process::exit(1);
        }
    }
}
