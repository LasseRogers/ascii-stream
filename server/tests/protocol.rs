use server::protocol::*;

#[test]
fn packs_colour_and_glyph() {
    let rgb = [0, 0, 0, 255, 255, 255, 255, 0, 0];
    assert_eq!(to_cells(&rgb), [0x0000, 0xfff9, 0xf002]);
}
