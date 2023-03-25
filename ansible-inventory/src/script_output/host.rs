//! [--host <HOST>](https://docs.ansible.com/ansible/latest/cli/ansible-inventory.html#cmdoption-ansible-inventory-host)

use serde::{Deserialize, Serialize};

use crate::host::HostVars;

//
wrapping_macro::wrapping!(
    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct Host(pub HostVars);
);

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::Value;

    #[test]
    fn test_de_host() {
        match serde_json::from_str::<Host>(include_str!(
            "../../tests/script_output_host_json_files/sample_3.json"
        )) {
            Ok(host) => {
                assert_eq!(host.get("VAR001"), Some(&Value::from("VALUE")));
                assert_eq!(host.get("VAR002"), Some(&Value::from("VALUE")));
            }
            Err(err) => panic!("{err}"),
        }
    }
}
