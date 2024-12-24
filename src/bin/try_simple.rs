//! Attempting auto-derivation using
//!
//! `xml_schema_generator snow_files/fake_simple.xml --derive 'Serialize, Deserialize, Debug' | pbcopy`

use std::fs;

use quick_xml::de;
use serde::{Deserialize, Serialize};

const SNOW_FAKE_SIMPLE_FILE: &str = "snow_files/fake_simple.xml";

fn main() -> Result<(), Box<dyn core::error::Error>> {
        println!("------------------------------------------");
        println!("Reading from file:\n{}", SNOW_FAKE_SIMPLE_FILE);
        println!("------------------------------------------");

        let simple_xml_string: String = fs::read_to_string(SNOW_FAKE_SIMPLE_FILE)?;

        println!("------------------------------------------");
        println!("Raw string read of xml:\n{}", simple_xml_string);
        println!("------------------------------------------");

        let test_serd_xml: Unload = de::from_str(&fs::read_to_string(SNOW_FAKE_SIMPLE_FILE)?)?;
        println!("------------------------------------------");
        println!("Debug version of derived struct:\n{:?}", test_serd_xml);
        println!("------------------------------------------");

        // let config: Config = from_str(XML)?;
        Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Unload {
        #[serde(rename = "@unload_date")]
        pub unload_date:            String,
        #[serde(rename = "$text")]
        pub text:                   Option<String>,
        pub herring1:               Herring1,
        pub herring2:               Herring2,
        pub sysevent_script_action: SyseventScriptAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Herring1 {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Herring2 {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyseventScriptAction {
        #[serde(rename = "@action")]
        pub action:                 String,
        #[serde(rename = "$text")]
        pub text:                   Option<String>,
        pub active:                 String,
        pub event_name:             String,
        pub sys_replace_on_upgrade: String,
        pub sys_scope:              SysScope,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SysScope {
        #[serde(rename = "@display_value")]
        pub display_value: String,
        #[serde(rename = "$text")]
        pub text:          Option<String>,
}
