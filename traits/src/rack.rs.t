use crate::{Msg, Response, Transport};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;
use std::sync::Arc;

#[derive(Clone, Copy)]
struct SimTransport {
    requests: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Transport for SimTransport {
    fn dial(&self, addr: &str, msg: Msg) -> Result<Response, Box<dyn Error>> {
        Ok(Response {
            body: "".to_string(),
        })
    }
}
