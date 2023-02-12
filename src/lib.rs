//! Look -- I'm the `lib.rs`'s doc string! :)

use std::fs;

/// An example function
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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
pub fn snow_example_long() -> String {
    let contents =
        fs::read_to_string("snow_files/sys_script_include.xml").expect("\nFile Read Error\n");
    contents
}

/// reads in a short xml-encoded SNOW file
/// (**42 lines**)
///
/// # Conceptual Structure:
/// - root
///     - unload
///         - sysevent_script_action (w/action = "INSERT_OR_UPDATE")
///             - various
///             - various
///             - ...
pub fn snow_example_short() -> String {
    let contents =
        fs::read_to_string("snow_files/sysevent_script_action.xml").expect("\nFile Read Error\n");
    contents
}

/// reads from a file and prints
pub fn basic_read() {
    let contents = snow_example_long();

    // iterate over in-memory clone of file
    contents
        .lines()
        .take(40)
        .for_each(|line| println!("{}", line));

    // // set up a stream to read file from
    // let mut stream = fs::File::open("snow_files/sys_script_include.xml").expect("\nFile Read Error\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// An example test
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
