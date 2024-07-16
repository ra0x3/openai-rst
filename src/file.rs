//! This module defines the structures and methods for managing files, including uploading, deleting, retrieving, and listing files.
//! It includes:
//! - `FileData`: Struct representing the data of a file.
//! - `FileListResponse`: Struct for the response from a request to list files.
//! - `FileUploadRequest`: Struct for creating a request to upload a file.
//! - `FileUploadResponse`: Struct for the response from a file upload request.
//! - `FileDeleteRequest`: Struct for creating a request to delete a file.
//! - `FileDeleteResponse`: Struct for the response from a file delete request.
//! - `FileRetrieveRequest`: Struct for creating a request to retrieve a file.
//! - `FileRetrieveResponse`: Struct for the response from a file retrieve request.
//! - `FileRetrieveContentRequest`: Struct for creating a request to retrieve the content of a file.
//! - `FileRetrieveContentResponse`: Struct for the response from a file content retrieve request.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the data of a file.
#[derive(Debug, Deserialize, Serialize)]
pub struct FileData {
    /// Unique identifier for the file.
    pub id: String,
    /// Object type, typically "file".
    pub object: String,
    /// Size of the file in bytes.
    pub bytes: i32,
    /// Timestamp of when the file was created.
    pub created_at: i64,
    /// Name of the file.
    pub filename: String,
    /// Purpose of the file.
    pub purpose: String,
}

/// Represents the response from a request to list files.
#[derive(Debug, Deserialize, Serialize)]
pub struct FileListResponse {
    /// Object type, typically "list".
    pub object: String,
    /// List of file data.
    pub data: Vec<FileData>,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request to upload a file.
#[derive(Debug, Serialize)]
pub struct FileUploadRequest {
    /// Path to the file to be uploaded.
    pub file: String,
    /// Purpose of the file.
    pub purpose: String,
}

impl FileUploadRequest {
    /// Creates a new `FileUploadRequest` with the specified file and purpose.
    pub fn new(file: String, purpose: String) -> Self {
        Self { file, purpose }
    }
}

/// Represents the response from a file upload request.
#[derive(Debug, Deserialize, Serialize)]
pub struct FileUploadResponse {
    /// Unique identifier for the uploaded file.
    pub id: String,
    /// Object type, typically "file".
    pub object: String,
    /// Size of the file in bytes.
    pub bytes: i32,
    /// Timestamp of when the file was created.
    pub created_at: i64,
    /// Name of the file.
    pub filename: String,
    /// Purpose of the file.
    pub purpose: String,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request to delete a file.
#[derive(Debug, Serialize)]
pub struct FileDeleteRequest {
    /// Identifier for the file to be deleted.
    pub file_id: String,
}

impl FileDeleteRequest {
    /// Creates a new `FileDeleteRequest` with the specified file ID.
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }
}

/// Represents the response from a file delete request.
#[derive(Debug, Deserialize, Serialize)]
pub struct FileDeleteResponse {
    /// Unique identifier for the deleted file.
    pub id: String,
    /// Object type, typically "file".
    pub object: String,
    /// Indicates whether the file was successfully deleted.
    pub delete: bool,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request to retrieve a file.
#[derive(Debug, Serialize)]
pub struct FileRetrieveRequest {
    /// Identifier for the file to be retrieved.
    pub file_id: String,
}

impl FileRetrieveRequest {
    /// Creates a new `FileRetrieveRequest` with the specified file ID.
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }
}

/// Represents the response from a file retrieve request.
#[derive(Debug, Deserialize, Serialize)]
pub struct FileRetrieveResponse {
    /// Unique identifier for the retrieved file.
    pub id: String,
    /// Object type, typically "file".
    pub object: String,
    /// Size of the file in bytes.
    pub bytes: i32,
    /// Timestamp of when the file was created.
    pub created_at: i64,
    /// Name of the file.
    pub filename: String,
    /// Purpose of the file.
    pub purpose: String,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}

/// Represents a request to retrieve the content of a file.
#[derive(Debug, Serialize)]
pub struct FileRetrieveContentRequest {
    /// Identifier for the file whose content is to be retrieved.
    pub file_id: String,
}

impl FileRetrieveContentRequest {
    /// Creates a new `FileRetrieveContentRequest` with the specified file ID.
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }
}

/// Represents the response from a file content retrieve request.
#[derive(Debug, Deserialize, Serialize)]
pub struct FileRetrieveContentResponse {
    /// Unique identifier for the file whose content was retrieved.
    pub id: String,
    /// Object type, typically "file".
    pub object: String,
    /// Size of the file in bytes.
    pub bytes: i32,
    /// Timestamp of when the file was created.
    pub created_at: i64,
    /// Name of the file.
    pub filename: String,
    /// Purpose of the file.
    pub purpose: String,
    /// Optional headers from the response.
    pub headers: Option<HashMap<String, String>>,
}
