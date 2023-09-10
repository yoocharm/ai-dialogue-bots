
use crate::prelude::*;
use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct App {
    #[command(flatten)]
    pub prompt: Prompt,
    #[command(flatten)]
    pub llm: LLM,
    #[command(flatten)]
    pub bot: Bot,