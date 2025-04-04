use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

// Function to add border to image
pub fn add_border(image: DynamicImage, border_width: u32, border_color: String) -> DynamicImage {
    let (width, height) = image.dimensions();
    let new_width = width + (border_width * 2);
    let new_height = height + (border_width * 2);
    
    // Parse the color string (expecting hex format like "FF0000" for red)
    let color_value = u32::from_str_radix(border_color.trim_start_matches('#').as_ref(), 16)
        .unwrap_or(0xFF0000); // Default to red if parsing fails
    
    let r = ((color_value >> 16) & 0xFF) as u8;
    let g = ((color_value >> 8) & 0xFF) as u8;
    let b = (color_value & 0xFF) as u8;
    
    // Create a new image with the border
    let mut new_image = RgbaImage::new(new_width, new_height);
    
    // Fill with border color (using alpha = 255 for fully opaque)
    for x in 0..new_width {
        for y in 0..new_height {
            new_image.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    
    // Copy the original image to the center
    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            new_image.put_pixel(x + border_width, y + border_width, pixel);
        }
    }
    
    DynamicImage::ImageRgba8(new_image)
}