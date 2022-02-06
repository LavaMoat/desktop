use actix::Addr;
use actix_web::{web, HttpRequest};
use oxide_auth_actix::{
    Authorize, OAuthOperation, OAuthRequest, OAuthResponse, Refresh, Token,
    WebError,
};

use super::pkce::{Extras, PkceSetup};

pub async fn get_authorize(
    req: OAuthRequest,
    state: web::Data<Addr<PkceSetup>>,
) -> Result<OAuthResponse, WebError> {
    state.send(Authorize(req).wrap(Extras::AuthGet)).await?
}

pub async fn post_authorize(
    httpreq: HttpRequest,
    req: OAuthRequest,
    state: web::Data<Addr<PkceSetup>>,
) -> Result<OAuthResponse, WebError> {
    state
        .send(
            Authorize(req)
                .wrap(Extras::AuthPost(httpreq.query_string().to_owned())),
        )
        .await?
}

pub async fn post_token(
    req: OAuthRequest,
    state: web::Data<Addr<PkceSetup>>,
) -> Result<OAuthResponse, WebError> {
    state.send(Token(req).wrap(Extras::Nothing)).await?
}

pub async fn post_refresh(
    req: OAuthRequest,
    state: web::Data<Addr<PkceSetup>>,
) -> Result<OAuthResponse, WebError> {
    state.send(Refresh(req).wrap(Extras::Nothing)).await?
}

/*
fn main() {
    std::env::set_var("RUST_LOG", "actix_shopper=info,actix_server=info,actix_web=info");
    env_logger::init();

    let sys = actix::System::new("HttpServerClient");

    let pkce_agent = PkceSetup::new().start();

    HttpServer::new(move || {
        App::new()
            .data(pkce_agent.clone())
            .service(web::scope("/oauth/")
                .service(
                    web::resource("/authorize")
                        .route(web::get().to(get_authorize))
                        .route(web::post().to(post_authorize))
                )
                .route("/token",web::post().to(post_token))
                .route("/refresh", web::post().to(post_refresh))
            )
            .route("/resource", web::get().to(resource))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8081")
    .expect("Failed to bind to socket")
    .run();

    sys.run().expect("Failed to start actors loop");
}
*/

/*
async fn resource(req: OAuthResource, state: web::Data<Addr<PkceSetup>>) -> Result<OAuthResponse, WebError> {
    let resource = state.send(Resource(req.into_request()).wrap(Extras::Nothing)).await?;
    match resource {
        Ok(_grant) => create_session(), //actix_files::Files::new("/", "./web/dist").index_file("index.html"),
        Err(Ok(response)) => Ok(response.body(DENY_TEXT)),
        Err(Err(e)) => Err(e.into()),
    }
}
*/
