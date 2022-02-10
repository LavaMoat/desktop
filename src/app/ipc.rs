use crate::user::USER_DATA;
use async_trait::async_trait;
use json_rpc2::{from_str, futures::*, Request, Response, Result};

struct IpcService;

#[async_trait]
impl Service for IpcService {
    type Data = ();
    async fn handle(
        &self,
        request: &Request,
        _ctx: &Self::Data,
    ) -> Result<Option<Response>> {
        let response = match request.method() {
            "Browser.open" => {
                let user = USER_DATA.read().unwrap();
                let url: String = request.deserialize()?;
                let _ = open::that(url).map_err(Box::from)?;
                //let value = serde_json::to_value(result).map_err(Box::from)?;
                None
            }
            "Account.exists" => {
                let user = USER_DATA.read().unwrap();
                let result = user.exists().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Account.recover" => {
                let mut user = USER_DATA.write().unwrap();
                let (mnemonic, passphrase, is_primary): (String, String, bool) =
                    request.deserialize()?;
                let result = user
                    .recover(&mnemonic, &passphrase, is_primary)
                    .map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Account.login" => {
                let mut user = USER_DATA.write().unwrap();
                let result = user.login().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Account.logout" => {
                let mut user = USER_DATA.write().unwrap();
                let result = user.logout().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Account.list" => {
                let user = USER_DATA.read().unwrap();
                let accounts = user.list_accounts().map_err(Box::from)?;
                let value =
                    serde_json::to_value(accounts).map_err(Box::from)?;
                Some((request, value).into())
            }
            // SIGNUP
            "Signup.start" => {
                let mut user = USER_DATA.write().unwrap();
                let result = user.signup_start().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Signup.passphrase" => {
                let mut user = USER_DATA.write().unwrap();
                let result = user.signup_passphrase().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Signup.mnemonic" => {
                let mut user = USER_DATA.write().unwrap();
                let result = user.signup_mnemonic().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Signup.totp" => {
                let mut user = USER_DATA.write().unwrap();
                let result = user.signup_totp().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Signup.verify" => {
                let user = USER_DATA.read().unwrap();
                let token: String = request.deserialize()?;
                let result = user.signup_verify(&token).map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Signup.build" => {
                let mut user = USER_DATA.write().unwrap();
                let result = user.signup_build().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Signup.finish" => {
                let mut user = USER_DATA.write().unwrap();
                let result = user.signup_finish().map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            _ => None,
        };
        Ok(response)
    }
}

pub(crate) async fn handle(message: &str) -> Result<Option<Response>> {
    let request = from_str(message)?;
    let service: Box<dyn Service<Data = ()>> = Box::new(IpcService {});
    let server = Server::new(vec![&service]);
    let response = server.serve(&request, &()).await;
    Ok(response)
}
