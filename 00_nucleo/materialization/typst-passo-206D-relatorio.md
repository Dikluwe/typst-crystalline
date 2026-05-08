# Relatório do passo P206D

**Data de execução**: 2026-05-08.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-206D.md`.
**Natureza**: implementação de relatório (matriz +
cobertura + sentinelas).
**Sub-passo `D` da série P206** — quarto de 5 (A–E).
**Magnitude planeada**: S–M.
**Magnitude real**: **S-M** (~40 min; 1 ficheiro código
novo + 1 manifest + 3 outputs documentais; sem refactor
mid-execution significativo).

---

## §1 O que foi feito

P206D consolidou matriz de paridade que integra dados
P206C nos relatórios `lab/parity/`:

- **Test consolidado novo**
  `lab/parity/tests/consolidado_p206d.rs` (~270 LOC)
  reutiliza `ParityMatrix` schema existente
  (`report.rs`).
- **Manifest documental** `lab/parity/SKIPS.md`
  documenta literalmente 3 SKIP-pre-existing + 10
  SKIP-feature + 3 INCLUDE-com-diff.
- **Matriz P206D produzida** em
  `lab/parity/reports/latest.md` + history versionado
  `2026-05-08-passo-206D.md`.
- **3 sentinelas dedicadas** (4 tests no total) per
  spec C5.
- **ADR-0075 §P206D** anotada `✅ MATERIALIZADO
  2026-05-08`.

P206D **não modificou** `corpus_completo_p3`
(harness P3 layout cristalino-only preservado intacto)
nem `report.rs` schema (já adequado para P206D).

### Caminho escolhido: B (test dedicado novo)

C2 fixou Caminho B per evidência empírica (C1.6):

- **Caminho A** (estender `corpus_completo_p3`):
  rejeitado. Toca test estabelecido com risco de
  regressão; mistura P3 layout focus com structural
  focus.
- **Caminho B** (test dedicado): fixado. Reusa
  `ParityMatrix` schema; isolamento de concerns;
  magnitude S-M.
- **Caminho C** (ficheiro versionado em git com
  diff-check CI): rejeitado. Overhead CI
  desproporcional para output já versionado via
  `reports/history/`.

### Resultado empírico da matriz

```
| Categoria | Total | Compila | text_content | structural |
|-----------|------:|--------:|-------------:|-----------:|
| code      |     2 |     2/2 |          N/A |        N/A |
| markup    |     7 |     6/7 |          6/7 |        6/7 |
| math      |     2 |     2/2 |          N/A |        N/A |
| semantic  |    10 |   10/10 |          N/A |        N/A |
| visual    |    15 |   14/15 |        14/15 |      14/15 |
| **Total** |    36 |   34/36 |        20/36 |      20/36 |
```

- **34/36 (94%)** corpus compila em cristalino.
- **20/36** com text_content matches em categorias
  INCLUDE (markup/visual; total 22 ficheiros).
- **20/36** com structural matches vs vanilla — mesma
  cobertura, paridade observable confirmada nas
  categorias INCLUDE.
- 13 SKIP justificados (3 pre-existing + 10 feature
  semantic) + 3 divergências documentadas (equation,
  cite-bibliography, outline-toc).

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-206D-inventario.md`.

Conteúdo:
- §1 C1 inventário (6 sub-secções; 5 CONFIRMADO + 1
  AJUSTE NECESSÁRIO).
- §2 C2 caminho fixado (B).
- §3 C3 estrutura concreta da matriz (5 colunas × 5
  categorias).
- §4 C4 manifest SKIPs.
- §5 10 decisões durante a leitura (D1-D10).
- §6 métricas.

Tamanho: ~12 KB.

### Output 2 — Relatório (este ficheiro)

### Output 3 — Alterações em código

#### Ficheiros novos

- **`lab/parity/tests/consolidado_p206d.rs`** (~270
  LOC):
  - `build_matriz_p206d(base) -> ParityMatrix` —
    constrói matriz reusing helpers P206C +
    `ParityMatrix` schema.
  - `selectors_for_category(category)` — selectors
    aplicados per categoria.
  - `category_included(category)` — INCLUDE check.
  - `skip_reason(category, file)` — SKIP discriminator.
  - 4 tests:
    - `p206d_corpus_consolidado` — produz matriz +
      writes reports.
    - `p206d_corpus_cobertura_minima` — INCLUDE ≥ 20.
    - `p206d_matriz_renderizavel` — sem panic +
      headers presentes.
    - `p206d_skips_documentados` — SKIPS.md cobre
      SKIPs literais + divergências.
  - 7 unit tests path-included via `#[path =
    "../src/structural_compare.rs"]` (duplicados
    da structural_parity.rs).
- **`lab/parity/SKIPS.md`** (~5 KB) — manifest
  documental.

#### Ficheiros docs

- **`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`**
  — §P206D anotado `✅ MATERIALIZADO 2026-05-08`.

#### Reports actualizados

- **`lab/parity/reports/latest.md`** sobrescrito com
  matriz P206D (era Passo 150 desactualizado).
- **`lab/parity/reports/history/2026-05-08-passo-206D.md`**
  versionado.

---

## §2 Tempo de execução

~40 minutos efectivos:

- ~5 min: leitura da spec + setup TaskList.
- ~10 min: C1 inventário empírico (6 sub-secções; leitura
  report.rs + corpus_completo_p3 + grep patterns).
- ~3 min: C2-C3 fixação Caminho B + estrutura literal.
- ~5 min: C4 escrita de SKIPS.md (~5 KB; 6 secções).
- ~10 min: C5 escrita de consolidado_p206d.rs (~270
  LOC; 4 tests + helpers + matriz builder).
- ~3 min: refactor pequeno (D3 — smoke "heading"
  universal; D4 — métricas per ficheiro).
- ~3 min: validação (cargo test workspace + lab/parity
  + lint + matriz produzida).
- ~1 min: ADR anotação.
- ~5 min: outputs documentais (este + inventário).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Caminho fixado | **B** (test dedicado novo) |
| Tests workspace cristalino antes | 1873 |
| Tests workspace cristalino depois | **1873** (∆ 0 — P206D é lab/parity quarentena) |
| Tests lab/parity antes | 64 |
| Tests lab/parity depois | **75** (∆+11: 4 sentinelas dedicadas + 7 path-included structural_compare duplicados) |
| Net P206D tests dedicados | 4 |
| Linter violations | 0 (sem alteração) |
| Linter warnings | 1 substantivo: `Skip` variant unused em CompareResult (preserved para futuras extensões; pre-P206D existente) |
| Ficheiros código novos | 1 (`consolidado_p206d.rs`) |
| Ficheiros código modificados | 0 |
| Ficheiros docs novos | 3 (SKIPS.md + inventário + este relatório) |
| Ficheiros docs modificados | 1 (ADR-0075 §P206D) |
| Reports actualizados | 2 (latest.md sobrescrito + history novo) |
| LOC novas (código) | ~270 (consolidado_p206d.rs) |
| LOC novas (docs) | ~1500+ (SKIPS.md + inventário + relatório + ADR patch) |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 1 minor (smoke "heading" universal; D3) |

### Tests por crate (workspace cristalino — sem alteração)

- `typst_core` unit: 1584.
- `typst_infra`: 242 (24 unit + 218 integration).
- `typst_shell` unit: 21.
- `typst_wiring` unit: 2.
- (doctest): 2 ignored.
- **Total workspace**: 1873.

### Tests lab/parity

- `parse_parity`: 50.
- `eval_parity`: 1.
- `layout_parity`: 1.
- `vanilla_cli_smoke` (P206B): 2.
- `structural_parity` (P206C): 10 (3 e2e + 7 unit
  path-included).
- `consolidado_p206d` (P206D novo): **11** (4 dedicated
  + 7 path-included structural_compare duplicados).
- **Total lab/parity**: 75.

---

## §4 Decisões

### D1 — Caminho B fixado por isolamento de concerns

C1.6 mostrou Caminho B = S-M com isolamento; Caminho A
mistura concerns sem benefício; Caminho C tem overhead
CI desproporcional. Decisão B é honesta: respeita
orçamento; reusa schema sem tocar testes existentes.

### D2 — `corpus_completo_p3` intacto

Pattern P204H/P205E (preservação histórica sem
reescrita) aplicado: P206D consolida em paralelo, não
modifica harness P3 layout estabelecido.

### D3 — Smoke "compiled" via "heading" selector universal

Inicial implementação usou `selectors_for_category`
para smoke compiled — falhou para math
(`selectors_for_category("math") = &[]` → any() vazio
= false). Fix: usar "heading" universal — qualquer
ficheiro deve aceitar "heading" selector mesmo que
count=0. Excepção: error.typ hardcoded skip.

### D4 — Métricas per ficheiro, não per selector

Inicial implementação contava per selector — produzia
counts inflados (ex: visual=15/15 quando alguns
ficheiros falhavam). Fix: contar `entry_had_text_ok`
e `entry_had_struct_match` per file. Métrica empírica
mais acurada.

### D5 — `selectors_for_category` exclui math

`math/{block,simple}.typ` falham contra vanilla com
"unknown variable equation" (vanilla usa `math.equation`
namespace). selectors_for_category("math") retorna
`&[]` para evitar inflar errors P206D. Categorizado
como SKIP-feature em SKIPS.md §3.

### D6 — Manifest SKIPS.md é prosa documental

Decisão: manifest como markdown explícito (não inline
no código) para legibilidade humana. Sentinela
`p206d_skips_documentados` verifica conteúdo via
`assert!(content.contains(...))` — pattern leve sem
parser de markdown.

### D7 — Sentinela cobertura mínima ≥ 20

P206C C6 reportou 23 INCLUDE. Threshold ≥ 20 dá margem
3 ficheiros para regressões aceitáveis sem falso
alarme. Threshold mais agressivo seria fragile;
threshold mais lax perderia poder diagnóstico.

### D8 — Path-include duplica unit tests

`structural_compare.rs` path-included em 2 test files;
seus 7 unit tests correm 2×. Pattern Rust testing
standard; sem trabalho extra significativo (cada test
binary compila independente). Total tests reportados
infla mas net P206D = 4 sentinelas dedicadas.

### D9 — vanilla CLI desnecessário para text_content

`text_content` mede cristalino-only; ausência de
vanilla afecta apenas `structural`. Schema
`Option<usize>` distingue N/A de 0 explicitamente.

### D10 — Sem refactor de `corpus_completo_p3`

Spec C2 deu liberdade entre A e B. Caminho B preserva
isolamento. Refactor expansão fora-de-escopo P206D —
pattern "medir, não refactorar".

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §8:

| Hipótese | Resultado |
|----------|-----------|
| §8: "C2 = Caminho B (matriz dedicada como struct + renderer)" | **CONFIRMADA** — C1.6 confirmou B = S-M |
| §8: "ParityMatrix provavelmente já existe em report.rs" | **CONFIRMADA** — schema robusto pré-P206D |
| §8: "Caminho A inflaciona corpus_completo_p3" | **CONFIRMADA** — P3 layout focus distinto de structural |
| §8: "Caminho C cria ficheiro que precisa diff-check em CI" | **CONFIRMADA** — overhead desproporcional |
| §8: "inflar matriz para demonstrar cobertura completa" | **EVITADO** — matriz reflecte realidade empírica (20/36 INCLUDE com SKIPs justificados) |
| §8: "duplicar lógica de renderização" | **EVITADO** — P206D reusa `render_markdown` existente |

6 hipóteses resolvidas pela auditoria empírica.

---

## §6 Sugestão para próximo sub-passo

P206D fechado per C12 com todos os critérios cumpridos:

- ✓ C1 inventário completo (6 sub-secções).
- ✓ C2 caminho fixado (B) com justificação.
- ✓ C3 estrutura concreta (5 colunas × 5 categorias).
- ✓ C4 categorias de Skip documentadas em
  `lab/parity/SKIPS.md`.
- ✓ C5 sentinelas dedicadas (3 + 1 corpus = 4).
- ✓ C6 formato output fixado (markdown via
  `render_markdown`).
- ✓ C7 compilação verde (`cargo check --all-targets`).
- ✓ C8 tests workspace cristalino 1873 mantém-se.
- ✓ C9 tests lab/parity 75 verdes (+11).
- ✓ C10 linter 0 violations.
- ✓ C11 ADR-0075 §P206D anotada.
- ✓ Inventário registado.
- ✓ Relatório escrito (este ficheiro).

**Próximo sub-passo**: **P206E — Encerramento + ADR
transições + DEBT fechos** (per ADR-0075 plano de
materialização).

P206E é magnitude S documental (~30-45 min):

- Auditoria das 7 condições ADR-0075 (todas cumpridas
  per P206B+C+D).
- Forma de fecho: Completo (final).
- Transições:
  - **ADR-0075** PROPOSTO → **ACEITE final**.
  - **ADR-0073** "estruturalmente fechado" → **"completo
    final"** (cond 9 fechada estruturalmente via P206
    matriz).
  - **DEBT-53** → **CLOSED** (vanilla integration
    materializada).
  - **DEBT-54** → **OBSOLETED** (workspace setup
    desnecessário).
- Relatório consolidado série P206A-E (paralelo
  P204H/P205E).
- Blueprint actualizado §3.0ter [P206E].

Pré-condições confirmadas por P206D:
- Matriz consolidada produzida em reports/latest.md +
  history.
- Sentinelas threshold-based activas.
- Manifest SKIPs documentado.
- Cobertura observable estrutural confirmada
  empíricamente (20/36 INCLUDE com matches).

---

## §7 Cross-references

- **Spec**: `00_nucleo/materialization/typst-passo-206D.md`.
- **Outputs P206D**:
  - `00_nucleo/diagnosticos/typst-passo-206D-inventario.md`.
  - `lab/parity/SKIPS.md` (manifest).
  - `lab/parity/reports/latest.md` (matriz P206D).
  - `lab/parity/reports/history/2026-05-08-passo-206D.md`.
- **ADR**:
  `00_nucleo/adr/typst-adr-0075-vanilla-integration.md`
  (§P206D ✅ MATERIALIZADO 2026-05-08).
- **Predecessores**:
  - P206C (helpers materializados;
    `query_helpers.rs` + `vanilla_invoke.rs` +
    `structural_compare.rs`).
  - P206B (harness reactivado).
  - P206A (diagnóstico-primeiro; ADR-0075 PROPOSTO).
- **Sucessor planeado**: P206E (encerramento + ADR
  ACEITE final).
- **Pendências endereçadas**:
  - DEBT-53: progresso material (matriz consolidada
    confirma vanilla integration funcional).
  - DEBT-54: confirmado obsoleto (workspace setup
    nunca foi necessário).
  - Cond 9 ADR-0073 PARCIAL: progresso significativo
    (P206E formaliza transição "completo final").
- **Pattern referência**: P204G (P204 série paralela);
  P204H consolidado (estrutura de fecho).
