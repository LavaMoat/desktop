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
