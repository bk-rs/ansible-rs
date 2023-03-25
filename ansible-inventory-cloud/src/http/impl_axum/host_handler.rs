use core::{future::Future, pin::Pin};

use axum::{
    body::Body,
    extract::{FromRequestParts as _, Path, Query, TypedHeader},
    handler::Handler,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    response::{IntoResponse as _, Json, Response},
};

use crate::http::{
    authentication::{
        Authentication, AuthenticationQuery, AuthenticationType, AuthenticationVerifier,
    },
    host_handler::{Fetcher, HostHandler},
    hostname::{Hostname, HostnameAxumPath, HostnameQuery, HostnameType},
};

//
impl<T, S, AQ, AO, HAP, HQ, Ctx> Handler<T, S, Body> for HostHandler<AQ, AO, HAP, HQ, Ctx>
where
    S: Send + Sync + 'static,
    AQ: AuthenticationQuery + Send + 'static,
    AO: Send + 'static,
    HAP: HostnameAxumPath + Send + 'static,
    HQ: HostnameQuery + Send + 'static,
    Ctx: Clone + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

    fn call(self, req: Request<Body>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, _) = req.into_parts();

            let mut authentication = None;
            for authentication_type in self.authentication_types {
                match authentication_type {
                    AuthenticationType::HeaderAuthorizationBearer => {
                        if let Ok(bearer) =
                            TypedHeader::<Authorization<Bearer>>::from_request_parts(
                                &mut parts, &state,
                            )
                            .await
                        {
                            authentication = Some(Authentication::HeaderAuthorizationBearer(
                                bearer.token().into(),
                            ));
                        }
                    }
                    AuthenticationType::Query => {
                        if let Ok(Query(query)) =
                            Query::<AQ>::from_request_parts(&mut parts, &state).await
                        {
                            authentication = Some(Authentication::Query(query));
                        }
                    }
                }
            }
            let authentication = if let Some(authentication) = authentication {
                authentication
            } else {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "authentication not found",
                )
                    .into_response();
            };

            let mut hostname = None;
            for hostname_type in self.hostname_types {
                match hostname_type {
                    HostnameType::AxumPath => {
                        if let Ok(Path(path)) =
                            Path::<HAP>::from_request_parts(&mut parts, &state).await
                        {
                            hostname = Some(Hostname::AxumPath(path));
                        }
                    }
                    HostnameType::Query => {
                        if let Ok(Query(query)) =
                            Query::<HQ>::from_request_parts(&mut parts, &state).await
                        {
                            hostname = Some(Hostname::Query(query));
                        }
                    }
                }
            }
            let hostname = if let Some(hostname) = hostname {
                hostname
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, "hostname not found").into_response();
            };

            let v = match &self.authentication_verifier {
                AuthenticationVerifier::Sync(f) => match f(authentication, self.ctx.clone()) {
                    Ok(x) => x,
                    Err(err) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("authentication verify failed, err:{err}"),
                        )
                            .into_response();
                    }
                },
                AuthenticationVerifier::Async(f) => match f(authentication, self.ctx.clone()).await
                {
                    Ok(x) => x,
                    Err(err) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("authentication verify failed, err:{err}"),
                        )
                            .into_response();
                    }
                },
            };

            let host = match &self.fetcher {
                Fetcher::Async(f) => match f(hostname, v, self.ctx.clone()).await {
                    Ok(x) => x,
                    Err(err) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("fetch failed, err:{err}"),
                        )
                            .into_response();
                    }
                },
            };

            Json(host).into_response()
        })
    }
}
