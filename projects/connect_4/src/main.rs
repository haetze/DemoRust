use std::io;

const HEIGHT : usize = 16;
const WIDTH : usize  = 16;
const LENGTH : usize = 4;
const ALPHA_LENGTH : usize = WIDTH; // Rename ALPHABET for understanding further down
const ALPHABET: [char; ALPHA_LENGTH] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P'];
                                                  
#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Down,
    Up,
    Left,
    Right,
    UpRight,
    DownLeft,
    DownRight,
    UpLeft,
}

impl Direction {
    fn dirs() -> [Direction; 8] {
        use Direction::*;
        return [Down, Up, Right,Left,UpLeft,DownLeft,UpRight,DownRight];
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum Field {
    X,
    O,
    E,
}

impl Field {
    fn to_char(&self) -> char {
        match self {
            Field::X => 'X',
            Field::O => 'O',
            Field::E => ' ',
        }
    }
}



#[derive(Debug, PartialEq, Copy, Clone)]
enum Board {
    Board([[Field; HEIGHT]; WIDTH]),
}

impl Board {
    fn get(&self) -> &[[Field; HEIGHT]; WIDTH] {
        match self {
            Board::Board(n) => n,
        }
    }

    fn get_mut(&mut self) -> &mut [[Field; HEIGHT]; WIDTH] {
        match self {
            Board::Board(n) => n,
        }
    }

    
    fn new() -> Self {
        Board::Board([[Field::E; HEIGHT]; WIDTH])
    }

    fn check(&self, row: usize, column: usize, dir : Direction) -> bool {
        // When the index is not on the field
        if row >= HEIGHT || column >= WIDTH {
            return false;
        }

        let board = self.get();
        let mut elems : Vec<Field> = Vec::new();
        match dir {
            Direction::Down => {
                if row < LENGTH - 1{
                    return false;
                }
                for i in 0..LENGTH {
                    elems.push(board[row-i][column]);
                }
            },
            
            Direction::Up => {
                if row + LENGTH - 1 >= HEIGHT {
                    return false;
                }
                for i in 0..LENGTH {
                    elems.push(board[row+i][column]);
                }
            },

            Direction::Right => {
                if column + LENGTH - 1 >= WIDTH {
                    return false;
                }
                for i in 0..LENGTH {
                    elems.push(board[row][column+i]);
                }
            },

            Direction::Left => {
                if column < LENGTH - 1 {
                    return false;
                }
                for i in 0..LENGTH {
                    elems.push(board[row][column-i]);
                }
            },

            Direction::UpLeft => {
                if column < LENGTH - 1 || row + LENGTH - 1 >= HEIGHT {
                    return false;
                }
                for i in 0..LENGTH {
                    elems.push(board[row+i][column-i]);
                }
            },

            Direction::DownLeft => {
                if column < LENGTH - 1 || row < LENGTH - 1{
                    return false;
                }
                for i in 0..LENGTH {
                    elems.push(board[row-i][column-i]);
                }
            },
            Direction::UpRight => {
                if column + LENGTH - 1 >= WIDTH || row + LENGTH - 1 >= HEIGHT {
                    return false;
                }
                for i in 0..LENGTH {
                    elems.push(board[row+i][column+i]);
                }
            },

            Direction::DownRight => {
                if column + LENGTH - 1 >= WIDTH || row < LENGTH - 1{
                    return false;
                }
                for i in 0..LENGTH {
                    elems.push(board[row-i][column+i]);
                }
            },
        }
        return elems.into_iter().all(|x| x == board[row][column] && x != Field::E);
        //return elems.into_iter().all(|x| x == board[row][column]);

    }


    fn print(&self) {
        let board = self.get();
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                print!("|{}", board[HEIGHT - i - 1][j].to_char());
            }
            println!("|")
        }
        for i in 0..WIDTH {
            print!("|{}", ALPHABET[i]);
        }
        println!("|")
    }

    fn insert(&mut self, f : Field, column : usize) -> bool{
        let board = self.get_mut();
        for i in 0 .. HEIGHT {
            if board[HEIGHT - i - 1][column] == Field::O ||
                board[HEIGHT - i - 1][column] == Field::X 
                {
                    if HEIGHT - i < HEIGHT && board[HEIGHT - i][column] == Field::E {
                        board[HEIGHT - i][column] = f;
                        return true;
                    }

                }
            if board[HEIGHT - i - 1][column] == Field::E &&
                HEIGHT - i - 1 == 0 {
                    board[0][column] = f;
                    return true;
                }
                

            
        }
        return false;
            
    }

    fn check_whole(&self, player : &mut usize) -> bool {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                for d in Direction::dirs().iter() {
                    if self.check(i, j, *d) {
                        println!("Player {} won:", player);
                        self.print();
                        return true;
                    } 
                }
                
            }
        }
        println!("Next player!");
        *player = (*player + 1) % 2;
        self.print();
        return false;
    }
}

fn num_to_field(n : usize) -> Field{
    if n == 0 {
        return Field::O;
    }
    return Field::X;
}

fn find_pos(c : char) -> Option<usize> {
    for i in 0..ALPHA_LENGTH {
        if ALPHABET[i] == c {
            return Some(i);
        }
    }
    return None;
}

fn handle_input(c : char, board : &mut Board, player : &mut usize) -> bool {
    match find_pos(c) {
        None => {
            println!("Please enter a valid Position!");
        },
        Some(n) => {
            if !board.insert(num_to_field(*player), n) {
                println!("Please enter a different column.");
            } else {
                if board.check_whole(player) {
                    return true;
                }
            }
        },
    }
    return false;
}


fn main() {
    let mut board = Board::new();
    board.print();
    let mut player = 0;
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        match stdin.read_line(&mut input) {
            Ok(n) => {
                if n > 4 {
                    println!("Only enter 1 character");
                } else {
                    let c = input.chars().nth(0).unwrap();
                    if handle_input(c, &mut board, &mut player) {
                        return;
                    }
                }
            },
            Err(error) => {
                println!("error: {}", error);
                return;
            }
        }
    }
    
}
