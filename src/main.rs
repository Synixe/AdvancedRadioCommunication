mod handler;
mod player;
mod server;
mod voice;

extern crate alto;

extern crate serenity;
use serenity::client::Client;
use serenity::prelude::Mutex;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use handler::Handler;
use player::*;
use voice::VoiceManager;

fn main() {

    let players: Arc<Mutex<HashMap<String, Player>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    {
        let mut data = client.data.lock();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<PlayerManager>(Arc::clone(&players.clone()));
    }

    client.start();
}
