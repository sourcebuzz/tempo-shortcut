# tempo-shortcut

A minimal Linux system-tray app for tracking time on Jira issues (works with
Jira Server/Data Center instances running the Tempo Timesheets plugin, since
Tempo tracks time through native Jira worklogs).

## Usage

- Click the tray icon to open the menu.
- **Start Timer...** asks for a Jira issue key (e.g. `RASA-123`) and starts
  tracking. The icon turns into an hourglass while a timer is running.
- Click the icon again and choose **Stop Timer** to stop tracking. You'll be
  asked for a description, which is then submitted as a work log on the issue
  via Jira's REST API (`POST /rest/api/2/issue/{key}/worklog`), using the
  elapsed time.
- **Settings...** lets you set/update the Jira base URL and API token.
- **Exit** quits the app.

## Configuration

Settings are stored in `~/.config/tempo-shortcut/config.toml` (created with
`0600` permissions since it contains your API token). Nothing is stored in
this repository - set the URL and token the first time from the tray's
**Settings...** menu.

The token is sent as `Authorization: Bearer <token>` (Jira Personal Access
Token). If your instance instead requires a Tempo-specific API token/header,
adjust [src/jira.rs](src/jira.rs).

## Requirements

- Linux desktop with a StatusNotifierItem/AppIndicator host (GNOME needs the
  "AppIndicator and KStatusNotifierItem Support" extension; KDE/most other
  DEs work out of the box).
- `zenity` (input dialogs) and `notify-send` (optional, for confirmations).

## Build & run

```sh
cargo build --release
./target/release/tempo-shortcut
```

Or install a pre-built binary with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
straight from GitHub releases (no compiling, no crates.io publish required):

```sh
git clone https://github.com/sourcebuzz/tempo-shortcut && cd tempo-shortcut
cargo binstall --manifest-path Cargo.toml --no-confirm tempo-shortcut
```

## Releasing

Bump `version` in [Cargo.toml](Cargo.toml), commit, then:

```sh
make release
```

This builds the release binary, tags the commit (`vX.Y.Z`), pushes the tag,
and publishes a GitHub release with the binary attached (requires the `gh`
CLI to be authenticated).

To also publish the crate to crates.io (requires `cargo login` first):

```sh
make publish
```

