use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: String
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UpdateBookRequest {
    pub id: i64,
    pub title: Option<String>,
    pub author: Option<String>
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct DeleteBookRequest {
    pub id: i64,
    pub title: Option<String>,
    pub author: Option<String>
}