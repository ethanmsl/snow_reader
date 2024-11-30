//! Quick-XML repurposed example
//!
//! `clear; clear; RUST_LOG=quickxml_ex=trace carrex quickxml_ex`

use quick_xml::{events::Event, reader::Reader};
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

fn main() {
        tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .pretty()
                .init();

        // let xml = r#"
        //      <tag1 att1 = "test">
        //         <tag2><!--Test comment-->Test</tag2>
        //         <tag2>Test 2</tag2>
        //      </tag1>"#;
        let xml = r#"
                <?xml version="1.0" encoding="UTF-8" ?>
                <unload unload_date="2022-10-14 22:34:04">
                    <herring1></herring1>
                    <herring2 />
                    <sysevent_script_action action="INSERT_OR_UPDATE">
                        <active>true</active>
                        <event_name>x_pd_integration.add_inc_external_ref</event_name>
                        <sys_replace_on_upgrade>false</sys_replace_on_upgrade>
                        <sys_scope display_value="PagerDuty Platform for Real-Time Operations">39a9d9664f834e00dd657bb28110c77b</sys_scope>
                    </sysevent_script_action>
                </unload>"#;

        // WARN: appears to assume ASCII [u8], rather than UTF-8
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut count = 0;
        let mut txt_vals = Vec::new();
        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
                // NOTE: this is the generic case when we don't know about the input BufRead.
                // when the input is a &str or a &[u8], we don't actually need to use another
                // buffer, we could directly call `reader.read_event()`
                match reader.read_event_into(&mut buf) {
                        Ok(Event::Start(e)) => {
                                info!(?e, "Start of Event");
                                println!(
                                        "attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                                );
                                count += 1;
                        }
                        Ok(Event::Text(e)) => {
                                info!(?e, "Event text!");
                                txt_vals.push(e.unescape().unwrap().into_owned())
                        }

                        // There are several other `Event`s we do not consider here
                        Err(e) => {
                                error!(?e, "Error at position {}", reader.buffer_position());
                                panic!("Error at position {}: {:?}", reader.error_position(), e)
                        }
                        Ok(Event::Eof) => {
                                warn!("End of File reached");
                                break;
                        }
                        _ => (),
                }
                // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
                buf.clear();
        }
        println!("Found {} items", count);
        println!("text: {:?}", txt_vals);
}
