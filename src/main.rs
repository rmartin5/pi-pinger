use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread::sleep;
use std::fs::OpenOptions;
use std::io::Write;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rand::Rng;

// TODO better organization of struct/enum/func
#[derive(Debug)]
struct Ping {
    avg: f32,
    ts: u64,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct PingConfig {
    #[arg(short, long)]
    output_file: Option<String>,
    #[arg(short, long)]
    ip: String,
    #[arg(short, long, default_value_t = 5)]
    count: u32,
    #[command(subcommand)]
    model: PingModel,
}

#[derive(Subcommand, Debug)]
enum PingModel {
    #[command(name = "random")]
    RandomIntervals{
        /// Max interval in seconds
        #[arg(short = 'm', long = "max", default_value_t = 3600)]
        max_interval: u64,
        /// Min interval in seconds
        #[arg(short = 'n', long = "min", default_value_t = 300)]
        min_interval: u64,
    },
    #[command(name = "constant")]
    ConstantIntervals{
        /// Interval in seconds
        #[arg(short, long, default_value_t = 300)]
        interval: u64,
    },
}

fn main() {
    let args = PingConfig::parse();

    println!("{:?}", args);

    match args.model {
        PingModel::RandomIntervals{min_interval, max_interval} if min_interval > max_interval => {
            println!("error: Minimum interval ({}) is greater than maximum interval ({})", min_interval, max_interval);
            return;
        },
        _ => {}
    }

    if let Err(e) = ping_start(args) {
        println!("error: {:?}", e);
    }
}

fn ping_start(cfg: PingConfig) -> Result<()> {
    let mut rng = rand::thread_rng();

    let file_path = cfg.output_file.unwrap_or("ping_result.csv".to_string());
    let mut file = OpenOptions::new().append(true).create(true).open(file_path)?;

    println!("time_stamp ping_average");

    loop {
        let ping = ping_average(&cfg.ip, cfg.count)?;
        let data = format!("{} {}\n", ping.ts, ping.avg);
        file.write_all(data.as_bytes())?;
        println!("{} {}", ping.ts, ping.avg);

        let sleep_time = match cfg.model {
            PingModel::RandomIntervals{min_interval, max_interval} => {
                rng.gen_range(min_interval..=max_interval)
            },
            PingModel::ConstantIntervals{interval} => {
                interval
            }
        };

        sleep(Duration::from_secs(sleep_time));
    }
}

/// Average round-trip ping
/// will be 0 (packets received) on ping timeout
fn ping_average(ip: &str, c: u32) -> Result<Ping> {
    let cmd = format!("ping {} -c {} | tail -1 | awk '{{print $4}}' | cut -d '/' -f 2", ip, c);
    let output = Command::new("sh").arg("-c").arg(cmd).output()?;

    let avg = String::from_utf8(output.stdout)?.trim().parse()?;
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    Ok(Ping{avg, ts})
}