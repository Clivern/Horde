// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::cmd::serve::BasicAuth;
use rocket::response::content;

#[get("/api/v1/<_project>/<_version>/state")]
pub fn get_state(
    basic_auth: BasicAuth,
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    println!("{:?}", basic_auth);
    content::Json("{\"status\":\"ok\"}")
}

#[post("/api/v1/<_project>/<_version>/state")]
pub fn update_state(
    basic_auth: BasicAuth,
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}

#[delete("/api/v1/<_project>/<_version>/state")]
pub fn delete_state(
    basic_auth: BasicAuth,
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
