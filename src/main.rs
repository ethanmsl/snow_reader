//! `main.rs`

use snow_read::snow_serde::{Script, ScriptContents, Unload};

fn main() {
    println!("--------------------------------");
    let contents = snow_read::snow_example_long();
    let item_long: Unload =
        serde_xml_rs::from_str(&contents).expect("Failed to parse `&contents` into `Unload`");
    let contents = snow_read::snow_example_search();
    let item_search: Unload =
        serde_xml_rs::from_str(&contents).expect("Failed to parse `&contents` into `Unload`");

    let Script::SysScriptInclude(search_contents) = &item_search.scripts[0] else {panic!("nope")};
    for script in &item_long.scripts {
        let contents = match script {
            Script::SysScriptInclude(x) | Script::SyseventScriptAction(x) => x,
        };
        if search_contents.name == contents.name {
            println!("found: {:?}", search_contents.name);
            println!("search: {:?}", search_contents.description);
            println!("base: {:?}", contents.description);
        }
    }
}
