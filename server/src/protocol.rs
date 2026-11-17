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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_colour_and_glyph() {
        let rgb = [0, 0, 0, 255, 255, 255, 255, 0, 0];
        assert_eq!(to_cells(&rgb), [0x0000, 0xfff9, 0xf002]);
    }
}
