//! Feature/variant handling for recipes.
//!
//! Features allow compile-time conditional compilation options:
//!
//! ```text
//! (features
//!   (default "x264" "opus")
//!   (x264 "Enable H.264 support")
//!   (vulkan "Enable Vulkan acceleration"))
//! ```

use std::collections::{HashMap, HashSet};
use std::fmt;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum FeatureError {
    #[error("unknown feature: {0}")]
    UnknownFeature(String),
    #[error("circular feature dependency: {0}")]
    CircularDependency(String),
}

/// A single feature definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    /// Feature name (e.g., "x264", "vulkan")
    pub name: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Other features this implies/enables
    pub implies: Vec<String>,
}

impl Feature {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            implies: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_implies(mut self, implies: Vec<String>) -> Self {
        self.implies = implies;
        self
    }
}

/// Collection of available features and defaults.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FeatureSet {
    /// All available features by name
    pub available: HashMap<String, Feature>,
    /// Features enabled by default
    pub default: HashSet<String>,
}

impl FeatureSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a feature to the set.
    pub fn add(&mut self, feature: Feature) {
        self.available.insert(feature.name.clone(), feature);
    }

    /// Mark a feature as default-enabled.
    pub fn set_default(&mut self, name: impl Into<String>) {
        self.default.insert(name.into());
    }

    /// Check if a feature exists.
    pub fn has(&self, name: &str) -> bool {
        self.available.contains_key(name)
    }

    /// Get a feature by name.
    pub fn get(&self, name: &str) -> Option<&Feature> {
        self.available.get(name)
    }

    /// Resolve a set of enabled features, including all implied features.
    /// Returns error if any feature is unknown or there's a circular dependency.
    pub fn resolve(&self, enabled: &[String]) -> Result<HashSet<String>, FeatureError> {
        let mut resolved = HashSet::new();
        let mut visiting = HashSet::new();

        for feature_name in enabled {
            self.resolve_one(feature_name, &mut resolved, &mut visiting)?;
        }

        Ok(resolved)
    }

    /// Resolve enabled features starting from defaults, with user overrides.
    /// `additions` are features to enable (prefixed with + in CLI)
    /// `removals` are features to disable (prefixed with - in CLI)
    pub fn resolve_with_defaults(
        &self,
        additions: &[String],
        removals: &[String],
    ) -> Result<HashSet<String>, FeatureError> {
        // Start with defaults
        let mut enabled: Vec<String> = self.default.iter().cloned().collect();

        // Add user-requested features
        for add in additions {
            if !enabled.contains(add) {
                enabled.push(add.clone());
            }
        }

        // Remove user-disabled features
        enabled.retain(|f| !removals.contains(f));

        self.resolve(&enabled)
    }

    fn resolve_one(
        &self,
        name: &str,
        resolved: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
    ) -> Result<(), FeatureError> {
        if resolved.contains(name) {
            return Ok(());
        }

        if visiting.contains(name) {
            return Err(FeatureError::CircularDependency(name.to_string()));
        }

        let feature = self
            .available
            .get(name)
            .ok_or_else(|| FeatureError::UnknownFeature(name.to_string()))?;

        visiting.insert(name.to_string());

        // Recursively resolve implied features
        for implied in &feature.implies {
            self.resolve_one(implied, resolved, visiting)?;
        }

        visiting.remove(name);
        resolved.insert(name.to_string());

        Ok(())
    }
}

/// A dependency that may be conditional on a feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepSpec {
    /// Always required dependency
    Always(crate::version::Dependency),
    /// Dependency only required when feature is enabled
    Conditional {
        feature: String,
        dep: crate::version::Dependency,
    },
}

impl DepSpec {
    /// Create an always-required dependency.
    pub fn always(dep: crate::version::Dependency) -> Self {
        DepSpec::Always(dep)
    }

    /// Create a conditional dependency.
    pub fn when(feature: impl Into<String>, dep: crate::version::Dependency) -> Self {
        DepSpec::Conditional {
            feature: feature.into(),
            dep,
        }
    }

    /// Check if this dependency is required given the set of enabled features.
    pub fn is_required(&self, enabled_features: &HashSet<String>) -> bool {
        match self {
            DepSpec::Always(_) => true,
            DepSpec::Conditional { feature, .. } => enabled_features.contains(feature),
        }
    }

    /// Get the underlying dependency.
    pub fn dependency(&self) -> &crate::version::Dependency {
        match self {
            DepSpec::Always(dep) => dep,
            DepSpec::Conditional { dep, .. } => dep,
        }
    }
}

impl fmt::Display for DepSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DepSpec::Always(dep) => write!(f, "{}", dep),
            DepSpec::Conditional { feature, dep } => write!(f, "(if {} {})", feature, dep),
        }
    }
}

/// Expand feature conditionals in a string.
/// Handles `$[if feature text]` syntax.
pub fn expand_feature_conditionals(s: &str, enabled: &HashSet<String>) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' && chars.peek() == Some(&'[') {
            chars.next(); // consume '['
            let mut content = String::new();
            let mut depth = 1;

            // Collect content until matching ]
            while let Some(c) = chars.next() {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                content.push(c);
            }

            // Parse "if feature text"
            if let Some(rest) = content.strip_prefix("if ") {
                let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    let feature = parts[0];
                    let text = parts[1];
                    if enabled.contains(feature) {
                        result.push_str(text);
                    }
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::Dependency;

    #[test]
    fn test_feature_set_resolution() {
        let mut features = FeatureSet::new();
        features.add(Feature::new("opengl").with_description("OpenGL support"));
        features.add(
            Feature::new("vulkan")
                .with_description("Vulkan support")
                .with_implies(vec!["gpu".to_string()]),
        );
        features.add(Feature::new("gpu").with_description("GPU support"));

        let enabled = vec!["vulkan".to_string()];
        let resolved = features.resolve(&enabled).unwrap();

        assert!(resolved.contains("vulkan"));
        assert!(resolved.contains("gpu")); // implied by vulkan
        assert!(!resolved.contains("opengl"));
    }

    #[test]
    fn test_feature_set_with_defaults() {
        let mut features = FeatureSet::new();
        features.add(Feature::new("x264"));
        features.add(Feature::new("x265"));
        features.add(Feature::new("opus"));
        features.set_default("x264");
        features.set_default("opus");

        let resolved = features
            .resolve_with_defaults(&["x265".to_string()], &["opus".to_string()])
            .unwrap();

        assert!(resolved.contains("x264")); // default
        assert!(resolved.contains("x265")); // added
        assert!(!resolved.contains("opus")); // removed
    }

    #[test]
    fn test_dep_spec() {
        let mut enabled = HashSet::new();
        enabled.insert("vulkan".to_string());

        let always_dep = DepSpec::always(Dependency::new("zlib"));
        assert!(always_dep.is_required(&enabled));

        let vulkan_dep = DepSpec::when("vulkan", Dependency::new("vulkan-loader"));
        assert!(vulkan_dep.is_required(&enabled));

        let x11_dep = DepSpec::when("x11", Dependency::new("libX11"));
        assert!(!x11_dep.is_required(&enabled));
    }

    #[test]
    fn test_expand_feature_conditionals() {
        let mut enabled = HashSet::new();
        enabled.insert("vulkan".to_string());
        enabled.insert("x264".to_string());

        let input = "./configure $[if vulkan --enable-vulkan] $[if x11 --enable-x11] $[if x264 --enable-libx264]";
        let output = expand_feature_conditionals(input, &enabled);

        assert_eq!(output, "./configure --enable-vulkan  --enable-libx264");
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut features = FeatureSet::new();
        features.add(Feature::new("a").with_implies(vec!["b".to_string()]));
        features.add(Feature::new("b").with_implies(vec!["a".to_string()]));

        let result = features.resolve(&["a".to_string()]);
        assert!(matches!(result, Err(FeatureError::CircularDependency(_))));
    }
}
