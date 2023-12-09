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