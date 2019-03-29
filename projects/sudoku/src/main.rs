#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)] 
enum Field {
    X,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Square_Index {
    Left_Up,
    Left_Middle,
    Left_Down,
    Middle_Up,
    Middle_Middle,
    Middle_Down,
    Right_Up,
    Right_Middle,
    Right_Down,
}

impl Field {
    fn from_i32(x: i32) -> Field {
        use Field::*;
        match x {
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            _ => X,
        }
    }
}

fn contains_once<A: PartialEq>(arr: &[A], e: &A) -> bool {
    let mut counter = 0;
    for a in arr {
        if a == e {
            counter = counter + 1;
        }
    }
    return counter == 1;
}

type Row = [Field; 9];
type Board = [Row; 9];

fn completly_filled(board: &Board) -> bool {
    for row in board {
        for field in row {
            if field == &Field::X {
                return false;
            }
        }
    }
    return true;
}

fn rows_ok(board: &Board) -> bool {
    for row in board {
        if !row_ok(row) {
            return false;
        }
    }
    return true;
}

fn row_ok(row: &Row) -> bool {
    for i in 1..10 {
        let c = Field::from_i32(i);
        if !contains_once(row, &c) {
            return false;
        }
    }
    return true;
}

fn columns_ok(board: &Board) -> bool {
    for x in 0..9 {
        let mut column = [Field::X; 9];
        for y in 0..9 {
            column[y] = board[x][y].clone();
        }
        if !row_ok(&column) {
                return false;
        }
    }
    return true;
}

fn square_of_index(board: &Board, index: Square_Index) -> Row {
    let mut column = [Field::X; 9];
    /*
    |------------------------+
    |  L  |  M  |  R  | |    |
    |0|1|2|3|4|5|6|7|8| |    |
    |-----|-----|-----|------|
    | | | | | | | | | |0|    |
    | LU  | MU  | RU  |1| U  |
    | | | | | | | | | |2|    |
    |-----|-----|-----|------|
    | | | | | | | | | |3|    |
    | LM  | MM  | RM  |4| M  |
    | | | | | | | | | |5|    |
    |-----|-----|-----|------|
    | | | | | | | | | |6|    |
    | LD  | MD  | RD  |7| D  |
    | | | | | | | | | |8|    |
    |-----|-----|-----|------+
     */
    let (x, y) = match index {
        Square_Index::Left_Up => (1,1),
        Square_Index::Left_Middle => (1,4),
        Square_Index::Left_Down => (1,7),
        Square_Index::Middle_Up => (4,1),
        Square_Index::Middle_Middle => (4,4),
        Square_Index::Middle_Down => (4,7),
        Square_Index::Right_Up => (7,1),
        Square_Index::Right_Middle => (7,4),
        Square_Index::Right_Down => (7,7),
    };
    column[0] = board[x][y-1].clone();
    column[1] = board[x-1][y-1].clone();
    column[2] = board[x+1][y-1].clone();
    
    column[3] = board[x][y].clone();
    column[4] = board[x-1][y].clone();
    column[5] = board[x+1][y].clone();

    column[6] = board[x][y+1].clone();
    column[7] = board[x-1][y+1].clone();
    column[8] = board[x+1][y+1].clone();

    return column;

}
                   
fn board_ok(board: &Board) -> bool {
    if !completly_filled(board) {
        return false;
    }
    if !rows_ok(board){
        return false;
    }
    if !columns_ok(board) {
        return false;
    }
    let indexs = [Square_Index::Left_Up,
                  Square_Index::Left_Middle,
                  Square_Index::Left_Down,
                  Square_Index::Right_Up,
                  Square_Index::Right_Middle,
                  Square_Index::Right_Down,
                  Square_Index::Middle_Up,
                  Square_Index::Middle_Middle,
                  Square_Index::Middle_Down];
    for index in indexs.iter() {
        let sq = square_of_index(board, index.clone());
        if !row_ok(&sq) {
            return false;
        }
    }

    return true;
}

#[test]
fn sudoku_test() {
    let board = [[Field::Seven,Field::Four,Field::Three,Field::Nine,Field::Five,Field::One,Field::Six,Field::Eight,Field::Two],
                 [Field::One,Field::Six,Field::Two,Field::Four,Field::Eight,Field::Seven,Field::Three,Field::Nine,Field::Five],
                 [Field::Nine,Field::Five,Field::Eight,Field::Six,Field::Three,Field::Two,Field::Seven,Field::One,Field::Four],
                 [Field::Two,Field::One,Field::Nine,Field::Eight,Field::Seven,Field::Three,Field::Five,Field::Four,Field::Six],
                 [Field::Three,Field::Seven,Field::Four,Field::Five,Field::Six,Field::Nine,Field::One,Field::Two,Field::Eight],
                 [Field::Five,Field::Eight,Field::Six,Field::One,Field::Two,Field::Four,Field::Nine,Field::Seven,Field::Three],
                 [Field::Four,Field::Nine,Field::Five,Field::Two,Field::One,Field::Six,Field::Eight,Field::Three,Field::Seven],
                 [Field::Eight,Field::Two,Field::Seven,Field::Three,Field::Nine,Field::Five,Field::Four,Field::Six,Field::One],
                 [Field::Six,Field::Three,Field::One,Field::Seven,Field::Four,Field::Eight,Field::Two,Field::Five,Field::Nine]];
    assert!(board_ok(&board));
}

#[test]
fn sudoku_rows_test() {
    let board = [[Field::Seven,Field::Four,Field::Three,Field::Nine,Field::Five,Field::One,Field::Six,Field::Eight,Field::Two],
                 [Field::One,Field::Six,Field::Two,Field::Four,Field::Eight,Field::Seven,Field::Three,Field::Nine,Field::Five],
                 [Field::Nine,Field::Five,Field::Eight,Field::Six,Field::Three,Field::Two,Field::Seven,Field::One,Field::Four],
                 [Field::Two,Field::One,Field::Nine,Field::Eight,Field::Seven,Field::Three,Field::Five,Field::Four,Field::Six],
                 [Field::Three,Field::Seven,Field::Four,Field::Five,Field::Six,Field::Nine,Field::One,Field::Two,Field::Eight],
                 [Field::Five,Field::Eight,Field::Six,Field::One,Field::Two,Field::Four,Field::Nine,Field::Seven,Field::Three],
                 [Field::Four,Field::Nine,Field::Five,Field::Two,Field::One,Field::Six,Field::Eight,Field::Three,Field::Seven],
                 [Field::Eight,Field::Two,Field::Seven,Field::Three,Field::Nine,Field::Five,Field::Four,Field::Six,Field::One],
                 [Field::Six,Field::Three,Field::One,Field::Seven,Field::Four,Field::Eight,Field::Two,Field::Five,Field::Nine]];
    assert!(rows_ok(&board));
}

#[test]
fn sudoku_columns_test() {
    let board = [[Field::Seven,Field::Four,Field::Three,Field::Nine,Field::Five,Field::One,Field::Six,Field::Eight,Field::Two],
                 [Field::One,Field::Six,Field::Two,Field::Four,Field::Eight,Field::Seven,Field::Three,Field::Nine,Field::Five],
                 [Field::Nine,Field::Five,Field::Eight,Field::Six,Field::Three,Field::Two,Field::Seven,Field::One,Field::Four],
                 [Field::Two,Field::One,Field::Nine,Field::Eight,Field::Seven,Field::Three,Field::Five,Field::Four,Field::Six],
                 [Field::Three,Field::Seven,Field::Four,Field::Five,Field::Six,Field::Nine,Field::One,Field::Two,Field::Eight],
                 [Field::Five,Field::Eight,Field::Six,Field::One,Field::Two,Field::Four,Field::Nine,Field::Seven,Field::Three],
                 [Field::Four,Field::Nine,Field::Five,Field::Two,Field::One,Field::Six,Field::Eight,Field::Three,Field::Seven],
                 [Field::Eight,Field::Two,Field::Seven,Field::Three,Field::Nine,Field::Five,Field::Four,Field::Six,Field::One],
                 [Field::Six,Field::Three,Field::One,Field::Seven,Field::Four,Field::Eight,Field::Two,Field::Five,Field::Nine]];
    assert!(columns_ok(&board));
}

fn find_first_empty(board: &Board) -> (usize,usize) {
    for (x,row) in board.iter().enumerate() {
        for (y,field) in row.iter().enumerate() {
            if field == &Field::X {
                return (x,y);
            }
        }
    }
    return (10,10);
}

fn solve(board: &mut Board) -> bool{
    let (x,y) = find_first_empty(board);
    if (10,10) == (x,y) {
        return board_ok(board);
    }
    board[x][y] = Field::One;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::Two;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::Three;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::Four;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::Five;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::Six;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::Seven;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::Eight;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::Nine;
    if solve(board) {
        return true;
    }
    board[x][y] = Field::X;
    return false;
    
}

fn print_sudoku(board: &Board) {
    use Field::*;
    
    for row in board {
        for field in row {
            let x = match field {
                One => 1,
                Two=> 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
                X => 0,
            };
            if x == 0 {
                print!("{} ", 'X');
            } else {
                print!("{} ", x);
            }
        }
        println!("");
    }
}
                


fn main() {
    let board_s = [[Field::Seven,Field::Four,Field::Three,Field::Nine,Field::Five,Field::One,Field::Six,Field::Eight,Field::Two],
                   [Field::One,Field::Six,Field::Two,Field::Four,Field::Eight,Field::Seven,Field::Three,Field::Nine,Field::Five],
                   [Field::Nine,Field::Five,Field::Eight,Field::Six,Field::Three,Field::Two,Field::Seven,Field::One,Field::Four],
                   [Field::Two,Field::One,Field::Nine,Field::Eight,Field::Seven,Field::Three,Field::Five,Field::Four,Field::Six],
                   [Field::Three,Field::Seven,Field::Four,Field::Five,Field::Six,Field::Nine,Field::One,Field::Two,Field::Eight],
                   [Field::Five,Field::Eight,Field::Six,Field::One,Field::Two,Field::Four,Field::Nine,Field::Seven,Field::Three],
                   [Field::Four,Field::Nine,Field::Five,Field::Two,Field::One,Field::Six,Field::Eight,Field::Three,Field::Seven],
                   [Field::Eight,Field::Two,Field::Seven,Field::Three,Field::Nine,Field::Five,Field::Four,Field::Six,Field::One],
                   [Field::Six,Field::Three,Field::One,Field::Seven,Field::Four,Field::Eight,Field::Two,Field::Five,Field::Nine]];

    let mut board = [[Field::X,Field::Four,Field::Three,Field::Nine,Field::Five,Field::One,Field::Six,Field::Eight,Field::Two],
                     [Field::One,Field::X,Field::Two,Field::Four,Field::X,Field::Seven,Field::Three,Field::Nine,Field::Five],
                     [Field::Nine,Field::X,Field::Eight,Field::Six,Field::Three,Field::Two,Field::Seven,Field::One,Field::Four],
                     [Field::Two,Field::One,Field::X,Field::Eight,Field::Seven,Field::Three,Field::Five,Field::Four,Field::Six],
                     [Field::Three,Field::Seven,Field::Four,Field::Five,Field::Six,Field::Nine,Field::One,Field::Two,Field::Eight],
                     [Field::Five,Field::Eight,Field::Six,Field::One,Field::Two,Field::Four,Field::Nine,Field::Seven,Field::Three],
                     [Field::Four,Field::Nine,Field::Five,Field::Two,Field::One,Field::Six,Field::Eight,Field::Three,Field::Seven],
                     [Field::Eight,Field::Two,Field::Seven,Field::Three,Field::Nine,Field::Five,Field::Four,Field::Six,Field::One],
                     [Field::Six,Field::Three,Field::One,Field::Seven,Field::Four,Field::Eight,Field::Two,Field::Five,Field::Nine]];

    solve(&mut board);
    print_sudoku(&board_s);
    println!("_________________________");
    print_sudoku(&board);
}
