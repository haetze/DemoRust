extern crate tokio;
extern crate tokio_io;
extern crate futures;
extern crate ncurses;


use futures::{Future, Stream};

use ncurses::*;

mod stdin_stream;

use stdin_stream::stdin;


fn main() {
    initscr();
    noecho();
    let mut count = 0;
    
    stdin()
        .and_then(|string| {
            count += 1;
            let s = format!("{}: {:?}\n", count,  string);
            printw(&s);
            Ok(())
        })
        .for_each(|_| Ok(()))
        .wait()
        .unwrap();
}



