use core::{future::Future, pin::Pin};
use std::sync::Arc;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

//
//
//
#[derive(Debug, Clone)]
pub enum Authentication<AQ>
where
    AQ: AuthenticationQuery,
{
    HeaderAuthorizationBearer(String),
    Query(AQ),
}

#[derive(Debug, Clone, Copy)]
pub enum AuthenticationType {
    HeaderAuthorizationBearer,
    Query,
}

//
//
//
pub trait AuthenticationQuery: DeserializeOwned {}
impl<T> AuthenticationQuery for T where T: DeserializeOwned {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GenericAuthenticationQuery {
    pub access_token: String,
}

//
//
//
pub enum AuthenticationVerifier<AQ, AO, Ctx>
where
    AQ: AuthenticationQuery,
{
    Sync(
        #[allow(clippy::type_complexity)]
        Arc<
            dyn Fn(Authentication<AQ>, Ctx) -> Result<AO, Box<dyn std::error::Error>> + Send + Sync,
        >,
    ),
    Async(
        #[allow(clippy::type_complexity)]
        Arc<
            dyn Fn(
                    Authentication<AQ>,
                    Ctx,
                ) -> Pin<
                    Box<
                        dyn Future<Output = Result<AO, Box<dyn std::error::Error>>>
                            + Send
                            + 'static,
                    >,
                > + Send
                + Sync,
        >,
    ),
}
impl<AQ, AO, Ctx> Clone for AuthenticationVerifier<AQ, AO, Ctx>
where
    AQ: AuthenticationQuery,
{
    fn clone(&self) -> Self {
        match self {
            Self::Sync(x) => Self::Sync(x.clone()),
            Self::Async(x) => Self::Async(x.clone()),
        }
    }
}

impl<AQ, AO, Ctx> AuthenticationVerifier<AQ, AO, Ctx>
where
    AQ: AuthenticationQuery,
{
    pub fn sync<F>(f: F) -> Self
    where
        F: Fn(Authentication<AQ>, Ctx) -> Result<AO, Box<dyn std::error::Error>>
            + Send
            + Sync
            + 'static,
    {
        Self::Sync(Arc::new(f))
    }

    pub fn r#async<F>(f: F) -> Self
    where
        F: Fn(
                Authentication<AQ>,
                Ctx,
            ) -> Pin<
                Box<dyn Future<Output = Result<AO, Box<dyn std::error::Error>>> + Send + 'static>,
            > + Send
            + Sync
            + 'static,
    {
        Self::Async(Arc::new(f))
    }
}
