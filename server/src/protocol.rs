pub const HELLO: u8 = 1;
pub const KEY: u8 = 2;
pub const DELTA: u8 = 3;
pub const VERSION: u8 = 1;
const LEVELS: u32 = 10;

pub fn to_cells(rgb: &[u8]) -> Vec<u16> {
    rgb.chunks_exact(3)
        .map(|p| {
            let (r, g, b) = (p[0] as u32, p[1] as u32, p[2] as u32);
            let glyph = ((54 * r + 183 * g + 19 * b) >> 8) * LEVELS >> 8;
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
