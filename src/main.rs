mod player;

use std::str;
use std::thread;
use std::net::UdpSocket;

extern crate regex;
use regex::Regex;

fn main() {

    let re_position = Regex::new(r"([0-9]+?)([x])([0-9\.]+?)([y])([0-9\.]+?)([z])([0-9\.]+)").unwrap();

    let socket = match UdpSocket::bind("0.0.0.0:5514") {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
    };

    let mut players: Vec<player::Player> = Vec::new();

    let mut buf = [0; 2048];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let data = str::from_utf8(&buf).unwrap_or("");
                println!("amt: {}", amt);
                println!("src: {}", src);
                println!("{}", data);
                match &data[..1] {
                    "p" => {
                        println!("Position Update");
                        let cap = re_position.captures(data).unwrap();
                        let discord = &cap[1];
                        let x = &cap[3];
                        let y = &cap[5];
                        let z = &cap[7];
                        println!("User {0} has moved to x = {1}, y = {2}, z = {3}",discord,x,y,z);
                    },
                    "o" => {
                        println!("Orientation Update");
                    },
                    _ => {
                        println!("Unknown");
                    }
                }
            },
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}
