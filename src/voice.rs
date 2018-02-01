extern crate alto;
use alto::Source;

use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::prelude::Mutex;
use serenity::voice::AudioReceiver;

extern crate typemap;
use self::typemap::Key;

use std::collections::HashMap;
use std::sync::Arc;

use player::Player;

pub struct VoiceManager;
impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct AltoManager;
impl Key for AltoManager {
    type Value = Arc<Mutex<alto::Device>>;
}

pub struct Receiver {
    audio: AudioSystem
}
impl Receiver {
    pub fn new(players: Arc<Mutex<HashMap<String, Player>>>) -> Self {
        let audio = AudioSystem::new(players);
        Self {
            audio: audio
        }
    }
}
impl AudioReceiver for Receiver {
    fn speaking_update(&mut self, ssrc: u32, user_id: u64, _speaking: bool) {
        self.audio.notice(ssrc, user_id);
    }
    fn voice_packet(&mut self, ssrc: u32, sequence: u16, _timestamp: u32, _stereo: bool, data: &[i16]) {
        self.audio.queue(ssrc, data);
    }
}

struct AudioSystem {
    context: alto::Context,
    device: alto::OutputDevice,
    ids: HashMap<u32, String>,
    players: Arc<Mutex<HashMap<String, Player>>>,
    sources: HashMap<u32, alto::StreamingSource>
}
impl AudioSystem {
    pub fn new(players: Arc<Mutex<HashMap<String, Player>>>) -> AudioSystem {
        let al = alto::Alto::load_default().unwrap();
        let device = al.open(None).unwrap();
        AudioSystem {
            context: device.new_context(None).unwrap(),
            device: device,
            ids: HashMap::new(),
            players: players,
            sources: HashMap::new()
        }
    }

    pub fn notice(&mut self, ssrc: u32, user_id: u64) {
        self.ids.entry(ssrc).or_insert(user_id.to_string());
    }

    pub fn queue(self, ssrc: u32, data: &[i16]) {
        let mut create = false;
        //This line isn't working and is the current hold up
        let buf: alto::Buffer = self.context.new_buffer::<alto::Stereo<i16>, alto::AsBufferData<i16>>(data, 48_000).unwrap();
        match self.sources.get_mut(&ssrc) {
            Some(source) => {
                match self.ids.get(&ssrc) {
                    Some(user_id) => {
                        let players = self.players.lock();
                        match players.get(user_id) {
                            Some(player) => {
                                source.queue_buffer(buf);
                                if source.state() != alto::SourceState::Playing {
                                    source.play();
                                }
                            },
                            None => {
                                println!("No positional data recieved for {}",user_id);
                            }
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
            match self.context.new_streaming_source() {
                Ok(source) => {
                    self.sources.insert(ssrc, source);
                },
                Err(e) => {
                    panic!("Error creating streaming source: {}", e);
                }
            }
        }
    }

}
