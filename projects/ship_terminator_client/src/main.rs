mod data;

use std::str;
use std::net::UdpSocket;
use std::net::IpAddr;
use std::io::{self};
use std::io::{Error, ErrorKind};

fn main() {

    let ip = find_partner().unwrap();

}

    
fn find_partner() -> io::Result<IpAddr> {
    println!("Looking for partner to play.\nEnter your handle: ");
    let socket = UdpSocket::bind("10.0.0.12:8888").expect("error in bind");
    socket.connect("10.0.0.12:8080").expect("connect function failed");


    connect_to_server(&socket);
    get_player_list(&socket);
    get_opponent(&socket)

}

fn get_opponent(socket: &UdpSocket) -> std::io::Result<IpAddr>{
    
    println!("Who do you wanna play with? ");
    let mut ip; 

    loop {
        let mut handle = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut handle).unwrap();
        
        let s = format!("request_player {}", handle);
        socket.send(s.trim().as_bytes());

        let mut response = vec![0; 1024];
        let (amt, _) = socket.recv_from(&mut response)?;
        
        let words: Vec<_>= response[..amt]
            .split(|e| *e == b' ')
            .map(|b| String::from_utf8(b.to_vec()).unwrap())
            .collect();

        let code: i32 = words[0].parse().unwrap();


            
        match code{
            202 => {
                ip = words[1].parse().unwrap();
                break;
            },
            _     => {
                println!("Not available, new Player: ");
            },
        };
    }

    Ok(ip)
}
    

fn get_player_list(socket: &UdpSocket){
        let s = "request_list".to_string();
    socket.send(s.as_bytes());

  
    let mut response = vec![0; 128];
    let (amt, _) = socket.recv_from(&mut response).unwrap();
    let words: Vec<_>= response[..amt]
        .split(|e| *e == b' ')
        .map(|b| String::from_utf8(b.to_vec()).unwrap())
        .collect();

    let number_of_player: i32 = words[1].parse().unwrap();

    println!("List of Player available:");
    for _ in 0..number_of_player{
        
        let mut response = vec![0; 128];
        let (amt, _) = socket.recv_from(&mut response).unwrap();
        let words: Vec<_>= response[..amt]
            .split(|e| *e == b' ')
            .map(|b| String::from_utf8(b.to_vec()).unwrap())
            .collect();

        let name = &words[0];
        println!("\t{}", name);
        

    }
}
    
fn connect_to_server(socket: &UdpSocket){    
    loop {
        let mut handle = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut handle).unwrap();
        
        let s = format!("available {}", handle);
        socket.send(s.trim().as_bytes());

        let mut response = vec![0; 10];
        let (amt, _) = socket.recv_from(&mut response).unwrap();

        let response = str::from_utf8(&response[..amt]).unwrap().trim();
        //println!("Received: {}", response);
            
        match response {
            "200" => {
                break;
            },
            _     => {
                println!("Handle is taken, new Handle: ");
            },
        };
    }
}
