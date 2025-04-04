use std::io::Cursor;

use image::{ DynamicImage, GenericImageView, ImageReader };
use serde::de::value;
use tokio::net::TcpListener;

use axum::{extract:: Multipart, http::{header, StatusCode}, response::{IntoResponse, Response}, routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new().route("/" , get(|| async { "hello"})).route("/process" , post(process_image))
        .layer(cors);

 println!("Server running on http://0.0.0.0:5000");
 let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
 axum::serve(listener, app).await.unwrap();
}


async fn process_image(mut multipart : Multipart ) -> Result<Response, StatusCode>{
    let image_vec= extract_image(&mut multipart).await.unwrap();
    let mut image: DynamicImage = load_image(&image_vec).await.unwrap();
    let (operation, value, height , width) = find_operation_value(&mut multipart).await.unwrap();
    // print!("Operation: {:?}", operation);
    print!("Value: {:?}", value);
    match operation.as_str() {
        "grayscale" => {
            let processed = image.grayscale();
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "brightness" => {
            let processed = image.brighten(value);
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "unsharpen" => {
            let processed = image.unsharpen(value as f32, 2);
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "huerotate"=>{
            let processed= image.huerotate(value);
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "blur" => {
            let processed = image.fast_blur(value as f32);
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "contrast" => {
            let processed = image.adjust_contrast(value as f32);
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "flip_horizontal" => {
            let processed = image.fliph();
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "flip_vertical" => {
            let processed = image.flipv();
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "rotate90" => {
            let processed = image.rotate90();
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "rotate270" => {
            let processed = image.rotate270();
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "rotate180" => {
            let processed = image.rotate180();
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "resize" => {
            // let (width, height) = image.dimensions();
            println!("Width: {:?} Height: {:?}", width, height);
            let processed = image.resize(width,height, image::imageops::FilterType::Triangle);
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        "crop" => {
            let (width, height) = image.dimensions();
            let processed = image.crop(10, 20, width / 2, height / 2);
            let result = image_to_bytes(processed).await.unwrap();
            Ok(send_image_response(result).await)
        }
        _ => {
            println!("Unknown operation");
            Ok(StatusCode::BAD_REQUEST.into_response())
        }
    }
}

async fn extract_image(multipart: &mut Multipart) -> Result<Vec<u8>, StatusCode> {
    while let Ok(Some(field)) = multipart.next_field().await{
        println!("Field name: {:?}", field.name());
        println!("Field content type: {:?}", field.content_type());
        if field.name() == Some("image") {
            let data = field.bytes().await.unwrap();
            return Ok(data.to_vec());
        }
    }
    Err(StatusCode::BAD_REQUEST)
}

async fn find_operation_value(multipart: &mut Multipart) -> Result<(String, i32 , u32 , u32), StatusCode> {
    let mut operation: Option<String> = None;
    let mut value: i32 = 50; // Default value
    let mut height: u32 = 500;
    let mut width: u32 = 500;

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
            _ => {}
        }
    }

    if let Some(op) = operation {
        Ok((op, value ,height as u32, width as u32))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}


async fn extract_image_size(multipart: &mut Multipart) -> Result<(u32, u32), StatusCode> {
    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
  
            _ => {}
        }
    }
    Err(StatusCode::BAD_REQUEST)
}


async fn image_to_bytes(image : DynamicImage)-> Result<Vec<u8>, StatusCode> {
    let mut buffer = Vec::new();
    image.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(buffer)
}

async fn send_image_response(image_vec : Vec<u8>) -> Response {

    Response::builder().status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/jpeg")
        .body(axum::body::Body::from(image_vec))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())

}

async fn load_image(image_vec : &[u8]) -> Result<DynamicImage, StatusCode> {
     ImageReader::new(Cursor::new(image_vec)).with_guessed_format().map_err(|_| StatusCode::BAD_REQUEST)?.decode().map_err(|_| StatusCode::BAD_REQUEST)

}