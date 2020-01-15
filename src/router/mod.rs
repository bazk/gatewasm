mod error;
mod route;

use arc_swap::ArcSwap;
use hyper::{Body, Request, Response, StatusCode};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

use crate::utils::GenericResult;

pub use error::RouterError;
pub use route::{Route, RouteHandler, RouteMethod};

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

    pub fn find(&self, method: &RouteMethod, path: &str) -> Option<&RouteHandler> {
        match self.index.get(path) {
            Some(methods) => match methods.get(method) {
                Some(&route_index) => Some(&self.routes[route_index].handler),
                None => None,
            },
            None => None,
        }
    }

    pub fn add(&mut self, route: Route) -> Result<(), RouterError> {
        for r in self.routes.iter() {
            if r.conflicts(&route) {
                return Err(RouterError::RouteAlreadyExists);
            }
        }

        self.routes.push(route);
        self.rebuild_index();
        Ok(())
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

pub async fn handle_request(
    req: Request<Body>,
    router: Arc<ArcSwap<Router>>,
) -> GenericResult<Response<Body>> {
    let router = router.load();

    match router.find(&RouteMethod::from_hyper(&req.method()), req.uri().path()) {
        Some(handler) => match handler.run() {
            Ok(ret) => Ok(Response::builder()
                .body(Body::from(json!({ "result": ret }).to_string()))
                .unwrap()),
            Err(error) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(
                    json!({ "error": format!("{}", error) }).to_string(),
                ))
                .unwrap()),
        },
        None => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(
                    json!({ "error": "route not found" }).to_string(),
                ))
                .unwrap())
        }
    }
}
