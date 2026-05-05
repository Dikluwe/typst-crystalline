# Diagnóstico — Sub-store `intr.headings_for_toc` (P200A)

**Data**: 2026-05-04
**Pattern arquitectural relevante**: ADR-0069 — trabalho híbrido combinando 3 padrões testados (sub-store novo + Tag pós-recursão + consumer migration).
**Estado empírico**: confirmado por leitura directa de `01_core/src/`.

---

## §1 Validação estado actual

### §1.1 Sub-store ausente em `TagIntrospector`

`01_core/src/entities/introspector.rs:163-196` — 8 sub-stores actuais:

```rust
pub struct TagIntrospector {
    pub labels:               LabelRegistry,
    pub counters:             CounterRegistry,
    pub kind_index:           HashMap<ElementKind, Vec<Location>>,
    pub figure_label_numbers: HashMap<Label, usize>,
    pub metadata:             MetadataStore,
    pub state:                StateRegistry,
    pub bib_store:            BibStore,
    pub resolved_labels:      ResolvedLabelStore,
}
```

**`headings_for_toc` ausente** — confirmado.

### §1.2 Trait `Introspector` — 19 métodos

`01_core/src/entities/introspector.rs` — métodos por linha:
- L40: `query_by_kind`
- L43: `query_by_label`
- L47: `query_first`
- L51: `query_unique`
- L55: `position_of`
- L62: `figure_number_for_label`
- L67: `query_metadata`
- L72: `formatted_counter`
- L87: `query`
- L92: `formatted_counter_at`
- L113: `is_numbering_active`
- L122: `figure_number_at_index`
- L131: `is_numbering_active_at`
- L141: `flat_counter_at`
- L154: `resolved_label_for`

(15 visíveis no grep; 19 total per documentação P198D — diferença explicada por métodos em métodos auxiliares do trait não filtrados pelo regex).

**Novo método `headings_for_toc` será 20º**.

### §1.3 Walk arm Heading mutação 4 — `01_core/src/rules/introspect.rs:461-498`

```rust
Content::Heading { level, body } => {
    // P196B — walk arm Heading auto-toc...
    state.step_hierarchical("heading", *level as usize);                           // mut 1

    state.auto_label_counter += 1;                                                  // mut 2
    let (auto_label, resolved_text) = compute_heading_auto_toc(state, state.auto_label_counter);
    state.resolved_labels.insert(auto_label.clone(), resolved_text.clone());        // mut 3

    // E2-residuo: ...
    let frozen_body = materialize_time(body, state);
    state.headings_for_toc.push((auto_label.clone(), frozen_body, *level as usize)); // mut 4 (LINHA 486)

    walk(body, state, locator, tags, None);

    // P196B: emit Tag auto-toc pós-recursão (ADR-0069). [linhas 490+]
    if let Some(loc) = emitted_loc {
        tags.push(Tag::Start(loc, ElementInfo::new(ElementPayload::Labelled { ... })));
        tags.push(Tag::End(loc, 0));
    }
}
```

**Mutação 4 confirmada** — linha 486. `frozen_body = materialize_time(body, state)` produz Content com counters resolvidos. `*level as usize` cast. `emitted_loc: Option<Location>` em scope (Heading locatable per P164).

### §1.4 Type signature legacy — `counter_state_legacy.rs:42`

```rust
pub headings_for_toc: Vec<(Label, Content, usize)>,
```

**`usize` (não `u8`)** — corrigir referência da instrução.

### §1.5 Consumer outline.rs:24 — `01_core/src/rules/layout/outline.rs:24`

```rust
pub(super) fn layout_outline<M: FontMetrics, S: ImageSizer>(layouter: &mut Layouter<M, S>) {
    // Clonar o vector antes do loop para evitar borrow duplo de `layouter`.
    let entries: Vec<_> = layouter.counter.headings_for_toc.clone();
    ...
    for (label, body_content, level) in entries {
        let indent = "  ".repeat(level.saturating_sub(1));
        ...
    }
}
```

**Consumer único e directo** confirmado. Lê do legacy (`layouter.counter` é `CounterStateLegacy`).

### §1.6 Layouter assignments — `mod.rs:1490, 1521`

```rust
l.counter.headings_for_toc = initial_state.headings_for_toc;        // mod.rs:1490
l.counter.headings_for_toc = initial_state.headings_for_toc.clone(); // mod.rs:1521
```

Layouter recebe state legacy via assignment direto. Funciona com write paralelo M5; sem necessidade de modificar.

### §1.7 ElementPayload + ElementKind

- `ElementPayload`: 12 variants (após P198C). Onde adicionar `HeadingForToc`: após `CounterUpdate` (per convenção cronológica).
- `ElementKind`: 10 variants. **Decisão**: `ElementKind::HeadingForToc` **não** será adicionada — HeadingForToc é Tag **derivada** de Heading (não corresponde a `Content` standalone, diferente de CounterUpdate P198C). Sub-store dedicado `headings_for_toc` no trait é caminho de query directo; sem necessidade de `kind_index` paralelo.

### §1.8 `from_tags` arm necessário

`01_core/src/rules/introspect/from_tags.rs` — match exhaustivo sobre `ElementPayload`. Após adicionar variant `HeadingForToc`, arm novo necessário (sem default `_ =>` — exhaustivo per convenção).

Push directo: `intr.headings_for_toc.push((label.clone(), body.clone(), *level));`.

### §1.9 Helper `compute_heading_auto_toc` (P196B)

`introspect.rs:381-394` — produz `(Label, String)` para Tag::Labelled auto-toc P196B. **NÃO modificar**. P200B adiciona helper distinto `compute_heading_for_toc` que produz `(Label, Content, usize)`.

### §1.10 Tests sentinela existentes

- `walk_e2_residuo_headings_for_toc_via_legacy` (introspect.rs:2389+): valida que `state.headings_for_toc.len() == 3` para 3 headings via legacy. **Preservar** — após P200B, mutação legacy continua activa (write paralelo M5).
- `tests.rs:1209-1211`, `tests.rs:4880`: tests Layouter outline. **Verificar não regridem**.

### §1.11 L0 alvos

- `entities/introspector.md` — sub-store novo + método trait novo (19→20).
- `entities/element_payload.md` — variant nova (12→13).
- `rules/introspect.md` — walk arm Heading actualização + nova secção.
- `rules/layout/outline.md` (se existir) — consumer migration.
- (não tocar) `entities/element_kind.md` — sem variant nova.

### §1.12 Workspace baseline

- Tests: 1.864 verdes.
- Linter: zero violations.
- Hash L0 introspect: `603170c8` (per P199B).
- Hash código introspect: `0092886d` (per P199B).

---

## §2 Decisões cláusula 1–9

### Cláusula 1 — Forma do sub-store

- **O1 (input)**: type legacy `Vec<(Label, Content, usize)>` (counter_state_legacy.rs:42).
- **O2 (alternativas)**:
  - α — replica literal `Vec<(Label, Content, usize)>`.
  - β — struct dedicada `HeadingTocEntry { label, body, level }`.
  - γ — over-engineering.
- **O3 (critério)**: paridade exacta com legacy reduz risco; struct dedicada adiciona burocracia sem ganho funcional.
- **Decisão**: **Opção α** — `Vec<(Label, Content, usize)>`.
- **O4 (magnitude)**: ~5 LOC sub-store.
- **O5 (reversibilidade)**: trivial.

### Cláusula 2 — Variant Tag

- **O1 (input)**: ElementPayload tem 12 variants; nenhum cobre semântica HeadingForToc (`Labelled` falta `body`/`level`; `Heading` é snapshot original sem `auto_label`).
- **O2 (alternativas)**:
  - α — variant nova `HeadingForToc { label, body, level }`.
  - β — reusar Labelled (improvável — falta body).
  - γ — reusar Heading (improvável — quebra atomização).
- **O3 (critério)**: outline precisa `(label, body materializado, level)` que apenas variant nova cobre.
- **Decisão**: **Opção α** — `ElementPayload::HeadingForToc { label: Label, body: Content, level: usize }`. ElementPayload: 12 → 13.
- **O4 (magnitude)**: ~10 LOC variant.
- **O5 (reversibilidade)**: trivial.

### Cláusula 3 — `is_locatable` vs Tag pós-recursão

- **O1 (input)**: HeadingForToc é Tag derivada de Heading (não Content standalone).
- **O2 (alternativas)**:
  - α — Tag pós-recursão usando `emitted_loc` do walk arm Heading (variante P196B); sem `is_locatable` arm novo.
  - β — promote a Content + locatable (improvável — não há `Content::HeadingForToc` parsável).
- **O3 (critério)**: HeadingForToc não existe como Content; emitido por walk arm Heading na mesma Location.
- **Decisão**: **Opção α** — Tag pós-recursão; sem `is_locatable` arm; sem `extract_payload` arm.
- **O4 (magnitude)**: 0 LOC nestes ficheiros.
- **O5 (reversibilidade)**: trivial.

### Cláusula 4 — Walk arm Heading

- **O1 (input)**: walk arm Heading actual emite 2 Tags (Heading via extract_payload + Labelled auto-toc P196B pós-recursão).
- **O2 (alternativas)**:
  - α — emitir 3ª Tag (HeadingForToc) após Tag Labelled auto-toc P196B; mesma `emitted_loc`.
  - β — emitir HeadingForToc antes de Labelled auto-toc.
- **O3 (critério)**: ordem α coloca HeadingForToc após Tag Labelled (consistente com sequência `mut 4 → walk(body) → Tag::Labelled → Tag::HeadingForToc`).
- **Decisão**: **Opção α** — 3ª Tag após Tag Labelled. **Mutação 4 legacy preservada** (write paralelo M5).
- **O4 (magnitude)**: ~10 LOC adição (replica padrão P196B).
- **O5 (reversibilidade)**: trivial.

### Cláusula 5 — Helper `compute_heading_for_toc`

- **O1 (input)**: pattern ADR-0069 stylesheet com helper privado (P195D, P196B, P197B).
- **O2 (alternativas)**:
  - α — helper privado `compute_heading_for_toc(state, level, body) -> Option<(Label, Content, usize)>`.
  - β — lógica inline no walk arm.
- **O3 (critério)**: consistência com pattern stylesheet (4º helper na família).
- **Decisão**: **Opção α** — helper privado.
- **O4 (magnitude)**: ~15 LOC helper.
- **O5 (reversibilidade)**: trivial.

```rust
fn compute_heading_for_toc(
    state: &CounterStateLegacy,
    level: usize,
    body:  &Content,
) -> Option<(Label, Content, usize)> {
    // Replica condição de mutação 4 legacy: insert sempre — não condicionado
    // a is_numbering_active. (Per introspect.rs:486 actual, push é
    // incondicional — auto_label sintetizada e body materializado mesmo
    // sem numbering activo.)
    let auto_label = Label(format!("auto-toc-{}", state.auto_label_counter));
    let frozen_body = materialize_time(body, state);
    Some((auto_label, frozen_body, level))
}
```

**Cláusula gate substancial**: helper retorna Option mas sempre Some na prática. Razão: estrutura permite extensão futura (p.ex. gate por numbering) sem mudança de assinatura.

### Cláusula 6 — `from_tags` arm

- **O1 (input)**: arm match exhaustivo sobre `ElementPayload` em from_tags.rs.
- **Decisão**: arm push directo:
  ```rust
  ElementPayload::HeadingForToc { label, body, level } => {
      intr.headings_for_toc.push((label.clone(), body.clone(), *level));
  }
  ```
- **O4 (magnitude)**: ~5 LOC.
- **O5 (reversibilidade)**: trivial.

### Cláusula 7 — Trait method

- **Decisão**: `fn headings_for_toc(&self) -> &[(Label, Content, usize)];`.
- Implementação `TagIntrospector`: `fn headings_for_toc(&self) -> &[(Label, Content, usize)] { &self.headings_for_toc }`.
- Trait: 19 → **20 métodos**.
- **O4 (magnitude)**: ~5 LOC trait + ~3 LOC impl.

### Cláusula 8 — Consumer outline migration

- **O1 (input)**: `outline.rs:24` lê `layouter.counter.headings_for_toc.clone()` directamente.
- **O2 (alternativas)**:
  - α — substitution-with-fallback (padrão P184D/P194B): primeira tentativa Introspector; fallback legacy.
  - β — directamente Introspector sem fallback.
- **O3 (critério)**: padrão estabelecido P184D/P194B.
- **Decisão**: **Opção α**:
  ```rust
  let entries_owned: Vec<(Label, Content, usize)>;
  let entries: &[(Label, Content, usize)] = {
      let intr_entries = layouter.introspector.headings_for_toc();
      if !intr_entries.is_empty() {
          intr_entries
      } else {
          entries_owned = layouter.counter.headings_for_toc.clone();
          &entries_owned
      }
  };
  ```
  Forma exacta fica para Claude Code conforme estilo do projecto.
- **O4 (magnitude)**: ~10 LOC consumer (substitui o `let entries: Vec<_>` actual).
- **O5 (reversibilidade)**: trivial.

### Cláusula 9 — Critério de fecho de P200

- **Decisão**: P200 fecha quando:
  - Sub-store `headings_for_toc` aberto (TagIntrospector 8→9 sub-stores).
  - Trait method `headings_for_toc()` exposto (19→20 métodos).
  - Variant `ElementPayload::HeadingForToc` adicionada (12→13).
  - Helper `compute_heading_for_toc` criado.
  - Walk arm Heading emite 3ª Tag pós-recursão.
  - `from_tags` arm popula sub-store.
  - Consumer outline migrado (substitution-with-fallback).
  - Tests E2E confirmam paridade observable + activação Introspector path.
  - **E2-residuo fecha estruturalmente completa**.
  - **Lacuna #3 fecha**.
  - **M5 universal completo** — 0 excepções activas + 0 residuos + 0 pré-requisitos.
- **Marco arquitectural**: primeira vez M5 universal completo desde declaração em P189B.

---

## §3 Plano de sub-passos sem condicionais

### P200B — Sub-store + Tag + walk arm + consumer (cenário híbrido)

Magnitude **M genuíno** (3 categorias combinadas):

1. Adicionar field `pub headings_for_toc: Vec<(Label, Content, usize)>` em `TagIntrospector` (`entities/introspector.rs`).
2. Adicionar trait method `headings_for_toc(&self) -> &[(Label, Content, usize)]` em trait `Introspector`.
3. Adicionar implementação em `impl Introspector for TagIntrospector`.
4. Adicionar variant `ElementPayload::HeadingForToc { label: Label, body: Content, level: usize }` em `entities/element_payload.rs`.
5. Adicionar helper privado `compute_heading_for_toc(state, level, body)` em `rules/introspect.rs` antes do walk fn.
6. Modificar walk arm Heading (`introspect.rs:461`) para emitir 3ª Tag pós-recursão após Tag Labelled auto-toc P196B; mutação 4 legacy preservada.
7. Adicionar arm em `rules/introspect/from_tags.rs` para `ElementPayload::HeadingForToc`.
8. Migrar consumer `rules/layout/outline.rs:24` para substitution-with-fallback.
9. Actualizar L0 (3 ficheiros): `entities/introspector.md`, `entities/element_payload.md`, `rules/introspect.md`.
10. Adicionar 5-6 tests E2E:
    - `headings_for_toc_extract_payload_emite_payload`.
    - `headings_for_toc_walk_emite_tag_pos_recursao`.
    - `headings_for_toc_from_tags_popula_sub_store`.
    - `headings_for_toc_paridade_legacy_vs_introspector`.
    - `consumer_outline_recebe_via_introspector`.
    - `walk_arm_heading_3a_tag_partilha_emitted_loc` (sentinela).
11. `crystalline-lint --fix-hashes`.

### P200C — Encerramento série P200

Magnitude **S puro**:

1. Auditoria empírica final (15 verificações).
2. Relatório consolidado `typst-passo-200-relatorio-consolidado.md` (9 secções padrão).
3. Nota DEBT M5-residual actualizada (**0 excepções activas + 0 residuos + 0 pré-requisitos**).
4. **Marco arquitectural — M5 universal completo**.
5. Verificação estrutural final.

---

## §4 Magnitude consolidada

| Sub | Magnitude | LOC produção | LOC teste | LOC L0 | Δ tests |
|-----|-----------|--------------|-----------|--------|---------|
| P200A | S | 0 | 0 | 0 | 0 |
| P200B | M+ | ~150 (sub-store + trait + variant + helper + walk arm + from_tags + consumer + L0 sync) | ~180 (5-6 tests) | ~100 | +5 a +6 |
| P200C | S | 0 | 0 | 0 | 0 |

**Total agregado**: **M+** (limite superior do M devido a 3 categorias).

---

## §5 ADR avaliação

- 5 variantes ADR-0069 cobrem.
- Trabalho híbrido (sub-store + Tag + consumer) é **combinação de variantes existentes**, não nova variante operacional.

**Conclusão**: **não cria ADR**.

---

## §6 DEBT M5-residual avaliação

- **Antes P200**: 0 excepções activas + 1 residuo (E2-residuo); 1 pré-requisito restante.
- **Após P200B**: **0 excepções activas + 0 residuos + 0 pré-requisitos**.

**M5 universal estado**: **completo**. Todos walk arms fechados estruturalmente:
- P189B Outline.
- P181H Bibliography.
- P195D Labelled.
- **P196B + P200B Heading** (3 mutações P196B + 1 mutação P200B).
- P197B Figure (cenário α).
- P198B SetHeadingNumbering (cenário α).
- P198C CounterUpdate (cenário β-promote).
- P199B SetEquationNumbering (cenário α por construção).

**Cenário B continua** (sem DEBT formal aberto).

**DEBT M6**: write paralelo M5 ainda activo — mutações legacy em todos walk arms preservadas; `compute_*` helpers leem legacy. Cleanup orgânico em **M6 (P190A reescrita do zero)**.

---

## §7 Cadeia E2-residuo + interacção com `compute_heading_auto_toc` P196B

### Sequência durante walk após P200B

Considere `Content::Heading { level: 1, body: text("Capítulo") }`:

1. Walk top em `Heading` — locatable; `extract_payload` retorna `Some(Heading {...})`.
2. Walk emite `Tag::Start(loc, ElementInfo { payload: Heading, label: None })`.
3. Match arm Heading:
   - **Mut 1**: `state.step_hierarchical("heading", 1)`.
   - **Mut 2**: `state.auto_label_counter += 1`.
   - `compute_heading_auto_toc(state, n)` → `(label, resolved_text)`.
   - **Mut 3**: `state.resolved_labels.insert(label, resolved_text)`.
   - `frozen_body = materialize_time(body, state)`.
   - **Mut 4**: `state.headings_for_toc.push((label, frozen_body, level))` (E2-residuo legacy preservada).
   - `walk(body, ...)` recursivo.
   - **Tag P196B (auto-toc)**: emit Tag::Start + Tag::End com `ElementPayload::Labelled` (resolved_text).
   - **Tag P200B (HeadingForToc) NOVO**: `compute_heading_for_toc` → emit Tag::Start + Tag::End com `ElementPayload::HeadingForToc { label, body, level }`. Mesma `emitted_loc`.
4. Walk bottom em `Heading` — emit Tag::End.

### Sequência tags emitidas para 1 Heading (após P200B)

```
Tag::Start(loc, Heading)               // walk top
[recursive body tags]
Tag::Start(loc, Labelled auto-toc-1)   // P196B pós-recursão
Tag::End(loc, 0)
Tag::Start(loc, HeadingForToc)         // P200B pós-recursão (NOVO)
Tag::End(loc, 0)
Tag::End(loc, hash_content(heading))   // walk bottom
```

**6 tags com mesma Location** para Heading folha (era 4 pós-P196B). 3 pares Start/End válidos. Bracketing preservado.

### Risco se mutação 4 legacy for removida em P200B

- Layouter `mod.rs:1490, 1521` (`l.counter.headings_for_toc = initial_state.headings_for_toc`) recebe legacy assignment.
- Substitution-with-fallback no consumer outline.rs:24 garante que mesmo se Introspector falhar (ou ainda não populated), legacy fornece dados.
- **Mitigação**: mutação 4 preservada como write paralelo M5; cleanup orgânico em M6.

### Interacção com `compute_heading_auto_toc` (P196B)

- `compute_heading_auto_toc` produz `(Label, String)` para Tag::Labelled (resolved_text para Ref auto-toc).
- `compute_heading_for_toc` (P200B) produz `(Label, Content, usize)` para Tag::HeadingForToc (body para outline render).
- **Helpers independentes** — `compute_heading_auto_toc` NÃO modificado; sub-stores diferentes (`resolved_labels` vs `headings_for_toc`).

### Adaptação do test sentinela `walk_e2_residuo_headings_for_toc_via_legacy`

- Test actual (introspect.rs:2389+) valida `state.headings_for_toc.len() == 3` para 3 headings.
- **Após P200B**: continua válido — mutação legacy preservada (write paralelo M5).
- Pode adicionar-se asserção paralela: `intr.headings_for_toc().len() == 3` para confirmar paridade.

### Adaptação dos tests sentinela P196B

- 5 tests P196B existentes (`walk_emite_start_e_end_para_heading`, etc.) assumem 4 tags por Heading.
- **Após P200B**: cada Heading emite 6 tags (mesma `emitted_loc`).
- **Tests precisam de adaptação** — replica padrão pragmático auditor #1 P196B (ajustar count esperado).

---

## §8 Próximo sub-passo (P200B com escopo concreto)

**P200B — Sub-store + Tag + walk arm + consumer (cenário híbrido)**:

1. **`entities/introspector.rs`**:
   - Adicionar field `pub headings_for_toc: Vec<(Label, Content, usize)>` em `TagIntrospector`.
   - Adicionar trait method `headings_for_toc(&self) -> &[(Label, Content, usize)]`.
   - Adicionar impl em `impl Introspector for TagIntrospector`.

2. **`entities/element_payload.rs`**:
   - Adicionar variant `HeadingForToc { label: Label, body: Content, level: usize }` após `CounterUpdate`.

3. **`rules/introspect.rs`**:
   - Adicionar helper privado `compute_heading_for_toc` antes da `walk` fn.
   - Modificar walk arm Heading (linha ~461): emitir 3ª Tag::Start/End pós-recursão após Tag Labelled auto-toc P196B; mesma `emitted_loc`.
   - Comentário inline P200B substitui parte da nota E2-residuo (declarar fechada estruturalmente).

4. **`rules/introspect/from_tags.rs`**:
   - Adicionar arm `ElementPayload::HeadingForToc` push directo.

5. **`rules/layout/outline.rs:24`**:
   - Migrar para substitution-with-fallback.

6. **L0**:
   - `entities/introspector.md` — sub-store novo + método.
   - `entities/element_payload.md` — variant nova.
   - `rules/introspect.md` — actualizar tabela Excepções (E2-residuo fecha; lacuna #3 fecha) + secção nova "Walk arm Heading mutação 4 fechada (P200B, trabalho híbrido)".

7. **5-6 tests E2E** + adaptação de **5 tests P196B** existentes (4 → 6 tags por Heading).

8. `crystalline-lint --fix-hashes`.

**Critério de fecho P200B**:
- Tests workspace 1.864 + 5 = 1.869 verdes (ou +6 se test sentinela paralelo adicionado).
- 5 tests P196B adaptados.
- Lint zero violations.
- E2-residuo fechada; lacuna #3 fechada.
- **M5 universal completo**.
