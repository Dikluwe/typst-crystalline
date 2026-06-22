# Prompt L0 — `Version` — número de versão semântico
Hash do Código: f597c1e8

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/version.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 401 — modelagem de `Value::Version` (S); último tipo S puro da série após abertura do portão ADR-0017.
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (constructor/comparações scope-out).

---

## 1. Contexto

O Typst vanilla expõe `version` como tipo de versão semântica (`major.minor.patch[-pre][+build]`). Em cristalino, o variant `Value::Version` estava ausente por causa do enum fechado (ADR-0017). Com o portão aberto e `Decimal`/`Duration` modelados, fecha-se a série com `Version`.

## 2. Tipo L1

```rust
// 01_core/src/entities/version.rs
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Vec<EcoString>,   // prerelease identifiers
    pub build: Vec<EcoString>, // build metadata
}
```

- `Clone` (não `Copy`) devido aos `Vec<EcoString>`.
- Zero I/O; alloc mínimo (vectores pequenos).
- Semântica semver 2.0.0: comparação ignora `build`; prerelease < release.

## 3. Construtores e acesso

```rust
impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self;
    pub fn with_pre(mut self, pre: Vec<EcoString>) -> Self;
    pub fn with_build(mut self, build: Vec<EcoString>) -> Self;
    pub fn to_string(&self) -> String;
    pub fn from_str(s: &str) -> Option<Self>;
}

impl Default for Version {
    fn default() -> Self { Self::new(0, 0, 0) }
}
```

## 4. Comparação semver

Implementar `PartialOrd` + `Ord` conforme semver 2.0.0:
- `major` → `minor` → `patch`.
- Sem prerelease > com prerelease.
- Identificadores numéricos comparados numericamente; não-numéricos lexicograficamente; numérico < não-numérico.
- `build` ignorado na comparação.

## 5. Variant `Value::Version`

```rust
// 01_core/src/entities/value.rs
Version(Arc<crate::entities::version::Version>),
```

- `Arc` para cheap clone (paridade pattern P395 `Value::Tiling`).
- `type_name()` → `"version"`.
- `cast_version()` extrai `Arc<Version>` ou converte `Str` (parse).
- `From<Version> for Value`.

## 6. Cast

- `Value::Version(v) -> Arc<Version>`: identidade.
- `Value::Str(s) -> Arc<Version>`: parse semver (`Version::from_str`).
- Outros tipos → `None`.

## 7. Repr

Formato canónico `"major.minor.patch[-pre][+build]"` (ex.: `"1.2.3-alpha.1+build.2"`).

## 8. Scope-out

- `native_version(...)` stdlib — passo futuro (S).
- Operações de comparação em stdlib (`>=`, `<`, etc.) — scope-out ADR-0054 graded.
- Field access (`version.major`) — scope-out ADR-0054 graded.
- Cast de `Array`/`Int`/`Float` → `Version` — não existe no vanilla.
