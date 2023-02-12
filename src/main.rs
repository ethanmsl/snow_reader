//! `main.rs`

#![allow(clippy::uninlined_format_args)]

use serde_xml_rs::from_str;
use snow_read::snow_obj::Unload;

fn main() {
    // #[derive(Debug, Serialize, Deserialize, PartialEq)]
    // struct PlateAppearance {
    //     #[serde(rename = "$value")]
    //     events: Vec<String>
    // }

    println!("--------------------------------");
    let contents = snow_read::snow_example_art();
    let doc = roxmltree::Document::parse(&contents).unwrap();
    println!("roxmltree doc:\n\n{:?}", doc);

    println!("--------------------------------");
    println!("--------------------------------");
    let contents = snow_read::snow_example_long();
    let item: Unload = from_str(&contents).expect("Failed to parse `&contents` into `Unload`");
    // println!("deserd'd:\n\n{:?}", item.unwrap());
    dbg!(item);
}
