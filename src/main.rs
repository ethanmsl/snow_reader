//! `main.rs`

use snow_read::snow_serde::{Script, ScriptContents, Unload};

fn main() {
    println!("--------------------------------");
    let contents = snow_read::snow_example_long();
    let item_long: Unload =
        serde_xml_rs::from_str(&contents).expect("Failed to parse `&contents` into `Unload`");
    // dbg!(item_long);

    println!("--------------------------------");
    let contents = snow_read::snow_example_short();
    let item_short: Unload =
        serde_xml_rs::from_str(&contents).expect("Failed to parse `&contents` into `Unload`");
    let _item_short = dbg!(item_short);

    println!("--------------------------------");
    for elem in &item_long.scripts {
        match elem {
            Script::SysScriptInclude(x) | Script::SyseventScriptAction(x) => {
                println!("\n{:?}", x.name)
            }
        }
    }

    println!("--------------------------------");
    let fnames = ScriptContents::FIELD_NAMES_AS_ARRAY;
    println!("fields: {:?}", fnames);

    println!("--------------------------------");
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
