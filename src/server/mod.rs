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
mod rpc;

#[actix_web::main]
pub async fn server<A: ToSocketAddrs>(
    addr: A,
    bind: oneshot::Sender<Option<SocketAddr>>,
) -> Result<()> {
    // Setup the Oauth2 PKCE configuration
    let mut pkce_setup = oauth::PkceSetup::new();

    pkce_setup.register_client(
        "LocalClient",
        "http://localhost:7778/".parse::<url::Url>()?,
        "default-scope",
    );

    // Start the actor
    let pkce_agent = pkce_setup.start();

    let server = HttpServer::new(move || {
        let origins = vec![
            "http://localhost:7777",
            // Test local client
            "http://localhost:7778",
        ];

        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                origins.iter().find(|o| *o == origin).is_some()
            })
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
