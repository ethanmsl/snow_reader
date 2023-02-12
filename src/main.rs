//! `main.rs`

#![allow(clippy::uninlined_format_args)]

use serde_xml_rs::from_str;
use snow_read::snow_obj::Unload;

fn main() {
    println!("--------------------------------");
    let contents = snow_read::snow_example_long();
    let item: Unload = from_str(&contents).expect("Failed to parse `&contents` into `Unload`");
    // println!("deserd'd:\n\n{:?}", item.unwrap());
    dbg!(item);

    println!("--------------------------------");
    let contents = snow_read::snow_example_short();
    let item: Unload = from_str(&contents).expect("Failed to parse `&contents` into `Unload`");
    // println!("deserd'd:\n\n{:?}", item.unwrap());
    dbg!(item);
}
