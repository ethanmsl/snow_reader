//! `main.rs`

mod error;

use std::fs;

use error::Result;
use snow_read::{differ::diff_inline,
                snow_serde::{Script, ScriptContents, Unload}};
use struct_field_names_as_array::FieldNamesAsArray;
use tracing::{Level, debug, info};

// long xml-encoded SNow file
const SNOW_EXAMPLE_LONG_FILE: &str = "snow_files/sys_script_include.xml";
// extracted & altered script-include to use to test search & diff'ing
const SNOW_EXAMPLE_SEARCH_FILE: &str = "snow_files/search_include.xml";
// short xml-encoded SNow file
const _SNOW_EXAMPLE_SHORT_FILE: &str = "snow_files/sysevent_script_action.xml";
// artificial, very short xml-encoded SNow file
const _SNOW_FAKE_SIMPLE_FILE: &str = "snow_files/fake_simple.xml";
const PD_SNOW_FILE: &str = "snow_files/PD Script Includes.xml";

fn main() -> Result<()>
{
        tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();
        info!("Starting up...");
        debug!("hi....");
        println!("--------------------------------");

        // Note: I'm a little surprised a reference works here, as I'd think the String would be immediately dropped.
        let item_long: Unload = serde_xml_rs::from_str(&fs::read_to_string(SNOW_EXAMPLE_LONG_FILE)?)?;
        let item_search: Unload = serde_xml_rs::from_str(&fs::read_to_string(SNOW_EXAMPLE_SEARCH_FILE)?)?;
        debug!(?item_long, "hi....");

        let Script::SysScriptInclude(search_contents) = &item_search.scripts[0] else { panic!("nope") };
        for script in &item_long.scripts {
                let contents = match script {
                        Script::SysScriptInclude(x) | Script::SysEventScriptAction(x) => x,
                };
                if search_contents.name == contents.name {
                        println!("\nATTENTION.");
                        println!("File difference detected in file:
                                    --- {} ---\n",
                                 search_contents.name);
                        let old = contents.script.as_ref().unwrap();
                        let new = search_contents.script.as_ref().unwrap();

                        // diffs & prints
                        diff_inline(old, new);
                }
        }

        println!("--------------------------------");
        let fnames = ScriptContents::FIELD_NAMES_AS_ARRAY;
        println!("fields recorded in (input) script:");
        print!(" {:?}", fnames);

        // println!("--------------------------------");
        // dbg!(item_search);
        //
        //
        let pd_file: Unload = serde_xml_rs::from_str(&fs::read_to_string(PD_SNOW_FILE)?)?;
        // println!("{:#?}", pd_file.scripts);
        for (i, script) in pd_file.scripts.iter().enumerate() {
                let (script_type, contents) = match script {
                        Script::SysScriptInclude(x) => ("ScriptInclude", x),
                        Script::SysEventScriptAction(x) => ("ScriptAction", x),
                };
                info!(i,
                      script_type,
                      name = contents.name,
                      active = contents.active,
                      updated_on = contents.sys_updated_on);
        }
        Ok(())
}
