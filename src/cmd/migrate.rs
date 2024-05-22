// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::db::client;
use crate::db::migrate;
use crate::util;

/// Migrate the database
pub fn migrate() {
    migrate::up(util::config::get_db_path().as_str());

    let client = client::Database::new(util::config::get_db_path().as_str());

    match client {
        Ok(client) => {
            let domains = util::config::get_domains();
            let random_email = client.get_random_email(&domains);
            println!("Random email: {:?}", random_email);

            /*
            let result = client.create_email(
                &random_email,
                true,
                Some(chrono::Utc::now() + chrono::Duration::days(1)),
            );
            println!("Email created successfully: {:?}", result);
            */
        }
        Err(err) => {
            println!("Error while connecting to database: {:?}", err);
        }
    }
}
