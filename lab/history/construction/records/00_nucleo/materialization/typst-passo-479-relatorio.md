# Relatório P479 — Sonda de paridade `lab/parity/` + gap S materializado

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P479 (Sonda de paridade + materialização condicional)
**Materialização:** Sonda + 1 gap S materializado (`bibliography` título padrão)

---

## 1. Resumo

**Sub-item A — Sonda de paridade:**
Suite `lab/parity/` executada contra corpus de 46 ficheiros (36 P206D + 10 P465–P477).
Resultado: **50 matches / 1 diff / 22 errors** (todos `equation` namespace pré-existente).
Evolução vs P206D: matches ~20 → 50; diffs 3 → 1. Significativa melhoria.

**Sub-item B — Gaps identificados e materialização:**
Dois diffs identificados pela sonda:
1. `cite-bibliography` heading (cristalino=0, vanilla=1) — **RESOLVIDO** (S-size fix).
2. `outline-toc` heading (cristalino=5, vanilla=6) — causa raiz identificada; **M-size scope-out P479**.

Após fix da bibliografia: Matches=50, Diffs=1.

---

## 2. Sub-item A — Sonda de paridade (ADR-0108)

### 2.1 Execução da suite

```
cd lab/parity
RUST_MIN_STACK=33554432 cargo test --test structural_parity -- --nocapture
```

Resultado bruto:

```
=== P206C — Matriz de paridade estrutural (pós-fix P479) ===
Total ficheiros corpus:   46
Includes (testados):      28
Skips:                    18
Errors:                   22
Comparações:              73
  - Matches:              50
  - Diffs:                1
```

### 2.2 Verificação dos 3 INCLUDE-com-diff P206D

| Divergência P206D | Estado P479 |
|-------------------|-------------|
| `equation` selector namespace | Pré-existente; 22 errors (todos vanilla rejeitando `equation` standalone). Arquitectónico. |
| `cite-bibliography` heading | **RESOLVIDO P479** (cristalino=1 = vanilla=1 pós-fix). |
| `outline-toc` heading | Diff mantido (cristalino=5, vanilla=6). Causa raiz identificada (ver §3.2). |

### 2.3 Verificação dos 10 SKIP-feature P206D

Os 10 ficheiros `semantic/` mantêm-se SKIP-feature. P467 (`Selector::Where`) e P474 (show-regex) implementam funcionalidade que resolveria os skip semânticos, mas os ficheiros de corpus `semantic/` são eval-fixtures (ex: `bool-true.typ`, `closure-aplicada.typ`) sem elementos introspectionáveis. SKIP-feature mantido.

Corpus cresceu de 36 → 46 (10 novos ficheiros adicionados P465–P477). Os 10 novos ficheiros `visual/` e `semantic/` são INCLUDE (visual/*) ou SKIP-feature (semantic/*).

---

## 3. Sub-item B — Análise dos diffs e materialização

### 3.1 Diff cite-bibliography (RESOLVIDO — S-size)

**Sonda** (`file:line`):

| Medição | Resultado | file:line |
|---------|-----------|-----------|
| Corpus `cite-bibliography.typ` query `heading` — cristalino | count=0 | pré-P479 |
| Corpus `cite-bibliography.typ` query `heading` — vanilla | count=1 | vanilla 0.14.2 |
| `BibliographyElem.title` default pré-P479 | `None` | `structural.rs:1376` |
| `walk` arm `Content::Bibliography` recursivo em `e.title`? | **Sim** | `introspect.rs:1164–1168` |
| `bibliography::layout` renderiza `e.title`? | **Sim** | `bibliography.rs:29–32` |

**Causa**: vanilla gera heading de título "Bibliography" por defeito. Cristalino definia `title = None`, logo sem heading no introspector pré-layout.

**Fix S**: em `native_bibliography` (`structural.rs:1376`), quando `title:` não é especificado:
```rust
// P479 — default heading "Bibliography"
None => Some(Content::heading(1, Content::text("Bibliography"))),
```

**Porque funciona**: `walk` arm `Content::Bibliography` já recursivo em `e.title` (`introspect.rs:1164`). Com `e.title = Some(Content::heading(1, ...))`, o walk encontra um `Content::Heading` e emite payload Heading → contado. `bibliography::layout` renderiza `e.title` via `layout_content` sem alteração.

**Resultado pós-fix**: cristalino=1 = vanilla=1 → match.

### 3.2 Diff outline-toc (SCOPE-OUT — M-size)

**Sonda** (`file:line`):

| Medição | Resultado | file:line |
|---------|-----------|-----------|
| Corpus `outline-toc.typ` headings explícitos | 5 (`= Introdução` ... `= Conclusão`) | corpus file |
| Cristalino query `heading` | count=5 | pós-P479 |
| Vanilla query `heading` | count=6 | vanilla 0.14.2 |
| `outline::layout` emite heading de título? | **Sim**, durante layout | `outline.rs:57` |
| `walk` arm `Content::Outline` recursivo em title? | **Não** (P189B vazio) | `introspect.rs:1225` |

**Causa raiz**: vanilla conta o heading do título do `#outline()` ("Contents") via query pós-layout. Cristalino usa introspector pré-layout que não vê headings criados durante layout. O arm `Content::Outline` no walk (`introspect.rs:1225`) não recursivo em `e.title`. A correção exige mudança tripla:
- `native_outline` → store heading como title.
- `walk` arm `Content::Outline` → recursão em `e.title`.
- `outline::layout` → não criar heading separado se title já inclui heading.

**Classificação**: M-size (3 ficheiros, testes existentes afectados). Scope-out P479. Documentado em `SKIPS.md §3 P479`.

---

## 4. Testes (4 novos / 1 actualizado)

### L2 — `rules::stdlib::tests` (4 testes)

| Teste | Cobertura |
|-------|-----------|
| `native_bibliography_default_vazia` (actualizado) | `entries.is_empty()` + title é `Some(heading)` com texto "Bibliography" |
| `p479_native_bibliography_default_titulo_e_heading` | default title é `Content::Heading`, `plain_text = "Bibliography"` |
| `p479_native_bibliography_title_none_suprime_titulo` | `title: none` → `BibliographyElem.title = None` |
| `p479_native_bibliography_title_explicito_preservado` | `title: "Referências"` → `plain_text = "Referências"` |

### Parity suite — sentinel P479 (1 teste)

| Teste | Cobertura |
|-------|-----------|
| `p479_corpus_paridade_actualizado` | corpus=46; INCLUDE≥28; diffs==1 |

### Resultados

```
rules::stdlib::tests::p479_native_bibliography_default_titulo_e_heading   ok
rules::stdlib::tests::p479_native_bibliography_title_none_suprime_titulo  ok
rules::stdlib::tests::p479_native_bibliography_title_explicito_preservado ok
p479_corpus_paridade_actualizado                                           ok

test result: ok. 24 passed (bibliography cluster); 11 passed (structural_parity)
```

---

## 5. Arquivos alterados

### Spec L0 (actualizada)

- `00_nucleo/prompts/engine/stdlib/structural.md`:
  - §`native_bibliography` — argumento `title` documentado com comportamento P479.
  - Hash: `ff17070d` (via `crystalline-lint --fix-hashes`).

### Código L1

- `01_core/src/engine/stdlib/structural.rs`:
  - `native_bibliography` — default title P479.
  - `@prompt-hash` → `eb51f7a4`.
- `01_core/src/engine/stdlib/mod.rs`:
  - `native_bibliography_default_vazia` actualizado.
  - 3 testes P479 adicionados.

### Parity suite

- `lab/parity/tests/structural_parity.rs`:
  - Corpus sentinel actualizado: 36 → 46 (P479).
  - Sentinel `p479_corpus_paridade_actualizado` adicionado.
- `lab/parity/SKIPS.md`:
  - §3 actualizado (P479 status table).
  - §4 matriz actualizada.
- `lab/parity/reports/latest.md`: actualizado P479.
- `lab/parity/reports/2026-06-27-passo-479.md`: criado.

### Não alterados

- `introspect.rs` — walk arm Bibliography já recursivo.
- `engine/layout/bibliography.rs` — já renderiza `e.title`.
- `engine/layout/outline.rs` — outline-toc scope-out.

---

## 6. `crystalline-lint` resultados

```
crystalline-lint --fix-hashes .
  Fixed 1 file:
    ./01_core/src/engine/stdlib/structural.rs  → eb51f7a4
  Re-running analysis... ✅ 0 drift warnings remaining

crystalline-lint .
  ✅ 0 erros V1–V14.
  Warnings V7 pré-existentes (prompts órfãos não relacionados com P479).
```

---

## 7. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **Outline title heading** | Fix M-size (walk/layout/native_outline); scope-out P479. Causa raiz documentada. |
| **`equation` selector namespace** | vanilla rejeita selector standalone; cristalino aceita. Arquitectónico; pré-existente. |
| **User-provided bibliography title como heading** | `title: "Custom"` não é wrapped em heading pelo cristalino (vanilla wraps). Scope-out. |
| **Paridade textual de título bibliography** | Cristalino usa "Bibliography"; vanilla pode usar localização ("References" em EN-US, etc.). Count match; texto scope-out. |

---

## 8. Critério de fecho

- [x] `typst --version` confirma 0.14.2.
- [x] Suite `lab/parity/` executa sem panic.
- [x] Matriz P479 produzida: 46 corpus, 28 INCLUDE, 50 matches, 1 diff.
- [x] INCLUDE ≥ 23 (threshold P206D) — cumprido com margem (28 INCLUDE).
- [x] 3 INCLUDE-com-diff P206D verificados: `equation` (pré-existente), `cite-bibliography` (RESOLVIDO), `outline-toc` (causa raiz documentada).
- [x] 18 SKIP-feature verificados: todos mantêm-se; P467/P474 não resolve corpus eval-only.
- [x] 1 gap S materializado (`bibliography` título padrão como heading).
- [x] Outline-toc diff: causa raiz M-size documentada e scoped-out.
- [x] 4 testes novos/actualizados verdes.
- [x] Sentinel `p479_corpus_paridade_actualizado` verde.
- [x] `cargo build --workspace` verde.
- [x] `crystalline-lint .` zero erros V1–V14.
- [x] `SKIPS.md` actualizado com estado P479.
- [x] Relatório versionado em `lab/parity/reports/2026-06-27-passo-479.md`.

---

## 9. Estado pós-P479

| Indicador | Estado |
|-----------|--------|
| DEBTs activos com critério de fecho | 0 |
| Trilhas completas | 1, 2, 3, 4, 7, 8 |
| Trilhas pendentes | 5 (épico XL), 6 (4/5) |
| Paridade | 50/73 comparações match; 1 diff documentado |
| Corpus paridade | 46 ficheiros |
| Matches vs P206D | ~20 → 50 (+30) |
| Diffs vs P206D | 3 → 1 (-2) |
| ADR-0083 operadores cor | TOTALMENTE FECHADO (P476+P477) |
| P295 Footnotes | FECHADO (P304/P305) |
| **P479** | **FECHADO** |

---

## 10. Próximo passo recomendado

Com P479 concluído e paridade em estado excelente (50/73 matches, 1 diff documentado):

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P480-A** | Outline-toc título heading fix — M-size (3 ficheiros identificados P479) | M |
| **P480-B** | `equation` selector namespace — expandir parsing para `math.equation` | S |
| **P480-C** | Épico Trilha 5 (shaping rustybuzz) — maior impacto qualitativo | XL |
| **P480-D** | Audit de cobertura final e estado do projecto | XS |
