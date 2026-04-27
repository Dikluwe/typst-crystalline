# Relatório Passo P159E — `url` + `doi` em `BibEntry`

Materialização do **primeiro sub-passo família 159 fora do
Bloco A** do diagnóstico P159B (Bloco A esgotado pós-P159F).
**Vigésima aplicação consecutiva de materialização** desde
P156C; **refino estrutural de tipo entity** com pattern P159D
replicado fielmente.

---

## Resumo do executado

1. **Diagnóstico** em `00_nucleo/diagnosticos/diagnostico-bibentry-url-doi-passo-159e.md`
   (9 secções: 7 canónicos ADR-0034 + 2 específicos para ordem
   layout + formato).

2. **Struct `BibEntry` extendido** em
   `01_core/src/entities/bib_entry.rs`:
   - 2 fields novos `Option<String>`: `url`, `doi`.
   - Backwards compat preservada — `BibEntry::new(4 args)`
     original continua a funcionar; fields novos default `None`.
   - **Builder pattern fluente** extendido: `with_url()`,
     `with_doi()` paridade P159D.
   - Total 10 fields (4 obrigatórios + 6 opcionais).

3. **Helper `extract_bib_entries` (P159A+P159D) extendido** em
   `01_core/src/rules/stdlib/structural.rs`:
   - Helper inline `optional_str(field)` reusado para `url` e
     `doi` — **cumulativo N=4** (P159D N=2 + P159E N=2).
   - Atinge limiar promoção a `pub(super)` ou helper público
     N=3-4 — promoção diferida per política consistente.

4. **Layout `format_bib_entry` extendido** em
   `01_core/src/rules/layout/mod.rs`:
   - Concatenação condicional após `(year).` (Opção C
     diagnóstico §8.2).
   - URL plaintext literal: `format!(" {}.", u)`.
   - DOI prefixo `doi:`: `format!(" doi:{}.", d)`.
   - Match nas 4 combinações de presença Some/None para evitar
     espaço vazio.
   - Backwards compat: quando ambos `None`, output P159D
     preservado exactamente.

5. **Tests +8** (1222 → 1230; range esperado +5-8):
   - 3 unit em `bib_entry.rs`: backwards compat url/doi None +
     builder url/doi + PartialEq cobre 10 fields.
   - 3 stdlib em `stdlib/mod.rs`: parse com url/doi presentes +
     regression sem url/doi (P159D) + tipo errado em doi
     rejeitado.
   - 2 layout E2E em `layout/tests.rs`: entry com url/doi
     formato extendido + entry sem url/doi regression P159D.

6. **Documentação atualizada**:
   - Tabela cobertura: entrada `bibliography` ganha nota P159E +
     footnote ³⁶ novo.
   - ADR-0061 §"Aplicações cumulativas" + padrão #16 atualizado
     N=1→2 (atinge meio-caminho limiar formalização).
   - ADR-0060 anotação Passo 159E.
   - README ADRs entrada P159E antes de P159F.

---

## Confirmação das verificações (1-12)

1. **`cargo test`**: 1230+215+24+21 = **1490 tests** workspace
   (era 1482; +8 dentro do range esperado +5-8). Zero falhas;
   6 ignored em integ tests pré-existentes (inalterado).

2. **`crystalline-lint`**: ✓ No violations found.

3. **Contagem variants Content**: **58** (inalterada — refino
   tipo entity).

4. **Contagem stdlib funcs**: **48** (inalterada —
   `native_bibliography` modificada via helper extendido).

5. **Hash `entities/content.rs` preservado** `ec58d849` —
   **décimo sexto passo consecutivo** ✓ via L0-baseline
   interpretation.

6. **Hash `entities/bib_entry.rs` preservado** `5a2c0ebd` ✓
   per L0-baseline (paridade P159D resultado — extensão via
   doc-comment do header não modifica prompt L0 `bib_entry.md`).

7. **Decisão sobre ordem layout registada**: **Opção C**
   adoptada (diagnóstico §8.2). url/doi após `(year).` per
   paridade APA + backwards compat. Alternativas (Opção A:
   depois publisher; Opção B: antes (year)) avaliadas em matriz
   multi-critério §8.1 e rejeitadas.

8. **Decisão sobre formato registada**: URL plaintext literal
   `https://...`; DOI prefixo `doi:10.1234/abc` (diagnóstico
   §9). Justificação: paridade APA estilo prose + compactness +
   reconhecibilidade humana.

9. **Sem novas reservas criadas** — política P158/P159
   preservada.

10. **Tests pré-existentes Bibliography (P159A+P159D) passam
    inalterados** ✓ — fields novos default None produz output
    P159D original. Verificado por
    `bibliography_entry_sem_url_doi_regression_p159d`.

11. **`layout_grid` original NÃO modificado** (paridade
    P157A/B/C + P159A/C/D/F).

12. **Restantes fields BibEntry vanilla** (`editor`/`series`/
    `note`/`isbn`/`location`/`organization`) NÃO materializados ✓
    — NÃO reservados; candidatos futuros.

---

## §Análise de risco (N=22)

**Vigésima segunda aplicação consecutiva** do padrão "§análise
de risco no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C +
P158/A/B/C + P159/A/B/C/D/F + ADR-0062-create + **P159E**).

**Risco realizado**: **baixo** (alinhado com previsão da spec
§"Natureza do passo").

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | nulo | Refino tipo entity ortogonal ao enum Content; nenhum variant tocado. |
| Backwards compat | nulo | `new(4 args)` preservado; fields novos default None produz output P159D. |
| Hash content.rs | nulo | L0-baseline preservou `ec58d849` (16º consecutivo). |
| Hash bib_entry.rs | nulo | L0-baseline preservou `5a2c0ebd` (paridade P159D resultado). |
| Pattern P159D replicado | nulo | Pattern validado N=1 em P159D; replicação trivial. |
| Decisão de ordem layout (Opção A/B/C) | mínimo | Matriz multi-critério resolvida em diagnóstico §8.1; Opção C escolhida com justificação clara. |
| Decisão de formato (URL/DOI) | mínimo | Paridade APA estilo prose; alternativas documentadas. |
| Tests dentro do range | nulo | +8 dentro do range +5-8 esperado pela spec. |

**Cenários da spec §"O que pode sair errado"**:
- Vanilla usar `url: Option<Url>` estruturado — **mitigado**:
  `Option<String>` per ADR-0054 graded; URL parsing diferido.
- DOI vanilla tipo dedicado com regex — **mitigado**:
  `Option<String>` literal; validation diferida.
- Layout output muito longo — **aceite**: linha única dentro
  do envelope normal; refactor multi-line diferido (Bloco C).
- Tests pré-existentes esperar formato exacto — **não realizado**:
  backwards compat trivial preserva output P159D exactamente
  quando url/doi `None`.
- L0-baseline NÃO preservar hash bib_entry.rs — **não realizado**:
  extensão via doc-comment do header preserva hash do prompt L0.

**Padrão consolidado**: **22 aplicações consecutivas** sem
materialização que exceda risco previsto. **Zero reformulações
mid-passo** em N=20 aplicações de materialização (9 Layout +
11 Model).

---

## Slope cumulativo Model (mesa P155-P159E)

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
| **P159E** | **url + doi em BibEntry (1º fora Bloco A)** | **0% agregado** | **50% inalterada (refino tipo entity)** | **+8** |

**Total Model pós-P159E**: +9pp Model agregado em 5 sub-passos
materiais (P155 + P157A + 10 refinos qualitativos/estruturais/
cosméticos/numbering/par identificadores: P157B/C, P158A,
P159A, P158B, P159C, P159D, P158C, P159F, P159E); cobertura
ampla impl+impl⁺+parcial cresce 22 → 24 parciais (P159A;
outros mantém).

**Padrão emergente reforçado**: 10 dos 12 sub-passos materiais
Model são refinos qualitativos/estruturais/cosméticos/numbering
(não +pp agregados) — **83% qualitativos**. P159E é **segundo
refino estrutural de tipo entity** (P159D + P159E) — subpadrão
#16 atinge meio-caminho limiar formalização N=3-4.

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P159E adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P159E | url + doi em BibEntry (refino família 159 fora Bloco A) | 0% agregado | Layout 78%; Model 50% inalterado (refino tipo entity); subpadrão #16 N=1→2 | +8 |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=19 → 20**.
- Inventariar primeiro: **N=21 → 22** (ADR-0065 critério #5
  décima aplicação concreta com pattern P159D replicado).
- §análise de risco no relatório: **N=21 → 22**.
- ADR-0064: NÃO directamente aplicável em P159E (Option<String>
  directo).
- Tipo entity em ficheiro próprio: **N=5 inalterado** (BibEntry
  expande mas continua em `bib_entry.rs`).
- Infraestrutura state lookup: **N=3 inalterado**.
- **Subpadrão #16 "refino tipo entity sem alteração Content":
  N=1 → 2** (atinge meio-caminho limiar formalização N=3-4).
- P155 cross-feature: **N=1 inalterado**.
- Refactor de field para Option: **N=1 inalterado**.
- **Helper `optional_str` cumulativo: N=2 → 4** (atinge limiar
  promoção a `pub(super)`/público N=3-4).

---

## Confirmações finais

- **Pattern P159D replicado fielmente**: ✓ confirmado.
  Estrutura paralela (2 fields opcionais + builder pattern +
  parsing inline + format condicional). Subpadrão #16 cresce
  N=1→2; padrão consolida-se.
- **Helper `optional_str` cumulativo N=2→4**: ✓ confirmado.
  Atinge limiar promoção a `pub(super)` ou helper público
  N=3-4 — promoção diferida em passo administrativo XS futuro
  NÃO reservado.
- **Estabilidade hash content.rs N=15 → 16**: ✓ confirmado via
  L0-baseline interpretation. Refino tipo entity ortogonal ao
  variant Content; doc-comment do header bib_entry.rs preserva
  prompt L0 `bib_entry.md`.
- **Política "sem novas reservas"** (P158/P159): ✓ preservada.

**Marca conceptual** ✓ confirmada: P159E é **primeiro sub-passo
família 159 fora Bloco A** do diagnóstico P159B (esgotado
pós-P159F). Preenche slot P159E reservado fracamente após P158C
ter ocupado identificador alternativo (decisão prévia diagnóstico
P159B §3.4 "P158C ou P159E"). Identificador preserva sequência
alfabética sem violar política "sem novas reservas" (que se
refere a scope, não identificadores).

---

## Estado pós-P159E

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada (refino tipo entity).
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada).
- **Cobertura arquitectural**: **82%** inalterada (refino tipo
  entity ortogonal ao enum Content; sem variants novos).
- **Hash `entities/content.rs`**: `ec58d849` (**16º passo
  consecutivo** preservado via L0-baseline).
- **Hash `entities/bib_entry.rs`**: `5a2c0ebd` (preservado
  paridade P159D resultado).
- **63 ADRs** (28 EM VIGOR; ADR-0060 IMPLEMENTADO; 12 PROPOSTO
  incluindo 0062 sem promoção).
- **Tests**: 1490 workspace (1230 lib + 215 integ + 24 + 21).
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada).
- **Padrões consolidados**:
  - Granularidade N=20.
  - Inventariar primeiro N=22.
  - Smart→Option Caso A patamar N=7 (inalterado).
  - §análise risco N=22.
  - Estabilidade hash L0 content.rs N=16.
  - Tipo entity em ficheiro próprio N=5 (inalterado).
  - Infraestrutura state lookup N=3 (inalterado).
  - **Subpadrão #16 (refino tipo entity sem alteração Content):
    N=1 → 2** (atinge meio-caminho limiar formalização).
  - P155 cross-feature N=1 (inalterado).
  - Refactor de field para Option N=1 (inalterado).
  - **Helper `optional_str` cumulativo N=2 → 4** (atinge
    limiar promoção).

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0062 mantém-se reserva PROPOSTO sem promoção**.

**Próxima decisão (sem candidata pré-acordada — política "sem
novas reservas")**:

- **Restantes fields BibEntry vanilla** (`editor`/`series`/
  `note`/`isbn`/`location`/`organization`): NÃO reservados.
  Candidatos a refinos M futuros se prioritários (subpadrão
  #16 cresceria N=2→3 se replicado).
- **ADR-0062-create** — XS administrativo; desbloqueia Bloco B.
- **Bloco B (hayagriva)**: P159G após ADR-0062 PROPOSTO.
- **Bloco C (cross-módulo)**: refactor multi-region L+
  (DEBT-34e + DEBT-56) ou Introspection P160.
- **Refinos Model fora Bloco A continuação** (mais langs em
  `figure_supplement_for_lang`; refactor `kind` em outros
  variants Content; etc.).
- **Mudança de módulo**: Layout Fase 3 (columns/colbreak) ou
  Introspection P160.
- **Passos administrativos XS**: actualizar L0 prompt
  `content.md` mencionando variants Bibliography/Cite/Cite.form;
  promover ADR-0060 a R1; ADR meta saturação ADR-0064; **ADR
  meta subpadrão #15 (state lookup; N=3 atinge limiar)**;
  **promoção `optional_str` a helper público (N=4 cumulativos
  atingem limiar)**.

**Pausa natural após P159E — BibEntry com 10 fields (4
obrigatórios + 6 opcionais); slot P159E preenchido com par
natural url+doi; pattern P159D replicado fielmente; subpadrão
#16 atinge N=2; helper `optional_str` cumulativo N=4 atinge
limiar promoção. Decisão humana sobre próxima direcção tem
máxima informação acumulada.**
