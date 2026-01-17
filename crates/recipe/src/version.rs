//! Version parsing and constraint handling.
//!
//! Supports semver-like versions and various constraint operators:
//! - `>=`, `>`, `<=`, `<`, `=` - Standard comparisons
//! - `~=` - Compatible release (major.minor compatible)
//! - No operator = any version

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum VersionError {
    #[error("invalid version format: {0}")]
    InvalidFormat(String),
    #[error("invalid constraint: {0}")]
    InvalidConstraint(String),
}

/// A semantic version with major, minor, patch, and optional prerelease.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub prerelease: Option<String>,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease: None,
        }
    }

    /// Check if this version is compatible with another (same major.minor).
    pub fn is_compatible_with(&self, other: &Version) -> bool {
        self.major == other.major && self.minor == other.minor
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(VersionError::InvalidFormat("empty version".to_string()));
        }

        // Split off prerelease (-alpha, -beta, -rc1, etc.)
        let (version_part, prerelease) = if let Some(idx) = s.find('-') {
            (&s[..idx], Some(s[idx + 1..].to_string()))
        } else {
            (s, None)
        };

        let parts: Vec<&str> = version_part.split('.').collect();
        if parts.is_empty() || parts.len() > 3 {
            return Err(VersionError::InvalidFormat(s.to_string()));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| VersionError::InvalidFormat(s.to_string()))?;

        let minor = parts
            .get(1)
            .map(|p| p.parse())
            .transpose()
            .map_err(|_| VersionError::InvalidFormat(s.to_string()))?
            .unwrap_or(0);

        let patch = parts
            .get(2)
            .map(|p| p.parse())
            .transpose()
            .map_err(|_| VersionError::InvalidFormat(s.to_string()))?
            .unwrap_or(0);

        Ok(Version {
            major,
            minor,
            patch,
            prerelease,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre) = self.prerelease {
            write!(f, "-{}", pre)?;
        }
        Ok(())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.minor.cmp(&other.minor) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.patch.cmp(&other.patch) {
            Ordering::Equal => {}
            ord => return ord,
        }
        // Prerelease versions sort before release versions
        match (&self.prerelease, &other.prerelease) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Less,    // 1.0.0-alpha < 1.0.0
            (None, Some(_)) => Ordering::Greater, // 1.0.0 > 1.0.0-alpha
            (Some(a), Some(b)) => a.cmp(b),       // Lexicographic for prereleases
        }
    }
}

/// Version constraint operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionConstraint {
    /// Any version satisfies
    Any,
    /// Exactly equal to version
    Eq(Version),
    /// Greater than version
    Gt(Version),
    /// Greater than or equal to version
    Gte(Version),
    /// Less than version
    Lt(Version),
    /// Less than or equal to version
    Lte(Version),
    /// Compatible release (~=): same major.minor, patch >= specified
    Compatible(Version),
}

impl VersionConstraint {
    /// Check if a version satisfies this constraint.
    pub fn satisfies(&self, version: &Version) -> bool {
        match self {
            VersionConstraint::Any => true,
            VersionConstraint::Eq(v) => version == v,
            VersionConstraint::Gt(v) => version > v,
            VersionConstraint::Gte(v) => version >= v,
            VersionConstraint::Lt(v) => version < v,
            VersionConstraint::Lte(v) => version <= v,
            VersionConstraint::Compatible(v) => {
                version.major == v.major && version.minor == v.minor && version.patch >= v.patch
            }
        }
    }

    /// Parse a constraint from a string like ">= 1.2.3" or "~= 2.0".
    pub fn parse(s: &str) -> Result<Self, VersionError> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(VersionConstraint::Any);
        }

        // Try to find an operator
        if let Some(rest) = s.strip_prefix(">=") {
            let version = rest.trim().parse()?;
            return Ok(VersionConstraint::Gte(version));
        }
        if let Some(rest) = s.strip_prefix("<=") {
            let version = rest.trim().parse()?;
            return Ok(VersionConstraint::Lte(version));
        }
        if let Some(rest) = s.strip_prefix("~=") {
            let version = rest.trim().parse()?;
            return Ok(VersionConstraint::Compatible(version));
        }
        if let Some(rest) = s.strip_prefix('>') {
            let version = rest.trim().parse()?;
            return Ok(VersionConstraint::Gt(version));
        }
        if let Some(rest) = s.strip_prefix('<') {
            let version = rest.trim().parse()?;
            return Ok(VersionConstraint::Lt(version));
        }
        if let Some(rest) = s.strip_prefix('=') {
            let version = rest.trim().parse()?;
            return Ok(VersionConstraint::Eq(version));
        }

        // No operator - just a version (exact match)
        let version = s.parse()?;
        Ok(VersionConstraint::Eq(version))
    }
}

impl fmt::Display for VersionConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionConstraint::Any => write!(f, "*"),
            VersionConstraint::Eq(v) => write!(f, "= {}", v),
            VersionConstraint::Gt(v) => write!(f, "> {}", v),
            VersionConstraint::Gte(v) => write!(f, ">= {}", v),
            VersionConstraint::Lt(v) => write!(f, "< {}", v),
            VersionConstraint::Lte(v) => write!(f, "<= {}", v),
            VersionConstraint::Compatible(v) => write!(f, "~= {}", v),
        }
    }
}

/// A dependency with name and optional version constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub constraint: VersionConstraint,
}

impl Dependency {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            constraint: VersionConstraint::Any,
        }
    }

    pub fn with_constraint(name: impl Into<String>, constraint: VersionConstraint) -> Self {
        Self {
            name: name.into(),
            constraint,
        }
    }

    /// Parse a dependency string like "openssl >= 1.1.0" or just "zlib".
    pub fn parse(s: &str) -> Result<Self, VersionError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(VersionError::InvalidConstraint("empty dependency".to_string()));
        }

        // Find the operator position
        let operators = [">=", "<=", "~=", ">", "<", "="];
        for op in operators {
            if let Some(idx) = s.find(op) {
                let name = s[..idx].trim().to_string();
                let constraint_str = &s[idx..];
                let constraint = VersionConstraint::parse(constraint_str)?;
                return Ok(Dependency { name, constraint });
            }
        }

        // No operator - just a package name (any version)
        Ok(Dependency {
            name: s.to_string(),
            constraint: VersionConstraint::Any,
        })
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.constraint {
            VersionConstraint::Any => write!(f, "{}", self.name),
            constraint => write!(f, "{} {}", self.name, constraint),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parse() {
        assert_eq!(
            "1.2.3".parse::<Version>().unwrap(),
            Version::new(1, 2, 3)
        );
        assert_eq!(
            "1.2".parse::<Version>().unwrap(),
            Version { major: 1, minor: 2, patch: 0, prerelease: None }
        );
        assert_eq!(
            "1".parse::<Version>().unwrap(),
            Version { major: 1, minor: 0, patch: 0, prerelease: None }
        );
        assert_eq!(
            "1.0.0-alpha".parse::<Version>().unwrap(),
            Version { major: 1, minor: 0, patch: 0, prerelease: Some("alpha".to_string()) }
        );
    }

    #[test]
    fn test_version_ordering() {
        let v1: Version = "1.0.0".parse().unwrap();
        let v2: Version = "1.0.1".parse().unwrap();
        let v3: Version = "1.1.0".parse().unwrap();
        let v4: Version = "2.0.0".parse().unwrap();
        let v5: Version = "1.0.0-alpha".parse().unwrap();

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
        assert!(v5 < v1); // prerelease < release
    }

    #[test]
    fn test_constraint_parse() {
        assert_eq!(
            VersionConstraint::parse(">= 1.2.3").unwrap(),
            VersionConstraint::Gte(Version::new(1, 2, 3))
        );
        assert_eq!(
            VersionConstraint::parse("< 2.0").unwrap(),
            VersionConstraint::Lt(Version { major: 2, minor: 0, patch: 0, prerelease: None })
        );
        assert_eq!(
            VersionConstraint::parse("~= 1.5").unwrap(),
            VersionConstraint::Compatible(Version { major: 1, minor: 5, patch: 0, prerelease: None })
        );
    }

    #[test]
    fn test_constraint_satisfies() {
        let v = Version::new(1, 5, 3);

        assert!(VersionConstraint::Any.satisfies(&v));
        assert!(VersionConstraint::Gte(Version::new(1, 5, 0)).satisfies(&v));
        assert!(VersionConstraint::Lte(Version::new(2, 0, 0)).satisfies(&v));
        assert!(!VersionConstraint::Lt(Version::new(1, 5, 0)).satisfies(&v));
        assert!(VersionConstraint::Compatible(Version::new(1, 5, 0)).satisfies(&v));
        assert!(!VersionConstraint::Compatible(Version::new(1, 4, 0)).satisfies(&v));
    }

    #[test]
    fn test_dependency_parse() {
        let dep = Dependency::parse("openssl >= 1.1.0").unwrap();
        assert_eq!(dep.name, "openssl");
        assert_eq!(dep.constraint, VersionConstraint::Gte(Version::new(1, 1, 0)));

        let dep = Dependency::parse("zlib").unwrap();
        assert_eq!(dep.name, "zlib");
        assert_eq!(dep.constraint, VersionConstraint::Any);

        let dep = Dependency::parse("glibc < 3.0").unwrap();
        assert_eq!(dep.name, "glibc");
        assert_eq!(dep.constraint, VersionConstraint::Lt(Version::new(3, 0, 0)));
    }
}
