# Relatório Passo P159F — Numbering numérico Bibliography

Materialização do quarto sub-passo Bibliography + Cite —
**último candidato Bloco A** do diagnóstico P159B (§3.5).
**Décima nona aplicação consecutiva de materialização** desde
P156C; **refino comportamental** com subpadrão #15
"infraestrutura state lookup" a atingir N=3.

---

## Resumo do executado

1. **Diagnóstico** em `00_nucleo/diagnosticos/diagnostico-bibliography-numbering-passo-159f.md`
   (10 secções: 7 canónicos ADR-0034 + 3 específicos para
   decisão arquitectural-chave Opção A/B/C + multi-Bibliography
   + interação Cite.form). Matriz multi-critério mais elaborada
   da série pós-P156.

2. **Field aditivo `pub bib_numbers: HashMap<String, u32>`** em
   `01_core/src/entities/counter_state.rs::CounterState`
   (paridade aditiva infraestrutura state lookup; **subpadrão
   #15 cresce N=2 → 3** via `state.lang` P158B + `state.bib_entries`
   P159C + **`state.bib_numbers` P159F**).

3. **Walk arm `Content::Bibliography`** em `01_core/src/rules/introspect.rs`:
   - Popula `state.bib_numbers` contínuamente:
     `state.bib_numbers.entry(key.clone()).or_insert(len + 1)`.
   - Multi-Bibliography preserva primeiro número (paridade
     HashMap; comportamento determinístico).

4. **Layout arm `Content::Cite { form: Normal/None }`** em
   `01_core/src/rules/layout/mod.rs`:
   - Lookup `state.bib_numbers.get(key)` → `[N]` ou fallback
     `[key]` (regression P159A).
   - Forms diferenciadas (Prose/Author/Year) inalteradas — match
     arms preservados (regression P159C).

5. **Propagação `bib_numbers`** em `layout()` (paridade P159C
   `bib_entries`):
   - Branch sem fixpoint: `l.counter.bib_numbers = initial_state.bib_numbers`.
   - Branch com fixpoint: `.clone()` em cada iteração.

6. **Tests +8** (1214 → 1222; range esperado +10-15
   ligeiramente abaixo por helper inline trivial):
   - 2 unit em `counter_state.rs`: `bib_numbers_default_empty`
     + `bib_numbers_insertion_e_lookup`.
   - 6 layout E2E em `layout/tests.rs`:
     - `cite_normal_renderiza_numero_quando_bib_populada`.
     - `cite_normal_fallback_placeholder_quando_bib_vazia`
       (regression P159A).
     - `cite_normal_multiple_entries_numeradas_em_ordem`.
     - `cite_form_prose_inalterada_com_bib_numerada` (regression
       P159C).
     - `cite_unknown_key_fallback_placeholder` (regression P159A).
     - `cite_normal_multi_bibliography_continua`.

7. **Documentação atualizada**:
   - Tabela cobertura: entradas `cite` e `bibliography` ganham
     nota P159F + footnote ³⁵ novo (ambas referenciam).
   - ADR-0061 §"Aplicações cumulativas" + padrão #15 atualizado
     N=2→3 (atinge limiar formalização N=3-4).
   - ADR-0060 anotação Passo 159F + marca conceptual "Bloco A
     esgotado".
   - README ADRs entrada P159F antes de P158C.

---

## Confirmação das verificações (1-14)

1. **`cargo test`**: 1222+215+24+21 = **1482 tests** workspace
   (era 1474; +8 dentro do range esperado +10-15 ligeiramente
   abaixo). Zero falhas; 6 ignored em integ tests pré-existentes
   (inalterado).

2. **`crystalline-lint`**: ✓ No violations found.

3. **Contagem variants Content**: **58** (inalterada — refino
   layout/introspect; sem variant novo).

4. **Contagem stdlib funcs**: **48** (inalterada — sem stdlib
   nova; helper inline trivial).

5. **Hash `entities/content.rs` preservado** `ec58d849` —
   **décimo quinto passo consecutivo** ✓ via L0-baseline
   interpretation. Sem alteração ao variant Content.

6. **Decisão arquitectural-chave default Opção A/B/C registada**:
   **Opção C** adoptada (Cite.form interaction sem field
   user-facing). Justificação multi-critério em diagnóstico §8.2:
   backwards compat trivial + sem alteração estrutural + reuso
   pattern P159C + comportamento intuitivo. Trade-off aceite:
   comportamento implícito depende ordem walk (single-pass
   garante state populado antes do layout).

7. **Decisão multi-Bibliography contínua registada**: paridade
   vanilla numeric style (decisão diagnóstico §9). Algoritmo:
   `or_insert` preserva primeiro número.

8. **Decisão interação Cite.form registada**: numeração só em
   Normal/None (decisão diagnóstico §10). Forms Prose/Author/Year
   inalteradas — preserva semântica forms diferenciadas P159C.

9. **Tests pré-existentes Cite Normal (P159A/C) passam** ✓:
   - `[smith2024]` quando Bibliography vazia ou Cite key não
     encontrada (regression).
   - `[N]` quando Bibliography populada e key encontrada
     (novo comportamento P159F — verificado por
     `cite_normal_renderiza_numero_quando_bib_populada` +
     `cite_normal_multiple_entries_numeradas_em_ordem`).

10. **Tests pré-existentes Cite forms Prose/Author/Year (P159C)
    passam inalterados** ✓ — verificado por
    `cite_form_prose_inalterada_com_bib_numerada`.

11. **Multi-Bibliography contínua funciona em E2E test** ✓ —
    `cite_normal_multi_bibliography_continua` verifica
    `third → [3]` em segunda Bibliography (após `first → [1]`,
    `second → [2]` em primeira).

12. **Sem novas reservas criadas** — política P158/P159
    preservada.

13. **ADR-0017 não promovida** — counter single-pass viável;
    sem cross-document refs.

14. **`layout_grid` original NÃO modificado** (paridade
    P157A/B/C + P159A/C/D).

---

## §Análise de risco (N=21)

**Vigésima primeira aplicação consecutiva** do padrão "§análise
de risco no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C +
P158/A/B/C + P159/A/B/C/D + ADR-0062-create + **P159F**).

**Risco realizado**: **baixo-médio** (alinhado com previsão
da spec §"Natureza do passo").

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | nulo | Field aditivo em CounterState; sem alteração ao variant Content. |
| Backwards compat | nulo | Tests P159A/C preservados via fallback `[key]` quando bib_numbers vazio ou key não encontrada. |
| Hash content.rs | nulo | L0-baseline preservou `ec58d849` (15º consecutivo). |
| Decisão arquitectural-chave A/B/C | mínimo | Matriz multi-critério resolvida em diagnóstico §8.2; pré-recomendação Opção C confirmada. |
| Subpadrão #15 N=2→3 | nulo | Patamar atinge limiar formalização N=3-4; precedentes P158B/C consolidados. |
| Tests dentro do range | mínimo | +8 ligeiramente abaixo do range +10-15; helper inline trivial reduz superfície de tests. |
| Multi-Bibliography contínua | nulo | Paridade vanilla; HashMap or_insert determinístico. |
| Interação Cite.form preservada | nulo | Match arm distinto para Normal/None vs forms diferenciadas. |

**Cenários da spec §"O que pode sair errado"**:
- Walk multi-Bibliography duplicar keys — **mitigado**:
  `or_insert` preserva primeiro número.
- Cite com key não em Bibliography — **mitigado**: fallback
  `[key]` testado por `cite_unknown_key_fallback_placeholder`.
- Decisão Opção A/B/C ambiguidade — **resolvida**: matriz
  multi-critério em diagnóstico §8.1; Opção C escolhida com
  justificação clara.
- Cite.form Normal interagir mal com walk single-pass —
  **não realizado**: walk corre antes do layout; state
  populado antes do lookup.
- Tests E2E sensíveis a ordem (Cite antes/depois Bibliography) —
  **mitigado**: `layout_with_introspect` corre walk completo
  antes do layout; ordem no documento não importa para
  state populado.
- L0-baseline NÃO preservar hash content.rs — **não realizado**:
  variant Content inalterado; refino vive em CounterState +
  introspect/layout.

**Padrão consolidado**: **21 aplicações consecutivas** sem
materialização que exceda risco previsto. **Zero reformulações
mid-passo** em N=19 aplicações de materialização (9 Layout +
10 Model).

---

## Slope cumulativo Model (mesa P155-P159F)

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
| **P159F** | **Bibliography numbering numérico** | **0% agregado** | **50% inalterada (numbering numérico)** | **+8** |

**Total Model pós-P159F**: +9pp Model agregado em 5 sub-passos
materiais (P155 + P157A + 9 refinos qualitativos/estruturais/
cosméticos/numbering: P157B/C, P158A, P159A, P158B, P159C,
P159D, P158C, P159F); cobertura ampla impl+impl⁺+parcial cresce
22 → 24 parciais (P159A; outros mantém).

**Padrão emergente reforçado**: 9 dos 11 sub-passos materiais
Model são refinos qualitativos/estruturais/cosméticos/numbering
(não +pp agregados) — **82% qualitativos**. P159F é **primeiro
refino comportamental com extensão de infraestrutura state
lookup** (subpadrão #15 cresce N=3).

**Marca conceptual P159F**: **Bloco A do diagnóstico P159B
ESGOTADO**. Tecto Model puro estimado (~55-60%) atingido
empiricamente com 24 entradas parciais.

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P159F adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P159F | Bibliography numbering numérico (Model bibliography+cite sub-passo 4 — **último Bloco A**) | 0% agregado | Layout 78%; Model 50% inalterado (numbering numérico); subpadrão #15 N=2→3 | +8 |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=18 → 19**.
- Inventariar primeiro: **N=20 → 21** (ADR-0065 critério #5
  nona aplicação concreta com matriz multi-critério Opção A/B/C
  mais elaborada do passo).
- §análise de risco no relatório: **N=20 → 21**.
- ADR-0064: NÃO directamente aplicável em P159F (Opção C; sem
  field novo).
- Tipo entity em ficheiro próprio: **N=5 inalterado**.
- **Subpadrão #15 "infraestrutura state lookup": N=2 → 3**
  (atinge limiar formalização N=3-4; promoção a ADR meta possível
  em passo administrativo XS futuro).
- P155 cross-feature: **N=1 inalterado**.
- Refino tipo entity sem alteração Content: **N=1 inalterado**.
- Refactor de field para Option: **N=1 inalterado**.

---

## Confirmações finais

- **Estabilidade hash content.rs N=14 → 15**: ✓ confirmado
  via L0-baseline interpretation. Refino comportamental em
  CounterState + introspect/layout não afecta variant Content.
- **Subpadrão #15 N=2 → 3**: ✓ confirmado. Atinge limiar
  formalização N=3-4; promoção a ADR meta possível em passo
  administrativo XS futuro.
- **Política "sem novas reservas"** (P158/P159): ✓ preservada.
- **Decisão arquitectural-chave Opção C**: documentada com
  matriz multi-critério em diagnóstico §8.

**Marca conceptual** ✓ confirmada: **Bloco A do diagnóstico
P159B ESGOTADO** após P159F. Tecto Model puro estimado
(~55-60%) atingido empiricamente com 24 entradas parciais.

---

## Estado pós-P159F

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada (refino comportamental).
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada).
- **Cobertura arquitectural**: **82%** inalterada (refino
  comportamental + extensão infraestrutura state lookup; sem
  alteração estrutural).
- **Hash `entities/content.rs`**: `ec58d849` (**15º passo
  consecutivo** preservado via L0-baseline).
- **63 ADRs** (28 EM VIGOR; ADR-0060 IMPLEMENTADO; 12 PROPOSTO
  incluindo 0062 sem promoção).
- **Tests**: 1482 workspace (1222 lib + 215 integ + 24 + 21).
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada).
- **Padrões consolidados**:
  - Granularidade N=19.
  - Inventariar primeiro N=21.
  - Smart→Option Caso A patamar N=7 (inalterado — P159F não
    aplica Caso A).
  - §análise risco N=21.
  - Estabilidade hash L0 content.rs N=15.
  - Tipo entity em ficheiro próprio N=5 (inalterado).
  - **Infraestrutura state lookup N=2 → 3** (atinge limiar
    formalização N=3-4).
  - P155 cross-feature N=1 (inalterado).
  - Refino tipo entity sem alteração Content N=1 (inalterado).
  - Refactor de field para Option N=1 (inalterado).

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0062 mantém-se reserva PROPOSTO sem promoção**.

**Marca conceptual P159F**: **Bloco A do diagnóstico P159B
ESGOTADO**. Tecto Model puro estimado (~55-60%) atingido
empiricamente com 24 entradas parciais.

**Próxima decisão (sem candidata pré-acordada — política "sem
novas reservas")**: pós-P159F, Bloco A esgotado. Próximas
direcções:

- **ADR-0062-create** — XS administrativo; desbloqueia Bloco B.
- **Bloco B (hayagriva)**: P159G (Cargo.toml + crystalline.toml
  hayagriva) após ADR-0062 PROPOSTO; depois P159H (hayagriva
  integration) → P159I (CSL APA) → P159J (CSL adicionais).
- **Bloco C (cross-módulo)**: refactor multi-region L+
  (DEBT-34e + DEBT-56) ou Introspection P160 (mudança de módulo).
- **Refinos Model fora Bloco A**: mais langs em
  `figure_supplement_for_lang`; `url`/`doi` em BibEntry; etc.
- **Mudança de módulo**: Layout Fase 3 (columns/colbreak) ou
  Introspection P160.
- **Passos administrativos XS**: actualizar L0 prompt
  `content.md` mencionando variants Bibliography/Cite/Cite.form;
  promover ADR-0060 a R1; ADR meta saturação ADR-0064;
  **ADR meta subpadrão #15 (infraestrutura state lookup; agora
  patamar N=3 atinge limiar)**.

**Pausa natural após P159F — Bibliography ganha numbering
numérico; Bloco A do diagnóstico P159B esgotado; tecto Model
puro atingido (~55-60% estimado com 24 entradas parciais);
estabilidade hash content.rs N=15; subpadrão #15 atinge N=3
(limiar formalização). Decisão humana sobre próxima direcção
tem máxima informação acumulada — informação útil para escolher
entre Bloco B/C, refinos não-listados, mudança de módulo, ou
passos administrativos.**
