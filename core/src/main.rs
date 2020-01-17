mod admin;
mod router;
mod utils;

use arc_swap::ArcSwap;
use futures_util::future::join;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::sync::Arc;

use crate::router::Router;
use crate::utils::GenericError;

#[tokio::main]
async fn main() {
    let router = Arc::new(ArcSwap::from(Arc::new(Router::new())));

    let gw_addr = ([0, 0, 0, 0], 8700).into();
    let adm_addr = ([0, 0, 0, 0], 8900).into();

    let gw_server = Server::bind(&gw_addr).serve(make_service_fn(|_| {
        let router = Arc::clone(&router);
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                router::handle_request(req, Arc::to_owned(&router))
            }))
        }
    }));

    let adm_server = Server::bind(&adm_addr).serve(make_service_fn(|_| {
        let router = Arc::clone(&router);
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                admin::handle_request(req, Arc::to_owned(&router))
            }))
        }
    }));

    println!("Listening on http://{} and http://{}", gw_addr, adm_addr);

    let _ret = join(gw_server, adm_server).await;
}
