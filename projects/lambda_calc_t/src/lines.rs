use terms::Term;
use terms::read::read_term;
use terms::read::read_var;
use terms::eval::Evaluate;
use terms::typable::Typable;
use terms::show::Show;

use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;


enum Kind {
    Quit,
    Reload,
    Term(String),
    Set(String),
    Type(String),
}


fn read_kind(mut st: String) -> Kind {
    if st.as_str() == ":q" {
        return Kind::Quit;
    }
    if st.as_str().starts_with(":t") {
        st.remove(0);
        st.remove(0);
        st.remove(0);
        return Kind::Type(st);
    }
    if st.as_str() == ":r" {
        return Kind::Reload;
    }
    if st.as_str().starts_with(":set") {
        st.remove(0);
        st.remove(0);
        st.remove(0);
        st.remove(0);
        st.remove(0);
        return Kind::Set(st);
    }
    return Kind::Term(st);

}


pub fn handle_line(line: Result<String, std::io::Error>,
                   vars: &mut HashMap<String, Term>,
                   paths: &Vec<String>) -> bool {
    let mut context = HashMap::new();
    for (k, v) in vars.iter() {
        context.insert(k.clone(), v.get_type().clone());
    }
    let context = &mut context;
    match line {
        Ok(exp_string) => {
            match read_kind(exp_string) {
                Kind::Quit => {
                    println!("Quitting..");
                    return true;
                },
                Kind::Type(mut exp_string) => {
                    let mut locals = HashSet::new();
                    match read_term(&mut exp_string, context, &mut locals, vars) {
                        Err(()) => println!(">!>!>!>! Error"),
                        Ok(exp) => {
                            println!(">=>=>=>= {} : {}", exp.get_type().show(), exp.show())
                        },
                    };
                },
                Kind::Reload => {
                    println!("Reloading..");
                    let mut map = HashMap::new();
                    for path in paths.iter() {
                        read_in_file(path, &mut map);
                    }
                    std::mem::swap(vars, &mut map);
                    return false;
                },
                Kind::Term(mut exp_string) => {
                    let mut locals = HashSet::new();
                    match read_term(&mut exp_string, context, &mut locals, vars) {
                        Err(()) => println!(">!>!>!>! Error"),
                        Ok(exp) => {
                            let exp = exp.eval(vars);
                            println!(">=>=>=>= {} : {}", exp.get_type().show(), exp.show())
                        },
                    };
                },
                Kind::Set(mut st) => {
                    if let Ok(Term::Var(var)) = read_var(&mut st, context, false) {
                        st.remove(0);
                        let mut locals = HashSet::new();
                        match read_term(&mut st, context, &mut locals, vars) {
                            Err(()) => println!(">!>!>!>! Error in Term"), 
                            Ok(exp) => {
                                println!(">=>=>=>= :set {} <- {}", var.show(), exp.show());
                                vars.insert(var.get_var(), exp);
                            },
                        };
                    } else {
                        println!(">!>!>!>! Error in Var");
                    }
                },
            }
        },
        Err(_) => println!("Read Error"),
    }
    
    return false;
}

pub fn read_in_file(path: &String,
                vars: &mut HashMap<String, Term>,) {
    let file = File::open(path).expect("file not found");
    let file = BufReader::new(&file);
    for line in file.lines() {
        handle_line(line, vars, &Vec::new());
    }
    
}
