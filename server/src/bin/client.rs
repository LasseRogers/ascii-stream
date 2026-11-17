use std::error::Error;
use std::io::{BufReader, ErrorKind, Read, Write};
use std::net::TcpStream;

use server::protocol::{DELTA, HELLO, KEY, VERSION, apply, read_message};
use server::terminal::Screen;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7878".into());
    let mut input = BufReader::new(TcpStream::connect(&addr)?);
    let (kind, hello) = read_message(&mut input)?.ok_or("stream ended before HELLO")?;
    if kind != HELLO || hello.len() < 13 || hello[0] != VERSION {
        return Err("expected HELLO, protocol version 1".into());
    }
    let cols = u16::from_le_bytes([hello[1], hello[2]]) as usize;
    let rows = u16::from_le_bytes([hello[3], hello[4]]) as usize;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(b"\x1b[?1049h\x1b[2J")?;
    let result = play(&mut input, &mut stdout, cols, rows);
    stdout.write_all(b"\x1b[0m\x1b[?1049l")?;
    stdout.flush()?;
    match result {
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok(()),
        other => other.map_err(Into::into),
    }
}

fn play(
    input: &mut impl Read,
    stdout: &mut impl Write,
    cols: usize,
    rows: usize,
) -> std::io::Result<()> {
    let mut cells = vec![0; cols * rows];
    let mut screen = Screen::new(cols, rows);
    let mut out = Vec::new();
    while let Some((kind, payload)) = read_message(input)? {
        if kind == KEY || kind == DELTA {
            apply(&mut cells, kind, &payload);
            out.clear();
            screen.draw(&mut out, &cells);
            stdout.write_all(&out)?;
            stdout.flush()?;
        }
    }
    Ok(())
}
