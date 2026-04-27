# Relatório Passo P158C — Refactor `Figure.kind: String → Option<String>`

Materialização do quarto sub-passo Model figure-kinds (Bloco A
do diagnóstico P159B §3.4). **Décima oitava aplicação consecutiva
de materialização** desde P156C; **refactor cosmético** com
primeiro Caso A "estrito" em refactor (não em variant aditivo).

---

## Resumo do executado

1. **Diagnóstico** em `00_nucleo/diagnosticos/diagnostico-figure-kind-refactor-passo-158c.md`
   (9 secções: 7 canónicos ADR-0034 + 2 específicos para
   cascading callers + backwards compat).

2. **Variant `Content::Figure` refactored** em
   `01_core/src/entities/content.rs`:
   - `kind: String → Option<String>`.
   - Doc-comment do field actualizado para mencionar ADR-0064
     Caso A estrito + default resolvido em uso.

3. **Stdlib `native_figure` adaptado** em
   `01_core/src/rules/stdlib/figure_image.rs`:
   - Retorna `Option<String>` directamente.
   - Fallback chain: `kind:` explícito > `infer_kind_from_body`
     > **None** (sem `unwrap_or("image".to_string())` final).
   - Aceita `kind: auto`/`kind: none` explícito → produz `None`
     directamente.

4. **Introspect adaptado** em `01_core/src/rules/introspect.rs`:
   - Walk arm: `kind.as_deref().unwrap_or("image").to_string()`
     em `local_figure_counters` e `figure_numbers`.
   - Labelled Figure arm: `kind_key = kind.as_deref().unwrap_or("image")`
     antes de `figure_supplement_for_lang(kind_key, lang)`.

5. **Layout adaptado** em `01_core/src/rules/layout/mod.rs`:
   - Figure arm: `kind_key = kind.as_deref().unwrap_or("image")`
     em `figure_progress` e `figure_numbers` lookup.

6. **Tests +2** (1212 → 1214; range esperado +2-4):
   - `figure_kind_auto_explicito_devolve_none` (stdlib/mod.rs):
     `kind: auto` produz None directo.
   - `introspect_figure_kind_none_resolve_para_image_no_counter`
     (introspect.rs): kind=None resolve a "image" no counter via
     fallback; label "Figura 1".

7. **~17 tests existentes adaptados**:
   - 14 sítios `kind: "image".to_string()` → `kind: Some("image".to_string())`
     em construtores de Figure (introspect.rs, layout/tests.rs).
   - 5 asserts `kind == "image"` → `kind.as_deref() == Some("image")`
     em stdlib/mod.rs + introspect.rs.
   - 1 sítio `kind: "custom".to_string()` → `kind: Some("custom".to_string())`.
   - 1 test renomeado: `figure_default_image_quando_body_nao_detectavel`
     → `figure_kind_none_quando_body_nao_detectavel` (semântica
     P158C: kind=None em vez de kind="image" default).

8. **Documentação atualizada**:
   - Tabela cobertura: entrada `figure` ganha nota P158C +
     footnote ³⁴ novo.
   - ADR-0061 §"Aplicações cumulativas" + padrão #17 NOVO
     ("refactor de field para Option" N=1).
   - ADR-0060 anotação Passo 158C.
   - README ADRs entrada P158C antes de P159D.

---

## Confirmação das verificações (1-12)

1. **`cargo test`**: 1214+215+24+21 = **1474 tests** workspace
   (era 1472; +2 dentro do range esperado +2-4). Zero falhas;
   6 ignored em integ tests pré-existentes (inalterado).

2. **`crystalline-lint`**: ✓ No violations found.

3. **Contagem variants Content**: **58** (inalterada — refactor
   de field, sem variant novo).

4. **Contagem stdlib funcs**: **48** (inalterada — `native_figure`
   modificada via parsing).

5. **Hash `entities/content.rs` preservado** `ec58d849` —
   **décimo quarto passo consecutivo** ✓ via L0-baseline
   interpretation. Lição P159A/C/D internalizada — preservação
   é regra default para refactors internos cosméticos.

6. **ADR-0064 Caso A patamar N=6 → 7** ✓ confirmado.
   **Primeiro Caso A "estrito" em refactor** (não em variant
   aditivo). Distribuição cross-domínio passa de 50/50 para
   **43/57 favorecendo Model** (3 Layout + 4 Model).

7. **Tests pré-existentes Figure passam inalterados/adaptados**:
   - Estrutura de asserts adaptada para `kind.as_deref() ==
     Some(...)` em vez de `kind == "..."`.
   - Constructor literals adaptados para `kind: Some("...".to_string())`.
   - Comportamento observable preservado integralmente
     (label "Figura 1", counters independentes, etc.).

8. **Cascading sítios callers identificados em .1 e cobertos
   integralmente em .2-.6**: ~10 sítios produtivos + ~17 sítios
   em tests. Audit completo via grep antes de iniciar edição.

9. **Sem novas reservas criadas** — política P158/P159
   preservada.

10. **ADR-0017 não afectada** — refactor não toca counters
    cross-document.

11. **`layout_grid` original NÃO modificado** (paridade
    P157A/B/C + P159A/C/D).

12. **Comportamento observable idêntico vs P158B**: None ↔ Auto
    produz default "image" per caller fallback. Verificado por
    test novo `introspect_figure_kind_none_resolve_para_image_no_counter`.

---

## §Análise de risco (N=20)

**Vigésima aplicação consecutiva** do padrão "§análise de risco
no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C + P158/A/B +
P159/A/B/C/D + ADR-0062-create + **P158C**).

**Risco realizado**: **baixo** (alinhado com previsão da spec
§"Natureza do passo").

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | nulo | Refactor de field; sem variant novo. |
| Backwards compat | nulo | Output observable preservado via fallback nos callers. |
| Hash content.rs | nulo | L0-baseline preservou `ec58d849` (14º consecutivo). |
| Cascading callers | mínimo | Audit completo via grep antes de edição; ~10 produtivos + ~17 tests cobertos. |
| Decisão default em uso vs construção | mínimo | Pattern Caso A canónico (N=6 já validado); decisão registada em diagnóstico §2. |
| Tests dentro do range | nulo | +2 dentro do range +2-4 esperado pela spec. |
| ADR-0064 Caso A patamar | nulo | N=6→7 já validado N=6; sem ambiguidade arquitectural; primeiro estrito em refactor. |
| Distribuição cross-domínio | mínimo | Passa de 50/50 para 43/57 favorecendo Model — tendência natural não problemática. |

**Cenários da spec §"O que pode sair errado"**:
- Sítio caller não previsto — **não realizado**: grep prévio
  identificou 100% dos sítios.
- Vanilla `Figure.kind` com tipo mais complexo (`Smart<Content>`
  em vez de `Smart<Str>`) — **não realizado**: vanilla é
  `Smart<FigureKind>` enum simplificado para String em P75;
  refactor para `Option<String>` mantém esta simplificação.
- Tests verificarem `kind == "image"` directo — **realizado e
  mitigado**: 5 asserts adaptados; padrão `as_deref() ==
  Some(...)` consistente.
- L0-baseline NÃO preservar hash `content.rs` — **não realizado**:
  doc-comment do field actualizado preserva hash do prompt L0
  (que não menciona kind explicitamente; alteração interna
  cabe na regra L0-baseline).

**Padrão consolidado**: **20 aplicações consecutivas** sem
materialização que exceda risco previsto. **Zero reformulações
mid-passo** em N=18 aplicações de materialização (9 Layout +
9 Model).

---

## Slope cumulativo Model (mesa P155-P158C)

| Passo | Feature | Slope Model agregado | Cobertura impl+impl⁺ Model | Tests Δ |
|-------|---------|---------------------:|---------------------------:|--------:|
| P155 | quote (smart-quotes lang-aware) | +4pp Model | 41% → ~45% | +29 |
| P156A-B | (Layout meta diagnósticos) | — | — | 0 |
| P156C-J | Layout Fase 1+2+3 sub-passos | 0% Model | ~45% inalterada | +149 |
| P156K | (meta ADRs 0064+0065) | — | — | 0 |
| P156L | pad refino sides | 0% Model | ~45% inalterada | +4 |
| P157 | (Model Fase 2 diagnóstico) | — | — | 0 |
| P157A | table minimal | +5pp Model | 45% → 50% | +16 |
| P157B | table cell sub-entrada | 0% agregado | 50% inalterada | +18 |
| P157C | table header+footer (par sim) | 0% agregado | 50% inalterada | +26 |
| P158 | (Model figure-kinds diagnóstico) | — | — | 0 |
| P158A | figure auto-detect kind | 0% agregado | 50% inalterada (refino) | +6 |
| P159 | (Bibliography+Cite diagnóstico) | — | — | 0 |
| P159A | par acoplado bibliography+cite | 0% agregado | 50% inalterada (parcial 22→24) | +27 |
| P159B | (P159 expansão diagnóstico) | — | — | 0 |
| ADR-0062-create | (administrativo XS) | — | — | 0 |
| P158B | figure supplement por lang | 0% agregado | 50% inalterada (2º refino figure) | +15 |
| P159C | cite.form variants | 0% agregado | 50% inalterada (refino estrutural cite) | +15 |
| P159D | BibEntry fields adicionais | 0% agregado | 50% inalterada (refino tipo entity) | +8 |
| **P158C** | **Figure.kind refactor String→Option** | **0% agregado** | **50% inalterada (refactor cosmético)** | **+2** |

**Total Model pós-P158C**: +9pp Model agregado em 5 sub-passos
materiais (P155 + P157A + 8 refinos qualitativos/estruturais/
refactors cosméticos: P157B/C, P158A, P159A, P158B, P159C,
P159D, P158C); cobertura ampla impl+impl⁺+parcial cresce 22 →
24 parciais (P159A; outros mantém).

**Padrão emergente reforçado**: 8 dos 10 sub-passos materiais
Model são refinos qualitativos/estruturais/cosméticos (não +pp
agregados) — **80% qualitativos**. P158C é **primeiro refactor
puramente cosmético** — distinto de refinos comportamentais
(P158A/B figure), refinos estruturais de variant Content (P156L
Pad, P159C Cite), e refino de tipo entity (P159D BibEntry).

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P158C adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P158C | Figure.kind refactor String→Option (Model figure-kinds sub-passo 3) | 0% agregado | Layout 78%; Model 50% inalterado (refactor cosmético); ADR-0064 Caso A N=6→7 | +2 |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=17 → 18**.
- Inventariar primeiro: **N=19 → 20** (ADR-0065 critério #5
  oitava aplicação concreta com diversidade reforçada em
  refactor cosmético).
- §análise de risco no relatório: **N=19 → 20**.
- ADR-0064 Caso A: **N=6 → 7** com **primeiro estrito em
  refactor** (distribuição passa de 50/50 para 43/57 favorecendo
  Model).
- Tipo entity em ficheiro próprio: **N=5 inalterado**.
- Infraestrutura state lookup: **N=2 inalterado**.
- P155 cross-feature: **N=1 inalterado**.
- Refino tipo entity sem alteração Content: **N=1 inalterado**.
- **Padrão #17 NOVO**: "Refactor de field para Option" N=1
  (precedente novo; primeiro Caso A em refactor não em variant
  aditivo).

---

## Confirmações finais

- **ADR-0064 Caso A patamar N=6 → 7**: ✓ confirmado.
  Distribuição cross-domínio desloca-se de 50/50 para 43/57
  favorecendo Model — tendência natural já que refinos Model
  têm dominado os últimos passos.
- **Estabilidade hash content.rs N=13 → 14**: ✓ confirmado via
  L0-baseline interpretation. Refactor cosmético interno cabe
  na regra de preservação. Lição P159A/C/D internalizada e
  agora aplicada conscientemente.
- **Política "sem novas reservas"** (P158/P159): ✓ preservada.
- **Subpadrão novo #17**: "refactor de field para Option" (N=1)
  — precedente novo distinto de variant aditivo com `Option<T>`
  field; aplicação em refactor de tipo existente; candidato a
  formalização se outros refactors análogos forem feitos.

---

## Estado pós-P158C

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada (refactor cosmético).
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada).
- **Cobertura arquitectural**: **82%** inalterada (refactor de
  field ortogonal ao enum Content; sem variants novos).
- **Hash `entities/content.rs`**: `ec58d849` (14º passo
  consecutivo preservado via L0-baseline).
- **63 ADRs** (28 EM VIGOR; ADR-0060 IMPLEMENTADO; 12 PROPOSTO
  incluindo 0062 sem promoção).
- **Tests**: 1474 workspace (1214 lib + 215 integ + 24 + 21).
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada).
- **Padrões consolidados**:
  - Granularidade N=18.
  - Inventariar primeiro N=20.
  - **Smart→Option Caso A patamar N=7** (primeiro estrito em
    refactor; distribuição 43/57 Layout/Model).
  - §análise risco N=20.
  - Estabilidade hash L0 content.rs N=14.
  - Tipo entity em ficheiro próprio N=5 (inalterado).
  - Infraestrutura state lookup N=2 (inalterado).
  - P155 cross-feature N=1 (inalterado).
  - Refino tipo entity sem alteração Content N=1 (inalterado).
  - **Refactor de field para Option N=1 (subpadrão novo #17)**.

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0062 mantém-se reserva PROPOSTO sem promoção**.

**Próxima decisão (sem candidata pré-acordada — política "sem
novas reservas")**: Bloco A do diagnóstico P159B fica **reduzido
a 1 candidato restante pós-P158C**:
- **P159F** — Numbering numérico simples Bibliography (M; counter
  local).

Outras direcções: ADR-0062-create (XS administrativo); mudança
de módulo (Introspection P160); Bloco B/C (com ADR-0062
PROPOSTO). Decisão humana com máxima informação.

**Pausa natural após P158C — Figure.kind refactored para
Option<String>; ADR-0064 Caso A patamar N=7 com primeiro
estrito em refactor; estabilidade hash content.rs N=14;
subpadrão novo #17 emerge. Bloco A reduzido a 1 candidato
único restante (P159F).**
