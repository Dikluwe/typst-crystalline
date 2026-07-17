# P428 — Fecho de débitos: DEBT-59 (CLI `--full-error`) + DEBT-60b (outline supplement)

> **Data:** 2026-06-23  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch:** Tekt  
> **Foco:** Fechar 2 débitos técnicos acumulados — prioridade S/XS-S

---

## ADR-0108 — Medir antes de decidir

### FASE A.0 — Sonda DEBT-59

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `RunIntent.full_error` existe em `02_shell/src/cli.rs`? | Campo `full_error: bool`, default `false` | ✅ |
| `Args` em `cli.rs` tem `--full-error`? | Ausente; requer `Arg::new("full-error")` | ❌ |
| `parse()` mapeia `full_error` para `RunIntent`? | Hoje fixa `false` literal | ❌ |
| `eval_with_full_error(..., bool)` existe em L1? | P350c; testável via `eval_with_full_error(..., true)` | ✅ |
| Fio `RunIntent → L1` existe? | `04_wiring/main.rs` ignora via `..` (spread struct) | ❌ |
| Bloqueadores externos? | Nenhum; não depende de hayagriva/rustybuzz/benchmark | ✅ |

**Reclassificação:** DEBT-59 = **S** (1 campo + 1 arg + 1 fio; ~1-2h).

### FASE A.0 — Sonda DEBT-60b

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `outline.rs` emite `resolved_text` "Secção {n}"? | Heading com supplement embutido | ✅ |
| Vanilla não emite supplement para heading no outline? | Probe P359 confirmado | ✅ |
| Fix identificado em P359? | Fiar Location/número em `HeadingForToc` → outline mostra número sem "Secção" | ✅ |
| Teste lacuna? | Nenhum teste assertia o número no outline (P359 notou) | ✅ |
| Bloqueadores? | Nenhum; toca apenas `introspect.rs` + `layout/outline.rs` | ✅ |

**Reclassificação:** DEBT-60b = **XS-S** (refino de texto em outline; ~30-60min).

---

## ADR-0107 — Paridade linguagem

**DEBT-59:** `typst --full-error doc.typ` com erro de recursão `#show` deve produzir 3º hint classificado; sem flag, mensagem byte-idêntica ao vanilla.

**DEBT-60b:** `#outline()` para heading `= A <lbl>` deve mostrar número puro (ex: "1.1") sem prefixo "Secção"; paridade vanilla literal.

---

## ADR-0109 — Atomização forma B

### DEBT-59 — 3 toques pontuais

1. `02_shell/src/cli.rs` — adicionar `Arg::new("full-error").long("full-error").action(ArgAction::SetTrue)` em `Args`
2. `02_shell/src/cli.rs` — `parse()` mapear `args.full_error` para `RunIntent.full_error`
3. `04_wiring/main.rs` — propagar `run_intent.full_error` até `eval_with_full_error(..., run_intent.full_error)` pelo caminho interno de L3 (`03_infra/pipeline.rs`)

### DEBT-60b — 2 toques pontuais

1. `01_core/src/engine/introspect.rs` — `headings_for_toc` armazenar `HeadingForToc { location, number: Option<String>, body }` em vez de `body` clonado com texto materializado
2. `01_core/src/engine/layout/outline.rs` — consumir `number` puro ao renderizar linha do TOC; supplement "Secção" removido do outline

---

## Decisões arquiteturais

| Decisão | Opção escolhida | Justificativa |
|---------|----------------|---------------|
| DEBT-59: path do fio | Interno via `pipeline.rs` (não público) | ADR-0107 §"decisão humana" — não expor `full_error` na API pública `compile_to_pdf_bytes` |
| DEBT-59: default da flag | `false` (off-by-default) | Paridade vanilla; zero impacto em comportamento existente |
| DEBT-60b: struct `HeadingForToc` | Reusar `CounterState::display_value(kind)` para número, não `resolved_text` | Separa texto de corpo (supplement) do número no TOC |
| DEBT-60b: backward compat | Preservar supplement em corpo do heading (não no outline) | Paridade vanilla: heading corpo continua "Secção 1.1" se `supplement` configurado; outline mostra só "1.1" |

---

## Scope-out explícito

- **DEBT-59:** formato rico de `Span` (linha/coluna) — continua como `Span(N)` opaco; não é parte deste fecho
- **DEBT-60a:** contador de heading divergente (`1.1` vs `0.1`) — **aceito como divergência consciente** per P359; não é corrigido aqui
- **DEBT-60b:** número no outline para headings sem `numbering` — scope-out (vanilla também omite)

---

## Critério de fecho

- [ ] `typst --full-error` produz 3º hint em erro de recursão `#show`
- [ ] Sem `--full-error`, mensagem de erro é byte-idêntica ao vanilla (baseline)
- [ ] `#outline()` não emite "Secção" para heading entries
- [ ] Teste E2E asserte número puro no outline (fecha lacuna P359)
- [ ] DEBT-59 e DEBT-60b reclassificados como **FECHADO** em `DEBT.md`
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations

---

## Próximo passo

Quando fechar o P428, continuamos com **DEBT-63** (cache de style em BibliographyElem — S-M) ou **DEBT-50** (show selector latente — S). Indique se quer ajustar o escopo do P428.
