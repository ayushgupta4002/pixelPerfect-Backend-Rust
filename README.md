# PixelPerfect Backend

A lightweight Rust web server built with Axum for processing images. This API allows users to upload images and apply various transformations such as grayscale conversion, brightness adjustment, cropping, rotation, and more.

https://github.com/user-attachments/assets/a2a125ac-7ccc-426a-b463-f80f7e1ddded

## Features

- **Multiple Image Operations**: Supports various image transformations:
  - Grayscale conversion
  - Brightness adjustment
  - Blur effects
  - Contrast adjustment
  - Hue rotation
  - Image flipping (horizontal/vertical)
  - Rotation (90°, 180°, 270°)
  - Resizing
  - Cropping
  - Adding borders with custom colors

- **Modern Rust Web Stack**:
  - Built with Axum web framework
  - Async processing with Tokio
  - CORS support for cross-origin requests

## Project Structure

```
src/
├── main.rs           # Entry point and server setup
├── handlers.rs       # Request handlers for API endpoints
├── models.rs         # Data structures and serialization
├── image_ops.rs      # Image processing functionality
└── utils.rs          # Utility functions for request/response handling
```

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- Basic understanding of HTTP and REST APIs

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/image-processing-api.git
   cd image-processing-api
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the server:
   ```
   cargo run --release
   ```

The server will start at http://0.0.0.0:5000

## API Endpoints

### `GET /`
- Basic health check endpoint
- Returns "hello" when the server is running

### `POST /process`
- Main image processing endpoint
- Accepts multipart form data with the following fields:
  - `image`: The image file to process
  - `operation`: The operation to perform (e.g., "grayscale", "brightness", "blur")
  - `value`: Numeric parameter for operations that require it
  - `width`, `height`: Dimensions for resize operation
  - `border_width`, `border_color`: Parameters for border operation
  - `crop`: JSON string containing crop coordinates and dimensions
- Returns the processed image as a response

### `POST /postprocess`
- Ignore this one,its purely for fun
- Test endpoint for JSON processing
- Accepts JSON with "name" and "age" fields
- Returns a greeting message

## Usage Examples

### Example: Apply Grayscale Effect

```bash
curl -X POST http://localhost:5000/process \
  -F "image=@/path/to/your/image.jpg" \
  -F "operation=grayscale" \
  -o processed_image.jpg
```

### Example: Adjust Brightness

```bash
curl -X POST http://localhost:5000/process \
  -F "image=@/path/to/your/image.jpg" \
  -F "operation=brightness" \
  -F "value=50" \
  -o processed_image.jpg
```

### Example: Add Border

```bash
curl -X POST http://localhost:5000/process \
  -F "image=@/path/to/your/image.jpg" \
  -F "operation=border" \
  -F "border_width=10" \
  -F "border_color=FF0000" \
  -o processed_image.jpg
```

## Error Handling

The API returns appropriate HTTP status codes:
- `200 OK`: Operation successful
- `400 Bad Request`: Invalid input parameters or image format
- `500 Internal Server Error`: Server-side processing error

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework used
- [image](https://github.com/image-rs/image) - Image processing library
- [Tokio](https://tokio.rs/) - Async runtime for Rust



