use std::sync::Arc;
use songbird;

pub struct Data {
    pub songbird: Arc<songbird::Songbird>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;