//! A variant's distro profile: the few facts that really differ underneath the
//! variants. Each variant declares them once (`[profile]` in its `distro.toml`);
//! the builder writes them into the image's `/etc/os-release` as the `REC_*`
//! fields, and the tools on the target read them back from there instead of
//! probing files or matching distro names.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// An enum of profile values, each with one snake_case spelling shared by
/// serde, `as_str`, `Display` and `FromStr`.
macro_rules! fact {
    ($(#[$meta:meta])* $name:ident { $($(#[$vmeta:meta])* $variant:ident => $text:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[cfg_attr(feature = "contracts", derive(schemars::JsonSchema, ts_rs::TS))]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($(#[$vmeta])* $variant),+
        }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            pub fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $text),+
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, String> {
                Self::ALL
                    .iter()
                    .copied()
                    .find(|v| v.as_str() == value)
                    .ok_or_else(|| {
                        let known: Vec<&str> = Self::ALL.iter().map(|v| v.as_str()).collect();
                        format!("'{value}' is not one of {}", known.join(", "))
                    })
            }
        }
    };
}

fact! {
    /// The package format the rootfs is assembled from.
    PackageBackend { Rpm => "rpm", Apk => "apk" }
}

fact! {
    /// PID 1 and its service manager.
    InitSystem { Systemd => "systemd", Openrc => "openrc" }
}

fact! {
    /// The C library every program on the image links against.
    Libc { Glibc => "glibc", Musl => "musl" }
}

fact! {
    /// What lets the admin group become root.
    PrivilegeTool { Sudo => "sudo", Doas => "doas" }
}

fact! {
    /// The desktop the variant is meant to carry.
    Desktop { Plasma => "plasma", Lxqt => "lxqt" }
}

fact! {
    /// Which kernel line the variant follows.
    KernelLine { Lts => "lts", Latest => "latest" }
}

fact! {
    /// Which system an image is: the base system, or the base system with
    /// the variant's desktop. Unlike the profile, it is a fact of the image,
    /// not of the variant: a variant builds both.
    Flavour { Base => "base", Desktop => "desktop" }
}

impl Desktop {
    /// The display manager's session for this desktop: `<session>.desktop`
    /// under `/usr/share/wayland-sessions` or `/usr/share/xsessions`.
    pub fn session(self) -> &'static str {
        match self {
            Desktop::Plasma => "plasma",
            Desktop::Lxqt => "lxqt",
        }
    }
}

impl Flavour {
    /// The image's `REC_FLAVOUR` line of `/etc/os-release`, ending in a newline.
    pub fn os_release_line(self) -> String {
        format!("{REC_FLAVOUR}={}\n", self.as_str())
    }

    /// The flavour in `os_release` (the text of an `/etc/os-release`).
    pub fn from_os_release(os_release: &str) -> Result<Self, ProfileError> {
        os_release_fact(os_release, REC_FLAVOUR)
    }
}

impl Libc {
    /// The `bun build --compile` target for the image.
    pub fn bun_target(self) -> &'static str {
        match self {
            Libc::Glibc => "bun-linux-x64",
            Libc::Musl => "bun-linux-x64-musl",
        }
    }

    /// Shared libraries a bun binary for this libc needs on the image
    /// (bun's musl builds link libstdc++).
    pub fn bun_libs(self) -> &'static [&'static str] {
        match self {
            Libc::Glibc => &[],
            Libc::Musl => &["libstdc++.so.6"],
        }
    }
}

/// The group whose members become root through the privilege tool.
pub const ADMIN_GROUP: &str = "wheel";

/// The groups a desktop session's user is in besides [`ADMIN_GROUP`], for the
/// devices and services it uses (graphics, input, sound, networks, printers,
/// scanners, cameras); a user joins those the image has. The live desktop's
/// user and every user recsetup makes on a desktop system get them.
pub const DESKTOP_SESSION_GROUPS: &[&str] =
    &["video", "input", "audio", "netdev", "plugdev", "lpadmin", "scanner", "camera"];

impl PrivilegeTool {
    /// The file, relative to the root, that the system recipe writes the admin
    /// rule into.
    pub fn admin_rule_file(self) -> &'static str {
        match self {
            PrivilegeTool::Sudo => "etc/sudoers.d/10-wheel",
            PrivilegeTool::Doas => "etc/doas.conf",
        }
    }

    /// The line of [`admin_rule_file`](Self::admin_rule_file) that lets
    /// [`ADMIN_GROUP`] become root with its own password.
    pub fn admin_rule(self) -> String {
        match self {
            PrivilegeTool::Sudo => format!("%{ADMIN_GROUP} ALL=(ALL:ALL) ALL"),
            PrivilegeTool::Doas => format!("permit persist :{ADMIN_GROUP}"),
        }
    }

    /// The tool's main configuration file, as an absolute path on the target.
    pub fn config_file(self) -> &'static str {
        match self {
            PrivilegeTool::Sudo => "/etc/sudoers",
            PrivilegeTool::Doas => "/etc/doas.conf",
        }
    }
}

/// The `/etc/os-release` field of each fact.
pub const REC_PACKAGES: &str = "REC_PACKAGES";
pub const REC_INIT: &str = "REC_INIT";
pub const REC_LIBC: &str = "REC_LIBC";
pub const REC_PRIVILEGE: &str = "REC_PRIVILEGE";
pub const REC_DESKTOP: &str = "REC_DESKTOP";
pub const REC_KERNEL_LINE: &str = "REC_KERNEL_LINE";
/// The image's [`Flavour`]; written beside the profile's fields.
pub const REC_FLAVOUR: &str = "REC_FLAVOUR";

/// A variant's base facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub packages: PackageBackend,
    pub init: InitSystem,
    pub libc: Libc,
    pub privilege: PrivilegeTool,
    pub desktop: Desktop,
    pub kernel_line: KernelLine,
}

impl Profile {
    /// The `REC_*` lines of `/etc/os-release`, one per fact, each ending in a newline.
    pub fn os_release_lines(&self) -> String {
        [
            (REC_PACKAGES, self.packages.as_str()),
            (REC_INIT, self.init.as_str()),
            (REC_LIBC, self.libc.as_str()),
            (REC_PRIVILEGE, self.privilege.as_str()),
            (REC_DESKTOP, self.desktop.as_str()),
            (REC_KERNEL_LINE, self.kernel_line.as_str()),
        ]
        .iter()
        .map(|(key, value)| format!("{key}={value}\n"))
        .collect()
    }

    /// The profile in `os_release` (the text of an `/etc/os-release`).
    pub fn from_os_release(os_release: &str) -> Result<Self, ProfileError> {
        Ok(Self {
            packages: os_release_fact(os_release, REC_PACKAGES)?,
            init: os_release_fact(os_release, REC_INIT)?,
            libc: os_release_fact(os_release, REC_LIBC)?,
            privilege: os_release_fact(os_release, REC_PRIVILEGE)?,
            desktop: os_release_fact(os_release, REC_DESKTOP)?,
            kernel_line: os_release_fact(os_release, REC_KERNEL_LINE)?,
        })
    }
}

/// One fact from the text of an `/etc/os-release`, for a reader that needs
/// only that one.
pub fn os_release_fact<T: FromStr<Err = String>>(
    os_release: &str,
    field: &'static str,
) -> Result<T, ProfileError> {
    let value = os_release_value(os_release, field).ok_or(ProfileError {
        field,
        problem: None,
    })?;
    value.parse().map_err(|problem| ProfileError {
        field,
        problem: Some(problem),
    })
}

/// The value of `field` in the text of an `/etc/os-release` (quotes removed),
/// when it is set and not empty.
pub fn os_release_value(os_release: &str, field: &str) -> Option<String> {
    os_release
        .lines()
        .filter_map(|line| line.trim().strip_prefix(field)?.strip_prefix('='))
        .map(|value| value.trim().trim_matches(['"', '\'']).trim().to_string())
        .rfind(|value| !value.is_empty())
}

/// A profile fact missing from an os-release, or set to an unknown value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileError {
    pub field: &'static str,
    /// `None` when the field is missing.
    pub problem: Option<String>,
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.problem {
            None => write!(
                f,
                "os-release has no {} field: the system was built without a distro profile (rebuild it with the current builder)",
                self.field
            ),
            Some(problem) => write!(f, "os-release field {}: {problem}", self.field),
        }
    }
}

impl std::error::Error for ProfileError {}
