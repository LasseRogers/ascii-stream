use std::io::Write;

use crate::protocol::{delta, key, to_cells};

struct Client<W> {
    out: W,
    synced: bool,
}

pub struct Broadcast<W> {
    hello: Vec<u8>,
    prev: Option<Vec<u16>>,
    clients: Vec<Client<W>>,
}

impl<W: Write> Broadcast<W> {
    pub fn new(hello: Vec<u8>) -> Self {
        Self { hello, prev: None, clients: Vec::new() }
    }

    pub fn add(&mut self, out: W) {
        self.clients.push(Client { out, synced: false });
    }

    pub fn frame(&mut self, rgb: &[u8]) {
        let cells = to_cells(rgb);
        let mut sync = self.hello.clone();
        key(&mut sync, &cells);
        let mut update = Vec::new();
        if let Some(prev) = &self.prev {
            delta(&mut update, prev, &cells);
        }
        self.clients.retain_mut(|c| {
            let message = if c.synced { &update } else { &sync };
            c.synced = true;
            c.out.write_all(message).is_ok()
        });
        self.prev = Some(cells);
    }
}
