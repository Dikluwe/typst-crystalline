# Relatório Passo P158B — Supplement automático por lang em figure

Materialização do segundo sub-passo Model figure-kinds (recomendação
primária do diagnóstico P159B §6). **Décima quinta aplicação
consecutiva de materialização** desde P156C; **segundo refino
qualitativo consecutivo de `figure`** (P158A→P158B).

---

## Resumo do executado

1. **Diagnóstico** em `00_nucleo/diagnosticos/diagnostico-figure-supplement-passo-158b.md`
   (9 secções): ADRs aplicáveis; comportamento vanilla; helpers
   reusáveis; **decisão arquitectural-chave fallback PT** (não EN)
   para preservar backwards compat; lang resolution via novo field
   `state.lang`; tests planeados; subpadrão emergente "padrão P155
   i18n reusado cross-feature".

2. **Helper novo** em `01_core/src/rules/lang/figure_supplement.rs`
   (ficheiro novo paralelo a `quotes.rs`):
   - `pub fn figure_supplement_for_lang(kind: &str, lang: Option<&Lang>)
     -> String` — lookup linear por exact match em tabela estática
     `LANG_SUPPLEMENTS: &[((&str, &str), &str)]` com 18 entradas (3
     kinds × 6 langs).
   - Fallback `DEFAULT_SUPPLEMENTS_PT` por kind quando lang
     desconhecido.
   - Helper privado `capitalize_first(s)` para kinds desconhecidos
     (devolve "Custom" para kind="custom").
   - `pub mod figure_supplement;` adicionado a `rules/lang/mod.rs`.

3. **Field novo** em `01_core/src/entities/counter_state.rs`:
   - `pub lang: Option<Lang>` em `CounterState`.
   - Default `None` → fallback PT (paridade backwards compat).
   - Caller pode setar `state.lang = Some(lang)` antes de passar
     a `layout()` para comportamento lang-aware.

4. **Modificação trivial** em `01_core/src/rules/introspect.rs`
   linha 334:
   - Antes: `Some(format!("Figura {}", n))` hardcoded PT.
   - Depois: `Some(format!("{} {}", figure_supplement_for_lang(
     kind.as_str(), state.lang.as_ref()), n))`.
   - **Sem alteração ao variant `Content::Figure`** (estrutura
     inalterada).

5. **Tests +15** (1174 → 1189):
   - 8 unit tests em `figure_supplement.rs`: lookup pt/de/it/en/fr;
     fallback lang desconhecido; fallback lang None; fallback kind
     desconhecido (capitalização).
   - 7 integration tests em `introspect.rs` (helper
     `introspect_with_lang` + 6 casos): default no lang set; pt
     image; en table; de raw; lang unknown fallback; kind custom;
     contadores independentes (regression P158A).

6. **Documentação atualizada**:
   - Tabela cobertura: entrada `figure` ganha nota P158B + footnote ³¹.
   - ADR-0061 §"Aplicações cumulativas" + padrão #13 novo
     (P155 cross-feature N=1).
   - ADR-0060 anotação Passo 158B.
   - README ADRs entrada P158B antes de ADR-0062-create.

---

## Confirmação das verificações (1-11)

1. **`cargo test`**: 1189+215+24+21 = **1449 tests** passam (era
   1434; +15 dentro do range esperado +12-15). Zero falhas; 6
   ignored em integration tests pré-existentes (inalterado).

2. **`crystalline-lint`**: ✓ No violations found.

3. **Contagem variants Content**: **58** (inalterada — refino
   qualitativo; sem variant novo).

4. **Contagem stdlib funcs**: **48** (inalterada — helper é
   privado em `rules/lang/figure_supplement.rs`, não stdlib func
   nova).

5. **Cobertura Model agregada**: ~50% (inalterada — refino
   qualitativo). Tabela cobertura atualizada com entrada `figure`
   ganhando footnote ³¹ explicando o refino P158B.

6. **Hash `figure_supplement.rs` gerado**: `4426dbc0` (igual a
   `quotes.rs` porque ambos apontam ao prompt L0 `lang.md` —
   correcto per arquitetura modular L0).

7. **Hash `entities/content.rs` permanece `ec58d849`** —
   **décimo primeiro passo consecutivo** (P156L → P158B) a
   preservar contrato L0 do variant Content. ✓

8. **Lang resolution decidido**: **field novo `pub lang: Option<Lang>`
   em `CounterState`** (decisão §8.2 do diagnóstico). Alternativas
   rejeitadas: modificar signature `introspect()` (quebra 10+ call
   sites); walk acompanhar Styled (complexidade arquitectural);
   lang resolution em layout (divergência semântica).

9. **Padrão P155 confirmado**: estrutura paralela
   `LANG_QUOTES: &[(&str, (&str, &str))]` (P155) →
   `LANG_SUPPLEMENTS: &[((&str, &str), &str)]` (P158B); ambos
   lookup linear por exact match + fallback constante. **Primeiro
   reuso explícito cross-feature** do pattern P155 (quotes →
   figure supplement).

10. **Sem novas reservas criadas** — política P158 preservada.
    Refinos i18n adicionais (mais langs, kinds custom, CSL-aware
    format, region-specific supplements, `supplement: Option<Content>`
    field user-facing) permanecem candidatos NÃO-reservados.

11. **Tests pré-existentes de figure label passam inalterados**
    — todas as 7 entradas `figure_label_*_devolve_figura` em
    `introspect.rs` continuam a verificar "Figura {n}" graças
    ao fallback PT (decisão arquitectural-chave §2 do diagnóstico).
    Regression P158A counters por kind: confirmado por
    integration test `figure_contadores_independentes_continuam`.

---

## §Análise de risco (N=17)

**Décima sétima aplicação consecutiva** do padrão "§análise de
risco no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C + P158/A +
P159/A/B + ADR-0062-create + **P158B**).

**Risco realizado**: **muito baixo** (alinhado com previsão da
spec §"Natureza do passo").

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | nulo | Sem alteração ao variant `Content::Figure`; sem variants novos. |
| Backwards compat | nulo | Fallback PT preserva todos os tests pré-existentes que esperam "Figura". |
| Hash content.rs | nulo | Preservado `ec58d849` (11º consecutivo). |
| Reuso de pattern | mínimo | Pattern P155 consolidado e replicado fielmente. |
| Tests dentro do range | nulo | +15 dentro do range +12-15 esperado pela spec. |
| Lang resolution | baixo | Field novo `state.lang: Option<Lang>` adiciona superfície mas é opcional + retrocompatível. |
| Decisão fallback PT vs EN | baixo | Decisão registada em diagnóstico §2 + §8.2 com motivação clara (backwards compat); refactor para EN ou outro NÃO reservado. |

**Cenários da spec §"O que pode sair errado"**:
- Tests pré-existentes esperarem prefix específico hardcoded —
  **realizado e mitigado**: fallback PT preserva o prefix esperado.
- Inventário .1 revelar lang resolution não-trivial — **realizado**:
  decidiu-se por field novo `state.lang` em vez de Style cascade.
- Helper colidir com estrutura existente em `rules/lang/` —
  **não realizado**: novo ficheiro `figure_supplement.rs` paralelo
  a `quotes.rs`.

**Padrão consolidado**: **17 aplicações consecutivas** sem
materialização que exceda risco previsto. **Zero reformulações
mid-passo** em N=15 aplicações de materialização.

---

## Slope cumulativo Model (mesa P155-P158B)

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
| **P158B** | **figure supplement por lang** | **0% agregado** | **50% inalterada (2º refino)** | **+15** |

**Total Model pós-P158B**: +9pp Model agregado em 4 sub-passos
materiais (P155 + P157A + 5 refinos qualitativos: P157B/C, P158A,
P159A, P158B); cobertura ampla impl+impl⁺+parcial cresce 22 → 24
parciais (cite/bib em P159A).

**Padrão emergente**: 5 dos 7 sub-passos materiais Model são
refinos qualitativos (não +pp agregados) — **76% qualitativos**.
Cobertura agregada Model captura insuficientemente o esforço
real; refinos qualitativos enriquecem entradas existentes sem
mover counts. P158B confirma o padrão (segundo refino consecutivo
de mesma feature `figure`).

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P158B adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P158B | figure supplement por lang (Model figure-kinds sub-passo 2) | 0% agregado | Layout 78%; Model 50% inalterado (refino qualitativo) | +15 |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=14 → 15**.
- Inventariar primeiro: **N=16 → 17** (ADR-0065 critério #5
  quinta aplicação concreta).
- §análise de risco no relatório: **N=16 → 17**.
- **Padrão #13 NOVO**: "Padrão P155 i18n reusado cross-feature"
  N=1 (subpadrão emergente; promoção diferida N=3-4 mínima).
- Helper `figure_supplement_for_lang` N=1 (sem reuso até agora).

---

## Confirmações finais

- **Padrão P155 reusado cross-feature**: ✓ confirmado.
  Estrutura paralela tabela estática + lookup linear + fallback
  constante. Primeiro reuso (quotes → figure supplement).
- **Estabilidade hash content.rs N=10 → 11**: ✓ confirmado
  `ec58d849` preservado.
- **Política "sem novas reservas"** (P158): ✓ preservada.
  Nenhuma reserva nova criada para passos pós-P158B.

---

## Estado pós-P158B

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada (segundo refino qualitativo
  consecutivo de `figure`).
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada vs
  P159A; refino qualitativo não move counts).
- **Cobertura arquitectural**: **82%** inalterada (refino de
  variant existente `Content::Figure`; sem variants novos).
- **Hash `entities/content.rs`**: `ec58d849` (11º passo
  consecutivo preservado).
- **63 ADRs** (28 EM VIGOR; ADR-0060 IMPLEMENTADO; 12 PROPOSTO
  incluindo 0062 sem ficheiro inalterado).
- **Tests**: 1449 workspace (1189 lib + 215 integ + 24 + 21).
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada — helper é privado).
- **Padrões consolidados**:
  - Granularidade N=15.
  - Inventariar primeiro N=17.
  - Smart→Option Caso A patamar N=5 (inalterado).
  - §análise risco N=17.
  - Estabilidade hash L0 content.rs N=11.
  - **Subpadrão novo**: P155 cross-feature N=1.

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0062 mantém-se reserva PROPOSTO sem promoção**.

**Próxima decisão (sem candidata pré-acordada — política "sem
novas reservas")**: Bloco A do diagnóstico P159B mantém 4
candidatos restantes pós-P158B (Cite.form, BibEntry fields,
kind refactor, bib numbering); ou outras direcções Bloco B/C
ou módulos diferentes. Decisão humana com máxima informação.
