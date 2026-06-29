# Roteiro para completar a refatoração — typst-cristalino

**Versão:** P480
**Data:** 2026-06-27
**Base:** Estado verificado por sondas P474–P480. Paridade estrutural: 73/73 matches.
Zero DEBTs activos. Todas as trilhas 1–4 e 7–8 completas.

---

## Estado das trilhas — resumo executivo

| Trilha | Descrição | Estado | Observação |
|--------|-----------|--------|------------|
| **1** | Numeração (heading, figure, equation, TOC, table) | **COMPLETA** | P451–P461 |
| **2** | Referências cruzadas (label, ref, /GoTo) | **COMPLETA** | P460–P463 |
| **3** | Selectors completos em show rules | **COMPLETA** | P393/P417/P467/P473/P474 |
| **4** | Visuais (gradientes, cor, tiling, operadores) | **COMPLETA** | P257–P477 |
| **5** | Shaping / rustybuzz | **Épico XL** | Sonda P481 inicia o épico |
| **6** | Bibliografia Fase 2 | **4/5** | LoF/LoT page numbers = scope-out longo prazo |
| **7** | Layout multi-região | **COMPLETA** | P216–P221 + P250 |
| **8** | Refinos de stdlib e tipos | **COMPLETA** | P465–P475 |
| **9** | DEBT-2 (closures eager) | **FECHADA** | Premissa refutada P458 |

---

## Paridade vs vanilla — evolução histórica

| Marco | Corpus | INCLUDE | Matches | Diffs | Errors |
|-------|--------|---------|---------|-------|--------|
| P206D (Mai 2026) | 36 | 23 | ~20 | 3 | — |
| P479 (Jun 2026) | 46 | 28 | 50 | 1 | 22 |
| **P480 (Jun 2026)** | **46** | **28** | **73** | **0** | **0** |

**Paridade estrutural: 100% (73/73).**

---

## Trilha 4 — Visuais: estado final (COMPLETA)

| Item | Passo | Estado |
|------|-------|--------|
| Color 8 espaços (sRGB, Linear RGB, Oklab, Oklch, CMYK, RGBA, Luma, HSL) | P257 | ✅ |
| Gradient Linear / Radial / Conic / Focal | P262–P269 | ✅ |
| ColorSpace runtime | P270 | ✅ |
| CMYK PDF nativo | P270.2/P270.4 | ✅ |
| Tiling com fallback Color | P395/P396 | ✅ |
| Operadores cor (lighten/darken/mix/negate/saturate/desaturate) | P476/P477 | ✅ |
| Constantes nomeadas CSS basic (18 cores) | P477 | ✅ |
| **ColorSpace runtime user-facing** | — | scope-out permanente |

---

## Trilha 5 — Shaping / rustybuzz (Épico XL)

**Estado confirmado P476:** `rustybuzz` em `Cargo.toml` (ADR-0019) mas sem uso activo. Shaping é stub sequencial (`FrameItem::Text { text: EcoString }`).

**Decomposição do épico (sujeita a revisão pós-P481):**

| Sub-passo | Descrição | Magnitude estimada |
|-----------|-----------|-------------------|
| P481 | Sonda arquitectural + ADR-0120 | S |
| P482 | `ShapedGlyph` + `FrameItem::TextShaped` + pipeline rustybuzz (LTR latino) | L |
| P483 | Migração `export.rs` para `TextShaped`; `Text` deprecated | M |
| P484 | RTL básico via `unicode-bidi` | L |
| P485+ | OpenType features, kern explícito, ligatures, scripts complexos | XL |

**Pré-requisitos para P482:**
- ADR-0120 ACEITE (redigida em P481).
- Sonda confirma que `FontBook` expõe bytes da fonte.
- Sonda confirma API `rustybuzz::shape()` disponível na versão instalada.

---

## Trilha 6 — Bibliografia Fase 2: estado final (4/5)

| Item | Passo | Estado |
|------|-------|--------|
| Estilos numéricos `[1]`, `[2]` | P468 | ✅ |
| Back-references | P472 | ✅ |
| `ibid.` | P472 | ✅ |
| LoF / LoT (sem page numbers) | P472 | ✅ |
| `op. cit.` | P473 | ✅ |
| **LoF/LoT com page numbers** | — | scope-out longo prazo (requer 2-pass convergente) |

---

## Débitos técnicos residuais (não activos, não bloqueantes)

| Item | Origem | Impacto | Resolução |
|------|--------|---------|-----------|
| `label_to_counter_key` duplicado em Introspector e TagIntrospector | P462 | Manutenção dupla | Refacto futuro orgânico |
| `LinkItem` com `items: Vec<FrameItem>` vs `body: Frame` | P452 | Divergência documentada | Manter `items` |
| Gate ADR-0117 no `crystalline-lint` | P453 | Manual | Implementar ou manter disciplina manual |
| Stack overflow em `p350c_flag_on_nao_convergente_classifica` | Pré-existente | Passa com `RUST_MIN_STACK=8388608` | Investigar separadamente se necessário |
| M6 — eliminação `CounterStateLegacy` | P190A | Refacto arquitectural | Pós-shaping quando pipeline estiver estável |

---

## Divergências de paridade declaradas (permanentes ou longo prazo)

| Divergência | ADR | Resolução |
|-------------|-----|-----------|
| Shaping (kern, ligatures, RTL, OpenType) | ADR-0054 graded | Trilha 5 épico |
| LoF/LoT com page numbers | ADR-0054 graded | Scope-out longo prazo |
| ColorSpace runtime user-facing | ADR-0083 | Scope-out permanente |
| Título outline não localizado | ADR-0054 graded | Aceite (string fixa "Índice") |
| Footnote numeração por página (sequência global vs por página) | ADR-0054 graded | Aceite |
| Pixel-perfect PDF | ADR-0054 graded | Inviável por design (FixedMetrics vs FontBookMetrics) |
| Constantes cor além de CSS basic | ADR-0083 | Aceite |
| `math.equation.where(...)` selector composto | ADR-0054 graded | Scope-out |

---

## Inventário de ADRs relevantes — estado

| ADR | Título | Estado |
|-----|--------|--------|
| ADR-0033 | Paridade funcional vanilla como invariante | EM VIGOR |
| ADR-0039 | `FrameItem::Text` — TextStyle como struct resolvido | EM VIGOR |
| ADR-0054 | Perfil graded de cobertura | EM VIGOR |
| ADR-0075 | Vanilla integration via CLI + comparação estrutural | ACEITE FINAL (P480) |
| ADR-0083 | Color paridade | IMPLEMENTADO |
| ADR-0107 | Paridade linguagem não mecânica | EM VIGOR |
| ADR-0117 | Spec propõe ficheiro existente | EM VIGOR |
| **ADR-0120** | **FrameItem::TextShaped + pipeline rustybuzz** | **PROPOSTA (P481)** |

---

## Regras do roteiro (preservadas)

1. **Sonda A.0 real antes da spec (ADR-0114).** Executada com `file:line` + commit. Violações históricas: P388, P409, P413, P416, P421, P447, P451, P452, P459.
2. **Verificar fronteiras/ADR vigentes antes de propor estrutura (ADR-0117 Cláusula 4).**
3. **Declarar divergência de paridade de saída (ADR-0107).**
4. **Confirmar o gate da ADR-0117 no linter** (implementar ou manter disciplina manual).

---

## Ordem sugerida para P481+

### Épico Trilha 5 (prioridade recomendada)

```
P481 — Sonda arquitectural + ADR-0120 (S, ~25 min)
P482 — ShapedGlyph + TextShaped + rustybuzz LTR (L, ~90 min)
P483 — export.rs migração TextShaped (M, ~45 min)
P484 — RTL básico unicode-bidi (L, ~90 min)
P485 — OpenType kern + ligatures explícitos (M, ~45 min)
```

### Alternativas de menor magnitude (entre sub-passos do épico)

| Passo | Descrição | Magnitude |
|-------|-----------|-----------|
| Footnote numeração por página | P295 scope-out revogável | S |
| Título outline localizado | Usar `figure_supplement_for_lang` pattern | S |
| `math.equation.where(...)` selector composto | Rever `parse_selector` | S |
| LoF/LoT com page numbers | Requer 2-pass convergente | M–L |
| Constantes cor extra (CSS extended) | ADR-0083 §Constantes | XS |
| Expansão corpus paridade | +10 ficheiros semantic/ | S |

---

## O que confirmar em P481 (sonda profunda Trilha 5)

1. **Sites de escrita de `FrameItem::Text`** — onde é construído; total.
2. **Sites de leitura de `FrameItem::Text`** — export.rs, parity, testes; total.
3. **Versão rustybuzz** — API disponível; `Face::from_slice`, `shape()`, `GlyphBuffer`.
4. **`FontBook` expõe bytes da fonte** — necessário para `rustybuzz::Face`.
5. **`unicode-bidi` ausente/presente** — determina se P484 precisa de nova dependência.
6. **Magnitude corrigida** — confirmar ou rever estimativa XL (8–12h).

---

## Inventário de DEBTs — estado final pós-P480

**DEBT-9** ("Cobertura de paridade — rastreamento contínuo") permanece activo por natureza — instrumento de monitorização, não item a fechar com critério binário.

**Todos os restantes DEBTs: FECHADOS.**

**Total DEBTs activos com critério de fecho: 0.**
