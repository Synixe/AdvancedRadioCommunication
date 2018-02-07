mod handler;
mod player;
mod server;
mod voice;

extern crate regex;
use regex::Regex;

extern crate serenity;
use serenity::client::Client;
use serenity::prelude::Mutex;

use std::collections::HashMap;
use std::env;
use std::net::UdpSocket;
use std::str;
use std::sync::Arc;
use std::thread;

use handler::Handler;
use player::*;
use voice::VoiceManager;

fn main() {

    let players: Arc<Mutex<HashMap<String, Player>>> = Arc::new(Mutex::new(HashMap::new()));
    let socket_players = players.clone();

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    {
        let mut data = client.data.lock();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<PlayerManager>(Arc::clone(&players.clone()));
    }

    thread::spawn(move || {
        let re_position = Regex::new(r"([0-9]+?)([x])([\-0-9\.]+?)([y])([\-0-9\.]+?)([z])([\-0-9\.]+)").unwrap();
        let socket = match UdpSocket::bind("0.0.0.0:5514") {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e)
        };
        let mut buf = [0; 2048];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((_amt, _src)) => {
                    let data = str::from_utf8(&buf).unwrap_or("");
                    match &data[..1] {
                        "p" => {
                            match re_position.captures(&data) {
                                Some(cap) => {
                                    let discord = cap[1].to_string();
                                    let x = &cap[3];
                                    let y = &cap[5];
                                    let z = &cap[7];
                                    if !socket_players.lock().contains_key(&discord) {
                                        println!("New User {0}",discord);
                                        socket_players.lock().insert(discord.clone(), Player::new());
                                    }
                                    match socket_players.lock().get_mut(&discord) {
                                        Some(player) => {
                                            player.set_position([
                                                x.parse::<f32>().unwrap_or(0.0),
                                                y.parse::<f32>().unwrap_or(0.0),
                                                z.parse::<f32>().unwrap_or(0.0)
                                            ]);
                                        },
                                        None => {
                                            println!("Couldn't find player!");
                                        }
                                    }
                                },
                                None => {
                                    println!("Failed regex on {}", data);
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
                    println!("couldn't receive a datagram: {}", e);
                }
            }
        }
    });

    client.start();
}
