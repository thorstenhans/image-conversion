use bytes::Bytes;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{FullBody, IntoResponse, Request, Response};
use spin_sdk::http_service;

#[http_service]
async fn handle_image_conversion(req: Request) -> anyhow::Result<impl IntoResponse> {
    let body = req.into_body().bytes().await?;
    let img = image::load_from_memory(&body)?;

    // Downsample using Lanczos3 for sharp visual quality
    let resized = img.resize(800, 600, FilterType::Lanczos3);

    // Encode to JPEG in memory with quality settings
    let mut buffer = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 85);
    encoder.encode_image(&resized)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "image/jpeg")
        .body(FullBody::new(Bytes::from(buffer)))?)
}
