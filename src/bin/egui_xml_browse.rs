//! Attempting to look through struct with egui
//!
//! `xml_schema_generator snow_files/fake_simple.xml --derive 'Serialize, Deserialize, Debug' | pbcopy`

use std::fs;

use eframe::run_native;
use egui::{CentralPanel, ScrollArea};
use egui_extras::syntax_highlighting::{CodeTheme, code_view_ui};
use quick_xml::de;
use serde::{Deserialize, Serialize};

const SNOW_SCRIPT_INCLUDE_FILE: &str = "snow_files/sys_script_include.xml";

/// Egui-Eframe app to page through xml values.
struct XmlBrowser {
        xml:        Unload,
        index:      usize,
        source:     String,
        raw_string: String,
}

impl eframe::App for XmlBrowser {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
                CentralPanel::default().show(ctx, |ui| {
                        ui.horizontal(|ui| {
                                if ui.button("prev").clicked() {
                                        self.index = self.index.saturating_sub(1);
                                }
                                if ui.button("next").clicked() && self.index < self.xml.sys_script_include.len() {
                                        self.index += 1;
                                };
                                ui.label(format!(
                                        "Script {} of {}",
                                        self.index + 1,
                                        self.xml.sys_script_include.len()
                                ));
                        });

                        ui.add_space(8.0);
                        if let Some(script) = self.xml.sys_script_include.get(self.index) {
                                ui.heading(&script.name);
                                ui.label(format!("API Name: {}", &script.api_name));
                                ui.add_space(8.0);
                                // ScrollArea::vertical()
                                //         .auto_shrink(false)
                                //         .show(ui, |ui| ui.label(&script.script));
                                // ui.add_space(8.0);
                                ScrollArea::vertical()
                                        .auto_shrink(false)
                                        .show(ui, |ui| code_view_ui(ui, &CodeTheme::dark(12.), &script.script, "js"));
                        };
                });
        }
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
        let source = SNOW_SCRIPT_INCLUDE_FILE;
        let raw_string: String = fs::read_to_string(source)?;
        let xml: Unload = de::from_str(&fs::read_to_string(source)?)?;
        let native_options = eframe::NativeOptions::default();

        let app_instantiated = XmlBrowser {
                xml,
                index: 0,
                source: source.to_string(),
                raw_string,
        };
        run_native(
                "XML Displayer",
                native_options,
                Box::new(|_cc| Ok(Box::new(app_instantiated))),
        )?;

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
