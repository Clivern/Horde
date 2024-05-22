// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::db::client;
use crate::util;
use rocket::serde::json::Json;
use serde_json::json;

#[post("/api/v1/email")]
pub fn generate_random_email() -> Json<serde_json::Value> {
    let client = client::Database::new(util::config::get_db_path().as_str());

    match client {
        Ok(client) => {
            let email_result = client.get_random_email(&util::config::get_domains());

            match email_result {
                Ok(email) => {
                    let record = client.create_email(
                        &email,
                        true,
                        Some(chrono::Utc::now() + chrono::Duration::days(1)),
                    );

                    match record {
                        Ok(email) => {
                            return Json(json!({
                                "id": email.id,
                                "uuid": email.uuid.to_string(),
                                "email": email.email.as_str(),
                                "expire": email.expire,
                                "expire_at": email.expire_at.unwrap().to_rfc3339(),
                                "inserted_at": email.inserted_at.to_rfc3339(),
                                "updated_at": email.updated_at.to_rfc3339(),
                            }));
                        }
                        Err(_) => {
                            return Json(json!({
                                "errorMessage": "Error! Internal server error"
                            }));
                        }
                    }
                }
                Err(_) => {
                    return Json(json!({
                        "errorMessage": "Error! Internal server error"
                    }));
                }
            }
        }
        Err(_) => {
            return Json(json!({
                "errorMessage": "Error!Internal server error"
            }));
        }
    }
}
