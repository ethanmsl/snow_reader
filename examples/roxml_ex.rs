//! roxmltree example getting stats from an XML file
//!
//! clear; carrex roxml_ex snow_files/PD\ Business\ rules.xml

use std::collections::HashSet;

fn main()
{
        let args = {
                let args: Vec<_> = std::env::args().collect();

                if args.len() != 2 {
                        println!("Usage:\n\tcargo run --example stats -- input.xml");
                        std::process::exit(1);
                }
                args
        };

        let text = std::fs::read_to_string(&args[1]).unwrap();
        let doc = {
                let opt = roxmltree::ParsingOptions { allow_dtd: true, ..roxmltree::ParsingOptions::default() };
                let doc = match roxmltree::Document::parse_with_options(&text, opt) {
                        Ok(v) => v,
                        Err(e) => {
                                println!("Error: {}.", e);
                                std::process::exit(1);
                        }
                };
                doc
        };

        // some prints
        {
                for (i, node) in doc.root().descendants().enumerate() {
                        println!("element {i}:\n{:?}\n", node);
                }
                println!("\nInput:\n---\n{}\n---\n", doc.input_text());
                println!("Elements count: {}", doc.root().descendants().filter(|n| n.is_element()).count());
        }

        let attrs_count: usize = doc.root().descendants().map(|n| n.attributes().len()).sum();
        let ns_count: usize = doc.root().descendants().map(|n| n.namespaces().len()).sum();

        println!("Attributes count: {}", attrs_count);
        println!("Namespaces count: {}", ns_count);

        let mut uris = HashSet::new();
        for node in doc.root().descendants() {
                for ns in node.namespaces() {
                        uris.insert((ns.name().unwrap_or("\"\"").to_string(), ns.uri().to_string()));
                }
        }
        println!("Unique namespaces count: {}", uris.len());
        if !uris.is_empty() {
                println!("Unique namespaces:");
                for (key, value) in uris {
                        println!("  {:?}: {}", key, value);
                }
        }

        println!("Comments count: {}", doc.root().descendants().filter(|n| n.is_comment()).count());

        println!("Comments:");
        for node in doc.root().descendants().filter(|n| n.is_comment()) {
                println!("{:?}", node.text().unwrap());
        }
}
