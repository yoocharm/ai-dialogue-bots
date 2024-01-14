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
        pub_subject: args.bot.pub_subject,
        sub_subject: args.bot.sub_subject,
        ..jet::Config::default()
    };
    let s = jet::Stream::new(c).await?;

    // NOTE: we could also add LLM::builder to the llm module
    // and instead of passing config we could build it by chaining methods.
    let c = llm::Config {
        hist_size: args.llm.hist_size,
        model_name: args.llm.model_name,
        seed_prompt: Some(seed_prompt),
        ..llm::Config::default()
    };
    let l = llm::LLM::new(c);

    // NOTE: we could also add TTS::builder to the tts module
    // and instead of passing config we could build it by chaining methods.
    let c = tts::Config {
        voice_id: Some(args.tts.voice_id),
        ..tts::Config::default()
    };
    let t = tts::TTS::new(c);

    let (prompts_tx, prompts_rx) = mpsc::channel::<String>(32);
    let (jet_chunks_tx, jet_chun