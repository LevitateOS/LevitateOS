//! The manifest, the keys, the signature and the installed release.

use anyhow::{bail, ensure, Context, Result};
use base64::Engine;
use ring::signature::{UnparsedPublicKey, ED25519};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub const DEFAULT_KEYS_DIR: &str = "/usr/share/levitate/release-keys";
pub const DEFAULT_RELEASE_FILE: &str = "/usr/lib/levitate/release.json";
pub const CHANNEL_FILE_LIMIT: u64 = 64 * 1024;

#[derive(Deserialize, Clone)]
pub struct Archive {
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

/// One category part of the release (`parts-<category>.tar.zst`): the package-cache files the
/// default set's recipes of that category pin. Empty `url` and `sha256` mean not published.
#[derive(Deserialize, Clone)]
pub struct Part {
    pub name: String,
    pub category: String,
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

impl Part {
    pub fn archive(&self) -> Archive {
        Archive {
            url: self.url.clone(),
            sha256: self.sha256.clone(),
            size: self.size,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Manifest {
    pub schema: u32,
    pub os: String,
    pub version: String,
    pub serial: u64,
    #[serde(default)]
    pub date: String,
    pub base: Archive,
    pub boot: Archive,
    pub recipes: Archive,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub parts: Vec<Part>,
}

impl Manifest {
    pub fn download_bytes(&self) -> u64 {
        self.base.size + self.boot.size + self.recipes.size
    }
}

/// A channel file whose signature, schema, OS and serial have been checked.
pub struct Verified {
    pub manifest: Manifest,
    pub bytes: Vec<u8>,
    pub sig: Vec<u8>,
    /// The channel itself was fetched over plain http (a local or test channel), so its
    /// archives may be too.
    pub plain_http: bool,
}

pub struct OsRelease {
    pub id: String,
    pub name: String,
}

pub fn os_release() -> Result<OsRelease> {
    let text = fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
        .context("cannot read /etc/os-release or /usr/lib/os-release")?;
    let field = |key: &str| {
        text.lines().find_map(|line| {
            let value = line.strip_prefix(key)?.strip_prefix('=')?;
            Some(
                value
                    .trim()
                    .trim_matches(|c| c == '"' || c == '\'')
                    .to_owned(),
            )
        })
    };
    let id = field("ID").context("os-release has no ID")?;
    let name = field("NAME").unwrap_or_else(|| id.clone());
    Ok(OsRelease { id, name })
}

pub struct Installed {
    pub os: String,
    pub version: String,
    pub serial: u64,
}

pub fn installed() -> Result<Installed> {
    let path = crate::util::env_path("RECUPDATE_RELEASE_FILE", DEFAULT_RELEASE_FILE);
    let text = fs::read_to_string(&path).with_context(|| {
        format!(
            "cannot read the installed release record {}",
            path.display()
        )
    })?;
    #[derive(Deserialize)]
    struct Record {
        os: String,
        version: String,
        serial: u64,
    }
    let record: Record = serde_json::from_str(&text)
        .with_context(|| format!("{} is not a release manifest", path.display()))?;
    Ok(Installed {
        os: record.os,
        version: record.version,
        serial: record.serial,
    })
}

pub fn channel_url(os_id: &str) -> String {
    std::env::var("RECUPDATE_CHANNEL_URL").ok().filter(|v| !v.is_empty()).unwrap_or_else(|| {
        format!("https://raw.githubusercontent.com/LevitateOS/releases/main/channels/{os_id}/stable.json")
    })
}

fn decode_b64(text: &[u8]) -> Result<Vec<u8>> {
    let text = std::str::from_utf8(text).context("not text")?;
    base64::engine::general_purpose::STANDARD
        .decode(text.trim())
        .context("not valid base64")
}

/// Marks the error of a release key file that exists but cannot be used: a fault of this
/// machine, unlike a missing key folder or channel.
#[derive(Debug)]
pub struct KeyFileFault;

impl std::fmt::Display for KeyFileFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a release key file is damaged")
    }
}

/// A release key: raw 32-byte Ed25519 public key.
type ReleaseKey = UnparsedPublicKey<[u8; 32]>;

fn keys() -> Result<Vec<ReleaseKey>> {
    let dir = crate::util::env_path("RECUPDATE_KEYS_DIR", DEFAULT_KEYS_DIR);
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .with_context(|| format!("cannot read the key folder {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "pub"))
        .collect();
    paths.sort();
    let mut keys = Vec::new();
    for path in &paths {
        let raw = fs::read(path)
            .map_err(anyhow::Error::from)
            .and_then(|bytes| decode_b64(&bytes))
            .with_context(|| format!("key {}", path.display()))
            .context(KeyFileFault)?;
        let array: [u8; 32] = raw.try_into().map_err(|_| {
            anyhow::anyhow!("key {} is not 32 bytes", path.display()).context(KeyFileFault)
        })?;
        keys.push(UnparsedPublicKey::new(&ED25519, array));
    }
    ensure!(
        !keys.is_empty(),
        "no release key (*.pub) in {}; refusing to trust any release",
        dir.display()
    );
    Ok(keys)
}

/// Download the channel file and its `.sig`, verify the signature over the exact bytes, then
/// parse and check schema, OS and serial.
pub fn load_channel() -> Result<Verified> {
    let os = os_release()?;
    let installed = installed()?;
    let keys = keys()?;
    let url = channel_url(&os.id);
    let bytes = crate::net::get_bounded(&url, CHANNEL_FILE_LIMIT)?;
    let sig_url = format!("{url}.sig");
    let sig_text = crate::net::get_bounded(&sig_url, CHANNEL_FILE_LIMIT)?;
    let manifest = verify(&keys, &os, &bytes, &sig_text, &url, &sig_url)?;
    ensure!(
        manifest.serial >= installed.serial,
        "the channel's serial {} is lower than the installed serial {}; refusing it",
        manifest.serial,
        installed.serial
    );
    Ok(Verified {
        plain_http: url.starts_with("http://"),
        manifest,
        bytes,
        sig: sig_text,
    })
}

/// The base address of one release's own assets: `RECUPDATE_RELEASE_BASE_URL` (a local test
/// channel), else the GitHub release `<os>-<serial>` of LevitateOS/releases. Always ends in `/`.
pub fn release_base_url(os_id: &str, serial: u64) -> String {
    let mut base = std::env::var("RECUPDATE_RELEASE_BASE_URL")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| {
            format!("https://github.com/LevitateOS/releases/releases/download/{os_id}-{serial}/")
        });
    if !base.ends_with('/') {
        base.push('/');
    }
    base
}

/// The manifest of the release this medium was built from (never the channel pointer, which
/// may already name a newer one): `release.json` and its `.sig` from the release's own assets,
/// or from the folder `from`, verified against the keys; `os` must be this system's and
/// `serial` exactly the medium's.
pub fn load_medium_release(from: Option<&Path>) -> Result<Verified> {
    let os = os_release()?;
    let installed = installed()?;
    ensure!(
        installed.os == os.id,
        "the installed release record is for {}, this system is {}",
        installed.os,
        os.id
    );
    let keys = keys()?;
    let (bytes, sig_text, what, sig_what, plain_http) = match from {
        Some(dir) => {
            let read = |name: &str| -> Result<(Vec<u8>, String)> {
                let path = dir.join(name);
                let meta = fs::metadata(&path)
                    .with_context(|| format!("cannot read {} (the folder must hold {name})", path.display()))?;
                ensure!(
                    meta.is_file() && meta.len() <= CHANNEL_FILE_LIMIT,
                    "{} is not a file of at most {CHANNEL_FILE_LIMIT} bytes",
                    path.display()
                );
                Ok((fs::read(&path)?, path.display().to_string()))
            };
            let (bytes, what) = read("release.json")?;
            let (sig, sig_what) = read("release.json.sig")?;
            (bytes, sig, what, sig_what, false)
        }
        None => {
            let base = release_base_url(&os.id, installed.serial);
            let url = format!("{base}release.json");
            let sig_url = format!("{url}.sig");
            let bytes = crate::net::get_bounded(&url, CHANNEL_FILE_LIMIT)?;
            let sig = crate::net::get_bounded(&sig_url, CHANNEL_FILE_LIMIT)?;
            (bytes, sig, url, sig_url, base.starts_with("http://"))
        }
    };
    let manifest = verify(&keys, &os, &bytes, &sig_text, &what, &sig_what)?;
    ensure!(
        manifest.serial == installed.serial,
        "{what} is serial {}, this medium was built from serial {}; refusing it",
        manifest.serial,
        installed.serial
    );
    Ok(Verified {
        plain_http,
        manifest,
        bytes,
        sig: sig_text,
    })
}

/// Check a manifest's signature over its exact bytes against the keys, then parse it and check
/// its schema and OS. `what` and `sig_what` name the two files in messages.
fn verify(
    keys: &[ReleaseKey],
    os: &OsRelease,
    bytes: &[u8],
    sig_text: &[u8],
    what: &str,
    sig_what: &str,
) -> Result<Manifest> {
    let sig = decode_b64(sig_text).with_context(|| format!("signature {sig_what}"))?;
    let sig_array: [u8; 64] = sig
        .try_into()
        .map_err(|_| anyhow::anyhow!("signature {sig_what} is not 64 bytes"))?;
    if !keys.iter().any(|key| key.verify(bytes, &sig_array).is_ok()) {
        bail!("the signature of {what} matches none of the release keys; refusing it");
    }
    let manifest: Manifest = serde_json::from_slice(bytes)
        .with_context(|| format!("{what} is not a release manifest"))?;
    ensure!(
        manifest.schema == 1,
        "{what} has schema {}, this recupdate understands schema 1",
        manifest.schema
    );
    ensure!(
        manifest.os == os.id,
        "the release is for {}, this system is {}",
        manifest.os,
        os.id
    );
    Ok(manifest)
}

/// The manifest of a release `fetch` left in `dir`, its signature checked again against the
/// keys (an interrupted `apply` resumes from such a folder). No serial check: by then Recipe
/// may already have written this release as the installed one.
pub fn verify_fetched(dir: &Path) -> Result<Manifest> {
    let os = os_release()?;
    let keys = keys()?;
    let path = dir.join("release.json");
    let sig_path = dir.join("release.json.sig");
    let bytes = fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
    let sig_text =
        fs::read(&sig_path).with_context(|| format!("reading {}", sig_path.display()))?;
    verify(
        &keys,
        &os,
        &bytes,
        &sig_text,
        &path.display().to_string(),
        &sig_path.display().to_string(),
    )
}
