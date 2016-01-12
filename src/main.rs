extern crate advent_16;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }

    'outer: for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        let checks = vec!["children: 3".to_string(),
                          "cats: 7".to_string(),
                          "samoyeds: 2".to_string(),
                          "pomeranians: 3".to_string(),
                          "akitas: 0".to_string(),
                          "vizslas: 0".to_string(),
                          "goldfish: 5".to_string(),
                          "trees: 3".to_string(),
                          "cars: 2".to_string(),
                          "perfumes: 1".to_string()];

        'inner: for check in &checks {
            if !advent_16::find_input(&line.to_string(), &check) {
                continue 'outer;
            }
        }

        println!("{}", line);
        break;
    }
}

