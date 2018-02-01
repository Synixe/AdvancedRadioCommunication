use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

use std::thread;

use player::PlayerManager;
use server::Server;
use voice::Receiver;
use voice::VoiceManager;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().unwrap();
        let mut manager = manager_lock.lock();
        let channel = ChannelId(385205936819666968);
        match manager.join(385205936819666964, channel) {
            Some(handler) => {
                let ret = Receiver::new(ctx.data.lock().get::<PlayerManager>().cloned().unwrap());
                handler.listen(Some(Box::new(ret)));

                thread::spawn(move || {
                    let server = Server::new(ctx.data.lock().get::<PlayerManager>().cloned().unwrap());
                    server.run();
                });
            },
            None => {
                println!("Failed to connect to voice channel");
            }
        }
    }
}
