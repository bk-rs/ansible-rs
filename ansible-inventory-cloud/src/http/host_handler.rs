use core::{future::Future, pin::Pin};
use std::sync::Arc;

use ansible_inventory::script_output::Host;

use crate::http::{
    authentication::{AuthenticationQuery, AuthenticationType, AuthenticationVerifier},
    hostname::{Hostname, HostnameAxumPath, HostnameQuery, HostnameType},
};

//
#[non_exhaustive]
pub struct HostHandler<AQ, AO, HAP, HQ, Ctx>
where
    AQ: AuthenticationQuery,
    HAP: HostnameAxumPath,
    HQ: HostnameQuery,
    Ctx: Clone,
{
    pub authentication_types: Vec<AuthenticationType>,
    pub authentication_verifier: AuthenticationVerifier<AQ, AO, Ctx>,
    pub hostname_types: Vec<HostnameType>,
    pub fetcher: Fetcher<AO, HAP, HQ, Ctx>,
    pub ctx: Ctx,
}

impl<AQ, AO, HAP, HQ, Ctx> HostHandler<AQ, AO, HAP, HQ, Ctx>
where
    AQ: AuthenticationQuery,
    HAP: HostnameAxumPath,
    HQ: HostnameQuery,
    Ctx: Clone,
{
    pub fn new(
        authentication_types: Vec<AuthenticationType>,
        authentication_verifier: AuthenticationVerifier<AQ, AO, Ctx>,
        hostname_types: Vec<HostnameType>,
        fetcher: Fetcher<AO, HAP, HQ, Ctx>,
        ctx: Ctx,
    ) -> Self {
        Self {
            authentication_types,
            authentication_verifier,
            hostname_types,
            fetcher,
            ctx,
        }
    }
}

impl<AQ, AO, HAP, HQ, Ctx> Clone for HostHandler<AQ, AO, HAP, HQ, Ctx>
where
    AQ: AuthenticationQuery,
    HAP: HostnameAxumPath,
    HQ: HostnameQuery,
    Ctx: Clone,
{
    fn clone(&self) -> Self {
        Self {
            authentication_types: self.authentication_types.clone(),
            authentication_verifier: self.authentication_verifier.clone(),
            hostname_types: self.hostname_types.clone(),
            fetcher: self.fetcher.clone(),
            ctx: self.ctx.clone(),
        }
    }
}

//
//
//
pub enum Fetcher<AO, HAP, HQ, Ctx>
where
    HAP: HostnameAxumPath,
    HQ: HostnameQuery,
{
    Async(
        #[allow(clippy::type_complexity)]
        Arc<
            dyn Fn(
                    Hostname<HAP, HQ>,
                    AO,
                    Ctx,
                ) -> Pin<
                    Box<
                        dyn Future<Output = Result<Host, Box<dyn std::error::Error>>>
                            + Send
                            + 'static,
                    >,
                > + Send
                + Sync,
        >,
    ),
}
impl<AO, HAP, HQ, Ctx> Clone for Fetcher<AO, HAP, HQ, Ctx>
where
    HAP: HostnameAxumPath,
    HQ: HostnameQuery,
{
    fn clone(&self) -> Self {
        match self {
            Self::Async(x) => Self::Async(x.clone()),
        }
    }
}

impl<AO, HAP, HQ, Ctx> Fetcher<AO, HAP, HQ, Ctx>
where
    HAP: HostnameAxumPath,
    HQ: HostnameQuery,
{
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(
                Hostname<HAP, HQ>,
                AO,
                Ctx,
            ) -> Pin<
                Box<dyn Future<Output = Result<Host, Box<dyn std::error::Error>>> + Send + 'static>,
            > + Send
            + Sync
            + 'static,
    {
        Self::Async(Arc::new(f))
    }
}
