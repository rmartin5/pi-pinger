use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use log::{info, debug, error};
use rand::Rng;
use std::{
    fs::OpenOptions,
    io::Write,
    process::Command,
    thread::sleep,
    time::{SystemTime, UNIX_EPOCH, Duration},
};

use crate::ping::*;
mod ping;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = PingConfig::parse();
    debug!("{:?}", args);

    match args.model {
        PingModel::RandomIntervals{min_interval, max_interval} if min_interval > max_interval => {
            error!("Minimum interval ({}) is greater than maximum interval ({})", min_interval, max_interval);
            return;
        },
        _ => {}
    }

    if let Err(e) = ping_start(args) {
        error!("Failure: {:?}", e);
    }
}

fn ping_start(cfg: PingConfig) -> Result<()> {
    let mut rng = rand::thread_rng();

    let file_path = cfg.output_file.unwrap_or("ping_result.csv".to_string());
    let mut file = OpenOptions::new().append(true).create(true).open(file_path)?;

    info!("time_stamp ping_average");

    loop {
        let ping = ping_average(&cfg.ip, cfg.count)?;
        let data = format!("{} {}\n", ping.ts, ping.avg);
        file.write_all(data.as_bytes())?;
        info!("{} {}", ping.ts, ping.avg);

        let sleep_time = match cfg.model {
            PingModel::RandomIntervals{min_interval, max_interval} => {
                rng.gen_range(min_interval..=max_interval)
            },
            PingModel::ConstantIntervals{interval} => {
                interval
            }
        };

        debug!("sleep for {} seconds", sleep_time);
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