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
                Some(c) => Err(OperatorParseError(Some(c), input)),
                
            }
        }

        let input: String = input.chars().rev().collect();
        match read_intern(input) {
            Err(OperatorParseError(x, input)) => Err(OperatorParseError(x, input.chars().rev().collect())),
            Ok((op, input)) => Ok((op, input.chars().rev().collect())),
            _ => panic!("Tried to read Operator and an unexpected error happend")
        }
    }
}

#[derive(Debug)]
enum Exp {
    Com(Op, Box<Exp>, Box<Exp>),
    Num(f64),
}


impl Exp {
    fn fold_num(self) -> f64 {
        use Op::*;
        use Exp::*;
        
        match self {
            Num(number) => number,
            Com(op, operand_1, operand_2) => {
                match op {
                    Plus => operand_1.fold_num() + operand_2.fold_num(),
                    Mins => operand_1.fold_num() - operand_2.fold_num(),
                    Mult => operand_1.fold_num() * operand_2.fold_num(),
                    Divs => operand_1.fold_num() / operand_2.fold_num(),
                }
            }
        }
    }

    fn read_number(input: String) -> Result<(f64, String), ReadError> {
        use ReadError::ExpectedNumberError;
        
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
        }
    }

}


                    


fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    print!("<=<=<=<= ");
    stdout.flush().ok();
    
    for line in stdin.lock().lines() {
        
        
        match line {
            Ok(exp_string) => {
                if exp_string == ":q" {
                    println!("Quitting..");
                    break;
                }
                match Exp::read(exp_string) {
                    Err(error) => println!(">!>!>!>! {:?}", error), 
                    Ok((exp, _)) => println!(">=>=>=>= {:?}", exp.fold_num()),
                };
            },
            Err(_) => panic!("Read Error"),
        }

        print!("<=<=<=<= ");
        stdout.flush().ok();
    }
    
}
