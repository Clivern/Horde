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
        CREATE TABLE IF NOT EXISTS emails (
            id                   SERIAL PRIMARY KEY,
            uuid                 UUID NOT NULL UNIQUE,
            email                VARCHAR(60) NOT NULL,
            expire               BOOLEAN DEFAULT true,
            expire_at            TIMESTAMP,
            inserted_at          TIMESTAMP DEFAULT NOW(),
            updated_at           TIMESTAMP DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS emails_meta (
            id                   SERIAL PRIMARY KEY,
            key                  VARCHAR(255) NOT NULL,
            value                TEXT NOT NULL,
            email_id             INTEGER NOT NULL,
            inserted_at          TIMESTAMP DEFAULT NOW(),
            updated_at           TIMESTAMP DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS messages (
            id                   SERIAL PRIMARY KEY,
            uuid                 UUID NOT NULL UNIQUE,
            from                 VARCHAR(255) NOT NULL,
            subject              VARCHAR(255) NOT NULL,
            content              TEXT NOT NULL,
            email_id             INTEGER NOT NULL,
            inserted_at          TIMESTAMP DEFAULT NOW(),
            updated_at           TIMESTAMP DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS messages_meta (
            id                   SERIAL PRIMARY KEY,
            key                  VARCHAR(255) NOT NULL,
            value                TEXT NOT NULL,
            message_id           INTEGER NOT NULL,
            inserted_at          TIMESTAMP DEFAULT NOW(),
            updated_at           TIMESTAMP DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS attachments (
            id                   SERIAL PRIMARY KEY,
            uuid                 UUID NOT NULL UNIQUE,
            filename             VARCHAR(255) NOT NULL,
            content_type         VARCHAR(100) NOT NULL,
            size                 BIGINT NOT NULL,
            data                 BYTEA NOT NULL,
            message_id           INTEGER NOT NULL,
            inserted_at          TIMESTAMP DEFAULT NOW(),
            updated_at           TIMESTAMP DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS attachments_meta (
            id                   SERIAL PRIMARY KEY,
            key                  VARCHAR(255) NOT NULL,
            value                TEXT NOT NULL,
            attachment_id        INTEGER NOT NULL,
            inserted_at          TIMESTAMP DEFAULT NOW(),
            updated_at           TIMESTAMP DEFAULT NOW()
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
        ALTER TABLE emails_meta ADD CONSTRAINT fk_emails_meta_email_id FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE ON UPDATE CASCADE;
        ALTER TABLE messages ADD CONSTRAINT fk_messages_email_id FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE ON UPDATE CASCADE;
        ALTER TABLE messages_meta ADD CONSTRAINT fk_messages_meta_message_id FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE ON UPDATE CASCADE;
        ALTER TABLE attachments ADD CONSTRAINT fk_attachments_message_id FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE ON UPDATE CASCADE;
        ALTER TABLE attachments_meta ADD CONSTRAINT fk_attachments_meta_attachment_id FOREIGN KEY (attachment_id) REFERENCES attachments(id) ON DELETE CASCADE ON UPDATE CASCADE;
    ",
    ) {
        Ok(_) => {}
        Err(_) => {}
    }
}
