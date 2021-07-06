// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub db: String,
    pub api: String,
}

pub fn get_env(key: &str, def: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => def.to_string(),
    }
}

pub fn get_configs(path: String) -> Config {
    let mut file = File::open(path.to_owned()).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let config: Config = toml::from_str(&contents).unwrap();

    return config;
}

pub fn get_base_path() -> String {
    let exe_path = env::current_exe().unwrap();

    let package_base_path = exe_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    return format!("{}/", package_base_path.display());
}

pub fn get_config_path() -> String {
    let exe_path = env::current_exe().unwrap();

    let package_base_path = exe_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    return format!("{}/rocket.toml", package_base_path.display());
}

#[test]
fn test_get_config() {
    assert_eq!(get_env("CARGO_PKG_NAME", ""), "horde-rs");
    assert_eq!(get_env("CARGO__PKG_NAME", "default"), "default");
}
