/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p ansible-inventory-cloud-demo-axum -- 8080
*/

use std::{env, net::SocketAddr, sync::Arc};

use ansible_inventory_cloud::{
    ansible_inventory::script_output::{Host, List},
    http::{
        authentication::{Authentication, AuthenticationType, AuthenticationVerifier},
        host_handler,
        hostname::{GenericHostnameQuery, Hostname, HostnameType},
        list_handler, HostHandler, ListHandler,
    },
};
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    pretty_env_logger::init();

    //
    let port = env::args()
        .nth(1)
        .ok_or("args port missing")?
        .parse::<u16>()
        .map_err(|_| "args port invalid")?;

    //
    let ctx = Arc::new(Context {});

    //
    let app = Router::new()
        .route(
            "/ansible_inventory/list",
            get::<_, (), _, _>(ListHandler::new(
                vec![AuthenticationType::HeaderAuthorizationBearer],
                AuthenticationVerifier::<_, _, _>::sync(|authentication, ctx| {
                    authentication_verify(authentication, ctx)
                }),
                list_handler::Fetcher::<_, _>::new(|v, ctx| Box::pin(list_fetch(v, ctx))),
                ctx.clone(),
            )),
        )
        .route(
            "/ansible_inventory/host",
            get::<_, (), _, _>(HostHandler::new(
                vec![AuthenticationType::HeaderAuthorizationBearer],
                AuthenticationVerifier::<_, _, _>::sync(|authentication, ctx| {
                    authentication_verify(authentication, ctx)
                }),
                vec![HostnameType::Query],
                host_handler::Fetcher::<_, _, _, _>::new(|hostname, v, ctx| {
                    Box::pin(host_fetch(hostname, v, ctx))
                }),
                ctx.clone(),
            )),
        )
        .route(
            "/ansible_inventory/host/:name",
            get::<_, (), _, _>(HostHandler::new(
                vec![AuthenticationType::HeaderAuthorizationBearer],
                AuthenticationVerifier::<_, _, _>::sync(|authentication, ctx| {
                    authentication_verify(authentication, ctx)
                }),
                vec![HostnameType::AxumPath],
                host_handler::Fetcher::<_, _, _, _>::new(|hostname, v, ctx| {
                    Box::pin(host_fetch(hostname, v, ctx))
                }),
                ctx.clone(),
            )),
        )
        .with_state(ctx.clone());

    let addr = format!("127.0.0.1:{port}").parse::<SocketAddr>()?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

//
pub struct Context {}

//
pub fn authentication_verify(
    authentication: Authentication<()>,
    _ctx: Arc<Context>,
) -> Result<(), Box<dyn std::error::Error>> {
    match authentication {
        Authentication::HeaderAuthorizationBearer(token) => {
            if token != "TOKEN" {
                return Err("token mismatch".into());
            }
            Ok(())
        }
        Authentication::Query(_) => Err("Unsupported get authentication from query".into()),
    }
}

//
pub async fn list_fetch(_: (), _ctx: Arc<Context>) -> Result<List, Box<dyn std::error::Error>> {
    Ok(List::default())
}

//
pub async fn host_fetch(
    hostname: Hostname<String, GenericHostnameQuery>,
    _: (),
    _ctx: Arc<Context>,
) -> Result<Host, Box<dyn std::error::Error>> {
    println!("{hostname:?}");
    Ok(Host::default())
}
