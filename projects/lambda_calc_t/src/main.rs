#![allow(dead_code)]
#![feature(box_patterns, box_syntax)]

use std::collections::HashMap;

mod terms;
use terms::*;
use terms::types::Show;

fn main() {
    let mut context = HashMap::new();
    let mut example_string = "x".to_string();
    match read_term(&mut example_string, &mut context) {
        Ok(exp) => println!("Type of {}: {}", exp.show(), exp.get_type().show()),
        Err(_) => panic!("Error while reading"),
    }

    let mut example_string = "(λx.3)".to_string();
    match read_term(&mut example_string, &mut context) {
        Ok(exp) => println!("Type of {}: {}", exp.show(), exp.get_type().show()),
        Err(_) => panic!("Error while reading"),
    }

    let mut example_string = "(λx.b)".to_string();
    match read_term(&mut example_string, &mut context) {
        Ok(exp) => println!("Type of {}: {}", exp.show(), exp.get_type().show()),
        Err(_) => panic!("Error while reading"),
    }

}
