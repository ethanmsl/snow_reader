//! struct(s) to define simple SNOW objects

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Unload {
    sysevent_script_action: Option<Vec<ScriptContents>>,
    sys_script_include: Option<Vec<ScriptContents>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ScriptContents {
    // access: Option<String>,
    // active: Option<String>,
    // api_name: Option<String>,
    // client_callable: Option<String>,
    // description: Option<String>,
    name: Option<String>,
    // script: Option<String>,
    // event_name: Option<String>,
    // sys_class_name: Option<String>,
    // sys_created_by: Option<String>,
    // sys_created_on: Option<String>,
    // sys_customer_update: Option<String>,
    // sys_id: Option<String>,
    // sys_mod_count: Option<String>,
    // sys_name: Option<String>,
    // sys_packaged_display_value: Option<String>,
    // sys_replace_on_upgrade: Option<String>,
    // sys_scope_display_value: Option<String>,
    // sys_update_name: Option<String>,
    // sys_updated_by: Option<String>,
    // sys_updated_on: Option<String>,
}






/////////////////////////////////////////
// this set doesn't capture info, but seems like it should
// the above doesn't seem like it should capture more than one script
// but captures all their contents, but errantly as children on the same level

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Unload2 {
    // sysevent_script_action: Option<Vec<ScriptContents>>,
    #[serde(rename = "$value")]
    scripts: Vec<Script>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Script {
    SysScriptInclude(SysScriptInclude),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SysScriptInclude {
    // sysevent_script_action: Option<Vec<ScriptContents>>,
    sys_script_include: Option<ScriptContents>,
}
