use bytes::Bytes;
use clap::Parser;
use prelude::*;
use rodio::{OutputStream, Sink};
use tokio::{
    self, io,
    sync::{mpsc, watch},
};

mod audio;
mod buffer;
mod cli;
mod history;
mod jet;
mod llm;
mod prelude;
mod signal;
mod tts;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::App::parse();

    let seed_prompt = args.prompt.seed.unwrap();

    // NOTE: we could also add Stream::builder to the jet module
    // and instead of passing config we could build it by chaining methods.
    let c = jet::Config {
        durable_name: args.bot.name,
        stream_name: args.bot.stream_name,
   