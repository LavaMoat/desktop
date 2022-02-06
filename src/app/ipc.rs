use json_rpc2::{from_str, Request, Response, Result, Server, Service};
use serde_json::Value;

struct ServiceHandler;
impl Service for ServiceHandler {
    type Data = ();
    fn handle(
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

pub(crate) fn handle(message: &str) -> Result<Option<Response>> {
    let request = from_str(message)?;
    let service: Box<dyn Service<Data = ()>> = Box::new(ServiceHandler {});
    let server = Server::new(vec![&service]);
    let response = server.serve(&request, &());
    Ok(response)
}
