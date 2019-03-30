#[derive(Clone,Copy,Debug, PartialEq)]
enum Field {
    Set,
    Empty,
    Blocked,
}

const SIZE : usize = 8;

type Row = [Field;SIZE];
type Board = [Row;SIZE];

fn count_set(row: &Row) -> u32 {
    row.iter().fold(0, |acc, e| {
        if e == &Field::Set {
            acc + 1
        } else {
            acc
        }
    })
}

fn row_valid(row: &Row) -> bool {
    count_set(row) <= 1
}

fn rows_valid(board: &Board) -> bool {
    board.iter().all(|r| row_valid(r))
}

fn board_rotate(board: &Board) -> Board {
    let row = [Field::Set; SIZE];
    let mut new_board = [row; SIZE]; // Is completely overwritten; just init
    for x in 0..SIZE {
        for y in 0..SIZE {
            new_board[y][x] = board[x][y].clone();
        }
    }
    new_board
}

fn columns_valid(board: &Board) -> bool {
    rows_valid(&board_rotate(board))
}

fn diagonals(board: &Board) -> Vec<Vec<Field>> {
    let mut diagonals = Vec::new();
    for X in 0..SIZE {
        // Diagonals in upper left
        let mut x = X;
        let mut y = 0;
        let mut diagonal = Vec::new();
        while y <= X {
            diagonal.push(board[x][y].clone());
            if x > 0 {
                x = x - 1;
            }
            y = y + 1;
        }
        diagonals.push(diagonal);

        // Diagonals in lower right
        let mut x = SIZE - 1;
        let mut y = SIZE - X - 1;
        let mut diagonal = Vec::new();
        while y <= SIZE - 1 {
            diagonal.push(board[x][y].clone());
            if x > 0 {
                x = x - 1;
            }
            y = y + 1;
        }
        diagonals.push(diagonal);

        // Diagonals in upper right
        let mut x = SIZE - X - 1;
        let mut y = 0;
        let mut diagonal = Vec::new();
        while x <= SIZE - 1 {
            diagonal.push(board[x][y].clone());
            x = x + 1;
            y = y + 1;
        }
        diagonals.push(diagonal);

        // Diagonals in lower left
        let mut x = 0;
        let mut y = SIZE - X - 1;
        let mut diagonal = Vec::new();
        while y <= SIZE - 1 {
            diagonal.push(board[x][y].clone());
            x = x + 1;
            y = y + 1;
        }
        diagonals.push(diagonal);
        
    }
    diagonals
}

fn diagonal_valid(v: &Vec<Field>) -> bool {
    v.iter().fold(0, |acc, e|
                  if e == &Field::Set {
                      acc + 1
                  } else {
                      acc
                  }) <= 1
}

fn diagonals_valid(v: &Vec<Vec<Field>>) -> bool {
    v.iter().all(|e| diagonal_valid(e))
}

// fn diagonals_test() {
//     use Field::*;
//     let row_1 = [Set, Set, Set];
//     let row_2 = [Empty, Empty, Empty];
//     let row_3 = [Blocked, Blocked, Blocked];
//     let board = [row_1, row_2, row_3];
//     let ds = diagonals(&board);
//     for d in ds {
//         println!("{:?}", d);
//     }

//     println!("{:?}", board[1][2]);
// }
        
fn board_valid(board: &Board) -> bool {
    rows_valid(board) &&
        columns_valid(board) &&
        diagonals_valid(&diagonals(board))
}

fn first_empty(board: &Board) -> (usize, usize) {
    for (y,row) in board.iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            if e == &Field::Empty {
                return (y,x);
            }
        }
    }
    (SIZE,SIZE)
}

fn solve_board(board: &mut Board) -> bool {
    let (x,y) = first_empty(board);
    if x == SIZE || SIZE == y {
        return board_valid(board);
    }
    board[x][y] = Field::Set;
    if solve_board(board) {
        return true;
    }
    board[x][y] = Field::Blocked;
    if solve_board(board) {
        return true;
    }
    board[x][y] = Field::Empty;
    return false;
          
}

fn print_board(board: &Board) {
    for row in board {
        for e in row {
            match e {
                Field::Set => print!("X"),
                Field::Empty => print!("O"),
                Field::Blocked => print!("B"),
            }
        }
        println!("");
    }
}


fn main() {
    let row = [Field::Empty; SIZE];
    let mut board = [row; SIZE];
    println!("{:?}",solve_board(&mut board));
    print_board(&board);
}
