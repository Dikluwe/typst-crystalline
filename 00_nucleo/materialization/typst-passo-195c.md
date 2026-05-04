# Passo P195C — Estender `from_tags` arm `Labelled` com populate

Segundo passo de implementação P195 (após P195A
diagnóstico, P195B variant + ADR PROPOSTO).
Magnitude **S**.

Substitui stub no-op
`ElementPayload::Labelled { .. } => {}` (P195B) por arm
funcional que popula `intr.resolved_labels` (P193B) e
`intr.figure_label_numbers` (P168) baseado nos campos
do payload.

Após P195C:
- `from_tags` arm Labelled funcional — popula sub-stores
  via Tag.
- Sub-store `intr.resolved_labels` populated quando Tag
  Labelled processada.
- Sub-store `intr.figure_label_numbers` populated quando
  Tag Labelled tem `figure_number = Some(n)`.
- Walk arm Labelled ainda **NÃO emite Tag** — P195D faz.
- Em produção real: Tags Labelled **não chegam a
  `from_tags`** (porque walk arm não emite). Sub-stores
  permanecem vazios em produção até P195D.
- Tests unit populam Tags manualmente para validar arm.

**Pré-condição**: P195B concluído. Tests workspace 1.830
verdes; zero violations. Variant
`ElementPayload::Labelled` declarado. Stub no-op
presente em `from_tags`.

**Restrições**:
- **Não** modificar walk arm — P195D.
- **Não** modificar variant `ElementPayload::Labelled` —
  P195B fechou.
- **Não** activar `is_locatable(Content::Labelled) =
  true` — decisão arquitectural Opção 1-modificada (sem
  locatable).
- **Não** adicionar arm em `extract_payload` —
  arquitecturalmente impossível per ADR-0069.
- **Não** modificar trait `Introspector` (P185B fechou).
- **Não** modificar `TagIntrospector` (P193B fechou).
- **Não** modificar sub-stores — P193B/P168 fecharam.
- **Não** modificar consumer C4 — P194B fechou.
- **Não** preservar nenhum write paralelo legacy —
  esse trabalho é P195D.
- API pública preservada.
- Output observable em produção **inalterado** — Tags
  Labelled não chegam a `from_tags` (walk arm não emite).

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `from_tags` actual:
   - `01_core/src/rules/introspect/from_tags.rs`.
   - Localizar stub no-op
     `ElementPayload::Labelled { .. } => {}`
     introduzido em P195B.
   - Verificar linha exacta.

2. Confirmar API `ResolvedLabelStore::insert`:
   - Per P193B `.B`: `pub(crate) fn insert(&mut self,
     label: Label, resolved: String)`.
   - Confirmar empiricamente.

3. Confirmar API `figure_label_numbers` populate:
   - Sub-store P168. Localizar arm Figure em `from_tags`
     que popula.
   - API esperada: `intr.figure_label_numbers.insert(label,
     n)` ou similar.
   - Replicar pattern em arm Labelled.

4. Confirmar variant `ElementPayload::Labelled` (P195B):
   - Campos: `label: Label`, `resolved_text:
     Option<String>`, `figure_number: Option<usize>`.
   - Verificar empiricamente.

5. Confirmar L0 `rules/introspect/from_tags.md`:
   - Localizar entrada Histórico de P195B.
   - Identificar onde adicionar entrada P195C.

6. Confirmar tests existentes em `from_tags.rs`:
   - Padrão de tests para outros arms locatable
     (P184B Figure, P186E Equation).
   - Replicar para Labelled mas com **populate manual de
     Tags** (sem walk arm emitir).

Output: tabela com item + estado + linhas exactas.

**Critério de saída**:
- Stub no-op localizado.
- APIs `insert` confirmadas.
- Tests pattern identificado.

### .B Substituir stub no-op por arm funcional

1. Em `01_core/src/rules/introspect/from_tags.rs`:
   - Localizar stub no-op P195B.
   - Substituir por:
     ```
     ElementPayload::Labelled {
                 label,
                 resolved_text,
                 figure_number,
             } => {
                 if let Some(text) = resolved_text {
                     intr.resolved_labels
                         .insert(label.clone(), text.clone());
                 }
                 if let Some(n) = figure_number {
                     intr.figure_label_numbers
                         .insert(label.clone(), *n);
                 }
             }
     ```
   - Forma exacta fica para Claude Code conforme
     convenção do projecto (clones, references).

2. Comentário inline opcional curto:
   ```
   // P195C — populate completo (era stub no-op em
   // P195B). Walk arm emite Tag em P195D; até lá,
   // Tags chegam apenas via tests.
   ```

3. Confirmar `@prompt-hash` actualiza após edit do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- Arm funcional presente.
- Stub no-op substituído.

### .C Actualizar L0 `rules/introspect/from_tags.md`

1. Adicionar entrada Histórico:
   - P195C — arm `ElementPayload::Labelled` estendido
     com populate completo de `intr.resolved_labels`
     (P193B) + `intr.figure_label_numbers` (P168).
   - P195D — walk arm emite Tag pós-recursão (próximo
     passo).
   - Cross-reference: ADR-0069.

2. Hash em branco aguarda recálculo manual em `.E`.

**Critério de saída**:
- L0 Histórico actualizado.

### .D Tests unit do arm

3-4 tests obrigatórios. Padrão dos arms `from_tags`
existentes mas com **populate manual de Tags** (walk não
emite até P195D).

#### Test 1 — `labelled_arm_popula_resolved_labels`

1. Construir Tag manualmente:
   ```
   let tag = Tag::Start(loc(0), ElementInfo::new(
       ElementPayload::Labelled {
           label:         Label("intro".to_string()),
           resolved_text: Some("Capítulo 1".to_string()),
           figure_number: None,
       },
   ));
   ```

2. Pipeline: `from_tags(vec![tag, ...])` ou directly
   processar o arm.

3. Asserções:
   - `intr.resolved_labels.get(&Label("intro".into()))`
     retorna `Some("Capítulo 1")`.
   - `intr.figure_label_numbers.get(&Label("intro".into()))`
     retorna `None` (figure_number era None).

#### Test 2 — `labelled_arm_popula_figure_label_numbers_quando_some`

1. Tag com `figure_number: Some(3)`:
   ```
   ElementPayload::Labelled {
       label:         Label("fig1".to_string()),
       resolved_text: Some("Figura 3".to_string()),
       figure_number: Some(3),
   }
   ```

2. Asserções:
   - `intr.resolved_labels.get(&Label("fig1".into()))`
     retorna `Some("Figura 3")`.
   - `intr.figure_label_numbers.get(&Label("fig1".into()))`
     retorna `Some(3)` ou `Some(&3)`.

#### Test 3 — `labelled_arm_resolved_text_none_nao_popula`

1. Tag com `resolved_text: None`:
   ```
   ElementPayload::Labelled {
       label:         Label("noref".to_string()),
       resolved_text: None,
       figure_number: None,
   }
   ```

2. Asserções:
   - `intr.resolved_labels.get(&Label("noref".into()))`
     retorna `None`.
   - `intr.figure_label_numbers.get(&Label("noref".into()))`
     retorna `None`.
   - **Caso edge**: ambos `None` é válido (Labelled com
     target não-resolvível); arm não panica.

#### Test 4 — `labelled_arm_múltiplos_labels`

1. 3 Tags com labels distintos.
2. Asserção: cada label retorna o seu valor único nos
   sub-stores.

Tests co-localizados em `mod tests` de `from_tags.rs`.

**Critério de saída**:
- 3-4 tests passam.
- Tests existentes não regridem.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P195B
   baseline (1.830): **+3 a +4**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Arm `ElementPayload::Labelled { ... }` em `from_tags`
   funcional (não é mais stub no-op).
5. `intr.resolved_labels.insert` chamado quando
   `resolved_text` é `Some`.
6. `intr.figure_label_numbers.insert` chamado quando
   `figure_number` é `Some`.
7. **Walk arm Labelled NÃO modificado** — P195D mexe.
8. Em produção: Tags Labelled não chegam a `from_tags`
   (walk arm legacy ainda muta directamente). Sub-store
   `intr.resolved_labels` permanece vazio em produção.
9. Trait `Introspector` NÃO modificado.
10. `TagIntrospector` NÃO modificado.
11. Consumer C4 NÃO modificado.
12. Snapshot tests verdes.
13. Linter passa final.

### .F Encerramento

Escrever
`00_nucleo/materialization/typst-passo-195c-relatorio.md`
com:

- Resumo: arm funcional substitui stub no-op; populate
  via Tag activa só em testes (walk arm não emite até
  P195D).
- Confirmação `.E` (13 verificações).
- Δ tests vs baseline P195B (esperado +3 a +4).
- Hashes finais L0 (`from_tags.md`).
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P195 série: A ✅ B ✅ C ✅ | D-E pendentes.
  - 67 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P195D — walk arm Labelled emite Tag
  pós-recursão. **Magnitude M** — primeiro pattern
  post-recursion materializado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Arm funcional substitui stub no-op em `from_tags`.
3. L0 `rules/introspect/from_tags.md` actualizado.
4. 3-4 tests unit passam.
5. Tests existentes não regridem.
6. Verificações `.E` passam (13/13).
7. Walk arm Labelled NÃO modificado.
8. Output observable em produção inalterado.
9. Relatório `.F` escrito.

---

## O que pode sair errado

- **`ResolvedLabelStore::insert` não é `pub(crate)`**
  acessível de `from_tags`: improvável (P193B `.B`
  decidiu `pub(crate)`); cláusula gate trivial — ajustar
  visibilidade.
- **`figure_label_numbers.insert` exige assinatura
  diferente**: cláusula gate trivial — replicar pattern
  P168 arm Figure.
- **Pattern matching exige `ref` keywords ou clones
  explícitos**: cláusula gate trivial — adaptar.
- **`label.clone()` exige `Clone` em `Label`**:
  improvável (P193A confirmou Label tem Clone derivado);
  cláusula gate trivial.
- **Test 3 (caso edge `None` ambos) falha por
  early-return ou panic no arm**: cláusula gate trivial
  — adaptar lógica para tolerar `None` ambos.
- **Tests existentes regridem**: improvável (arm é
  aditivo; sem `else` que mude comportamento de outros
  arms). Investigar se acontecer.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~10 LOC arm + ~50 LOC tests +
  edit L0.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão replicado**: P186E (Equation arm `from_tags`
  estendido após stub no-op). Forma idêntica.
- **Cláusula gate trivial**: aplicável a visibilidade
  APIs, pattern matching, clones, ordering arms.
- **Sem cláusula gate substancial esperada**.
- **Estado intermédio seguro**:
  - Arm funcional mas walk não emite Tag.
  - Tests populam manualmente.
  - Output observable em produção inalterado.
  - Sub-stores `intr.resolved_labels` /
    `intr.figure_label_numbers` permanecem vazios em
    produção (até P195D).
- **Próximo passo P195D**: magnitude M. Walk arm
  Labelled modificado para emitir Tag pós-recursão
  (pattern ADR-0069). Helper `compute_labelled` (per
  P195A §11.6) introduzido. **Mantém mutação legacy
  paralela** durante janela compat M5.
