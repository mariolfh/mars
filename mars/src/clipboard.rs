use arboard::Clipboard;

pub fn copy(piece: &mut String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(piece.to_string()).unwrap();
}

pub fn paste() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let paster = clipboard.get_text().unwrap();
    return paster;
}