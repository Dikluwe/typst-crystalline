//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/init.md
//! @prompt-hash 1b601c5e
//! @layer L3

use std::fs;
use std::path::{Component, Path, PathBuf};

use typst_core::contracts::package_downloader::PackageDownloader;
use typst_core::entities::package_spec::{
    PackageSpec, PackageVersion, VersionlessPackageSpec,
};

use crate::package_downloader::HttpPackageDownloader;

pub struct InitResult {
    pub destination: PathBuf,
    pub entrypoint: PathBuf,
    pub spec: PackageSpec,
}

pub fn initialize(
    template: &str,
    destination: Option<PathBuf>,
    custom_ca: Option<PathBuf>,
) -> Result<InitResult, String> {
    let (spec, package_dir) = resolve_package(template, custom_ca)?;
    let destination = destination.unwrap_or_else(|| PathBuf::from(&spec.name));
    if destination.exists() {
        return Err(format!("destination already exists: {}", destination.display()));
    }

    let manifest_path = package_dir.join("typst.toml");
    let manifest_text = fs::read_to_string(&manifest_path)
        .map_err(|_| "template package has no readable typst.toml".to_string())?;
    let manifest: toml::Value = manifest_text
        .parse()
        .map_err(|_| "template package has an invalid typst.toml".to_string())?;
    validate_identity(&manifest, &spec)?;
    let template_table = manifest
        .get("template")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "package does not contain a [template] section".to_string())?;
    let template_path = relative_path(template_table, "path")?;
    let entrypoint = relative_path(template_table, "entrypoint")?;

    let package_root = package_dir
        .canonicalize()
        .map_err(|_| "template package directory is not accessible".to_string())?;
    let source = package_root.join(&template_path);
    let source = source
        .canonicalize()
        .map_err(|_| "template path does not exist".to_string())?;
    if !source.starts_with(&package_root) || !source.is_dir() {
        return Err("template path escapes the package root".to_string());
    }
    let source_entrypoint = source.join(&entrypoint);
    let source_entrypoint = source_entrypoint
        .canonicalize()
        .map_err(|_| "template entrypoint does not exist".to_string())?;
    if !source_entrypoint.starts_with(&source) || !source_entrypoint.is_file() {
        return Err("template entrypoint escapes the template directory".to_string());
    }

    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    if !parent.is_dir() {
        return Err(format!("destination parent does not exist: {}", parent.display()));
    }
    let leaf = destination
        .file_name()
        .ok_or_else(|| "destination must have a directory name".to_string())?;
    let staging = parent.join(format!(
        ".{}.typst-init-{}-{:08x}",
        leaf.to_string_lossy(),
        std::process::id(),
        fastrand::u32(..),
    ));
    let guard = StagingGuard(staging.clone());
    fs::create_dir(&staging)
        .map_err(|error| format!("failed to create staging directory: {error}"))?;
    copy_tree(&source, &staging)?;
    fs::rename(&staging, &destination)
        .map_err(|error| format!("failed to finalize project: {error}"))?;
    std::mem::forget(guard);

    Ok(InitResult {
        destination: destination.clone(),
        entrypoint: destination.join(entrypoint),
        spec,
    })
}

fn resolve_package(
    raw: &str,
    custom_ca: Option<PathBuf>,
) -> Result<(PackageSpec, PathBuf), String> {
    let spec = match raw.parse::<PackageSpec>() {
        Ok(spec) => spec,
        Err(error) if error.to_string() == "package specification is missing version" => {
            let parsed = format!("{raw}:0.0.0")
                .parse::<PackageSpec>()
                .map_err(|error| error.to_string())?;
            let versionless =
                VersionlessPackageSpec { namespace: parsed.namespace, name: parsed.name };
            let version = latest_version(&versionless, custom_ca.clone())?;
            versionless.at(version)
        }
        Err(error) => return Err(error.to_string()),
    };

    for base in package_roots(true) {
        let candidate = base
            .join(&spec.namespace)
            .join(&spec.name)
            .join(spec.version.to_string());
        if candidate.is_dir() {
            return Ok((spec, candidate));
        }
    }
    if spec.namespace == "preview" {
        let cache = package_cache_root()
            .ok_or_else(|| "package cache directory is unavailable".to_string())?;
        let downloader = HttpPackageDownloader::new(cache).with_custom_ca(custom_ca);
        let directory = downloader.download(&spec).map_err(|error| error.to_string())?;
        return Ok((spec, directory));
    }
    Err(format!("package not found (searched for {spec})"))
}

fn latest_version(
    spec: &VersionlessPackageSpec,
    custom_ca: Option<PathBuf>,
) -> Result<PackageVersion, String> {
    if spec.namespace == "preview" {
        let cache = package_cache_root()
            .ok_or_else(|| "package cache directory is unavailable".to_string())?;
        return HttpPackageDownloader::new(cache)
            .with_custom_ca(custom_ca)
            .latest_version(&spec.name)?
            .ok_or_else(|| format!("package not found (searched for {spec})"));
    }
    let data = package_data_root()
        .ok_or_else(|| "package data directory is unavailable".to_string())?;
    let versions = data.join(&spec.namespace).join(&spec.name);
    let mut latest = None;
    if let Ok(entries) = fs::read_dir(versions) {
        for entry in entries.flatten() {
            if let Some(version) =
                entry.file_name().to_str().and_then(|name| name.parse().ok())
            {
                latest = Some(
                    latest
                        .map_or(version, |current: PackageVersion| current.max(version)),
                );
            }
        }
    }
    latest.ok_or_else(|| format!("package not found (searched for {spec})"))
}

fn validate_identity(manifest: &toml::Value, spec: &PackageSpec) -> Result<(), String> {
    let package = manifest
        .get("package")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "typst.toml has no [package] section".to_string())?;
    let name = package.get("name").and_then(toml::Value::as_str);
    let version = package.get("version").and_then(toml::Value::as_str);
    if name != Some(spec.name.as_str())
        || version != Some(spec.version.to_string().as_str())
    {
        return Err(
            "template manifest identity does not match the requested package".to_string()
        );
    }
    Ok(())
}

fn relative_path(
    table: &toml::map::Map<String, toml::Value>,
    key: &str,
) -> Result<PathBuf, String> {
    let value = table
        .get(key)
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("[template].{key} is missing"))?;
    let path = PathBuf::from(value);
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path.components().any(|part| {
            matches!(
                part,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(format!(
            "[template].{key} must be a relative path inside the package"
        ));
    }
    Ok(path)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    for entry in fs::read_dir(source)
        .map_err(|error| format!("failed to read template: {error}"))?
    {
        let entry = entry.map_err(|error| format!("failed to read template: {error}"))?;
        let kind = entry
            .file_type()
            .map_err(|error| format!("failed to inspect template: {error}"))?;
        let target = destination.join(entry.file_name());
        if kind.is_symlink() {
            return Err(
                "template contains a symbolic link; refusing unsafe materialization"
                    .to_string(),
            );
        } else if kind.is_dir() {
            fs::create_dir(&target).map_err(|error| {
                format!("failed to create template directory: {error}")
            })?;
            copy_tree(&entry.path(), &target)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), target)
                .map_err(|error| format!("failed to copy template file: {error}"))?;
        }
    }
    Ok(())
}

fn package_data_root() -> Option<PathBuf> {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share"))
        })
        .map(|root| root.join("typst/packages"))
}

fn package_cache_root() -> Option<PathBuf> {
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache"))
        })
        .map(|root| root.join("typst/packages"))
}

fn package_roots(include_cache: bool) -> Vec<PathBuf> {
    package_data_root()
        .into_iter()
        .chain(include_cache.then(package_cache_root).flatten())
        .collect()
}

struct StagingGuard(PathBuf);

impl Drop for StagingGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
