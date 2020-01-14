use arc_swap::ArcSwap;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::sync::Arc;

use crate::router::Router;
use crate::utils::GenericResult;

pub async fn handle_request(
    req: Request<Body>,
    router_arc: Arc<ArcSwap<Router>>,
) -> GenericResult<Response<Body>> {
    let router = router_arc.load();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/routes") => {
            Ok(Response::new(Body::from(serde_json::to_string(&**router)?)))
        }
        (&Method::POST, "/routes") => {
            let mut new_router = Router::clone(&router);
            new_router.add(
                Method::GET,
                String::from("/route"),
                String::from("handler code"),
            );
            router_arc.store(Arc::new(new_router));

            Ok(Response::new(Body::from("ok")))
        }
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}
