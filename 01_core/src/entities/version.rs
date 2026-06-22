//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/version.md
//! @prompt-hash e681232c
//! @layer L1
//! @updated 2026-06-22
//!
//! Número de versão semântico — `Value::Version`.
//! Passo 401: tipo L1 puro, zero I/O; segue semver 2.0.0.

use ecow::EcoString;

/// Número de versão semântico (semver 2.0.0).
///
/// Representa `major.minor.patch[-pre][+build]`. Prerelease e build metadata
/// são listas de identificadores separados por `.`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    /// Identificadores de prerelease (ex.: `["alpha", "1"]`).
    pub pre: Vec<EcoString>,
    /// Build metadata (ex.: `["build", "2"]`); ignorado na comparação.
    pub build: Vec<EcoString>,
}

impl Version {
    /// Cria uma versão `major.minor.patch` sem prerelease nem build metadata.
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: Vec::new(),
            build: Vec::new(),
        }
    }

    /// Adiciona prerelease identifiers.
    pub fn with_pre(mut self, pre: Vec<EcoString>) -> Self {
        self.pre = pre;
        self
    }

    /// Adiciona build metadata identifiers.
    pub fn with_build(mut self, build: Vec<EcoString>) -> Self {
        self.build = build;
        self
    }

    /// Formato canónico `"major.minor.patch[-pre][+build]"`.
    pub fn to_string(&self) -> String {
        let mut s = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if !self.pre.is_empty() {
            s.push('-');
            s.push_str(&self.pre.join("."));
        }
        if !self.build.is_empty() {
            s.push('+');
            s.push_str(&self.build.join("."));
        }
        s
    }

    /// Parse de string semver. Aceita `"major.minor.patch[-pre][+build]"`.
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        // Build metadata vem depois do '+'.
        let (s, build) = if let Some(pos) = s.find('+') {
            let (core, rest) = s.split_at(pos);
            let build = rest[1..].split('.').map(EcoString::from).collect();
            (core, build)
        } else {
            (s, Vec::new())
        };

        // Prerelease vem depois do primeiro '-'.
        let (s, pre) = if let Some(pos) = s.find('-') {
            let (core, rest) = s.split_at(pos);
            let pre = rest[1..].split('.').map(EcoString::from).collect();
            (core, pre)
        } else {
            (s, Vec::new())
        };

        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let major = parts[0].parse::<u64>().ok()?;
        let minor = parts[1].parse::<u64>().ok()?;
        let patch = parts[2].parse::<u64>().ok()?;

        Some(Self { major, minor, patch, pre, build })
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self
            .major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch));

        if ord != std::cmp::Ordering::Equal {
            return ord;
        }

        // Sem prerelease > com prerelease.
        match (self.pre.is_empty(), other.pre.is_empty()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => cmp_pre(&self.pre, &other.pre),
        }
    }
}

/// Compara prerelease identifiers lexicograficamente / numericamente.
fn cmp_pre(a: &[EcoString], b: &[EcoString]) -> std::cmp::Ordering {
    let len = a.len().min(b.len());
    for i in 0..len {
        let ord = cmp_pre_id(&a[i], &b[i]);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }
    }
    a.len().cmp(&b.len())
}

/// Compara um identificador de prerelease segundo semver 2.0.0.
fn cmp_pre_id(a: &str, b: &str) -> std::cmp::Ordering {
    match (a.parse::<u64>(), b.parse::<u64>()) {
        (Ok(a_num), Ok(b_num)) => a_num.cmp(&b_num),
        (Ok(_), Err(_)) => std::cmp::Ordering::Less,    // numeric < non-numeric
        (Err(_), Ok(_)) => std::cmp::Ordering::Greater, // non-numeric > numeric
        (Err(_), Err(_)) => a.cmp(b),                   // lexicographic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(parts: &[&str]) -> Vec<EcoString> {
        parts.iter().map(|s| EcoString::from(*s)).collect()
    }

    #[test]
    fn version_new() {
        let v = Version::new(1, 2, 3);
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert!(v.pre.is_empty());
        assert!(v.build.is_empty());
    }

    #[test]
    fn version_to_string_core() {
        assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
    }

    #[test]
    fn version_to_string_pre() {
        let v = Version::new(1, 2, 3).with_pre(ids(&["alpha", "1"]));
        assert_eq!(v.to_string(), "1.2.3-alpha.1");
    }

    #[test]
    fn version_to_string_build() {
        let v = Version::new(1, 2, 3).with_build(ids(&["build", "2"]));
        assert_eq!(v.to_string(), "1.2.3+build.2");
    }

    #[test]
    fn version_to_string_pre_build() {
        let v = Version::new(1, 2, 3)
            .with_pre(ids(&["alpha", "1"]))
            .with_build(ids(&["build", "2"]));
        assert_eq!(v.to_string(), "1.2.3-alpha.1+build.2");
    }

    #[test]
    fn version_parse_core() {
        let v = Version::from_str("1.2.3").unwrap();
        assert_eq!(v, Version::new(1, 2, 3));
    }

    #[test]
    fn version_parse_pre() {
        let v = Version::from_str("1.2.3-alpha.1").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.pre, ids(&["alpha", "1"]));
    }

    #[test]
    fn version_parse_build() {
        let v = Version::from_str("1.2.3+build.2").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.build, ids(&["build", "2"]));
    }

    #[test]
    fn version_parse_pre_build() {
        let v = Version::from_str("1.2.3-alpha.1+build.2").unwrap();
        assert_eq!(v.pre, ids(&["alpha", "1"]));
        assert_eq!(v.build, ids(&["build", "2"]));
    }

    #[test]
    fn version_parse_invalid() {
        assert!(Version::from_str("abc").is_none());
        assert!(Version::from_str("").is_none());
        assert!(Version::from_str("1.2").is_none());
        assert!(Version::from_str("1.2.3.4").is_none());
    }

    #[test]
    fn version_default() {
        assert_eq!(Version::default(), Version::new(0, 0, 0));
    }

    // ── Comparação semver ────────────────────────────────────────────────────

    #[test]
    fn version_cmp_major() {
        assert!(Version::new(1, 0, 0) < Version::new(2, 0, 0));
    }

    #[test]
    fn version_cmp_minor() {
        assert!(Version::new(1, 1, 0) < Version::new(1, 2, 0));
    }

    #[test]
    fn version_cmp_patch() {
        assert!(Version::new(1, 0, 1) < Version::new(1, 0, 2));
    }

    #[test]
    fn version_cmp_pre_vs_release() {
        let pre = Version::new(1, 0, 0).with_pre(ids(&["alpha"]));
        let rel = Version::new(1, 0, 0);
        assert!(pre < rel);
    }

    #[test]
    fn version_cmp_pre_numeric() {
        let a = Version::new(1, 0, 0).with_pre(ids(&["alpha", "1"]));
        let b = Version::new(1, 0, 0).with_pre(ids(&["alpha", "2"]));
        assert!(a < b);
    }

    #[test]
    fn version_cmp_pre_mixed() {
        let a = Version::new(1, 0, 0).with_pre(ids(&["alpha"]));
        let b = Version::new(1, 0, 0).with_pre(ids(&["beta"]));
        assert!(a < b);
    }

    #[test]
    fn version_cmp_pre_num_vs_str() {
        let a = Version::new(1, 0, 0).with_pre(ids(&["1"]));
        let b = Version::new(1, 0, 0).with_pre(ids(&["alpha"]));
        assert!(a < b);
    }

    #[test]
    fn version_cmp_build_ignored() {
        let a = Version::new(1, 0, 0).with_build(ids(&["build1"]));
        let b = Version::new(1, 0, 0).with_build(ids(&["build2"]));
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
    }

    #[test]
    fn version_cmp_pre_len() {
        let a = Version::new(1, 0, 0).with_pre(ids(&["alpha"]));
        let b = Version::new(1, 0, 0).with_pre(ids(&["alpha", "1"]));
        assert!(a < b);
    }
}
