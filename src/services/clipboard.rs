use arboard::Clipboard;
use tokio::time::{sleep, Duration};
use zeroize::Zeroizing;

pub fn copy_with_timeout(secret: Zeroizing<String>, timeout_secs: u64) -> String {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(secret.to_string()).unwrap();

    // Spawn a background task to clear the clipboard after `timeout_secs`
    tokio::spawn(async move {
        sleep(Duration::from_secs(timeout_secs)).await;
        if let Ok(mut clipboard) = Clipboard::new() {
            let _ = clipboard.set_text("".to_string()); // clear it
        }
    });

    format!("Copied to clipboard for {timeout_secs} seconds...")
}
