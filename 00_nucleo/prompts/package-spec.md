# PackageSpec — identificação de pacotes Typst

**Camada**: L1 — entities
**Criado em**: 2026-03-22
**Arquivos gerados**: `01_core/src/entities/package_spec.rs`

---

## Contexto

`PackageSpec` identifica um pacote Typst pelo seu namespace, nome e
versão. É um tipo de domínio puro — não tem I/O, não tem serialização,
não tem parsing de strings arbitrárias.

O código original em `lab/typst-original/crates/typst-syntax/src/package.rs`
usa `EcoString` nos campos e `unscanny` no `FromStr`. Em L1:
- `EcoString` → `String` (nomes de pacotes são curtos, sem ganho de Arc)
- `FromStr` com `unscanny` → fica em L3 (DTO pattern, ADR-0005)
- `FromStr` com stdlib pura → migra para L1 (`PackageVersion`, `VersionBound`)

`PackageManifest`, `PackageInfo`, `ToolInfo`, `TemplateInfo` são
desserialização de TOML — pertencem inteiramente a L3.

---

## Restrições Estruturais

- Zero dependências externas (apenas `std` e `thiserror`)
- Sem `#[derive(Serialize, Deserialize)]` em nenhum tipo
- `FromStr` apenas quando a implementação usa stdlib pura
- `EcoString` substituído por `String` em todos os campos

---

## Tipos a migrar

### `PackageSpec`

```rust
pub struct PackageSpec {
    pub namespace: String,
    pub name:      String,
    pub version:   PackageVersion,
}
```

Deriva: `Debug`, `Clone`, `Eq`, `PartialEq`, `Hash`, `Ord`, `PartialOrd`.
`Display`: `@{namespace}/{name}:{version}`.
`FromStr`: **não migra** — usa `unscanny`, fica em L3.

### `VersionlessPackageSpec`

```rust
pub struct VersionlessPackageSpec {
    pub namespace: String,
    pub name:      String,
}
```

Deriva: `Debug`, `Clone`, `Eq`, `PartialEq`, `Hash`, `Ord`, `PartialOrd`.
`Display`: `@{namespace}/{name}`.
`FromStr`: **não migra** — usa `unscanny`, fica em L3.

### `PackageVersion`

```rust
pub struct PackageVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
```

Deriva: `Debug`, `Copy`, `Clone`, `Eq`, `PartialEq`, `Hash`, `Ord`, `PartialOrd`.
`Display`: `{major}.{minor}.{patch}`.
`FromStr`: **migra** — usa apenas `str::split` e `parse::<u32>()`.
Erro: `PackageVersionError(String)` — sem dependências externas.

`compiler()` → **não migra**: chama `typst_utils::version()`, dependência externa.

### `VersionBound`

```rust
pub struct VersionBound {
    pub major: u32,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
}
```

Deriva: `Debug`, `Clone`, `Eq`, `PartialEq`, `Hash`.
`matches(version: &PackageVersion) -> bool`: verifica compatibilidade.
`FromStr`: **migra** — usa apenas stdlib.

---

## Critérios de Verificação

```
Dado PackageSpec { namespace: "preview", name: "algo", version: 0.1.0 }
Quando Display for chamado
Então retorna "@preview/algo:0.1.0"

Dado PackageVersion { major: 1, minor: 2, patch: 3 }
Quando Display for chamado
Então retorna "1.2.3"

Dado "1.2.3".parse::<PackageVersion>()
Quando bem formado
Então Ok(PackageVersion { major: 1, minor: 2, patch: 3 })

Dado "1.x.3".parse::<PackageVersion>()
Quando mal formado
Então Err(PackageVersionError)

Dado PackageVersion(1,0,0) e PackageVersion(2,0,0)
Quando comparados com Ord
Então 1.0.0 < 2.0.0

Dado VersionBound { major: 1, minor: Some(2), patch: None }
Quando matches(PackageVersion(1,2,5))
Então true

Dado VersionBound { major: 1, minor: Some(2), patch: None }
Quando matches(PackageVersion(1,3,0))
Então false

Dado PackageSpec com namespace e name iguais mas versão diferente
Quando comparados com Eq
Então não são iguais

Dado VersionlessPackageSpec::from(PackageSpec)
Quando convertido
Então perde a versão, preserva namespace e name
```

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação inicial — ADR-0005 (DTO pattern) | package_spec.rs |
