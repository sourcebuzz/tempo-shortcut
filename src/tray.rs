use crate::config::Config;
use crate::{dialogs, icons, jira};
use chrono::{DateTime, Local};
use ksni::menu::StandardItem;
use ksni::{Icon, MenuItem, Tray};
use std::time::Instant;

pub enum State {
    Idle,
    Running {
        issue_key: String,
        started_at: DateTime<Local>,
        started_instant: Instant,
    },
}

pub struct AppTray {
    config: Config,
    state: State,
}

impl AppTray {
    pub fn new(config: Config) -> Self {
        AppTray { config, state: State::Idle }
    }

    fn start_timer(&mut self) {
        if !self.config.is_configured() {
            dialogs::show_error(
                "Tempo Shortcut",
                "Set the Jira URL and API token first, using the Settings menu item.",
            );
            return;
        }

        let Some(issue_key) = dialogs::prompt_text(
            "Start Timer",
            "Jira issue key (e.g. RASA-123):",
            "",
        ) else {
            return;
        };
        let issue_key = issue_key.trim().to_string();
        if issue_key.is_empty() {
            return;
        }

        self.state = State::Running {
            issue_key,
            started_at: Local::now(),
            started_instant: Instant::now(),
        };
    }

    fn stop_timer(&mut self) {
        let State::Running { issue_key, started_at, started_instant } =
            std::mem::replace(&mut self.state, State::Idle)
        else {
            return;
        };

        let elapsed_seconds = started_instant.elapsed().as_secs() as i64;

        let Some(comment) = dialogs::prompt_text(
            "Stop Timer",
            &format!("Work log description for {issue_key}:"),
            "",
        ) else {
            // Cancelled: keep the timer running so no tracked time is lost.
            self.state = State::Running { issue_key, started_at, started_instant };
            return;
        };

        match jira::add_worklog(
            &self.config.jira_url,
            &self.config.api_token,
            &issue_key,
            started_at,
            elapsed_seconds,
            &comment,
        ) {
            Ok(()) => {
                let minutes = (elapsed_seconds as f64 / 60.0).round() as i64;
                dialogs::notify("Tempo Shortcut", &format!("Logged {minutes} min on {issue_key}"));
            }
            Err(err) => {
                dialogs::show_error(
                    "Tempo Shortcut - worklog failed",
                    &format!(
                        "Could not log time on {issue_key} ({elapsed_seconds}s elapsed):\n{err}\n\nDescription: {comment}"
                    ),
                );
            }
        }
    }

    fn open_settings(&mut self) {
        let Some(url) = dialogs::prompt_text(
            "Jira Settings",
            "Jira base URL (e.g. https://jira.arasdp.ir):",
            &self.config.jira_url,
        ) else {
            return;
        };

        // Left blank on purpose: the field is never pre-filled with the
        // current token (see prompt_secret), so blank means "keep it".
        let Some(token_input) = dialogs::prompt_secret(
            "Jira Settings",
            "Jira API token (leave blank to keep the current one):",
        ) else {
            return;
        };

        let new_url = url.trim().trim_end_matches('/').to_string();
        let new_token = if token_input.trim().is_empty() {
            self.config.api_token.clone()
        } else {
            token_input.trim().to_string()
        };

        match jira::verify_token(&new_url, &new_token) {
            Ok(display_name) => {
                self.config.jira_url = new_url;
                self.config.api_token = new_token;
                match self.config.save() {
                    Ok(()) => dialogs::notify(
                        "Tempo Shortcut",
                        &format!("Settings saved. Signed in as {display_name}."),
                    ),
                    Err(err) => dialogs::show_error(
                        "Tempo Shortcut",
                        &format!("Verified but failed to save settings: {err}"),
                    ),
                }
            }
            Err(err) => dialogs::show_error(
                "Tempo Shortcut - Settings not saved",
                &format!("Could not verify the Jira URL/token, nothing was changed:\n{err}"),
            ),
        }
    }
}

impl Tray for AppTray {
    // Left click opens the menu directly instead of calling `activate`.
    const MENU_ON_ACTIVATE: bool = true;

    fn id(&self) -> String {
        "tempo-shortcut".into()
    }

    fn title(&self) -> String {
        match &self.state {
            State::Idle => "Tempo Shortcut".into(),
            State::Running { issue_key, .. } => format!("Tempo Shortcut - tracking {issue_key}"),
        }
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        match self.state {
            State::Idle => vec![icons::idle_icon()],
            State::Running { .. } => vec![icons::running_icon()],
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let mut items = Vec::new();

        match &self.state {
            State::Idle => items.push(
                StandardItem {
                    label: "Start Timer...".into(),
                    icon_name: "media-playback-start".into(),
                    activate: Box::new(|tray: &mut AppTray| tray.start_timer()),
                    ..Default::default()
                }
                .into(),
            ),
            State::Running { issue_key, .. } => items.push(
                StandardItem {
                    label: format!("Stop Timer ({issue_key})"),
                    icon_name: "media-playback-stop".into(),
                    activate: Box::new(|tray: &mut AppTray| tray.stop_timer()),
                    ..Default::default()
                }
                .into(),
            ),
        }

        items.push(MenuItem::Separator);
        items.push(
            StandardItem {
                label: "Settings...".into(),
                icon_name: "preferences-system".into(),
                activate: Box::new(|tray: &mut AppTray| tray.open_settings()),
                ..Default::default()
            }
            .into(),
        );
        items.push(MenuItem::Separator);
        items.push(
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_: &mut AppTray| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        );

        items
    }
}
