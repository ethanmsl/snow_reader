//! Look -- I'm the `lib.rs`'s doc string! :)

use std::fs;

/// An example function
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// reads from a file and prints
pub fn basic_read() {
    // read in file
    let contents =
        fs::read_to_string("snow_files/sys_script_include.xml").expect("\nFile Read Error\n");

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
