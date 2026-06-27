# Relatório P478 — Sonda de estado geral + Verificação Footnotes Fase 2

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P478 (Sonda de estado geral + verificação de estado real)
**Materialização:** Sonda only — zero código produzido

---

## 1. Resumo

**Sub-item A — Sonda de estado geral:**
Varredura de todos os itens indicados como potencialmente pendentes no spec P478.
**Resultado:** todos os itens da lista estão implementados. DEBT.md tem apenas
DEBT-9 como item activo (rastreamento contínuo de paridade; sem critério de fecho
por natureza). Inventário de débitos: **efectivamente limpo** pós-P477.

**Sub-item B — Footnotes Fase 2:**
Sub-item B é um **falso pendente**. O spec P478 assumia P295.1 como "body
descartado silenciosamente" — mas **P304 já materializou o rodapé completo** e
**P305 materializou overflow multi-página**. `flush_pending_footnote_bodies`
em `cursor.rs:377` implementa greedy fit + defer; 12 testes verdes. P295.1 e
P295.2 **estão fechados desde P304/P305**.

**Zero código produzido neste passo.** O estado é melhor do que o spec assumia.

---

## 2. Sub-item A — Sonda de estado geral (ADR-0108)

### 2.1 Itens probed com `grep`/`file:line`

| Item | Hipótese P478 | Resultado sonda | file:line |
|------|---------------|-----------------|-----------|
| `calc` module (trig, log, exp) | "Provável stub ou ausente" | **Implementado** (P433) | `stdlib/calc.rs:48–61` |
| `Value::Bytes` stdlib | "P398 modelou tipo; stdlib scope-out" | **Implementado** (P398/DEBT-62) | `entities/value.rs:106` |
| `Value::Decimal` | "Modelado mas sem funcs user-facing" | **Implementado** (P399) | `entities/value.rs:110` |
| `Value::Duration` | "Modelado mas sem funcs user-facing" | **Implementado** (P400) | `entities/value.rs:114` |
| `Value::Version` | "Modelado mas sem funcs user-facing" | **Implementado** (P401) | `entities/value.rs:118` |
| `Terms`/`TermItem` layout | "Listado em roteiro como pendente" | **Implementado** (P154B, P380) | `layout/terms.rs:18`, `term_item.rs:21` |
| `Raw` layout refinos | "Raw tem highlighting scope-out" | **Implementado** (P381, atomizado) | `layout/raw.rs:15` |
| `Divider` layout | "Linha horizontal (simples)" | **Implementado** (P154B, P381) | `layout/divider.rs:17` |
| `Underline`/`Strike`/`Overline` | (não listado — bonus probe) | **Implementado** (P286) | `layout/decorations.rs:17` |
| DEBT.md itens abertos | "6 abertos pós-P282" | **0 activos** (DEBT-9 = rastreamento contínuo) | `DEBT.md:644` |

**Conclusão:** Nenhum item pendente material identificado.

### 2.2 Estado de DEBTs

Último saldo reflectido em DEBT.md: **6 abertos pós-P282** (2026-05-18). Desde
então, closures acumuladas:

| DEBT | Fecho | Passo |
|------|-------|-------|
| DEBT-42 (`get_unchecked` scanner) | FECHADO excepção permanente ADR-0116 | P443 |
| DEBT-43 (linter type-level whitelist) | FECHADO implementado | P440 |
| DEBT-57 (specs L0 ausentes ~70 fns) | FECHADO todos os subsets | P438 |
| DEBT-61 (F-5b de-bake TextStyle) | FECHADO em 2 fatias | P371+P373 |
| DEBT-62 (`Value::Bytes`) | FECHADO implementado | P398 |
| DEBT-2 (closures eager vs lazy) | ENCERRADO premissa inválida | P458 |
| DEBT-55 (Bibliography+Cite XL) | FECHADO ADR-0062 IMPLEMENTADO | P439 |
| DEBT-63 (`resolved_style` em BibElem) | FECHADO tabela lateral | P429 |
| DEBT-59 (flag `--full-error` CLI) | FECHADO fio L3→L1 | P428 |
| DEBT-60 (heading counter + outline) | FECHADO (60b) / aceite (60a) | P428 |
| DEBT-50 (show Strong/Emph origem) | ENCERRADO implementado | P431 |

**DEBT-9** ("Cobertura de paridade — rastreamento contínuo") permanece activo por
natureza — não é um item a fechar, é um instrumento de monitorização (`DEBT.md:644`).

**Total DEBTs activos com critério de fecho:** **0**.

### 2.3 Estado das Trilhas (actualizado pós-P477)

| Trilha | Estado pós-P477 | Observação |
|--------|-----------------|------------|
| 1 — Numeração | COMPLETA | — |
| 2 — Referências cruzadas | COMPLETA | — |
| 3 — Selectors show rules | COMPLETA | — |
| 4 — Color/Gradient/Visualize | COMPLETA | ColorSpace runtime = scope-out permanente |
| 5 — Shaping rustybuzz | Épico XL declarado | stub confirmado P476 |
| 6 — Bibliografia Fase 2 | 4/5 | LoF/LoT page numbers = scope-out longo prazo |
| 7 — Layout multi-região | COMPLETA | — |
| 8 — Refinos stdlib | COMPLETA | — |

---

## 3. Sub-item B — Verificação Footnotes Fase 2

### 3.1 Sonda: estado real vs hipótese P478

O spec P478 descreveu P295.1 (body no rodapé) como "body silenciosamente
descartado" e P295.2 como "scope-out". As sondas revelam o oposto:

| Sonda | Hipótese P478 | Resultado | file:line |
|-------|---------------|-----------|-----------|
| `pending_footnote_bodies` existe no Layouter? | "a criar em P478" | **Sim** — P304 | `layout/mod.rs:318` |
| `footnote::layout` acumula body? | "body descartado" | **Sim** — `push((n, body))` | `layout/footnote.rs:27` |
| `flush_pending_footnote_bodies` implementado? | "a criar em P478" | **Sim** — P304/P305 | `layout/cursor.rs:377` |
| Overflow multi-página suportado? | "scope-out P295.2" | **Sim** — P305 greedy fit+defer | `cursor.rs:409–438` |
| Testes P304/P305 existem? | "a criar em P478" | **Sim** — 12 testes verdes | `layout/tests.rs:13235–13535` |

### 3.2 O que P304 implementou (resumo)

`flush_pending_footnote_bodies` (`cursor.rs:377`):
1. Calcula `top_safe` (máximo Y dos itens actuais — evita overlap).
2. Calcula `available_h = (page_bottom - top_safe).max(0.0)`.
3. **Pass 1 (measure):** para cada body acumulado, mede altura via
   `layout_sub_frame_with_width`. Greedy fit: cabem → `measured`; não cabem →
   `remainder`.
4. Fallback defensivo: se único body > página inteira, emite para evitar loop
   infinito (paralelo ao `forwarded_count` limit do P251).
5. **Pass 2 (place):** coloca measured top-down a partir de `area_bot - acc_h`.
6. `remainder` → reinserido em `pending_footnote_bodies` para próxima página.

### 3.3 O que P305 implementou

`finish()` (`mod.rs:1101`): se `pending_footnote_bodies` não vazio após flush
da última página, itera `new_page()` com limite defensivo até buffer esvaziado
(paridade P251 `forwarded_count`).

### 3.4 Testes existentes (12 verdes)

| Teste | Cobertura |
|-------|-----------|
| `p304_footnote_body_presente_no_documento` | body aparece nos FrameItems |
| `p304_footnote_body_no_rodape_y_alto` | Y do body > Y do corpo principal |
| `p304_marker_inline_acima_do_body` | marker `[1]` antes do body em Y |
| `p304_multiplos_footnotes_bodies_empilhados` | 2 bodies empilhados |
| `p304_documento_sem_footnote_sem_impacto` | regressão zero sem footnotes |
| `p304_footnote_body_complex_content_renderizado` | body com Sequence |
| `p305_overflow_body_grande_distribui_no_documento` | body grande → 2+ páginas |
| `p305_overflow_multiplos_bodies_todos_preservados` | todos bodies preservados |
| `p305_regressao_p304_single_page_preservado` | bit-exact single page |
| `p305_regressao_documento_sem_footnote_bit_exact` | bit-exact sem footnotes |
| `p305_body_gigante_nao_loop_infinito` | body > página → emite sem loop |
| `p305_bug_fix_overflow_sem_overlap_no_top` | clamp Y evita overlap |

```
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 3408 filtered out
```

**P295.1 e P295.2 estão FECHADOS desde P304/P305.**

---

## 4. Arquivos alterados

**Nenhum.** P478 é sonda pura. Zero código produzido; zero spec L0 actualizada;
`crystalline-lint` não precisa de correr (sem modificações).

---

## 5. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **Numeração de footnote por página** | cristalino usa sequência global; vanilla recomeça por página |
| **`numbering:` configurável** | scope-out declarado P295 |
| **Separador visual antes das footnotes** | vanilla emite linha horizontal; cristalino usa marker `[N] ` sem linha |
| **Shaping rustybuzz real** | Trilha 5 épico XL; declarado P476; sem data |
| **LoF/LoT page numbers** | Trilha 6 scope-out longo prazo |
| **ColorSpace runtime** | ADR-0083 scope-out permanente |
| **Constantes nomeadas além de CSS basic** | scope-out pós-P477 |

---

## 6. Critério de fecho

- [x] Sonda A: DEBT.md varrido — 0 itens activos com critério de fecho.
- [x] Sonda A: todos os itens da tabela P478 probed com `file:line` — todos
      implementados.
- [x] Sonda A: tabela de trilhas actualizada — Trilhas 1–4, 7, 8 COMPLETAS;
      Trilha 5 épico XL; Trilha 6 4/5.
- [x] Sonda B: `pending_footnote_bodies` — confirmado em `mod.rs:318`.
- [x] Sonda B: `flush_pending_footnote_bodies` — confirmado em `cursor.rs:377`.
- [x] Sonda B: overflow P295.2 — confirmado em `cursor.rs:409–438` + `mod.rs:1101`.
- [x] Sonda B: 12 testes P304/P305 verdes — confirmado.
- [x] **P295.1 FECHADO** desde P304 (verificado P478).
- [x] **P295.2 FECHADO** desde P305 (verificado P478).
- [x] **DEBT.md efectivamente limpo** pós-P477 (0 itens activos com critério de fecho).

---

## 7. Próximo passo recomendado

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P479-A** | Sonda paridade geral — varrer diferenças cristalino vs vanilla via corpus `lab/parity/` | XS (sonda) |
| **P479-B** | SmartQuote refino — verificar cobertura actual de `"..."` → `"…"` locale-aware | XS |
| **P479-C** | Tiling sonda — verificar se `Value::Tiling` activo ou stub | XS |
| **P479-D** | Épico Trilha 5 (shaping rustybuzz) | XL (épico dedicado) |

**Recomendação:** P479-A (sonda paridade) para identificar empiricamente o que
produz diffs vs vanilla antes de escolher o próximo alvo concreto.
