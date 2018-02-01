use serenity::prelude::Mutex;

use std::collections::HashMap;
use std::net::UdpSocket;
use std::str;
use std::sync::Arc;

use player::Player;

pub struct Server {
    socket: UdpSocket,
    players: Arc<Mutex<HashMap<String, Player>>>
}
impl Server {
    pub fn new(players: Arc<Mutex<HashMap<String, Player>>>) -> Server {
        let socket = match UdpSocket::bind("0.0.0.0:3615") {
            Ok(s) => s,
            Err(e) => panic!("Unable to bind socket: {}", e)
        };
        println!("Socket started on 3615!");
        Server {
            socket: socket,
            players: players
        }
    }

    pub fn run(&self) {
        let mut buf = [0; 2048];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((_amt, _src)) => {
                    let data = str::from_utf8(&buf).unwrap_or("e");
                    println!("Data recv: {}", data);
                },
                Err(e) => {
                    println!("Unable to receive datagram: {}", e);
                }
            }
        }
    }
}
