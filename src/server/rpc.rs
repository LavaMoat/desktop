use actix::Addr;
use actix_web::web;
use anyhow::Result;

use log::info;

use oxide_auth_actix::{
    OAuthOperation, OAuthResource, OAuthResponse, Resource, WebError,
};

use super::oauth::{Extras, PkceSetup};

static DENY_TEXT: &str = "<html>
<h1>NO!!!</h1>
This page should be accessed via an oauth token from the client in the example. Click
<a href=\"http://localhost:8081/oauth/authorize?response_type=code&client_id=LocalClient\">
here</a> to begin the authorization process.
</html>";

pub fn create_session() -> Result<OAuthResponse, WebError> {
    info!("create_session!!");
    Ok(OAuthResponse::ok()
        .content_type("text/plain")?
        .body("Logged in"))
}

/// Handles JSON-RPC POST requests.
pub(crate) async fn handler(
    req: OAuthResource,
    state: web::Data<Addr<PkceSetup>>,
) -> Result<OAuthResponse, WebError> {
    let resource = state
        .send(Resource(req.into_request()).wrap(Extras::Nothing))
        .await?;
    match resource {
        Ok(_grant) => create_session(), //actix_files::Files::new("/", "./web/dist").index_file("index.html"),
        Err(Ok(response)) => Ok(response.body(DENY_TEXT)),
        Err(Err(e)) => Err(e.into()),
    }
}
