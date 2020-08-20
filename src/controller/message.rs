// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::cmd::serve::ApiKey;
use rocket::response::content;

#[post("/api/v1/<_project>/<_version>")]
pub fn create_project(
    api_key: ApiKey,
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}

#[delete("/api/v1/<_project>/<_version>")]
pub fn delete_project(
    api_key: ApiKey,
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}

#[get("/api/v1/<_project>/<_version>")]
pub fn get_project(
    api_key: ApiKey,
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
