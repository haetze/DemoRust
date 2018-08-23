#![allow(dead_code)]
#![feature(box_patterns, box_syntax)]
#![feature(extern_prelude)]

use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::io::BufRead;
use std::env;



mod lines;
use lines::read_in_file;
use lines::handle_line;
mod terms;
use terms::*;
use terms::types::Type;

fn main() -> Result<(), ()>{

    let mut vars: HashMap<String, Term> = HashMap::new();
    let mut context: HashMap<String, Type> = HashMap::new();
    let paths: Vec<String>           = env::args().skip(1).collect();
    
    for path in &paths {
        read_in_file(path, &mut vars, &mut context);
    }
    
    let stdin = io::stdin();

    print!("<=<=<=<= ");
    io::stdout().flush().ok();
    
    for line in stdin.lock().lines() {
        
        if handle_line(line, &mut vars, &mut context, &paths) {
            break;
        }
        print!("<=<=<=<= ");
        io::stdout().flush().ok();
 
    }

    Ok(())
}
