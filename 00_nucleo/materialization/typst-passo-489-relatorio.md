# Estado Final P489 — Audit empírico pós-P488

> **Passo:** 489
> **Data:** 2026-06-29
> **Tipo:** Audit empírico — zero código de produção
> **Fix inline:** `vanilla_query_summary` indefinida (XS)

---

## Tabela de estado final

| Área | Estado | Detalhe |
|------|--------|---------|
| Build (`cargo build --workspace`) | ✅ | 0 erros |
| Testes workspace | ✅ | 3435+526+24+2+21+2 passed, 0 failed |
| `crystalline-lint .` | ✅ | 0 violations |
| Suite de paridade | ✅ (pós-fix XS) | 17/17 testes — ver §Fix inline |
| DEBTs com critério activo | ✅ | 0 (DEBT-9 = instrumento; stylechain = sem prazo técnico) |
| L0 drift (V5) | ✅ | 0 warnings |
| Funcionalidades chave | ✅ | Todas as contagens > 0 — ver Grupo 5 |
| Scope-outs permanentes | ✅ | Confirmados via `grep` — ver Grupo 6 |
| Sentinelas p4xx (core + infra) | ✅ | 83+25 passam, 0 FAILED |
| ADR-0120 | ✅ | ACEITE; Fases 1+2+3 todas FECHADAS |

---

## ADR-0108 — Lente "medir antes de decidir"

Sondas executadas com output registado antes de qualquer classificação.

---

## Grupo 1 — Build e testes

```
cargo build --workspace        → 0 erros
cargo test --workspace         → 3435 (L1) + 526 (L3) + 24 + 2 + 21 + 2 passed
                                  0 failed em todas as suites
crystalline-lint .             → ✓ No violations found
```

---

## Grupo 2 — Suite de paridade

**Pré-fix:** suite não compilava — `vanilla_query_summary` indefinida (ver §Fix inline).

**Pós-fix:** 17/17 testes passed (11.26s). Vanilla CLI ausente em CI → sentinelas entram em modo skip graceful. Corpus 48 ficheiros confirmado.

---

## Grupo 3 — DEBTs activos

**DEBT.md** (`00_nucleo/diagnosticos/debt/DEBT.md`):
- DEBT-9 — "Cobertura de paridade — tracking contínuo" → instrumento permanente; não fecha por design.
- Todos os restantes DEBT-1…60b marcados FECHADO/ENCERRADO/RESOLVIDO/DISSOLVIDO/ACEITE.

**Anexos:**
- `debt-layout-noop-dinamico.md` → Estado: **FECHADO**
- `debt-stylechain-nao-materializada.md` → Estado: **aberto** (Pré-condição de fecho: "nenhuma técnica imediata; depende de prioridade humana" — sem prazo activo)

**Critério cumprido:** 0 DEBTs com critério de fecho activo.

---

## Grupo 4 — L0 drift

```
crystalline-lint . → ✓ No violations found
```

0 warnings V5 (hash drift). Todos os `@prompt-hash` em L1-L4 estão sincronizados com os L0 em `00_nucleo/prompts/`.

---

## Grupo 5 — Funcionalidades chave

| Feature | Sonda | Contagem |
|---------|-------|---------|
| Trilha 5 — shaping (`FrameItem::TextShaped`, `ShapedGlyph`, `shape_document`) | `grep -rn ... 01_core/ 03_infra/` | **61** |
| Trilha 6 — LoF/LoT pages (`figure_page_numbers`, `known_figure`) | `grep -rn ... 01_core/` | **50** |
| Trilha 8 — marcadores (`ListMarker`, `EnumNumbering`) | `grep -rn ... 01_core/` | **68** |
| Trilha 4 — cor (`fn lighten`, `fn darken`, `fn saturate`, `fn mix`, `fn negate`) | `grep -rn ... 01_core/` | **5** |
| Symbol/sym (`build_sym_dict`, `SYM_TABLE`, `Value::Symbol`) | `grep -rn ... 01_core/` | **27** |
| unicode-bidi (`BidiInfo`, `bidi_runs`) | `grep -rn ... 03_infra/` | **16** |

Todas as contagens > 0. Funcionalidades presentes.

---

## Grupo 6 — Scope-outs permanentes confirmados

| Scope-out | Sonda | Resultado |
|-----------|-------|---------|
| `FrameItem::Text` preservado (fallback deprecated) | `grep "FrameItem::Text {" 01_core/src/` | ✅ Presente em `layout_types.rs` (deprecated, usado em testes de fallback) |
| `y_offset` em `stream.rs` (scope-out emit) | `grep y_offset 03_infra/src/export/stream.rs` | ✅ Apenas em dados de teste (`y_offset: 0`); nenhuma lógica de produção |
| `dir: rtl` em stdlib (scope-out) | `grep "dir.*rtl\|Direction::RTL" 01_core/src/engine/` | ✅ Apenas em mensagem de erro do `stack(dir:)`, não em `#set text(dir:)` |
| `ColorSpace` user-facing | `grep ColorSpace 01_core/src/engine/stdlib/` | ✅ Apenas em `gradients.rs` (Oklab interno); sem funcs user-facing |
| `native_color_saturate` presente (P477) | `grep native_color_saturate 01_core/src/` | ✅ Definida em `color.rs:164`, registada na dict |

Todos os scope-outs confirmados. Nenhuma implementação parcial inconsistente detectada.

**Nota lente ADR-0108** — `stack(dir:)` aceita `"rtl"` como *argumento de stack* (eixo btt/ltr/rtl). `#set text(dir: rtl)` é uma *set rule de text* — entidade diferente. A grep não nega o scope-out de `text(dir:)`.

---

## Grupo 7 — Sentinelas p4xx

```
cargo test -p typst-core -- p468 p469 p470 p471 p472 p473 p474 p475 p476 p477
                            p480 p482 p483 p484 p485 p486 p488
→ test result: ok. 83 passed; 0 failed (3352 filtered out)

cargo test -p typst-infra -- p482 p483 p484 p485 p486 p488
→ test result: ok. 25 passed; 0 failed (507 filtered out)
```

Todos os sentinelas com prefix `p4xx` que existem passam.

---

## Grupo 8 — ADR-0120

```
Status: ACEITE (P482 2026-06-27; materializado em P482)
Fase 1 (P482) — FECHADA
Fase 2 (P483) — FECHADA
Fase 3 (P484+) — FECHADA
y_offset scope-out: documentado permanente
```

ADR-0120 em estado ACEITE; Fases 1–3 documentadas e fechadas; scope-outs permanentes listados (`y_offset`, RTL stdlib).

---

## Fix inline XS — `vanilla_query_summary` indefinida

**Causa (ADR-0108 § medir antes de decidir):**

Sonda `cargo test --manifest-path lab/parity/Cargo.toml --test structural_parity`:
```
error[E0425]: cannot find function `vanilla_query_summary` in this scope  (×3)
```

Medição `git log --oneline lab/parity/tests/structural_parity.rs`:
- `vanilla_query_summary` introduzida em P485 commit (`5b975acef`) como chamada mas nunca definida.
- Bug pré-existente; P488 propagou o padrão ao adicionar o sentinela p488.
- Não é regressão de P488 — P487-D não executou a suite de paridade.

**Decisão:** substituir os 3 sites pelo padrão estabelecido em p482/p483 (`run_typst_query` + `compare_query_outputs`) — mesmo comportamento semântico, funções já importadas no ficheiro.

**Ficheiro alterado:** `lab/parity/tests/structural_parity.rs` — linhas p485, p486, p488.

**Verificação pós-fix:**
```
cargo test --manifest-path lab/parity/Cargo.toml --test structural_parity
→ test result: ok. 17 passed; 0 failed; finished in 11.26s

crystalline-lint .  →  ✓ No violations found
```

---

## Conclusão

Todos os 8 grupos auditados. 0 itens ❌ sem resolução:

- 1 item ❌ detectado (suite de paridade não compilava) → fix XS executado inline.
- Pós-fix: estado totalmente verde.

O projecto está no estado declarado nos relatórios P488:

| Indicador | Estado confirmado empiricamente |
|-----------|--------------------------------|
| Build | ✅ 0 erros |
| Testes | ✅ 3435+526+24+2+21+2 passed, 0 failed |
| crystalline-lint | ✅ 0 violations |
| Paridade | ✅ 17/17 (vanilla CLI ausente → skip graceful) |
| DEBTs activos | ✅ 0 (DEBT-9 = instrumento; stylechain = sem prazo) |
| L0 drift | ✅ 0 V5 warnings |
| Funcionalidades chave | ✅ todas presentes |
| Scope-outs | ✅ todos confirmados e consistentes |
| Sentinelas p4xx | ✅ 83+25 passam |
| ADR-0120 | ✅ ACEITE, Fases 1–3 fechadas |
