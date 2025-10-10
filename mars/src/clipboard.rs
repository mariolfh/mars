use arboard::Clipboard;

fn copy(){
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text("".to_string()).unwrap();
}