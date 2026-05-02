# Passo 181A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- 1.700 tests workspace verdes (per auditoria fresh
  2026-04-29).
- `crystalline-lint .` zero violations.
- Refactor Introspection P161–P180 fechado.
- Lacuna #6 (`bib_entries` / `bib_numbers`) inventariada
  em P180. Magnitude **S-M**. Recomendação: implementação
  directa em P181 via padrão sub-store + locatable kind.

Material de partida verificado:

- `00_nucleo/diagnosticos/inventario-bib-state.md` (P180)
  — inventário factual, 9 secções, 6 decisões pendentes
  listadas em §6.
- `00_nucleo/diagnosticos/m1-lacunas-captura.md` — registo
  da lacuna #6 e estado pós-P180.
- `00_nucleo/diagnosticos/auditoria-fresh-projecto.md`
  (2026-04-29) — confirma F1 (`CounterStateLegacy`) ainda
  em aberto; P181 contribui para o caminho M6.

---

## Postura do auditor / executor

P181A é passo **L0-puro / diagnóstico-primeiro**, no mesmo
registo de P154A. Aplicam-se as restrições padrão:

- **Zero código tocado** em `01_core/`, `02_shell/`,
  `03_infra/`, `04_wiring/`.
- **Zero testes** novos ou modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural o
  exigir.
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não materializa** `BibStore`, não modifica
  `extract_payload`, não toca walk arm `Bibliography`.
  Esse trabalho é P181B em diante.

O executor lê `inventario-bib-state.md` como **contexto
factual já validado**. Não re-inventaria. P181A consome o
inventário P180 e produz **decisões + plano executável**.

---

## Escopo

**Primário**: lacuna #6 conforme delimitada por P180.

**Confirmação**: validar que o inventário P180 continua
factualmente correcto (linhas, fields, consumers) na data
de P181A.

**Decisões a tomar**: as 6 cláusulas listadas em
`inventario-bib-state.md` §6:

1. Forma de `BibStore` (`Vec<BibEntry>` simples vs
   `IndexMap<key, BibEntry>`).
2. Multi-Bibliography concat semantics.
3. `bib_numbers` order preservation.
4. Walk arm modificação (Opção α dual-state vs Opção β
   walk puro).
5. Layouter cite-arm migração (caminho similar a P168
   figure-ref).
6. Critério de fecho da lacuna #6.

**Fora de escopo**:

- Implementação de `BibStore` (P181B+).
- Promoção de `Content::Bibliography` a locatable
  (P181C+).
- Migração do Layouter cite-arm (P181 sub-passo tardio).
- Eliminação de `bib_entries` / `bib_numbers` legacy
  (M6).
- Outras lacunas (#1, #2, #4 — não pertencem a P181).

---

## Critérios objectivos

Para cada decisão das 6 cláusulas, registar:

### O1 — Inputs verificáveis

Que ficheiros / linhas / fields foram inspeccionados para
confirmar que a decisão é aplicável. Citação directa
(ficheiro:linha) onde aplicável.

### O2 — Alternativas consideradas

Listar as opções que foram pesadas. Mínimo 2 por decisão
quando há margem real de escolha. Decisões triviais
(ex.: cláusula 3 — preservar `or_insert`) podem ter 1
opção quando a alternativa é regredir comportamento.

### O3 — Critério de escolha

Que regra arquitectural justifica a opção escolhida.
Pode ser ADR existente, invariante de walk puro,
simetria com sub-store anterior (LabelRegistry,
CounterRegistry, MetadataStore, StateRegistry), ou
custo de implementação.

### O4 — Magnitude da decisão

Trivial (sem efeito a montante / a jusante) vs
substancial (afecta API pública, exige ADR nova,
muda invariante).

### O5 — Reversibilidade

Decisão é reversível em sub-passo posterior sem custo
prático? Ou fixa direcção que será cara mudar depois?

---

## Critérios qualitativos

Para o plano agregado (não decisão a decisão):

### Q1 — Consistência com padrão estabelecido

Plano replica padrão sub-store + locatable kind (P165,
P169, P171, P177)? Ou inventa estrutura nova?

### Q2 — Honestidade de magnitude

Estimativa S-M de P180 continua válida após decisões
fixadas? Se decisão escolhida em cláusula 4 (Opção β)
adiciona trabalho, registar revisão.

### Q3 — Simetria com vanilla

Onde cristalino diverge de vanilla (sem hayagriva, sem
CSL, `BibEntry` com 16 fields), divergência é
intencional e documentada? Plano preserva a divergência
ou re-introduz dependência externa?

### Q4 — Fechamento de lacuna

Critério de fecho da lacuna #6 (cláusula 6) é
verificável? Auditor seguinte consegue confirmar
"fechada" sem julgamento subjectivo?

### Q5 — Granularidade dos sub-passos

Os 10 sub-passos `.A`–`.J` propostos por P180 §5 são
S-M individualmente? Ou algum deles esconde trabalho L+?

---

## Sub-passos de P181A

P181A é trabalho de leitura + decisão. Sequência
sugerida:

### Sub-passo 181A.A — Validação do inventário P180

Auditor confirma empiricamente:

- `01_core/src/entities/counter_state_legacy.rs` ainda
  contém `bib_entries` e `bib_numbers` com tipos
  declarados em P180 (`Vec<BibEntry>` e
  `HashMap<String, u32>`).
- `01_core/src/entities/bib_entry.rs` continua com 16
  fields conforme P159A–G.
- `01_core/src/rules/introspect.rs` walk arm
  `Content::Bibliography` continua na linha indicada
  (linha 567 em P180; pode ter shift por edits
  posteriores — registar linha actual).
- `01_core/src/rules/layout/mod.rs` continua com
  Layouter cite-arm como único consumer (linha 584-597
  + sites de cópia state→Layouter em 1386-1388 e
  1414-1416).
- `01_core/src/rules/stdlib/structural.rs::extract_bib_entries`
  continua na função.

Output: tabela com linha por item, "confirmado" /
"desviado". Se houver desvio, registar linha actual e
verificar se afecta decisões P181A.

### Sub-passo 181A.B — Decisão cláusula 1 (forma de BibStore)

Avaliar `Vec<BibEntry>` vs `IndexMap<String, BibEntry>`
contra critérios O1–O5.

Inputs verificáveis: olhar como `LabelRegistry`,
`CounterRegistry`, `MetadataStore`, `StateRegistry`
escolheram a sua estrutura interna. Que padrão emergiu?

Considerar:

- Vanilla usa `IndexMap<PicoStr, hayagriva::Entry>`
  (lookup O(1) + ordem preservada).
- Sugestão P180: `IndexMap` por simetria.
- Custo: `IndexMap` já usado em cristalino (ADR-0023
  autoriza).

Output: decisão fixada com justificação literal.

### Sub-passo 181A.C — Decisão cláusula 2 (Multi-Bibliography concat)

`state.bib_entries.extend(...)` actual concatena.
`BibStore::add_bibliography` deve replicar ou divergir?

Inputs: walk arm actual + tests existentes que cubram
multi-Bibliography.

Output: decisão + critério de fecho operacional ("arm
chama `add_bibliography(entries)` por cada
`Content::Bibliography` encontrado; segundo call
concatena ao Vec interno; numbering preserva primeiro
número via `or_insert`").

### Sub-passo 181A.D — Decisão cláusula 3 (bib_numbers preservation)

Comportamento `or_insert` actual preserva primeiro
número se key duplicada. Cláusula trivial — manter ou
regredir?

Output: decisão = manter (justificação: regressão de
comportamento exige ADR; sem benefício identificado).

### Sub-passo 181A.E — Decisão cláusula 4 (walk arm modificação)

**Esta é a decisão substancial de P181A**.

Opção α (dual-state): walk continua a mutar
`state.bib_*` directamente; `from_tags` adicionalmente
popula `BibStore`. Dual-state durante transição.

Opção β (walk puro): walk emite só Tag; `from_tags`
popula `BibStore`; Layouter usa `BibStore`. Walk arm
puro.

Critério de escolha: P163 invariante "walk não modifica
nada além de emitir Tags + popular CounterStateLegacy"
(a tese de P3 da auditoria fresh: walk puro preservado
em 15 passos consecutivos).

Inputs verificáveis: P162 estabeleceu o padrão
(extract_payload yielda; from_tags popula sub-store).
P165, P169, P171, P177 replicaram. Há precedente?

Considerar: Opção β alinha; Opção α viola invariante.
Mas Opção β exige promoção de `Content::Bibliography` a
locatable kind (`ElementKind::Bibliography` +
`ElementPayload::Bibliography { entries }`), o que é
trabalho adicional.

Output: decisão fixada. Se Opção β escolhida, registar
que payload kind precisa ser adicionado em sub-passo
P181B/C.

Magnitude da decisão: substancial (afecta forma do
ElementPayload + extract_payload + from_tags).

### Sub-passo 181A.F — Decisão cláusula 5 (Layouter cite-arm)

Caminho similar a P168 figure-ref. Inputs: como P168
migrou Layouter? Função `layout_with_introspector`
actual aceita `Introspector` ou clone de state?

Output: decisão sobre quando Layouter migra (P181
sub-passo tardio vs M6 cleanup) e que método trait
adicionar (`Introspector::bib_entry_for_key(key) ->
Option<&BibEntry>` + `bib_number_for_key(key) ->
Option<u32>`).

### Sub-passo 181A.G — Decisão cláusula 6 (critério de fecho)

Lacuna #6 fecha quando:

Opção 1: infraestrutura pronta (BibStore + locatable
kind + Introspector trait methods) **mesmo que**
Layouter não migrou. Layouter migra em sub-passo
posterior; lacuna #6 ficaria meio-fechada.

Opção 2: infraestrutura pronta **e** Layouter migrado
**e** `bib_entries`/`bib_numbers` legacy redundantes.
Critério forte; lacuna fecha quando consumer único
deixar de usar legacy.

Opção 3 (P180 sugerida): "infraestrutura pronta +
consumer migrado". Equivalente a Opção 2 sem exigir
remoção do legacy (M6 elimina campos legacy quando
todos os call-sites tiverem migrado).

Output: critério fixado em palavras literais
verificáveis. Auditor seguinte deve poder marcar lacuna
como ✅ sem ambiguidade.

### Sub-passo 181A.H — Validação do plano de sub-passos

P180 §5 propõe 10 sub-passos `.A`–`.J`. Re-validar
contra decisões fixadas em .B–.G:

- Algum sub-passo precisa ser dividido (L+ disfarçado)?
- Algum precisa ser eliminado (decisão de P181A
  removeu necessidade)?
- Ordem é correcta dada a decisão da cláusula 4?

Output: tabela 10 linhas com sub-passo, escopo
revisto, magnitude S/M/L, dependência (qual sub-passo
deve correr antes).

### Sub-passo 181A.I — ADR

Avaliar se as decisões de P181A justificam ADR nova:

- Se cláusula 4 escolheu Opção β e isso introduz padrão
  novo (não — replica P162/P165/P169/P171/P177): **não
  ADR**.
- Se cláusula 1 escolheu `IndexMap` e ADR-0023 cobre:
  **não ADR nova**.
- Se cláusula 6 fixou critério de fecho de lacuna em
  vocabulário compatível com P180: **não ADR nova**.

Conclusão esperada: P181A **não** cria ADR. Decisões
são consequência de invariantes estabelecidas. Se
auditor identificar excepção, criar ADR `PROPOSTO`.

### Sub-passo 181A.J — Outputs

Produzir 3 ficheiros (mesmo padrão de 154A):

1. **`00_nucleo/diagnosticos/diagnostico-bib-store-passo-181a.md`**
   — diagnóstico com 8 secções:

   - §1 Validação inventário P180.
   - §2 Decisões cláusula 1–6 (uma sub-secção cada,
     formato O1–O5 + opção escolhida + justificação
     literal).
   - §3 Plano de sub-passos revisto (tabela 10 linhas).
   - §4 Magnitude consolidada (re-confirmar S-M de P180
     ou registar revisão).
   - §5 ADR avaliação (se sim, link; se não, justificar).
   - §6 DEBT avaliação (mesmo registo).
   - §7 Critério de fecho lacuna #6 fixado.
   - §8 Próximo sub-passo (P181B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-181a-relatorio.md`**
   — relatório de fecho do passo, com 14 secções
   numeradas no padrão P154A:

   - §1 Sumário.
   - §2–§7 espelham §2–§6 do diagnóstico.
   - §8 Plano de materialização.
   - §9 ADR (se houver).
   - §10 DEBTs (se houver).
   - §11 Inventário 148 actualizado (entrada lacuna #6
     muda de "Inventário concluído P180" para
     "Decisões fixadas P181A").
   - §12 README ADRs actualizado (se §9 produziu ADR).
   - §13 Próximo passo (P181B).
   - §14 Verificação final (tabela com itens cumpridos).

3. **Actualização de
   `00_nucleo/diagnosticos/m1-lacunas-captura.md`** —
   linha lacuna #6 muda de:

   ```
   **Inventário P180**: magnitude S-M confirmada.
   Recomendação: implementação directa em P181 via
   padrão sub-store + locatable kind. 10 sub-passos
   planeados; ~+15-25 tests.
   ```

   para:

   ```
   **Inventário P180 + decisões P181A**: magnitude
   {confirmada/revisada para X}. Sub-passos validados
   (link diagnóstico). Critério de fecho fixado:
   {literal}. Próximo: P181B (link).
   ```

---

## Restrições

- **Zero código tocado** em qualquer ficheiro de
  cristalino fora de `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não materializar `BibStore`** — esse é P181B.
- **Não promover `Content::Bibliography` a locatable** —
  esse é P181B/C.
- **Não modificar walk arm** — esse é P181 sub-passo
  posterior.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória**: se decisão for trivial,
  registar como trivial; se for substancial, registar
  como substancial. Sem "no entanto isto é compensado
  por..." que apaga magnitude.
- **Não pedir confirmação ao humano antes de fixar
  decisões**: P180 §6 já listou as 6 cláusulas como
  decisões a tomar; P181A toma-as com critérios.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-bib-store-passo-181a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-181a-relatorio.md`
  com 14 secções produzido.
- 6 cláusulas de P180 §6 fechadas com decisão literal.
- Plano de 10 sub-passos validado (tabela com escopo +
  magnitude + dependência).
- `m1-lacunas-captura.md` actualizado.
- Magnitude S-M re-confirmada ou revisada com
  justificação.
- Critério de fecho lacuna #6 fixado em palavras
  verificáveis.
- ADR avaliada (criada com `PROPOSTO` se necessária; ou
  justificação literal de "não necessária").
- Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
  `04_wiring/` tocado.
- `cargo test --workspace --lib`: 1.700 inalterados.
- `crystalline-lint .`: zero violations.

P181A é instrumento. Implementação concreta da lacuna #6
fica para P181B em diante.
