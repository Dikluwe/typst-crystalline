# Relatório do passo P207A

**Data de execução**: 2026-05-12.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-207A.md`.
**Natureza**: diagnóstico-primeiro (zero código tocado)
+ auditoria empírica de profundidade alta. 40ª aplicação
consecutiva do padrão.
**Sub-passo `A` da série P207** — primeiro de 5 (A-E) per
plano marco M9c.
**Magnitude planeada**: M (M auditoria + S diagnóstico) com
ressalva L se gap revelar-se extenso.
**Magnitude real**: **M** (~50 min; 0 ficheiros código + 4
outputs documentais; sem refactor mid-execução).

---

## §1 O que foi feito

P207A executou diagnóstico-primeiro formal para fixar
empíricamente o **gap entre o trait `Introspector` cristalino
e o vanilla v0.14.2** antes de qualquer decisão de "completar".

Auditoria empírica em 5 blocos (A1–A16) + decisões fixadas em
13 cláusulas (C1–C13) + ADR-0076 PROPOSTO produzido per C9
afirmativa + `P207A.div-1` registada per C12.

Conclusões-chave da auditoria empírica:

- **Trait cristalino tem 20 métodos** (`01_core/src/entities/introspector.rs:41-179`,
  `@prompt-hash 918d279b`); **vanilla tem 16 métodos**
  (`lab/typst-original/.../introspector.rs:28-89`). Cristalino
  é **mais especializado**; vanilla é **mais genérico** via
  Selector polymorphic.
- **Vanilla tem 4 impls** (`EmptyIntrospector`, `PagedIntrospector`,
  `HtmlIntrospector`, `BundleIntrospector`) wrap de
  `ElementIntrospector<P>`; cristalino tem **1 impl única**
  (`TagIntrospector` enriquecido por simplicidade per
  `P205A.div-1`).
- **Sub-stores cristalino (10) vs acceleration structures
  vanilla (5)** — arquitecturas **não-isomorphic**. Cristalino
  tem `LabelRegistry`/`CounterRegistry`/`StateRegistry`/
  `BibStore`/`MetadataStore`/`ResolvedLabelStore`/
  `headings_for_toc`/`SealedPositions` + 2 inline
  (`kind_index`, `figure_label_numbers`). Vanilla tem
  `elems`/`keys`/`locations`/`labels`/`queries cache` +
  Counter/State como **domain types separados**.
- **`here()` e `locate()` confirmados ausentes** em cristalino.
  Bloqueiam ~5+ outros itens (counter.get, state.get,
  Selector::Before/After consumers).
- **Selector cristalino tem 1 variant** (`Kind(ElementKind)`);
  vanilla tem 10 variants (`Elem`, `Location`, `Label`, `Regex`,
  `Can`, `Or`, `And`, `Before`, `After`, `Within`).
- **62 itens classificados** em A15: 11% PARIDADE LITERAL +
  44% DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA + 31% EXTENSÃO
  NECESSÁRIA + 11% DECISÃO PENDENTE.

### Marco fixado: M9c — M9-completion

Per C7 + C6:

- **M9** (Stdlib introspection 11/11) foi fechado em P182F sob
  critério limitado.
- Auditoria P207A revela que trait + sub-store + Selector têm
  gaps que continuam M9 com escopo expandido empírico.
- **Em vez de abrir M10 novo**, M9c reconhece continuidade
  narrativa. Caminho 3 (marco arquitectónico) per C6 fixado.

### Caminho fixado: redução de escopo per `P207A.div-1`

Per C12 + A15 + A16:

- Escopo amplo literal absoluto (62 itens) seria XL+ (~50-60h)
  sem benefício proporcional.
- 44% DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA — tentar "completar"
  inverteria ADR-0073/0074 e perderia propriedades cristalinas.
- **`P207A.div-1` recomenda escopo reduzido**: 22 itens
  materializáveis (19 EXTENSÃO + 3 Selector minimal); 27 itens
  divergência preservados; 7 itens DECISÃO PENDENTE para humano.
- Magnitude resultante: **L** (~30h) em vez de XL.

### Output 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-207A-auditoria-introspector.md`.

Conteúdo:
- §1 Bloco 1 — Trait Introspector (A1-A3; 20 métodos cristalino +
  16 vanilla; 3 tabelas de comparação).
- §2 Bloco 2 — Sub-stores (A4-A6; 10 cristalino vs 5 acceleration
  structures vanilla).
- §3 Bloco 3 — Consumers (A7-A11; here/locate ausentes;
  counter/state minimal; outline/bib parciais).
- §4 Bloco 4 — Selector enum (A12-A14; 1 variant cristalino vs
  10 vanilla).
- §5 Bloco 5 — Classificação (A15-A16; 62 itens; magnitude L-XL).
- §6 Resumo final.

Tamanho: ~21 KB.

### Output 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-207A-diagnostico.md`.

Conteúdo:
- §1-§13 Cláusulas C1-C13 fixadas com valores concretos +
  justificação.
- §14 Decisões durante a leitura (D1-D7).
- §15 Resumo de métricas previstas.

Tamanho: ~17 KB.

### Output 3 — Relatório (este ficheiro)

### Output 4 — ADR-0076 PROPOSTO (per C9 afirmativa)

Localização:
`00_nucleo/adr/typst-adr-0076-introspector-completion.md`.

Estrutura paralela a ADR-0072/0073/0074/0075:
- Contexto + Decisão + Mecanismo (per P207A C1-C8) + Escopo
  reduzido (`P207A.div-1`).
- 5 alternativas consideradas e rejeitadas (escopo amplo vs
  outras paths).
- Consequências positivas/negativas/neutras.
- Plano de validação (7 condições para transitar ACEITE em P211B).
- Plano de materialização (P207-P211 séries M9c).
- Cross-references + Pattern emergente.

Tamanho: ~11 KB.

---

## §2 Tempo de execução

~50 minutos efectivos:

- ~5 min: leitura da spec + setup TaskList + reference files (P206A
  formato).
- ~12 min: A1-A3 (trait cristalino: 20 métodos; trait vanilla: 16
  métodos; comparação trait-a-trait com 3 tabelas).
- ~10 min: A4-A6 (sub-stores cristalino: 10; vanilla: 5
  acceleration structures + Counter/State domain types;
  comparação literal).
- ~8 min: A7-A11 (here/locate ausentes confirmadas; counter/state
  minimal cristalino; outline/bib parciais; page-aware ausentes).
- ~5 min: A12-A14 (Selector cristalino 1 variant; vanilla 10).
- ~5 min: A15-A16 (classificação 62 itens; magnitude L-XL).
- ~3 min: C1-C13 (decisões fixadas com base em auditoria).
- ~2 min: outputs documentais (auditoria + diagnóstico + ADR-0076
  + relatório).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace antes (P207A) | 1873 |
| Tests workspace depois (P207A) | **1873** (∆ 0 — diagnóstico) |
| Tests P207A novos | 0 |
| Linter violations | 0 (sem alteração) |
| Ficheiros novos código | 0 |
| Ficheiros modificados código | 0 |
| Ficheiros novos docs | 4 (auditoria + diagnóstico + relatório + ADR-0076) |
| Ficheiros modificados docs | 0 |
| LOC novas (código) | 0 |
| LOC novas (docs) | ~3500+ (4 outputs P207A) |
| Cargo deps adicionados | 0 |
| Refactor mid-execução | 0 |
| Itens auditados (A15) | 62 |
| Itens classificados PARIDADE LITERAL | 7 |
| Itens classificados DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | 27 |
| Itens classificados EXTENSÃO NECESSÁRIA | 19 |
| Itens classificados DECISÃO PENDENTE | 7 |
| Itens DEBT-55 separados | 2 |

### Tests por crate (sem alteração)

- `typst_core` unit: 1597.
- `typst_infra` unit: 37.
- `typst_shell` unit: 21.
- `typst_wiring` unit: 2.
- Integration tests: 216.
- **Total**: 1873.

---

## §4 Decisões

### D1 — Vanilla trait tem 16 métodos, cristalino 20 — paridade não é numérica

Esperava-se que cristalino fosse subset de vanilla. Empírico:
cristalino tem **mais métodos** (20 vs 16), mas com semântica
**mais especializada** (figure_number_for_label,
formatted_counter_at, is_numbering_active, etc.). Vanilla é
**mais genérico** (query via Selector polymorphic).

Implicação: classificação A15 não é "X tem, Y não tem" — é
"X resolve via specialização, Y resolve via genericidade". 44%
DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA reflecte isto.

### D2 — Sub-stores cristalino têm divergência arquitectónica fundamental

Cristalino tem 9 sub-stores nomeados + SealedPositions; vanilla
tem 5 acceleration structures internas + Counter/State como
domain types. **Não são isomorphic**. Tentar reduzir cristalino a
"ElementIntrospector único" inverteria ADR-0029/0073/0074.

### D3 — `here()` é dependência hub mais crítica

A11 + A16 mostram que `here()` (item 47) bloqueia ~5+ outros
itens. Razão custo (M ~3-4h) / desbloqueio (5+ items) é a mais
alta da auditoria. Por isso C5 fixa série P208 dedicada.

### D4 — `LabelRegistry` single-label é único refactor estrutural necessário

Restantes sub-stores são divergências legítimas. Apenas
`LabelRegistry → MultiMap` (item 37) precisa refactor para
desbloquear `label_count` (item 7).

### D5 — Marco M9c reconhece M9 fechado prematuramente

P182F fechou M9 sob critério "stdlib 11/11". Auditoria P207A
revela trait + sub-store + Selector gaps que continuam M9 com
escopo expandido. **Em vez de abrir M10 novo**, M9c reconhece
continuidade narrativa.

### D6 — Bibliography continua em DEBT-55 separado

DEBT-55 ("Bibliography + Cite XL") continua tracker próprio.
P207 NÃO endereça hayagriva integration. Cristalino bib_*
methods (P181F) continuam parcial; aceitável fora P207.

### D7 — Pixel-perfect / observable divergence preservada (per ADR-0075)

C12 redução de escopo NÃO inverte ADR-0075 (paridade observable
estrutural-only via `typst query`). Maioria dos page-aware items
(C1) afecta layout PDF, mas geometric divergência (FixedMetrics
vs FontBookMetrics) permanece N/A.

### D8 — `P207A.div-1` registada per C12 com fundamento empírico

Pré-fixação ("escopo amplo: trait + sub-stores + consumers")
foi guidance. Auditoria empírica mostra escopo absoluto seria
XL+ (~50-60h) sem benefício proporcional. `P207A.div-1`
recomenda escopo reduzido (22 itens materializáveis) com
fundamento empírico — solicita decisão humana antes de P207B.

### D9 — Sub-passos pós-A em marco M9c (5 séries)

C6 magnitude L agregado (~30h escopo reduzido) é viável em marco
M9c com 5 séries P207-P211, **não** série única. Series dedicadas
(P208 stdlib + P209 Selector + P210 page-aware) separam concerns
ortogonais.

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §9 e cláusulas de decisão:

| Hipótese | Resultado |
|----------|-----------|
| §9: "A15 produz mix ~30% PARIDADE + ~25% DIVERGÊNCIA + ~35% EXTENSÃO + ~10% DECISÃO PENDENTE" | **PARCIALMENTE** — empírico 11/44/31/11; PARIDADE menor, DIVERGÊNCIA maior; arquitecturas mais divergentes do que esperado |
| §9: "magnitude agregada provável: L (com sub-séries) ou XL (se completar tudo)" | **CONFIRMADA** — L escopo reduzido per `P207A.div-1`; XL escopo completo |
| §9: "C6 = Caminho 2 ou 3" | **CONFIRMADA — Caminho 3 (marco arquitectónico)** |
| C9: "Hipótese mais provável: criar ADR-0076" | **CONFIRMADA** — ADR-0076 PROPOSTO criado |
| C1 hipótese P205D D3: "page-relevantes + label_count são EXTENSÃO NECESSÁRIA prováveis" | **CONFIRMADA** literalmente — 5/6 prioritários C1 são page-aware ou multi-label |
| C2 hipótese P205D: "SealedLabelPages foi deferido; P207A pode reabrir" | **REJEITADA** — A11 mostra que `pages(loc)` não tem consumer real imediato; reabrir seria especulativo |
| C3 hipótese P206C D9: "here() desbloqueia consumer real para position_of + Selector::Label" | **CONFIRMADA** literalmente — A11 + A16 mostram here() como dependência hub crítica |
| C5 hipótese: "rich Counter/State materializar como parte de P207" | **REJEITADA** — DECISÃO PENDENTE 49+50; recomendação β manter forma minimal |
| C7 hipótese: "M9 ou F4" | **PARCIAL** — escolhido M9c (M9-completion), híbrido entre os dois |
| §9: "assumir que vanilla é a verdade canónica e classificar tudo como EXTENSÃO NECESSÁRIA" | **EVITADO** — 44% classificado como DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA |
| §9: "inflar escopo para completar tudo" | **EVITADO via `P207A.div-1`** |
| §9: "ignorar que alguns gaps são DEBT-X pre-existing" | **EVITADO** — DEBT-55 (bibliography) e ADR-0054 (FixedMetrics) tratados como divergências legítimas |

12 hipóteses resolvidas pela auditoria empírica. A spec previu
correctamente os critérios; P207A executou-os literalmente.
Pattern alinhado com P204A/P205A/P206A (diagnóstico-primeiro
literal).

---

## §6 Sugestão para próximo sub-passo

P207A fechado per C13 (sem cláusulas condicionais) com **ressalva
crítica**:

- ✓ A1-A16 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  (1 DIVERGÊNCIA ARQUITECTÓNICA fundamental documentada em A6).
- ✓ C1-C12 instanciadas com valores concretos.
- ✓ ADR-0076 PROPOSTO escrito (C9 afirmativa).
- ✓ Magnitude calibrada (C8 = L escopo reduzido).
- ✓ Plano `*B+` sem condicionais (5 séries P207-P211).
- ✓ `P207A.div-1` registada (C12) com escopo reduzido fundamentado.
- ⚠ **C10 lista 4 questões para humano** que precisam ser
  respondidas **antes de P207B começar**:
  - Q1 — Rich `Counter`/`State` types? (recomendação β)
  - Q2 — `Selector::Where` (Element field filter)? (recomendação γ)
  - Q3 — `Selector::Regex` e `Selector::Location`? (recomendação β)
  - Q4 — `query_count_before(sel, end)`? (recomendação β)

**Próximo sub-passo**: **decisão humana sobre `P207A.div-1` +
respostas Q1-Q4**.

Após decisão humana:
- Se `P207A.div-1` aceite + recomendações β/γ confirmadas →
  P207B materializa `query_labelled` (S; sem dependências).
- Se humano rejeita `P207A.div-1` → re-classificar A15 com
  novo critério; potencialmente re-emitir P207A.div-2.
- Se humano clarifica Q1-Q4 com decisão diferente da
  recomendação → ajustar escopo M9c em conformidade.

---

## §7 Cross-references

- **Spec**: `00_nucleo/materialization/typst-passo-207A.md`.
- **Outputs P207A**:
  - `00_nucleo/diagnosticos/typst-passo-207A-auditoria-introspector.md`.
  - `00_nucleo/diagnosticos/typst-passo-207A-diagnostico.md`.
- **ADR produzida (PROPOSTO)**:
  `00_nucleo/adr/typst-adr-0076-introspector-completion.md`
  (PROPOSTO 2026-05-12; transita ACEITE em P211B se 7/7 conds
  cumpridas).
- **Pendências endereçadas**:
  - Gap trait Introspector cristalino vs vanilla — auditado
    empíricamente (62 itens).
  - Marco M9c declarado — extensão narrativa de M9 fechado em
    P182F.
- **Pendências preservadas (fora P207)**:
  - DEBT-55 (Bibliography + Cite XL) — separado.
  - Rich Counter/State types — DECISÃO PENDENTE Q1.
  - `Selector::Where`/`Regex`/`Location` — DECISÃO PENDENTE Q2/Q3.
- **Predecessores série**:
  - P206E (vanilla integration ACEITE final per ADR-0075).
  - P205E (F3 ACEITE final per ADR-0074).
  - P204H (M8 estruturalmente fechado per ADR-0073).
- **Pattern referência**: P206A diagnóstico-primeiro 16 cláusulas
  A1-A16 (paralelo a P207A 16 cláusulas).
- **ADRs cross-referenciadas (não modificadas)**:
  - ADR-0073 (M8 `#[comemo::track]`).
  - ADR-0074 (F3 sub-stores trackable).
  - ADR-0075 (vanilla integration).
  - ADR-0029 (Pureza física `Arc` permitido).
  - ADR-0054 (FixedMetrics — divergência observable preservada).
- **Vanilla typst v0.14.2**:
  `lab/typst-original/crates/typst-library/src/introspection/`
  (13 ficheiros; 3983L) + `lab/typst-original/crates/typst-library/src/foundations/selector.rs`.
