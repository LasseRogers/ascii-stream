use std::io::{self, Write};
use std::net::TcpStream;

use tungstenite::{Message, WebSocket};

pub struct WsWriter(WebSocket<TcpStream>);

impl WsWriter {
    pub fn accept(stream: TcpStream) -> io::Result<Self> {
        tungstenite::accept(stream)
            .map(Self)
            .map_err(|e| io::Error::other(e.to_string()))
    }
}

impl Write for WsWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .send(Message::binary(buf.to_vec()))
            .map_err(io::Error::other)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush().map_err(io::Error::other)
    }
}
