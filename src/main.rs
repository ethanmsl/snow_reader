//! `main.rs`

use snow_read::differ::diff_inline;
use snow_read::snow_serde::{Script, Unload};

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
            println!("\nATTENTION.");
            println!(
                "File difference detected in file:
                                    --- {} ---\n",
                search_contents.name
            );
            let old = contents.script.as_ref().unwrap();
            let new = search_contents.script.as_ref().unwrap();

            // diffs & prints
            diff_inline(old, new);
        }
    }
}
