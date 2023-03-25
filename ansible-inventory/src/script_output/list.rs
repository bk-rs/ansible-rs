//! [--list](https://docs.ansible.com/ansible/latest/cli/ansible-inventory.html#cmdoption-ansible-inventory-list)

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::{
    group::{GroupName, GroupVars},
    host::{HostName, HostVars},
};

//
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct List {
    #[serde(rename = "_meta", default)]
    pub meta: ListMeta,
    #[serde(flatten)]
    pub groups: IndexMap<GroupName, ListGroup>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ListMeta {
    pub hostvars: IndexMap<HostName, HostVars>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ListGroup {
    #[serde(default, skip_serializing_if = "IndexSet::is_empty")]
    pub children: IndexSet<GroupName>,
    #[serde(default, skip_serializing_if = "IndexSet::is_empty")]
    pub hosts: IndexSet<HostName>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub vars: GroupVars,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::Value;

    #[test]
    fn test_de_list() {
        match serde_json::from_str::<List>(include_str!(
            "../../tests/script_output_list_json_files/sample_1.json"
        )) {
            Ok(list) => {
                assert_eq!(list.meta.hostvars.len(), 6);
                assert_eq!(
                    list.meta
                        .hostvars
                        .get(&HostName("leaf01".into()))
                        .and_then(|x| x.get("ansible_host")),
                    Some(&Value::from("10.16.10.11"))
                );
                assert_eq!(
                    list.groups
                        .get(&GroupName::Other("leafs".into()))
                        .map(|x| x.hosts.len()),
                    Some(2)
                );
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<List>(include_str!(
            "../../tests/script_output_list_json_files/sample_2_1.json"
        )) {
            Ok(list) => {
                assert!(list.groups.is_empty());
            }
            Err(err) => panic!("{err}"),
        }
        match serde_json::from_str::<List>(include_str!(
            "../../tests/script_output_list_json_files/sample_2_2.json"
        )) {
            Ok(list) => {
                assert!(list.meta.hostvars.is_empty());
                assert!(list.groups.is_empty());
            }
            Err(err) => panic!("{err}"),
        }
        match serde_json::from_str::<List>(include_str!(
            "../../tests/script_output_list_json_files/sample_2_3.json"
        )) {
            Ok(list) => {
                assert!(list.meta.hostvars.is_empty());
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<List>(include_str!(
            "../../tests/script_output_list_json_files/sample_3.json"
        )) {
            Ok(list) => {
                assert!(list.meta.hostvars.is_empty());
            }
            Err(err) => panic!("{err}"),
        }
    }
}
