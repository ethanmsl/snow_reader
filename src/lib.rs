//! Look -- I'm the `lib.rs`'s doc string! :)

use std::{fs, path::Path};
pub mod differ;
mod error;
pub mod snow_serde;
use error::Result;

const EXAMPLE_SCRIPT_INCLUDE: &str = "snow_files/sys_script_include.xml";

/// reads in a long xml-encoded SNOW file
/// (**14_831 lines**)
///
/// # Conceptual Structure:
/// - root
///     - unload
///         - sysevent_script_action (w/action = "INSERT_OR_UPDATE")
///             - various
///             - various
///             - ...
///     - unload
///         - ...
///     - ...
pub fn snow_example_long<P>(path: Option<&P>) -> Result<String>
        where P: AsRef<Path>
{
        let string_path = match path {
                Some(path) => fs::read_to_string(path)?,
                None => fs::read_to_string(EXAMPLE_SCRIPT_INCLUDE)?,
        };
        Ok(string_path)
}

/// extracted & altered script-include to use to test search & diff'ing
pub fn snow_example_search() -> String
{
        fs::read_to_string("snow_files/search_include.xml").expect("\nFile Read Error\n")
}

/// reads in a short xml-encoded SNOW file
/// (uses diff't script type than `sys_script_include`)
/// (**42 lines**)
///
/// # Conceptual Structure:
/// - root
///     - unload
///         - sysevent_script_action (w/action = "INSERT_OR_UPDATE")
///             - various
///             - various
///             - ...
pub fn snow_example_short() -> String
{
        fs::read_to_string("snow_files/sysevent_script_action.xml").expect("\nFile Read Error\n")
}

/// reads in an artificial (very short) xml-encoded SNOW file
/// (**9 lines**)
///
/// # Conceptual Structure:
/// - root
///     - unload
///         - sysevent_script_action (w/action = "INSERT_OR_UPDATE")
///             - various
///             - various
///             - ...
pub fn snow_example_art() -> String
{
        fs::read_to_string("snow_files/fake_simple.xml").expect("\nFile Read Error\n")
}

// // Error specification issue -- I need the internet to figure this out
// /// reads from a (long) xml-encoded SNOW file and a section of raw lines to screen
// pub fn basic_read<U>(lines_to_print: u32) -> Result<(), U:Error> {
//     let contents = snow_example_long();
//     // iterate over in-memory clone of file
//     contents
//         .lines()
//         .take(lines_to_print.try_into()?)
//         .for_each(|line| println!("{}", line));
//
//     Ok(())
// }
