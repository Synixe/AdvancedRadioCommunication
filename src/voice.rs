extern crate openal;

use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::prelude::Mutex;
use serenity::voice::AudioReceiver;

extern crate typemap;
use self::typemap::Key;

use std::cmp;
use std::collections::HashMap;
use std::f32;
use std::sync::Arc;

use player::Player;
use player::Vector3;

pub struct VoiceManager;
impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct Receiver<'a> {
    listener: openal::Listener<'a>,
    ids: HashMap<u32, String>,
    players: Arc<Mutex<HashMap<String, Player>>>,
    sources: HashMap<u32, openal::source::Stream<'a>>,
    buffer: [i16; 960]
}
impl <'a>Receiver<'a> {
    pub fn new(players: Arc<Mutex<HashMap<String, Player>>>) -> Self {
        let mut listener = openal::listener::default(&Default::default()).unwrap();
        listener.set_position(&openal::Position(openal::Vector{x: 0.0, y: 0.0, z: 0.0}));
        let mut buf: [i16; 960] = [0; 960];
        Self {
            listener: listener,
            ids: HashMap::new(),
            players: players,
            sources: HashMap::new(),
            buffer: buf
        }
    }
}
impl <'a>AudioReceiver for Receiver<'a> {
    fn speaking_update(&mut self, ssrc: u32, user_id: u64, _speaking: bool) {
        self.ids.entry(ssrc).or_insert(user_id.to_string());
    }
    fn voice_packet(&mut self, ssrc: u32, sequence: u16, _timestamp: u32, stereo: bool, data: &[i16]) {
        let mut create = false;

        match self.sources.get_mut(&ssrc) {
            Some(source) => {
                match self.ids.get(&ssrc) {
                    Some(user_id) => {
                        let players = self.players.lock();
                        match players.get(user_id) {
                            Some(player) => {
                                let position = player.get_position();
                                let orientation = player.get_orientation();
                                drop(player);
                                process(data, &position, &orientation, source, &mut self.buffer, stereo);
                                if source.state() != openal::source::State::Playing {
                                    source.play();
                                }
                            },
                            None => {}
                        }
                    },
                    None => {
                        println!("User {} not in id list", ssrc);
                    }
                }
            },
            None => {
                create = true;
            }
        }
        if create {
            let mut stream = self.listener.source().unwrap().stream();
            stream.enable_relative();
            self.sources.insert(ssrc, stream);
        }
    }
}

fn process(data: &[i16], pos: &Vector3, orientation: &Vector3, source: &mut openal::source::Stream, buf: &mut [i16; 960], stereo: bool) {
    let mut i = 0;
    while i < data.len() / 2 {
        buf[i] = (data[i * 2] / 2) + (data[(i * 2) + 1 ] / 2);
        i += 1;
    }
    source.set_position(&openal::Position(openal::Vector{x: pos.x, y: pos.y, z: pos.z}));
    source.push(1, buf, 48000);
}
