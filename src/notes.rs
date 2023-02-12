//! this is not connected to `lib.rs` nor `main.rs`
//! it's just here as a repository of information

use roxmltree;

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
