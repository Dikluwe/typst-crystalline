# Prompt L0 — `Version` — sequência de componentes inteiros
Hash do Código: 2bb18cd1

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

## 8a. Método `.at(index)` — P796

Medido no vanilla (`lab/typst-original/crates/typst-library/src/foundations/version.rs:109-134`,
sonda directa em 0.15.0: `#sys.version.at(0)` com `sys.version == version(0, 15, 0)` → `0`,
paridade com `major`).

```rust
impl Version {
    /// Índice negativo conta a partir do fim da lista de componentes
    /// **explícita** (`self.components.len()`, não a sequência infinita
    /// zero-pad usada por `component`/`Ord`/`Eq`). Índice positivo além do
    /// comprimento explícito devolve `0` (zero-pad, igual a `component`).
    /// Índice negativo fora de limites é erro.
    pub fn at(&self, index: i64) -> Result<i64, String>;
}
```

- Resolução: `index < 0` → `len_explícito.checked_add(index)`; se `< 0` ou overflow,
  erro. `index >= 0` (já resolvido ou originalmente não-negativo) → `component(i)`
  (0 se além do comprimento — reaproveita a semântica já existente).
- **Mensagem de erro replica o vanilla ao carácter** (ADR-0108, excepção —
  mecânica é o observável em mensagens de erro):
  `"component index out of bounds (index: {index}, len: {len})"`, onde `len` é
  `self.components.len()` (comprimento explícito, não zero-pad).
- Exposto como método de instância (`sys.version.at(0)`), não como field access —
  dispatch dedicado em `01_core/src/engine/eval/bindings.rs`
  (`eval_version_method_value`, mesmo padrão de `eval_counter_method_value`/
  `eval_color_method`), interceptado em `closures.rs` antes do fallback
  genérico de field access. `has_readonly_method` já antecipava isto
  (`Value::Version(_), "at" => true`, P716).
- Sem argumentos nomeados (vanilla não tem `default:` em `version.at`, ao
  contrário de `array.at`/`dict.at`). Argumento posicional não-inteiro ou
  aridade errada → erro genérico existente (`expected integer, found …` /
  `version.at() requires exactly one positional argument`), sem paridade
  literal exigida (mecânica de argparsing, não observável de linguagem).

## 8b. Exibição em markup (`#sys.version`) — P796

**Achado (P786)**: `#sys.version` interpolado em markup mostrava
`version(0, 15, 0)` (o `repr()`) em vez de `0.15.0` (o `Display`/`to_string()`).
Causa: `value_to_display_content` (`01_core/src/engine/eval/mod.rs`) não tinha
armo dedicado para `Value::Version` e caía no catch-all `other` que usa
`repr_value`.

Medido no vanilla (`foundations/value.rs::Value::display`, linha ~200):
`Self::Version(v) => TextElem::packed(eco_format!("{v}"))` — usa o `Display`
do tipo (equivalente a `Version::to_string()` no cristalino), **não** o repr.
`repr(sys.version)` continua a mostrar `version(0, 15, 0)` (repr não muda).

Correção: `value_to_display_content` ganha um armo dedicado, **antes** do
catch-all:

```rust
Value::Version(v) => {
    let text = v.to_string();
    if text.is_empty() { None } else { Some(Content::Text(text.into())) }
}
```

(`version()` vazia → `to_string()` = `""` → `None`, mesmo tratamento que os
outros armos de texto vazio nesta função.)

## 9. Constante de paridade partilhada — P796

`PARITY_VERSION: (u64, u64, u64) = (0, 15, 0)` — a versão **de paridade** com
a linguagem Typst (não a versão do crate/binário cristalino, ver decisão em
`engine/stdlib/sys.md` §"Decisão: versão de paridade"). Definida **aqui**
(`01_core/src/entities/version.rs`, `pub const PARITY_VERSION`) como fonte
única, consumida por:

- `01_core/src/engine/stdlib/sys.rs` (`sys.version`) — já usava este valor
  localmente antes de P796; passa a importar a constante em vez de duplicá-la.
- `02_shell/src/cli.rs` (`--version` do CLI) — P796 fecha a divergência
  registada no achado de P786 (`--version` mostrava `0.1.0`, a versão do
  crate Cargo, inconsistente com `sys.version` == `0.15.0`). Ver decisão e
  mecanismo completo em `shell/cli.md` §"Decisão — número de versão do CLI".

## 10. Scope-out

- Nenhum item pendente conhecido após P796 (`.at()` e exibição em markup
  fechados; ver §8a/§8b).
