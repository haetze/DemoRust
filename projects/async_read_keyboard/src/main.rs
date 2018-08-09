extern crate tokio;
extern crate futures;
extern crate ncurses;


use futures::Stream;

use ncurses::*;

mod stdin_stream;

use stdin_stream::stdin;
use futures::IntoFuture;


fn main() {
    initscr();
    noecho();
    let mut count = 0;
    
    let f = stdin()
        .and_then(move |string| {
            count += 1;
            let s = format!("{}: {:?}\n", count,  string);
            printw(&s);
            Ok(())
        })
        .for_each(|_| Ok(()))
        .into_future();

    tokio::run(f);
}



