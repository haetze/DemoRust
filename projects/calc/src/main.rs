use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug)]
enum ReadError {
    OperatorParseError(Option<char>, String),
    ExpectedNumberError(Option<char>, String),
}

#[derive(Debug)]
enum Op {
    Plus,
    Mins,
    Mult,
    Divs,
    Powr,
    Modu,
    Rond,
}

impl Op {
    fn read(input: String) -> Result<(Op, String), ReadError> {
        use Op::*;
        use ReadError::*;
            
        fn read_intern(mut input: String) -> Result<(Op, String), ReadError> {
            match input.pop() {
                None => Err(OperatorParseError(None, input)),
                Some('+') => Ok((Plus, input)),
                Some('-') => Ok((Mins, input)),
                Some('*') => Ok((Mult, input)),
                Some('/') => Ok((Divs, input)),
                Some('^') => Ok((Powr, input)),
                Some('%') => Ok((Modu, input)),
                Some('&') => Ok((Rond, input)),
                Some(c) => {
                    input.push(c);
                    Err(OperatorParseError(Some(c), input))
                },
            }
        }

        let input: String = input.trim_left().chars().rev().collect();
        match read_intern(input) {
            Err(OperatorParseError(x, input)) => Err(OperatorParseError(x, input.chars().rev().collect())),
            Ok((op, input)) => Ok((op, input.chars().rev().collect())),
            _ => panic!("Tried to read Operator and an unexpected error happend")
        }
    }
}

#[derive(Debug)]
enum Exp {
    Num(f64),
    Single(Op, Box<Exp>),
    Com(Op, Box<Exp>, Box<Exp>),
}


impl Exp {
    fn fold_num(self) -> f64 {
        use Op::*;
        use Exp::*;
        
        match self {
            Num(number) => number,
            Single(op, operand_1) => {
                match op {
                    Rond => operand_1.fold_num().round(),
                    Plus => operand_1.fold_num(),
                    Mult => operand_1.fold_num(),
                    Divs => 1.0 / operand_1.fold_num(),
                    Mins => operand_1.fold_num() * -1.0,
                    Powr => operand_1.fold_num().powf(2.0),
                    _    => panic!("Error unexpected Operator"),
                }
            },
            Com(op, operand_1, operand_2) => {
                match op {
                    Plus => operand_1.fold_num() + operand_2.fold_num(),
                    Mins => operand_1.fold_num() - operand_2.fold_num(),
                    Mult => operand_1.fold_num() * operand_2.fold_num(),
                    Divs => operand_1.fold_num() / operand_2.fold_num(),
                    Powr => operand_1.fold_num().powf(operand_2.fold_num()),
                    Modu => operand_1.fold_num() % operand_2.fold_num(),
                    Rond => panic!("Error unexpected Operator"),
                }
            },
        }
    }

    fn read_number(input: String) -> Result<(f64, String), ReadError> {
        use ReadError::ExpectedNumberError;
        let input: String = input.trim_left().chars().collect();
        
        let number: String = input.chars()
            .into_iter()
            .take_while(|c| char::is_numeric(*c) || *c == '.')
            .collect();
        let input : String = input.chars()
            .into_iter()
            .skip_while(|c| char::is_numeric(*c) || *c == '.')
            .collect();

        match number.parse() {
            Ok(num) => Ok((num, input)),
            Err(_)  => Err(ExpectedNumberError(number.chars().into_iter().nth(0), input))
        }
    }

    fn read_from_high_prior(operand_1: Exp, operator: Op, input: String)
                       -> Result<(Exp, String), ReadError> {
        use Exp::*;
        
        let (number, input) = Exp::read_number(input)?;
        if input.is_empty() {
            return Ok((Com(operator,
                           Box::new(operand_1),
                           Box::new(Num(number))),
                       input));
        }
        let (operator_2, input) = Op::read(input)?;
        let (right, input)      = Exp::read(input)?;
        let left = Com(operator, Box::new(operand_1), Box::new(Num(number)));
        let total = Com(operator_2, Box::new(left), Box::new(right));
        Ok((total, input))
    }

    fn read_from_low_prior(operand_1: Exp, operator: Op, input: String)
                           -> Result<(Exp, String), ReadError> {
        use Exp::Com;
        
        let (operand_2, input) = Exp::read(input)?;
        Ok((Com(operator, Box::new(operand_1), Box::new(operand_2)), input))
    }
    
    fn read(input: String) -> Result<(Exp, String), ReadError> {
        use Op::*;
        use Exp::*;
        use ReadError::*;
        
        match Op::read(input) {
            Ok((operator, rest_input)) => {
                let (exp, string) = Exp::read(rest_input)?;
                Ok((Single(operator, Box::new(exp)), string))
            },
            Err(OperatorParseError(_, input))  => {
                let (operand_1, input) = Exp::read_number(input)?;
                if input.is_empty() {
                    return Ok((Num(operand_1), input));
                }
                let (operator, input) = Op::read(input)?;
                match operator {
                    Divs => Exp::read_from_high_prior(Num(operand_1), operator, input),
                    Mult => Exp::read_from_high_prior(Num(operand_1), operator, input),
                    Mins => Exp::read_from_low_prior(Num(operand_1), operator, input),
                    Plus => Exp::read_from_low_prior(Num(operand_1), operator, input),
                    Powr => Exp::read_from_low_prior(Num(operand_1), operator, input),
                    Modu => Exp::read_from_low_prior(Num(operand_1), operator, input),
                    Rond => Ok((Single(operator, Box::new(Num(operand_1))), input)),
                }
            },
            _ => panic!("Error"),
            
        }
    }

}


fn handle_line(line: Result<String, std::io::Error>) -> bool{
    
    let mut stdout = io::stdout();

    match line {
        Ok(exp_string) => {
            match exp_string.as_str() {
                ":q" => {
                    println!("Quitting..");
                    return true;
                },
                _ => {
                    match Exp::read(exp_string) {
                        Err(error) => println!(">!>!>!>! {:?}", error), 
                        Ok((exp, _)) => println!(">=>=>=>= {:?}", exp.fold_num()),
                    };
                },
            }
        },
        Err(_) => panic!("Read Error"),
    }
    
    print!("<=<=<=<= ");
    stdout.flush().ok();
    return false;
}
                    


fn main() {
   
    
    let stdin = io::stdin();

    print!("<=<=<=<= ");
    io::stdout().flush().ok();
    
    for line in stdin.lock().lines() {
        
        if handle_line(line) {
            break;
        }
 
    }
    
}
