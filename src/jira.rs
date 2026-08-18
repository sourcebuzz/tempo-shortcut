//! Creates a Jira worklog on an issue. Jira Data Center's Tempo plugin tracks
//! time through native issue worklogs, so this uses Jira's own REST API
//! rather than a Tempo-specific endpoint.
use chrono::{DateTime, Local};
use reqwest::blocking::Client;
use serde_json::json;
use std::time::Duration;

pub fn add_worklog(
    base_url: &str,
    api_token: &str,
    issue_key: &str,
    started: DateTime<Local>,
    elapsed_seconds: i64,
    comment: &str,
) -> Result<(), String> {
    let url = format!(
        "{}/rest/api/2/issue/{}/worklog",
        base_url.trim_end_matches('/'),
        issue_key.trim()
    );

    // Jira rejects sub-minute worklogs, so round up to at least 60 seconds.
    let body = json!({
        "comment": comment,
        "started": started.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string(),
        "timeSpentSeconds": elapsed_seconds.max(60),
    });

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("failed to build HTTP client: {e}"))?;

    let response = client
        .post(&url)
        .bearer_auth(api_token)
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("request to Jira failed: {e}"))?;

    let status = response.status();
    if status.is_success() {
        Ok(())
    } else {
        let text = response.text().unwrap_or_default();
        Err(format!("Jira returned {status}: {text}"))
    }
}

/// Confirms the URL/token work by calling Jira's "myself" endpoint, returning the display name.
pub fn verify_token(base_url: &str, api_token: &str) -> Result<String, String> {
    let url = format!("{}/rest/api/2/myself", base_url.trim_end_matches('/'));

    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("failed to build HTTP client: {e}"))?;

    let response = client
        .get(&url)
        .bearer_auth(api_token)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| format!("request to Jira failed: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return Err(format!("Jira returned {status}: {text}"));
    }

    let value: serde_json::Value = response
        .json()
        .map_err(|e| format!("could not parse Jira response: {e}"))?;
    Ok(value
        .get("displayName")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown user")
        .to_string())
}
