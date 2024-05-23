// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::db::client;
use crate::util;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde_json::json;

#[post("/api/v1/email")]
pub fn generate_random_email() -> Result<
    status::Custom<Json<serde_json::Value>>,
    status::Custom<Json<serde_json::Value>>,
> {
    let domains = util::config::get_domains();

    if domains.is_empty() {
        return Err(status::Custom(
            Status::BadRequest,
            Json(json!({ "errorMessage": "No domains configured" })),
        ));
    }

    let internal_error = || {
        status::Custom(
            Status::InternalServerError,
            Json(json!({ "errorMessage": "Error! Internal server error" })),
        )
    };

    let client = client::Database::new(util::config::get_db_path().as_str())
        .map_err(|_| internal_error())?;

    let email = client
        .get_random_email(&domains)
        .map_err(|_| internal_error())?;

    let record = client
        .create_email(
            &email,
            true,
            Some(chrono::Utc::now() + chrono::Duration::days(1)),
        )
        .map_err(|_| internal_error())?
        .unwrap();

    Ok(status::Custom(
        Status::Created,
        Json(json!({
            "id": record.id,
            "uuid": record.uuid.to_string(),
            "email": record.email,
            "expire": record.expire,
            "expireAt": record.expire_at.map(|dt| dt.to_rfc3339()),
            "insertedAt": record.inserted_at.to_rfc3339(),
            "updatedAt": record.updated_at.to_rfc3339(),
        })),
    ))
}
