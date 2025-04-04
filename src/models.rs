use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct CropData {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}