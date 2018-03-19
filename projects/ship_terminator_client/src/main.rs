mod data;
use data::*;

use std::str;
use std::net::UdpSocket;
use std::net::IpAddr;
use std::io::{self};

fn main() {
    let ip = find_partner().unwrap();
    let mut board = Board::new();

    setup_board(&mut board);

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

fn print_ship_in_message(type_of_ship: String, size: u32) -> Option<Ship>{
    let stdin = io::stdin();
    println!("Create new ship of type {}", type_of_ship);

    let mut handle = String::new();
    println!("Input x coord.");
    stdin.read_line(&mut handle).unwrap();
    let x: u32 = handle.trim().parse().unwrap();

    let mut handle = String::new();
    println!("Input y coord.");
    stdin.read_line(&mut handle).unwrap();
    let y: u32 = handle.trim().parse().unwrap();

    let start = Point {
        x: x,
        y: y,
    };
    
    let mut handle = String::new();
    println!("Enter direction (r=1,l=2,u=3,d=4))");
    stdin.read_line(&mut handle).unwrap();
    let dir: u32 = handle.trim().parse().unwrap();

    let dir = match dir {
        1 => Direction::Right,
        2 => Direction::Left,
        3 => Direction::Up,
        4 => Direction::Down,
        _ => {
            println!("You idiot that direction does not exist");
            panic!("Idiot playing");
        },
    };
    
    Ship::new(start, dir, size)
}

    

fn setup_board(board: &mut Board){

    //Loop for small/3
    let mut small_count = 0;
    loop {
        if small_count >= 3 {
            break;
        }
        match print_ship_in_message("small".to_string(), 3) {
            None => {
                continue;
            },
            Some(ship) => {
                if board.add(ship) {
                    small_count = small_count + 1;
                }else{
                    continue;
                }
            },
        };
    }
            
        


    //Loop for Medium/2
    let mut medium_count = 0;
    loop {
        if medium_count >= 2 {
            break;
        }
        match print_ship_in_message("medium".to_string(), 2) {
            None => {
                continue;
            },
            Some(ship) => {
                if board.add(ship) {
                    medium_count = medium_count + 1;
                }else{
                    continue;
                }
            },
        };
    }
    


    //Loop for Big/1
    let mut big_count = 0;
    loop {
        if big_count >= 1 {
            break;
        }
        match print_ship_in_message("big".to_string(), 1) {
            None => {
                continue;
            },
            Some(ship) => {
                if board.add(ship) {
                    big_count = big_count + 1;
                }else{
                    continue;
                }
            },
        };
    }
    

}
