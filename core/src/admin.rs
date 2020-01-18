use arc_swap::ArcSwap;
use hyper::{body, Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::router::{Route, RouteHandler, RouteMethod, Router, RouterError};
use crate::utils::GenericResult;

#[derive(Debug, Serialize, Deserialize)]
struct CreateRouteRequest {
    method: RouteMethod,
    path: String,
    handler: RouteHandler,
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
        (&Method::GET, "/routes") => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header(
                "Access-Control-Allow-Methods",
                "GET, POST, PUT, PATCH, DELETE, OPTIONS",
            )
            .header("Access-Control-Allow-Headers", "Content-Type")
            .header("Access-Control-Max-Age", "600")
            .body(Body::from(serde_json::to_string(&router.routes)?))
            .unwrap()),
        (&Method::POST, "/routes") => match CreateRouteRequest::new(req).await {
            Ok(create_request) => {
                let route = Route {
                    method: create_request.method,
                    path: create_request.path,
                    handler: create_request.handler,
                };

                let mut new_router = Router::clone(&router);
                match new_router.add(route.clone()) {
                    Ok(_) => {
                        router_arc.store(Arc::new(new_router));
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header("Access-Control-Allow-Origin", "*")
                            .header(
                                "Access-Control-Allow-Methods",
                                "GET, POST, PUT, PATCH, DELETE, OPTIONS",
                            )
                            .header("Access-Control-Allow-Headers", "Content-Type")
                            .header("Access-Control-Max-Age", "600")
                            .body(Body::from(serde_json::to_string(&route)?))
                            .unwrap())
                    }
                    Err(error) => match error {
                        RouterError::RouteAlreadyExists => Ok(Response::builder()
                            .status(StatusCode::CONFLICT)
                            .header("Access-Control-Allow-Origin", "*")
                            .header(
                                "Access-Control-Allow-Methods",
                                "GET, POST, PUT, PATCH, DELETE, OPTIONS",
                            )
                            .header("Access-Control-Allow-Headers", "Content-Type")
                            .header("Access-Control-Max-Age", "600")
                            .body(Body::from(
                                json!({ "error": format!("{}", error) }).to_string(),
                            ))
                            .unwrap()),
                    },
                }
            }
            Err(error) => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Access-Control-Allow-Origin", "*")
                .header(
                    "Access-Control-Allow-Methods",
                    "GET, POST, PUT, PATCH, DELETE, OPTIONS",
                )
                .header("Access-Control-Allow-Headers", "Content-Type")
                .header("Access-Control-Max-Age", "600")
                .body(Body::from(
                    json!({ "error": format!("{}", error) }).to_string(),
                ))
                .unwrap()),
        },
        (&Method::OPTIONS, _) => Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header("Access-Control-Allow-Origin", "*")
            .header(
                "Access-Control-Allow-Methods",
                "GET, POST, PUT, PATCH, DELETE, OPTIONS",
            )
            .header("Access-Control-Allow-Headers", "Content-Type")
            .header("Access-Control-Max-Age", "600")
            .body(Body::empty())
            .unwrap()),
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
