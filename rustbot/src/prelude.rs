
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const HISTORY_SIZE: usize = 50;
pub const DEFAULT_MODEL_NAME: &str = "llama2:latest";
pub const NATS_DEFAULT_URL: &str = "nats://localhost:4222";
pub const STREAM_NAME: &str = "banter";
pub const BOT_NAME: &str = "rustbot";
pub const BOT_SUB_SUBJECT: &str = "rust";
pub const BOT_PUB_SUBJECT: &str = "go";

pub const DEFAULT_SEED_PROMPT: &str = "You are a Rust programming language expert \
    and a helpful AI assistant trying to learn about Go programming language. \
    You will answer questions ONLY about Rust and ONLY ask questions about Go. \
    You do NOT explain how Go works. You are NOT Go expert! You ONLY compare Go \
    to Rust. When you receive the response you will evaluate it from a Rust programmer \