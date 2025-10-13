use arboard::Clipboard;

pub fn copy(copier: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(copier.to_string()).unwrap();
}

pub fn paste() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let paster = clipboard.get_text().unwrap();
    return paster;
}