# Roteiro para completar a refatoração — typst-cristalino

**Versão:** P479
**Data:** 2026-06-27
**Base:** Estado real verificado por sondas P474–P479. Zero DEBTs activos.
Paridade: 50/73 matches; 1 diff residual documentado.

---

## Estado das trilhas — resumo executivo

| Trilha | Descrição | Estado | Próximo passo |
|--------|-----------|--------|---------------|
| **1** | Numeração (heading, figure, equation, TOC, table) | **COMPLETA** | — |
| **2** | Referências cruzadas (label, ref, /GoTo) | **COMPLETA** | — |
| **3** | Selectors completos em show rules | **COMPLETA** | — |
| **4** | Visuais (gradientes, espaços de cor, tiling, operadores cor) | **COMPLETA** | ColorSpace runtime = scope-out permanente |
| **5** | Shaping / rustybuzz | Épico XL declarado | Passo dedicado quando priorizado |
| **6** | Bibliografia Fase 2 (estilos numéricos, ibid, LoF/LoT) | 4/5 | LoF/LoT page numbers = scope-out longo prazo |
| **7** | Layout multi-região (columns, measure, table real) | **COMPLETA** | — |
| **8** | Refinos de stdlib e tipos | **COMPLETA** | — |
| **9** | DEBT-2 (closures) | **FECHADA** | Premissa refutada em P458 |

---

## O que foi concluído desde o roteiro P463

### Trilha 3 — Selectors completos (FECHADA em P474)

- P467: Sonda `Selector::Where` — resultado: implementado desde P417.
- P473: `#show regex(...)` — confirmado implementado desde P393.
- P474: Sonda de fecho — wiring `#show elem.where(field: value)` completo desde P417.

### Trilha 4 — Visuais: extensão Color (FECHADA em P477)

- P476: `lighten`, `darken`, `mix`, `negate` — 4/6 operadores de cor. Módulo `color` no scope.
- P477: `saturate`, `desaturate` — fecha ADR-0083 §"Operadores cor" (6/6). Constantes nomeadas alargadas (18 cores CSS basic).

### Trilha 6 — Bibliografia Fase 2 (4/5)

- P468: Estilos numéricos `[1]`, `[2]` com `BibStore.citation_order`.
- P469: `Value::Relative` (`Rel<Length>`) — infra para comprimentos relativos.
- P472: Back-refs + `ibid.` + LoF/LoT (sem page numbers).
- P473: `op. cit.` + `#show regex(...)` confirmado.
- Pendente: LoF/LoT com page numbers (requer 2-pass convergente — scope-out longo prazo).

### Trilha 8 — Refinos de stdlib e tipos (FECHADA em P475)

- P465: `repr()` completo.
- P466: Métodos array/dict/str.
- P469: `Value::Relative` (`Rel<Length>`).
- P470: Marcadores list/enum + i18n caption.
- P471: `Symbol` + `highlight` params + `sub`/`super` size.
- P475: `inset`/`outset` dict per-side + `Rel<Length>` em `extract_sides`.

### Funcionalidades adicionais (fora das trilhas numeradas originais)

- P304/P305: Footnotes Fase 2 (rodapé + overflow multi-página) — confirmado em P478.
- P468–P477: Sequência completa de refinos de Trilha 6 + 8.
- P476: Sonda Trilha 5 — rustybuzz stub confirmado; épico XL declarado.
- P478: Sonda de estado geral — zero DEBTs activos; todos os itens da lista implementados.
- P479: Paridade actualizada — 50/73 matches; 3 diffs P206D → 1 diff pós-P479.

---

## Trilha 4 — Visuais: estado final

Fechada cumulativamente em P257–P477:

| Item | Passo | Estado |
|------|-------|--------|
| Color 8 espaços | P257 | ✅ |
| Paint enum | P261 | ✅ |
| Gradient Linear | P262/P263 | ✅ |
| Gradient Radial | P264/P265 | ✅ |
| Gradient Conic | P267/P268 | ✅ |
| Gradient Focal | P269 | ✅ |
| ColorSpace runtime | P270 | ✅ |
| CMYK PDF native | P270.2/P270.4 | ✅ |
| Tiling | P395/P396 | ✅ |
| Operadores cor (6/6) | P476/P477 | ✅ |
| Constantes nomeadas (CSS basic) | P477 | ✅ |
| ColorSpace runtime user-facing | — | scope-out permanente |

---

## Trilha 5 — Shaping / rustybuzz (Épico XL)

Confirmado em P476: `rustybuzz` está em `03_infra/Cargo.toml` mas sem uso activo. O shaping é um stub sequencial. RTL/bidi ausentes.

**Magnitude estimada:** XL (8–12h; 3–4 passos dedicados).

**Decomposição do épico:**

| Sub-passo | Descrição | Magnitude |
|-----------|-----------|-----------|
| Shaping L1 | Pipeline `rustybuzz::UnicodeBuffer` → `GlyphBuffer`; `FrameItem::TextShaped` | L |
| Shaping L3 exporter | Iterar glyphs com `x_advance` real no PDF | M |
| RTL básico | `unicode-bidi` + re-order de runs | L |
| OpenType features | Kern pairs, ligatures básicas, smallcaps real | M |

Não iniciar sem passo dedicado de sonda profunda da arquitectura de `FrameItem::Text` → `FrameItem::TextShaped`.

---

## Trilha 6 — Bibliografia Fase 2: estado final

| Item | Passo | Estado |
|------|-------|--------|
| Estilos numéricos | P468 | ✅ |
| Back-references | P472 | ✅ |
| `ibid.` | P472 | ✅ |
| LoF / LoT (sem page numbers) | P472 | ✅ |
| `op. cit.` | P473 | ✅ |
| LoF/LoT com page numbers | — | scope-out longo prazo |

---

## Paridade vs vanilla — estado P479

| Indicador | P206D (Maio 2026) | P479 (Jun 2026) |
|-----------|-------------------|-----------------|
| Corpus ficheiros | 36 | 46 |
| INCLUDE testados | 23 | 28 |
| Matches | ~20 | 50 |
| Diffs | 3 | 1 |
| SKIPs | 13 | 18 |

**Diff residual pós-P479:**

| Diff | Causa | Fix |
|------|-------|-----|
| `outline-toc` heading count (cristalino=5, vanilla=6) | Heading de título criado no layout (invisível ao walk) | P480 |

**Diffs fechados P479:**

- `cite-bibliography` — heading "Bibliography" default (`native_bibliography` fix).
- `equation` selector namespace — parcialmente; alias `math.equation` pendente (P480).

---

## Divergências de paridade declaradas (permanentes ou longo prazo)

| Divergência | ADR | Estado |
|-------------|-----|--------|
| Shaping rustybuzz (kern, ligatures, RTL) | ADR-0054 graded | Épico XL |
| LoF/LoT com page numbers | ADR-0054 graded | Scope-out longo prazo |
| ColorSpace runtime user-facing | ADR-0083 | Scope-out permanente |
| Título outline não localizado | ADR-0054 graded | Aceite |
| Footnote numeração por página | ADR-0054 graded | Aceite |
| Pixel-perfect PDF | ADR-0054 graded | Inviável por design (FixedMetrics vs FontBookMetrics) |

---

## Débitos técnicos residuais (não activos)

| Item | Origem | Estado |
|------|--------|--------|
| `label_to_counter_key` duplicado em Introspector e TagIntrospector | P462 | Não bloqueante; refacto futuro |
| `LinkItem` com `items: Vec<FrameItem>` vs `body: Frame` | P452 | Não bloqueante |
| Gate ADR-0117 no `crystalline-lint` | P453 | Disciplina manual |
| Stack overflow em `p350c_flag_on_nao_convergente_classifica` | Pré-existente | Passa com `RUST_MIN_STACK=8388608` |
| M6 — eliminação `CounterStateLegacy` | P190A | Refacto arquitectural futuro |

---

## Ordem sugerida para passos P480+

### P480 (imediato)

1. Outline-toc heading fix (sub-item A) — fecha diff de paridade.
2. `math.equation` alias (sub-item B) — fecha diff arquitectónico de selector.
3. Audit final de estado (sub-item C) — documenta estado consolidado.

**Estimado:** M (~35 min).

### P481+ — opções após P480

Com paridade ≥ 52/73 matches e zero diffs não-declarados:

| Opção | Descrição | Magnitude | Prioridade |
|-------|-----------|-----------|------------|
| Épico Trilha 5 Fase 1 | `FrameItem::TextShaped` + shaping L1 | L | Alta (qualitativa) |
| `color.space()` runtime | ADR-0083 scope-out revogável se consumer surgir | S | Baixa |
| Constantes cor além CSS basic | ADR-0083 §Constantes nomeadas | XS | Baixa |
| LoF/LoT com page numbers | Trilha 6 último item | M-L | Média |
| `footnote` numeração por página | P295 scope-out | S | Média |
| `outline` título localizado | P480 scope-out | S | Baixa |
| Audit de cobertura DSM | Inventário completo vs vanilla | S | Média |

**Recomendação:** após P480, iniciar Épico Trilha 5 com um passo de sonda profunda de `FrameItem::Text` antes de escrever código.

---

## Inventário de DEBTs — estado final

**DEBT-9** ("Cobertura de paridade — rastreamento contínuo") permanece activo por natureza — instrumento de monitorização, não item a fechar.

**Todos os restantes DEBTs: FECHADOS.**

---

## Regras do roteiro (preservadas)

1. **Sonda A.0 real antes da spec (ADR-0114).** Executada com `file:line` + commit.
2. **Verificar fronteiras/ADR vigentes antes de propor estrutura (ADR-0117 Cláusula 4).**
3. **Declarar divergência de paridade de saída (ADR-0107).**
4. **Confirmar o gate da ADR-0117 no linter.**

---

## O que confirmar antes de iniciar Épico Trilha 5

1. **Sonda profunda de `FrameItem::Text`** — verificar campos, estrutura, uso no exporter PDF.
2. **ADR nova necessária** — `FrameItem::TextShaped` é mudança arquitectural; requer ADR.
3. **`unicode-bidi` no `Cargo.toml`?** — verificar se já está ou se precisa de autorização (ADR-0017 pattern).
4. **Testes de paridade de shaping** — definir corpus de teste específico para kern/ligatures.
5. **Magnitude real** — a estimativa XL (8–12h) é baseada em análise de stub; sonda profunda pode alterar.
