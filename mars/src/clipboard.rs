use arboard::Clipboard;

pub fn paste() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let paster = clipboard.get_text().unwrap();
    return paster;
}