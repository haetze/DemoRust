extern crate tokio;
extern crate futures;
extern crate console;

use futures::Stream;

use console::Term;

mod stdin_stream;

use stdin_stream::stdin;
use futures::IntoFuture;


fn main() {
    let mut count = 0;
    let term = Term::stdout();
    let f = stdin()
        .and_then(move |string| {
            count += 1;
            let s = format!("{}: {:?}", count,  string);
            term.write_line(&s).unwrap();
            term.clear_line().unwrap();
            Ok(())
        })
        .for_each(|_| Ok(()))
        .into_future();

    tokio::run(f);
}



