use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

const DEF_IP: &str = "google.com";
const DEF_COUNT: u32 = 5;

#[derive(Debug)]
struct Ping {
    avg: f32,
    ts: u64,
}

fn main() {
    println!("time_stamp ping_average");
    println!("----------------------");

    match ping_average(DEF_IP, DEF_COUNT) {
        Ok(p) => {
            println!("{} {}", p.ts, p.avg);
        },
        Err(e) => {
            println!("err: {:?}", e);
        }
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