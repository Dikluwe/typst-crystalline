# Relatório do passo P205B

**Data de execução**: 2026-05-07.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-205B.md`.
**Natureza**: implementação focada (sealing infrastructure).
**Sub-passo `B` da série P205** — segundo de 5 (A-E).
**Magnitude planeada**: S–M.
**Magnitude real**: **S** (~30 min; 1 ficheiro novo + 3
modificados; 4 tests novos; sem refactor mid-execution).

---

## §1 O que foi feito

P205B materializou a infraestrutura de sealing para
sub-store `positions` per ADR-0074:

- Tipo `SealedPositions` em L1 (`01_core/src/entities/sealed_positions.rs`).
- `#[comemo::track] impl SealedPositions { fn position_of(...) }`
  per Padrão A literal (paridade arquitectónica com M8).
- Sealing point em `Layouter::finish` —
  `self.runtime.positions` consumido para
  `SealedPositions::from_runtime(...)` e atribuído a
  `doc.extracted_positions`.
- Campo `pub extracted_positions: SealedPositions` em
  `PagedDocument` (Caminho B fixado em C2; precedente
  `extracted_label_pages` Passo 63).
- 4 tests novos: 2 sentinelas + 2 unit tests substantivos.

P205B **não migra consumers** de `position_of` — esse
trabalho é de P205C.

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-205B-inventario.md`.

Conteúdo:
- §1 C1 inventário (7 sub-secções; todas CONFIRMADO).
- §2 C2 forma de sealing fixada (Caminho B).
- §3 C3+C4 forma e localização (newtype com field
  privado; módulo dedicado).
- §4 C5 alterações literais nos 3 ficheiros tocados.
- §5 5 decisões durante a leitura (D1–D5).

Tamanho: ~9 KB.

### Output 2 — Relatório (este ficheiro)

### Output 3 — Alterações em código

#### Ficheiros novos

- **`00_nucleo/prompts/entities/sealed-positions.md`** (L0;
  hash `94c68ba8`).
- **`01_core/src/entities/sealed_positions.rs`** (L1;
  hash `89baeda9`; ~135 LOC com tests).

#### Ficheiros modificados

- **`01_core/src/entities/mod.rs`** — `pub mod
  sealed_positions;` adicionado entre `position` e `tag`.
- **`01_core/src/entities/layout_types.rs`** — campo
  `pub extracted_positions: SealedPositions` em
  `PagedDocument`; construtor `new` actualizado.
- **`01_core/src/rules/layout/mod.rs`** — `Layouter::finish`
  popula `doc.extracted_positions` via
  `SealedPositions::from_runtime(self.runtime.positions)`
  (~3 linhas).
- **`00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`**
  — §P205B anotada com `✅ MATERIALIZADO 2026-05-07` +
  sumário literal.

---

## §2 Tempo de execução

~30 minutos efectivos:

- ~5 min: leitura da spec + setup TaskList.
- ~10 min: C1 inventário empírico (7 sub-secções).
- ~5 min: C6 redacção do L0 prompt.
- ~5 min: C2-C5 escrita do código + tests.
- ~5 min: C8-C10 build + tests + lint + fix-hashes.
- ~5 min: C11 anotação ADR + outputs (inventário e
  este relatório).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace antes | 1852 |
| Tests workspace depois | 1856 (+4) |
| Tests sealed_positions | 4 (2 sentinelas + 2 unit) |
| Linter violations | 0 |
| Linter warnings | 0 |
| Ficheiros novos | 2 (1 L0 + 1 L1) |
| Ficheiros modificados | 4 (mod.rs, layout_types.rs, layout/mod.rs, ADR-0074) |
| LOC novas | ~135 (L1) + ~190 (L0) = ~325 |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |

Distribuição P205B:

- `typst_core` unit: 1576 → **1580** (+4 sealed_positions
  tests).
- Restantes crates: sem alteração.

---

## §4 Decisões

### D1 — Caminho B (field anexado em PagedDocument)

C1.2 confirmou precedente literal em
`extracted_label_pages` (Passo 63). Construtor `new`
inicializa com `Default::default()` — retrocompatível
com os ~10 call sites em testes que usam
`PagedDocument::new(vec![...])`. Caminho A (tuple)
seria mais invasivo sem ganho.

### D2 — Sem `Arc` interno

C1.4 confirmou que sub-stores cristalinos
(`bib_store`, `state_registry`, `metadata_store`) usam
HashMap directo sem Arc. Mantive o pattern. Clone O(n)
só acontece em re-tracking (raro); overhead aceitável.

### D3 — Newtype com field privado (não tuple)

`SealedPositions { positions: HashMap<...> }` em vez
de `SealedPositions(HashMap<...>)`. Nome do field é
explícito em construções literais e pattern matching
futuro.

### D4 — `from_runtime` consume por valor

`Layouter::finish` consume `self`; HashMap pode ser
moved sem clone. Sealing é zero-cost.

### D5 — 4 tests (acima do mínimo 2)

Spec C7 exigia mínimo 2 sentinelas. Acrescentei 2 unit
tests substantivos (empty + from_runtime preserva
mappings) que exercem o macro `#[comemo::track]` via
`.track().position_of(...)`. Cobertura mais densa
trivial.

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §3 e §8:

| Hipótese | Resultado |
|----------|-----------|
| `Layouter::finish` pode não existir como método nomeado | **REFUTADA** — existe em `mod.rs:1167`, assinatura simples |
| `PagedDocument` pode ter muitos consumers externos (Caminho A mais seguro) | **REFUTADA parcialmente** — só ~10 call sites em testes; Caminho B retrocompatível com Default |
| `Arc<HashMap>` pode não satisfazer `Hash`/`Send`/`Sync` | **NÃO MATERIALIZOU** — decidi não usar Arc per padrão dos sub-stores |
| Tentação trait dedicada `PositionStore` por simetria | **EVITADA** — struct concreta basta (única impl); per spec §8 risco específico |
| Sealing global pós-fixpoint vs por iteração | **CONFIRMADO por iteração** — `Layouter::finish` é per-iteração; sealing acontece naturalmente onde ADR-0074 fixou |
| `Hash`/`Send`/`Sync` falham em compilação | **NÃO MATERIALIZOU** — Position e Location já satisfazem; HashMap herda |

5 de 6 hipóteses refutadas/não materializadas pela
auditoria empírica (C1) — implementação foi directa sem
surpresas.

---

## §6 Sugestão para próximo passo

P205B fechado per C12 com todos os critérios cumpridos:

- ✓ C1 inventário completo (7 sub-secções; todas
  CONFIRMADO).
- ✓ C2 forma de sealing fixada (Caminho B).
- ✓ C3+C4 definição literal + localização.
- ✓ C5 sealing aplicado em `Layouter::finish`.
- ✓ C6 L0 prompt criado (`entities/sealed-positions.md`).
- ✓ C7 sentinelas (2 + 2 unit tests = 4).
- ✓ C8 compilação verde.
- ✓ C9 tests workspace verdes (1856).
- ✓ C10 linter 0 violations.
- ✓ C11 ADR-0074 anotada.
- ✓ Inventário registado.
- ✓ Relatório escrito.

**Próximo sub-passo**: **P205C — `position_of` impl real
+ consumer migration** (per ADR-0074 plano de
materialização):

- `TagIntrospector` (ou novo wrapper) consome
  `SealedPositions` para impl `position_of` retornando
  `Some(Position)` real (em vez de `None` per ADR-0073
  §C6a).
- Consumers do dual path migrate para
  `Introspector::position_of` exclusivamente.
- Tests E2E (consumer recebe Position correcta).

Magnitude estimada: S–M (~1-2h).

---

## §7 Cross-references

- **Spec**:
  `00_nucleo/materialization/typst-passo-205B.md`.
- **Outputs**:
  - `00_nucleo/diagnosticos/typst-passo-205B-inventario.md`.
  - `00_nucleo/prompts/entities/sealed-positions.md`
    (hash `94c68ba8`).
  - `01_core/src/entities/sealed_positions.rs` (hash
    `89baeda9`).
- **ADR**:
  `00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`
  (§P205B ✅ MATERIALIZADO 2026-05-07).
- **Predecessor**: P205A (diagnóstico-primeiro de F3).
- **Sucessor planeado**: P205C (consumer migration).
- **Pattern referência**: `01_core/src/entities/bib_store.rs`
  (pattern de sub-store cristalino).
- **Pattern `#[comemo::track]`**:
  `01_core/src/entities/introspector.rs:40` (M8 P204B).
- **Vanilla referência**:
  `lab/typst-original/crates/typst-layout/src/introspect.rs:60-63`
  (`PagedIntrospector::position` — não paridade literal).
