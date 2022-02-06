use actix_web::{HttpRequest, HttpResponse};
use rust_embed::RustEmbed;

const INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "./static"]
pub struct Assets;

// Serves the assets embedded in the executable.
pub(crate) async fn handler(req: HttpRequest) -> HttpResponse {
    let memfs_path = if req.path() == "/" {
        INDEX_HTML
    } else {
        req.path().trim_start_matches("/")
    };
    if let Some(memfs_file) = Assets::get(memfs_path) {
        let mime_type = mime_guess::from_path(memfs_path)
            .first()
            .unwrap_or(mime::TEXT_PLAIN);
        HttpResponse::Ok()
            .content_type(mime_type)
            .body(memfs_file.data.into_owned())
    } else {
        HttpResponse::NotFound()
            .content_type("text/html")
            .body("NOT_FOUND")
    }
}
