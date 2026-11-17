use std::io::Write;

use crate::protocol::{delta, hello, key, to_cells};

struct Client<W> {
    out: W,
    synced: bool,
}

pub struct Broadcast<W> {
    format: [u16; 4],
    prev: Option<Vec<u16>>,
    clients: Vec<Client<W>>,
}

impl<W: Write> Broadcast<W> {
    pub fn new(cols: u16, rows: u16, fps_num: u16, fps_den: u16) -> Self {
        Self {
            format: [cols, rows, fps_num, fps_den],
            prev: None,
            clients: Vec::new(),
        }
    }

    pub fn add(&mut self, out: W) {
        self.clients.push(Client { out, synced: false });
    }

    pub fn frame(&mut self, rgb: &[u8], index: u32) -> usize {
        let cells = to_cells(rgb);
        let [cols, rows, fps_num, fps_den] = self.format;
        let mut sync = Vec::new();
        hello(&mut sync, cols, rows, fps_num, fps_den, index);
        key(&mut sync, &cells);
        let mut update = Vec::new();
        if let Some(prev) = &self.prev {
            delta(&mut update, prev, &cells);
        }
        let before = self.clients.len();
        self.clients.retain_mut(|c| {
            let message = if c.synced { &update } else { &sync };
            c.synced = true;
            c.out.write_all(message).is_ok()
        });
        self.prev = Some(cells);
        before - self.clients.len()
    }
}
