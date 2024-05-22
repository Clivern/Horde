// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use chrono::Utc;
use rocket::serde::json::Json;
use serde_json::json;

#[derive(serde::Serialize)]
struct EmailMessage {
    id: String,
    from: String,
    to: String,
    subject: String,
    body: String,
    received_at: String,
}

#[get("/api/v1/email/<email_uuid>/message")]
pub fn get_email_messages(email_uuid: String) -> Json<serde_json::Value> {
    // For now, return mock data. In a real implementation, this would query a database
    let messages = vec![
        EmailMessage {
            id: "msg_001".to_string(),
            from: "sender@example.com".to_string(),
            to: "recipient@horde.local".to_string(),
            subject: "Welcome to Horde".to_string(),
            body: "This is a welcome message from Horde email service.".to_string(),
            received_at: Utc::now().to_rfc3339(),
        },
        EmailMessage {
            id: "msg_002".to_string(),
            from: "noreply@service.com".to_string(),
            to: "user@horde.local".to_string(),
            subject: "Account Verification".to_string(),
            body: "Please verify your account by clicking the link below.".to_string(),
            received_at: Utc::now().to_rfc3339(),
        },
    ];

    let response = json!({
        "email_uuid": email_uuid,
        "messages": messages,
        "total": messages.len(),
        "status": "success",
        "message": "Email messages retrieved successfully"
    });

    Json(response)
}
