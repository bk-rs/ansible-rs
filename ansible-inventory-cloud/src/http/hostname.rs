use serde::{de::DeserializeOwned, Deserialize, Serialize};

//
//
//
#[derive(Debug, Clone)]
pub enum Hostname<HAP, HQ>
where
    HAP: HostnameAxumPath,
    HQ: HostnameQuery,
{
    AxumPath(HAP),
    Query(HQ),
}

#[derive(Debug, Clone, Copy)]
pub enum HostnameType {
    AxumPath,
    Query,
}

//
//
//
pub trait HostnameAxumPath: DeserializeOwned {}
impl<T> HostnameAxumPath for T where T: DeserializeOwned {}

//
//
//
pub trait HostnameQuery: DeserializeOwned {}
impl<T> HostnameQuery for T where T: DeserializeOwned {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GenericHostnameQuery {
    pub name: String,
}
