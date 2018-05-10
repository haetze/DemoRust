use std::fmt;

#[derive(Debug)]
enum RegExp<'a>{
    Const(String),
    Seq(&'a RegExp<'a>, &'a RegExp<'a>),
    Par(&'a RegExp<'a>, &'a RegExp<'a>),
    Rep(&'a RegExp<'a>),
    Empty,
    Epsilon,
}

impl<'a> fmt::Display for RegExp<'a> {
    fn fmt(&self,  f: &mut fmt::Formatter) -> fmt::Result {
        use RegExp::*;
        let string = match *self {
            Const(ref s) => format!("{}", s),
            Seq(r, s)    => format!("{}{}", r, s),
            Par(r, s)    => format!("{}+{}", r, s),
            Rep(r)       => format!("({}^+)", r),
            Empty        => "empty".to_string(),
            Epsilon      => "eps".to_string(),
            _        => "".to_string(),
        };
        write!(f, "{}", string)
    }
}

fn repeat<'a>(r: &'a RegExp) -> RegExp<'a> {
    RegExp::Rep(r)
}

fn main() {
    use RegExp::*;
    let a = Const("a".to_string());
    let plus_a = Rep(&a);
    let star_a = Par(&Epsilon, &plus_a);
    println!("{}", star_a);
}
