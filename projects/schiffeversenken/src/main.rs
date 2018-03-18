mod data;

extern crate futures_await as futures;
#[macro_use]
extern crate tokio_core;

use std::{env, io};
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
    playerlist: data::Playerlist,
}


impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            if let Some((size, peer)) = self.to_send {
                let words: Vec<_>= self.buf[..size].split(|e| *e == b' ')
                    .map(|b| String::from_utf8(b.to_vec()).unwrap())
                    .collect();
                println!("{:?}", words);
                match &words[0].trim() as &str {
                    "available" => {
                        if self.playerlist.exists(&words[1]){
                            try_nb!(self.socket.send_to(b"226\n" , &peer))
                        }else{
                            let ip_string = format!("{}", peer.ip());
                            self.playerlist.add(&words[1].trim().to_string(), &ip_string);
                            try_nb!(self.socket.send_to(b"200" , &peer))
                        }
                    },
                    "request_list" => {
                        let mut s = format!("201 {}", self.playerlist.list.len());
                        try_nb!(self.socket.send_to(s.as_bytes() , &peer));
                        for player in &self.playerlist.list {
                            let s = format!("{}", player.name);
                            try_nb!(self.socket.send_to(s.as_bytes() , &peer));
                        }
                        try_nb!(self.socket.send_to(b"\n" , &peer))
                    },
                    "request_player" => {
                        match self.playerlist.find(&words[1].trim().to_string()) {
                            None =>  try_nb!(self.socket.send_to(b"404" , &peer)),
                            Some(player) => {
                                let s = format!("202 {}", player.ip);
                                println!("{}", s);
                                try_nb!(self.socket.send_to(s.as_bytes() , &peer))
                            },
                        }

                    },
                    _                => {
                        try_nb!(self.socket.send_to(b"no match" , &peer))
                    },
                };
                
                self.to_send = None;
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            self.to_send = Some(try_nb!(self.socket.recv_from(&mut self.buf)));
        }
    }
}

fn main() {
    println!("Hello, world!");


    let addr = env::args().nth(1).unwrap_or("10.0.0.12:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Create the event loop that will drive this server, and also bind the
    // socket we'll be listening to.
    let mut l = Core::new().unwrap();
    let handle = l.handle();
    let socket = UdpSocket::bind(&addr, &handle).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    // Next we'll create a future to spawn (the one we defined above) and then
    // we'll run the event loop by running the future.
    l.run(Server {
        socket: socket,
        buf: vec![0; 1024],
        to_send: None,
        playerlist: data::Playerlist::new(),
    }).unwrap();
}
