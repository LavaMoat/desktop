use async_trait::async_trait;
use json_rpc2::{from_str, futures::*, Request, Response, Result};
use serde_json::Value;

use crate::user::USER_DATA;

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
            "Account.exists" => {
                let user = USER_DATA.read().unwrap();
                let result = user.exists();
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Account.signup" => {
                let mut user = USER_DATA.write().unwrap();
                let passphrase: String = request.deserialize()?;
                let result = user.signup(&passphrase).map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            "Account.login" => {
                let mut user = USER_DATA.write().unwrap();
                let passphrase: String = request.deserialize()?;
                let result = user.login(&passphrase).map_err(Box::from)?;
                let value = serde_json::to_value(result).map_err(Box::from)?;
                Some((request, value).into())
            }
            /*
            "Account.create" => {
                let mut user = USER_DATA.write().unwrap();
                let address = user.create_account().map_err(Box::from)?;
                Some((request, Value::String(address)).into())
            }
            */
            "Account.list" => {
                let user = USER_DATA.read().unwrap();
                let accounts = user.list_accounts();
                let value =
                    serde_json::to_value(accounts).map_err(Box::from)?;
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
