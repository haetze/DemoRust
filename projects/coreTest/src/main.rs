extern crate tokio_core;
extern crate futures_await as futures;
extern crate num_cpus;
extern crate tokio_io;


use std::env;
use std::net::{self, SocketAddr};
use std::thread;
use std::io::{Error, BufReader, ErrorKind};


use futures::Future;
use futures::future::{ok, err, loop_fn, Loop};
use futures::stream::Stream;
use futures::sync::mpsc;
use tokio_io::AsyncRead;
use tokio_io::io;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;


fn main(){
    let addr = env::args().nth(1).unwrap_or("10.0.0.12:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let num_threads = env::args().nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(num_cpus::get());


    let listener = net::TcpListener::bind(&addr)
        .expect("failed to bind");
    println!("Listening on: {}", addr);
    
    let mut channels = Vec::new();
    for _ in 0..num_threads {
        let (tx, rx) = mpsc::unbounded();
        channels.push(tx);
        thread::spawn(|| worker(rx));
    }

    let mut next = 0;
    for socket in listener.incoming() {
        let socket = socket.expect("failed to accept");
        channels[next]
            .unbounded_send(socket)
            .expect("worker thread died");
        next = (next + 1) % channels.len();
    }

}


fn worker(rx: mpsc::UnboundedReceiver<net::TcpStream>) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let done = rx.for_each(move |socket| {

        let socket = TcpStream::from_stream(socket, &handle)
            .expect("failed to associate TCP stream");
        
        let (reader, writer) = socket.split();
        let reader = BufReader::new(reader);

        let f = loop_fn((reader, writer), |(r,w)| {
            io::read_until(r, b'\n', Vec::new())
                .and_then(|(r,b)| {
                    if b.len() > 0 {
                        match io::write_all(w, b).wait() {
                            Ok((w,b)) => {
                                let s = String::from_utf8(b).unwrap();
                                let s = s.trim_matches('\n');
                                println!("Read {} from reader, writing to writer!", s); 
                                ok((r,w))
                            },
                            _         => err(Error::new(ErrorKind::Other, "Error")),
                        }
                    }else{
                        err(Error::new(ErrorKind::Other, "Error"))
                    }
                })
                .then(|result| {
                    match result {
                        Ok((r,w))         => Ok(Loop::Continue((r,w))),
                        Err(e)        => Ok(Loop::Break("End of Loop")),
                    }
                })
                
        }).and_then(|_| Ok(()));

       
        handle.spawn(f);

        Ok(())
    });
    core.run(done).unwrap();


}
