// writting a program that calculates paper needed for elves. 
// read line by line and parse
// to find slack: L x W
// dimensions: L x W x H

use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    // open the file
    let f = File::open("presents.txt").unwrap();
    // move the portion of the file into RAM
    let mut buf_reader = BufReader::new(f);

    // declare string to appnd
    let mut line: String = String::new();
    
    // declare your prism
    let (mut l,mut w,mut h)= (0,0,0);

    let mut total = 0;
    // read the first line
    while buf_reader.read_line(&mut line).unwrap() > 0 {
        
    }
    println!("{}",total);

}

fn slice_prism(line:String) -> i32 {
    
    // declare the sides of prism
    let mut prism = vec![0,0,0];
    
    // declare the index we are in.
    let mut index = 0;

    // loop throuh the line and seperate l,w,h.
    while index < 3 {

        // extract each sides
        'Outer: for i in 0..line.len() {
            // pop the last number and assign to num
            let num = line.pop();
            
            // match the value.
            match num {
                None => break 'Outer,
                Some('x') => continue,
                _ => {
                    let side:u32 = num.to_digit();
                    prism[index] = num;
                },
            }

        }
        // increment index
        index += 1;
    }

    let mut present = 0;
    present

}