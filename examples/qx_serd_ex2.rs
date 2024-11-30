//! Quick-XMl + Serde
//!
//! Generate schema:
//! xml_schema_generator snow_files/fake_complex.xml --derive 'Serialize, Deserialize, Debug' | pbcopy
//!
//! Run program:
//! clear; RUST_LOG=qx_serd_ex=trace carrex qx_serd_ex

// note: to use serde, the feature needs to be enabled

use std::collections::HashMap;

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

const SNOW_FAKE_EXAMPLE: &str = r#"
<?xml version="1.0" encoding="UTF-8" ?>
<unload unload_date="2022-10-14 22:34:04">
    <herring1>smack ad oo
    </herring1>
    <herring2 />
    <sysevent_script_action action="INSERT_OR_UPDATE">
        <active>true</active>
        <event_name>x_pd_integration.add_inc_external_ref</event_name>
        <sys_replace_on_upgrade>false</sys_replace_on_upgrade>
        <sys_scope display_value="PagerDuty Platform for Real-Time Operations">39a9d9664f834e00dd657bb28110c77b</sys_scope>
    </sysevent_script_action>
    <action_insert>true</action_insert>
    <action_query>false</action_query>
    <is_rest>false</is_rest>
    <45/>
    <xxyxx_script_action action="INSERT_OR_UPDATE">
        <active>true</active>
        <event_name>x_pd_integration.add_inc_external_ref</event_name>
        <sys_replace_on_upgrade>false</sys_replace_on_upgrade>
        <sys_scope display_value="PagerDuty Platform for Real-Time Operations">39a9d9664f834e00dd657bb28110c77b</sys_scope>
    </xxyxx_script_action>
</unload>"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct Unload {
        #[serde(rename = "@unload_date")]
        pub unload_date:            String,
        #[serde(rename = "$text")]
        pub text:                   Option<String>,
        pub herring1:               Herring1,
        pub herring2:               Herring2,
        pub sysevent_script_action: SyseventScriptAction,
        #[serde(flatten)]
        pub other:                  HashMap<String, serde_json::Value>,
        // pub action_insert:          String,
        // pub action_query:           String,
        // pub is_rest:                String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterCondition {
        #[serde(rename = "@table")]
        pub table: String,
        #[serde(rename = "$text")]
        pub text:  Option<String>,
        pub item:  Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
        #[serde(rename = "@goto")]
        pub goto:     String,
        #[serde(rename = "@or")]
        pub or:       String,
        #[serde(rename = "@field")]
        pub field:    String,
        #[serde(rename = "@endquery")]
        pub endquery: String,
        #[serde(rename = "@value")]
        pub value:    String,
        #[serde(rename = "@operator")]
        pub operator: String,
        #[serde(rename = "@newquery")]
        pub newquery: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt()
                .with_timer(tracing_subscriber::fmt::time::SystemTime)
                .pretty()
                .with_env_filter(EnvFilter::from_default_env())
                .init();

        let config: Config = from_str(XML)?;
        dbg!(config);

        let snow_xml: Unload = match quick_xml::de::from_str(SNOW_FAKE_EXAMPLE) {
                Ok(x) => x,
                Err(e) => panic!("Error: {}", e),
        };
        dbg!(snow_xml);

        Ok(())
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct Config {
        #[serde(rename = "DefaultSettings")]
        settings:     DefaultSettings,
        localization: Localization,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
struct DefaultSettings {
        #[serde(rename = "@Language")]
        language: String,
        #[serde(rename = "@Greeting")]
        greeting: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Localization {
        translation: Vec<Translation>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
struct Translation {
        #[serde(rename = "@Tag")]
        tag:  String,
        #[serde(rename = "@Language")]
        lang: String,
        #[serde(rename = "$text")]
        text: String,
}
const XML: &str = r#"
<?xml version="1.0" encoding="utf-8"?>
<Config>
  <DefaultSettings Language="es" Greeting="HELLO"/>
  <Localization>
    <Translation Tag="HELLO" Language="ja">
      こんにちは
    </Translation>
    <Translation Tag="BYE" Language="ja">
      さようなら
    </Translation>
    <Translation Tag="HELLO" Language="es">
      Hola
    </Translation>
    <Translation Tag="BYE" Language="es">
      Adiós
    </Translation>
  </Localization>
</Config>
"#;
