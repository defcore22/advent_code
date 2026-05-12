#[allow(unused_variables)]
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    // floor variable
    let mut floor = 0;
    // open the file
    let f = File::open("input.txt").unwrap();
    // buffer for efficient reading
    let reader = BufReader::new(f);
    

    for i in reader.bytes() {

        let i = i.unwrap();


        if i == b'(' {
            floor += 1;
        } else if i == b')' {
            floor -= 1;
        }
    }

    println!("{}", floor);

}