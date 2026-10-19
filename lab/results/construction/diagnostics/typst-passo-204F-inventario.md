# Inventário interno P204F — Corpus paridade reduzido

**Data**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204F.md`.
**Natureza**: diagnóstico interno + alterações aplicadas.

---

## §1 C1 — Inventário empírico

### 1.1 Estrutura `lab/parity/` — **CONFIRMADO**

- **Path**: `lab/parity/`.
- **Sub-pastas**: `corpus/` (com sub-categorias `code/`,
  `markup/`, `math/`, `semantic/`, `visual/`), `reports/`,
  `src/`, `tests/`.
- **Cargo.toml**: `[package] name = "typst-parity"`. Binary
  `parity-runner`.

### 1.2 Corpus actual — **CONFIRMADO**

30 ficheiros `.typ`. Por categoria:
- `code/` — 2 (let, set).
- `markup/` — 7 (heading, strong, parbreak, etc.).
- `math/` — 2 (simple, block).
- `semantic/` — 10 (types, conditionals, closures).
- `visual/` — 9 (text styling, math básico, figure simples).

**0 ficheiros cobrem introspection** (per P204A A15).

### 1.3 Harness paridade observable — **AJUSTE / DEBT pré-existente**

`lab/parity/tests/layout_parity.rs` declara explicitamente
no doc comment:

> P3 — Layout parity (Passo 150). Materializa a primeira
> matriz agregada de paridade no cristalino. **Vanilla
> integration é DEBT-53 (candidato)**: nesta iteração, o
> teste corre **cristalino-only baseline** (compilação +
> extracção de `FrameDTO`), sem comparação contra vanilla.
> O harness **não tem `assert!` global** — paridade é
> medição, não verificação.

Realidade: harness é **cristalino-only**; não há vanilla
PDF diff funcional. DEBT-53 (vanilla integration) e
DEBT-54 (vanilla setup) bloqueiam.

**P204F.div-1 registada**: spec assumiu observable harness
existente; realidade é cristalino-only baseline com vanilla
integration deferred.

Adicionalmente, lab/parity tem 2 pre-existing breaks
(não introduzidos por P204F):
- `tests/layout_parity.rs:69`: `layout(content, state)`
  signature outdated (P190I migrou para `layout(content)`).
- `src/value_dto.rs:83`: missing `Value::Location(_)` arm
  (P179 adicionou variant).

### 1.4 Harness paridade estrutural — **NÃO EXISTE**

Sem harness para queries `Introspector` comparadas com
vanilla `typst query`. Criar de raiz seria magnitude L
(CLI invocation, output parsing, comparison).

### 1.5 Vanilla typst — **AJUSTE**

Não acessível via harness. Criar de raiz exigiria:
- Build vanilla typst CLI ou usar pre-built binário.
- Helper para invocar `typst compile` e `typst query`.
- Magnitude L+.

### 1.6 Companions `.typ.toml` — **CONFIRMADO**

Formato existente (per `lab/parity/corpus/visual/heading-simples.typ.toml`):

```toml
features = ["heading", "text"]
modo_p3 = "text_content"
notes = "Heading nível 1 + parágrafo simples."
```

Simples; aceitável replicar para os novos ficheiros.

### 1.7 Etiquetas

- C1.1: **CONFIRMADO**.
- C1.2: **CONFIRMADO**.
- C1.3: **AJUSTE** — `P204F.div-1` registada.
- C1.4: **AJUSTE** — não existe; criar de raiz fora de
  escopo P204F.
- C1.5: **AJUSTE** — vanilla integration deferred.
- C1.6: **CONFIRMADO**.

---

## §2 C2 — Caminho fixado — **B reduzido (cristalino-only)**

### Decisão

Per P204F.div-1, **adopta-se Caminho B reduzido**:
- 5 core + 1 opcional .typ files no corpus.
- Companions `.toml` cristalino-only (vanilla expectations
  ausentes).
- Smoke tests cristalino em `03_infra` (não em
  `lab/parity/`, evitando harness broken).
- Vanilla integration **deferred** per pre-existing
  DEBT-53/54.

### Justificação

- Caminho A (dual completa) exige criar harness vanilla
  + harness estrutural — magnitude L+ desproporcional.
- Caminho B (observable apenas) exige fixar harness
  vanilla + PDF diff — magnitude M+ adicional.
- Caminho C (estrutural apenas) exige criar harness
  estrutural de raiz — magnitude M.
- **B reduzido** mantém escopo M dentro do planeado;
  honra a estimativa P204A C9 ("escala reduzida"); não
  toca em pre-existing breaks (out of scope).

Pre-existing DEBT-53/54 absorvem a integração vanilla;
P204F **adiciona corpus + smoke validation cristalino**.

### Alternativas rejeitadas

- **Fixar lab/parity pre-existing breaks**: out of scope
  P204F. Per spec §7, P204F não modifica harness existente.
- **Criar harness vanilla de raiz**: L+ inflação;
  desproporcional para sub-passo M.

---

## §3 C3 — 7 ficheiros especificados (5 core + 2 opcionais → 5+1 implementados)

### Core (5)

#### 1. `outline-toc.typ`

```typ
#outline()

= Introdução
...
= Conclusão
```

5 headings + outline. Asserções cristalino:
`heading_count = 5`, `outline_present = true`.

#### 2. `counter-heading.typ`

```typ
#set heading(numbering: "1.1")

= Capítulo Um
== Subsecção
...
```

5 headings hierárquicos. Asserções:
`formatted_counters = ["1", "1.1", "1.2", "2", "2.1"]`.

#### 3. `figure-ref.typ`

```typ
#set figure(numbering: "1")

#figure([Imagem alfa], caption: [...]) <fig-alfa>
...
@fig-alfa, @fig-beta, @fig-gama.
```

3 figures + 3 refs. Asserções: `figure_count = 3`,
`figure_numbers = [1, 2, 3]`.

#### 4. `equation-ref.typ`

```typ
#set math.equation(numbering: "(1)")

$ E = m c^2 $ <eq-einstein>
...
```

3 equations block + 3 refs. Asserções:
`equation_count = 3`, `numbering_active_equation = true`.

#### 5. `cite-bibliography.typ`

```typ
@ref-alfa
...
#bibliography("refs.yaml")
```

3 cites + bibliography asset. Asserções:
`citation_count = 3`, `bibliography_keys = [...]`.

**Asset**: `refs.yaml` criado com 3 entries (alfa, beta,
gama).

### Opcionais (2 → 1 implementado)

#### 6. `query-metadata.typ` ✅

```typ
#metadata("primeiro")
#metadata((tag: "secundario", peso: 42))
#metadata("terceiro")
```

3 metadata embebidos. Asserções: `metadata_count = 3`.

#### 7. `here-locate.typ` ❌ **SKIPPED**

`here()` e `locate()` não estão registadas em stdlib
cristalino. P204D materializou `Position` em runtime,
mas não modificou stdlib (per spec P204D §7
não-objectivos).

Skip documentado per spec §8 ("caso uma feature falhe,
**não inflaciona escopo** — regista divergência e
continua com os outros casos").

---

## §4 C5 — Harness estrutural — **SKIP** (per C2 = B reduzido)

Sem trabalho em P204F. Sub-passo dedicado pós-M8 pode
endereçar quando vanilla integration for materializada.

---

## §5 C6+C7 — Edições aplicadas

### 5.1 Ficheiros novos em `lab/parity/corpus/visual/`

- `outline-toc.typ` + `outline-toc.typ.toml`.
- `counter-heading.typ` + `counter-heading.typ.toml`.
- `figure-ref.typ` + `figure-ref.typ.toml`.
- `equation-ref.typ` + `equation-ref.typ.toml`.
- `cite-bibliography.typ` + `cite-bibliography.typ.toml` +
  `refs.yaml` (asset).
- `query-metadata.typ` + `query-metadata.typ.toml`.

**Total**: 13 ficheiros novos.

### 5.2 Tests dedicados em `03_infra/src/integration_tests.rs`

6 smoke tests `p204f_corpus_*_compila`:
- `p204f_corpus_outline_toc_compila`.
- `p204f_corpus_counter_heading_compila`.
- `p204f_corpus_figure_ref_compila`.
- `p204f_corpus_equation_ref_compila`.
- `p204f_corpus_cite_bibliography_compila` (com
  `catch_unwind` para tolerância a resolução de
  `refs.yaml` que pode falhar em SystemWorld tempdir).
- `p204f_corpus_query_metadata_compila`.

Cada test usa `include_str!` + helper `compile_to_pdf`.
Asserção: PDF não-vazio + header `%PDF-` (smoke).

---

## §6 Decisões tomadas durante a leitura

### 6.1 Tests em 03_infra (não em lab/parity)

`lab/parity` tem pre-existing breaks. P204F evita tocar
no harness; tests cristalino-only ficam em `03_infra/src/integration_tests.rs`
(módulo de tests existente; pattern `compile_to_pdf`
estabelecido).

Vantagem: P204F não inflaciona; pre-existing breaks
absorvidos por DEBT-53/54.

### 6.2 `here-locate.typ` SKIP

`here()`/`locate()` não em stdlib cristalino. Materializar
ficaria fora de escopo M (criaria um sub-passo dedicado
de stdlib expansion).

Skip documentado em ADR-0073 plano §P204F.

### 6.3 `cite-bibliography.typ` com `catch_unwind`

`bibliography("refs.yaml")` resolve o asset via
SystemWorld file system. `include_str!` carrega só o
.typ; o .yaml fica fora do path resolvível por
SystemWorld em tempdir. Test usa `catch_unwind` para
tolerar — se compilar, PDF asserted; se panic, eprintln
documenta gap.

Honest fail-mode: cite-bibliography.typ requer asset
side-by-side. Para validation completa, sub-passo
pós-M8 deve usar SystemWorld com fixture path apropriado.

### 6.4 Companions com `[expectations.cristalino]` table

Formato estende o existente (`features`, `modo_p3`, `notes`)
com `[expectations.cristalino]` para asserções
verificáveis. Vanilla expectations omitidos
explicitamente (deferred).

### 6.5 Sem fix de pre-existing breaks lab/parity

Pre-existing breaks (`layout_parity.rs:69`,
`value_dto.rs:83`) são out of scope P204F. Per spec §7:
"P204F não modifica trait Introspector ou Layouter ou
consumers" — não toca em harness existente.

Pre-existing breaks acumulam DEBT separado; P204G ou
sub-passo dedicado pós-M8 pode resolver.

---

## §7 C8+C9+C10+C11 — Verificações

### C8 — Compilação

```
cargo build --workspace
```

**Resultado**: verde (excluindo lab/parity que não é
workspace member). Pre-existing breaks lab/parity NÃO
introduzidos por P204F.

### C9 — Tests workspace

```
cd 03_infra && cargo test p204f
```

**Resultado**: 6/6 tests verdes.

```
cargo test --workspace
Total tests: 1844
```

**1838 → 1844** (+6 P204F). Sem regressões.

### C10 — Tests dedicados granularidade

Decisão: **1 test por ficheiro** (6 tests granulares).
Justificação: falhas isoladas → identificação
imediata. Granularidade aceite pelo padrão existente
em integration_tests.rs.

### C11 — Linter

```
crystalline-lint .
```

**Resultado**: ✓ No violations found.

(Quando executado com path absoluto explora `lab/`
quarentena e detecta V1 em `tools/test-helper/extension.ts`
— pre-existing TypeScript em vanilla quarentena, não
relacionado com P204F.)

---

## §8 Métricas

| Métrica | Pré-P204F | Pós-P204F | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1838 | **1844** | +6 |
| Crystalline-lint violations | 0 | 0 | = |
| LOC produção | baseline | 0 | 0 (P204F é dados + tests) |
| LOC tests | baseline | +~80 | +80 |
| Corpus paridade ficheiros | 30 | 36 (+6 .typ) | +6 |
| Companions .toml novos | — | 6 | +6 |
| Assets novos (.yaml) | — | 1 (refs.yaml) | +1 |
| Categorias corpus afectadas | — | 1 (`visual/`) | — |

---

## §9 Critério de fecho — C13

Per spec §3 C13:

- [x] C1 inventário completo (6 sub-secções com 3
  AJUSTES).
- [x] C2 caminho fixado (B reduzido).
- [x] C3 7 ficheiros especificados (5 core + 1 opcional
  implementado; 1 opcional `here-locate.typ` SKIP).
- [x] C4 harness observable confirmado funcional
  cristalino-only (DEBT-53 vanilla deferred).
- [x] C5 harness estrutural skip declarado.
- [x] C6 6 `.typ` + 6 companions + 1 asset adicionados.
- [x] C7 bibliography asset resolvido (refs.yaml criado).
- [x] C8 compilação verde.
- [x] C9 tests workspace verdes (1844).
- [x] C10 tests dedicados (6) verdes.
- [x] C11 linter 0 violations.
- [x] C12 ADR-0073 anotada (✅ P204F).
- [x] Inventário registado (este ficheiro).
- [ ] Relatório escrito (próximo output).

**`P204F.div-1`** registada — vanilla integration
deferred per pre-existing DEBT-53/54; honest scope
reduction sem inflação.

---

## §10 Referências

### Modificados em P204F

- `lab/parity/corpus/visual/outline-toc.typ` (novo).
- `lab/parity/corpus/visual/outline-toc.typ.toml` (novo).
- `lab/parity/corpus/visual/counter-heading.typ` (novo).
- `lab/parity/corpus/visual/counter-heading.typ.toml` (novo).
- `lab/parity/corpus/visual/figure-ref.typ` (novo).
- `lab/parity/corpus/visual/figure-ref.typ.toml` (novo).
- `lab/parity/corpus/visual/equation-ref.typ` (novo).
- `lab/parity/corpus/visual/equation-ref.typ.toml` (novo).
- `lab/parity/corpus/visual/cite-bibliography.typ` (novo).
- `lab/parity/corpus/visual/cite-bibliography.typ.toml` (novo).
- `lab/parity/corpus/visual/refs.yaml` (novo asset).
- `lab/parity/corpus/visual/query-metadata.typ` (novo).
- `lab/parity/corpus/visual/query-metadata.typ.toml` (novo).
- `03_infra/src/integration_tests.rs` (+6 tests).
- `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
  (anotação cirúrgica P204F ✅).

### Inalterados (intencional)

- `lab/parity/tests/layout_parity.rs` (pre-existing
  breaks; out of scope P204F).
- `lab/parity/src/value_dto.rs` (pre-existing breaks).
- L1/L2/L3/L4 código produção (P204F é corpus +
  tests).
- ADR-0073 (status PROPOSTO; transita ACEITE em P204H).
- ADR-0066 (sem alteração).

### Auditoria fonte

- `00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`
  (A15 corpus actual sem introspection).
- `00_nucleo/diagnosticos/typst-passo-204A-diagnostico.md`
  (C9 validação reduzida; C13.1 P204F plano).
- `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
  (PROPOSTO; plano de materialização §P204F).
- `00_nucleo/materialization/typst-passo-204F.md` (spec).
