//! [host.py](https://github.com/ansible/ansible/blob/devel/lib/ansible/inventory/host.py)

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

//
wrapping_macro::wrapping_string!(
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct HostName(pub String);
);

wrapping_macro::wrapping!(
    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct HostVars(pub Map<String, Value>);
);

//
#[derive(Debug, Clone)]
pub struct Host {
    // TODO,
}
