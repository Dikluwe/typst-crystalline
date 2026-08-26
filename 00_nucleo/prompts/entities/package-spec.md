# Prompt L0 — `entities/package_spec`
Hash do Código: 765e500c

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/package_spec.rs`
**Criado em**: 2026-03-22 (Passo 3 — PackageSpec DTO pattern)
**Atualizado em**: 2026-04-12 (restauro — expandido com `VersionBound`, `VersionlessPackageSpec` e validação via `is_ident`)
**ADRs relevantes**: ADR-0005 (PackageSpec DTO; `serde` nunca em L1)

---

## Contexto e Objetivo

A linguagem Typst permite importar pacotes externos com o formato
`@namespace/nome:versão` (ex: `@preview/algo:0.1.0`). Este módulo encapsula
a **validação e parsing** desse identificador no domínio (L1).

O parsing usa o `Scanner` de L1 (do módulo `rules::lexer`) para não depender
de crates de parsing externas. A validação de nomes usa `is_ident` do lexer,
que define exatamente o que é um identificador válido no Typst.

> **Fonte de paridade (P1031)** — vanilla ratificado (`e0e8ca4d`),
> `crates/typst-syntax/src/package.rs`. As regras deste L0 são **citação literal** do
> parser do vanilla, incluindo o uso de `is_ident` e do `Scanner` — não é só analogia:
>
> - **Estrutura do identificador** — `package.rs:224-232`: `/// Identifies a package.` sobre
>   `pub struct PackageSpec { pub namespace: EcoString, pub name: EcoString, pub version:
>   PackageVersion }`; o `Display` da forma sem versão é `write!(f, "@{}/{}", …)`
>   (`package.rs:310`). Confirma o formato `@namespace/nome:versão`.
> - **`@` obrigatório** — `package.rs:314-317`:
>   `if !s.eat_if('@') { Err("package specification must start with '@'")?; }`
> - **Namespace: não vazio e identificador válido** — `package.rs:319-324`:
>   `if namespace.is_empty() { Err("package specification is missing namespace")? } else if
>   !is_ident(namespace) { Err(eco_format!("`{namespace}` is not a valid package namespace"))? }`
> - **Nome: idem** — `package.rs:329-338`, com
>   `` "`{name}` is not a valid package name" ``.
> - **Versão obrigatória** — `package.rs:344-350`:
>   `if version.is_empty() { Err("package specification is missing version")?; }`
>
> O vanilla usa **o mesmo `is_ident`** (`package.rs:12`: `use crate::is_ident;`) e **o mesmo
> `Scanner`**, pelo que a escolha deste L0 é convergência, não coincidência.
>
> **Medição de confirmação (2026-08-13)** — `#import "@preview/example"` (sem versão):
> vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) e cristalino
> `target/release/typst` (fonte em HEAD `4f64e4e69`) dão ambos
> `error: package specification is missing version` na **mesma coluna (8)**. Mensagem de
> erro é caso em que a mecânica **é** o observável (ADR-0108) — e aqui coincide à letra.

**ADR-0005**: `serde` nunca entra em L1. Serialização de `PackageSpec` para
JSON/TOML acontece em L3 via DTO padrão
(`03_infra/dto/package_spec_dto.rs`).

Origem: `lab/typst-original/crates/typst-syntax/src/package.rs`

---

## Restrições Estruturais

- Camada **L1**: zero I/O. Usa `Scanner` de `rules::lexer` (também L1).
- Sem dependências externas (`serde`, `semver`, etc. são proibidos aqui).
- Erros de parse retornam tipos de erro próprios (`PackageSpecError`,
  `PackageVersionError`) — sem `anyhow`.
- Todos os tipos derivam `Clone + Debug + PartialEq + Eq + Hash`.
- `PackageSpec` e `PackageVersion` derivam também `PartialOrd + Ord` para
  ordenação léxica em resolução de dependências.
- `PackageVersion` deriva `Copy` (três `u32` — barato copiar).

---

## Instrução

### Tipos públicos

```rust
/// Identifica um pacote Typst pelo namespace, nome e versão.
/// Formato: @namespace/name:major.minor.patch
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageSpec {
    pub namespace: String,
    pub name:      String,
    pub version:   PackageVersion,
}

impl PackageSpec {
    /// Remove a versão para obter um VersionlessPackageSpec.
    pub fn versionless(&self) -> VersionlessPackageSpec
}

impl fmt::Display for PackageSpec  // → "@namespace/name:major.minor.patch"
impl FromStr      for PackageSpec  // valida namespace e name via is_ident(...)

// —————————————————————————————————————————————————————————————

/// Identifica um pacote sem especificar a versão (para lookup).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VersionlessPackageSpec {
    pub namespace: String,
    pub name:      String,
}

impl VersionlessPackageSpec {
    pub fn at(self, version: PackageVersion) -> PackageSpec
}

impl fmt::Display for VersionlessPackageSpec  // → "@namespace/name"

// —————————————————————————————————————————————————————————————

/// Versão semântica com componentes major, minor, patch.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl fmt::Display for PackageVersion  // → "major.minor.patch"
impl FromStr      for PackageVersion  // rejeita < 3 ou > 3 componentes

// —————————————————————————————————————————————————————————————

/// Erro retornado pelo parsing de PackageSpec.
#[derive(Debug)]
pub struct PackageSpecError(pub String);
impl fmt::Display for PackageSpecError
impl std::error::Error for PackageSpecError

/// Erro retornado pelo parsing de PackageVersion.
#[derive(Debug)]
pub struct PackageVersionError(pub String);
impl fmt::Display for PackageVersionError
impl std::error::Error for PackageVersionError

// —————————————————————————————————————————————————————————————

/// Limite de versão para compatibilidade (wildcard nos componentes ausentes).
/// major é obrigatório; minor e patch são opcionais.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VersionBound {
    pub major: u32,
    pub minor: Option<u32>,
    pub patch: Option<u32>,  // apenas se minor estiver definido
}

impl VersionBound {
    /// Verifica se uma versão satisfaz este limite.
    /// Componentes ausentes no bound funcionam como wildcard.
    pub fn matches(&self, v: &PackageVersion) -> bool
}

impl fmt::Display for VersionBound  // → "major[.minor[.patch]]"
```

### Regras de validação do `FromStr` para `PackageSpec`

1. Deve começar com `@`
2. `namespace`: até `/` — validado via `is_ident()` do lexer
3. `name`: até `:` — validado via `is_ident()` do lexer
4. `version`: o restante — parseable como `PackageVersion`
5. Qualquer campo em falta → `PackageSpecError`

---

## Critérios de Verificação

```
"@preview/algo:0.1.0".parse::<PackageSpec>() = Ok(PackageSpec {
    namespace: "preview", name: "algo",
    version: PackageVersion { major: 0, minor: 1, patch: 0 }
})

PackageSpec { .. }.to_string() = "@preview/algo:0.1.0"
PackageVersion { 1, 2, 3 }.to_string() = "1.2.3"

// Erros de parsing
"algo:0.1.0".parse::<PackageSpec>()   → Err (falta @)
"@/algo:0.1.0".parse::<PackageSpec>() → Err (namespace vazio)
"@preview/:0.1.0".parse::<PackageSpec>() → Err (name vazio)
"@preview/algo".parse::<PackageSpec>()   → Err (falta versão)

"1.2.3".parse::<PackageVersion>()   = Ok(PackageVersion { 1, 2, 3 })
"1.x.3".parse::<PackageVersion>()   → Err (componente não numérico)
"1.2".parse::<PackageVersion>()     → Err (menos de 3 componentes)
"1.2.3.4".parse::<PackageVersion>() → Err (mais de 3 componentes)

// Ordenação
PackageVersion { 1, 0, 0 } < PackageVersion { 2, 0, 0 }

// VersionBound — wildcard
VersionBound { major: 1, minor: Some(2), patch: None }
    .matches(PackageVersion { 1, 2, 5 }) = true
    .matches(PackageVersion { 1, 3, 0 }) = false

// VersionlessPackageSpec
spec.versionless().to_string() = "@preview/algo"
vl.at(PackageVersion { 0, 1, 0 }) = PackageSpec { .. }
```

---

## Resultado Esperado

- `01_core/src/entities/package_spec.rs` com todos os tipos documentados acima
- Testes co-localizados em `#[cfg(test)]` cobrindo os critérios acima
- Cabeçalho de linhagem apontando para este ficheiro
  (`@prompt 00_nucleo/prompts/entities/package-spec.md` ou conforme o path existente)

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 3: PackageSpec DTO pattern sem serde em L1 | `package_spec.rs` |
| 2026-04-12 | Restauro — expandido com `VersionlessPackageSpec`, `VersionBound`, regras de validação via `is_ident` e critérios completos | `package-spec.md` (na raiz de prompts e em entities/) |
