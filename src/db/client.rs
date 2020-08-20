// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::util;
use postgres::{Client, NoTls, Row};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Email {
    pub id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub expire: bool,
    pub expire_at: Option<chrono::DateTime<chrono::Utc>>,
    pub inserted_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct Message {
    pub id: i32,
    pub uuid: Uuid,
    pub from: String,
    pub subject: String,
    pub content: String,
    pub email_id: i32,
    pub inserted_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct Attachment {
    pub id: i32,
    pub uuid: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub data: Vec<u8>,
    pub message_id: i32,
    pub inserted_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new() -> Result<Database, String> {
        let rocket_config = util::config::get_env(
            "ROCKET_CONFIG",
            util::config::get_config_path().as_str(),
        );

        let config = util::config::get_configs(rocket_config.to_string());

        match Client::connect(config.global.db.as_str(), NoTls) {
            Ok(client) => Ok(Database { client: client }),
            Err(_err) => Err(String::from("Unable to establish database connection!")),
        }
    }

    // Email operations
    pub fn create_email(
        &self,
        email: &str,
        expire: bool,
        expire_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Email, String> {
        let uuid = Uuid::new_v4();
        let query = "INSERT INTO emails (uuid, email, expire, expire_at) VALUES ($1, $2, $3, $4) RETURNING *";

        match self
            .client
            .query_one(query, &[&uuid, &email, &expire, &expire_at])
        {
            Ok(row) => Ok(self.row_to_email(row)),
            Err(e) => Err(format!("Failed to create email: {}", e)),
        }
    }

    pub fn get_email_by_uuid(&self, uuid: &Uuid) -> Result<Option<Email>, String> {
        let query = "SELECT * FROM emails WHERE uuid = $1";

        match self.client.query_opt(query, &[uuid]) {
            Ok(Some(row)) => Ok(Some(self.row_to_email(row))),
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Failed to get email: {}", e)),
        }
    }

    pub fn get_email_by_address(&self, email: &str) -> Result<Option<Email>, String> {
        let query = "SELECT * FROM emails WHERE email = $1";

        match self.client.query_opt(query, &[&email]) {
            Ok(Some(row)) => Ok(Some(self.row_to_email(row))),
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Failed to get email: {}", e)),
        }
    }

    pub fn delete_email(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM emails WHERE uuid = $1";

        match self.client.execute(query, &[uuid]) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete email: {}", e)),
        }
    }

    // Message operations
    pub fn create_message(
        &self,
        from: &str,
        subject: &str,
        content: &str,
        email_id: i32,
    ) -> Result<Message, String> {
        let uuid = Uuid::new_v4();
        let query = "INSERT INTO messages (uuid, from, subject, content, email_id) VALUES ($1, $2, $3, $4, $5) RETURNING *";

        match self
            .client
            .query_one(query, &[&uuid, &from, &subject, &content, &email_id])
        {
            Ok(row) => Ok(self.row_to_message(row)),
            Err(e) => Err(format!("Failed to create message: {}", e)),
        }
    }

    pub fn get_message_by_uuid(&self, uuid: &Uuid) -> Result<Option<Message>, String> {
        let query = "SELECT * FROM messages WHERE uuid = $1";

        match self.client.query_opt(query, &[uuid]) {
            Ok(Some(row)) => Ok(Some(self.row_to_message(row))),
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Failed to get message: {}", e)),
        }
    }

    pub fn get_messages_by_email_id(
        &self,
        email_id: i32,
    ) -> Result<Vec<Message>, String> {
        let query =
            "SELECT * FROM messages WHERE email_id = $1 ORDER BY inserted_at DESC";

        match self.client.query(query, &[&email_id]) {
            Ok(rows) => Ok(rows
                .into_iter()
                .map(|row| self.row_to_message(row))
                .collect()),
            Err(e) => Err(format!("Failed to get messages: {}", e)),
        }
    }

    pub fn delete_message(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM messages WHERE uuid = $1";

        match self.client.execute(query, &[uuid]) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete message: {}", e)),
        }
    }

    // Attachment operations
    pub fn create_attachment(
        &self,
        filename: &str,
        content_type: &str,
        data: &[u8],
        message_id: i32,
    ) -> Result<Attachment, String> {
        let uuid = Uuid::new_v4();
        let size = data.len() as i64;
        let query = "INSERT INTO attachments (uuid, filename, content_type, size, data, message_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";

        match self.client.query_one(
            query,
            &[&uuid, &filename, &content_type, &size, &data, &message_id],
        ) {
            Ok(row) => Ok(self.row_to_attachment(row)),
            Err(e) => Err(format!("Failed to create attachment: {}", e)),
        }
    }

    pub fn get_attachment_by_uuid(
        &self,
        uuid: &Uuid,
    ) -> Result<Option<Attachment>, String> {
        let query = "SELECT * FROM attachments WHERE uuid = $1";

        match self.client.query_opt(query, &[uuid]) {
            Ok(Some(row)) => Ok(Some(self.row_to_attachment(row))),
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Failed to get attachment: {}", e)),
        }
    }

    pub fn get_attachments_by_message_id(
        &self,
        message_id: i32,
    ) -> Result<Vec<Attachment>, String> {
        let query =
            "SELECT * FROM attachments WHERE message_id = $1 ORDER BY inserted_at ASC";

        match self.client.query(query, &[&message_id]) {
            Ok(rows) => Ok(rows
                .into_iter()
                .map(|row| self.row_to_attachment(row))
                .collect()),
            Err(e) => Err(format!("Failed to get attachments: {}", e)),
        }
    }

    pub fn delete_attachment(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM attachments WHERE uuid = $1";

        match self.client.execute(query, &[uuid]) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete attachment: {}", e)),
        }
    }

    // Metadata operations
    pub fn set_email_meta(
        &self,
        email_id: i32,
        key: &str,
        value: &str,
    ) -> Result<(), String> {
        let query = "INSERT INTO emails_meta (email_id, key, value) VALUES ($1, $2, $3) ON CONFLICT (email_id, key) DO UPDATE SET value = $3, updated_at = NOW()";

        match self.client.execute(query, &[&email_id, &key, &value]) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to set email metadata: {}", e)),
        }
    }

    pub fn get_email_meta(
        &self,
        email_id: i32,
    ) -> Result<HashMap<String, String>, String> {
        let query = "SELECT key, value FROM emails_meta WHERE email_id = $1";

        match self.client.query(query, &[&email_id]) {
            Ok(rows) => {
                let mut meta = HashMap::new();
                for row in rows {
                    let key: String = row.get("key");
                    let value: String = row.get("value");
                    meta.insert(key, value);
                }
                Ok(meta)
            }
            Err(e) => Err(format!("Failed to get email metadata: {}", e)),
        }
    }

    // Helper methods to convert rows to structs
    fn row_to_email(&self, row: Row) -> Email {
        Email {
            id: row.get("id"),
            uuid: row.get("uuid"),
            email: row.get("email"),
            expire: row.get("expire"),
            expire_at: row.get("expire_at"),
            inserted_at: row.get("inserted_at"),
            updated_at: row.get("updated_at"),
        }
    }

    fn row_to_message(&self, row: Row) -> Message {
        Message {
            id: row.get("id"),
            uuid: row.get("uuid"),
            from: row.get("from"),
            subject: row.get("subject"),
            content: row.get("content"),
            email_id: row.get("email_id"),
            inserted_at: row.get("inserted_at"),
            updated_at: row.get("updated_at"),
        }
    }

    fn row_to_attachment(&self, row: Row) -> Attachment {
        Attachment {
            id: row.get("id"),
            uuid: row.get("uuid"),
            filename: row.get("filename"),
            content_type: row.get("content_type"),
            size: row.get("size"),
            data: row.get("data"),
            message_id: row.get("message_id"),
            inserted_at: row.get("inserted_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
