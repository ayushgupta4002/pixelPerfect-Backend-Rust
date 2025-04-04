use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use image::{DynamicImage, GenericImageView};

use crate::{img_ops::add_border, models::{CropData, User, UserResponse}};
use crate::utils::{extract_image, find_operation_value, load_image, image_to_bytes, send_image_response};

pub async fn test_process(Json(user): Json<User>) -> impl IntoResponse {
    println!("User: {:?}", user);
    let response = format!("Hello, {}! You are {} years old.", user.name, user.age);
    let resp = UserResponse {
        message: response,
    };
    (StatusCode::OK, Json(resp))
}

pub async fn process_image(mut multipart: Multipart) -> Result<Response, StatusCode> {
    let image_vec = extract_image(&mut multipart).await?;
    let mut image: DynamicImage = load_image(&image_vec).await?;
    let (operation, value, height, width, border_width, border_color, crop_data) = 
        find_operation_value(&mut multipart).await?;
    
    print!("Value: {:?}", value);
    match operation.as_str() {
        "grayscale" => {
            let processed = image.grayscale();
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "brightness" => {
            let processed = image.brighten(value);
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "unsharpen" => {
            let processed = image.unsharpen(value as f32, 2);
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "huerotate" => {
            let processed = image.huerotate(value);
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "blur" => {
            let processed = image.fast_blur(value as f32);
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "contrast" => {
            let processed = image.adjust_contrast(value as f32);
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "flip_horizontal" => {
            let processed = image.fliph();
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "flip_vertical" => {
            let processed = image.flipv();
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "rotate90" => {
            let processed = image.rotate90();
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "rotate270" => {
            let processed = image.rotate270();
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "rotate180" => {
            let processed = image.rotate180();
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "resize" => {
            println!("Width: {:?} Height: {:?}", width, height);
            let processed = image.resize(width, height, image::imageops::FilterType::Triangle);
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        "crop" => {
            let (width, height) = image.dimensions();
            let crop_info = match serde_json::from_str::<CropData>(&crop_data) {
                Ok(data) => Some(data),
                Err(e) => {
                    println!("Error parsing crop data: {:?}", e);
                    None
                }
            };

            if let Some(crop_data) = crop_info {
                println!("Crop data: {:?}", crop_data);
                println!("Width: {:?} Height: {:?}", width, height);
                let processed = image.crop(crop_data.x, crop_data.y, crop_data.width, crop_data.height);
                let result = image_to_bytes(processed).await?;
                Ok(send_image_response(result).await)
            } else {
                println!("no cropper details present");
                Ok(StatusCode::BAD_REQUEST.into_response())
            }
        }
        "border" => {
            let processed = add_border(image, border_width, border_color);
            let result = image_to_bytes(processed).await?;
            Ok(send_image_response(result).await)
        }
        _ => {
            println!("Unknown operation");
            Ok(StatusCode::BAD_REQUEST.into_response())
        }
    }
}