//! Native dialogs and notifications, implemented by shelling out to `zenity`
//! (and `notify-send` for non-blocking confirmations). Both are standard on
//! most Linux desktops.
use std::process::Command;

fn zenity_entry(title: &str, text: &str, default: &str, hide_text: bool) -> Option<String> {
    let mut cmd = Command::new("zenity");
    cmd.arg("--entry")
        .arg("--title")
        .arg(title)
        .arg("--text")
        .arg(text)
        .arg("--entry-text")
        .arg(default)
        .arg("--width=420");
    if hide_text {
        cmd.arg("--hide-text");
    }

    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

/// Prompt for a single line of plain text. Returns `None` if cancelled.
pub fn prompt_text(title: &str, text: &str, default: &str) -> Option<String> {
    zenity_entry(title, text, default, false)
}

/// Prompt for a secret value (input is masked). Returns `None` if cancelled.
///
/// Deliberately never pre-fills the field: zenity's masked entry can drop the
/// first character when text is pasted over a pre-selected default value.
pub fn prompt_secret(title: &str, text: &str) -> Option<String> {
    zenity_entry(title, text, "", true)
}

pub fn show_error(title: &str, text: &str) {
    let _ = Command::new("zenity")
        .arg("--error")
        .arg("--title")
        .arg(title)
        .arg("--text")
        .arg(text)
        .arg("--width=420")
        .output();
}

/// Non-blocking desktop notification, falls back to a zenity info dialog.
pub fn notify(summary: &str, body: &str) {
    let sent_ok = Command::new("notify-send")
        .arg(summary)
        .arg(body)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    if !sent_ok {
        let _ = Command::new("zenity")
            .arg("--info")
            .arg("--title")
            .arg(summary)
            .arg("--text")
            .arg(body)
            .arg("--width=420")
            .output();
    }
}
