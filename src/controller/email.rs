// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::cmd::serve::BasicAuth;
use rocket::response::content;

#[put("/api/v1/<_project>/<_version>/lock")]
pub fn lock(
    basic_auth: BasicAuth,
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}

#[put("/api/v1/<_project>/<_version>/unlock")]
pub fn unlock(
    basic_auth: BasicAuth,
    _project: String,
    _version: String,
) -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
