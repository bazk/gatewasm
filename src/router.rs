use arc_swap::ArcSwap;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::utils::GenericResult;

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
    pub fn from_hyper_method(method: &Method) -> Self {
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

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct Route {
//     method: RouteMethod,
//     path: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Router {
    routes: HashMap<String, HashMap<RouteMethod, String>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn find(&self, method: &Method, path: &str) -> Option<&String> {
        match self.routes.get(path) {
            Some(methods) => methods.get(&RouteMethod::from_hyper_method(&method)),
            None => None,
        }
    }

    pub fn add(&mut self, method: Method, path: String, handler: String) {
        let method = RouteMethod::from_hyper_method(&method);

        match self.routes.get_mut(&path) {
            Some(methods) => {
                methods.insert(method, handler);
            }
            None => {
                let mut methods = HashMap::new();
                methods.insert(method, handler);
                self.routes.insert(path, methods);
            }
        }
    }
}

async fn calc(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn handle_request(
    req: Request<Body>,
    router: Arc<ArcSwap<Router>>,
) -> GenericResult<Response<Body>> {
    let router = router.load();

    match router.find(req.method(), req.uri().path()) {
        Some(route) => {
            calc(2, 3).await;
            Ok(Response::new(Body::from(String::from(route))))
        }
        None => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}
