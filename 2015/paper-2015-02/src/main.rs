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

    // declare the string to append
    let mut line: String = String::new();
    
    // declare your prism
    // let (mut l,mut w,mut h)= (0,0,0);

    let mut total = 0;
    // read the first line
    while buf_reader.read_line(&mut line).unwrap() > 0 {
        total += slice_prism(&mut line);
        line.clear();
    }
    println!("{}",total);

}

fn slice_prism(line: &mut String) -> u32 {
    
    // declare the sides of prism [h,l,w]
    let mut prism:Vec<u32> = vec![0,0,0];
    
    // declare the index we are in.
    let mut index:usize = 0;

    // declare decimal point
    let mut point = 0;

    // extract each sides
    for _i in 0..line.len() {
        // pop the last number and assign to num
        let num = line.pop();
        
        // match the value.
        match num {
            Some('x') => {
                point = 0;
                index += 1;
                continue
            },
            _ => { 
                // try without if let.
                if let Some(side) = num.unwrap().to_digit(10) {
                    prism[index] += side*10_u32.pow(point);
                    point += 1;
                }
            },
        }

    }

    // return 2*l*w + 2*w*h + 2*h*l
    2*(prism[1]*prism[2] + prism[2]*prism[0] + prism[0]*prism[1])

}