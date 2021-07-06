// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use rocket::response::content;

#[get("/api/v1/tf")]
pub fn tfv1() -> content::Json<&'static str> {
    content::Json("{\"status\":\"ok\"}")
}
