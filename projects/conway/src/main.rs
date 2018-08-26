mod board;
use board::Board;

use std::{thread, time};

fn main() {
    let v_1 = vec![false, false, false, false, false];
    let v_2 = vec![false, false, true, false, false];
    let v_3 = vec![false, false, true, false, false];
    let v_4 = vec![false, false, true, false, false];
    let v_5 = vec![false, false, false, false, false];

    let f = vec![v_1, v_2, v_3, v_4, v_5];
    let mut board = Board::new(f);

    loop {
        println!("{}", board.show());
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
        board = board.step();
        
    }
    
}
