use clap::{Parser, Subcommand};

/// Ping output layout
#[derive(Debug)]
pub struct Ping {
    /// Average round-trip of ping(s)
    pub avg: f32,
    /// Unix Epoch current timestamp
    pub ts: u64,
}

/// Configuration of current pinger
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PingConfig {
    /// Location of output file. Default to "ping_result.csv" of current directory.
    #[arg(short, long)]
    pub output_file: Option<String>,
    /// Target ping server. DNS or IPv4 address.
    #[arg(short, long)]
    pub ip: String,
    /// Number of pings per ping iteration. Defaults to 5.
    #[arg(short, long, default_value_t = 5)]
    pub count: u32,
    /// Which ping interval model to use.
    #[command(subcommand)]
    pub model: PingModel,
}

#[derive(Subcommand, Debug)]
pub enum PingModel {
    /// Random ping interval based on min and max input
    #[command(name = "random")]
    RandomIntervals{
        /// Max interval in seconds
        #[arg(short = 'm', long = "max", default_value_t = 3600)]
        max_interval: u64,
        /// Min interval in seconds
        #[arg(short = 'n', long = "min", default_value_t = 300)]
        min_interval: u64,
    },
    /// Constant ping interval
    #[command(name = "constant")]
    ConstantIntervals{
        /// Interval in seconds
        #[arg(short, long, default_value_t = 300)]
        interval: u64,
    },
}