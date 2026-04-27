# Relatório Passo P159G — `BibEntry` 6 fields restantes

Materialização do **segundo sub-passo família 159 fora do
Bloco A** do diagnóstico P159B (Bloco A esgotado pós-P159F).
**Vigésima primeira aplicação consecutiva de materialização**
desde P156C; **refino estrutural de tipo entity** com pattern
P159D replicado pela terceira vez — subpadrão #16 atinge N=3
(limiar formalização).

---

## Resumo do executado

1. **Diagnóstico** em `00_nucleo/diagnosticos/diagnostico-bibentry-restantes-passo-159g.md`
   (9 secções: 7 canónicos ADR-0034 + 2 específicos para ordem
   layout + formatos individuais).

2. **Struct `BibEntry` extendido** em
   `01_core/src/entities/bib_entry.rs`:
   - 6 fields novos `Option<String>`: `editor`, `series`, `note`,
     `isbn`, `location`, `organization`.
   - Total **16 fields** (4 obrigatórios + 12 opcionais;
     cobertura ~70-75% hayagriva universais).
   - Backwards compat preservada — `BibEntry::new(4 args)`
     original continua a funcionar; fields novos default `None`.
   - **Builder pattern fluente** extendido (6 novos `with_*`
     métodos paridade P159D/E).

3. **Helper `extract_bib_entries` (P159A+P159D+P159E) extendido**
   em `01_core/src/rules/stdlib/structural.rs`:
   - Helper inline `optional_str(field)` reusado para os 6
     fields — **cumulativo N=12** (4 P159D + 2 P159E + 6 P159G).
   - Largamente acima limiar promoção a `pub(super)` ou helper
     público N=3-4 — promoção diferida em passo administrativo
     XS futuro NÃO reservado.

4. **Layout `format_bib_entry` extendido** em
   `01_core/src/rules/layout/mod.rs`:
   - Concatenação condicional APA-like extendida (decisões
     diagnóstico §8.2 ordem + §9 formatos individuais).
   - Editor `(Ed. {editor})` após title.
   - Series `({series})` após title.
   - Location: antes de publisher (`{location}: {publisher}`).
   - Organization substitutivo a publisher quando publisher
     ausente (decisão arbitrária per ADR-0054 graded).
   - ISBN antes de url/doi com prefixo lowercase
     `isbn:{isbn}` (paridade P159E).
   - Note ao final entre brackets `[{note}]`.
   - Backwards compat: quando todos os 6 fields P159G `None`,
     output P159E preservado exactamente.

5. **Tests +11** (1230 → 1241; range esperado +8-12):
   - 4 unit em `bib_entry.rs`: backwards compat fields P159G
     None + builder P159G + PartialEq cobre 16 fields + builder
     subset P159G.
   - 4 stdlib em `stdlib/mod.rs`: parse com 6 fields presentes +
     parse com subset (3 fields) + regression sem P159G fields
     (P159E) + tipo errado em isbn rejeitado.
   - 3 layout E2E em `layout/tests.rs`: entry com 6 fields
     formato extendido + entry sem fields P159G regression
     P159E + organization substitutivo a publisher.

6. **Documentação atualizada**:
   - Tabela cobertura: entrada `bibliography` ganha nota P159G +
     footnote ³⁷ novo.
   - ADR-0061 §"Aplicações cumulativas" + padrão #16 atualizado
     N=2→3 (atinge limiar formalização N=3-4).
   - ADR-0060 anotação Passo 159G + sequência alfabética
     identificadores família 159 não-monótona registada.
   - README ADRs entrada P159G antes de P159E.

---

## Confirmação das verificações (1-14)

1. **`cargo test`**: 1241+215+24+21 = **1501 tests** workspace
   (era 1490; +11 dentro do range esperado +8-12). Zero falhas;
   6 ignored em integ tests pré-existentes (inalterado).

2. **`crystalline-lint`**: ✓ No violations found.

3. **Contagem variants Content**: **58** (inalterada — refino
   tipo entity).

4. **Contagem stdlib funcs**: **48** (inalterada —
   `native_bibliography` modificada via helper extendido).

5. **Hash `entities/content.rs` preservado** `ec58d849` —
   **décimo sétimo passo consecutivo** ✓ via L0-baseline
   interpretation.

6. **Hash `entities/bib_entry.rs` preservado** `5a2c0ebd` ✓
   per L0-baseline (paridade P159D+P159E resultado — extensão
   via doc-comment do header não modifica prompt L0).

7. **Decisão sobre ordem layout dos 6 fields registada**:
   editor após title, series após title, location antes
   publisher, organization substitutivo a publisher, isbn antes
   url/doi, note ao final (decisão diagnóstico §8.2 com
   justificação per paridade APA).

8. **Decisões sobre formatos individuais registadas**: editor
   `(Ed. ...)`, series `(...)`, location `{location}:`,
   organization substitutivo, isbn lowercase prefix, note
   brackets (decisão diagnóstico §9 com paridade APA estilo
   prose).

9. **Sem novas reservas criadas** — política P158/P159
   preservada.

10. **Tests pré-existentes Bibliography (P159A+P159D+P159E)
    passam inalterados** ✓ — fields novos default None produz
    output P159E original. Verificado por
    `bibliography_entry_sem_p159g_fields_regression_p159e`.

11. **`layout_grid` original NÃO modificado** (paridade
    P157A/B/C + P159A/C/D/F/E).

12. **Helper `optional_str` cumulativo N=4 → 12 usos** ✓ —
    largamente acima limiar promoção (N=3-4); promoção diferida
    em passo administrativo XS NÃO reservado.

13. **Subpadrão #16 "refino tipo entity sem alteração Content"
    cresce N=2 → 3** ✓ — atinge limiar formalização N=3-4;
    promoção a ADR meta possível em passo administrativo XS
    NÃO reservado.

14. **Restantes fields BibEntry vanilla** (`booktitle`/`address`/
    `chapter`/`type`/`institution`/etc.) NÃO materializados ✓
    — NÃO reservados; candidatos futuros.

---

## §Análise de risco (N=23)

**Vigésima terceira aplicação consecutiva** do padrão "§análise
de risco no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C +
P158/A/B/C + P159/A/B/C/D/F/E + ADR-0062-create + **P159G**).

**Risco realizado**: **baixo** (alinhado com previsão da spec
§"Natureza do passo").

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | nulo | Refino tipo entity ortogonal ao enum Content; nenhum variant tocado. |
| Backwards compat | nulo | `new(4 args)` preservado; fields novos default None produz output P159E. |
| Hash content.rs | nulo | L0-baseline preservou `ec58d849` (17º consecutivo). |
| Hash bib_entry.rs | nulo | L0-baseline preservou `5a2c0ebd` (paridade P159D+P159E). |
| Pattern P159D replicado pela 3ª vez | nulo | Pattern validado N=2 (P159D + P159E); replicação trivial. |
| Decisões cosméticas (ordem + formatos) | mínimo | 6 decisões individuais resolvidas em diagnóstico §8.2 + §9 com paridade APA. |
| Helper N=12 cumulativos | nulo | Largamente acima limiar promoção; reuso seguro. |
| Tests dentro do range | nulo | +11 dentro do range +8-12 esperado pela spec. |
| Subpadrão #16 N=3 limiar formalização | mínimo | Patamar N=3 atinge limiar; promoção a ADR meta diferida não-bloqueante. |

**Cenários da spec §"O que pode sair errado"**:
- Vanilla usar estruturas complexas (e.g. `editor: Vec<Person>`)
  — **mitigado**: `Option<String>` per ADR-0054 graded; tipos
  estruturados diferidos.
- Ordem APA não-trivial — **mitigado**: pré-decisões §"Decisões
  diferidas" confirmadas em §8.2 sem matriz multi-critério
  necessária.
- Layout output muito longo (16 fields) — **aceite**: linha
  única; refactor multi-line diferido (Bloco C).
- Tests pré-existentes esperar formato exacto — **não realizado**:
  backwards compat trivial preserva output P159E exactamente
  quando 6 fields novos `None`.
- Organization vs publisher conflito — **mitigado**: decisão
  arbitrária em §9.4 (organization substitutivo apenas quando
  publisher ausente).
- Note position dependente do tipo — **mitigado**: posição fixa
  final per ADR-0054 graded.
- Match exhaustivo factorial — **mitigado**: concatenação
  condicional in-place via `if let Some(...)` em vez de match
  exhaustivo nas 64 combinações.
- L0-baseline NÃO preservar hash bib_entry.rs — **não realizado**:
  extensão via doc-comment do header preserva hash do prompt L0.

**Padrão consolidado**: **23 aplicações consecutivas** sem
materialização que exceda risco previsto. **Zero reformulações
mid-passo** em N=21 aplicações de materialização (9 Layout +
12 Model).

---

## Slope cumulativo Model (mesa P155-P159G)

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
| P158C | Figure.kind refactor String→Option | 0% agregado | 50% inalterada (refactor cosmético) | +2 |
| P159F | Bibliography numbering numérico (**último Bloco A**) | 0% agregado | 50% inalterada (numbering numérico) | +8 |
| P159E | url + doi em BibEntry (1º fora Bloco A) | 0% agregado | 50% inalterada (refino tipo entity) | +8 |
| **P159G** | **6 fields restantes em BibEntry (2º fora Bloco A)** | **0% agregado** | **50% inalterada (refino tipo entity)** | **+11** |

**Total Model pós-P159G**: +9pp Model agregado em 5 sub-passos
materiais (P155 + P157A + 11 refinos qualitativos/estruturais/
cosméticos/numbering/identificadores/restantes: P157B/C, P158A,
P159A, P158B, P159C, P159D, P158C, P159F, P159E, P159G);
cobertura ampla impl+impl⁺+parcial cresce 22 → 24 parciais
(P159A; outros mantém).

**Padrão emergente reforçado**: 11 dos 13 sub-passos materiais
Model são refinos qualitativos/estruturais/cosméticos/numbering
(não +pp agregados) — **85% qualitativos**. P159G é **terceiro
refino estrutural de tipo entity** consecutivo (P159D + P159E +
P159G) — subpadrão #16 atinge limiar formalização N=3-4.

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P159G adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P159G | 6 fields restantes em BibEntry (refino família 159 fora Bloco A) | 0% agregado | Layout 78%; Model 50% inalterado (refino tipo entity); **subpadrão #16 N=2→3** atinge limiar formalização | +11 |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=20 → 21**.
- Inventariar primeiro: **N=22 → 23** (ADR-0065 critério #5
  décima primeira aplicação concreta com pattern P159D
  replicado pela terceira vez).
- §análise de risco no relatório: **N=22 → 23**.
- ADR-0064: NÃO directamente aplicável em P159G
  (Option<String> directo).
- Tipo entity em ficheiro próprio: **N=5 inalterado** (BibEntry
  expande mas continua em `bib_entry.rs`).
- Infraestrutura state lookup: **N=3 inalterado**.
- **Subpadrão #16 "refino tipo entity sem alteração Content":
  N=2 → 3** (atinge limiar formalização N=3-4; promoção a ADR
  meta possível em passo administrativo XS futuro NÃO reservado).
- P155 cross-feature: **N=1 inalterado**.
- Refactor de field para Option: **N=1 inalterado**.
- **Helper `optional_str` cumulativo: N=4 → 12** (largamente
  acima limiar promoção a `pub(super)`/público N=3-4).

---

## Confirmações finais

- **Pattern P159D replicado pela terceira vez**: ✓ confirmado.
  Estrutura paralela (6 fields opcionais + builder pattern +
  parsing inline + format condicional). Subpadrão #16 atinge
  N=3 (limiar formalização).
- **Helper `optional_str` cumulativo N=12**: ✓ confirmado.
  Largamente acima limiar promoção a `pub(super)` ou helper
  público N=3-4 — promoção diferida em passo administrativo
  XS futuro NÃO reservado.
- **Estabilidade hash content.rs N=16 → 17**: ✓ confirmado via
  L0-baseline interpretation. Refino tipo entity ortogonal ao
  variant Content; doc-comment do header bib_entry.rs preserva
  prompt L0 `bib_entry.md`.
- **Política "sem novas reservas"** (P158/P159): ✓ preservada.

**Marca conceptual** ✓ confirmada: P159G é **segundo sub-passo
família 159 fora Bloco A** (Bloco A esgotado pós-P159F).
**BibEntry pós-P159G com 16 fields total** — cobertura ~70-75%
hayagriva universais. Sequência alfabética identificadores
família 159 **não-monótona** registada: A → B → C → D → F → E →
G (preserva slot E para refinos família 159 que surgiram após
P158C ocupar identificador alternativo).

---

## Estado pós-P159G

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada (refino tipo entity).
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada).
- **Cobertura arquitectural**: **82%** inalterada (refino tipo
  entity ortogonal ao enum Content; sem variants novos).
- **Hash `entities/content.rs`**: `ec58d849` (**17º passo
  consecutivo** preservado via L0-baseline).
- **Hash `entities/bib_entry.rs`**: `5a2c0ebd` (preservado
  paridade P159D+P159E).
- **63 ADRs** (28 EM VIGOR; ADR-0060 IMPLEMENTADO; 12 PROPOSTO
  incluindo 0062 sem promoção).
- **Tests**: 1501 workspace (1241 lib + 215 integ + 24 + 21).
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada).
- **BibEntry**: **16 fields** (4 obrigatórios + 12 opcionais;
  cobertura ~70-75% hayagriva universais).
- **Padrões consolidados**:
  - Granularidade N=21.
  - Inventariar primeiro N=23.
  - Smart→Option Caso A patamar N=7 (inalterado).
  - §análise risco N=23.
  - Estabilidade hash L0 content.rs N=17.
  - Tipo entity em ficheiro próprio N=5 (inalterado).
  - Infraestrutura state lookup N=3 (inalterado).
  - **Subpadrão #16 (refino tipo entity sem alteração Content):
    N=2 → 3** (atinge limiar formalização).
  - P155 cross-feature N=1 (inalterado).
  - Refactor de field para Option N=1 (inalterado).
  - **Helper `optional_str` cumulativo N=4 → 12** (largamente
    acima limiar promoção).

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0062 mantém-se reserva PROPOSTO sem promoção**.

**Próxima decisão (sem candidata pré-acordada — política "sem
novas reservas")**:

- **Restantes fields BibEntry vanilla** (`booktitle`/`address`/
  `chapter`/`type`/`institution`/etc.): NÃO reservados.
  Candidatos a refinos M futuros se prioritários (subpadrão
  #16 cresceria N=3→4 se replicado).
- **ADR-0062-create** — XS administrativo; desbloqueia Bloco B.
- **Bloco B (hayagriva)**: P159H após ADR-0062 PROPOSTO.
- **Bloco C (cross-módulo)**: refactor multi-region L+
  (DEBT-34e + DEBT-56) ou Introspection P160.
- **Refinos Model fora Bloco A continuação** (mais langs em
  `figure_supplement_for_lang`; etc.).
- **Mudança de módulo**: Layout Fase 3 (columns/colbreak) ou
  Introspection P160.
- **Passos administrativos XS atingidos múltiplos limiares**:
  - **Promoção `optional_str` a helper público** (N=12
    cumulativos largamente atingem limiar).
  - **ADR meta subpadrão #16** (refino tipo entity sem Content;
    N=3 atinge limiar formalização).
  - **ADR meta subpadrão #15** (state lookup; N=3 atinge limiar).
  - L0 content.md update.
  - Promover ADR-0060 a R1.
  - ADR meta saturação ADR-0064.

**Pausa natural após P159G — BibEntry com 16 fields (cobertura
~70-75% hayagriva); pattern P159D replicado pela terceira vez;
subpadrão #16 atinge N=3 (limiar formalização); helper
`optional_str` cumulativo N=12 (largamente promovível); sequência
alfabética P159 não-monótona estabelecida e registada. Decisão
humana sobre próxima direcção tem máxima informação.**
