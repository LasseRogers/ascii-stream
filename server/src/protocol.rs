pub const HELLO: u8 = 1;
pub const KEY: u8 = 2;
pub const DELTA: u8 = 3;
pub const VERSION: u8 = 1;
const LEVELS: u32 = 10;

pub fn to_cells(rgb: &[u8]) -> Vec<u16> {
    rgb.as_chunks::<3>().0.iter()
        .map(|p| {
            let (r, g, b) = (p[0] as u32, p[1] as u32, p[2] as u32);
            let glyph = (((54 * r + 183 * g + 19 * b) >> 8) * LEVELS) >> 8;
            ((r >> 4) << 12 | (g >> 4) << 8 | (b >> 4) << 4 | glyph) as u16
        })
        .collect()
}

fn message(out: &mut Vec<u8>, kind: u8, payload: &[u8]) {
    out.push(kind);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
}

pub fn hello(out: &mut Vec<u8>, cols: u16, rows: u16, fps_num: u16, fps_den: u16) {
    let mut payload = vec![VERSION];
    for v in [cols, rows, fps_num, fps_den] {
        payload.extend_from_slice(&v.to_le_bytes());
    }
    message(out, HELLO, &payload);
}

pub fn key(out: &mut Vec<u8>, cells: &[u16]) {
    let payload: Vec<u8> = cells.iter().flat_map(|c| c.to_le_bytes()).collect();
    message(out, KEY, &payload);
}

pub fn delta(out: &mut Vec<u8>, prev: &[u16], cells: &[u16]) {
    let mut payload = Vec::new();
    let mut i = 0;
    while i < cells.len() {
        if cells[i] == prev[i] {
            i += 1;
            continue;
        }
        let start = i;
        while i < cells.len() && cells[i] != prev[i] {
            i += 1;
        }
        let header = [start as u16, (i - start) as u16];
        for v in header.iter().chain(&cells[start..i]) {
            payload.extend_from_slice(&v.to_le_bytes());
        }
    }
    message(out, DELTA, &payload);
}

#[derive(Default)]
pub struct Encoder {
    prev: Option<Vec<u16>>,
}

impl Encoder {
    pub fn frame(&mut self, out: &mut Vec<u8>, rgb: &[u8]) {
        let cells = to_cells(rgb);
        match &self.prev {
            None => key(out, &cells),
            Some(prev) => delta(out, prev, &cells),
        }
        self.prev = Some(cells);
    }
}

pub fn encode(out: &mut Vec<u8>, rgb: &[u8], cols: u16, rows: u16, fps_num: u16, fps_den: u16) {
    hello(out, cols, rows, fps_num, fps_den);
    let mut encoder = Encoder::default();
    for frame in rgb.chunks_exact(cols as usize * rows as usize * 3) {
        encoder.frame(out, frame);
    }
}

pub fn read_message(input: &mut impl std::io::Read) -> std::io::Result<Option<(u8, Vec<u8>)>> {
    let mut header = [0; 5];
    match input.read_exact(&mut header) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }
    let length = u32::from_le_bytes(header[1..].try_into().unwrap()) as usize;
    let mut payload = vec![0; length];
    input.read_exact(&mut payload)?;
    Ok(Some((header[0], payload)))
}

pub fn apply(cells: &mut [u16], kind: u8, payload: &[u8]) {
    let mut values = payload.as_chunks::<2>().0.iter().map(|b| u16::from_le_bytes(*b));
    match kind {
        KEY => cells.iter_mut().zip(values).for_each(|(c, v)| *c = v),
        DELTA => {
            while let (Some(start), Some(count)) = (values.next(), values.next()) {
                for c in &mut cells[start as usize..][..count as usize] {
                    *c = values.next().unwrap_or(0);
                }
            }
        }
        _ => {}
    }
}
