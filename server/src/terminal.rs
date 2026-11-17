use std::io::Write;

const RAMP: &[u8] = b" .:-=+*#%@";

pub struct Screen {
    cols: usize,
    shown: Vec<u16>,
}

impl Screen {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            shown: vec![u16::MAX; cols * rows],
        }
    }

    pub fn draw(&mut self, out: &mut Vec<u8>, cells: &[u16]) {
        let mut cursor = usize::MAX;
        let mut colour = u16::MAX;
        for (i, (&cell, shown)) in cells.iter().zip(&mut self.shown).enumerate() {
            if cell == *shown {
                continue;
            }
            *shown = cell;
            if i != cursor || i % self.cols == 0 {
                write!(out, "\x1b[{};{}H", i / self.cols + 1, i % self.cols + 1).unwrap();
            }
            if cell >> 4 != colour {
                colour = cell >> 4;
                let [r, g, b] =
                    [cell >> 12, (cell >> 8) & 15, (cell >> 4) & 15].map(|v| v * 16 + 8);
                write!(out, "\x1b[38;2;{r};{g};{b}m").unwrap();
            }
            out.push(RAMP[(cell & 15) as usize]);
            cursor = i + 1;
        }
    }
}
