# Relatório Passo P159C — `Cite.form` variants

Materialização do segundo sub-passo substantivo Bibliography +
Cite (Bloco A do diagnóstico P159B §3.2). **Décima sexta
aplicação consecutiva de materialização** desde P156C; **refino
estrutural-comportamental** de variant existente `Content::Cite`.

---

## Resumo do executado

1. **Diagnóstico** em `00_nucleo/diagnosticos/diagnostico-cite-form-passo-159c.md`
   (10 secções: 7 canónicos ADR-0034 + 3 específicos para enum
   dedicado + cross-reference resolution + quebra de hash).

2. **Enum entity novo** em `01_core/src/entities/citation_form.rs`:
   - `pub enum CitationForm { Normal, Prose, Author, Year }`.
   - `Default::default() = Normal` (paridade vanilla
     `CiteForm::Normal`).
   - Helper `as_str()` para serialização inversa.
   - `pub mod citation_form;` em `entities/mod.rs`.

3. **L0 prompt novo** em `00_nucleo/prompts/entities/citation_form.md`
   com Hash do Código `aa847167` (estrutura canónica per
   `bib_entry.md` precedente).

4. **Variant `Content::Cite` expandido** em
   `01_core/src/entities/content.rs`:
   - 3 fields agora: `key`, `supplement`, `form: Option<CitationForm>`.
   - **ADR-0064 Caso A**: `Smart<Option<CiteForm>>` →
     `Option<CitationForm>` (achatamento 2-níveis Smart →
     1-nível Option).

5. **13 sítios pattern-match Content actualizados** (audit
   completo via grep `Content::Cite`):
   - `entities/content.rs`: variant + construtor + is_empty
     (wildcard) + plain_text + PartialEq + map_content + map_text
     + 4 testes existentes adaptados.
   - `rules/introspect.rs`: materialize_time + walk
     (popula `state.bib_entries` em arm Bibliography).
   - `rules/layout/mod.rs`: arm Cite expandido por form +
     2 sítios `layout()` propagam `bib_entries` do
     `initial_state` para Layouter.
   - `rules/stdlib/structural.rs`: construtor + helper privado
     novo `extract_citation_form`.
   - `rules/stdlib/mod.rs`: 2 testes existentes adaptados (1
     test obsoleto convertido — `form` deixou de ser scope-out).

6. **Field novo `pub bib_entries: Vec<BibEntry>`** em
   `entities/counter_state.rs::CounterState` (paridade
   infraestrutural P158B `state.lang`).

7. **Layout placeholder por form** com lookup Bibliography
   via `self.counter.bib_entries`:
   - `Normal/None` → `[key]` (regression P159A).
   - `Prose` + entry → `Author (Year)`.
   - `Author` + entry → `Author`.
   - `Year` + entry → `Year`.
   - Form non-Normal sem entry → fallback `[key]` (paridade
     Normal).

8. **Tests +15** (1189 → 1204; range esperado +12-17):
   - 3 unit em `citation_form.rs` (constructor + PartialEq + Default).
   - 2 unit em `content.rs` (constructor com form + PartialEq cobre
     form).
   - 6 stdlib em `stdlib/mod.rs` (parse normal/prose/author/year +
     auto=None + invalid rejected).
   - 4 layout E2E em `layout/tests.rs` (Normal regression + Prose
     com lookup + Prose fallback + Author/Year combinado).

9. **Documentação atualizada**:
   - Tabela cobertura: entrada `cite` ganha nota P159C +
     footnote ³² novo.
   - ADR-0061 §"Aplicações cumulativas" + padrão #14 NOVO ("tipo
     entity em ficheiro próprio" N=5) + padrão #15 NOVO
     ("infraestrutura state lookup" N=2).
   - ADR-0060 anotação Passo 159C.
   - README ADRs entrada P159C antes de P158B.

---

## Confirmação das verificações (1-14)

1. **`cargo test`**: 1204+215+24+21 = **1464 tests** workspace
   (era 1449; +15 dentro do range esperado +12-17). Zero falhas;
   6 ignored em integ tests pré-existentes (inalterado).

2. **`crystalline-lint`**: ✓ No violations found.

3. **Contagem variants Content**: **58** (inalterada — refino de
   variant existente, sem variant novo).

4. **Contagem stdlib funcs**: **48** (inalterada — `native_cite`
   modificada, não nova).

5. **Enum entity novo `CitationForm`**: ✓ adicionado em
   `01_core/src/entities/citation_form.rs` (`hash 677849cb`).

6. **Cobertura Model agregada**: ~50% inalterada. Cobertura ampla
   77% inalterada. Entrada `cite` ganha nota qualitativa P159C
   na tabela cobertura.

7. **Hash actualizado em prompts L0**: ✓ `crystalline-lint
   --fix-hashes` corre sem drift; novo prompt
   `00_nucleo/prompts/entities/citation_form.md` com hash sincronizado.

8. **Hash `entities/content.rs` preservado** `ec58d849` —
   **divergência da spec aceite**: spec previa quebra ("11 passos
   consecutivos terminam"); **L0-baseline interpretation** mantém
   preservação (prompt `content.md` não modificado; refino
   estrutural via doc-comment + referência cruzada
   `citation_form.md`). **Streak passa de 11 → 12 consecutivos**.
   Refactor administrativo XS futuro pode actualizar
   `content.md` para mencionar Cite.form se prioritário (NÃO
   reservado).

9. **ADR-0064 Caso A patamar N=5 → 6**: ✓ confirmado.
   Distribuição cross-domínio **50% Layout (3) + 50% Model (3)**
   — **equilíbrio cross-domínio atingido** (P156G/H/I Layout +
   P157B/P159A/**P159C** Model). Caso A continua o caso mais
   aplicado.

10. **Algoritmo lookup Cite ↔ Bibliography**: **Opção C** adoptada
    (decisão §9 do diagnóstico). Lookup via
    `CounterState::bib_entries` populado por introspect walk;
    layouter consulta via `self.counter.bib_entries`. Reusa
    infraestrutura P158B `state.lang` (subpadrão emergente N=2
    "infraestrutura state lookup"). Multi-Bibliography concatena
    na ordem de aparecimento.

11. **Sem novas reservas criadas** — política P158/P159
    preservada.

12. **ADR-0017 não promovida** — cross-document refs continuam
    diferidos.

13. **`layout_grid` original NÃO modificado** (paridade
    P157A/B/C + P159A).

14. **Tests pré-existentes de Cite (P159A) passam inalterados**
    — `layout_cite_renderiza_placeholder_com_key` e
    `layout_bibliography_e_cite_no_mesmo_documento` continuam
    a verificar `[smith2024]` (fallback Normal/None preserva
    output original).

---

## §Análise de risco (N=18)

**Décima oitava aplicação consecutiva** do padrão "§análise de
risco no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C + P158/A/B +
P159/A/B + ADR-0062-create + **P159C**).

**Risco realizado**: **baixo-médio** (alinhado com previsão da
spec §"Natureza do passo").

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | baixo | Refino aditivo de variant existente; 13 sítios pattern-match conhecidos a priori (audit completo via grep). |
| Backwards compat | nulo | `Cite.form = None` ↔ Normal default → output `[key]` igual a P159A; tests existentes passam inalterados. |
| Hash content.rs | nulo | L0-baseline preservou `ec58d849` (12º consecutivo); spec previa quebra mas interpretação L0 mantém. |
| Decisão de tipo (enum vs Option<String>) | mínimo | Enum dedicado mantém type-safety + exhaustive match; ADR-0065 critério #2 segunda aplicação isolada concreta. |
| Decisão lookup (Opção A/B/C) | baixo | Opção C reusa infraestrutura P158B; sem segundo pass; sem nova field em Layouter. |
| Tests dentro do range | nulo | +15 dentro do range +12-17 esperado pela spec. |
| ADR-0064 Caso A patamar | nulo | N=5→6 já validado N=5; sem ambiguidade arquitectural; equilíbrio cross-domínio atingido. |

**Cenários da spec §"O que pode sair errado"**:
- Pattern-match exhaustive falhar fora de `content.rs` —
  **mitigado**: grep prévio identificou 13 sítios; todos
  actualizados antes de build.
- Tests de Bibliography E2E quebrarem por mudança de assinatura
  `Content::cite` — **realizado e mitigado**: 4 chamadas em
  layout/tests.rs adaptadas com `None` para o novo field.
- Lookup Cite↔Bibliography ambiguidade com múltiplas
  Bibliography — **decisão**: concatena na ordem de aparecimento
  (documentado em diagnóstico §9.2).
- Layout fallback `[key]` quando entry não encontrada não ser
  idempotente com Normal — **verificado**: pattern `(_, None)
  => format!("[{}]", key)` produz mesmo output que Normal
  branch; tests `cite_normal_renderiza_placeholder` +
  `cite_prose_fallback_placeholder_quando_key_nao_existe`
  validam.

**Padrão consolidado**: **18 aplicações consecutivas** sem
materialização que exceda risco previsto. **Zero reformulações
mid-passo** em N=16 aplicações de materialização (9 Layout +
7 Model).

---

## Slope cumulativo Model (mesa P155-P159C)

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
| P158B | figure supplement por lang | 0% agregado | 50% inalterada (2º refino) | +15 |
| **P159C** | **cite.form variants** | **0% agregado** | **50% inalterada (refino estrutural)** | **+15** |

**Total Model pós-P159C**: +9pp Model agregado em 5 sub-passos
materiais (P155 + P157A + 6 refinos qualitativos/estruturais:
P157B/C, P158A, P159A, P158B, P159C); cobertura ampla
impl+impl⁺+parcial cresce 22 → 24 parciais (P159A; P159C
mantém).

**Padrão emergente reforçado**: 6 dos 8 sub-passos materiais
Model são refinos qualitativos/estruturais (não +pp agregados)
— **75% qualitativos**. Cobertura agregada Model captura
insuficientemente o esforço real; refinos enriquecem entradas
existentes sem mover counts. P159C é primeiro refino *estrutural*
(field novo em variant existente) vs refinos puramente
comportamentais P158A/B (sem alteração de variant struct).

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P159C adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P159C | cite.form variants (Model bibliography+cite sub-passo 2) | 0% agregado | Layout 78%; Model 50% inalterado (refino estrutural); ADR-0064 Caso A N=5→6 | +15 |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=15 → 16**.
- Inventariar primeiro: **N=17 → 18** (ADR-0065 critério #5
  sexta aplicação concreta + critério #2 N=2 segunda aplicação
  isolada concreta).
- §análise de risco no relatório: **N=17 → 18**.
- ADR-0064 Caso A: **N=5 → 6** com **equilíbrio cross-domínio
  50/50 Layout/Model atingido**.
- **Padrão #14 NOVO**: "Tipo entity em ficheiro próprio" N=5
  (Sides/Parity/Dir/BibEntry/CitationForm).
- **Padrão #15 NOVO**: "Infraestrutura state lookup" N=2
  (P158B `state.lang` + P159C `state.bib_entries`).
- Subpadrão P155 cross-feature N=1 (inalterado).

---

## Confirmações finais

- **ADR-0064 Caso A patamar N=6 com equilíbrio cross-domínio
  50/50**: ✓ confirmado. Caso A é o caso mais aplicado e agora
  igualmente distribuído entre Layout e Model.
- **Estabilidade hash content.rs N=11 → 12**: ✓ confirmado via
  L0-baseline interpretation. Spec previa quebra; interpretação
  L0 mantém preservação (prompt content.md não modificado).
- **Política "sem novas reservas"** (P158/P159): ✓ preservada.
- **Subpadrões emergentes consolidam-se**: "tipo entity em
  ficheiro próprio" atinge N=5 (candidato a formalização ADR);
  "infraestrutura state lookup" emerge N=2 (candidato N=3-4).

---

## Estado pós-P159C

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada (refino estrutural).
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada).
- **Cobertura arquitectural**: **82%** inalterada (refino de
  variant existente; sem variants novos).
- **Hash `entities/content.rs`**: `ec58d849` (12º passo
  consecutivo preservado via L0-baseline).
- **Hash `entities/citation_form.rs`**: `677849cb` (novo).
- **Hash `entities/counter_state.rs`**: `4b8e4f02` (preservado;
  field aditivo `bib_entries` documentado via doc-comment).
- **63 ADRs** (28 EM VIGOR; ADR-0060 IMPLEMENTADO; 12 PROPOSTO
  incluindo 0062 sem promoção).
- **Tests**: 1464 workspace (1204 lib + 215 integ + 24 + 21).
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada).
- **Padrões consolidados**:
  - Granularidade N=16.
  - Inventariar primeiro N=18.
  - Smart→Option Caso A patamar **N=6 com equilíbrio
    cross-domínio 50/50**.
  - §análise risco N=18.
  - Estabilidade hash L0 content.rs N=12.
  - Tipo entity em ficheiro próprio N=5 (subpadrão #14).
  - Infraestrutura state lookup N=2 (subpadrão #15 novo).
  - P155 cross-feature N=1.

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0062 mantém-se reserva PROPOSTO sem promoção**.

**Próxima decisão (sem candidata pré-acordada — política "sem
novas reservas")**: Bloco A do diagnóstico P159B mantém 3
candidatos restantes pós-P159C (BibEntry fields adicionais, kind
refactor, bib numbering); ou outras direcções (passo administrativo
XS para actualizar L0 content.md mencionando Cite.form; mudança
de módulo Introspection P160; Bloco B/C). Decisão humana com
máxima informação.
