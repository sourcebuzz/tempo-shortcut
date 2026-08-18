mod config;
mod dialogs;
mod icons;
mod jira;
mod tray;

use config::Config;
use ksni::blocking::TrayMethods;
use tray::AppTray;

fn main() {
    let config = Config::load();
    if !config.is_configured() {
        dialogs::notify(
            "Tempo Shortcut",
            "Not configured yet - use the tray menu's Settings item to set your Jira URL and API token.",
        );
    }

    let handle = AppTray::new(config)
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("Failed to start tray icon: {err}");
            std::process::exit(1);
        });

    loop {
        std::thread::park();
        if handle.is_closed() {
            break;
        }
    }
}
