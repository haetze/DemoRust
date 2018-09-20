pub mod types;
pub mod vali32;
pub mod eval;
pub mod typable;
pub mod show;
pub mod read;
pub mod matching;
mod valbool;
mod var;
mod lambda;
mod app;
mod build_ins;

use terms::types::Type;
use terms::show::Show;
use terms::typable::Typable;
use terms::eval::Evaluate;
use terms::vali32::ValI32;
use terms::valbool::ValBool;
use terms::var::Var;
use terms::lambda::Lambda;
use terms::app::App;
use terms::build_ins::BuildIns;
use std::collections::HashMap;



#[derive(Debug, Clone, PartialEq)]
pub enum Term{
    ValI32(ValI32),
    ValBool(ValBool),
    Var(Var),
    Lambda(Lambda),
    App(App),
    BuildIn(BuildIns),
}



impl Evaluate for Term {
    fn eval(self, context: &mut HashMap<String, Term>) -> Term {
        match self {
                Term::ValI32(v) => v.eval(context),
                Term::ValBool(v) => v.eval(context),
                Term::Var(v) => v.eval(context),
                Term::Lambda(v) => v.eval(context),
                Term::App(v) => v.eval(context),
                Term::BuildIn(v) => v.eval(context),
        }
    }
}


impl Typable for Term {
    fn get_type(&self) -> &Type {
        match self {
            Term::ValI32(v) => v.get_type(),
            Term::ValBool(v) => v.get_type(),
            Term::Var(v) => v.get_type(),
            Term::Lambda(v) => v.get_type(),
            Term::App(v) => v.get_type(),
            Term::BuildIn(b) => b.get_type(),
        }
    }
}


impl Show for Term {
    fn show(&self) -> String {
        match self {
            Term::ValI32(v) => v.show(),
            Term::ValBool(v) => v.show(),
            Term::Var(v) => v.show(),
            Term::Lambda(v) => v.show(),
            Term::App(v) => v.show(),
            Term::BuildIn(b) => b.show(),
        }
    }
}



