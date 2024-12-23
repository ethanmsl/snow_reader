//! Attempting auto-derivation using
//!
//! `xml_schema_generator snow_files/fake_simple.xml --derive 'Serialize, Deserialize, Debug' | pbcopy`

use std::fs;

use quick_xml::de;
use serde::{Deserialize, Serialize};

const SNOW_SCRIPT_INCLUDE_FILE: &str = "snow_files/sys_script_include.xml";

fn main() -> Result<(), Box<dyn core::error::Error>> {
        println!("------------------------------------------");
        println!("Reading from file:\n{}", SNOW_SCRIPT_INCLUDE_FILE);
        println!("------------------------------------------");

        let simple_xml_string: String = fs::read_to_string(SNOW_SCRIPT_INCLUDE_FILE)?;

        println!("------------------------------------------");
        println!("Raw string read:\n{}", simple_xml_string);
        println!("------------------------------------------");

        let test_serd_xml: Unload = de::from_str(&fs::read_to_string(SNOW_SCRIPT_INCLUDE_FILE)?)?;
        println!("------------------------------------------");
        println!("Debug view of derived struct:\n{:?}", test_serd_xml);
        println!("------------------------------------------");

        // let config: Config = from_str(XML)?;
        Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unload {
        #[serde(rename = "@unload_date")]
        pub unload_date:        String,
        #[serde(rename = "$text")]
        pub text:               Option<String>,
        pub sys_script_include: Vec<SysScriptInclude>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SysScriptInclude {
        #[serde(rename = "@action")]
        pub action:                 String,
        #[serde(rename = "$text")]
        pub text:                   Option<String>,
        pub access:                 String,
        pub active:                 String,
        pub api_name:               String,
        pub caller_access:          CallerAccess,
        pub client_callable:        String,
        pub description:            String,
        pub name:                   String,
        pub script:                 String,
        pub sys_class_name:         String,
        pub sys_created_by:         String,
        pub sys_created_on:         String,
        pub sys_customer_update:    String,
        pub sys_id:                 String,
        pub sys_mod_count:          String,
        pub sys_name:               String,
        pub sys_package:            SysPackage,
        pub sys_policy:             SysPolicy,
        pub sys_replace_on_upgrade: String,
        pub sys_scope:              SysScope,
        pub sys_update_name:        String,
        pub sys_updated_by:         String,
        pub sys_updated_on:         String,
        pub u_checked_out:          UCheckedOut,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallerAccess {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SysPackage {
        #[serde(rename = "@display_value")]
        pub display_value: String,
        #[serde(rename = "@source")]
        pub source:        String,
        #[serde(rename = "$text")]
        pub text:          Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SysPolicy {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SysScope {
        #[serde(rename = "@display_value")]
        pub display_value: String,
        #[serde(rename = "$text")]
        pub text:          Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UCheckedOut {}
