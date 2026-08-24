//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/path.md
//! @prompt-hash c9a7b4f1
//! @layer L1
//! @updated 2026-08-24

use ecow::EcoString;

use super::package_spec::PackageSpec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VirtualRoot {
    Project,
    Package(PackageSpec),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VirtualPath(EcoString);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathError {
    Escapes,
    Backslash,
}

impl VirtualPath {
    pub fn new(path: &str) -> Result<Self, PathError> {
        Self::normalize(std::iter::empty::<&str>(), path)
    }

    pub fn join(&self, path: &str) -> Result<Self, PathError> {
        Self::normalize(self.segments(), path)
    }

    fn normalize<'a>(
        base: impl Iterator<Item = &'a str>,
        path: &str,
    ) -> Result<Self, PathError> {
        if path.contains('\\') {
            return Err(PathError::Backslash);
        }
        let mut segments: Vec<&str> =
            if path.starts_with('/') { Vec::new() } else { base.collect() };
        for part in path.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    if segments.pop().is_none() {
                        return Err(PathError::Escapes);
                    }
                }
                other => segments.push(other),
            }
        }
        let mut normalized = EcoString::from("/");
        normalized.push_str(&segments.join("/"));
        Ok(Self(normalized))
    }

    fn segments(&self) -> impl Iterator<Item = &str> {
        self.0.trim_start_matches('/').split('/').filter(|s| !s.is_empty())
    }

    pub fn parent(&self) -> Option<Self> {
        if self.0 == "/" {
            return None;
        }
        let mut segments: Vec<_> = self.segments().collect();
        segments.pop();
        Some(Self(EcoString::from(format!("/{}", segments.join("/")))))
    }

    pub fn get_with_slash(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootedPath {
    root: VirtualRoot,
    vpath: VirtualPath,
}

impl RootedPath {
    pub fn new(root: VirtualRoot, vpath: VirtualPath) -> Self {
        Self { root, vpath }
    }
    pub fn root(&self) -> &VirtualRoot {
        &self.root
    }
    pub fn vpath(&self) -> &VirtualPath {
        &self.vpath
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathOrStr {
    Path(RootedPath),
    Str(EcoString),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1141_normaliza_dot_parent_e_absoluto() {
        assert_eq!(VirtualPath::new("a/./b/../c").unwrap().get_with_slash(), "/a/c");
        let base = VirtualPath::new("/sub").unwrap();
        assert_eq!(base.join("./x").unwrap(), base.join("x").unwrap());
        assert_eq!(base.join("/root/x").unwrap().get_with_slash(), "/root/x");
    }

    #[test]
    fn p1141_rejeita_escape_e_backslash() {
        assert_eq!(VirtualPath::new("../x"), Err(PathError::Escapes));
        assert_eq!(VirtualPath::new("a\\b"), Err(PathError::Backslash));
    }

    #[test]
    fn p1141_root_participa_da_identidade() {
        let v = VirtualPath::new("/x").unwrap();
        assert_ne!(
            RootedPath::new(VirtualRoot::Project, v.clone()),
            RootedPath::new(
                VirtualRoot::Package("@preview/demo:1.0.0".parse().unwrap()),
                v,
            )
        );
    }
}
