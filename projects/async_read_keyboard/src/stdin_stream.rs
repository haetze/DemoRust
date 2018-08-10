extern crate futures;
extern crate console;

use futures::{Future, Sink, Stream};
use futures::sync::mpsc::channel;

use self::console::Term;
use self::console::Key::Char;

use std::thread;

pub fn stdin() -> impl Stream<Item = char, Error = ()> {
    let (mut tx, rx) = channel(1);
    thread::spawn(move || {
        let term = Term::stdout();
        loop {
            let c = match term.read_key().unwrap() {
                Char(c) => c,
                _       => ' ',
            };
            match tx.send(c).wait() {
                Ok(s) => tx = s,
                Err(_) => break,
            }
        }
    });
    rx
}
