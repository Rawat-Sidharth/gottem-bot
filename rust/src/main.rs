use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!ligma") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "ligma balls! GOTTEM").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content.starts_with("!sugma") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "sugma dick! GOTTEM").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is online!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
