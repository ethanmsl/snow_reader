//! struct(s) to define simple SNOW objects

use serde::{Deserialize, Serialize};
// use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Unload {
    sysevent_script_action: Option<Vec<OtherStuff>>,
    sys_script_include: Option<Vec<OtherStuff>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OtherStuff {
    active: Option<String>,
    event_name: Option<String>,
    sys_replace_on_upgrade: Option<String>,
    sys_scope: Option<String>,
}
