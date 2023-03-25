//! [group.py](https://github.com/ansible/ansible/blob/devel/lib/ansible/inventory/group.py)

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GroupName {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "ungrouped")]
    Ungrouped,
    #[serde(other)]
    Other(String),
}

wrapping_macro::wrapping!(
    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct GroupVars(pub Map<String, Value>);
);

//
#[derive(Debug, Clone)]
pub struct Group {
    // TODO,
}
