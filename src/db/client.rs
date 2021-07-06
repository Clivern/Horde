// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::util;
use postgres::{Client, NoTls};

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new() -> Result<Database, String> {
        let rocket_config = util::config::get_env(
            "ROCKET_CONFIG",
            util::config::get_config_path().as_str(),
        );

        let config = util::config::get_configs(rocket_config.to_string());

        match Client::connect(config.global.db.as_str(), NoTls) {
            Ok(client) => Ok(Database { client: client }),
            Err(_err) => Err(String::from("Unable to establish database connection!")),
        }
    }

    pub fn get_project(&self, _project: String, _version: String) {}
    pub fn create_project(
        &self,
        _project: String,
        _version: String,
    ) -> Result<u64, String> {
    }
    pub fn delete_project(&self, _project: String, _version: String) {}

    pub fn lock(&self, _project: String, _version: String) {}
    pub fn unlock(&self, _project: String, _version: String) {}

    pub fn get_state(&self, _project: String, _version: String) {}
    pub fn update_state(&self, _project: String, _version: String) {}
    pub fn delete_state(&self, _project: String, _version: String) {}
}
