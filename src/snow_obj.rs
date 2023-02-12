//! struct(s) to define simple SNOW objects

use serde::{Deserialize, Serialize};

/// the Parent(most) structure
/// a series of script types
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Unload {
    // sysevent_script_action: Option<Vec<ScriptContents>>,
    #[serde(rename = "$value")]
    pub scripts: Vec<Script>,
}

/// the kinds of Scripts that can be found
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Script {
    SysScriptInclude(ScriptContents),
    SyseventScriptAction(ScriptContents),
}

/// represents the content elements of an individual script
/// Currently all set as "Option" -- as I'm uncertain regarding consistency
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ScriptContents {
    pub access: Option<String>,
    pub active: Option<String>,
    pub api_name: Option<String>,
    pub client_callable: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub script: Option<String>,
    pub event_name: Option<String>,
    pub sys_class_name: Option<String>,
    pub sys_created_by: Option<String>,
    pub sys_created_on: Option<String>,
    pub sys_customer_update: Option<String>,
    pub sys_id: Option<String>,
    pub sys_mod_count: Option<String>,
    pub sys_name: Option<String>,
    pub sys_packaged_display_value: Option<String>,
    pub sys_replace_on_upgrade: Option<String>,
    pub sys_scope_display_value: Option<String>,
    pub sys_update_name: Option<String>,
    pub sys_updated_by: Option<String>,
    pub sys_updated_on: Option<String>,
}
