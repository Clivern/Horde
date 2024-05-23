// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::util;
use rand::prelude::IndexedRandom;
use rand::rng;
use rand::Rng;
use rusqlite::{params, Connection, Row};
use std::collections::HashMap;
use uuid::Uuid; // For random_range

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
const USERNAME_LEN: usize = 10;

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
    pub path: String,
    pub message_id: i32,
    pub inserted_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    // New database client
    pub fn new(db_path: &str) -> Result<Database, String> {
        let conn = Connection::open(db_path)
            .map_err(|e| format!("Unable to establish database connection! {}", e))?;

        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| format!("Failed to enable foreign key constraints: {}", e))?;

        Ok(Database { conn })
    }

    // Get a random email from the database
    // @param domains: The domains to use
    // @return: The random email
    pub fn get_random_email(&self, domains: &Vec<String>) -> Result<String, String> {
        loop {
            let mut rng = rng();
            let username: String = (0..USERNAME_LEN)
                .map(|_| {
                    let idx = rng.random_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();

            let domain = domains.choose(&mut rng).ok_or("No domains provided")?;
            let email = format!("{}@{}", username, domain);
            let result = self.get_email_by_address(&email);

            match result {
                Ok(None) => {
                    return Ok(email);
                }
                Err(_) => {
                    return Ok(email);
                }
                Ok(Some(_)) => {
                    continue;
                }
            }
        }
    }

    // Create an email
    // @param email: The email to create
    // @param expire: Whether the email should expire
    // @param expire_at: The date and time the email should expire
    // @return: The created email
    pub fn create_email(
        &self,
        email: &str,
        expire: bool,
        expire_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Option<Email>, String> {
        let uuid = Uuid::new_v4();
        let query = "INSERT INTO emails (uuid, email, expire, expire_at) VALUES (?1, ?2, ?3, ?4)";

        self.conn
            .execute(query, params![uuid.to_string(), email, expire, expire_at])
            .map_err(|e| format!("Failed to create email: {}", e))?;

        let id = self.conn.last_insert_rowid() as i32;

        self.get_email_by_id(id)
    }

    // Get an email by id
    // @param id: The id of the email to get
    // @return: The email
    fn get_email_by_id(&self, id: i32) -> Result<Option<Email>, String> {
        let query = "SELECT * FROM emails WHERE id = ?1";

        match self
            .conn
            .query_row(query, params![id], |row| self.row_to_email(row))
        {
            Ok(email) => Ok(Some(email)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get email: {}", e)),
        }
    }

    // Get an email by uuid
    // @param uuid: The uuid of the email to get
    // @return: The email
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

    // Get an email by address
    // @param email: The address of the email to get
    // @return: The email
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

    // Delete an email by uuid
    // @param uuid: The uuid of the email to delete
    // @return: The result of the operation
    pub fn delete_email_by_uuid(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM emails WHERE uuid = ?1";

        self.conn
            .execute(query, params![uuid.to_string()])
            .map_err(|e| format!("Failed to delete email: {}", e))
            .map(|_| ())
    }

    // Delete an email by id
    // @param id: The id of the email to delete
    // @return: The result of the operation
    pub fn delete_email_by_id(&self, id: i32) -> Result<(), String> {
        let query = "DELETE FROM emails WHERE id = ?1";

        self.conn
            .execute(query, params![id])
            .map_err(|e| format!("Failed to delete email: {}", e))
            .map(|_| ())
    }

    // Create a message
    // @param from: The from address
    // @param subject: The subject of the message
    // @param content: The content of the message
    // @param email_id: The id of the email
    // @return: The created message
    pub fn create_message(
        &self,
        from: &str,
        subject: &str,
        content: &str,
        email_id: i32,
    ) -> Result<Option<Message>, String> {
        let uuid = Uuid::new_v4();
        let query = "INSERT INTO messages (uuid, `from`, subject, content, email_id) VALUES (?1, ?2, ?3, ?4, ?5)";

        self.conn
            .execute(
                query,
                params![uuid.to_string(), from, subject, content, email_id],
            )
            .map_err(|e| format!("Failed to create message: {}", e))?;

        let id = self.conn.last_insert_rowid() as i32;
        self.get_message_by_id(id)
    }

    // Get a message by id
    // @param id: The id of the message to get
    // @return: The message
    fn get_message_by_id(&self, id: i32) -> Result<Option<Message>, String> {
        let query = "SELECT * FROM messages WHERE id = ?1";

        match self
            .conn
            .query_row(query, params![id], |row| self.row_to_message(row))
        {
            Ok(message) => Ok(Some(message)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get message: {}", e)),
        }
    }

    // Get a message by uuid
    // @param uuid: The uuid of the message to get
    // @return: The message
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

    // Get messages by email id
    // @param email_id: The id of the email to get messages for
    // @return: The messages
    pub fn get_messages_by_email_id(
        &self,
        email_id: i32,
    ) -> Result<Vec<Message>, String> {
        let query =
            "SELECT * FROM messages WHERE email_id = ?1 ORDER BY inserted_at DESC";

        let mut stmt = self
            .conn
            .prepare(query)
            .map_err(|e| format!("Failed to prepare query: {}", e))?;

        let rows = stmt
            .query_map(params![email_id], |row| self.row_to_message(row))
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        let messages: Result<Vec<_>, _> = rows.collect();

        messages.map_err(|e| format!("Failed to process message: {}", e))
    }

    // Delete a message by uuid
    // @param uuid: The uuid of the message to delete
    // @return: The result of the operation
    pub fn delete_message_by_uuid(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM messages WHERE uuid = ?1";

        self.conn
            .execute(query, params![uuid.to_string()])
            .map_err(|e| format!("Failed to delete message: {}", e))
            .map(|_| ())
    }

    // Delete a message by id
    // @param id: The id of the message to delete
    // @return: The result of the operation
    pub fn delete_message_by_id(&self, id: i32) -> Result<(), String> {
        let query = "DELETE FROM messages WHERE id = ?1";
        self.conn
            .execute(query, params![id])
            .map_err(|e| format!("Failed to delete message: {}", e))
            .map(|_| ())
    }

    // Create an attachment
    pub fn create_attachment(
        &self,
        filename: &str,
        content_type: &str,
        data: &[u8],
        message_id: i32,
    ) -> Result<Attachment, String> {
        let uuid = Uuid::new_v4();
        let size = data.len() as i64;
        // Write file to disk under storage path using uuid for uniqueness
        let storage_root = util::config::get_storage();
        let attachment_dir = format!("{}attachments/{}", storage_root, uuid);
        std::fs::create_dir_all(&attachment_dir)
            .map_err(|e| format!("Failed to create attachment directory: {}", e))?;

        let file_path = format!("{}/{}", attachment_dir, filename);
        std::fs::write(&file_path, data)
            .map_err(|e| format!("Failed to write attachment to disk: {}", e))?;

        let query = "INSERT INTO attachments (uuid, filename, content_type, size, path, message_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

        self.conn
            .execute(
                query,
                params![
                    uuid.to_string(),
                    filename,
                    content_type,
                    size,
                    file_path,
                    message_id
                ],
            )
            .map_err(|e| format!("Failed to create attachment: {}", e))?;

        let id = self.conn.last_insert_rowid() as i32;
        self.get_attachment_by_id(id)
    }

    // Get an attachment by id
    fn get_attachment_by_id(&self, id: i32) -> Result<Attachment, String> {
        let query = "SELECT * FROM attachments WHERE id = ?1";

        self.conn
            .query_row(query, params![id], |row| self.row_to_attachment(row))
            .map_err(|e| format!("Failed to get attachment: {}", e))
    }

    // Get an attachment by uuid
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

    // Get attachments by message id
    pub fn get_attachments_by_message_id(
        &self,
        message_id: i32,
    ) -> Result<Vec<Attachment>, String> {
        let query =
            "SELECT * FROM attachments WHERE message_id = ?1 ORDER BY inserted_at ASC";

        let mut stmt = self
            .conn
            .prepare(query)
            .map_err(|e| format!("Failed to prepare query: {}", e))?;

        let rows = stmt
            .query_map(params![message_id], |row| self.row_to_attachment(row))
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        let attachments: Result<Vec<_>, _> = rows.collect();
        attachments.map_err(|e| format!("Failed to process attachment: {}", e))
    }

    // Delete an attachment by uuid
    pub fn delete_attachment(&self, uuid: &Uuid) -> Result<(), String> {
        let query = "DELETE FROM attachments WHERE uuid = ?1";

        self.conn
            .execute(query, params![uuid.to_string()])
            .map_err(|e| format!("Failed to delete attachment: {}", e))
            .map(|_| ())
    }

    // Set an email metadata
    pub fn set_email_meta(
        &self,
        email_id: i32,
        key: &str,
        value: &str,
    ) -> Result<(), String> {
        // First try to update existing record
        let update_query = "UPDATE emails_meta SET value = ?1, updated_at = CURRENT_TIMESTAMP WHERE email_id = ?2 AND key = ?3";

        let rows_affected = self
            .conn
            .execute(update_query, params![value, email_id, key])
            .map_err(|e| format!("Failed to update email metadata: {}", e))?;

        if rows_affected > 0 {
            return Ok(());
        }

        // If no rows were affected, insert new record
        let insert_query =
            "INSERT INTO emails_meta (email_id, key, value) VALUES (?1, ?2, ?3)";

        self.conn
            .execute(insert_query, params![email_id, key, value])
            .map_err(|e| format!("Failed to insert email metadata: {}", e))
            .map(|_| ())
    }

    pub fn get_email_meta(
        &self,
        email_id: i32,
    ) -> Result<HashMap<String, String>, String> {
        let query = "SELECT key, value FROM emails_meta WHERE email_id = ?1";

        let mut stmt = self
            .conn
            .prepare(query)
            .map_err(|e| format!("Failed to prepare query: {}", e))?;

        let rows = stmt
            .query_map(params![email_id], |row| {
                let key: String = row.get("key")?;
                let value: String = row.get("value")?;
                Ok((key, value))
            })
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        let pairs: Result<Vec<(String, String)>, _> = rows.collect();
        let pairs = pairs.map_err(|e| format!("Failed to process metadata: {}", e))?;
        Ok(pairs.into_iter().collect())
    }

    // Helper methods to convert rows to structs
    fn row_to_email(&self, row: &Row) -> Result<Email, rusqlite::Error> {
        let uuid_str: String = row.get("uuid")?;
        Ok(Email {
            id: row.get("id")?,
            uuid: Uuid::parse_str(&uuid_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
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
        let uuid_str: String = row.get("uuid")?;
        Ok(Message {
            id: row.get("id")?,
            uuid: Uuid::parse_str(&uuid_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?,
            from: row.get("from")?,
            subject: row.get("subject")?,
            content: row.get("content")?,
            email_id: row.get("email_id")?,
            inserted_at: row.get("inserted_at")?,
            updated_at: row.get("updated_at")?,
        })
    }

    fn row_to_attachment(&self, row: &Row) -> Result<Attachment, rusqlite::Error> {
        let uuid_str: String = row.get("uuid")?;
        Ok(Attachment {
            id: row.get("id")?,
            uuid: Uuid::parse_str(&uuid_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?,
            filename: row.get("filename")?,
            content_type: row.get("content_type")?,
            size: row.get::<_, i64>("size")?,
            path: row.get("path")?,
            message_id: row.get("message_id")?,
            inserted_at: row.get("inserted_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}
