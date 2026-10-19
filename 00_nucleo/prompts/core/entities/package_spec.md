# Prompt L0 — `entities/package_spec`
Hash do Código: 4a31c1ed

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/package_spec.rs`
**Reference:** ADR-0005, pure package identifier/DTO boundary.

## Evidence before parsing contract

Historical upstream e0e8ca4d syntax/package.rs:224-232,310,314-350 specifies
@namespace/name:version, mandatory @, nonempty is_ident-validated namespace/
name and required version, using its Scanner/is_ident. The 2026-08-13 import
"@preview/example" probe produced `error: package specification is missing version`
at column 8 in both that vanilla and crystalline HEAD 4f64e4e69. Here diagnostic
text/location are observable. This historical evidence does not certify a
later pin or require copying upstream storage.

## Complete interface

Use the L1 Scanner and is_ident, no I/O/serde/semver/anyhow/external parsers.
Domain types retain the exact derives below; the original blanket all-types
trait sentence does not add traits to the explicitly Debug-only error wrappers.

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageSpec {
    pub namespace: String,
    pub name:      String,
    pub version:   PackageVersion,
}

impl PackageSpec {
    pub fn versionless(&self) -> VersionlessPackageSpec
}

impl fmt::Display for PackageSpec
impl FromStr      for PackageSpec


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VersionlessPackageSpec {
    pub namespace: String,
    pub name:      String,
}

impl VersionlessPackageSpec {
    pub fn at(self, version: PackageVersion) -> PackageSpec
}

impl fmt::Display for VersionlessPackageSpec


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl fmt::Display for PackageVersion
impl FromStr      for PackageVersion


#[derive(Debug)]
pub struct PackageSpecError(pub String);
impl fmt::Display for PackageSpecError
impl std::error::Error for PackageSpecError

#[derive(Debug)]
pub struct PackageVersionError(pub String);
impl fmt::Display for PackageVersionError
impl std::error::Error for PackageVersionError


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VersionBound {
    pub major: u32,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
}

impl VersionBound {
    pub fn matches(&self, v: &PackageVersion) -> bool
}

impl fmt::Display for VersionBound
```

PackageSpec Display is @namespace/name:major.minor.patch; versionless removes
version and Display gives @namespace/name; at(version) restores it.
PackageVersion displays major.minor.patch and requires exactly three numeric
u32 components. PackageSpec FromStr consumes @, namespace through /, name
through :, and remaining version; missing fields or invalid identifiers yield
PackageSpecError. Preserve diagnostics: `package specification must start with '@'`,
`package specification is missing namespace`, `package specification is missing version`,
and the invalid-namespace/name forms `` `{namespace}` is not a valid package namespace ``
and `` `{name}` is not a valid package name ``.

VersionBound requires major; minor/patch are optional, patch only with minor.
Missing components are wildcards; Display is major[.minor[.patch]].
PackageSpec/VersionlessPackageSpec/PackageVersion retain lexical ordering;
PackageVersion/VersionBound are Copy.

## Acceptance

@preview/algo:0.1.0 parses namespace preview,name algo,version0/1/0 and
roundtrips. Version1/2/3 displays 1.2.3. Reject missing@,empty namespace/name,
missing version,1.x.3,1.2 and 1.2.3.4. Version1.0.0 sorts before 2.0.0.
Bound{major1,minorSome2,patchNone} matches1.2.5,not 1.3.0.
versionless displays @preview/algo and at(0.1.0) restores the full spec.
Keep colocated tests and canonical lineage.

## Serialization boundary

Serde never enters L1. Future JSON/TOML DTO serialization belongs in L3.
The proposed03_infra/dto/package_spec_dto.rs consumer does not exist and this
owner cannot authorize it. Its complete unresolved conversion/API proposal is
preserved in `lab/history/construction/retired-l0/package-spec-dto.md`, with
disposition in `00_nucleo/diagnostics/retired-nonowners-2026-10-18.md`.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/package_spec.md`. Historical observations are not a new parity certification.
