// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use rocket::serde::json::Json;
use serde_json::json;
use uuid::Uuid;

#[post("/api/v1/email")]
pub fn generate_random_email() -> Json<serde_json::Value> {
    let random_id = Uuid::new_v4();
    let email = format!("{}@horde.local", random_id);

    let response = json!({
        "email": email,
        "status": "success",
        "message": "Random email generated successfully"
    });

    Json(response)
}
