use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::Result;
use rust_embed::RustEmbed;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::sync::oneshot;

const INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "./static"]
pub struct Assets;

async fn embedded_handler(req: HttpRequest) -> HttpResponse {
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

#[actix_web::main]
pub async fn server<A: ToSocketAddrs>(
    addr: A,
    bind: oneshot::Sender<Option<SocketAddr>>,
) -> Result<()> {
    let server = HttpServer::new(|| {
        App::new().service(
            web::resource("/{tail:.*}").route(web::get().to(embedded_handler)),
        )
    })
    .workers(1)
    .disable_signals()
    .bind(addr)?;

    let mut addrs = server.addrs();
    let bind_notify = async move {
        if !addrs.is_empty() {
            let addr = addrs.swap_remove(0);
            match bind.send(Some(addr)) {
                Err(_) => {
                    panic!("failed to send connection info on bind channel");
                }
                _ => {}
            }
        } else {
            // No socket addr so we got a bind error
        }
        Ok(())
    };

    let server = server.run();
    futures::try_join!(server, bind_notify)?;
    Ok(())
}
