use serde::Deserialize;
use actix_web::{web, HttpResponse, Result};
use qrcode::QrCode;
use image::{Luma, DynamicImage};

#[derive(Deserialize)]
pub(crate) struct QueryString {
    text: String,
}

pub(crate) async fn handler(query: web::Query<QueryString>) -> Result<HttpResponse> {
    // Encode some data into bits.
    let code = QrCode::new(query.text.as_bytes()).unwrap();
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    let png_image = DynamicImage::ImageLuma8(image);
    let mut bytes: Vec<u8> = Vec::new();
    png_image.write_to(&mut bytes, image::ImageOutputFormat::Png).map_err(Box::from)?;

    //let data = image.into_vec();
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(bytes))
}
