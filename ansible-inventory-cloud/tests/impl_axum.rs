#![cfg(feature = "impl_axum")]

use std::net::SocketAddr;

use ansible_inventory_cloud::{
    ansible_inventory::script_output::{Host, List},
    http::{
        authentication::{
            Authentication, AuthenticationType, AuthenticationVerifier, GenericAuthenticationQuery,
        },
        host_handler,
        hostname::{GenericHostnameQuery, Hostname, HostnameType},
        list_handler, HostHandler, ListHandler,
    },
};
use axum::{routing::get, Router, Server};
use isahc::{AsyncReadResponseExt, Request, RequestExt as _};

#[tokio::test]
async fn simple() -> Result<(), Box<dyn std::error::Error>> {
    //
    let listen_addr = SocketAddr::from(([127, 0, 0, 1], portpicker::pick_unused_port().unwrap()));
    println!("listen_addr {listen_addr:?}");

    //
    let server_task = tokio::task::spawn(async move {
        let app = Router::new()
            .route(
                "/ansible_inventory/list",
                get::<_, (), _, _>(ListHandler::new(
                    vec![
                        AuthenticationType::HeaderAuthorizationBearer,
                        AuthenticationType::Query,
                    ],
                    AuthenticationVerifier::<_, _, _>::sync(|authentication, ctx| {
                        authentication_verify(authentication, ctx)
                    }),
                    list_handler::Fetcher::<_, _>::new(|v, ctx| Box::pin(list_fetch(v, ctx))),
                    (),
                )),
            )
            .route(
                "/ansible_inventory/host",
                get::<_, (), _, _>(HostHandler::new(
                    vec![
                        AuthenticationType::HeaderAuthorizationBearer,
                        AuthenticationType::Query,
                    ],
                    AuthenticationVerifier::<_, _, _>::sync(|authentication, ctx| {
                        authentication_verify(authentication, ctx)
                    }),
                    vec![HostnameType::Query],
                    host_handler::Fetcher::<_, _, _, _>::new(|hostname, v, ctx| {
                        Box::pin(host_fetch(hostname, v, ctx))
                    }),
                    (),
                )),
            )
            .route(
                "/ansible_inventory/host/:name",
                get::<_, (), _, _>(HostHandler::new(
                    vec![
                        AuthenticationType::HeaderAuthorizationBearer,
                        AuthenticationType::Query,
                    ],
                    AuthenticationVerifier::<_, _, _>::sync(|authentication, ctx| {
                        authentication_verify(authentication, ctx)
                    }),
                    vec![HostnameType::AxumPath],
                    host_handler::Fetcher::<_, _, _, _>::new(|hostname, v, ctx| {
                        Box::pin(host_fetch(hostname, v, ctx))
                    }),
                    (),
                )),
            )
            .with_state(());

        let server = Server::bind(&listen_addr).serve(app.into_make_service());

        server.await.expect("server error");
    });

    //
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    //
    let req = Request::get(format!(
        "http://{}{}",
        listen_addr, "/ansible_inventory/list"
    ))
    .header("Authorization", "Bearer TOKEN_1")
    .body(())?;
    println!("{req:?}");
    let mut resp = req.send_async().await?;
    println!("{resp:?}");
    assert!(resp.status().is_success());
    assert!(serde_json::from_slice::<List>(resp.bytes().await?.as_ref()).is_ok());

    //
    let req = Request::get(format!(
        "http://{}{}",
        listen_addr, "/ansible_inventory/list?access_token=TOKEN_2"
    ))
    .body(())?;
    println!("{req:?}");
    let mut resp = req.send_async().await?;
    println!("{resp:?}");
    assert!(resp.status().is_success());
    assert!(serde_json::from_slice::<List>(resp.bytes().await?.as_ref()).is_ok());

    //
    let req = Request::get(format!(
        "http://{}{}",
        listen_addr, "/ansible_inventory/host/hostname_1"
    ))
    .header("Authorization", "Bearer TOKEN_1")
    .body(())?;
    println!("{req:?}");
    let mut resp = req.send_async().await?;
    println!("{resp:?}");
    assert!(resp.status().is_success());
    assert!(serde_json::from_slice::<Host>(resp.bytes().await?.as_ref()).is_ok());

    //
    let req = Request::get(format!(
        "http://{}{}",
        listen_addr, "/ansible_inventory/host?name=hostname_2"
    ))
    .header("Authorization", "Bearer TOKEN_1")
    .body(())?;
    println!("{req:?}");
    let mut resp = req.send_async().await?;
    println!("{resp:?}");
    assert!(resp.status().is_success());
    assert!(serde_json::from_slice::<Host>(resp.bytes().await?.as_ref()).is_ok());

    //
    server_task.abort();
    assert!(server_task.await.unwrap_err().is_cancelled());

    Ok(())
}

//
pub struct Context {}

//
pub fn authentication_verify(
    authentication: Authentication<GenericAuthenticationQuery>,
    _ctx: (),
) -> Result<(), Box<dyn std::error::Error>> {
    match authentication {
        Authentication::HeaderAuthorizationBearer(access_token) => {
            assert_eq!(access_token, "TOKEN_1");
        }
        Authentication::Query(q) => {
            assert_eq!(q.access_token, "TOKEN_2");
        }
    }

    Ok(())
}

//
pub async fn list_fetch(_: (), _ctx: ()) -> Result<List, Box<dyn std::error::Error>> {
    Ok(List::default())
}

//
pub async fn host_fetch(
    hostname: Hostname<String, GenericHostnameQuery>,
    _: (),
    _ctx: (),
) -> Result<Host, Box<dyn std::error::Error>> {
    match hostname {
        Hostname::AxumPath(path) => {
            assert_eq!(path, "hostname_1");
        }
        Hostname::Query(q) => {
            assert_eq!(q.name, "hostname_2");
        }
    }

    Ok(Host::default())
}
