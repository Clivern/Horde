// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use rocket::response::content;

#[get("/api/v1/<_project>/<_version>/state")]
pub fn get_state(_project: String, _version: String) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}

#[post("/api/v1/<_project>/<_version>/state")]
pub fn update_state(_project: String, _version: String) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}

#[delete("/api/v1/<_project>/<_version>/state")]
pub fn delete_state(_project: String, _version: String) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
