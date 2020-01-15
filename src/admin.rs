use arc_swap::ArcSwap;
use hyper::{body, Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::router::{Route, RouteMethod, Router};
use crate::utils::GenericResult;

#[derive(Debug, Serialize, Deserialize)]
struct CreateRouteRequest {
    method: RouteMethod,
    path: String,
    handler: String,
}
impl CreateRouteRequest {
    pub async fn new(req: Request<Body>) -> GenericResult<CreateRouteRequest> {
        let bytes = body::to_bytes(req.into_body()).await?;
        let content = String::from_utf8(bytes.to_vec())?;
        match serde_json::from_str(&content) {
            Ok(request) => Ok(request),
            Err(err) => Err(Box::new(err)),
        }
    }
}

pub async fn handle_request(
    req: Request<Body>,
    router_arc: Arc<ArcSwap<Router>>,
) -> GenericResult<Response<Body>> {
    let router = router_arc.load();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/routes") => Ok(Response::new(Body::from(serde_json::to_string(
            &router.routes,
        )?))),
        (&Method::POST, "/routes") => match CreateRouteRequest::new(req).await {
            Ok(create_request) => {
                let route = Route {
                    method: create_request.method,
                    path: create_request.path,
                    handler: create_request.handler,
                };

                let mut new_router = Router::clone(&router);
                new_router.add(route.clone());
                router_arc.store(Arc::new(new_router));

                Ok(Response::new(Body::from(serde_json::to_string(&route)?)))
            }
            Err(error) => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(
                    json!({ "error": format!("{}", error) }).to_string(),
                ))
                .unwrap()),
        },
        _ => {
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
