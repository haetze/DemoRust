extern crate rayon;

mod board;
use board::Board;

mod read;
use read::read_from_file;

use std::{thread, time};
use std::env;

fn main() {
    let v_1 = vec![false, false, false, false, false];
    let v_2 = vec![false, false, true, false, false];
    let v_3 = vec![false, false, true, false, false];
    let v_4 = vec![false, false, true, false, false];
    let v_5 = vec![false, false, false, false, false];

    let f = match env::args().skip(1).next() {
        None => vec![v_1, v_2, v_3, v_4, v_5],
        Some(s) => {
            read_from_file(s)
        },
    };
    let mut board = Board::new(f);

    loop {
        println!("{}", board.show());
        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        board = board.step();
        
    }
    
}
