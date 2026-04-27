# Relatório Passo P159D — `BibEntry` fields adicionais

Materialização do terceiro sub-passo substantivo Bibliography +
Cite (Bloco A do diagnóstico P159B §3.3). **Décima sétima
aplicação consecutiva de materialização** desde P156C; **refino
estrutural de tipo entity** `BibEntry` sem alteração ao variant
Content (precedente novo).

---

## Resumo do executado

1. **Diagnóstico** em `00_nucleo/diagnosticos/diagnostico-bibentry-fields-passo-159d.md`
   (10 secções: 7 canónicos ADR-0034 + 3 específicos para
   constructor pattern + selecção de fields + layout formato).

2. **Struct `BibEntry` extendido** em
   `01_core/src/entities/bib_entry.rs`:
   - 4 fields novos `Option<String>`: `volume`, `pages`,
     `journal`, `publisher`.
   - Backwards compat preservada — `BibEntry::new(4 args)` original
     continua a funcionar; fields novos default `None`.
   - **Builder pattern fluente** (Opção C diagnóstico §8):
     `with_volume()`, `with_pages()`, `with_journal()`,
     `with_publisher()` — consomem `self`, devolvem `Self`.

3. **Helper `extract_bib_entries` (P159A) extendido** em
   `01_core/src/rules/stdlib/structural.rs`:
   - Helper inline `optional_str(field)` para parsing uniforme
     dos 4 fields opcionais.
   - Validação tipo `Value::Str`; outros tipos rejeitados com
     mensagem mencionando field específico.
   - Constructor `BibEntry::new(...)` chamado com fields novos
     atribuídos directamente.

4. **Helper privado novo `format_bib_entry`** em
   `01_core/src/rules/layout/mod.rs`:
   - Concatenação condicional APA-like.
   - Backwards compat: quando todos os 4 fields opcionais são
     `None`, output idêntico a P159A (`[key] author. title (year).`).
   - Output extendido: `[key] author. title journal vol. volume,
     pp. pages. publisher (year).`.

5. **Tests +8** (1204 → 1212; range esperado +5-8):
   - 3 unit em `bib_entry.rs`: backwards compat fields novos
     None + builder pattern fluente + PartialEq cobre 8 fields.
   - 3 stdlib em `stdlib/mod.rs`: parse com fields novos +
     regression sem fields (P159A) + tipo errado rejeitado.
   - 2 layout E2E em `layout/tests.rs`: entry completa formato
     extendido + entry mínima formato P159A regression.

6. **Documentação atualizada**:
   - Tabela cobertura: entrada `bibliography` ganha nota P159D +
     footnote ³³ novo.
   - ADR-0061 §"Aplicações cumulativas" + padrão #16 NOVO
     ("refino de tipo entity sem alteração ao variant Content"
     N=1).
   - ADR-0060 anotação Passo 159D.
   - README ADRs entrada P159D antes de P159C.

---

## Confirmação das verificações (1-13)

1. **`cargo test`**: 1212+215+24+21 = **1472 tests** workspace
   (era 1464; +8 dentro do range esperado +5-8). Zero falhas;
   6 ignored em integ tests pré-existentes (inalterado).

2. **`crystalline-lint`**: ✓ No violations found.

3. **Contagem variants Content**: **58** (inalterada — refino
   de tipo entity, sem variant novo).

4. **Contagem stdlib funcs**: **48** (inalterada — `native_bibliography`
   modificada via helper extendido; sem stdlib func nova).

5. **Hash `entities/content.rs` permanece** `ec58d849` —
   **décimo terceiro passo consecutivo** via L0-baseline
   interpretation ✓.

6. **Hash `entities/bib_entry.rs` quebra esperada**: **divergência
   da spec aceite** — spec previa quebra (`5a2c0ebd → novo`);
   **L0-baseline interpretation** mantém preservação `5a2c0ebd`
   (prompt `bib_entry.md` não modificado; extensão via
   doc-comment do header + struct doc-comment expandido). Refactor
   administrativo XS futuro pode actualizar `bib_entry.md` para
   mencionar fields opcionais se prioritário (NÃO reservado).

7. **Hash actualizado em prompts L0**: `crystalline-lint
   --fix-hashes` → "Nothing to fix"; nenhum drift.

8. **Decisão sobre constructor pattern**: **Opção C** adoptada
   (builder pattern fluente). Justificação registada em
   diagnóstico §8.2: legibilidade superior em tests, backwards
   compat trivial via `new()` original preservado, idiomático
   Rust. Alternativas Opção A (field assignment directo;
   rejeitada por construtor incompleto) e Opção B (`new_full(8 args)`
   adicional; rejeitada por verbosidade) documentadas.

9. **Decisão de 4 fields escolhidos**: **volume/pages/journal/
   publisher** com justificação per ADR-0065 critério #2
   (universalidade cross-style + cobertura de classes de
   publicação distintas — journals/papers/books/manuals).
   Alternativas (`url`/`doi`/`editor`/`series`/`note`/`isbn`/
   `location`/`organization`) avaliadas em diagnóstico §9.3 e
   diferidas (`url`/`doi` candidatos a sub-passo M futuro como
   par natural). Decisão registada para refinos futuros.

10. **Layout formato extendido**: ordem APA-like decidida em
    diagnóstico §10:
    `[key] author. title journal vol. volume, pp. pages. publisher (year).`
    Separadores: `". "` entre title e journal, `" "` entre
    journal e `vol.`, `", "` entre volume e `pp.`, `". "` entre
    pages e publisher, `" "` entre publisher e `(year)`.

11. **Sem novas reservas criadas** — política P158/P159
    preservada.

12. **Tests pré-existentes Bibliography (P159A) passam
    inalterados** — `layout_bibliography_renderiza_entries_como_lista`
    e `layout_bibliography_e_cite_no_mesmo_documento` continuam
    a verificar `[smith2024]`/`Smith`/`2024`/`Referências` (fields
    novos `None` produzem output P159A original).

13. **`layout_grid` original NÃO modificado** (paridade
    P157A/B/C + P159A/C).

---

## §Análise de risco (N=19)

**Décima nona aplicação consecutiva** do padrão "§análise de
risco no relatório" (P156F/G/H/I/J/K/L + P157/A/B/C + P158/A/B +
P159/A/B/C + ADR-0062-create + **P159D**).

**Risco realizado**: **baixo** (alinhado com previsão da spec
§"Natureza do passo").

**Eixos avaliados**:

| Eixo | Risco | Justificação |
|------|-------|--------------|
| Estrutura | nulo | Refino de tipo entity ortogonal ao enum Content; nenhum variant tocado. |
| Backwards compat | nulo | `new(4 args)` original preservado; fields novos default None produzem output P159A. |
| Hash content.rs | nulo | L0-baseline preservou `ec58d849` (13º consecutivo). |
| Hash bib_entry.rs | nulo | L0-baseline preservou `5a2c0ebd` (spec previa quebra mas extensão via doc-comment). |
| Decisão builder pattern | mínimo | Opção C escolhida com justificação clara; precedente novo idiomático Rust. |
| Selecção de fields universais | mínimo | ADR-0065 critério #2 terceira aplicação concreta; alternativas documentadas e diferidas. |
| Tests dentro do range | nulo | +8 dentro do range +5-8 esperado pela spec. |
| Layout formato | mínimo | APA-like com ordem documentada; backwards compat preservada exactamente. |

**Cenários da spec §"O que pode sair errado"**:
- Constructor `new()` original com signature usada em múltiplos
  sítios — **mitigado**: signature preservada inalterada via
  fields novos default None; testes existentes passam sem
  modificação.
- Layout formato extendido produzir output muito longo —
  **aceite**: linha única dentro do envelope normal de
  bibliografias; refactor multi-line diferido (depende
  multi-region DEBT-56).
- Tests pré-existentes (P159A) esperarem formato exacto
  conflitante — **não realizado**: backwards compat trivial
  preserva output P159A exactamente.
- Helper `extract_bib_entries` complexidade — **mitigado**:
  helper inline `optional_str` parameteriza parsing uniforme;
  redução de duplicação.

**Padrão consolidado**: **19 aplicações consecutivas** sem
materialização que exceda risco previsto. **Zero reformulações
mid-passo** em N=17 aplicações de materialização (9 Layout +
8 Model).

---

## Slope cumulativo Model (mesa P155-P159D)

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
| **P159D** | **BibEntry fields adicionais** | **0% agregado** | **50% inalterada (refino tipo entity)** | **+8** |

**Total Model pós-P159D**: +9pp Model agregado em 5 sub-passos
materiais (P155 + P157A + 7 refinos qualitativos/estruturais:
P157B/C, P158A, P159A, P158B, P159C, P159D); cobertura ampla
impl+impl⁺+parcial cresce 22 → 24 parciais (P159A; P159C/D
mantém).

**Padrão emergente reforçado**: 7 dos 9 sub-passos materiais
Model são refinos qualitativos/estruturais (não +pp agregados)
— **78% qualitativos**. P159D é primeiro a refinar **tipo entity
puro** (sem afectar variant Content) — distinto dos refinos
estruturais de variant Content (P156L Pad, P159C Cite) e
comportamentais (P158A/B figure).

---

## ADR-0061 §"Aplicações cumulativas" anotada

Linha P159D adicionada à tabela:

| Passo | Feature(s) | Slope | Cobertura cumulativa | Tests Δ |
|-------|-----------|------:|---------------------:|--------:|
| P159D | BibEntry fields adicionais (Model bibliography+cite sub-passo 3) | 0% agregado | Layout 78%; Model 50% inalterado (refino tipo entity); ADR-0065 #2 N=2→3 | +8 |

Padrões atualizados:
- Granularidade 1-2 features/passo: **N=16 → 17**.
- Inventariar primeiro: **N=18 → 19** (ADR-0065 critério #5
  sétima aplicação concreta + critério #2 N=2→**3** terceira
  aplicação isolada concreta — selecção de fields universais).
- §análise de risco no relatório: **N=18 → 19**.
- ADR-0064: **NÃO directamente aplicável** (fields são
  `Option<String>` directos sem mapping `Smart<T>`).
- Tipo entity em ficheiro próprio: **N=5 inalterado** (BibEntry
  expande mas continua em `bib_entry.rs`).
- Infraestrutura state lookup: **N=2 inalterado**.
- P155 cross-feature: **N=1 inalterado**.
- **Padrão #16 NOVO**: "Refino de tipo entity sem alteração ao
  variant Content" N=1 (precedente novo; distinto de P156L `Pad`
  e P159C `Cite` que tocaram variants Content).

---

## Confirmações finais

- **ADR-0065 critério #2 patamar N=2 → 3**: ✓ confirmado.
  Aplicações cumulativas: P159A (escolha tipo entity BibEntry
  com 4 fields minimais) + P159C (escolha tipo enum CitationForm
  vs Option<String>) + **P159D (selecção de 4 fields universais
  vs alternativas)**. Critério valida diversidade de aplicação
  (estrutura tipo / variant tipo / selecção fields).
- **Estabilidade hash content.rs N=12 → 13**: ✓ confirmado via
  L0-baseline interpretation. P159D toca apenas tipo entity
  ortogonal ao enum Content.
- **Política "sem novas reservas"** (P158/P159): ✓ preservada.
- **Subpadrão novo #16**: "refino de tipo entity sem alteração
  ao variant Content" (N=1) — precedente útil para refinos
  futuros de outros tipos entity (CitationForm, BibEntry,
  CitationForm enum, etc.).

---

## Estado pós-P159D

- **Layout**: 78% inalterada.
- **Model agregado**: ~50% inalterada (refino tipo entity).
- **Cobertura ampla impl+impl⁺+parcial**: 77% (inalterada).
- **Cobertura arquitectural**: **82%** inalterada (refino tipo
  entity ortogonal ao enum Content).
- **Hash `entities/content.rs`**: `ec58d849` (13º passo
  consecutivo preservado via L0-baseline).
- **Hash `entities/bib_entry.rs`**: `5a2c0ebd` (preservado via
  L0-baseline; spec previa quebra).
- **63 ADRs** (28 EM VIGOR; ADR-0060 IMPLEMENTADO; 12 PROPOSTO
  incluindo 0062 sem promoção).
- **Tests**: 1472 workspace (1212 lib + 215 integ + 24 + 21).
- **Variants Content**: 58 (inalterada).
- **Stdlib funcs**: 48 (inalterada).
- **Padrões consolidados**:
  - Granularidade N=17.
  - Inventariar primeiro N=19.
  - Smart→Option Caso A patamar N=6 (inalterado — P159D não
    aplica Caso A).
  - §análise risco N=19.
  - Estabilidade hash L0 content.rs N=13.
  - Tipo entity em ficheiro próprio N=5 (inalterado).
  - Infraestrutura state lookup N=2 (inalterado).
  - P155 cross-feature N=1 (inalterado).
  - **Refino tipo entity sem alteração Content N=1 (subpadrão
    novo #16)**.

**ADR-0060 mantém-se `IMPLEMENTADO`**. **ADR-0061 mantém-se
`PROPOSTO`**. **ADR-0062 mantém-se reserva PROPOSTO sem promoção**.

**Próxima decisão (sem candidata pré-acordada — política "sem
novas reservas")**: 2 candidatos restantes pós-P159D em Bloco A
do diagnóstico P159B:
- **P158C** — Refactor `kind: String → Option<String>` (XS;
  benefício marginal; quebra hash content.rs **inevitável**;
  ADR-0064 Caso A patamar N=6 → 7).
- **P159F** — Numbering numérico simples Bibliography (M;
  counter local).

Outras direcções: ADR-0062-create (XS administrativo); mudança
de módulo (Introspection P160); Bloco B/C. Decisão humana com
máxima informação.
