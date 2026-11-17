use server::broadcast::Broadcast;
use server::protocol::{DELTA, KEY, apply, hello, read_message};

#[test]
fn late_client_starts_with_key() {
    let rgb = std::fs::read("../testdata/input.rgb").unwrap();
    let frames: Vec<&[u8]> = rgb.chunks(8 * 4 * 3).collect();
    let mut greeting = Vec::new();
    hello(&mut greeting, 8, 4, 30, 1);
    let (mut early, mut late) = (Vec::new(), Vec::new());
    let mut broadcast = Broadcast::new(greeting);
    broadcast.add(&mut early);
    broadcast.frame(frames[0]);
    broadcast.frame(frames[1]);
    broadcast.add(&mut late);
    broadcast.frame(frames[2]);
    broadcast.frame(frames[3]);
    drop(broadcast);
    assert_eq!(early, std::fs::read("../testdata/stream.bin").unwrap());
    let mut input = late.as_slice();
    let mut cells = vec![0u16; 8 * 4];
    let mut screens = Vec::new();
    while let Some((kind, payload)) = read_message(&mut input).unwrap() {
        if kind == KEY || kind == DELTA {
            apply(&mut cells, kind, &payload);
            screens.extend(cells.iter().flat_map(|c| c.to_le_bytes()));
        }
    }
    assert_eq!(screens, std::fs::read("../testdata/cells.bin").unwrap()[128..]);
}
