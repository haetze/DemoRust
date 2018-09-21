#![allow(dead_code)]
#![feature(box_patterns, box_syntax)]
#![feature(extern_prelude)]
extern crate liner;

use liner::Context;

use std::collections::HashMap;
use std::io;
use std::io::Write;
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
    


    let mut con = Context::new();
    
    loop {
        let line = con.read_line("<=<=<=< ", &mut |_| {});
        match &line {
            Ok(s) => con.history.push(s.clone().into()),
            _     => Ok(()),
        }.expect("Some Real Problem");
        if handle_line(line, &mut vars, &mut context, &paths) {
            break;
        }
        print!("<=<=<=<= ");
        io::stdout().flush().ok();
 
    }

    Ok(())
}
