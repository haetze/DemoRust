mod data;

use std::net::TcpListener;
use std::net;
use std::io;
use std::io::BufWriter;
use std::io::BufReader;
use std::thread::spawn;

fn main(){
    expect_connection_loop("127.0.0.1:17007").expect("error: ");

}

fn expect_connection_loop(add: &str) -> io::Result<()> {
    let listener = TcpListener::bind(add)?;
    loop {
        let (mut stream, adr) = listener.accept()?;
        spawn(move || handle(stream));
    }
}


fn handle(stream: net::TcpStream){
    let mut player;
    let mut writer = stream.try_clone()?;
    let mut reader = stream.try_clone()?;
    let mut writer = BufWriter::new(writer);
    let mut reader = BufWriter::new(writer);

    loop {
        let mut buffer = Vec::new();
        io
}


