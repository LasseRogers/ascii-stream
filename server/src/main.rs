use std::io::Write;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

use server::broadcast::Broadcast;
use server::video::{frames, probe};
use server::websocket::WsWriter;

const ADDR: &str = "127.0.0.1:7878";
const WS_ADDR: &str = "127.0.0.1:7879";
const WRITE_TIMEOUT: Duration = Duration::from_millis(250);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: server <video>")?;
    let info = probe(&path)?;
    let cols: u16 = 160;
    let rows = (cols as f64 * info.height as f64 / info.width as f64 * 0.5).round() as u16;
    let broadcast = Arc::new(Mutex::new(Broadcast::<Box<dyn Write + Send>>::new(
        cols,
        rows,
        info.fps_num,
        info.fps_den,
    )));
    let listener = TcpListener::bind(ADDR)?;
    eprintln!("listening on {ADDR}");
    let accepting = Arc::clone(&broadcast);
    spawn(move || {
        for stream in listener.incoming().flatten() {
            if let Ok(peer) = stream.peer_addr() {
                eprintln!("client {peer}");
            }
            stream
                .set_write_timeout(Some(WRITE_TIMEOUT))
                .expect("timeout is non-zero");
            accepting.lock().unwrap().add(Box::new(stream));
        }
    });
    let ws_listener = TcpListener::bind(WS_ADDR)?;
    eprintln!("websocket on {WS_ADDR}");
    let shared = Arc::clone(&broadcast);
    spawn(move || accept_websockets(ws_listener, shared));
    let mut buf = vec![0; cols as usize * rows as usize * 3];
    let frame_time = Duration::from_secs_f64(info.fps_den as f64 / info.fps_num as f64);
    let start = Instant::now();
    let mut count = 0;
    loop {
        let mut video = frames(&path, cols, rows)?;
        let mut index = 0;
        while video.read(&mut buf)? {
            let dropped = broadcast.lock().unwrap().frame(&buf, index);
            if dropped > 0 {
                eprintln!("dropped {dropped} client(s)");
            }
            count += 1;
            index += 1;
            if let Some(wait) = (start + frame_time * count).checked_duration_since(Instant::now())
            {
                sleep(wait);
            }
        }
    }
}

type Shared = Arc<Mutex<Broadcast<Box<dyn Write + Send>>>>;

fn accept_websockets(listener: TcpListener, broadcast: Shared) {
    for stream in listener.incoming().flatten() {
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("timeout is non-zero");
        stream
            .set_write_timeout(Some(WRITE_TIMEOUT))
            .expect("timeout is non-zero");
        match WsWriter::accept(stream) {
            Ok(ws) => {
                eprintln!("websocket client");
                broadcast.lock().unwrap().add(Box::new(ws));
            }
            Err(e) => eprintln!("websocket handshake failed: {e}"),
        }
    }
}
