mod player;
use player::*;

use std::str;
use std::thread;
use std::net::UdpSocket;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;
use std::thread::Thread;

fn main() {

    let re_position = Regex::new(r"([0-9]+?)([x])([0-9\.]+?)([y])([0-9\.]+?)([z])([0-9\.]+)").unwrap();

    let socket = match UdpSocket::bind("0.0.0.0:5514") {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
    };

    let mut players: HashMap<String, Player> = HashMap::new();

    let mut buf = [0; 2048];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let data = str::from_utf8(&buf).unwrap_or("");
                match &data[..1] {
                    "p" => {
                        println!("Position Update");
                        let cap = re_position.captures(data).unwrap();
                        let discord = cap[1].to_string();
                        let x = &cap[3];
                        let y = &cap[5];
                        let z = &cap[7];
                        if !players.contains_key(&discord) {
                            println!("New User {0}",discord);
                            players.insert(discord.clone(), Player::new(discord.clone().parse::<i64>().unwrap()));
                        }
                        match players.get_mut(&discord) {
                            Some(player) => {
                                player.set_position([
                                    x.parse::<f32>().unwrap_or(0.0),
                                    y.parse::<f32>().unwrap_or(0.0),
                                    z.parse::<f32>().unwrap_or(0.0)
                                ]);
                                let pos = player.get_position();
                                println!("Pos: {}, {}, {}",pos.x,pos.y,pos.z);
                                let vel = player.get_velocity();
                                println!("Vel: {}, {}, {}",vel.x,vel.y,vel.z);
                            },
                            None => {
                                println!("Couldn't find player!");
                            }
                        }
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
