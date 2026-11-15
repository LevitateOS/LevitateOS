//! What `recupdate apply` reads and writes outside Recipe: whether this is a mutable
//! installation, and the boot step on its EFI system partition.
//!
//! A mutable installation is set up by `recsetup --boot mutable`, which runs the live medium's
//! `recab init --mode mutable`: systemd-boot on the ESP (mounted at `/boot`), one entry
//! `loader/entries/<ID>.conf` loading `/<ID>/vmlinuz` and `/<ID>/initramfs.img` with
//! `options root=PARTUUID=<root> ro rootwait panic=10 <extra>`, `loader.conf`'s `default`
//! and the firmware's `LoaderEntryDefault` naming it. `<ID>` is the os-release `ID`. Nothing
//! A/B is put into the system: no recab, no `/var/lib/recab/installation.json`, no
//! `/usr/lib/recab/immutable`, and no `recab.immutable` on the kernel command line.
//!
//! An update adds an entry in exactly that format, `loader/entries/<ID>-<serial>.conf` loading
//! `/<ID>-<serial>/vmlinuz` and `/<ID>-<serial>/initramfs.img`, with the kernel options of the
//! entry the system booted with, copied unchanged, and the title `<NAME> <version>`. It
//! becomes the default; the booted entry stays as the second; this system's other entries
//! (`<ID>.conf`, `<ID>-<serial>.conf`) and their directories are removed. The booted entry's
//! file and the files it loads are never written or removed.

use anyhow::{anyhow, bail, ensure, Context, Result};
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use crate::release::Manifest;
use recinit::installed::{IMMUTABLE_CONFIG, IMMUTABLE_OPTION};
use recpart::policy::{ESP_MOUNT, STATE_MOUNT};

/// systemd-boot's loader configuration and entries, relative to the ESP (as recab writes them).
const LOADER_CONF: &str = "loader/loader.conf";
const ENTRIES_DIR: &str = "loader/entries";
/// The kernel and initramfs file names in an entry's directory, and in `boot.tar.zst`.
const KERNEL_FILE: &str = "vmlinuz";
const INITRD_FILE: &str = "initramfs.img";
/// Where the kernel and initramfs of `boot.tar.zst` are unpacked first.
const BOOT_FILES_DIR: &str = "/run/recupdate/boot-files";
const CMDLINE: &str = "/proc/cmdline";

/// The firmware variables, and systemd-boot's vendor GUID that names its own.
const EFI_DIR: &str = "/sys/firmware/efi";
const EFIVARS: &str = "/sys/firmware/efi/efivars";
const LOADER_GUID: &str = "4a67b082-0a4c-41cf-b6c7-440b29bb8c4f";
const VAR_DEFAULT: &str = "LoaderEntryDefault";
const VAR_ONESHOT: &str = "LoaderEntryOneShot";
const VAR_SELECTED: &str = "LoaderEntrySelected";
/// Non-volatile, boot-service and runtime access (as bootctl and recab write them).
const VAR_ATTRIBUTES: u32 = 0x7;
const EFIVARFS_MAGIC: i64 = 0xde5e81e4;
const FS_IMMUTABLE_FL: libc::c_int = 0x10;

/// Refuse an A/B installation, naming `recab update`. The facts are the ones recab and its
/// initramfs use: the installation record on `/var`, the slot marker in each root, and the
/// kernel option of an A/B entry.
pub fn require_mutable() -> Result<()> {
    let cmdline = fs::read_to_string(CMDLINE).with_context(|| format!("reading {CMDLINE}"))?;
    let record = PathBuf::from(format!("{STATE_MOUNT}/lib/recab/installation.json"));
    let marker = Path::new("/").join(IMMUTABLE_CONFIG);
    let fact = if cmdline.split_whitespace().any(|w| w == IMMUTABLE_OPTION) {
        Some(format!(
            "the kernel command line carries {IMMUTABLE_OPTION}"
        ))
    } else if exists(&record)? {
        Some(format!("{} exists", record.display()))
    } else if exists(&marker)? {
        Some(format!("{} exists", marker.display()))
    } else {
        None
    };
    if let Some(fact) = fact {
        bail!(
            "this is an A/B installation ({fact}); update it with `recab update`, which installs \
the release into the other root and starts it at the next boot"
        );
    }
    Ok(())
}

fn exists(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
        Err(e) => Err(anyhow!(e).context(format!("checking {}", path.display()))),
    }
}

/// One boot entry, as lines.
struct Entry {
    lines: Vec<String>,
}

/// A line's key and value, or `None` for a blank line or a comment (recab's grammar).
fn key_value(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    Some(
        line.split_once(char::is_whitespace)
            .map_or((line, ""), |(key, value)| (key, value.trim())),
    )
}

impl Entry {
    fn parse(text: &str) -> Self {
        Self {
            lines: text.lines().map(str::to_string).collect(),
        }
    }

    fn values<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        self.lines
            .iter()
            .filter_map(move |line| key_value(line).filter(|(k, _)| *k == key).map(|(_, v)| v))
    }

    /// The kernel options, all `options` lines joined (as systemd-boot does).
    fn options(&self) -> String {
        self.values("options").collect::<Vec<_>>().join(" ")
    }

    fn title(&self) -> Option<&str> {
        self.values("title").next()
    }

    /// The files the entry loads from the ESP, relative to it.
    fn boot_files(&self) -> Vec<String> {
        ["linux", "efi", "initrd", "devicetree"]
            .iter()
            .flat_map(|key| self.values(key))
            .filter_map(|v| v.split_whitespace().next())
            .map(|v| v.trim_start_matches('/').to_string())
            .collect()
    }

    /// The top-level ESP directories the entry loads files from, lower-cased (FAT).
    fn dirs(&self) -> Vec<String> {
        self.boot_files()
            .iter()
            .filter_map(|f| f.split_once('/').map(|(d, _)| d.to_ascii_lowercase()))
            .collect()
    }
}

/// The last `root=` of a command line, as the kernel uses it.
fn root_word(cmdline: &str) -> Option<&str> {
    cmdline
        .split_whitespace()
        .filter_map(|w| w.strip_prefix("root="))
        .next_back()
}

/// loader.conf with `entry` as its only `default`, other lines kept (as recab writes it).
fn loader_with_default(text: Option<&str>, entry: &str) -> String {
    let mut lines = vec![format!("default {entry}")];
    match text {
        Some(text) => lines.extend(
            text.lines()
                .filter(|line| !key_value(line).is_some_and(|(key, _)| key == "default"))
                .map(str::to_string),
        ),
        None => lines.extend(["timeout 5".to_string(), "editor 0".to_string()]),
    }
    lines.join("\n") + "\n"
}

// ---- systemd-boot's firmware variables ----

fn var_path(name: &str) -> PathBuf {
    Path::new(EFIVARS).join(format!("{name}-{LOADER_GUID}"))
}

/// A variable's entry id: 4 bytes of attributes, then NUL-terminated UTF-16LE. `None` when
/// unset or empty.
fn get_var(name: &str) -> Result<Option<String>> {
    let path = var_path(name);
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(anyhow!(e).context(format!("reading {}", path.display()))),
    };
    let units: Vec<u16> = bytes
        .get(4..)
        .with_context(|| format!("{} is shorter than its attributes", path.display()))?
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .take_while(|unit| *unit != 0)
        .collect();
    let value =
        String::from_utf16(&units).map_err(|_| anyhow!("{} is not UTF-16", path.display()))?;
    Ok((!value.is_empty()).then_some(value))
}

/// Set a variable to `value`, or remove it with `None`.
fn set_var(name: &str, value: Option<&str>) -> Result<()> {
    let path = var_path(name);
    let context = || format!("writing the EFI variable {}", path.display());
    if path.exists() {
        clear_immutable(&path).with_context(context)?;
    }
    let Some(value) = value else {
        return match fs::remove_file(&path) {
            Err(e) if e.kind() != ErrorKind::NotFound => Err(anyhow!(e)).with_context(context),
            _ => Ok(()),
        };
    };
    let mut bytes = VAR_ATTRIBUTES.to_le_bytes().to_vec();
    for unit in value.encode_utf16().chain([0]) {
        bytes.extend(unit.to_le_bytes());
    }
    // efivarfs replaces a variable with each complete write; it is never truncated.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(&path)
        .with_context(context)?;
    let written = file.write(&bytes).with_context(context)?;
    ensure!(
        written == bytes.len(),
        "{}: short write ({written} of {} bytes)",
        context(),
        bytes.len()
    );
    Ok(())
}

/// efivarfs marks variables immutable: clear that before changing one.
fn clear_immutable(path: &Path) -> std::io::Result<()> {
    use std::os::fd::AsRawFd;
    let file = fs::File::open(path)?;
    let mut flags: libc::c_int = 0;
    // SAFETY: FS_IOC_GETFLAGS/SETFLAGS read and write one int on an open fd.
    unsafe {
        if libc::ioctl(file.as_raw_fd(), libc::FS_IOC_GETFLAGS, &mut flags) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        if flags & FS_IMMUTABLE_FL != 0 {
            flags &= !FS_IMMUTABLE_FL;
            if libc::ioctl(file.as_raw_fd(), libc::FS_IOC_SETFLAGS, &flags) != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
    }
    Ok(())
}

fn is_efivarfs(dir: &Path) -> std::io::Result<bool> {
    use std::os::unix::ffi::OsStrExt;
    let path = std::ffi::CString::new(dir.as_os_str().as_bytes())?;
    // SAFETY: statfs writes one statfs struct this function owns.
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(path.as_ptr(), &mut stat) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(stat.f_type as i64 == EFIVARFS_MAGIC)
}

/// The mount exactly at `point` (the last one there is the visible one).
fn mount_at(table: &[target_root::Mount], point: &Path) -> Option<target_root::Mount> {
    table.iter().rev().find(|m| m.mount_point == point).cloned()
}

fn is_rw(mount: &target_root::Mount) -> bool {
    mount.options.split(',').any(|o| o == "rw")
}

// ---- the boot step ----

/// The ESP and the entry this system booted with, checked before anything changes.
pub struct Boot {
    esp: PathBuf,
    /// The os-release `ID`, lower-cased: the entries' name prefix.
    prefix: String,
    /// The os-release `NAME`, for the new entry's title.
    name: String,
    /// The booted entry's file name in `loader/entries` (as it is on the ESP).
    booted_id: String,
    booted: Entry,
}

/// What `install` did.
pub struct Installed {
    /// The new default entry's file name.
    pub entry: String,
    /// The title of the entry that stays as the second.
    pub previous: String,
}

/// Every check the boot step needs, and nothing changed: the ESP mounted read-write at
/// `/boot`, the firmware variables writable (else the new entry could not be made the default
/// over the `LoaderEntryDefault` the installer set), and the booted entry found.
pub fn prepare() -> Result<Boot> {
    let os = crate::release::os_release()?;
    let prefix = os.id.to_ascii_lowercase();
    ensure!(
        !prefix.is_empty()
            && prefix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-'),
        "the os-release ID '{}' cannot name boot entries",
        os.id
    );
    let table = target_root::mount_table().context("reading the mount table")?;

    let esp = PathBuf::from(ESP_MOUNT);
    let mount = mount_at(&table, &esp).ok_or_else(|| {
        anyhow!(
            "the EFI system partition is not mounted at {ESP_MOUNT}; mount it there (fstab names \
it) and run `recupdate apply` again"
        )
    })?;
    ensure!(
        mount.fstype == "vfat",
        "{ESP_MOUNT} holds a {} filesystem, not the EFI system partition (vfat)",
        mount.fstype
    );
    ensure!(
        is_rw(&mount),
        "the EFI system partition is mounted read-only at {ESP_MOUNT}; remount it read-write \
(`mount -o remount,rw {ESP_MOUNT}`) and run `recupdate apply` again"
    );

    let vars = Path::new(EFIVARS);
    ensure!(
        Path::new(EFI_DIR).is_dir(),
        "{EFI_DIR} is missing: this system did not start through UEFI, so systemd-boot's default \
entry cannot be set"
    );
    let vars_mount = mount_at(&table, vars);
    let usable = vars_mount
        .as_ref()
        .is_some_and(|m| m.fstype == "efivarfs" && is_rw(m))
        && is_efivarfs(vars).unwrap_or(false);
    ensure!(
        usable,
        "the firmware's boot variables are not writable (efivarfs is not mounted read-write at \
{EFIVARS}), so the new kernel could not be made the default boot entry; mount it \
(`mount -t efivarfs efivarfs {EFIVARS}`, or `mount -o remount,rw {EFIVARS}`) and run \
`recupdate apply` again"
    );

    let (booted_id, booted) = find_booted(&esp, &prefix)?;
    Ok(Boot {
        esp,
        prefix,
        name: os.name,
        booted_id,
        booted,
    })
}

/// The `.conf` files in `loader/entries`.
fn entry_files(esp: &Path) -> Result<Vec<String>> {
    let dir = esp.join(ENTRIES_DIR);
    let mut names = Vec::new();
    for entry in fs::read_dir(&dir).with_context(|| format!("reading {}", dir.display()))? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if name.to_ascii_lowercase().ends_with(".conf") {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}

/// This system's own entry names, as stems: `<prefix>` (the installer's) and
/// `<prefix>-<serial>` (an update's).
fn is_own_stem(prefix: &str, stem: &str) -> bool {
    let stem = stem.to_ascii_lowercase();
    stem == prefix
        || stem
            .strip_prefix(prefix)
            .and_then(|rest| rest.strip_prefix('-'))
            .is_some_and(|serial| !serial.is_empty() && serial.bytes().all(|b| b.is_ascii_digit()))
}

fn stem(name: &str) -> &str {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".conf") {
        &name[..name.len() - 5]
    } else {
        name
    }
}

fn read_entry(esp: &Path, name: &str) -> Result<Entry> {
    let path = esp.join(ENTRIES_DIR).join(name);
    let text = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    Ok(Entry::parse(&text))
}

/// The entry this system started from: systemd-boot's `LoaderEntrySelected`, else the one
/// entry of this system whose `root=` is the running kernel's. Its `root=` must be the
/// running kernel's.
fn find_booted(esp: &Path, prefix: &str) -> Result<(String, Entry)> {
    let cmdline = fs::read_to_string(CMDLINE).with_context(|| format!("reading {CMDLINE}"))?;
    let root = root_word(&cmdline)
        .ok_or_else(|| anyhow!("the kernel command line has no root="))?
        .to_string();
    let names = entry_files(esp)?;
    let id = match get_var(VAR_SELECTED)? {
        Some(selected) => {
            let wanted = stem(&selected).to_ascii_lowercase();
            names
                .iter()
                .find(|n| stem(n).to_ascii_lowercase() == wanted)
                .cloned()
                .ok_or_else(|| {
                    anyhow!(
                        "systemd-boot started entry '{selected}', which is not in {}; \
recupdate keeps the entry the system started from as the fallback, so it needs that entry",
                        esp.join(ENTRIES_DIR).display()
                    )
                })?
        }
        None => {
            let mut matching = Vec::new();
            for name in names.iter().filter(|n| is_own_stem(prefix, stem(n))) {
                if root_word(&read_entry(esp, name)?.options()) == Some(root.as_str()) {
                    matching.push(name.clone());
                }
            }
            match matching.as_slice() {
                [one] => one.clone(),
                [] => bail!(
                    "no boot entry in {} boots root={root}, the root this system runs from",
                    esp.join(ENTRIES_DIR).display()
                ),
                more => bail!(
                    "systemd-boot did not say which entry it started ({VAR_SELECTED} is unset), \
and {} entries boot root={root} ({}); start the system once through systemd-boot and run \
`recupdate apply` again",
                    more.len(),
                    more.join(", ")
                ),
            }
        }
    };
    let entry = read_entry(esp, &id)?;
    ensure!(
        root_word(&entry.options()) == Some(root.as_str()),
        "the booted entry {id} names root={}, but the running kernel's is root={root}",
        root_word(&entry.options()).unwrap_or("nothing")
    );
    ensure!(
        entry.values("linux").next().is_some(),
        "the booted entry {id} loads no kernel (linux line); recupdate copies its format"
    );
    Ok((id, entry))
}

impl Boot {
    pub fn booted_title(&self) -> String {
        self.booted
            .title()
            .map(str::to_string)
            .unwrap_or_else(|| stem(&self.booted_id).to_string())
    }

    /// The boot step: see the module documentation. A failure before the default is set
    /// removes what this step wrote and leaves the old entry the default.
    pub fn install(&self, folder: &Path, manifest: &Manifest) -> Result<Installed> {
        let dir_name = format!("{}-{}", self.prefix, manifest.serial);
        let entry_name = format!("{dir_name}.conf");
        let previous = self.booted_title();
        if stem(&self.booted_id).eq_ignore_ascii_case(&dir_name) {
            // The system already runs this release's kernel from this entry.
            return Ok(Installed {
                entry: self.booted_id.clone(),
                previous,
            });
        }
        if self.booted.dirs().contains(&dir_name.to_ascii_lowercase()) {
            bail!(
                "the booted entry {} loads files from /{dir_name}, which this update would \
replace; refusing to change the files the system runs from",
                self.booted_id
            );
        }

        let files = unpack(folder, manifest)?;
        let dir = self.esp.join(&dir_name);
        let entry_path = self.esp.join(ENTRIES_DIR).join(&entry_name);
        let written = (|| -> Result<()> {
            fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
            for name in [INITRD_FILE, KERNEL_FILE] {
                target_root::copy_durable(&files.dir.join(name), &dir.join(name))?;
            }
            target_root::sync_dir(&dir)?;
            let text = format!(
                "title {} {}\nlinux /{dir_name}/{KERNEL_FILE}\ninitrd /{dir_name}/{INITRD_FILE}\noptions {}\n",
                self.name,
                manifest.version,
                self.booted.options()
            );
            target_root::write_durable(&entry_path, text.as_bytes(), Some(0o644))
                .with_context(|| format!("writing {}", entry_path.display()))?;
            target_root::sync_dir(&self.esp.join(ENTRIES_DIR))?;
            self.make_default(&entry_name)
        })();
        drop(files);
        if let Err(error) = written {
            let _ = fs::remove_file(&entry_path);
            let _ = fs::remove_dir_all(&dir);
            return Err(error.context(format!(
                "the new kernel was not installed; {} stays the default boot entry",
                self.booted_id
            )));
        }
        self.prune(&entry_name);
        Ok(Installed {
            entry: entry_name,
            previous,
        })
    }

    /// Make `id` the default: loader.conf, then `LoaderEntryDefault` (which wins over it). If
    /// the variable cannot be written, loader.conf is put back.
    fn make_default(&self, id: &str) -> Result<()> {
        let conf = self.esp.join(LOADER_CONF);
        let before = target_root::read_optional(&conf)?;
        target_root::write_durable(
            &conf,
            loader_with_default(before.as_deref(), id).as_bytes(),
            None,
        )
        .with_context(|| format!("writing {}", conf.display()))?;
        if let Err(error) = set_var(VAR_DEFAULT, Some(id)) {
            let _ = match &before {
                Some(text) => target_root::write_durable(&conf, text.as_bytes(), None),
                None => fs::remove_file(&conf),
            };
            return Err(error.context("setting systemd-boot's default entry"));
        }
        // A one-time pick left behind would win over the default at the next boot.
        if get_var(VAR_ONESHOT).ok().flatten().is_some() {
            if let Err(error) = set_var(VAR_ONESHOT, None) {
                eprintln!("recupdate: warning: could not clear {VAR_ONESHOT}: {error:#}");
            }
        }
        Ok(())
    }

    /// Remove this system's entries other than `new` and the booted one, then this system's
    /// entry directories no remaining entry loads from. Failures are warnings: the update is in
    /// place.
    fn prune(&self, new: &str) {
        let keep = |name: &str| {
            name.eq_ignore_ascii_case(new) || name.eq_ignore_ascii_case(&self.booted_id)
        };
        let warn = |what: String, error: anyhow::Error| {
            eprintln!("recupdate: warning: could not remove {what}: {error:#}");
        };
        let names = match entry_files(&self.esp) {
            Ok(n) => n,
            Err(error) => return warn("older boot entries".into(), error),
        };
        for name in names
            .iter()
            .filter(|n| is_own_stem(&self.prefix, stem(n)) && !keep(n))
        {
            let path = self.esp.join(ENTRIES_DIR).join(name);
            if let Err(error) = fs::remove_file(&path) {
                warn(path.display().to_string(), error.into());
            }
        }
        let _ = target_root::sync_dir(&self.esp.join(ENTRIES_DIR));

        // Every directory any remaining entry loads from stays (other systems' included).
        let mut used: Vec<String> = self.booted.dirs();
        match entry_files(&self.esp) {
            Ok(names) => {
                for name in names {
                    match read_entry(&self.esp, &name) {
                        Ok(entry) => used.extend(entry.dirs()),
                        Err(error) => return warn("older kernel directories".into(), error),
                    }
                }
            }
            Err(error) => return warn("older kernel directories".into(), error),
        }
        let Ok(dirs) = fs::read_dir(&self.esp) else {
            return;
        };
        for dir in dirs.flatten() {
            let name = dir.file_name().to_string_lossy().into_owned();
            let path = dir.path();
            if is_own_stem(&self.prefix, &name)
                && path.is_dir()
                && !used.contains(&name.to_ascii_lowercase())
            {
                if let Err(error) = fs::remove_dir_all(&path) {
                    warn(path.display().to_string(), error.into());
                }
            }
        }
        let _ = target_root::sync_dir(&self.esp);
    }
}

/// The unpacked kernel and initramfs; the folder is removed when dropped.
struct BootFiles {
    dir: PathBuf,
}

impl Drop for BootFiles {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

/// Check `boot.tar.zst` against the manifest's SHA-256 and unpack it. It holds exactly two
/// regular files at its top level, `vmlinuz` and `initramfs.img` (a `./` prefix and a `.`
/// directory entry are allowed); anything else is refused.
fn unpack(folder: &Path, manifest: &Manifest) -> Result<BootFiles> {
    let archive = folder.join("boot.tar.zst");
    let want = manifest.boot.sha256.to_ascii_lowercase();
    let got = crate::net::sha256_file(&archive)?;
    ensure!(
        got == want,
        "{} has SHA-256 {got}, but release.json says {want}",
        archive.display()
    );

    let files = BootFiles {
        dir: PathBuf::from(BOOT_FILES_DIR),
    };
    match fs::remove_dir_all(&files.dir) {
        Err(e) if e.kind() != ErrorKind::NotFound => {
            return Err(anyhow!(e).context(format!("removing {}", files.dir.display())));
        }
        _ => {}
    }
    fs::create_dir_all(&files.dir).with_context(|| format!("creating {}", files.dir.display()))?;
    let decoder = zstd::stream::read::Decoder::new(
        fs::File::open(&archive).with_context(|| format!("opening {}", archive.display()))?,
    )
    .with_context(|| format!("reading {}", archive.display()))?;
    let mut tar = tar::Archive::new(decoder);
    let mut found: Vec<String> = Vec::new();
    for entry in tar
        .entries()
        .with_context(|| format!("reading {}", archive.display()))?
    {
        let mut entry = entry.with_context(|| format!("reading {}", archive.display()))?;
        let path = entry.path()?.to_string_lossy().into_owned();
        let name = path.trim_start_matches("./").trim_end_matches('/');
        let kind = entry.header().entry_type();
        if (name.is_empty() || name == ".") && kind.is_dir() {
            continue;
        }
        if !(kind.is_file() && [KERNEL_FILE, INITRD_FILE].contains(&name))
            || found.iter().any(|f| f == name)
        {
            bail!(
                "{} holds '{path}'; it must hold exactly the two files {KERNEL_FILE} and \
{INITRD_FILE}",
                archive.display()
            );
        }
        let dest = files.dir.join(name);
        let mut out =
            fs::File::create(&dest).with_context(|| format!("creating {}", dest.display()))?;
        std::io::copy(&mut entry, &mut out)
            .with_context(|| format!("unpacking {name} from {}", archive.display()))?;
        out.sync_all()?;
        found.push(name.to_string());
    }
    for name in [KERNEL_FILE, INITRD_FILE] {
        ensure!(
            found.iter().any(|f| f == name),
            "{} has no {name}; it must hold exactly the two files {KERNEL_FILE} and {INITRD_FILE}",
            archive.display()
        );
    }
    Ok(files)
}
