use core::{future::Future, pin::Pin};
use std::sync::Arc;

use ansible_inventory::script_output::List;

use crate::http::authentication::{
    AuthenticationQuery, AuthenticationType, AuthenticationVerifier,
};

//
//
//
#[non_exhaustive]
pub struct ListHandler<AQ, AO, Ctx>
where
    AQ: AuthenticationQuery,
    Ctx: Clone,
{
    pub authentication_types: Vec<AuthenticationType>,
    pub authentication_verifier: AuthenticationVerifier<AQ, AO, Ctx>,
    pub fetcher: Fetcher<AO, Ctx>,
    pub ctx: Ctx,
}

impl<AQ, AO, Ctx> ListHandler<AQ, AO, Ctx>
where
    AQ: AuthenticationQuery,
    Ctx: Clone,
{
    pub fn new(
        authentication_types: Vec<AuthenticationType>,
        authentication_verifier: AuthenticationVerifier<AQ, AO, Ctx>,
        fetcher: Fetcher<AO, Ctx>,
        ctx: Ctx,
    ) -> Self {
        Self {
            authentication_types,
            authentication_verifier,
            fetcher,
            ctx,
        }
    }
}

impl<AQ, AO, Ctx> Clone for ListHandler<AQ, AO, Ctx>
where
    AQ: AuthenticationQuery,
    Ctx: Clone,
{
    fn clone(&self) -> Self {
        Self {
            authentication_types: self.authentication_types.clone(),
            authentication_verifier: self.authentication_verifier.clone(),
            fetcher: self.fetcher.clone(),
            ctx: self.ctx.clone(),
        }
    }
}

//
//
//
pub enum Fetcher<AO, Ctx> {
    Async(
        #[allow(clippy::type_complexity)]
        Arc<
            dyn Fn(
                    AO,
                    Ctx,
                ) -> Pin<
                    Box<
                        dyn Future<Output = Result<List, Box<dyn std::error::Error>>>
                            + Send
                            + 'static,
                    >,
                > + Send
                + Sync,
        >,
    ),
}
impl<AO, Ctx> Clone for Fetcher<AO, Ctx> {
    fn clone(&self) -> Self {
        match self {
            Self::Async(x) => Self::Async(x.clone()),
        }
    }
}

impl<AO, Ctx> Fetcher<AO, Ctx> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(
                AO,
                Ctx,
            ) -> Pin<
                Box<dyn Future<Output = Result<List, Box<dyn std::error::Error>>> + Send + 'static>,
            > + Send
            + Sync
            + 'static,
    {
        Self::Async(Arc::new(f))
    }
}
