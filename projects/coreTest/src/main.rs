extern crate tokio_core;
extern crate futures_await as futures;
extern crate num_cpus;
extern crate tokio_io;


use std::env;
use std::net::{self, SocketAddr};
use std::thread;


use futures::Future;
use futures::stream::Stream;
use futures::sync::mpsc;
use tokio_io::AsyncRead;
use tokio_io::io::copy;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

fn main(){
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
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
        let addr = socket.peer_addr()
            .expect("failed to get remote address");


        let (reader, writer) = socket.split();
        let amt = copy(reader, writer);
        let msg = amt.then(move |result| {
            match result {
                Ok((amt, _, _)) =>
                    println!("wrote {} bytes to {}", amt, addr),
                Err(e) => println!("error on {}: {}", addr, e),
            }

            Ok(())
        });
        handle.spawn(msg);

        Ok(())
    });
    core.run(done).unwrap();


}
