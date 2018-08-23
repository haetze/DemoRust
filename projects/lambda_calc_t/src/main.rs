#![allow(dead_code)]
#![feature(box_patterns, box_notation)]

use std::collections::HashMap;

mod terms;
use terms::*;
use terms::types::Show;

fn main() {
    let mut context = HashMap::new();
    let var_1 = Var::new("x".to_string(), &mut context);
    println!("Type of {:?}: {}", var_1.show(), var_1.get_type().show());
    println!("{:?}", context);
    
    let var_2 = Var::new("y".to_string(), &mut context);
    println!("Type of {:?}: {}", var_2.show(), var_2.get_type().show());
    println!("{:?}", context);

    let val_1 = ValI32::new(1);
    println!("Type of {:?}: {}", val_1.show(), val_1.get_type().show());
    println!("{:?}", context);

    let app_1 = App::new(Term::Var(var_1.clone()), Term::Var(var_2.clone()), &mut context);
    match app_1 {
        Ok(app_1) => println!("Type of {:?}: {}", app_1.show(), app_1.get_type().show()),
        Err(e)    => println!("{:?}", e),
    }
    println!("{:?}", context);

    let lam_1 = Lambda::new(var_2.clone(), Term::ValI32(val_1.clone()));
    println!("Type of {:?}: {}", lam_1.show(), lam_1.get_type().show());
    
    println!("{:?}", context);
}
