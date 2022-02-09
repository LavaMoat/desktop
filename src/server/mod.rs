use actix::Actor;
use actix_cors::Cors;
use actix_web::{
    http,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::sync::oneshot;

mod assets;
mod oauth;
mod qrcode;
mod rpc;

#[actix_web::main]
pub async fn server<A: ToSocketAddrs>(
    addr: A,
    bind: oneshot::Sender<Option<SocketAddr>>,
) -> Result<()> {
    let pkce_agent = oauth::PkceSetup::new().start();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:7777")
            .allowed_origin("http://localhost:7778")
            //.allowed_origin_fn(|origin, _req_head| {
            //origin.as_bytes().ends_with(b".rust-lang.org")
            //})
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(Data::new(pkce_agent.clone()))
            .wrap(cors)
            .service(
                web::resource("/qrcode").route(web::get().to(qrcode::handler)))
            .service(
                web::scope("/oauth")
                    .service(
                        web::resource("/authorize")
                            .route(web::get().to(oauth::get_authorize))
                            .route(web::post().to(oauth::post_authorize)),
                    )
                    .route("/token", web::post().to(oauth::post_token))
                    .route("/refresh", web::post().to(oauth::post_refresh)),
            )
            .service(web::resource("/rpc").route(web::post().to(rpc::handler)))
            .service(
                web::resource("/{tail:.*}")
                    .route(web::get().to(assets::handler)),
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
