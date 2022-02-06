use async_trait::async_trait;
use json_rpc2::{futures::*, Request, Response, Result, from_str};
use serde_json::Value;

struct ServiceHandler;

#[async_trait]
impl Service for ServiceHandler {
    type Data = ();
    async fn handle(
        &self,
        request: &Request,
        _ctx: &Self::Data,
    ) -> Result<Option<Response>> {
        let response = match request.method() {
            "hello" => {
                let params: String = request.deserialize()?;
                let message = format!("Hello, {}!", params);
                Some((request, Value::String(message)).into())
            }
            _ => None,
        };
        Ok(response)
    }
}

pub(crate) async fn handle(message: &str) -> Result<Option<Response>> {
    let request = from_str(message)?;
    let service: Box<dyn Service<Data = ()>> = Box::new(ServiceHandler {});
    let server = Server::new(vec![&service]);
    let response = server.serve(&request, &()).await;
    Ok(response)
}
