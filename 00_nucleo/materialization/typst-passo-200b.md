# Passo P200B — Sub-store `headings_for_toc` + Tag + consumer

Primeiro passo de implementação P200 (após P200A
diagnóstico). Magnitude **M+ genuína** — trabalho
híbrido combinando 3 padrões testados (sub-store novo
+ Tag pós-recursão + consumer migration).

P200A confirmou empiricamente:
- Sub-store ausente em `TagIntrospector`.
- Type legacy: `Vec<(Label, Content, usize)>` (corrigido
  em P200A §3 — não `u8`).
- Walk arm Heading mutação 4: `introspect.rs:486`.
- Consumer único: `outline.rs:24`.
- **Layouter assignments** descobertos: `mod.rs:1490,
  1521` fazem `l.counter.headings_for_toc =
  initial_state.headings_for_toc` — write paralelo M5
  obrigatório preservar.
- 5 tests P196B precisam adaptação (4 → 6 tags por
  Heading folha).

P200B é trabalho **híbrido** — sem nova variante
operacional ADR-0069. Combinação de:
- **Categoria A**: sub-store novo (replica P193B).
- **Categoria B**: variant Tag pós-recursão locatable
  (variante P196B).
- **Categoria C**: consumer migration
  substitution-with-fallback (P184D / P194B).

Trabalho concreto:
1. Adicionar field `headings_for_toc: Vec<(Label,
   Content, usize)>` em `TagIntrospector`.
2. Adicionar trait method
   `headings_for_toc(&self) -> &[(Label, Content,
   usize)]` (trait 19 → 20 métodos).
3. Adicionar variant `ElementPayload::HeadingForToc {
   label, body, level }` (12 → 13).
4. Decidir empiricamente sobre `ElementKind::HeadingForToc`
   (deferido ao auditor per `.A.6`).
5. Adicionar helper privado `compute_heading_for_toc`
   análogo a `compute_heading_auto_toc` P196B.
6. Modificar walk arm Heading: emitir 3ª Tag
   pós-recursão (após Tag Labelled auto-toc P196B;
   mesma `emitted_loc`) com mutação 4 legacy
   preservada como write paralelo M5.
7. Adicionar arm em `from_tags.rs` que faz push
   directo em `intr.headings_for_toc`.
8. Migrar consumer `outline.rs:24` para
   substitution-with-fallback.
9. **Não modificar** Layouter assignments
   (`mod.rs:1490, 1521`) — write paralelo M5.
10. **Não modificar** `compute_heading_auto_toc`
    (P196B) — sub-stores diferentes.
11. Comentário inline P200B substitui parte da nota
    E2-residuo no walk arm.
12. L0 actualizado: tabela Excepções (E2-residuo
    fecha; lacuna #3 fecha); secção nova "Walk arm
    Heading mutação 4 migrada (P200B, trabalho
    híbrido)".
13. 5-6 tests E2E + adaptação 5 tests P196B
    existentes (4 → 6 tags).

Após P200B:
- E2-residuo declarada formalmente fechada
  estruturalmente.
- Lacuna #3 declarada formalmente fechada.
- **M5 universal completo** — 0 excepções activas + 0
  residuos + 0 pré-requisitos restantes.
- Trait `Introspector`: 19 → 20 métodos.
- `ElementPayload`: 12 → 13 variants.
- `TagIntrospector`: 8 → 9 sub-stores.
- Helpers privados família ADR-0069: 3 → 4
  (`compute_heading_for_toc` adicionado).
- 7 aplicações ADR-0069 stylesheet (P195D + P196B +
  P197B + P198B + P198C + P199B + **P200B**).
- Mutações legacy preservadas — `compute_*` helpers
  + Layouter assignments continuam funcionais.

**Pré-condição**: P200A concluído. Tests workspace
1.864 verdes; zero violations. 9 cláusulas P200A
fechadas. Trabalho híbrido identificado.

**Restrições**:
- **Não** materializar parser sintáctico.
- **Manter mutação 4 legacy** —
  `state.headings_for_toc.push` necessária durante
  janela compat M5 (Layouter assignments
  `mod.rs:1490, 1521` dependem).
- **Não** modificar Layouter assignments
  (`mod.rs:1490, 1521`).
- **Não** modificar `compute_heading_auto_toc`
  (P196B) — sub-stores diferentes (`resolved_labels`
  vs `headings_for_toc`).
- **Não** modificar walk arm SetHeadingNumbering
  (P198B), CounterUpdate (P198C),
  SetEquationNumbering (P199B), Figure (P197B),
  Labelled (P195D), Outline (P189B).
- **Não** materializar P190A — aguarda M5 universal
  fechar empiricamente.
- API pública preservada (adições retrocompatíveis).

---

## Sub-passos

### .A Auditoria L0

#### Sub-store + trait

1. Confirmar `TagIntrospector` em
   `01_core/src/entities/tag_introspector.rs`:
   - 8 sub-stores actuais (per P198D).
   - Linha exacta para inserir `headings_for_toc:
     Vec<(Label, Content, usize)>`.

2. Confirmar trait `Introspector` em
   `01_core/src/entities/introspector.rs`:
   - 19 métodos.
   - Linha exacta para inserir `headings_for_toc`.

#### Walk arm Heading

3. Confirmar walk arm Heading em
   `01_core/src/rules/introspect.rs:486` (per P200A
   §3):
   - Mutação 4 legacy localizada.
   - Comentário inline P196B sobre E2-residuo
     presente (linhas 461-484).
   - Variável `emitted_loc` em scope.
   - Identificar Tag::Labelled auto-toc P196B emit
     (após walk recursivo do body).
   - **3ª Tag emit P200B** vai depois de Tag::Labelled
     auto-toc.

4. Confirmar `compute_heading_auto_toc` (P196B):
   - Helper produz `(Label, String)`.
   - **NÃO modificar** — distinto do P200B helper.

#### Variant + ElementKind

5. Confirmar `ElementPayload` em
   `01_core/src/entities/element_payload.rs`:
   - 12 variants (após P198C).
   - Linha exacta para inserir `HeadingForToc { label,
     body, level }` após `CounterUpdate`.

6. Confirmar `ElementKind` em
   `01_core/src/entities/element_kind.rs`:
   - 10 variants.
   - **Decisão obrigatória**: HeadingForToc tem
     ElementKind correspondente?
   - Per convenção P198C: todo ElementPayload
     locatable tem ElementKind. Mas HeadingForToc é
     Tag derivada de Heading — caso de fronteira.
   - **Auditor decide empiricamente**:
     - Se sim: `ElementKind::HeadingForToc` (10 →
       11); `kind_index[HeadingForToc]` populated
       em `from_tags`.
     - Se não: justificar excepção em L0
       `entities/element_kind.md`.

7. Confirmar `from_tags` em
   `01_core/src/rules/introspect/from_tags.rs`:
   - Match exhaustivo per P186B descoberta.
   - Linha exacta para inserir arm `HeadingForToc`.

#### Consumer outline

8. Confirmar consumer outline em
   `01_core/src/rules/layout/outline.rs:24`:
   - Per P200A §3:
     `let entries: Vec<_> = layouter.counter.headings_for_toc.clone();`
   - Onde inserir substitution-with-fallback.

9. Confirmar Layouter assignments (`mod.rs:1490,
   1521`):
   - **NÃO modificar** — write paralelo M5.
   - Apenas verificar que continuam funcionais após
     P200B.

#### Tests existentes

10. Confirmar 5 tests P196B que precisam adaptação
    (per P200A §12 ponto 7):
    - Identificar tests com asserções "4 tags por
      Heading" (após P196B).
    - Após P200B: **6 tags** (Heading + Labelled
      auto-toc + HeadingForToc; Start+End cada).
    - Forma exacta: 3 Tag::Start + 3 Tag::End.

11. Confirmar sentinela E2-residuo P196B
    (`walk_e2_residuo_headings_for_toc_via_legacy`):
    - Test confirma mutação 4 legacy continua
      activa.
    - **Após P200B**: mutação 4 continua preservada
      (write paralelo M5). Sentinela continua
      válida; pode reforçar-se com asserção
      adicional sobre paridade legacy vs Introspector.
    - **Decisão**: preservar test; adicionar
      asserção paridade.

#### L0 alvos

12. Confirmar L0 alvos:
    - `entities/tag_introspector.md` — sub-store
      novo.
    - `entities/introspector.md` — método novo.
    - `entities/element_payload.md` — variant nova.
    - (eventualmente) `entities/element_kind.md` —
      variant nova ou justificação excepção.
    - `rules/introspect.md` — tabela Excepções M5;
      ordem inversa; secção nova.

Output: tabela com item + estado verificado.

**Critério de saída**:
- Sub-store, walk arm, variant, consumer, tests
  identificados empiricamente.
- Convenção `ElementKind::HeadingForToc` decidida.
- Linhas exactas para edits identificadas.

### .B Adicionar field `headings_for_toc` em `TagIntrospector`

1. Em `01_core/src/entities/tag_introspector.rs`:
   - Adicionar field após sub-store mais recente
     (provavelmente `resolved_labels` P193B):
     ```
     pub headings_for_toc: Vec<(Label, Content, usize)>,
     ```
   - Inicializar em `new()` ou `Default`:
     ```
     headings_for_toc: Vec::new(),
     ```

2. Confirmar `cargo check --workspace` passa após
   adição (sem ainda ser usada).

**Critério de saída**:
- Field presente.
- Inicializado.
- `cargo check --workspace` passa.

### .C Adicionar trait method `headings_for_toc`

1. Em `01_core/src/entities/introspector.rs`:
   - Adicionar método ao trait após
     `resolved_label_for` (P193B):
     ```
     fn headings_for_toc(&self) -> &[(Label, Content, usize)];
     ```

2. Em `impl Introspector for TagIntrospector`:
   - Adicionar implementação:
     ```
     fn headings_for_toc(&self) -> &[(Label, Content, usize)] {
         &self.headings_for_toc
     }
     ```

3. **Trait passa de 19 → 20 métodos**.

4. Confirmar `@prompt-hash` actualiza após edit do
   L0.

**Critério de saída**:
- Trait method exposto.
- Implementação funcional.
- `cargo check --workspace` passa.

### .D Adicionar variant `ElementPayload::HeadingForToc`

1. Em `01_core/src/entities/element_payload.rs`:
   - Adicionar variant após `CounterUpdate` (P198C):
     ```
     /// Tag derivada de Heading com auto-toc activo.
     /// Emitida pós-recursão pelo walk arm Heading
     /// (P200B). Popula sub-store
     /// intr.headings_for_toc via from_tags arm.
     /// Consumer outline.rs:24 lê via Introspector
     /// path com fallback legacy (M5 compat).
     HeadingForToc {
         label: Label,
         body:  Content,
         level: usize,
     },
     ```

2. **`ElementPayload`: 12 → 13 variants**.

3. Confirmar `@prompt-hash` actualiza.

**Critério de saída**:
- Variant declarável.
- `cargo check --workspace` falha (esperado — match
  exhaustivos não cobertos ainda).

### .E Decidir e adicionar `ElementKind::HeadingForToc` (se aplicável)

Conforme `.A.6`:

**Se decisão "sim"**:
1. Em `01_core/src/entities/element_kind.rs`:
   - Adicionar variant:
     ```
     HeadingForToc,
     ```
   - Actualizar `as_str()`: retorna
     `"heading_for_toc"`.
   - Actualizar `from_name()`: aceita
     `"heading_for_toc"`.
2. **`ElementKind`: 10 → 11 variants**.

**Se decisão "não"**:
1. Documentar excepção em L0
   `entities/element_kind.md`:
   - Justificação: HeadingForToc é Tag derivada de
     Heading — não é Content standalone; sem
     `is_locatable` arm; sem
     `ElementKind::HeadingForToc` correspondente.
   - Cross-reference: P200B + ADR-0069 trabalho
     híbrido.

**Critério de saída**:
- Decisão materializada per `.A.6`.
- L0 actualizado.
- `cargo check --workspace` passa após adições.

### .F Cobrir match arms exaustivos induzidos

Per padrão P198C / P199B (`cargo check` revela
sítios via warnings non-exhaustive):

1. `cargo check --workspace` produz warnings.

2. Adicionar arms para `HeadingForToc` em todos os
   match exhaustivos relevantes:
   - `ElementPayload::eq` / `partial_cmp` (se houver).
   - `ElementPayload::display` ou `to_string` (se
     houver).
   - Outros descobertos via `cargo check`.

3. Confirmar `cargo check --workspace` passa.

**Critério de saída**:
- Match arms cobertos.
- `cargo check --workspace` passa.

### .G Adicionar helper `compute_heading_for_toc`

1. Em `01_core/src/rules/introspect.rs`:
   - Adicionar helper privado análogo a
     `compute_heading_auto_toc` P196B:
     ```
     fn compute_heading_for_toc(
         state: &CounterStateLegacy,
         level: usize,
         body:  &Content,
     ) -> Option<(Label, Content, usize)> {
         if !state.is_numbering_active("heading") {
             return None;
         }
         let auto_label = Label(
             format!("auto-toc-{}", state.auto_label_counter),
         );
         let frozen_body = body.clone(); // ou materialize_time per .A.3
         Some((auto_label, frozen_body, level))
     }
     ```
   - Forma exacta de `frozen_body` replica literal
     a actual mutação 4 walk arm Heading per `.A.3`
     (provável `materialize_time(body, state)` ou
     similar — confirmar empiricamente).
   - **Cláusula gate substancial**: se `frozen_body`
     diverge da mutação legacy, paridade quebra.

2. Visibilidade: privada (sem `pub`).

3. **4º helper na família ADR-0069 stylesheet**.

**Critério de saída**:
- Helper criado em `introspect.rs`.
- Forma exacta replica mutação 4 legacy.
- `cargo check --workspace` passa.

### .H Modificar walk arm Heading

1. Em `01_core/src/rules/introspect.rs:486` (per
   `.A.3`):
   - Mutação 4 legacy **continua preservada** (write
     paralelo M5):
     ```
     state.headings_for_toc.push((
         auto_label.clone(),
         frozen_body.clone(),
         *level,
     ));
     ```
   - **Após Tag::Labelled auto-toc P196B emit**,
     adicionar 3ª Tag pós-recursão:
     ```
     // P200B — emit Tag::HeadingForToc pós-recursão
     // para popular sub-store intr.headings_for_toc.
     // Mesma Location que Heading + Labelled
     // auto-toc (sub-stores diferentes — sem
     // conflito per P196A §11.5).
     if let Some(loc) = emitted_loc {
         if let Some((label, body, level)) =
             compute_heading_for_toc(state, *level, &frozen_body)
         {
             let payload = ElementPayload::HeadingForToc {
                 label,
                 body,
                 level,
             };
             tags.push(Tag::Start(
                 loc,
                 ElementInfo::new(payload),
             ));
             tags.push(Tag::End(loc, 0));
         }
     }
     ```

2. **Comentário inline P200B obrigatório**:
   ```
   // P200B — E2-residuo fechada estruturalmente
   // (trabalho híbrido: sub-store P193B-style +
   // Tag pós-recursão P196B-style + consumer
   // migration P184D-style).
   //
   // Walk arm emite 3ª Tag pós-recursão
   // (Tag::HeadingForToc) após Tag::Labelled
   // auto-toc P196B. Sub-store
   // intr.headings_for_toc populated via from_tags
   // arm. Consumer outline.rs:24 lê via Introspector
   // path com fallback legacy.
   //
   // Mutação 4 legacy preservada como write paralelo
   // M5: Layouter assignments mod.rs:1490, 1521
   // dependem de state.headings_for_toc. Cleanup
   // orgânico em M6.
   ```

3. **Comentário inline E2-residuo P196B substituído
   ou actualizado** — E2-residuo já não é "residuo";
   fecha completamente após P200B.

4. Confirmar `@prompt-hash` actualiza.

**Critério de saída**:
- Walk arm emite 3ª Tag.
- Mutação 4 legacy preservada.
- 2 comentários inline presentes (P200B novo;
  P196B actualizado para reflectir fecho).
- `cargo check --workspace` passa.

### .I Adicionar `from_tags` arm

1. Em `01_core/src/rules/introspect/from_tags.rs`:
   - Adicionar arm:
     ```
     ElementPayload::HeadingForToc { label, body, level } => {
         intr.headings_for_toc.push((
             label.clone(),
             body.clone(),
             *level,
         ));
         // Se ElementKind::HeadingForToc adicionado em .E:
         // intr.kind_index.entry(ElementKind::HeadingForToc)
         //     .or_default().push(*loc);
     }
     ```

2. Confirmar `cargo check --workspace` passa (match
   exhaustivo satisfeito).

**Critério de saída**:
- Arm adicionada.
- `cargo check --workspace` passa.

### .J Migrar consumer `outline.rs:24`

1. Em `01_core/src/rules/layout/outline.rs:24`:
   - Substituir leitura directa do legacy por
     substitution-with-fallback (padrão P184D /
     P194B):
     ```
     // P200B — substitution-with-fallback.
     // Caminho Introspector activa após P200B
     // walk arm Heading emite Tag::HeadingForToc.
     // Fallback legacy preservado durante janela
     // compat M5; cleanup orgânico em M6.
     let entries: Vec<(Label, Content, usize)> = {
         let intr_entries = layouter.introspector.headings_for_toc();
         if !intr_entries.is_empty() {
             intr_entries.to_vec()
         } else {
             layouter.counter.headings_for_toc.clone()
         }
     };
     ```
   - Forma exacta depende da estrutura empírica do
     consumer (per `.A.8`).

2. Confirmar `@prompt-hash` actualiza.

**Critério de saída**:
- Consumer migrado.
- Fallback preservado.
- `cargo check --workspace` passa.

### .K Actualizar L0

1. `entities/tag_introspector.md`:
   - Entrada para sub-store novo
     `headings_for_toc: Vec<(Label, Content, usize)>`.

2. `entities/introspector.md`:
   - Método novo `headings_for_toc()` (trait passa
     a 20 métodos).
   - Histórico actualizado.

3. `entities/element_payload.md`:
   - Variant nova `HeadingForToc { label, body, level }`.

4. (Se `.E` adicionou) `entities/element_kind.md`:
   - Variant nova `HeadingForToc`.

5. `rules/introspect.md`:
   - Tabela "Excepções M5" — actualizar entrada
     E2-residuo:
     - Estado: **fechada estruturalmente** (P200B —
       trabalho híbrido).
     - Sub-store `intr.headings_for_toc` aberto.
     - Mutação 4 legacy preservada como write
       paralelo M5.
     - Cleanup em M6.
   - Tabela "Lacunas" — actualizar lacuna #3:
     - Estado: **fechada** (P200B).
   - Lista "Ordem inversa à mutação" — passo final
     marcado ✅ (P200B).
   - **Marco arquitectural**: secção nova "M5
     universal completo" — primeira vez desde P189B.
   - Secção nova "Walk arm Heading mutação 4 migrada
     (P200B, trabalho híbrido)".
   - Cross-references: P193B (sub-store padrão),
     P196B (variante Tag pós-recursão), P184D / P194B
     (consumer migration), ADR-0069.

6. Hash em branco aguarda recálculo manual em `.M`.

**Critério de saída**:
- L0s actualizados.
- Cross-references presentes.

### .L Tests E2E

#### Tests novos (5-6)

##### Test 1 — `headings_for_toc_walk_emite_tag_e_popula_sub_store`

1. Documento com SetHeadingNumbering + Heading:
   ```
   Content::Sequence(vec![
       Content::SetHeadingNumbering { active: true },
       Content::Heading {
           level: 1,
           body: Box::new(Content::Text("Intro".into())),
       },
   ])
   ```

2. Pipeline: walk + from_tags → TagIntrospector
   populated.

3. Asserções:
   - `intr.headings_for_toc().len() == 1`.
   - Entry: `(Label("auto-toc-1"), <body>, 1)`.
   - **Confirma activação Introspector path para
     headings_for_toc**.

##### Test 2 — `headings_for_toc_paridade_legacy_vs_introspector`

1. Mesmo documento.
2. Asserções de paridade:
   - `state.headings_for_toc.len() ==
     intr.headings_for_toc().len()`.
   - Conteúdo idêntico (Label + Content + level).
   - Confirma write paralelo legacy + Introspector.

##### Test 3 — `headings_for_toc_numbering_inactivo_nao_emite_tag`

1. Documento com Heading **sem** SetHeadingNumbering:
   ```
   Content::Sequence(vec![
       Content::Heading {
           level: 1,
           body: Box::new(Content::Text("Intro".into())),
       },
   ])
   ```

2. Asserções:
   - `intr.headings_for_toc().is_empty() == true`
     (helper retorna `None`).
   - Tag não emitida.
   - Walk continua sem panic.

##### Test 4 — `walk_e2_residuo_headings_for_toc_via_legacy_E_introspector` (sentinela actualizada)

Substitui sentinela P196B
`walk_e2_residuo_headings_for_toc_via_legacy`:

1. Documento com Heading numerado.
2. Asserções:
   - `state.headings_for_toc.len() == 1` (mutação
     legacy preservada).
   - `intr.headings_for_toc().len() == 1` (sub-store
     populated).
   - Conteúdo idêntico.
   - Confirma E2-residuo fecha + write paralelo M5
     activo.

##### Test 5 — `consumer_outline_recebe_entries_via_introspector`

1. Documento com Heading numerado + outline:
   ```
   Content::Sequence(vec![
       Content::SetHeadingNumbering { active: true },
       Content::Heading { ... },
       Content::Outline,
   ])
   ```

2. Pipeline real (walk + from_tags + layout).

3. Asserções:
   - `outline.rs:24` first branch (Introspector path)
     activa.
   - Output observable: outline contém entry para
     "auto-toc-1".
   - Fallback legacy não chamado mas continua
     funcional.

##### Test 6 (opcional) — `bracketing_valido_6_tags_por_heading`

1. Documento com 1 Heading folha numerado.
2. Pipeline: walk produz tags.
3. Asserções:
   - 6 tags total: 3 Tag::Start (Heading, Labelled
     auto-toc, HeadingForToc) + 3 Tag::End.
   - Bracketing válido.
   - Mesma Location para todos.

#### Tests existentes adaptados (5)

5 tests P196B precisam adaptação (4 → 6 tags por
Heading folha — per padrão pragmático auditor #1):

| Test | Antes (após P196B) | Depois (após P200B) |
|---|---|---|
| `walk_emite_start_e_end_para_heading` | 4 tags | 6 tags (3 Start + 3 End) |
| `walk_aninha_start_end_para_heading_contendo_figure` | 6 tags | 8 tags |
| `walk_emite_tags_em_paralelo_com_state` | 10 tags (1 Set × 2 + 2 Heading × 4) | 14 tags (1 Set × 2 + 2 Heading × 6) |
| `bracketing_valido_em_sequencia_plana` | 12 tags (3 Heading × 4) | 18 tags (3 Heading × 6) |
| `end_hash_distingue_conteudo` | filter `hash != 0` | filter `hash != 0` (inalterado) |

Tests novos co-localizados em submódulo
`p200b_headings_for_toc` em `tests.rs`.

**Critério de saída**:
- 5-6 tests novos passam.
- 5 tests P196B adaptados passam.

### .M Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P200A
   baseline (1.864): **+5 a +6**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p200b_headings_for_toc::*` passam
   isoladamente.
5. Tests P196B adaptados passam (5 tests).
6. Tests existentes não regridem.
7. **Field `headings_for_toc`** presente em
   TagIntrospector (9º sub-store).
8. **Trait method** `headings_for_toc()` presente
   (20º método).
9. **Variant `ElementPayload::HeadingForToc`**
   presente (13ª variant).
10. **Decisão `ElementKind::HeadingForToc`**
    materializada (per `.E`).
11. **Helper `compute_heading_for_toc`** presente
    (4º na família ADR-0069 stylesheet).
12. **Walk arm Heading** emite 3 Tags pós-recursão
    (Heading + Labelled auto-toc + HeadingForToc).
13. **Mutação 4 legacy preservada** —
    `state.headings_for_toc.push` continua.
14. **`from_tags` arm HeadingForToc** funcional.
15. **Consumer outline.rs:24 migrado** —
    substitution-with-fallback.
16. **2 comentários inline** presentes (P200B novo;
    P196B actualizado para reflectir fecho).
17. **L0s actualizados** (`tag_introspector.md`,
    `introspector.md`, `element_payload.md`,
    eventualmente `element_kind.md`,
    `introspect.md`).
18. **Tabela Excepções M5** com E2-residuo marcada
    "fechada estruturalmente"; lacuna #3 fechada.
19. **Marco "M5 universal completo"** registado em
    L0 `introspect.md`.
20. Layouter assignments (`mod.rs:1490, 1521`) **NÃO
    modificados**.
21. `compute_heading_auto_toc` (P196B) **NÃO
    modificado**.
22. Walk arms outros (Labelled, Figure,
    SetHeadingNumbering, CounterUpdate,
    SetEquationNumbering) **NÃO modificados**.
23. Snapshot tests verdes.
24. Linter passa final.

### .N Encerramento

Escrever
`00_nucleo/materialization/typst-passo-200b-relatorio.md`
com:

- Resumo: trabalho híbrido (3 categorias combinadas);
  E2-residuo + lacuna #3 fecham; **M5 universal
  completo**.
- Confirmação `.M` (24 verificações).
- Δ tests vs baseline P200A (esperado +5 a +6).
- Hashes finais L0 (3-4 ficheiros).
- Decisões de execução notáveis:
  - Decisão `ElementKind::HeadingForToc` (per `.E`).
  - Forma de `frozen_body` confirmada empiricamente.
  - Tests P196B adaptados (5 tests).
- Estado actual:
  - P200 série: A ✅ B ✅ | C pendente.
  - **E2-residuo + lacuna #3 fechadas**.
  - **M5 universal completo** — primeira vez desde
    P189B.
  - 84 passos executados.
- Pendências cumulativas: 0 excepções activas + 0
  residuos + 0 pré-requisitos.
- Próximo passo: P200C — relatório consolidado P200
  + actualização DEBT M5-residual + **marco M5
  universal**.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. Field `headings_for_toc` adicionado em
   TagIntrospector (`.B`).
3. Trait method `headings_for_toc` exposto (`.C`).
4. Variant `ElementPayload::HeadingForToc`
   adicionada (`.D`).
5. Decisão `ElementKind::HeadingForToc` materializada
   (`.E`).
6. Match arms exaustivos cobertos (`.F`).
7. Helper `compute_heading_for_toc` criado (`.G`).
8. Walk arm Heading emite 3ª Tag pós-recursão
   (`.H`).
9. `from_tags` arm HeadingForToc funcional (`.I`).
10. Consumer `outline.rs:24` migrado (`.J`).
11. L0s actualizados (`.K`).
12. 5-6 tests novos + 5 tests P196B adaptados
    passam (`.L`).
13. Verificações `.M` passam (24/24).
14. Mutações legacy preservadas (mutação 4 walk arm
    + Layouter assignments).
15. Output observable em produção alterado **apenas
    para activar** Introspector path
    (substitution-with-fallback fornece fallback).
16. Relatório `.N` escrito.

---

## O que pode sair errado

- **Forma de `frozen_body` diverge da mutação 4
  legacy**: cláusula gate substancial — investigar
  empiricamente em `.A.3`. Se diverge, paridade
  quebra.
- **`materialize_time` ou similar tem efeitos
  colaterais não previstos**: cláusula gate
  substancial.
- **Layouter assignments quebram após adição de
  trait method**: improvável (trait method é
  adição); cláusula gate substancial se acontecer.
- **Tests P196B adaptação produz off-by-one**:
  cláusula gate trivial — re-verificar contagem
  exacta.
- **`ElementKind::HeadingForToc` decisão tem
  consequências em testes existentes** que iteram
  sobre todos ElementKinds: cláusula gate trivial —
  ajustar fixture.
- **Match arms exaustivos divergem do esperado**
  (ficheiros não previstos): cláusula gate trivial.
- **Test 5 (consumer outline activação) falha**:
  indica que substitution-with-fallback não activa.
  Cláusula gate substancial.
- **Test 6 (bracketing 6 tags) revela ordem
  divergente**: investigar ordem de emit no walk
  arm.
- **Snapshot tests divergem**: pode acontecer se
  outline produz output ligeiramente diferente.
  Investigar.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M+ genuíno. ~150 LOC produção
  (sub-store + trait + variant + helper + walk arm
  + from_tags + consumer + match arms induzidos) +
  ~180 LOC tests + ~100 LOC L0.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **DEBT M6**: write paralelo M5 ainda activo —
  mutações legacy + Layouter assignments + `compute_*`
  helpers leem legacy. Cleanup orgânico em M6.
- **Padrão materializado**: trabalho híbrido
  (combinação de 3 padrões testados). **Sem nova
  variante operacional ADR-0069** — combinação directa
  de variantes existentes.
- **Cláusula gate trivial**: aplicável a forma exacta
  de helper, recálculo de hashes, decisão
  ElementKind, adaptação tests P196B.
- **Cláusula gate substancial**: aplicável a:
  - `frozen_body` diverge.
  - Layouter quebra.
  - Test 5 (activação consumer) falha.
  - Test 6 (bracketing) revela ordem divergente.
- **Próximo passo P200C**: relatório consolidado
  P200 (9 secções) + actualização DEBT M5-residual +
  **marco M5 universal**. Magnitude S puro.
- **Estado pós-P200C**: M5 universal completo;
  desbloqueia M6 (P190A reescrita do zero;
  magnitude L cross-modular).
- **Marco arquitectural significativo**: primeira
  vez M5 universal completo desde declaração em
  P189B (>14 séries entre declaração e fecho final).
