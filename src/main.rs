// use file system

use std::fs;

fn main() {
    println!("Hello, world!");

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
