// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::util;
use rusqlite::{params, Connection, Row};
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
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Database, String> {
        let rocket_config = util::config::get_env(
            "ROCKET_CONFIG",
            util::config::get_config_path().as_str(),
        );

        let config = util::config::get_configs(rocket_config.to_string());

        match Connection::open(&config.global.db) {
            Ok(conn) => {
                // Enable foreign key constraints
                conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
                Ok(Database { conn })
            }
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
        let query = "INSERT INTO emails (uuid, email, expire, expire_at) VALUES (?1, ?2, ?3, ?4)";

        match self
            .conn
            .execute(query, params![uuid.to_string(), email, expire, expire_at])
        {
            Ok(_) => {
                // Get the last inserted row
                let id = self.conn.last_insert_rowid() as i32;
                self.get_email_by_id(id)
            }
            Err(e) => Err(format!("Failed to create email: {}", e)),
        }
    }

    fn get_email_by_id(&self, id: i32) -> Result<Email, String> {
        let query = "SELECT * FROM emails WHERE id = ?1";

        match self
            .conn
            .query_row(query, params![id], |row| self.row_to_email(row))
        {
            Ok(email) => Ok(email),
            Err(e) => Err(format!("Failed to get email: {}", e)),
        }
    }

    pub fn get_email_by_uuid(&self, uuid: &Uuid) -> Result<Option<Email>, String> {
        let query = "SELECT * FROM emails WHERE uuid = ?1";

        match self
            .conn
            .query_row(query, params![uuid.to_string()], |row| {
                self.row_to_email(row)
            }) {
            Ok(email) => Ok(Some(email)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get email: {}", e)),
        }
    }

    pub fn get_email_by_address(&self, email: &str) -> Result<Option<Email>, String> {
        let query = "SELECT * FROM emails WHERE email = ?1";

        match self
            .conn
            .query_row(query, params![email], |row| self.row_to_email(row))
        {
            Ok(email) => Ok(Some(email)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get email: {}", e)),
        }
    }

    pub fn delete_email(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM emails WHERE uuid = ?1";

        match self.conn.execute(query, params![uuid.to_string()]) {
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
        let query = "INSERT INTO messages (uuid, `from`, subject, content, email_id) VALUES (?1, ?2, ?3, ?4, ?5)";

        match self.conn.execute(
            query,
            params![uuid.to_string(), from, subject, content, email_id],
        ) {
            Ok(_) => {
                let id = self.conn.last_insert_rowid() as i32;
                self.get_message_by_id(id)
            }
            Err(e) => Err(format!("Failed to create message: {}", e)),
        }
    }

    fn get_message_by_id(&self, id: i32) -> Result<Message, String> {
        let query = "SELECT * FROM messages WHERE id = ?1";

        match self
            .conn
            .query_row(query, params![id], |row| self.row_to_message(row))
        {
            Ok(message) => Ok(message),
            Err(e) => Err(format!("Failed to get message: {}", e)),
        }
    }

    pub fn get_message_by_uuid(&self, uuid: &Uuid) -> Result<Option<Message>, String> {
        let query = "SELECT * FROM messages WHERE uuid = ?1";

        match self
            .conn
            .query_row(query, params![uuid.to_string()], |row| {
                self.row_to_message(row)
            }) {
            Ok(message) => Ok(Some(message)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get message: {}", e)),
        }
    }

    pub fn get_messages_by_email_id(
        &self,
        email_id: i32,
    ) -> Result<Vec<Message>, String> {
        let query =
            "SELECT * FROM messages WHERE email_id = ?1 ORDER BY inserted_at DESC";

        let mut stmt = match self.conn.prepare(query) {
            Ok(stmt) => stmt,
            Err(e) => return Err(format!("Failed to prepare query: {}", e)),
        };

        let message_iter =
            match stmt.query_map(params![email_id], |row| self.row_to_message(row)) {
                Ok(iter) => iter,
                Err(e) => return Err(format!("Failed to execute query: {}", e)),
            };

        let mut messages = Vec::new();
        for message in message_iter {
            match message {
                Ok(msg) => messages.push(msg),
                Err(e) => return Err(format!("Failed to process message: {}", e)),
            }
        }

        Ok(messages)
    }

    pub fn delete_message(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM messages WHERE uuid = ?1";

        match self.conn.execute(query, params![uuid.to_string()]) {
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
        let size = data.len() as i32;
        let query = "INSERT INTO attachments (uuid, filename, content_type, size, data, message_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

        match self.conn.execute(
            query,
            params![
                uuid.to_string(),
                filename,
                content_type,
                size,
                data,
                message_id
            ],
        ) {
            Ok(_) => {
                let id = self.conn.last_insert_rowid() as i32;
                self.get_attachment_by_id(id)
            }
            Err(e) => Err(format!("Failed to create attachment: {}", e)),
        }
    }

    fn get_attachment_by_id(&self, id: i32) -> Result<Attachment, String> {
        let query = "SELECT * FROM attachments WHERE id = ?1";

        match self
            .conn
            .query_row(query, params![id], |row| self.row_to_attachment(row))
        {
            Ok(attachment) => Ok(attachment),
            Err(e) => Err(format!("Failed to get attachment: {}", e)),
        }
    }

    pub fn get_attachment_by_uuid(
        &self,
        uuid: &Uuid,
    ) -> Result<Option<Attachment>, String> {
        let query = "SELECT * FROM attachments WHERE uuid = ?1";

        match self
            .conn
            .query_row(query, params![uuid.to_string()], |row| {
                self.row_to_attachment(row)
            }) {
            Ok(attachment) => Ok(Some(attachment)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get attachment: {}", e)),
        }
    }

    pub fn get_attachments_by_message_id(
        &self,
        message_id: i32,
    ) -> Result<Vec<Attachment>, String> {
        let query =
            "SELECT * FROM attachments WHERE message_id = ?1 ORDER BY inserted_at ASC";

        let mut stmt = match self.conn.prepare(query) {
            Ok(stmt) => stmt,
            Err(e) => return Err(format!("Failed to prepare query: {}", e)),
        };

        let attachment_iter = match stmt
            .query_map(params![message_id], |row| self.row_to_attachment(row))
        {
            Ok(iter) => iter,
            Err(e) => return Err(format!("Failed to execute query: {}", e)),
        };

        let mut attachments = Vec::new();
        for attachment in attachment_iter {
            match attachment {
                Ok(att) => attachments.push(att),
                Err(e) => return Err(format!("Failed to process attachment: {}", e)),
            }
        }

        Ok(attachments)
    }

    pub fn delete_attachment(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM attachments WHERE uuid = ?1";

        match self.conn.execute(query, params![uuid.to_string()]) {
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
        // First try to update existing record
        let update_query = "UPDATE emails_meta SET value = ?1, updated_at = CURRENT_TIMESTAMP WHERE email_id = ?2 AND key = ?3";

        match self
            .conn
            .execute(update_query, params![value, email_id, key])
        {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    return Ok(());
                }
            }
            Err(e) => return Err(format!("Failed to update email metadata: {}", e)),
        }

        // If no rows were affected, insert new record
        let insert_query =
            "INSERT INTO emails_meta (email_id, key, value) VALUES (?1, ?2, ?3)";

        match self
            .conn
            .execute(insert_query, params![email_id, key, value])
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to insert email metadata: {}", e)),
        }
    }

    pub fn get_email_meta(
        &self,
        email_id: i32,
    ) -> Result<HashMap<String, String>, String> {
        let query = "SELECT key, value FROM emails_meta WHERE email_id = ?1";

        let mut stmt = match self.conn.prepare(query) {
            Ok(stmt) => stmt,
            Err(e) => return Err(format!("Failed to prepare query: {}", e)),
        };

        let meta_iter = match stmt.query_map(params![email_id], |row| {
            let key: String = row.get("key")?;
            let value: String = row.get("value")?;
            Ok((key, value))
        }) {
            Ok(iter) => iter,
            Err(e) => return Err(format!("Failed to execute query: {}", e)),
        };

        let mut meta = HashMap::new();
        for item in meta_iter {
            match item {
                Ok((key, value)) => {
                    meta.insert(key, value);
                }
                Err(e) => return Err(format!("Failed to process metadata: {}", e)),
            }
        }

        Ok(meta)
    }

    // Helper methods to convert rows to structs
    fn row_to_email(&self, row: &Row) -> Result<Email, rusqlite::Error> {
        Ok(Email {
            id: row.get("id")?,
            uuid: Uuid::parse_str(&row.get::<_, String>("uuid")?).map_err(|_e| {
                rusqlite::Error::InvalidColumnType(
                    0,
                    "uuid".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?,
            email: row.get("email")?,
            expire: row.get("expire")?,
            expire_at: row.get("expire_at")?,
            inserted_at: row.get("inserted_at")?,
            updated_at: row.get("updated_at")?,
        })
    }

    fn row_to_message(&self, row: &Row) -> Result<Message, rusqlite::Error> {
        Ok(Message {
            id: row.get("id")?,
            uuid: Uuid::parse_str(&row.get::<_, String>("uuid")?).map_err(|_e| {
                rusqlite::Error::InvalidColumnType(
                    0,
                    "uuid".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?,
            from: row.get("`from`")?,
            subject: row.get("subject")?,
            content: row.get("content")?,
            email_id: row.get("email_id")?,
            inserted_at: row.get("inserted_at")?,
            updated_at: row.get("updated_at")?,
        })
    }

    fn row_to_attachment(&self, row: &Row) -> Result<Attachment, rusqlite::Error> {
        Ok(Attachment {
            id: row.get("id")?,
            uuid: Uuid::parse_str(&row.get::<_, String>("uuid")?).map_err(|_e| {
                rusqlite::Error::InvalidColumnType(
                    0,
                    "uuid".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?,
            filename: row.get("filename")?,
            content_type: row.get("content_type")?,
            size: row.get::<_, i32>("size")? as i64,
            data: row.get("data")?,
            message_id: row.get("message_id")?,
            inserted_at: row.get("inserted_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}
