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
use terms::matching::Match;


use std::collections::HashSet;
use terms::show::Show;

fn main() {
    let mut locals: HashSet<_> = HashSet::new();
    let mut globals: HashMap<String, Term> = HashMap::new();
    let mut context: HashMap<String, Type> = HashMap::new();
    let mut t_1_s = "((cons 1) ((cons 2) nil))".to_string();
    let t_1 = read::read_term(&mut t_1_s,
                              &mut context,
                              &mut locals,
                              &mut globals).unwrap();
    println!("{}", t_1.show());


    let mut t_2_s = "((cons 1) ((cons n) nil))".to_string();
    let t_2 = read::read_term(&mut t_2_s,
                              &mut context,
                              &mut locals,
                              &mut globals).unwrap();
    println!("{}", t_2.show());

    let mut t_3_s = "n".to_string();
    let t_3 = read::read_term(&mut t_3_s,
                              &mut context,
                              &mut locals,
                              &mut globals).unwrap();
    println!("{}", t_3.show());

    let m = Match::new(t_1, t_2, t_3);

    let t_4 = m.exec_match().unwrap();
    println!("{}", t_4.show());

    
}

// fn main() -> Result<(), ()>{

//     let mut vars: HashMap<String, Term> = HashMap::new();
//     let mut context: HashMap<String, Type> = HashMap::new();
//     let paths: Vec<String>           = env::args().skip(1).collect();
    
//     for path in &paths {
//         read_in_file(path, &mut vars, &mut context);
//     }
    


//     let mut con = Context::new();
    
//     loop {
//         let line = con.read_line("<=<=<=< ", &mut |_| {});
//         match &line {
//             Ok(s) => con.history.push(s.clone().into()),
//             _     => Ok(()),
//         }.expect("Some Real Problem");
//         if handle_line(line, &mut vars, &mut context, &paths) {
//             break;
//         }
//         print!("<=<=<=<= ");
//         io::stdout().flush().ok();
 
//     }

//     Ok(())
// }
