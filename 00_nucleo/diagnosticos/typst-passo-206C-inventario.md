# P206C — Inventário interno (vanilla integration runtime + comparação estrutural)

**Data**: 2026-05-08.
**Spec**: `00_nucleo/materialization/typst-passo-206C.md`.
**Output 1 de 3** (inventário interno).
**Caminho fixado**: **B (helper em 03_infra)**.
**Divergência**: `P206C.div-1` (CLI subcomando deferred).

---

## §1 C1 — Inventário empírico (7 sub-secções)

### §1.1 C1.1 — Vanilla `typst query` CLI

**Status**: `CONFIRMADO`.

Smoke tests em corpus P204F:

```
$ typst query lab/parity/corpus/visual/heading-simples.typ heading --format json
[{"func":"heading","level":1,"depth":1,"offset":0,"numbering":null,
  "supplement":{"func":"text","text":"Section"},"outlined":true,
  "bookmarked":"auto","hanging-indent":"auto",
  "body":{"func":"text","text":"Título principal"}}]

$ typst query .../figure-ref.typ "<fig-alfa>" --format json
[{"func":"figure","body":{...},"label":"<fig-alfa>",...}]

$ typst query .../figure-ref.typ figure --format json
[{...},{...},{...}]  # 3 figures
```

Selectors aceitos:
- Nome de kind (sem `<>`): `heading`, `figure`, `metadata`,
  `bibliography`, etc.
- Label sintaxe `<label-name>`.
- **NÃO aceito** standalone: `equation` → "unknown
  variable" (vanilla usa `math.equation` namespace).
- Selectors complexos: `heading.where(level: 1)`.

Estrutura JSON:
- Array de elementos.
- Cada elemento: `func` (nome do kind), structural
  fields per kind (level, body, caption, label, etc.).
- Verbose com defaults preenchidos.

### §1.2 C1.2 — Cristalino consumers de `Introspector`

**Status**: `CONFIRMADO`.

Trait `Introspector` (após P204B `#[comemo::track]`):
- `query_by_kind(kind) -> Vec<Location>`.
- `query_by_label(label) -> Option<Location>`.
- `query_metadata() -> &[Value]`.
- `query(selector) -> Vec<Location>` (P175 minimal só
  `Selector::Kind`).
- 16 outros métodos para state/counters/etc.

Pipeline cristalino: `Selector` → `query` → `Vec<Location>`.
Caller extrai `Position`/`Content` per location.

**Sem serialização JSON existente** em workspace
cristalino. `Content` enum pode ser serializado via
Debug mas não via serde derive.

### §1.3 C1.3 — Cristalino CLI actual

**Status**: `CONFIRMADO` — sem subcomandos.

`04_wiring/src/main.rs` (per P204E):
- Binary-only.
- Aceita `<file.typ>` arg + flags.
- Produz PDF.
- **Sem `typst query` equivalente**.

Refactor para subcomando dispatch exigiria:
- Adicionar `enum Command { Compile, Query }` em
  `02_shell::cli`.
- Refactor `main.rs` para dispatch.
- Selector parsing + JSON serialization.
- Tests em 04_wiring.
- L0 prompts updated (wiring.md + cli.md em L2).

Magnitude estimada: **L** (~3-5h cross-modular).

### §1.4 C1.4 — Tipos cristalinos para serialização

**Status**: `CONFIRMADO + AJUSTE`.

- **`Selector` cristalino** (`01_core/src/entities/selector.rs`):
  enum com **apenas** `Kind(ElementKind)` (P175 minimal).
  Sem `Selector::Label`, `Where`, `And`, `Or`. Estender
  exige passo dedicado em L1.
- **`ElementKind`** (`01_core/src/entities/element_kind.rs`):
  10 variants; `as_str()` produz nome textual estável
  paralelo a vanilla (`heading`, `figure`, `metadata`,
  etc.); `from_name(s)` parsing inverso disponível.
- **`Content` enum**: P204B adicionou Hash impl manual
  para coexistência com `#[comemo::track]`; **sem serde
  derive** ou pattern de serialização declarado.
  Adicionar `Serialize` derive afectaria L1 com nova
  external dep — fora-de-escopo P206.
- **`Location`** (P179): `pub fn as_u128() -> u128`;
  serializável via Debug.

**Ajuste necessário**: cristalino não tem rota standard
para JSON output de query results. Helper P206C produz
**domain struct** (`QuerySummary`) sem serde — caller
(lab/parity) faz conversão JSON.

### §1.5 C1.5 — Lab/parity actual

**Status**: `AJUSTE NECESSÁRIO`.

`lab/parity/Cargo.toml` (per P206B):
- `[dependencies]`: typst-syntax, typst-core (sem
  serde_json).
- `[dev-dependencies]`: typst-syntax, typst-core,
  typst-infra, pretty_assertions, walkdir (sem
  serde_json).

**Adicionar `serde_json = "1"` a dev-dependencies**
(lab/parity é quarentena; sem ADR exigido). Done in
P206C.

`Command::new` pattern: zero usos pre-existentes em
03_infra; vanilla_cli_smoke (P206B) usa em test directo.
P206C generaliza para helper reusable.

### §1.6 C1.6 — Custo "novo CLI cristalino"

**Estimativa: L (~3-5h)**.

Decomposição:
- Refactor `04_wiring/main.rs` para subcommand dispatch:
  ~50 LOC.
- Cli args via `02_shell::cli`: ~50 LOC.
- Selector parsing (Kind + Label): ~30 LOC.
- Serialização JSON do output (cristalino Content):
  ~150-200 LOC se replicar shape vanilla; ~50 LOC se
  minimal (count + labels).
- Tests em 04_wiring + 02_shell: ~50-100 LOC.
- L0 prompts: `wiring.md` updated; possibly novo
  `cli.md` se subcommand expansion exigir.

**Magnitude L** confirmada. Caminho A acima do
orçamento P206 série (M agregado).

### §1.7 C1.7 — Custo "test helper interno" (Caminho B)

**Estimativa: M (~1-1.5h)**.

Decomposição:
- L0 prompt `infra/query-helpers.md`: ~5 KB documento.
- Helper `03_infra/src/query_helpers.rs`: ~150 LOC com
  tests inline (~80 unit + ~70 main).
- `03_infra/src/lib.rs` export: 1L.
- Helpers lab/parity (vanilla_invoke + structural_compare):
  ~250 LOC + ~100 LOC tests.
- Tests parameterized lab/parity: ~150 LOC.
- `lab/parity/Cargo.toml` serde_json dep: 1L.
- ADR anotação: 1 bloco.

**Magnitude M** confirmada. Dentro do orçamento.

---

## §2 C2 — Forma do helper: **Caminho B**

Decisão fixada: **Caminho B (helper em 03_infra)**.

Justificação literal (per C1.6 vs C1.7):

1. **Caminho A é L** (3-5h; refactor cross-modular).
   Acima do orçamento P206 série (M agregado per P206A
   C10).
2. **Caminho B é M** (~1-1.5h). Dentro do orçamento.
3. **Caminho C** (helper em lab/parity) **rejeitado** por
   contradizer clarificação inicial ("cristalino expõe
   helper") — lab/ é quarentena, não exposição
   cristalino.
4. Caminho B satisfaz a **intenção** da clarificação
   ("cristalino expõe helper") via API L3 público.
   Subcomando CLI literal pode ser sub-passo dedicado
   pós-P206 sem fricção (helper L3 reutilizado pelo
   subcomando hipotético).

---

## §3 C3 — Resolução parcial da tensão: `P206C.div-1`

**`P206C.div-1`** registado:

**Divergência**: clarificação inicial fixou "novo CLI
cristalino" (Caminho A); P206C C2 fixou Caminho B
(helper em workspace cristalino sem subcomando CLI
exposto).

**Fundamento empírico**:
- Caminho A custo ~3-5h vs Caminho B ~1.5h (per C1.6,
  C1.7).
- Caminho A exigia parsing de selector novo
  (`Selector::Label` + extension de Selector enum em
  L1) + refactor `main.rs` cross-modular + L0 prompts.
- Caminho B satisfaz "cristalino expõe helper" via API
  L3 reusável.

**Resolução**:
- Caminho B materializado.
- Subcomando CLI deferred para sub-passo dedicado
  pós-P206 (não contradiz clarificação; honra
  parcialmente).
- Sem necessidade de solicitar decisão ao humano (per
  spec C3 "Caminho B é compromisso aceitável") — divergência
  cosmética, não estrutural.

---

## §4 C4-C5 — Helpers literais

### §4.1 C4 — `vanilla_invoke.rs`

`lab/parity/src/vanilla_invoke.rs` (~95 LOC):

```rust
pub fn run_typst_query(typ_path: &Path, selector: &str)
    -> Result<serde_json::Value, VanillaInvokeError>;

pub fn vanilla_cli_available() -> bool;
```

`VanillaInvokeError` enum: `NotInstalled`, `CommandFailed`,
`JsonParseError`, `IoError`. Skip graceful via
`NotInstalled` quando binário ausente em PATH.

### §4.2 C5 — `structural_compare.rs`

`lab/parity/src/structural_compare.rs` (~270 LOC com 7
tests inline):

```rust
pub fn compare_query_outputs(
    cristalino: &QuerySummary,
    vanilla: &serde_json::Value,
) -> CompareResult;

pub enum CompareResult { Match, Diff(Vec<String>), Skip(String) }
```

Comparação:
- **Count** estrita.
- **Kind name** (Kind selector): vanilla `func` vs
  cristalino `kind_name`.
- **Label match** (Label selector): cristalino
  `label_found` vs vanilla `[0].label`.
- **Metadata values**: containment + tokens
  alfanuméricos para tolerar Dict shape diff.

`values_compatible(c, v)`: critério lax para Dict/Array
(Cristalino Debug vs vanilla nested JSON).

---

## §5 C6 — Tabela de cobertura 36 ficheiros

| # | Ficheiro | Etiqueta | Razão |
|---|----------|----------|-------|
| 1-2 | code/{let,set}.typ | **SKIP-feature** | Categoria fora-de-escopo P206C (sem elementos query-able típicos) |
| 3 | markup/empty.typ | INCLUDE | heading count=0 em ambos ✓ |
| 4 | markup/error.typ | **SKIP-pre-existing** | Sintaxe inválida intencional (P206A C2 + harness pre-existing skip) |
| 5 | markup/heading.typ | INCLUDE | heading count=2 ✓ match |
| 6-9 | markup/{parbreak,plain,spaces,strong}.typ | INCLUDE | heading count=0 ✓ match |
| 10-11 | math/{block,simple}.typ | INCLUDE | equation selector vanilla rejeita ("unknown variable equation"); divergência arquitectónica |
| 12-21 | semantic/* (10) | **SKIP-feature** | Categoria P2 eval, fora-de-escopo P206C (introspection) |
| 22 | visual/cite-bibliography.typ | INCLUDE-com-erro | cristalino eval falha (bibliography stdlib parcial); documentado |
| 23 | visual/counter-heading.typ | INCLUDE | heading count=5 ✓; figure count=0 ✓; metadata count=0 ✓ |
| 24 | visual/equation-ref.typ | INCLUDE-parcial | heading/figure/metadata ✓; equation vanilla rejeita |
| 25 | visual/figure-ref.typ | INCLUDE | heading=0 ✓; figure=3 ✓; metadata=0 ✓ |
| 26 | visual/heading-simples.typ | INCLUDE | heading=1 ✓ |
| 27 | visual/math-basico.typ | INCLUDE | heading/figure/metadata=0 ✓ |
| 28 | visual/multi-font.typ | INCLUDE | heading/figure/metadata=0 ✓ |
| 29 | visual/outline-toc.typ | INCLUDE-com-diff | heading count diff cristalino vs vanilla (TOC entries) |
| 30 | visual/paragrafo-justificado.typ | INCLUDE | heading/figure/metadata=0 ✓ |
| 31 | visual/query-metadata.typ | INCLUDE-com-diff | metadata values mostram diff (vanilla shape complexa) |
| 32-35 | visual/set-text-{bold,fill,size,tracking}.typ | INCLUDE | heading/figure/metadata=0 ✓ |
| 36 | visual/show-strong.typ | INCLUDE | heading/figure/metadata=0 ✓ |

**Sumário**: 36 corpus, 23 INCLUDE, 13 SKIP (3 pre-existing + 10 feature). Diffs
documentados em 3 ficheiros (math/* + outline-toc + query-metadata).

---

## §6 Decisões durante a leitura

### D1 — Caminho B fixado por evidência (não por preferência)

C1.6 mostrou Caminho A magnitude L (3-5h); orçamento
P206 série é M agregado. Caminho B é M (1.5h). Per
spec §8 hipótese "C2 = B é compromisso pragmático":
confirmada. `P206C.div-1` documenta resolução parcial.

### D2 — `from_name` em vez de `from_str`

Tentativa inicial usou `ElementKind::from_str` —
método não existe; nome correcto é `from_name`. Fix
trivial; lição: verificar API exact via grep antes de
usar.

### D3 — Tests de summarize_query usam SystemWorld
end-to-end, não construção directa de TagIntrospector

LabelRegistry/MetadataStore/Location têm constructors
`pub(crate)` — não acessíveis de L3 tests. Solução:
tests usam `query_to_summary` end-to-end com
SystemWorld + tempdir + source minimal. Pattern
paralelo a `03_infra/src/integration_tests.rs`.

### D4 — `Selector` em L1 não estendido

Per spec C7 + L0 prompt: P206C **não estende**
`Selector` enum em L1 (P175 minimal mantém-se). Helper
P206C tem **own ParsedSelector** enum (Kind+Label) que
dispatcha a `Introspector::query_by_kind` ou
`Introspector::query_by_label` directamente. L1
inalterado. Fora-de-escopo P206; passo dedicado
futuro.

### D5 — `serde_json` em lab/parity dev-deps, não em
03_infra deps

03_infra adicionar `serde_json` exigiria justificação
ADR (allowlist deps L3). lab/parity é quarentena; sem
ADR exigido. **Domain struct** `QuerySummary` (sem
serde) preserva 03_infra clean; lab/parity converte
para JSON quando necessário.

### D6 — `equation` selector vanilla diverge —
documentação, não fix

Vanilla aceita `math.equation` (com namespace); rejeita
`equation` standalone. Cristalino aceita `equation`
via `ElementKind::Equation` (P186B). Divergência
arquitectónica registada — fix exigiria parsing vanilla
namespace em cristalino (fora-de-escopo P206C).

### D7 — `cite-bibliography.typ` fail é gap conhecido

Cristalino eval falha em bibliography (1 diagnostic).
Bibliography stdlib parcial (P181 series); gap
documentado. Não regressão; documentação.

### D8 — Matriz produz dados diagnósticos sem assert
global

Per pattern existente (eval_parity, layout_parity):
`p206c_corpus_estrutural_36_ficheiros` reporta matches/
diffs/errors via `eprintln!` mas não falha test. Pattern
consistente: paridade é medição.

### D9 — `tempdir` helper duplicado

Pattern `TempDir` + `tempdir()` repetido em
`structural_parity.rs` (paralelo a `eval_parity.rs`,
`layout_parity.rs`). Refactor para shared helper
deferred — pattern existente em harness; refactor seria
lab/parity-wide cleanup, fora-de-escopo P206C.

### D10 — Sem `P206C.div-N` adicional (apenas div-1)

C1.3 confirmou exhaustivamente: `equation` divergência
é selector parsing; `cite-bibliography` é bibliography
stdlib gap; outline-toc heading diff é design decision
TOC entries. Todos documentados em §5; sem `div-N`
necessário (são empíricos esperados, não obstáculos
estruturais).

---

## §7 Resumo — métricas

| Métrica | Valor |
|---------|-------|
| Caminho fixado | **B** (helper em 03_infra) |
| `P206C.div-N` | **1** (div-1: CLI subcomando deferred) |
| Tests workspace cristalino antes | 1860 |
| Tests workspace cristalino depois | **1873** (+13 query_helpers) |
| Tests lab/parity antes | 54 |
| Tests lab/parity depois | **64** (+10 P206C: 7 unit structural_compare + 2 e2e + 1 corpus) |
| Linter violations | 0 (sem alteração) |
| Ficheiros código novos | 4 (`03_infra/src/query_helpers.rs`; `lab/parity/src/vanilla_invoke.rs`; `lab/parity/src/structural_compare.rs`; `lab/parity/tests/structural_parity.rs`) |
| Ficheiros código modificados | 2 (`03_infra/src/lib.rs` export; `lab/parity/Cargo.toml` serde_json dep) |
| Ficheiros L0 novos | 1 (`00_nucleo/prompts/infra/query-helpers.md`) |
| Ficheiros docs novos | 2 (este + relatório) |
| Ficheiros docs modificados | 1 (ADR-0075 §P206C) |
| LOC novas (código) | ~700 |
| Cargo deps adicionados | 1 (`serde_json = "1"` em lab/parity dev-deps) |
| Refactor mid-execution | 0 |
