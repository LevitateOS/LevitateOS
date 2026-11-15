//! `recupdate notify-user`: one desktop notification per new serial per user.

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn memory_file() -> PathBuf {
    let base = std::env::var_os("XDG_STATE_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    base.join("recupdate/last-notified")
}

/// Quote a string as a GVariant text literal.
fn gv(text: &str) -> String {
    format!("'{}'", text.replace('\\', "\\\\").replace('\'', "\\'"))
}

pub fn run() -> Result<()> {
    let path = crate::check::state_dir().join("available.json");
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => return Ok(()), // no check has run yet
    };
    let report: serde_json::Value =
        serde_json::from_str(&text).with_context(|| format!("{} is not JSON", path.display()))?;
    let Some(available) = report["available"].as_object() else {
        return Ok(());
    };
    let serial = available
        .get("serial")
        .and_then(|s| s.as_u64())
        .context("available.json has no serial")?;
    let version = available
        .get("version")
        .and_then(|s| s.as_str())
        .unwrap_or("");

    let memory = memory_file();
    if fs::read_to_string(&memory)
        .ok()
        .and_then(|t| t.trim().parse::<u64>().ok())
        == Some(serial)
    {
        return Ok(());
    }

    let name = crate::release::os_release()
        .map(|o| o.name)
        .unwrap_or_else(|_| "The system".into());
    let title = format!("{name} {version} is available");
    let body = if std::path::Path::new("/usr/bin/recab").exists() {
        "Open the Recab app to update."
    } else {
        "Open the updater to update."
    };
    send(&title, body)?;
    if let Some(dir) = memory.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(&memory, format!("{serial}\n"))
        .with_context(|| format!("writing {}", memory.display()))?;
    Ok(())
}

fn send(title: &str, body: &str) -> Result<()> {
    let status = if crate::util::which("notify-send").is_some() {
        Command::new("notify-send")
            .args([
                "-a",
                "recupdate",
                "-i",
                "system-software-update",
                title,
                body,
            ])
            .status()
    } else if crate::util::which("gdbus").is_some() {
        Command::new("gdbus")
            .args([
                "call",
                "--session",
                "--dest",
                "org.freedesktop.Notifications",
                "--object-path",
                "/org/freedesktop/Notifications",
                "--method",
                "org.freedesktop.Notifications.Notify",
                "recupdate",
                "0",
                "system-software-update",
                &gv(title),
                &gv(body),
                "[]",
                "{}",
                "-1",
            ])
            .stdout(std::process::Stdio::null())
            .status()
    } else {
        bail!("neither notify-send nor gdbus is installed; cannot show the notification");
    };
    let status = status.context("running the notification program")?;
    if !status.success() {
        bail!("the notification program failed ({status})");
    }
    Ok(())
}
