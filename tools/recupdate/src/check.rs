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
    Ok(build_report(&os, &installed, &channel.manifest))
}

/// The report of section 10 from the three things a check reads.
fn build_report(
    os: &crate::release::OsRelease,
    installed: &crate::release::Installed,
    m: &crate::release::Manifest,
) -> Value {
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
    // An apply that began changing the system and did not finish (`recupdate apply` finishes
    // it). An unreadable record counts as one: apply reports what is wrong with it.
    let interrupted = match crate::apply::read_intent() {
        Ok(Some(intent)) => json!({ "version": intent.version, "serial": intent.serial }),
        Ok(None) => Value::Null,
        Err(_) => json!({ "version": "", "serial": 0 }),
    };
    json!({
        "schema": 1,
        "os": os.id,
        "installed": { "version": installed.version, "serial": installed.serial },
        "available": available,
        "interrupted": interrupted,
        "checked_at": crate::util::now_rfc3339(),
    })
}

pub fn check(json: bool) -> Result<()> {
    let report = report()?;
    if json {
        println!("{}", serde_json::to_string(&report)?);
    } else {
        let installed = &report["installed"];
        if let Some(i) = report["interrupted"].as_object() {
            println!(
                "an update to {} (serial {}) was interrupted; run `recupdate apply` as root to finish it",
                i["version"].as_str().unwrap_or(""),
                i["serial"].as_u64().unwrap_or(0)
            );
        }
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

/// Run the check and write `available.json`; the background job of the daily timer.
///
/// A check that could not be made is a normal outcome for a background job: the machine is
/// offline, the channel does not exist yet (404), the channel's files do not verify, the
/// system has no release record or no release key. It prints one line to stderr, leaves the
/// old `available.json` as it is and exits 0, so the timer's unit does not count as failed.
/// A fault of this machine still exits 1: unreadable `/etc/os-release`, a release record that
/// exists but is damaged, a damaged release key file, a state folder that cannot be written.
pub fn notice() -> Result<()> {
    use crate::release::{self, KeyFileFault};
    let os = release::os_release()?;
    let installed = match release::installed() {
        Ok(installed) => installed,
        Err(error) => {
            let missing = error
                .root_cause()
                .downcast_ref::<std::io::Error>()
                .is_some_and(|io| io.kind() == std::io::ErrorKind::NotFound);
            if missing {
                eprintln!("recupdate: no update check: {error:#}");
                return Ok(());
            }
            return Err(error);
        }
    };
    let channel = match release::load_channel() {
        Ok(channel) => channel,
        Err(error) => {
            if error.downcast_ref::<KeyFileFault>().is_some() {
                return Err(error);
            }
            eprintln!("recupdate: could not check for an update: {error:#}");
            return Ok(());
        }
    };
    let report = build_report(&os, &installed, &channel.manifest);
    let mut text = serde_json::to_string(&report)?;
    text.push('\n');
    crate::util::write_atomic(&state_dir().join("available.json"), text.as_bytes(), 0o644)
}
