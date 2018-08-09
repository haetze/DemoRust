extern crate futures;
extern crate ncurses;

use self::ncurses::*;

use futures::{Future, Sink, Stream};
use futures::sync::mpsc::channel;

use std::thread;

pub fn stdin() -> impl Stream<Item = char, Error = ()> {
    let (mut tx, rx) = channel(1);
    thread::spawn(move || {
        loop {
            let c = getch() as u8 as char; 
            match tx.send(c).wait() {
                Ok(s) => tx = s,
                Err(_) => break,
            }
        }
    });
    rx
}
