# P206D — Inventário interno (matriz consolidada + cobertura)

**Data**: 2026-05-08.
**Spec**: `00_nucleo/materialization/typst-passo-206D.md`.
**Output 1 de 3** (inventário interno).
**Caminho fixado**: **B** (test dedicado novo;
reuso de `ParityMatrix` schema).

---

## §1 C1 — Inventário empírico (6 sub-secções)

### §1.1 C1.1 — `ParityMatrix` actual

**Status**: `CONFIRMADO`.

Localização: `lab/parity/src/report.rs`.

Estrutura:
```rust
pub struct ParityMatrix {
    pub categories: Vec<CategoryRow>,
    pub date:       String,
    pub passo:      String,
    pub summary:    String,
}

pub struct CategoryRow {
    pub name: String,
    pub total_files:        usize,
    pub compiled_ok:        usize,
    pub text_content_passed: Option<usize>,
    pub structural_passed:   Option<usize>,
    pub geometric_max_dx:    Option<f64>,
    pub geometric_max_dy:    Option<f64>,
    pub geometric_mean_dx:   Option<f64>,
    pub geometric_mean_dy:   Option<f64>,
}
```

API:
- `render_markdown() -> String` — produz markdown table.
- `write_latest(base) -> Result<PathBuf>` — escreve
  `reports/latest.md`.
- `write_history(base) -> Result<PathBuf>` — escreve
  `reports/history/{date}-passo-{passo}.md`.

**Schema preserva colunas relevantes para P206D** —
`text_content_passed` + `structural_passed` como
`Option<usize>` permitem distinguir N/A de 0 explícito.

### §1.2 C1.2 — `corpus_completo_p3`

**Status**: `CONFIRMADO`.

Localização: `lab/parity/tests/layout_parity.rs:108-169`.

Estrutura actual (pós-P206B fix):
- Itera 4 categorias (markup, math, code, visual);
  excluindo semantic.
- `compile_cristalino` per ficheiro.
- Constrói `CategoryRow` apenas com `compiled_ok`;
  text_content_passed / structural_passed / geometric_*
  ficam `None` → render como N/A.
- Chama `matrix.write_latest()` + `write_history()`.

**Não toca em vanilla**. Pre-P206C, esta era a única
matriz produzida. Pós-P206C, structural_parity test
produz dados via eprintln mas não escreve em reports/.

**Decisão**: P206D **não modifica** `corpus_completo_p3`
— deixa intacto. Test novo dedicado escreve em reports/
com dados P206D consolidados.

### §1.3 C1.3 — Estrutura `structural_parity.rs` (P206C)

**Status**: `CONFIRMADO`.

`lab/parity/tests/structural_parity.rs` (novo em P206C):
- 3 tests: `p206c_corpus_estrutural_36_ficheiros`,
  `p206c_query_simple_heading`, `p206c_query_metadata_values_e2e`.
- 7 unit tests path-included via `#[path =
  "../src/structural_compare.rs"]`.
- Output: matriz markdown via `eprintln!` em runtime.
- **Não escreve em reports/** — apenas test logs.

P206D consolida via test paralelo que **escreve em
reports/** usando `ParityMatrix` schema.

### §1.4 C1.4 — Patterns de relatório

**Status**: `CONFIRMADO`.

`lab/parity/reports/`:
- `latest.md` — sempre o mais recente; sobre-escrito a
  cada test run (pre-P206D era Passo 150 desactualizado).
- `history/` — cópias imutáveis por sub-passo.
  - `2026-04-25-passo-150.md` (P150).
  - `2026-04-25-passo-153.md` (P153).
- Convenção: `{date}-passo-{NNN}.md`.

**Latest desactualizado**: P206D update natural — escreve
nova matriz com `passo: "206D"`.

### §1.5 C1.5 — Sentinelas existentes

**Status**: `AJUSTE NECESSÁRIO`.

`lab/parity/tests/`:
- `parse_parity.rs`: 50 tests (parse comparison).
- `eval_parity.rs`: 1 test (corpus_completo_p2;
  cristalino-only baseline).
- `layout_parity.rs`: 1 test (corpus_completo_p3;
  cristalino-only baseline).
- `vanilla_cli_smoke.rs` (P206B): 2 tests.
- `structural_parity.rs` (P206C): 10 tests.

**Sem threshold-based sentinels** (assertions sobre N
mínimo). `corpus_completo_p3` não falha se cobertura
diminuir — apenas reporta. P206D adiciona sentinelas
threshold-based:
- `p206d_corpus_cobertura_minima` (≥ 20 INCLUDE).
- `p206d_matriz_renderizavel` (sem panic).
- `p206d_skips_documentados` (manifest existe).

### §1.6 C1.6 — Custos das alternativas C2

**Estimativas empíricas**:

| Caminho | Magnitude | Trabalho |
|---------|-----------|----------|
| **A — estender `corpus_completo_p3`** | M | Toca test estabelecido (P3 layout focus); risco re-introduzir bugs no harness P3 cristalino-only |
| **B — test dedicado novo** | S-M | Reusa `ParityMatrix` schema; escreve reports/; isolamento de concerns |
| **C — reports/ ficheiro versionado em git** | M | Diff-check em CI; overhead de regenerar diffs em runs locais |

**Caminho B** é favorito (per spec §8 hipótese
"Caminho B"; melhor isolamento; reuso máximo do schema
sem tocar `corpus_completo_p3`).

---

## §2 C2 — Caminho fixado: **B** (test dedicado novo)

Justificação literal:

1. **Reuso máximo do schema existente** — `ParityMatrix`
   já tem todas as colunas necessárias
   (`text_content_passed`, `structural_passed`,
   `geometric_*` como `Option<usize>`/`Option<f64>`).
2. **Isolamento de concerns** — Caminho A misturaria
   P3 layout focus (`corpus_completo_p3`) com
   structural focus (P206D). Caminho B mantém
   `corpus_completo_p3` intacto.
3. **Magnitude S-M** dentro do orçamento P206 série
   (M agregado per P206A C10).
4. **Hipótese spec §8** confirmada — "P206D pode
   reutilizar pattern ou refactorizar para renderer
   dedicado. Caminho B é favorito".

Caminho A rejeitado: tocar test estabelecido sem
benefício observable.

Caminho C rejeitado: diff-check em CI exigiria
tooling adicional (CI workflow updates); overhead
desproporcional para output que já é versionado via
`reports/history/`.

---

## §3 C3 — Estrutura concreta da matriz

### §3.1 Colunas e linhas

5 colunas (paralelo a P150 schema):
- `Total` — total de ficheiros por categoria.
- `Compila (cristalino)` — count de ficheiros que
  compilam (`query_to_summary` para "heading" não
  retorna erro).
- `text_content` — count de ficheiros que produzem
  pelo menos 1 `QuerySummary` não-erro (cristalino-only).
- `structural` — count de ficheiros com pelo menos 1
  match cristalino vs vanilla via `compare_query_outputs`
  (vanilla CLI necessário; N/A se ausente).
- `geometric (experimental)` — N/A per ADR-0054.

5 linhas (1 por categoria):
- code (2 ficheiros).
- markup (7 ficheiros).
- math (2 ficheiros).
- semantic (10 ficheiros).
- visual (15 ficheiros).
- **Total** (sumário 36).

### §3.2 Cells

Tipos per cell (per `Option<usize>` schema):
- `usize` rendered como `N/M` (ex: `6/7`).
- `None` rendered como `N/A`.

### §3.3 SKIPs literais aplicados

Per skip_reason() em consolidado_p206d.rs:
- `code` categoria → `text_content`/`structural` N/A.
- `semantic` categoria → idem.
- `math` categoria → text_content/structural N/A
  (selectors_for_category retorna `&[]`; não há
  selectors validados em ambos cristalino+vanilla
  por equation namespace divergence).
- `markup/error.typ` → `compiled = false` (não conta
  como compiled_ok).
- `visual/cite-bibliography.typ` → eval falha em
  cristalino → não conta nem em compiled_ok nem em
  text_content/structural (seja por filter
  skip_reason None mas eval failure).

### §3.4 Render markdown

Reusa `ParityMatrix::render_markdown()` existente —
sem alteração em `report.rs`. Apenas o test novo
constrói o `ParityMatrix` com dados P206D.

---

## §4 C4 — Manifest SKIPs

`lab/parity/SKIPS.md` (novo, ~5 KB) documenta
literalmente:

- §1: 3 SKIP-pre-existing (`markup/error.typ`,
  `code/let.typ`, `code/set.typ`).
- §2: 10 SKIP-feature (`semantic/*` → P2 eval scope).
- §3: 3 INCLUDE-com-diff:
  - `math/{block,simple}.typ` + visuais com equation:
    vanilla rejeita `equation` standalone.
  - `visual/cite-bibliography.typ`: cristalino
    bibliography stdlib parcial.
  - `visual/outline-toc.typ`: TOC entries contadas
    distintamente.
- §4: sumário cobertura (23 INCLUDE / 13 SKIP / 36 total).
- §5: convenção de manutenção (futuras adições).
- §6: cross-references (ADR-0075, P206C, ADR-0054,
  DEBT-53, P204F.div-1).

Sentinela `p206d_skips_documentados` verifica
empíricamente que SKIPS.md cobre os 3 SKIP-pre-existing
+ pelo menos 2 ficheiros semantic + 3 divergências.

---

## §5 Decisões durante a leitura

### D1 — Caminho B fixado por isolamento de concerns

C1.6 mostrou Caminho B = S-M com isolamento; Caminho
A = M com risco de regressão em harness P3 estabelecido.
Caminho C = M com overhead CI desproporcional. Decisão
B é honesta: respeita orçamento + reusa schema sem
tocar testes existentes.

### D2 — `corpus_completo_p3` permanece intacto

P206D **não toca** `corpus_completo_p3` (P206B fix
preservado; latest.md output sobrescrito para P206D
mas history preservado). Pattern P204H/P205E:
preservação histórica sem reescrita.

### D3 — Smoke "compiled" via "heading" selector universal

Inicialmente: `compiled = any(selector valido)` falhou
para math (selectors_for_category("math") retorna `&[]`).
Fix: usar selector "heading" como universal smoke —
qualquer categoria deve aceitar "heading" mesmo que
count=0. Excepção: `error.typ` (sintaxe inválida) é
hardcoded skip.

### D4 — text_content e structural per ficheiro, não per selector

Inicial implementação contava per selector — produzia
counts inflados (ex: visual=15/15 quando alguns ficheiros
falhavam). Fix: contar per ficheiro com pelo menos 1
selector match. Métrica empírica mais acurada.

### D5 — `selectors_for_category` exclui semantic e code

Categorias SKIP-feature: `selectors_for_category` retorna
`&[]`. Loop interno não executa; text_content/structural
ficam N/A. Coerente com SKIPS.md.

### D6 — Manifest é prosa, não código

`lab/parity/SKIPS.md` é documento markdown, não código.
Sentinela `p206d_skips_documentados` verifica
existência + conteúdo via grep — equivalente a
`assert!(content.contains("..."))`. Pattern coerente
com convenção lab/parity (medição + reporting; não
verificação rígida).

### D7 — Sentinela cobertura mínima ≥ 20

P206C C6 reportou 23 INCLUDE. Threshold ≥ 20 dá margem
3 ficheiros para regressões aceitáveis sem falso
alarme. Threshold mais agressivo (≥ 22) seria fragile.
Threshold mais lax (≥ 15) perderia poder diagnóstico.

### D8 — Test consolidado_p206d.rs duplicate path-include

`structural_compare.rs` é path-included em **dois**
test files (structural_parity.rs e consolidado_p206d.rs).
Os 7 unit tests de structural_compare correm 2× — uma
vez em cada binary de test. Não é regressão — pattern
Rust testing standard. Total tests reportados aumenta
mas não há trabalho extra significativo.

### D9 — vanilla CLI desnecessário para text_content

`text_content` mede cristalino-only (QuerySummary
produzido sem erro). Vanilla CLI ausência **não**
afecta text_content; afecta apenas structural. Schema
reflecte isto via `Option<usize>` distinto.

### D10 — Sem refactor de `corpus_completo_p3`

Spec C2 deu liberdade entre Caminho A (estender) e
Caminho B (paralelo). Caminho B preserva isolamento.
Refactor de `corpus_completo_p3` para usar P206C
helpers seria expansão fora-de-escopo P206D — pattern
"medir, não refactorar".

---

## §6 Resumo — métricas

| Métrica | Valor |
|---------|-------|
| Caminho fixado | **B** (test dedicado novo) |
| Tests workspace cristalino antes | 1873 |
| Tests workspace cristalino depois | **1873** (∆ 0 — P206D é lab/parity) |
| Tests lab/parity antes | 64 |
| Tests lab/parity depois | **75** (∆+11: 4 sentinelas dedicadas + 7 path-included structural_compare unit duplicados) |
| Net P206D tests (sem duplicados path) | 4 (sentinelas dedicadas) |
| Linter violations | 0 (sem alteração) |
| Ficheiros código novos | 1 (`lab/parity/tests/consolidado_p206d.rs`) |
| Ficheiros código modificados | 0 (corpus_completo_p3 intacto; report.rs intacto) |
| Ficheiros docs novos | 3 (SKIPS.md + inventário + relatório) |
| Ficheiros docs modificados | 1 (ADR-0075 §P206D) |
| Reports actualizados | 2 (`latest.md` sobrescrito + `history/2026-05-08-passo-206D.md` novo) |
| LOC novas (código) | ~250 (test consolidado) |
| LOC novas (docs) | ~600 (SKIPS.md + inventário) |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 1 (smoke compiled via "heading"; per D3) |
