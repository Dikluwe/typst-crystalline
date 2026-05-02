# Diagnóstico Bib Store — Passo 181A

Documento P181A (2026-05-01). Decisões fixadas para as 6 cláusulas
listadas em `inventario-bib-state.md` §6 (P180), validação empírica
do inventário, plano de sub-passos revisto e critério de fecho da
lacuna #6.

Padrão estabelecido por P154A (`diagnostico-model-passo-154a.md`).

P181A é passo **L0-puro / diagnóstico-primeiro**. Zero código tocado
em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/`. Zero testes
modificados.

---

## §1 — Validação do inventário P180

Tabela linha-a-linha contra os pontos âncora declarados no passo P180
(`inventario-bib-state.md`). Linhas confirmadas via `grep -n` directo
sobre o estado actual da `Tekt`.

| Item P180 | Localização declarada | Estado actual | Confirmação |
|-----------|-----------------------|---------------|-------------|
| `bib_entries: Vec<BibEntry>` em `CounterStateLegacy` | `01_core/src/entities/counter_state_legacy.rs:84` | Linha **84** | ✓ confirmado |
| `bib_numbers: HashMap<String, u32>` em `CounterStateLegacy` | `01_core/src/entities/counter_state_legacy.rs:92` | Linha **92** | ✓ confirmado |
| `BibEntry` com 16 fields | `01_core/src/entities/bib_entry.rs:82-100` | 16 `pub` fields (4 obrig. + 4 comuns + 2 ident. + 6 restantes) | ✓ confirmado |
| Walk arm `Content::Bibliography` | `01_core/src/rules/introspect.rs:567` | Linha **567** | ✓ confirmado (sem shift) |
| Walk arm corpo (lookup `or_insert` + `extend`) | `introspect.rs:567-573` | Idêntico ao bloco citado em P180 §1.5 | ✓ confirmado |
| Layouter cite-arm | `01_core/src/rules/layout/mod.rs:584-597` | Linhas **584-597** (lookup `bib_entries.iter().find` + match `(form, entry)`) | ✓ confirmado |
| Layouter copy-site #1 (`pub fn layout`) | `mod.rs:1386-1388` | Linhas **1385-1388** | ⚠ shift de **1 linha**; comportamento idêntico |
| Layouter copy-site #2 (`pub fn layout_with_introspector`) | `mod.rs:1414-1416` | Linhas **1413-1416** | ⚠ shift de **1 linha**; comportamento idêntico |
| `extract_bib_entries` em stdlib | `01_core/src/rules/stdlib/structural.rs:516` | Linha **516** | ✓ confirmado |

**Conclusão**: inventário P180 factualmente correcto. Único desvio
são 2 deslocamentos de **1 linha** nos copy-sites do Layouter (efeito
de edits cosméticos sem alteração de comportamento). Nenhuma
decisão de P181A é afectada.

**Baseline empírica** confirmada hoje: `cargo test --workspace --lib`
verde (1440 core + 215 infra +6 ignored + 24 shell); `crystalline-lint .`
zero violations. Estado coerente com auditoria fresh 2026-04-29.

---

## §2 — Decisões cláusulas 1–6

Cada cláusula registada com o esquema **O1–O5** (inputs verificáveis,
alternativas consideradas, critério de escolha, magnitude,
reversibilidade) seguido da opção fixada em palavras literais.

### §2.1 — Cláusula 1: Forma de `BibStore`

**O1 — Inputs verificáveis**:
- `LabelRegistry` (`label_registry.rs:24`) → `HashMap<Label, Location>`.
- `CounterRegistry` (`counter_registry.rs:26-33`) → `HashMap<String, Vec<usize>>` + `history: HashMap<String, Vec<(Location, Vec<usize>)>>`.
- `MetadataStore` (`metadata_store.rs:25`) → `Vec<Value>` (linear).
- `StateRegistry` (`state_registry.rs:34`) → `HashMap<String, Vec<(Location, Value)>>`.
- Estado actual: `bib_entries: Vec<BibEntry>` (ordem de inserção) + `bib_numbers: HashMap<String, u32>` (lookup O(1)).
- Layouter cite-arm (`mod.rs:584`) faz `iter().find(|e| e.key == *key)` — lookup O(n) actualmente.

**O2 — Alternativas consideradas**:
- (a) `Vec<BibEntry>` + `HashMap<String, u32>` — replica literalmente o estado actual; simetria com `MetadataStore` (Vec) e `StateRegistry` (HashMap).
- (b) `IndexMap<String, BibEntry>` (sugestão P180) + `HashMap<String, u32>` — ordem preservada + lookup O(1) num único colectivo.
- (c) `Vec<BibEntry>` + `HashMap<String, u32>` + `entry_index: HashMap<String, usize>` — lookup O(1) sem alterar shape.

**O3 — Critério de escolha**: nenhum sub-store cristalino existente
usa `IndexMap` (4/4 sub-stores usam `HashMap` ou `Vec`). ADR-0023
autoriza `IndexMap` em L1 mas o seu uso primário é `Scope` (Binding
ordering significativo). Bib lookup hot-path não foi medido como
gargalo; cite-arm é chamada uma vez por `Content::Cite`. Replicação
literal do shape actual elimina ambiguidade de migração.

**O4 — Magnitude**: trivial. Decisão de shape interno; sem efeito a
montante (walk arm) ou a jusante (Layouter trait API).

**O5 — Reversibilidade**: alta. Trocar shape interno do `BibStore` em
sub-passo posterior é local — apenas o ficheiro `bib_store.rs` muda.
Métodos `lookup_entry(key) -> Option<&BibEntry>` + `lookup_number(key)
-> Option<u32>` permanecem estáveis.

**Decisão fixada**: opção (a). `BibStore` armazena
`entries: Vec<BibEntry>` + `numbers: HashMap<String, u32>`,
replicando literalmente o shape actual em `CounterStateLegacy`.

### §2.2 — Cláusula 2: Multi-Bibliography concat semantics

**O1**: walk arm em `introspect.rs:572` faz
`state.bib_entries.extend(entries.iter().cloned())`. Concatena por
ordem de aparecimento. Sem deduplicação.

**O2**:
- (a) Replicar — `BibStore::add_bibliography(entries)` faz `self.entries.extend(entries)`.
- (b) Deduplicar por `key` — segundo bib com mesma key não adiciona.

**O3**: alterar comportamento exige ADR (regressão observable).
Inventário P180 §6.2 sugeriu replicar; nenhum consumer pediu
deduplicação.

**O4**: trivial.

**O5**: alta — método interno; muda apenas o corpo de `add_bibliography`.

**Decisão fixada**: opção (a). Arm chama
`bib_store.add_bibliography(entries)` por cada `Content::Bibliography`
encontrado; segundo call concatena ao `Vec` interno; numbering
preserva primeiro número via `or_insert` (ver §2.3).

### §2.3 — Cláusula 3: `bib_numbers` order preservation

**O1**: walk arm em `introspect.rs:569-571` usa
`state.bib_numbers.entry(entry.key.clone()).or_insert(next_num)` —
duplicate key NÃO sobrescreve número.

**O2**:
- (a) Manter `or_insert` (não sobrescreve duplicate).
- (b) Regressão para `insert` (sobrescreve duplicate).

**O3**: cláusula trivial; (b) é regressão de comportamento documentado
em P159F. Nenhum benefício identificado.

**O4**: trivial.

**O5**: irrelevante (decisão = manter status quo).

**Decisão fixada**: opção (a). `BibStore::add_bibliography(entries)`
itera entries e aplica `numbers.entry(key.clone()).or_insert(next_num)`
para cada — primeiro número de uma key persiste.

### §2.4 — Cláusula 4: Walk arm modificação **(decisão substancial)**

**O1**:
- P162 estabeleceu o padrão: `extract_payload` yielda payload;
  `from_tags` popula sub-store; walk emite Tag mas não muta state
  além de `CounterStateLegacy` legacy (excepção declarada P162).
- P165 (LabelRegistry), P169 (MetadataStore), P171 (StateRegistry),
  P177 (CounterRegistry hierarchical), P178 (Outline) replicaram o
  padrão.
- P163 invariante "walk não modifica nada além de emitir Tags +
  popular CounterStateLegacy" preservado em 15 passos consecutivos
  (P3 da auditoria fresh 2026-04-29).
- Estado actual walk arm `Content::Bibliography`
  (`introspect.rs:567-573`): muta `state.bib_entries` e
  `state.bib_numbers` directamente. Não emite Tag — `Content::Bibliography`
  não é locatable hoje (`locatable.rs:85` lista Bibliography no
  bloco non-locatable).

**O2**:
- **Opção α (dual-state)**: walk continua a mutar `state.bib_*`;
  `from_tags` adicionalmente popula `BibStore`. Dual-state durante
  transição. Layouter pode escolher consumer.
- **Opção β (walk puro)**: walk emite Tag (`ElementInfo { payload:
  Bibliography { entries } }`); `from_tags` arm Bibliography popula
  `BibStore`; Layouter consulta via Introspector. Mutação directa
  removida.

**O3**: invariante P163 — walk puro — é a forma estabelecida
pós-P162. Cinco features (P165, P169, P171, P177, P178) foram
materializadas via Opção β; nenhuma usou Opção α. Opção α reintroduz
mutação directa em walk arm, contradizendo o padrão consolidado.

**O4**: substancial. Afecta `ElementKind` (+1 variant `Bibliography`),
`ElementPayload` (+1 variant `Bibliography { entries: Vec<BibEntry> }`),
`is_locatable` (Bibliography move de não-locatable para locatable),
`extract_payload` (+1 arm), `from_tags` (+1 arm), e o próprio walk
arm. Mas **não introduz padrão novo** — replica P162/P165/P169/P171/
P177/P178.

**O5**: baixa. Reverter Opção β em sub-passo posterior exige eliminar
o variant locatable + reintroduzir mutação em walk — múltiplos sítios.

**Decisão fixada**: **Opção β (walk puro)**.

- `ElementKind::Bibliography` adicionado.
- `ElementPayload::Bibliography { entries: Vec<BibEntry> }`
  adicionado.
- `is_locatable(Content::Bibliography) == true`.
- `extract_payload` arm Bibliography retorna
  `Some(ElementPayload::Bibliography { entries: entries.clone() })`.
- `from_tags` arm Bibliography chama
  `bib_store.add_bibliography(entries)`.
- Walk arm `Content::Bibliography` deixa de mutar `state.bib_*`;
  apenas desce no `title` quando presente. Tag é emitida pelo
  mecanismo standard de locatable (ver `walk` para `Heading` /
  `Outline`).

### §2.5 — Cláusula 5: Layouter cite-arm migração

**O1**:
- P168 (figure-ref) migrou Layouter cite-arm análoga (figure
  references) com mesmo padrão: `Introspector::figure_label_number(key)
  -> Option<usize>` adicionado ao trait; cite-arm consulta via
  Introspector.
- `pub fn layout_with_introspector` (`mod.rs:1413+`) já existe (entry
  point P168). Aceita `Box<dyn Introspector>` + clone de state inicial
  para construir Layouter.
- Cite-arm actual lê `self.counter.bib_entries.iter().find(...)` e
  `self.counter.bib_numbers.get(key)` — duas leituras directas.

**O2**:
- (a) Adicionar `Introspector::bib_entry_for_key(key) ->
  Option<&BibEntry>` + `Introspector::bib_number_for_key(key) ->
  Option<u32>` ao trait + impl em `TagIntrospector`. Cite-arm passa
  a consultar via `self.introspector.bib_entry_for_key(...)`.
- (b) Migrar Layouter para receber `&BibStore` directo (sem passar
  pelo trait Introspector).
- (c) Adiar migração para M6 (deixar legacy ainda durante todo P181).

**O3**: simetria com P168 — mesmo padrão (trait method + cite-arm
consume). Opção (b) introduz dependência Layouter→`BibStore` directa,
quebrando encapsulação Introspector. Opção (c) deixa lacuna #6
meio-fechada (infraestrutura sem consumer migrado).

**O4**: substancial (toca trait `Introspector`, impl
`TagIntrospector`, cite-arm em Layouter, copy-sites em `pub fn
layout`/`pub fn layout_with_introspector`). Mas é replicação literal
do padrão P168.

**O5**: alta. Reverter trait method é remoção local; reverter cite-arm
é restaurar 4-5 linhas.

**Decisão fixada**: opção (a). Em sub-passo P181 tardio (`.G`):

- `Introspector::bib_entry_for_key(&self, key: &str) ->
  Option<&BibEntry>` adicionado ao trait.
- `Introspector::bib_number_for_key(&self, key: &str) -> Option<u32>`
  adicionado ao trait.
- `TagIntrospector` delega para `self.bib_store.lookup_entry(key)` e
  `self.bib_store.lookup_number(key)`.
- Layouter cite-arm (`mod.rs:584-597`) consulta via
  `self.introspector.bib_entry_for_key(key)` em vez de
  `self.counter.bib_entries.iter().find(...)`.
- Copy-sites em `pub fn layout` (linhas 1385-1388) e `pub fn
  layout_with_introspector` (linhas 1413-1416) podem permanecer
  durante P181 (compat) e ser eliminados em M6 quando os campos
  legacy forem removidos.

### §2.6 — Cláusula 6: Critério de fecho da lacuna #6

**O1**:
- M6 cleanup é objectivo explícito da migração — eliminar
  `CounterStateLegacy` field-a-field (auditoria fresh F1, lacuna #6
  inventariada em P180).
- Padrão Outline (P178) fechou lacuna #7 com critério "infraestrutura
  pronta + query("outline") retorna count correcto" — sem exigir
  remoção do field legacy `state.has_outline`.

**O2**:
- **Opção 1 (meio-fecho)**: lacuna fecha quando infraestrutura está
  pronta (BibStore + locatable kind + trait methods), mesmo que
  Layouter ainda consuma legacy.
- **Opção 2 (fecho forte)**: lacuna fecha quando infraestrutura
  pronta + Layouter migrado + fields legacy removidos.
- **Opção 3 (sugestão P180)**: lacuna fecha quando infraestrutura
  pronta + consumer migrado, **mantendo** fields legacy até M6 (que
  remove fields quando todos os call-sites tiverem migrado).

**O3**: Opção 1 deixa "lacuna #6 fechada" mas walk continua a
mutar legacy state — P163 invariante violado. Opção 2 mistura M6
(remoção do `CounterStateLegacy`) com fecho de lacuna específica;
M6 é trabalho separado documentado em F1 da auditoria fresh. Opção
3 alinha com Outline (P178) e separa "feature canonicalmente
introspectada" de "field legacy eliminado".

**O4**: trivial (escolha de critério, não de implementação).

**O5**: alta. Mudar critério em sub-passo posterior é re-redação de
texto.

**Decisão fixada**: **Opção 3**.

Lacuna #6 ficará marcada **fechada** em
`m1-lacunas-captura.md` quando os 3 itens seguintes forem todos
verdade simultaneamente:

1. `01_core/src/entities/bib_store.rs` existe e contém
   `BibStore { entries: Vec<BibEntry>, numbers: HashMap<String, u32> }`
   com método `add_bibliography(entries)` + `lookup_entry(key)` +
   `lookup_number(key)`.
2. `Introspector::bib_entry_for_key(key)` e
   `Introspector::bib_number_for_key(key)` existem no trait + impl
   `TagIntrospector`. `from_tags` arm Bibliography popula
   `bib_store`.
3. Layouter cite-arm (`mod.rs:584-597`) consulta via
   `self.introspector.bib_entry_for_key(...)` /
   `self.introspector.bib_number_for_key(...)` — sem leitura directa
   de `self.counter.bib_entries` ou `self.counter.bib_numbers`.

Os fields `bib_entries` e `bib_numbers` em `CounterStateLegacy`
permanecem após o fecho da lacuna #6; a sua remoção fica para **M6**
quando F1 (eliminar `CounterStateLegacy`) for retomado e todos os
call-sites tiverem migrado.

---

## §3 — Plano de sub-passos revisto

P180 §5 propôs 10 sub-passos `.A`–`.J`. Após decisões §2, P181A
absorve `.A` (validação + decisões); restam 9 sub-passos para P181B+.

| Sub-passo | Escopo revisto | Magnitude | Depende de |
|-----------|----------------|-----------|------------|
| ~~`.A`~~ | Absorvido por P181A (validação inventário P180 + decisões cláusula 1–6) | — | (este documento) |
| `.B` | Criar `01_core/src/entities/bib_store.rs`. Struct `BibStore { entries: Vec<BibEntry>, numbers: HashMap<String, u32> }` (cláusula 1). Métodos: `empty()`, `pub(crate) fn add_bibliography(&mut self, entries: &[BibEntry])` (cláusula 2 + 3), `lookup_entry(key) -> Option<&BibEntry>`, `lookup_number(key) -> Option<u32>`, `len`, `is_empty`. Tests unitários: default empty + add concat + or_insert preservation. Adicionar field `bib_store: BibStore` a `TagIntrospector`; método `pub fn bib_store(&self) -> &BibStore`. | **S** | — |
| `.C` | Adicionar `ElementKind::Bibliography` (`element_kind.rs:16`); `ElementPayload::Bibliography { entries: Vec<BibEntry> }` (`element_payload.rs`). Actualizar `as_str()` + `from_name()`. Tests: variant existe + `as_str == "bibliography"` + `from_name("bibliography")`. | **S** | `.B` |
| `.D` | `is_locatable(Content::Bibliography) == true` (`locatable.rs:85` move Bibliography do bloco non-locatable para locatable). `extract_payload` arm `Content::Bibliography { entries, .. } => Some(ElementPayload::Bibliography { entries: entries.clone() })`. Tests: `is_locatable(bib) == true` + `extract_payload(bib).is_some()` + invariante. | **S** | `.C` |
| `.E` | `from_tags.rs` arm `ElementPayload::Bibliography { entries }`: indexa em `kind_index[Bibliography]` e chama `bib_store.add_bibliography(entries)`. Tests: dois `Content::Bibliography` em sequência → `bib_store.entries.len() == n1+n2` + `bib_store.numbers` preserva primeiro número. | **S** | `.C`, `.B` |
| `.F` | `Introspector` trait (`introspector.rs`) ganha `fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry>` + `fn bib_number_for_key(&self, key: &str) -> Option<u32>`. `TagIntrospector` implementa delegando para `self.bib_store.lookup_*`. Tests trait-level. | **S** | `.B` |
| `.G` | Layouter cite-arm (`layout/mod.rs:584-597`) consulta via `self.introspector.bib_entry_for_key(key)` + `self.introspector.bib_number_for_key(key)` em vez de `self.counter.bib_entries.iter().find(...)`. Copy-sites em `pub fn layout` (1385-1388) e `pub fn layout_with_introspector` (1413-1416) **mantidos** (compat M6). Tests E2E: cite Normal/Prose/Author/Year contra Bibliography → output idêntico ao path legacy. | **M** | `.F` |
| `.H` | Walk arm `Content::Bibliography` (`introspect.rs:567-573`) deixa de mutar `state.bib_entries` / `state.bib_numbers`; passa apenas a descer no `title` quando presente. Tag é emitida automaticamente via mecanismo locatable (P164). Tests E2E confirmam paridade `introspect_with_introspector` produz `BibStore` idêntico ao state legacy preenchido pelo path legacy. | **S** | `.E`, `.G` |
| `.I` | Tests E2E adicionais: `query("bibliography")` retorna count correcto (paralelo a outline P178); `Introspector::bib_entry_for_key` em E2E; multi-Bibliography paridade. Lacuna #6 marcada **fechada** em `m1-lacunas-captura.md` (cumprir critério §2.6). | **S** | `.H` |
| `.J` | Relatório P181 sub-passo final: tabela diff (ficheiros tocados, +tests, +linhas), inventário 148 actualizado, README ADRs (sem ADR nova esperada). | **S** | `.I` |

**Total**: 9 sub-passos S + 1 M (`.G`). Estimativa **+15 a +25 tests**
mantida (cf. P180 §4).

**Ordem condicionada por decisão cláusula 4 (β)**: `.G` (Layouter
migra) **deve preceder** `.H` (walk puro) para evitar janela em que
Layouter lê legacy vazio. Esta ordem espelha o critério de fecho §2.6
(consumer migra antes de field legacy ficar redundante).

---

## §4 — Magnitude consolidada

P180 §4 declarou **S-M (+15-25 tests, ~10 sub-passos)**. Após
decisões §2:

- 9 sub-passos restantes (P181A absorveu `.A`).
- 8 são **S** + 1 é **M** (`.G` Layouter migration). Distribuição
  consistente com P171 (StateRegistry) e P177 (CounterRegistry
  hierarchical).
- Estimativa de tests inalterada (+15-25).

**Magnitude S-M re-confirmada**. Sem revisão.

Decisão cláusula 4 = β é a única substancial; mas é replicação do
padrão P162/P165/P169/P171/P177/P178 — sem novo padrão arquitectural.

---

## §5 — ADR avaliação

Decisões fixadas em §2 contra critérios para criação de ADR nova:

| Decisão | Padrão arquitectural | ADR existente cobre? |
|---------|----------------------|----------------------|
| Cláusula 1 (Vec + HashMap) | Replica `MetadataStore` (Vec) e `LabelRegistry` (HashMap) | Sim — não exige ADR. (`IndexMap` autorizado por ADR-0023 mas não escolhido.) |
| Cláusula 2 (concat) | Replica `state.bib_entries.extend` actual | Sem ADR necessária (preserva semântica documentada em P159C) |
| Cláusula 3 (or_insert) | Preserva semântica P159F | Sem ADR necessária |
| Cláusula 4 (Opção β) | Replica P162/P165/P169/P171/P177/P178 | Sem ADR necessária — invariante P163 já estabelecida |
| Cláusula 5 (trait Introspector + Layouter migra) | Replica P168 figure-ref | Sem ADR necessária |
| Cláusula 6 (Opção 3) | Replica fecho de lacuna #7 (P178 Outline) | Sem ADR necessária |

**Conclusão**: P181A **não cria ADR**. Todas as decisões são
consequência de invariantes/padrões já estabelecidos (P162→P178). ADR
nova exigiria padrão genuinamente novo — nenhum identificado.

Excepção que poderia exigir ADR: se cláusula 4 tivesse escolhido
Opção α (dual-state), introduzir-se-ia precedente para "feature em
modo transitório com walk impuro temporário". Não é o caso.

---

## §6 — DEBT avaliação

Trabalho identificado em P181A que possa exigir abertura de DEBT:

| Item | Avaliação |
|------|-----------|
| Eliminação de `bib_entries` / `bib_numbers` de `CounterStateLegacy` | Já coberto por **F1** (auditoria fresh) e **M6** roadmap. Não é DEBT novo. |
| Layouter copy-sites (1385-1388, 1413-1416) durante a janela compat | Mesma cobertura M6 — desaparecem quando fields legacy forem removidos. Não é DEBT novo. |
| Fecho de paridade "100% hayagriva" (`bib_entry.rs` ~70-75% cobertura) | Coberto por **DEBT-55** + **ADR-0062** (`hayagriva` adopt). Independente de P181. |

**Conclusão**: P181A **não abre DEBT novo**. Trabalho residual já
coberto por F1/M6/DEBT-55.

---

## §7 — Critério de fecho lacuna #6 fixado

Texto literal a colocar em `m1-lacunas-captura.md` linha 89 quando
P181 (sub-passo `.I`) cumprir os 3 itens enumerados em §2.6:

> ✅ **Resolvida em P181** (sub-store + locatable kind + cascade
> Layouter). `BibStore` populado via `from_tags` arm Bibliography;
> `Introspector::bib_entry_for_key` + `bib_number_for_key`
> consumidos pelo Layouter cite-arm. Fields `bib_entries` /
> `bib_numbers` em `CounterStateLegacy` permanecem (M6 elimina
> quando F1 for retomada).

**Verificável**: auditor seguinte abre os 3 ficheiros enumerados em
§2.6 (1, 2, 3) e confirma a presença literal de cada item. Sem
julgamento subjectivo.

---

## §8 — Próximo sub-passo (P181B)

**Escopo concreto P181B** (cf. §3 linha `.B`):

1. Redigir prompt L0 `00_nucleo/prompts/entities/bib_store.md` com
   spec literal: campos, métodos, invariantes (multi-Bib concat,
   or_insert preservation), tests obrigatórios.
2. Humano confirma hash do L0.
3. Materializar `01_core/src/entities/bib_store.rs` com:
   - `pub struct BibStore { entries: Vec<BibEntry>, numbers: HashMap<String, u32> }`
   - `pub fn empty() -> Self`
   - `pub(crate) fn add_bibliography(&mut self, entries: &[BibEntry])`
   - `pub fn lookup_entry(&self, key: &str) -> Option<&BibEntry>`
   - `pub fn lookup_number(&self, key: &str) -> Option<u32>`
   - `pub fn len(&self) -> usize` + `pub fn is_empty(&self) -> bool`
4. Adicionar field `bib_store: BibStore` a `TagIntrospector`
   (`introspector.rs`); método `pub fn bib_store(&self) -> &BibStore`.
5. Tests unitários co-localizados: default empty; add concat;
   or_insert preservation; lookup miss/hit.
6. `cargo build` + `cargo test --workspace --lib` verde.
7. `crystalline-lint .` zero violations.

**Magnitude P181B**: **S** (~150-200 linhas + ~5-7 tests).

**Sem dependências externas** (Vec/HashMap stdlib; `BibEntry` já
existe).
