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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    pub method: RouteMethod,
    pub handler: String,
}

#[derive(Debug, Clone)]
pub struct Router {
    pub routes: Vec<Route>,
    index: HashMap<String, HashMap<RouteMethod, usize>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub fn find(&self, method: &RouteMethod, path: &str) -> Option<&str> {
        match self.index.get(path) {
            Some(methods) => match methods.get(method) {
                Some(&route_index) => Some(&self.routes[route_index].handler),
                None => None,
            },
            None => None,
        }
    }

    pub fn add(&mut self, route: Route) {
        // Remove the route if it already exists
        match self
            .routes
            .iter()
            .position(|r| r.path == route.path && r.method == route.method)
        {
            Some(index) => {
                self.routes.remove(index);
            }
            None => (),
        }

        // Create the route and push it to the list
        self.routes.push(route);

        self.rebuild_index();
    }

    fn rebuild_index(&mut self) {
        for (route_index, route) in self.routes.iter().enumerate() {
            match self.index.get_mut(&route.path) {
                Some(methods) => {
                    methods.insert(route.method.clone(), route_index);
                }
                None => {
                    let mut methods = HashMap::new();
                    methods.insert(route.method.clone(), route_index);
                    self.index.insert(route.path.clone(), methods);
                }
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

    match router.find(
        &RouteMethod::from_hyper_method(&req.method()),
        req.uri().path(),
    ) {
        Some(handler) => {
            calc(2, 3).await;
            Ok(Response::new(Body::from(String::from(handler))))
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
