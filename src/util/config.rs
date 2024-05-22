// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub global: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub db: String,
    pub domains: Vec<String>,
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

pub fn get_db_path() -> String {
    let rocket_config = get_env("ROCKET_CONFIG", get_config_path().as_str());
    let config = get_configs(rocket_config.to_string());

    return config.global.db;
}

pub fn get_domains() -> Vec<String> {
    let rocket_config = get_env("ROCKET_CONFIG", get_config_path().as_str());
    let config = get_configs(rocket_config.to_string());

    return config.global.domains;
}

#[test]
fn test_get_config() {
    assert_eq!(get_env("CARGO_PKG_NAME", ""), "horde-rs");
    assert_eq!(get_env("CARGO__PKG_NAME", "default"), "default");
}
