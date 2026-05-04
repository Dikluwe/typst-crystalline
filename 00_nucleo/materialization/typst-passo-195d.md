# Passo P195D — Walk arm `Labelled` emite Tag pós-recursão

Terceiro passo de implementação P195 (após P195A
diagnóstico, P195B variant + ADR PROPOSTO, P195C
`from_tags` arm funcional).
Magnitude **M** — **primeiro pattern post-recursion
materializado em walk** (pattern arquitectural ADR-0069
aplicado pela primeira vez).

Modifica walk arm `Content::Labelled` em
`01_core/src/rules/introspect.rs:432-486` para:

1. **Manter mutação legacy paralela** (write paralelo
   durante janela compat M5 — preserva fallback C4 P194
   funcional).
2. **Computar `resolved_text` e `figure_number`** via
   helper privado `compute_labelled(target, state,
   label) -> (Option<String>, Option<usize>)`.
3. **Emitir Tag manualmente após recursão**:
   `Tag::Start(loc, ElementInfo::new(
   ElementPayload::Labelled { label, resolved_text,
   figure_number }))` + `Tag::End(loc, 0)`.

Helper `compute_labelled` isola a lógica de computação
(per P195A §11.6) — facilita reuso entre mutação legacy
e populate Tag, reduz duplicação.

Após P195D:
- Walk arm Labelled emite Tag pós-recursão em produção.
- `from_tags` arm Labelled (P195C) processa Tag → popula
  `intr.resolved_labels` + `intr.figure_label_numbers`.
- Consumer C4 (P194B) começa a receber `Some(text)`
  do Introspector para **explicit labels**. **Inversão
  observable parcial** — primeira activação real do
  caminho Introspector para resolved labels.
- Mutação legacy preservada — fallback continua
  funcional. Output observable em produção inalterado
  (Introspector path activa mas legacy fornece valores
  idênticos; substitution-with-fallback chamado em
  ambos os casos).
- **E4 fecha estruturalmente**. Funcionalmente fecha em
  M6 quando mutação legacy for removida.
- E2 (Heading auto-toc) continua activa — só fecha em
  P196.

**Pré-condição**: P195C concluído. Tests workspace 1.834
verdes; zero violations. Variant declarado, ADR
PROPOSTO, `from_tags` arm funcional. Walk arm Labelled
**ainda não modificado**.

**Restrições**:
- **Manter mutação legacy paralela** —
  `state.resolved_labels.insert(...)` +
  `state.figure_label_numbers.insert(...)` continuam
  presentes durante janela compat M5. Removidas em M6.
- **Não** activar `is_locatable(Content::Labelled) =
  true` — pattern post-recursion bypass mecanismo
  locatable per ADR-0069.
- **Não** adicionar arm em `extract_payload` —
  arquitecturalmente impossível per ADR-0069.
- **Não** adicionar `ElementKind::Labelled` — sem
  locatable kind.
- **Não** modificar walk arm Heading — P196.
- **Não** modificar walk arm Figure — P197.
- **Não** modificar `Locator` ou `current_location` —
  P185 fechou.
- API pública preservada.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar walk arm Labelled actual:
   - `01_core/src/rules/introspect.rs:432-486` (per
     P195A §2.1). Re-verificar empiricamente.
   - Localizar:
     - Recursão `walk(target, ..., Some(label))` —
       propagação de label via `label_from_parent`.
     - Mutações legacy
       `state.resolved_labels.insert(label, text)`.
     - Mutações legacy `state.figure_label_numbers
       .insert(label, n)` (se aplicável).
     - Lógica que computa `text` (depende de target
       type — Heading/Equation/Figure).

2. Confirmar campos do walk arm:
   - Variáveis em escopo: `state`, `locator`, `tags`,
     `label_from_parent`?
   - Identificar qual variável é o `label` neste
     contexto.

3. Confirmar `Locator::next` e gestão de locations no
   walk:
   - Walk arm Labelled emite Tags? Per P195A: walk
     legacy faz mutação directa, não emite Tag para
     Labelled.
   - Após P195D: walk arm precisa de chamar
     `locator.next()` para obter Location para a Tag
     nova.
   - **Cuidado**: Labelled não é locatable
     (`is_locatable=false`). Per P185 e ADR-0068,
     gating em `layout_content` não dispara para
     Labelled. Locator do **walk** chama `next()` aqui;
     Locator do **Layouter** não. Resultado: locations
     dessincronizam para Labelled?

   **Cláusula gate substancial potencial**: se Locator
   walk avança para Labelled mas Layouter não, ADR-0068
   sincronização-por-construção fica violada para
   Labelled. Auditar empiricamente em `.A.3`.

   Possíveis soluções:
   - (a) Reusar Location do parent (não chamar
     `locator.next()`).
   - (b) Aceitar dessincronização — Labelled não tem
     consumer location-aware (consumer C4 P194 não usa
     `current_location`).
   - (c) Activar `is_locatable(Labelled) = true` para
     sincronizar (mas viola decisão arquitectural Opção
     1-modificada).

3-bis. **Decisão preliminar**: Opção (a) — reusar
Location do parent. Walk emite Tag com a mesma Location
do target (recursão acabou de emitir Location para
target via gating walk-side). Implicação: Tag Labelled
"encavalita-se" sobre Location do target.

   Alternativa: Opção (b) se reuso não funcionar
   semanticamente. Auditor decide empiricamente.

4. Confirmar lógica de computação de `text`:
   - Per P195A §11.1: depende de `state.format_hierarchical`,
     `state.get_flat`, `state.figure_numbers`,
     `state.lang`.
   - Match sobre `target` type.

5. Confirmar lógica de computação de `figure_number`:
   - `state.figure_numbers["figure"]` se target é
     Figure numerada.
   - `None` caso contrário.

6. Confirmar L0 `rules/introspect.md`:
   - Localizar entrada walk arm Labelled.
   - Identificar onde adicionar nota sobre emissão Tag
     pós-recursão.

7. Confirmar tests existentes:
   - Tests em P189B sentinela E4 (per P189B §4 ponto 3).
   - Identificar quais devem manter-se inalterados.

Output: tabela com item + estado + decisão sobre Locator
(cláusula gate em `.A.3`).

**Critério de saída**:
- Walk arm localizado.
- Decisão Locator (a/b/c) fixada.
- Helper `compute_labelled` esquema confirmado.

### .B Criar helper privado `compute_labelled`

1. Em `01_core/src/rules/introspect.rs` (ou módulo
   similar):
   - Adicionar função privada:
     ```
     fn compute_labelled(
         target: &Content,
         state:  &CounterStateLegacy,
         _label: &Label,
     ) -> (Option<String>, Option<usize>) {
         match target {
             Content::Heading { .. } => {
                 let prefix = state.format_hierarchical("heading");
                 (prefix.map(|n| format!("Secção {}", n)), None)
             }
             Content::Equation { block, .. } if *block => {
                 let n = state.get_flat("equation");
                 if n > 0 {
                     (Some(format!("Equação ({})", n)), None)
                 } else {
                     (None, None)
                 }
             }
             Content::Figure { kind, numbering, caption, .. } => {
                 // replica lógica actual
                 // ...
             }
             _ => (None, None),
         }
     }
     ```
   - Forma exacta replica lógica actual do walk arm.
   - Visibilidade: `fn` privada (sem `pub`) —
     uso interno apenas.

2. Confirmar `@prompt-hash` actualiza após edit do L0
   em `.E`.

**Critério de saída**:
- Helper criado.
- `cargo check --workspace` passa.
- Sem regressão (helper não invocado ainda).

### .C Modificar walk arm Labelled

1. Em `01_core/src/rules/introspect.rs:432-486`:
   - Antes da recursão: nada muda.
   - Recursão: nada muda
     (`walk(target, ..., Some(label))`).
   - **Após recursão** (mudança nova):
     ```
     let (resolved_text, figure_number) =
         compute_labelled(target, state, label);

     // Mutação legacy preservada (write paralelo durante
     // janela compat M5; remover em M6 per ADR-0069 §7).
     if let Some(ref text) = resolved_text {
         state.resolved_labels
             .insert(label.clone(), text.clone());
     }
     if let Some(n) = figure_number {
         state.figure_label_numbers
             .insert(label.clone(), n);
     }

     // Tag emit pós-recursão (pattern ADR-0069).
     // Reusa Location do target (per .A.3 decisão).
     if resolved_text.is_some() || figure_number.is_some() {
         let payload = ElementPayload::Labelled {
             label: label.clone(),
             resolved_text,
             figure_number,
         };
         tags.push(Tag::Start(loc, ElementInfo::new(payload)));
         tags.push(Tag::End(loc, 0));
     }
     ```
   - `loc` é a Location do target (reuso per cláusula
     gate `.A.3` Opção (a)).
   - Forma exacta fica para Claude Code conforme
     convenção do projecto.

2. Comentário inline obrigatório:
   ```
   // P195D — walk arm Labelled emite Tag pós-recursão
   // (pattern ADR-0069 post-recursion-tag-emission).
   // Mutação legacy preservada como write paralelo
   // durante janela compat M5; removida em M6.
   ```

3. Confirmar `@prompt-hash` actualiza após edit do L0
   em `.E`.

**Critério de saída**:
- Walk arm Labelled emite Tag pós-recursão.
- Mutação legacy preservada paralela.
- `cargo check --workspace` passa.

### .D Tests E2E paridade + activação Introspector

3-4 tests obrigatórios. Padrão tests E2E P186F /
P187B `.D`.

#### Test 1 — `labelled_walk_emite_tag_e_popula_introspector`

1. Construir documento com Heading + Labelled:
   ```
   Content::Sequence(vec![
       Content::SetHeadingNumbering { active: true },
       Content::Labelled {
           label:  Label("intro".into()),
           target: Box::new(Content::Heading {
               level: 1,
               body: Box::new(Content::Text("Intro".into())),
               label: None,
           }),
       },
       Content::Ref(Label("intro".into())),
   ])
   ```

2. Pipeline: walk + from_tags → TagIntrospector populated.

3. Asserções:
   - `intr.resolved_labels.get(&Label("intro".into()))`
     retorna `Some("Secção 1")` (ou texto equivalente).
   - `intr.figure_label_numbers.get(&Label("intro".into()))`
     retorna `None` (Heading, não Figure).
   - **Confirmação central**: caminho Introspector
     activa para explicit labels.

#### Test 2 — `labelled_paridade_observable_legacy_vs_introspector`

1. Mesmo documento.
2. Asserções de paridade:
   - `state.resolved_labels.get(&Label("intro".into()))`
     == `intr.resolved_labels.get(&Label("intro".into()))`.
   - Confirma write paralelo legacy + Introspector.
3. Layout completo:
   - Consumer C4 (P194B) recebe `Some(text)` do
     Introspector path.
   - `or_else` fallback legacy não chamado (mas continua
     funcional como backup).
   - `plain_text` contém "Secção 1" (não `@intro`).

#### Test 3 — `labelled_figure_target_popula_figure_label_numbers`

1. Documento com Labelled target=Figure:
   ```
   Content::Labelled {
       label:  Label("fig1".into()),
       target: Box::new(Content::Figure {
           kind: Some("figure".into()),
           numbering: Some(...),
           caption: ...,
           ..
       }),
   }
   ```

2. Asserções:
   - `intr.figure_label_numbers.get(&Label("fig1".into()))`
     retorna `Some(1)` (ou número correcto).
   - `intr.resolved_labels.get(&Label("fig1".into()))`
     retorna `Some("Figura 1")` (ou texto equivalente).

#### Test 4 — `labelled_target_nao_resolvivel_nao_emite_tag`

1. Documento com Labelled target=Text simples (sem
   numeração):
   ```
   Content::Labelled {
       label:  Label("foo".into()),
       target: Box::new(Content::Text("not numbered".into())),
   }
   ```

2. Asserções:
   - `intr.resolved_labels.get(&Label("foo".into()))`
     retorna `None`.
   - `intr.figure_label_numbers.get(&Label("foo".into()))`
     retorna `None`.
   - Tag não emitida (per condição
     `if resolved_text.is_some() || figure_number.is_some()`).
   - Confirma caso edge — sem tag inútil.

Tests co-localizados em submódulo `p195d_walk_labelled`
em `tests.rs`.

**Critério de saída**:
- 3-4 tests passam.
- Tests existentes não regridem.

### .E Actualizar L0 `rules/introspect.md`

1. Adicionar entrada para walk arm Labelled
   modificado:
   - Walk arm emite Tag pós-recursão (pattern ADR-0069).
   - Helper `compute_labelled` introduzido.
   - Mutação legacy preservada — write paralelo durante
     janela compat M5.
   - **E4 fecha estruturalmente** (Introspector activa;
     funcionalmente fecha em M6).
   - Cross-references: ADR-0069, P195A §11.6, P189B
     §5 E4.

2. Hash em branco aguarda recálculo manual em `.F`.

**Critério de saída**:
- L0 reflecte mudança ao walk arm.
- Cross-references presentes.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P195C
   baseline (1.834): **+3 a +4**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p195d_walk_labelled::*` passam isoladamente.
5. Tests existentes (incluindo sentinela E4 P189B) **não
   regridem** — mutação legacy preservada.
6. Walk arm Labelled emite Tag pós-recursão em produção.
7. Helper `compute_labelled` invocado pelo walk arm.
8. **Mutação legacy preservada** — `state.resolved_labels
   .insert(...)` + `state.figure_label_numbers.insert(...)`
   continuam presentes.
9. `from_tags` arm Labelled (P195C) processa Tags em
   produção real → popula sub-stores.
10. **`is_locatable(Content::Labelled)` continua
    `false`** — sem janela invariante quebrada.
11. **`extract_payload(Content::Labelled)` continua a
    retornar `None`** — pattern post-recursion bypass.
12. Trait `Introspector` NÃO modificado.
13. `TagIntrospector` NÃO modificado.
14. Consumer C4 (P194B) NÃO modificado — apenas começa
    a receber `Some(text)` em vez de `None` do
    Introspector path.
15. Snapshot tests verdes.
16. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-195d-relatorio.md`
com:

- Resumo: walk arm emite Tag pós-recursão; helper
  `compute_labelled`; E4 fecha estruturalmente.
- Confirmação `.F` (16 verificações).
- Δ tests vs baseline P195C (esperado +3 a +4).
- Hashes finais L0 (`rules/introspect.md`).
- Decisões de execução notáveis:
  - Decisão Locator (`.A.3` opção a/b/c).
  - Helper `compute_labelled` materializado.
  - Mutação legacy preservada (write paralelo).
- Estado actual:
  - P195 série: A ✅ B ✅ C ✅ D ✅ | E pendente.
  - **E4 fecha estruturalmente**.
  - Inversão observable parcial — explicit labels via
    Introspector; auto-toc continua legacy (E2 activa).
  - 68 passos executados.
- Pendências cumulativas: E1, E2, E3, E5, E6 activas.
- Próximo passo: P195E — tests E2E + transição
  ADR-0069 PROPOSTO → ACEITE + relatório consolidado P195.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria + decisão Locator fixada.
2. Helper `compute_labelled` criado (`.B`).
3. Walk arm Labelled emite Tag pós-recursão (`.C`).
4. Mutação legacy preservada paralela.
5. 3-4 tests E2E passam (`.D`).
6. L0 `rules/introspect.md` actualizado (`.E`).
7. Verificações `.F` passam (16/16).
8. Tests existentes não regridem (paridade observable
   preservada).
9. Output observable em produção inalterado (Introspector
   activa mas legacy fornece valores idênticos via write
   paralelo).
10. Relatório `.G` escrito.

---

## O que pode sair errado

- **Locator dessincroniza** (cláusula gate substancial
  per `.A.3`): se reuso de Location não funcionar
  semanticamente, considerar Opção (b) — aceitar
  dessincronização (consumer C4 não usa
  `current_location`). **Risco moderado** — auditor
  decide empiricamente.
- **Helper `compute_labelled` exige acesso a state
  parcial não-disponível** em algum caso edge: cláusula
  gate substancial. Investigar empiricamente.
- **Tests E2E falham por timing** (Tag Labelled
  processada antes de Tag do target?): cláusula gate
  substancial. Investigar ordem em `tags`.
- **Test 4 (target não-resolvível) falha por emit de
  Tag inútil**: cláusula gate trivial — ajustar
  condição `if resolved_text.is_some() ||
  figure_number.is_some()`.
- **Tests sentinela E4 P189B regridem**: indica que
  mutação legacy não foi preservada como esperado.
  Cláusula gate substancial — re-verificar `.C`.
- **`compute_labelled` duplica lógica do walk arm e
  diverge**: cláusula gate trivial — refactor para
  unificar.
- **Snapshot tests divergem**: indica que ordem de
  Tags ou contagem mudou. Investigar.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: M genuíno. ~30 LOC walk arm + ~50 LOC
  helper + ~150 LOC tests + edits L0.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão materializado**: ADR-0069 post-recursion-tag-emission
  aplicado pela primeira vez. Precedente para P196/P197/P198.
- **Cláusula gate trivial**: aplicável a forma exacta
  do helper, ordem das mutações, recálculo de hashes.
- **Cláusula gate substancial**: aplicável a:
  - Locator dessincronização (cláusula `.A.3`).
  - Tests sentinela E4 regridem (mutação legacy não
    preservada).
  - `compute_labelled` exige state não-disponível.
  - Ordem de Tags causa timing issues.
- **Inversão observable parcial em produção**: explicit
  labels via Introspector; auto-toc continua legacy.
  Primeira inversão real do trabalho M5.
- **E4 fecha estruturalmente após P195D**:
  - Caminho Introspector activa para explicit labels.
  - Mutação legacy preservada como fallback.
  - Funcionalmente fecha em M6 quando legacy for
    removido.
- **Próximo passo P195E**: tests E2E paridade
  observable + transição ADR-0069 PROPOSTO → ACEITE +
  relatório consolidado P195. Magnitude S.
- **Padrão pragmático auditor disponível**: durante
  execução pode ser necessário aplicar 1 dos 3 padrões
  M4-residual (ajustar fixture, violar restrição
  justificadamente, inlining para evitar circularidade).
