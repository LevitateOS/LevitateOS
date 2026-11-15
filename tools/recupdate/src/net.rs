//! HTTP: bounded downloads with ureq (the client Recipe uses).

use anyhow::{bail, ensure, Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::time::{Duration, Instant};

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(30))
        .timeout_read(Duration::from_secs(60))
        .user_agent("recupdate")
        .build()
}

fn describe(error: ureq::Error, url: &str) -> anyhow::Error {
    match error {
        ureq::Error::Status(code, _) => anyhow::anyhow!("{url} answered HTTP status {code}"),
        other => anyhow::anyhow!("cannot reach {url}: {other}"),
    }
}

/// Download a small file into memory; refuse more than `limit` bytes.
pub fn get_bounded(url: &str, limit: u64) -> Result<Vec<u8>> {
    let response = agent().get(url).call().map_err(|e| describe(e, url))?;
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .with_context(|| format!("reading {url}"))?;
    ensure!(
        bytes.len() as u64 <= limit,
        "{url} is larger than the {limit} byte limit"
    );
    Ok(bytes)
}

/// SHA-256 of a file, as lowercase hex.
pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(crate::util::hex(&hasher.finalize()))
}

/// Download `url` to `part`, resuming with a Range request when `part` exists. The file ends at
/// exactly `size` bytes. `progress(done)` is called as bytes arrive.
pub fn download(url: &str, part: &Path, size: u64, progress: &mut dyn FnMut(u64)) -> Result<()> {
    let mut have = fs::metadata(part).map(|m| m.len()).unwrap_or(0);
    if have > size {
        fs::remove_file(part)?;
        have = 0;
    }
    if have == size {
        progress(size);
        return Ok(());
    }
    let agent = agent();
    let mut request = agent.get(url);
    if have > 0 {
        request = request.set("Range", &format!("bytes={have}-"));
    }
    let response = match request.call() {
        Ok(r) => r,
        Err(ureq::Error::Status(416, _)) if have > 0 => {
            // The partial file does not fit what the server has; start over.
            fs::remove_file(part)?;
            return download(url, part, size, progress);
        }
        Err(e) => return Err(describe(e, url)),
    };
    let resumed = response.status() == 206;
    let mut file = if resumed {
        fs::OpenOptions::new().append(true).open(part)?
    } else {
        have = 0;
        fs::File::create(part)?
    };
    let mut reader = response.into_reader().take(size - have + 1);
    let mut done = have;
    let mut buf = vec![0u8; 256 * 1024];
    let mut last = Instant::now();
    loop {
        let n = reader
            .read(&mut buf)
            .with_context(|| format!("reading {url}"))?;
        if n == 0 {
            break;
        }
        done += n as u64;
        if done > size {
            drop(file);
            let _ = fs::remove_file(part);
            bail!("{url} sent more than the {size} bytes the manifest names");
        }
        file.write_all(&buf[..n])?;
        if last.elapsed() >= Duration::from_millis(250) {
            progress(done);
            last = Instant::now();
        }
    }
    file.flush()?;
    progress(done);
    ensure!(
        done == size,
        "{url} ended after {done} of {size} bytes; run fetch again to resume"
    );
    Ok(())
}
