use actix::Addr;
use actix_web::web;
use anyhow::Result;

use oxide_auth_actix::{
    OAuthOperation, OAuthResource, OAuthResponse, Resource, WebError,
};

use super::oauth::{Extras, PkceSetup};

static DENY_TEXT: &str = "NOT_AUTHORIZED";

pub fn create_session() -> Result<OAuthResponse, WebError> {
    // TODO: expose RPC API
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
        Ok(_grant) => create_session(),
        Err(Ok(response)) => Ok(response.body(DENY_TEXT)),
        Err(Err(e)) => Err(e.into()),
    }
}
