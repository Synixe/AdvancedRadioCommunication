extern crate openal;

use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::prelude::Mutex;
use serenity::voice::AudioReceiver;

extern crate typemap;
use self::typemap::Key;

use std::collections::HashMap;
use std::sync::Arc;
use std::f32;

use player::Vector3;
use player::Player;

pub struct VoiceManager;
impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct Receiver<'a> {
    listener: openal::Listener<'a>,
    ids: HashMap<u32, String>,
    players: Arc<Mutex<HashMap<String, Player>>>,
    sources: HashMap<u32, openal::source::Stream<'a>>,
    buffer: [i16; 1920]
}
impl <'a>Receiver<'a> {
    pub fn new(players: Arc<Mutex<HashMap<String, Player>>>) -> Self {
        let listener = openal::listener::default(&Default::default()).unwrap();
        let mut buf: [i16; 1920] = [0; 1920];
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
                                process_ild(data, &position, &orientation, source, &mut self.buffer, stereo);
                                //source.push(2, data, 48000);
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
            self.sources.insert(ssrc, self.listener.source().unwrap().stream());
        }
    }
}

fn process_ild(data: &[i16], pos: &Vector3, orientation: &Vector3, source: &mut openal::source::Stream, buf: &mut [i16; 1920], stereo: bool) {
    if stereo {
        //let angle = orientation.x * f32::consts::PI / 180.0;
        let mut dir = pos.y.atan2(pos.x);
        while dir > f32::consts::PI {
            dir = dir - (2.0 * f32::consts::PI);
        }
        let gain_left   = -0.375 * dir.cos() + 0.625;
        let gain_right   = 0.375 * dir.cos() + 0.625;
        for i in 0..(data.len()) {
            if i % 2 == 0 {
                buf[i] = (data[i] as f32 * gain_left) as i16;
            } else {
                buf[i] = (data[i] as f32 * gain_right) as i16;
            }
        }
        source.push(2, buf, 48000);
    }
}
