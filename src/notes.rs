//! this is not connected to `lib.rs` nor `main.rs`
//! it's just here as a repository of information

use roxmltree;
use snow_read::differ::diff_inline;
use snow_read::snow_serde::{Script, ScriptContents, Unload};

/////////////////////////////////////////////////////////
// old main printing
pub fn fake_main() {
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
}

/////////////////////////////////////////////////////////
// a `roxmltree` based parsing
// What's **nice**: is that it gives all the info
// (whereas `xml_serde_rs` seems to toss out attribute info)

pub fn roxml_parse() {
    println!("--------------------------------");
    let contents = snow_read::snow_example_art();
    let doc = roxmltree::Document::parse(&contents).unwrap();
    println!("roxmltree doc:\n\n{:?}", doc);
}
