//! `main.rs`

use serde_xml_rs::from_str;
use snow_read::snow_serde::{Script, Unload};

fn main() {
    println!("--------------------------------");
    let contents = snow_read::snow_example_long();
    let item_long: Unload = from_str(&contents).expect("Failed to parse `&contents` into `Unload`");
    // println!("deserd'd:\n\n{:?}", item.unwrap());
    // dbg!(item_long);

    println!("--------------------------------");
    let contents = snow_read::snow_example_short();
    let item_short: Unload =
        from_str(&contents).expect("Failed to parse `&contents` into `Unload`");
    // println!("deserd'd:\n\n{:?}", item.unwrap());
    let _item_short = dbg!(item_short);

    println!("--------------------------------");
    for elem in &item_long.scripts {
        match elem {
            Script::SysScriptInclude(x) | Script::SyseventScriptAction(x) => {
                println!("\n{:?}", x.name)
            }
        }
    }
}
