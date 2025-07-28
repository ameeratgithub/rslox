
pub mod chunk;
#[cfg(feature = "debug_trace_execution")]
pub mod debug;
pub mod value;
pub mod vm;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author,version, about, long_about=None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,
}