use arboard::Clipboard;

pub fn copy_to_clipboard(value: String) -> String {
    let mut clippy = Clipboard::new().unwrap();
    clippy.set_text(value).unwrap();
    "Copied to clipboard!".into()
}
