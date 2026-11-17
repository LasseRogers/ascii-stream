use server::terminal::Screen;

#[test]
fn draws_only_changes() {
    let mut screen = Screen::new(3, 2);
    let mut cells = [0u16; 6];
    let mut out = Vec::new();
    screen.draw(&mut out, &cells);
    assert_eq!(out, b"\x1b[1;1H\x1b[38;2;8;8;8m   \x1b[2;1H   ");
    out.clear();
    screen.draw(&mut out, &cells);
    assert!(out.is_empty());
    cells[5] = 0xfff9;
    out.clear();
    screen.draw(&mut out, &cells);
    assert_eq!(out, b"\x1b[2;3H\x1b[38;2;248;248;248m@");
}
