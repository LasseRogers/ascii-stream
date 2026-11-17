use server::video::{frames, probe};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("usage: server <video>")?;
    let info = probe(&path)?;
    let cols: u16 = 160;
    let rows = (cols as f64 * info.height as f64 / info.width as f64 * 0.5).round() as u16;
    let mut video = frames(&path, cols, rows)?;
    let mut buf = vec![0; cols as usize * rows as usize * 3];
    let mut count = 0;
    while video.read(&mut buf)? {
        count += 1;
    }
    println!("{cols}x{rows}, {count} frames");
    Ok(())
}
