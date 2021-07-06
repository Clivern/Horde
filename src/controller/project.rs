// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use rocket::response::content;

#[post("/api/v1/<_project>/<_version>")]
pub fn create_project(
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}

#[delete("/api/v1/<_project>/<_version>")]
pub fn delete_project(
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}

#[put("/api/v1/<_project>/<_version>")]
pub fn update_project(
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
