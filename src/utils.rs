use std::io::Cursor;
use axum::{
    extract::Multipart,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use image::{DynamicImage, ImageReader};

pub async fn extract_image(multipart: &mut Multipart) -> Result<Vec<u8>, StatusCode> {
    while let Ok(Some(field)) = multipart.next_field().await {
        println!("Field name: {:?}", field.name());
        println!("Field content type: {:?}", field.content_type());
        if field.name() == Some("image") {
            let data = field.bytes().await.unwrap();
            return Ok(data.to_vec());
        }
    }
    Err(StatusCode::BAD_REQUEST)
}

pub async fn find_operation_value(multipart: &mut Multipart) -> Result<(String, i32, u32, u32, u32, String, String), StatusCode> {
    let mut operation: Option<String> = None;
    let mut value: i32 = 50; // Default value
    let mut height: u32 = 500;
    let mut width: u32 = 500;
    let mut border_width: u32 = 5; // Default border width
    let mut border_color: String = "000000".to_string(); // Default border color (black)
    let mut crop_data: String = "{{\"x\":33.42135868591466,\"y\":4.530004678323855e-15,\"width\":37.03703703703704,\"height\":32.03203203203203}}".to_string(); // Default crop data

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("operation") => {
                if let Ok(data) = String::from_utf8(field.bytes().await.unwrap().to_vec()) {
                    operation = Some(data);
                }
            }
            Some("value") => {
                if let Ok(data) = String::from_utf8(field.bytes().await.unwrap().to_vec()) {
                    value = data.parse().unwrap_or(50);
                }
            }
            Some("height") => {
                if let Ok(data) = String::from_utf8(field.bytes().await.unwrap().to_vec()) {
                    height = data.parse().unwrap_or(500);
                }
            }
            Some("width") => {
                if let Ok(data) = String::from_utf8(field.bytes().await.unwrap().to_vec()) {
                    width = data.parse().unwrap_or(500);
                }
            }
            Some("border_width") => {
                if let Ok(data) = String::from_utf8(field.bytes().await.unwrap().to_vec()) {
                    border_width = data.parse().unwrap_or(5);
                }
            }
            Some("border_color") => {
                if let Ok(data) = String::from_utf8(field.bytes().await.unwrap().to_vec()) {
                    border_color = data;
                }
            }
            Some("crop") => {
                if let Ok(data) = String::from_utf8(field.bytes().await.unwrap().to_vec()) {
                    // Handle crop operation
                    crop_data = data;
                }
            }
            _ => {}
        }
    }

    if let Some(op) = operation {
        Ok((op, value, height, width, border_width, border_color, crop_data))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn image_to_bytes(image: DynamicImage) -> Result<Vec<u8>, StatusCode> {
    let mut buffer = Vec::new();
    image.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(buffer)
}

pub async fn send_image_response(image_vec: Vec<u8>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/jpeg")
        .body(axum::body::Body::from(image_vec))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

pub async fn load_image(image_vec: &[u8]) -> Result<DynamicImage, StatusCode> {
    ImageReader::new(Cursor::new(image_vec))
        .with_guessed_format()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .decode()
        .map_err(|_| StatusCode::BAD_REQUEST)
}