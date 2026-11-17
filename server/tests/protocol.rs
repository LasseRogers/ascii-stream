use server::protocol::*;

#[test]
fn packs_colour_and_glyph() {
    let rgb = [0, 0, 0, 255, 255, 255, 255, 0, 0];
    assert_eq!(to_cells(&rgb), [0x0000, 0xfff9, 0xf002]);
}

#[test]
fn hello_bytes() {
    let mut out = Vec::new();
    hello(&mut out, 160, 45, 30, 1);
    assert_eq!(out, [1, 9, 0, 0, 0, 1, 0xa0, 0, 0x2d, 0, 0x1e, 0, 1, 0]);
}

#[test]
fn key_bytes() {
    let mut out = Vec::new();
    key(&mut out, &[0x0000, 0xfff9, 0xf002]);
    assert_eq!(out, [2, 6, 0, 0, 0, 0, 0, 0xf9, 0xff, 0x02, 0xf0]);
}
