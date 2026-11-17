use std::io::{BufReader, Write};
use std::net::TcpStream;

use server::protocol::{DELTA, HELLO, KEY, VERSION, apply, read_message};
use server::terminal::Screen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:7878".into());
    let mut input = BufReader::new(TcpStream::connect(&addr)?);
    let (kind, hello) = read_message(&mut input)?.ok_or("stream ended before HELLO")?;
    if kind != HELLO || hello.len() < 9 || hello[0] != VERSION {
        return Err("expected HELLO, protocol version 1".into());
    }
    let cols = u16::from_le_bytes([hello[1], hello[2]]) as usize;
    let rows = u16::from_le_bytes([hello[3], hello[4]]) as usize;
    let mut cells = vec![0; cols * rows];
    let mut screen = Screen::new(cols, rows);
    let mut out = Vec::new();
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(b"\x1b[2J")?;
    while let Some((kind, payload)) = read_message(&mut input)? {
        if kind == KEY || kind == DELTA {
            apply(&mut cells, kind, &payload);
            out.clear();
            screen.draw(&mut out, &cells);
            stdout.write_all(&out)?;
            stdout.flush()?;
        }
    }
    write!(stdout, "\x1b[0m\x1b[{};1H", rows + 1)?;
    Ok(())
}
