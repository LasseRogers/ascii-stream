use std::io::Write;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

use server::broadcast::Broadcast;
use server::protocol::hello;
use server::video::{frames, probe};

const ADDR: &str = "127.0.0.1:7878";
const WRITE_TIMEOUT: Duration = Duration::from_millis(250);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: server <video>")?;
    let info = probe(&path)?;
    let cols: u16 = 160;
    let rows = (cols as f64 * info.height as f64 / info.width as f64 * 0.5).round() as u16;
    let mut greeting = Vec::new();
    hello(&mut greeting, cols, rows, info.fps_num, info.fps_den);
    let broadcast = Arc::new(Mutex::new(Broadcast::<Box<dyn Write + Send>>::new(greeting)));
    let listener = TcpListener::bind(ADDR)?;
    eprintln!("listening on {ADDR}");
    let accepting = Arc::clone(&broadcast);
    spawn(move || {
        for stream in listener.incoming().flatten() {
            if let Ok(peer) = stream.peer_addr() {
                eprintln!("client {peer}");
            }
            stream.set_write_timeout(Some(WRITE_TIMEOUT)).expect("timeout is non-zero");
            accepting.lock().unwrap().add(Box::new(stream));
        }
    });
    let mut buf = vec![0; cols as usize * rows as usize * 3];
    let frame_time = Duration::from_secs_f64(info.fps_den as f64 / info.fps_num as f64);
    let start = Instant::now();
    let mut count = 0;
    loop {
        let mut video = frames(&path, cols, rows)?;
        while video.read(&mut buf)? {
            let dropped = broadcast.lock().unwrap().frame(&buf);
            if dropped > 0 {
                eprintln!("dropped {dropped} slow client(s)");
            }
            count += 1;
            if let Some(wait) = (start + frame_time * count).checked_duration_since(Instant::now()) {
                sleep(wait);
            }
        }
    }
}
