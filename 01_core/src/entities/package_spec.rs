//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/package-spec.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-03-22

use std::fmt;
use std::str::FromStr;

use crate::rules::lexer::{is_ident};
use crate::rules::lexer::scanner::Scanner;

/// Identifica um pacote Typst pelo seu namespace, nome e versão.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageSpec {
    pub namespace: String,
    pub name:      String,
    pub version:   PackageVersion,
}

impl PackageSpec {
    /// Remove a versão, retornando um `VersionlessPackageSpec`.
    pub fn versionless(&self) -> VersionlessPackageSpec {
        VersionlessPackageSpec {
            namespace: self.namespace.clone(),
            name:      self.name.clone(),
        }
    }
}

impl fmt::Display for PackageSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}/{}:{}", self.namespace, self.name, self.version)
    }
}

/// Identifica um pacote sem especificar a versão.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VersionlessPackageSpec {
    pub namespace: String,
    pub name:      String,
}

impl VersionlessPackageSpec {
    /// Preenche a versão para obter um `PackageSpec` completo.
    pub fn at(self, version: PackageVersion) -> PackageSpec {
        PackageSpec { namespace: self.namespace, name: self.name, version }
    }
}

impl fmt::Display for VersionlessPackageSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}/{}", self.namespace, self.name)
    }
}

/// Versão semântica de um pacote Typst.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl fmt::Display for PackageVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Erro ao fazer parse de um `PackageSpec` ou `PackageVersion`.
#[derive(Debug)]
pub struct PackageSpecError(pub String);

impl fmt::Display for PackageSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PackageSpecError {}

impl FromStr for PackageSpec {
    type Err = PackageSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sc = Scanner::new(s);

        // namespace: @name/
        if !sc.eat_if('@') {
            return Err(PackageSpecError("package specification must start with '@'".into()));
        }
        let namespace = sc.eat_until('/');
        if namespace.is_empty() {
            return Err(PackageSpecError("package specification is missing namespace".into()));
        }
        if !is_ident(namespace) {
            return Err(PackageSpecError(
                format!("`{namespace}` is not a valid package namespace"),
            ));
        }

        // name: name:
        sc.eat_if('/');
        let name = sc.eat_until(':');
        if name.is_empty() {
            return Err(PackageSpecError("package specification is missing name".into()));
        }
        if !is_ident(name) {
            return Err(PackageSpecError(
                format!("`{name}` is not a valid package name"),
            ));
        }

        // version: x.y.z
        sc.eat_if(':');
        let version_str = sc.after();
        if version_str.is_empty() {
            return Err(PackageSpecError("package specification is missing version".into()));
        }
        let version = version_str.parse::<PackageVersion>()
            .map_err(|e| PackageSpecError(e.0))?;

        Ok(Self {
            namespace: namespace.to_owned(),
            name: name.to_owned(),
            version,
        })
    }
}

/// Erro ao fazer parse de uma `PackageVersion`.
#[derive(Debug)]
pub struct PackageVersionError(pub String);

impl fmt::Display for PackageVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PackageVersionError {}

impl FromStr for PackageVersion {
    type Err = PackageVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let mut next = |kind: &str| -> Result<u32, PackageVersionError> {
            let part = parts
                .next()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| PackageVersionError(
                    format!("version number is missing {kind} version"),
                ))?;
            part.parse::<u32>().map_err(|_| PackageVersionError(
                format!("`{part}` is not a valid {kind} version"),
            ))
        };
        let major = next("major")?;
        let minor = next("minor")?;
        let patch = next("patch")?;
        if let Some(rest) = parts.next() {
            return Err(PackageVersionError(format!(
                "version number has unexpected fourth component: `{rest}`"
            )));
        }
        Ok(Self { major, minor, patch })
    }
}

/// Limite de versão para especificações de compatibilidade.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VersionBound {
    pub major: u32,
    pub minor: Option<u32>,
    /// Só pode estar presente se `minor` também estiver.
    pub patch: Option<u32>,
}

impl VersionBound {
    /// Verifica se `version` satisfaz este limite.
    ///
    /// Componentes ausentes no bound são ignorados (wildcard).
    pub fn matches(&self, v: &PackageVersion) -> bool {
        v.major == self.major
            && self.minor.is_none_or(|m| v.minor == m)
            && self.patch.is_none_or(|p| v.patch == p)
    }
}

impl fmt::Display for VersionBound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.major)?;
        if let Some(minor) = self.minor {
            write!(f, ".{minor}")?;
        }
        if let Some(patch) = self.patch {
            write!(f, ".{patch}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn preview_algo_010() -> PackageSpec {
        PackageSpec {
            namespace: "preview".to_string(),
            name: "algo".to_string(),
            version: PackageVersion { major: 0, minor: 1, patch: 0 },
        }
    }

    #[test]
    fn package_spec_display() {
        assert_eq!(preview_algo_010().to_string(), "@preview/algo:0.1.0");
    }

    #[test]
    fn package_version_display() {
        let v = PackageVersion { major: 1, minor: 2, patch: 3 };
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn package_version_from_str_ok() {
        let v = "1.2.3".parse::<PackageVersion>().unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn package_version_from_str_err_non_numeric() {
        assert!("1.x.3".parse::<PackageVersion>().is_err());
    }

    #[test]
    fn package_version_from_str_err_too_few_parts() {
        assert!("1.2".parse::<PackageVersion>().is_err());
    }

    #[test]
    fn package_version_from_str_err_too_many_parts() {
        assert!("1.2.3.4".parse::<PackageVersion>().is_err());
    }

    #[test]
    fn package_version_ord() {
        let v1 = PackageVersion { major: 1, minor: 0, patch: 0 };
        let v2 = PackageVersion { major: 2, minor: 0, patch: 0 };
        assert!(v1 < v2);
        assert!(v2 > v1);
    }

    #[test]
    fn version_bound_matches_minor_any_patch() {
        let bound = VersionBound { major: 1, minor: Some(2), patch: None };
        assert!(bound.matches(&PackageVersion { major: 1, minor: 2, patch: 5 }));
        assert!(bound.matches(&PackageVersion { major: 1, minor: 2, patch: 0 }));
    }

    #[test]
    fn version_bound_matches_false_different_minor() {
        let bound = VersionBound { major: 1, minor: Some(2), patch: None };
        assert!(!bound.matches(&PackageVersion { major: 1, minor: 3, patch: 0 }));
    }

    #[test]
    fn package_spec_ne_when_version_differs() {
        let a = preview_algo_010();
        let b = PackageSpec { version: PackageVersion { major: 0, minor: 2, patch: 0 }, ..a.clone() };
        assert_ne!(a, b);
    }

    #[test]
    fn versionless_from_package_spec() {
        let spec = preview_algo_010();
        let vl = spec.versionless();
        assert_eq!(vl.namespace, "preview");
        assert_eq!(vl.name, "algo");
    }

    #[test]
    fn versionless_display() {
        let vl = VersionlessPackageSpec {
            namespace: "preview".to_string(),
            name: "algo".to_string(),
        };
        assert_eq!(vl.to_string(), "@preview/algo");
    }
}
