# Relatório do passo P206C

**Data de execução**: 2026-05-08.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-206C.md`.
**Natureza**: implementação cross-modular (helper L3 +
vanilla CLI invocation + comparação estrutural).
**Sub-passo `C` da série P206** — terceiro de 5 (A–E).
**Magnitude planeada**: M (com ressalva L se Caminho A
inflasse).
**Magnitude real**: **M** (~1.5h; 4 ficheiros novos
código + 1 L0 prompt + 2 ficheiros modificados +
3 outputs documentais; **`P206C.div-1` registado**).

---

## §1 O que foi feito

P206C materializou comparação estrutural cristalino vs
vanilla via JSON shape compatível com `typst query`
per Caminho B fixado em C2:

- **Helper L3** `03_infra/src/query_helpers.rs` (novo;
  hash `51294329`) — `query_to_summary(world, source,
  selector) -> QuerySummary` + parsing de selector
  (Kind names + label syntax `<...>`) + dispatch a
  `Introspector::query_*`.
- **L0 prompt** `00_nucleo/prompts/infra/query-helpers.md`
  (novo; hash `c7ea6387`).
- **Helper vanilla** `lab/parity/src/vanilla_invoke.rs`
  — `run_typst_query` via `std::process::Command` +
  `vanilla_cli_available` guard.
- **Helper comparação** `lab/parity/src/structural_compare.rs`
  — `compare_query_outputs(cristalino, vanilla) ->
  CompareResult` com count + label + metadata
  comparison + tolerância estruturada.
- **Tests parameterized** `lab/parity/tests/structural_parity.rs`
  — corpus completo 36 ficheiros (3 SKIP pre-existentes
  + 10 SKIP feature + 23 INCLUDE testados).

### Caminho escolhido: B (helper em 03_infra)

C2 fixou Caminho B per evidência empírica (C1.6 vs
C1.7):

- **Caminho A** (subcomando CLI em `04_wiring/`):
  rejeitado. Estimativa L (~3-5h). Acima do orçamento
  P206 série (M agregado per P206A C10). Exigia
  refactor cross-modular `main.rs` + Selector::Label
  extension em L1 + JSON shape vanilla replication +
  L0 prompts updated.
- **Caminho B** (helper em 03_infra; cristalino expõe
  helper via API L3): fixado. Estimativa M (~1.5h).
  Satisfaz "cristalino expõe helper" sem refactor
  desproporcional.
- **Caminho C** (helper em lab/parity quarentena):
  rejeitado por contradizer clarificação inicial
  ("cristalino expõe helper" implica workspace
  cristalino).

### `P206C.div-1` registado

Resolução parcial da clarificação inicial:
- "Novo CLI cristalino" satisfeito **parcialmente** via
  helper L3 público (cristalino expõe API).
- Subcomando CLI literal **deferred** para sub-passo
  dedicado pós-P206 — não contradiz clarificação;
  honra parcialmente.
- Fundamento empírico: Caminho A custo desproporcional
  (3-5h) face a orçamento série (M agregado).

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-206C-inventario.md`.

Conteúdo:
- §1 C1 inventário (7 sub-secções: 5 CONFIRMADO + 2
  AJUSTE NECESSÁRIO).
- §2 C2 forma do helper (Caminho B fixado).
- §3 C3 resolução parcial via `P206C.div-1`.
- §4 C4-C5 helpers literais (vanilla_invoke +
  structural_compare).
- §5 C6 tabela 36 ficheiros com etiquetas.
- §6 10 decisões durante a leitura (D1-D10).
- §7 métricas.

Tamanho: ~13 KB.

### Output 2 — Relatório (este ficheiro)

### Output 3 — Alterações em código

#### Ficheiros novos

- **`03_infra/src/query_helpers.rs`** (~290 LOC com
  tests) — helper L3 com:
  - `ParsedSelector` enum (Kind + Label).
  - `QuerySummary` struct domain-level (sem serde).
  - `SelectorKind` enum (discriminador).
  - `QueryError` enum (EvalFailed/NoContent/InvalidSelector).
  - `parse_selector(s) -> Result<ParsedSelector, QueryError>`.
  - `summarize_query(intr, parsed, raw) -> QuerySummary`.
  - `query_to_summary(world, source, selector) -> Result<QuerySummary, QueryError>` (entry point).
  - `value_plain_text(v) -> String` (helper privado).
  - 13 tests unit (9 parse + 4 query end-to-end via
    SystemWorld).
- **`00_nucleo/prompts/infra/query-helpers.md`** (~5
  KB; hash `c7ea6387`) — L0 prompt per Protocolo de
  Nucleação.
- **`lab/parity/src/vanilla_invoke.rs`** (~95 LOC) —
  helper de invocação vanilla CLI.
- **`lab/parity/src/structural_compare.rs`** (~270 LOC
  com 7 tests inline) — helper de comparação JSON.
- **`lab/parity/tests/structural_parity.rs`** (~250
  LOC) — tests parameterized:
  - `p206c_corpus_estrutural_36_ficheiros` — itera
    corpus completo; produz matriz markdown via
    eprintln!.
  - `p206c_query_simple_heading` — smoke e2e.
  - `p206c_query_metadata_values_e2e` — smoke metadata.

#### Ficheiros modificados

- **`03_infra/src/lib.rs`** — `pub mod query_helpers;`
  adicionado (1 line).
- **`lab/parity/Cargo.toml`** — `serde_json = "1"`
  adicionado a `[dev-dependencies]` (1 line).

#### Ficheiros docs modificados

- **`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`**
  — §P206C anotado `✅ MATERIALIZADO 2026-05-08` +
  sumário literal incluindo achados empíricos.

---

## §2 Tempo de execução

~1.5h efectivos:

- ~10 min: leitura da spec + setup TaskList + contexto
  P206A/P206B.
- ~25 min: C1 inventário empírico (7 sub-secções; smoke
  tests vanilla query + grep cristalino + análise de
  Selector + ElementKind APIs).
- ~5 min: C2-C3 fixação Caminho B + `P206C.div-1`
  documentação.
- ~30 min: C4 + L0 prompt `query-helpers.md` + helper
  L3 `query_helpers.rs` (parse + summarize + entry
  point + 13 tests).
- ~15 min: vanilla_invoke + structural_compare em
  lab/parity (~365 LOC + 7 tests).
- ~15 min: structural_parity.rs tests parameterized +
  matriz corpus completo (~250 LOC).
- ~5 min: C8-C11 validação (`crystalline-lint --fix-hashes`
  + cargo test workspace + cargo test lab/parity + lint).
- ~5 min: ADR-0075 §P206C anotação.
- ~5 min: outputs documentais (este + inventário).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Caminho fixado | **B** (helper em 03_infra) |
| `P206C.div-N` | **1** (div-1: CLI deferred) |
| Tests workspace cristalino antes | 1860 |
| Tests workspace cristalino depois | **1873** (∆+13 query_helpers) |
| Tests lab/parity antes | 54 |
| Tests lab/parity depois | **64** (∆+10 P206C) |
| Tests P206C novos | 23 (13 query_helpers + 7 structural_compare unit + 2 e2e + 1 corpus) |
| Linter violations | 0 (sem alteração) |
| Linter warnings | 0 substantivos (1 warning "Skip variant unused" em structural_compare — preserved para futuras extensões) |
| Ficheiros código novos | 5 (4 .rs + 1 L0 .md) |
| Ficheiros código modificados | 2 (`03_infra/src/lib.rs`; `lab/parity/Cargo.toml`) |
| Ficheiros docs novos | 2 (inventário + este relatório) |
| Ficheiros docs modificados | 1 (ADR-0075 §P206C) |
| LOC novas (código) | ~700 (290 query_helpers + 95 vanilla_invoke + 270 structural_compare + 250 structural_parity) |
| LOC novas (docs) | ~2000+ (L0 + inventário + relatório + ADR patch) |
| Cargo deps adicionados | 1 (`serde_json` em lab/parity dev-deps) |
| Refactor mid-execution | 0 |

### Tests por crate (workspace cristalino)

- `typst_core` unit: 1584 (sem alteração).
- `typst_infra` unit: 24 (sem alteração no lib unit).
- `typst_infra` integration: 229 → **242** (+13
  query_helpers tests).
- `typst_shell` unit: 21.
- `typst_wiring` unit: 2.
- **Total workspace**: 1860 → **1873** (+13).

### Tests lab/parity

- `parse_parity`: 50 (sem alteração).
- `eval_parity`: 1.
- `layout_parity`: 1.
- `vanilla_cli_smoke` (P206B): 2.
- `structural_parity` (P206C novo): **10** (7 unit
  structural_compare + 2 e2e + 1 corpus).
- **Total lab/parity**: 54 → **64** (+10).

---

## §4 Decisões

### D1 — Caminho B fixado por evidência

C1.6 mostrou Caminho A magnitude L (3-5h); Caminho B é
M (1.5h). Caminho C contradiz clarificação. Decisão B é
honesta: respeita orçamento série; satisfaz "cristalino
expõe helper" via API L3 público; honra clarificação
parcialmente com `P206C.div-1` documentando deferral.

### D2 — `from_name` (não `from_str`)

`ElementKind` API correcta é `from_name(s: &str) ->
Option<Self>` — não `from_str`. Fix trivial após primeira
compilation error.

### D3 — Tests usam SystemWorld end-to-end

`LabelRegistry::add`, `MetadataStore::add`,
`Location::from_raw` são `pub(crate)` — não acessíveis
de L3 tests. Solução: tests usam `query_to_summary`
end-to-end com SystemWorld + source minimal. Pattern
paralelo a `03_infra/src/integration_tests.rs`.

### D4 — `Selector` L1 NÃO estendido

P175 minimal preservado. Helper P206C tem `ParsedSelector`
próprio que dispatcha directamente a `query_by_kind` ou
`query_by_label`. L1 inalterado.

### D5 — `serde_json` em lab/parity dev-deps

Adicionar `serde_json` a 03_infra exigiria justificação
ADR (allowlist deps). Lab/parity é quarentena; sem ADR
exigido. **Domain struct** `QuerySummary` (sem
serde) preserva L3 clean; lab/parity converte JSON.

### D6 — Divergências empíricas documentadas, não fixadas

Matriz revelou:
- `equation` selector vanilla rejeita ("unknown
  variable") — vanilla usa `math.equation` namespace.
- `cite-bibliography.typ` cristalino eval falha
  (bibliography stdlib parcial).
- `outline-toc.typ` heading diff (TOC entries
  contadas).

Estas são divergências arquitectónicas legítimas, não
regressões. Documentadas em ADR-0075 §P206C. Fix
exigiria stdlib expansion ou refactor — fora-de-escopo
P206C.

### D7 — Matriz sem assert global de match

Pattern existente (eval_parity, layout_parity): paridade
é medição via `eprintln!`, não verificação via
`assert!`. `p206c_corpus_estrutural_36_ficheiros`
preserva pattern; reporta sem falhar.

### D8 — Tempdir helper duplicado (deferred refactor)

Pattern `TempDir` + `tempdir()` repetido em 4 test files
(eval, layout, vanilla_cli_smoke, structural_parity).
Refactor para shared helper seria lab/parity-wide
cleanup, fora-de-escopo P206C. Pattern consistente.

### D9 — `crystalline-lint --fix-hashes` automatiza
sincronização

L0 prompt + L1 file foram criados com placeholder
`ZZZZZZZZ`; `crystalline-lint --fix-hashes` calculou e
sincronizou hash automaticamente. Pattern padrão per
P204 série.

### D10 — `P206C.div-1` é divergência cosmética

Caminho B satisfaz intenção da clarificação ("cristalino
expõe helper") via API L3. Subcomando CLI literal
deferred. Per spec C3 "Caminho B é compromisso aceitável
(cristalino expõe helper, mas não como CLI)" — não
exige solicitar decisão ao humano. Documentação suficiente.

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §8 e cláusulas
de decisão:

| Hipótese | Resultado |
|----------|-----------|
| §8: "C2 = B é compromisso pragmático que satisfaz a intenção da clarificação" | **CONFIRMADA** — C1.6 vs C1.7 mostra B é proporcional |
| §8: "subestimar custo de novo CLI porque parece trivial" | **EVITADO** — C1.6 estimou L (~3-5h) com decomposição |
| §8: "inflar para honrar 'novo CLI' pré-fixado" | **EVITADO** — `P206C.div-1` documenta deferral honesto |
| §8: "escopo '36 ficheiros' inclui lab/typst-original/ corpus" | **REJEITADA** — corpus refere-se a `lab/parity/corpus/` apenas |
| C14: "se C1.6 mostrar custo XL, registar div-N" | **PARCIALMENTE** — C1.6 mostrou L (não XL); div-1 registado mesmo assim para honestidade |
| C2: "Caminho A é preferido pela clarificação" | **HONRADO PARCIALMENTE** — Caminho B com div-1 |

6 hipóteses resolvidas pela auditoria empírica.

---

## §6 Sugestão para próximo sub-passo

P206C fechado per C13 com todos os critérios cumpridos:

- ✓ C1 inventário completo (7 sub-secções).
- ✓ C2 forma do helper fixada (B) com justificação.
- ✓ C3 tensão resolvida via `P206C.div-1`.
- ✓ C4 helper de invocação vanilla aplicado.
- ✓ C5 helper de comparação JSON aplicado.
- ✓ C6 tabela de cobertura 36 ficheiros documentada.
- ✓ C7 tests parameterized (10 lab/parity novos).
- ✓ C8 compilação verde.
- ✓ C9 tests workspace cristalino 1873 verdes (+13).
- ✓ C10 tests lab/parity 64 verdes (+10).
- ✓ C11 linter 0 violations.
- ✓ C12 ADR-0075 §P206C anotada.
- ✓ Inventário registado.
- ✓ Relatório escrito (este ficheiro).

**Próximo sub-passo**: **P206D — Cobertura corpus 36 +
matriz consolidada** (per ADR-0075 plano de
materialização).

P206D é magnitude S-M (~45-60 min):

- Estender `corpus_completo_p3` para incluir vanilla
  side via helpers P206C.
- Update `ParityMatrix` (`lab/parity/src/report.rs`)
  para colunas `text_content / structural` populadas
  com dados P206C.
- Render matriz consolidada para 36 entradas com
  match/diff/skip per categoria.
- Sentinelas: 2-3 (matriz produzida; N entradas
  cobertas; cond 9 ADR-0073 cumprida estruturalmente).
- Outputs: 3 ficheiros (inventário + relatório +
  alterações de código).

Pré-condições confirmadas por P206C:
- Helper L3 `query_to_summary` funcional (13 tests
  verdes).
- Helpers vanilla_invoke + structural_compare
  reusáveis.
- 23 ficheiros corpus produzem comparação INCLUDE
  válida; 13 SKIP documentados.

---

## §7 Cross-references

- **Spec**: `00_nucleo/materialization/typst-passo-206C.md`.
- **Outputs P206C**:
  - `00_nucleo/diagnosticos/typst-passo-206C-inventario.md`.
- **L0 prompt novo**:
  `00_nucleo/prompts/infra/query-helpers.md` (hash
  `c7ea6387`).
- **ADR**:
  `00_nucleo/adr/typst-adr-0075-vanilla-integration.md`
  (§P206C ✅ MATERIALIZADO 2026-05-08).
- **Predecessores**:
  - P206B (harness reactivado; vanilla CLI smoke
    confirma pré-condições).
  - P206A (diagnóstico-primeiro; ADR-0075 PROPOSTO).
- **Sucessor planeado**: P206D (matriz consolidada +
  cobertura completa).
- **Pendências endereçadas**:
  - DEBT-53 (vanilla integration): progresso material
    significativo (helper L3 + comparação estrutural).
  - `P204F.div-1`: matriz produz dados empíricos para
    documentar.
  - Cond 9 ADR-0073: progresso parcial (paridade
    observable estruturalmente confirmada parcialmente).
- **`P206C.div-1`** (novo): CLI subcomando deferred;
  Caminho B materializado; tensão resolvida
  parcialmente per spec C3.
- **Vanilla typst v0.14.2**:
  - Path dep: `lab/typst-original/crates/typst-syntax`.
  - Binary: `/usr/local/bin/typst v0.14.2 (b33de9de)`.
  - Subcomando `query` confirmado funcional via P206B
    smoke + P206C usage massivo.
- **Pattern referência**: P204G (módulo dedicado em L3
  com L0 prompt + tests inline; pattern paralelo).
