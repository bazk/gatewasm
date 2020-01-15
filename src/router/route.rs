use hyper::Method;
use serde::{Deserialize, Serialize};
use wasmer_runtime::{error, imports, instantiate, Value};

use super::error::RouterError;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub enum RouteMethod {
    OPTIONS,
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    TRACE,
    CONNECT,
    PATCH,
}

impl RouteMethod {
    pub fn from_hyper(method: &Method) -> Self {
        match method {
            &Method::OPTIONS => Self::OPTIONS,
            &Method::GET => Self::GET,
            &Method::POST => Self::POST,
            &Method::PUT => Self::PUT,
            &Method::DELETE => Self::DELETE,
            &Method::HEAD => Self::HEAD,
            &Method::TRACE => Self::TRACE,
            &Method::CONNECT => Self::CONNECT,
            &Method::PATCH => Self::PATCH,
            _ => panic!(format!("unknown method: {:?}", method)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RouteHandler {
    #[serde(with = "crate::utils::serde_base64")]
    code: Vec<u8>,
}

impl RouteHandler {
    pub fn run(&self) -> Result<i32, RouterError> {
        match RouteHandler::execute_wasm(&self.code) {
            Ok(Value::I32(ret)) => Ok(ret),
            _ => Err(RouterError::HandlerExecutionFailed),
        }
    }

    fn execute_wasm(code: &Vec<u8>) -> error::Result<Value> {
        let import_object = imports! {};
        let instance = instantiate(code.as_slice(), &import_object)?;
        let mut values = instance.dyn_func("add_one")?.call(&[Value::I32(42)])?;
        Ok(values.pop().unwrap())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    pub method: RouteMethod,
    pub handler: RouteHandler,
}

impl Route {
    pub fn conflicts(&self, other: &Route) -> bool {
        self.path == other.path && self.method == other.method
    }
}
