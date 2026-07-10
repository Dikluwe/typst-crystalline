# Prompt L0 — `Version` — sequência de componentes inteiros
Hash do Código: 970fa7c3

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/version.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 401 — modelagem de `Value::Version`. **Corrigido em P684** — a
leitura original (semver 2.0.0 com `pre`/`build`) foi uma confusão com o SemVer
geral; o `version` do Typst é outro conceito (ver nota de origem no diagnóstico
de P684).
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029
(pureza L1), ADR-0108 (medir antes de decidir).

---

## 1. Contexto

O Typst `version` (documentação oficial + sonda directa em `0.15.0 (969087ec)`)
é **uma sequência arbitrária de componentes inteiros**; só os três primeiros têm
nome (`major`, `minor`, `patch`). Não há pré-lançamento nem metadados de build
como texto. Confirmado por sonda:

- `version()`, `version(1)`, `version(1, 2)`, `version(1, 2, 3, 4, 5)` → todos aceites.
- `version(1, 2, 3, pre: "alpha")` → `unexpected argument: pre`.
- `version(1, 2, 3, "alpha.1")` → `expected integer or array, found string`.
- `version(1, 2, 3) == version(1, 2, 3, 0)` → `true` (componente em falta = `0`).
- `repr(version(0, 11, 0))` → `"version(0, 11, 0)"` (repr preserva zeros à direita).

## 2. Tipo L1

```rust
// 01_core/src/entities/version.rs
#[derive(Debug, Clone)]
pub struct Version {
    pub components: Vec<u64>, // guardados como foram dados (zeros à direita preservados)
}
```

- `Clone` (não `Copy`) devido ao `Vec<u64>`. Zero I/O.
- `PartialEq`/`Eq`/`Hash` e `Ord` implementados **à mão** com semântica
  **zero-pad** (componente em falta = `0`), para reproduzir o vanilla:
  - `eq`: compara `component(i)` (0 se ausente) para `i` em `0..max(len)`.
  - `hash`: ignora zeros à direita (consistente com `eq`).
  - `cmp`: lexicográfico zero-pad sobre `0..max(len)`.
- `repr` usa `components` como dados (preserva zeros à direita).

## 3. Construtores e acesso

```rust
impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self;
    pub fn from_components(components: Vec<u64>) -> Self;
    pub fn component(&self, i: usize) -> u64; // 0 se ausente
    pub fn major(&self) -> u64;  // component(0)
    pub fn minor(&self) -> u64;  // component(1)
    pub fn patch(&self) -> u64;  // component(2)
    pub fn to_string(&self) -> String;        // "c0.c1.c2…" (vazio para version())
    pub fn from_str(s: &str) -> Option<Self>; // só inteiros separados por '.'
}
impl Default for Version { fn default() -> Self } // version() vazia
```

`from_str` rejeita string vazia, não-inteiros e negativos; **não** aceita
`pre`/`build` textuais (`"1.2.3-alpha.1"`, `"1.2.3+build.2"` → `None`).

## 4. Comparação (zero-pad, paridade vanilla)

- Igualdade directa sobre todos os componentes com zero-pad (sem qualquer
  tratamento especial de `pre`/`build`, que não existem).
- Ordem lexicográfica zero-pad: mais curto (prefixo) < mais longo;
  componente a componente; trailing zero não altera a ordem.

## 5. Variant `Value::Version`

```rust
// 01_core/src/entities/value.rs
Version(Arc<crate::entities::version::Version>),
```

- `Arc` para cheap clone (padrão P395).
- `type_name()` → `"version"`.
- `cast_version()` extrai `Arc<Version>` ou converte `Str` (parse int-only).
- `From<Version> for Value`.

## 6. Cast

- `Value::Version(v) -> Arc<Version>`: identidade.
- `Value::Str(s) -> Arc<Version>`: parse de componentes inteiros (`Version::from_str`).
- Outros tipos → `None`.

## 7. Repr

`version({componentes separados por ", "})`, tal como dados (ex.:
`version(1, 2, 3, 4, 5)`, `version(0, 11, 0)`, `version()` se vazio).

## 8. Field access

Só os três primeiros componentes têm nome: `.major`, `.minor`, `.patch` →
`component(0|1|2)` (0 se ausente). `.pre`/`.build` → `campo desconhecido em version`.

## 9. Scope-out

- `.at(i)` / métodos de array sobre `version` — não implementado (o vanilla expõe
  componentes via `.at`; fora do scope de P684).
