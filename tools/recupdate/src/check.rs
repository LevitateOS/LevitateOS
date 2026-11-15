//! `recupdate check` and `recupdate notice`.

use anyhow::Result;
use serde_json::{json, Value};
use std::path::PathBuf;

pub const DEFAULT_STATE_DIR: &str = "/var/lib/recupdate";

pub fn state_dir() -> PathBuf {
    crate::util::env_path("RECUPDATE_STATE_DIR", DEFAULT_STATE_DIR)
}

/// The answer of one check, in the JSON shape of section 10.
pub fn report() -> Result<Value> {
    let os = crate::release::os_release()?;
    let installed = crate::release::installed()?;
    let channel = crate::release::load_channel()?;
    let m = &channel.manifest;
    let available = if m.serial > installed.serial {
        json!({
            "version": m.version,
            "serial": m.serial,
            "date": m.date,
            "notes": m.notes,
            "download_bytes": m.download_bytes(),
        })
    } else {
        Value::Null
    };
    Ok(json!({
        "schema": 1,
        "os": os.id,
        "installed": { "version": installed.version, "serial": installed.serial },
        "available": available,
        "checked_at": crate::util::now_rfc3339(),
    }))
}

pub fn check(json: bool) -> Result<()> {
    let report = report()?;
    if json {
        println!("{}", serde_json::to_string(&report)?);
    } else {
        let installed = &report["installed"];
        match report["available"].as_object() {
            Some(a) => println!(
                "{} {} is available (installed: {}); download {} MiB",
                report["os"].as_str().unwrap_or(""),
                a["version"].as_str().unwrap_or(""),
                installed["version"].as_str().unwrap_or(""),
                a["download_bytes"].as_u64().unwrap_or(0) / (1024 * 1024)
            ),
            None => println!(
                "{} {} is up to date",
                report["os"].as_str().unwrap_or(""),
                installed["version"].as_str().unwrap_or("")
            ),
        }
    }
    Ok(())
}

/// Run the check and write `available.json`. A failed check leaves the old file.
pub fn notice() -> Result<()> {
    let report = report()?;
    let mut text = serde_json::to_string(&report)?;
    text.push('\n');
    crate::util::write_atomic(&state_dir().join("available.json"), text.as_bytes(), 0o644)
}
