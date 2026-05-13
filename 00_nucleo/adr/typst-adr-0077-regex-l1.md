# ⚖️ ADR-0077: `regex` → `[l1_allowed_external]`

**Status**: **ACEITE** (transição PROPOSTO → ACEITE em P209E
2026-05-12 após verificação dos 8 critérios do §Plano de
validação).
**Data**: 2026-05-12 (PROPOSTO P209D; ACEITE P209E).

---

## Contexto

P209D (per ADR-0076 §Plano de materialização — M9c Bloco VI)
introduz `Selector::Regex(Regex)` como 5º variant do Selector
enum. Sub-mecanismo fixado em P209A C2 = **Caminho A** (`regex`
full Unicode), per paridade vanilla (vanilla usa `regex::Regex`
crate directamente em `Selector::Regex`).

Allowlist L1 actual (pre-P209D, `crystalline.toml:64` —
`l1_allowed_external.rust`):

```
thiserror, comemo, unicode_ident, unicode_math_class,
unicode_script, unicode_segmentation, rustc_hash, time,
indexmap, ecow, hypher
```

11 entries; **`regex` não está**. Adicionar a dep em L1 requer
ADR per política ADR-0029 (allowlist deps L1).

`regex` 1.x é o crate Rust standard para regex (features
Unicode default, ~1.5 MB binário). Alternativas:

- **`regex-lite`**: subset ASCII + limited Unicode; ~200 KB.
  Rejeitado por divergência de paridade vanilla (users
  esperam Unicode regex).
- **Sem regex em L1; pattern como string**: semântica
  degenerada — consumer não pode evaluar matches.

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — regex puro CPU/memória |
| Zero estado global mutável | ✓ — engine sem global state |
| Determinismo total | ✓ — pattern → DFA determinístico |
| Dependências transitivas | ✓ — `aho-corasick`, `memchr` (já comuns no ecossistema) |
| Compatibilidade Hash/Eq/Send/Sync | parcial — `regex::Regex` deriva `Send + Sync + Clone` mas **não** `Hash`/`Eq`; resolvido via wrapper L1 (`entities::regex::Regex`) com Hash/Eq manuais via pattern string |

`regex` é uma biblioteca pura de algoritmos (motor de DFA/NFA
sobre strings); sem I/O. Conceptualmente equivalente a outras
deps L1 (estruturas de dados / algoritmos sobre memory-only).

---

## Decisão

**`regex` 1.x autorizado em L1** com features default
(Unicode full):

```toml
# crystalline.toml:64
[l1_allowed_external]
rust = [
    "thiserror",
    "comemo",
    "unicode_ident",
    "unicode_math_class",
    "unicode_script",
    "unicode_segmentation",
    "rustc_hash",
    "time",
    "indexmap",
    "ecow",
    "hypher",
    "regex",        # ADR-0077 — Selector::Regex em L1 (M9c P209D)
]
```

```toml
# Cargo.toml workspace
regex = "1"         # ADR-0077 — Selector::Regex em L1; full Unicode

# 01_core/Cargo.toml
regex = { workspace = true }  # ADR-0077 — Selector::Regex em L1 (P209D / M9c)
```

**Wrapper L1 isolado**:
`01_core/src/entities/regex.rs` é o **único** consumer directo
do crate `regex` em L1. Hash/Eq/PartialEq/Clone/Debug manuais
via field `pattern: String`. Consumers transitivos
(`Selector::Regex`, futuros) importam `crate::entities::regex::Regex`,
não `regex::Regex` directo.

---

## Consequências

**Positivas**:
- Paridade vanilla regex syntax (Unicode classes, look-around,
  etc.).
- Crate Rust standard — users esperam.
- Pattern wrapper L1 permite swap futuro (`regex-lite` ou
  outro motor) sem refactor cross-modular.

**Negativas**:
- Binário +~1.5 MB por features Unicode default.
- Hash/Eq manuais no wrapper (Hash via pattern string;
  compilação assumida determinística).
- `Clone` no wrapper re-compila (custo amortizado mas não
  zero); alternativa `Arc<Regex>` adicionaria complexidade
  prematura — actualmente Selector::Regex Clone é raro
  (apenas em construção de And/Or compósitos).

**Neutras**:
- Dep transitiva `aho-corasick` (algoritmos multi-pattern)
  e `memchr` (SIMD search) já comuns no ecossistema Rust.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **`regex` full (escolhida)** | Paridade vanilla; Unicode completo; ergonomia | Binário ~1.5 MB |
| `regex-lite` | Binário ~200 KB; menos deps transitivas | Subset ASCII; sem `(?i)`, look-around, Unicode classes |
| Sem `regex` (pattern como string) | Sem dep nova | `Selector::Regex` semanticamente degenerado; consumer não pode evaluar |
| `regex` com `default-features = false` + features mínimas | Binário menor mantendo crate | Cherry-picking features é frágil entre versões; paridade vanilla incerta |

**Escolha**: `regex` full por paridade vanilla literal +
ergonomia para users. Custo binário é aceitável per
política ADR-0029 + padrão ADR-0023/0024 (deps L1
justificadas por valor de domínio).

---

## Prompts Afetados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/entities/regex.md` | NOVO — wrapper L1 (P209D) |
| `00_nucleo/prompts/entities/selector.md` | +variant `Regex(Regex)` em Interface; +query semantics stub (P209D) |

---

## Plano de validação

ADR-0077 é considerado MATERIALIZADO ESTRUCTURALMENTE quando:

1. `regex` aparece em `crystalline.toml:64` allowlist
   `l1_allowed_external.rust`.
2. `regex` aparece em `Cargo.toml` `[workspace.dependencies]`.
3. `regex` aparece em `01_core/Cargo.toml` `[dependencies]`.
4. `01_core/src/entities/regex.rs` existe com Hash/Eq
   manuais.
5. `Selector::Regex(Regex)` variant existe em
   `entities::selector`.
6. `cargo build --workspace` verde.
7. `crystalline-lint` 0 violations (sem V14
   `ExternalTypeInContract` — wrapper isola).
8. Tests `regex_*` (6+) em `entities::regex` verdes.

**Transição PROPOSTO → ACEITE**: em P209E (encerramento
série P209) **ou** P212 (encerramento M9c). Critério:
ausência de regressão durante materialização restante M9c.

---

## Cross-references

- **ADR-0076 PROPOSTO** (M9c Introspector completion) —
  §Plano P209D autoriza ADR-0077 nova.
- **ADR-0029** (allowlist deps L1 política) — fundamento
  para ADR-0077 ser ADR formal.
- **ADR-0023** (indexmap em L1) — template estrutural
  paralelo.
- **ADR-0024** (ecow em L1) — template estrutural paralelo.
- **P209A C2** (Caminho A `regex` full fixado).
- **P209A C1** (estrutura `Selector::Regex(Regex)` wrapper).
- **`entities/regex.md`** L0 do wrapper.
- **Vanilla `Selector::Regex(Regex)`**:
  `lab/typst-original/crates/typst-library/src/foundations/selector.rs:86`
  (referência para paridade structural).

---

## Histórico

| Data | Status | Notas |
|------|--------|-------|
| 2026-05-12 | PROPOSTO | Criada em P209D para autorizar dep `regex` em L1. |
| 2026-05-12 | ACEITE   | Transição em P209E após verificação empírica dos 8 critérios do §Plano de validação. Caminho A fixado em P209E C1.1 — ADR independente de ADR-0076 (escopo: dep `regex`, não marco M9c inteiro); sem dependência futura que possa invalidar. |
