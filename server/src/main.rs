use std::io::Write;
use std::net::TcpListener;
use std::thread::sleep;
use std::time::{Duration, Instant};

use server::protocol::{Encoder, hello};
use server::video::{frames, probe};

const ADDR: &str = "127.0.0.1:7878";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: server <video>")?;
    let info = probe(&path)?;
    let cols: u16 = 160;
    let rows = (cols as f64 * info.height as f64 / info.width as f64 * 0.5).round() as u16;
    let listener = TcpListener::bind(ADDR)?;
    eprintln!("listening on {ADDR}");
    let (mut stream, peer) = listener.accept()?;
    eprintln!("client {peer}");
    let mut out = Vec::new();
    hello(&mut out, cols, rows, info.fps_num, info.fps_den);
    stream.write_all(&out)?;
    let mut video = frames(&path, cols, rows)?;
    let mut encoder = Encoder::default();
    let mut buf = vec![0; cols as usize * rows as usize * 3];
    let frame_time = Duration::from_secs_f64(info.fps_den as f64 / info.fps_num as f64);
    let start = Instant::now();
    let mut count = 0;
    while video.read(&mut buf)? {
        out.clear();
        encoder.frame(&mut out, &buf);
        stream.write_all(&out)?;
        count += 1;
        if let Some(wait) = (start + frame_time * count).checked_duration_since(Instant::now()) {
            sleep(wait);
        }
    }
    eprintln!("sent {count} frames");
    Ok(())
}
