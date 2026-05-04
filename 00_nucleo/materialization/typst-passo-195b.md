# Passo P195B — `ElementPayload::Labelled` + ADR PROPOSTO

Primeiro passo de implementação P195 (após P195A
diagnóstico). Magnitude **S**.

Adiciona variant `ElementPayload::Labelled { label,
resolved_text, figure_number }` ao enum
`ElementPayload`. Adiciona stub no-op em `from_tags`
(cláusula gate trivial per P186B aprendizado — match
exaustivo força arm explícito). Cria **ADR PROPOSTO**
formalizando pattern arquitectural novo "post-recursion
tag emission for state-dependent payload" identificado
em P195A §11.1-§11.2.

**Sem** `ElementKind::Labelled`. **Sem** arm em
`is_locatable`. **Sem** arm em `extract_payload`. Per
decisão arquitectural P195A (Opção 1-modificada) —
walk arm emite Tag manualmente em P195D, sem passar
por mecanismo locatable.

Após P195B:
- `ElementPayload` ganha 11ª variant.
- Stub no-op `ElementPayload::Labelled { .. } => {}` em
  `from_tags` (P186B-style; P195C estende).
- ADR PROPOSTO documenta pattern.
- Walk arm Labelled **NÃO modificado** — P195D.
- `is_locatable(Content::Labelled)` **continua `false`**
  — sem janela invariante quebrada (per P195A §11.1
  decisão).
- Sub-store `intr.resolved_labels` continua **vazio em
  produção** — populate via Tag activa em P195C+P195D.

**Pré-condição**: P195A concluído. Tests workspace 1.825
verdes; zero violations. 7 cláusulas P195A fechadas.
Decisão Opção 1-modificada fixada.

**Restrições**:
- **Não** activar `is_locatable(Content::Labelled) =
  true` — decisão Opção 1-modificada (sem locatable).
- **Não** adicionar arm em `extract_payload` —
  arquitecturalmente impossível per P195A §11.1.
- **Não** adicionar `ElementKind::Labelled` — sem
  locatable kind.
- **Não** modificar walk arm — P195D.
- **Não** estender stub no-op em `from_tags` — P195C.
- **Não** modificar consumer C4 — P194B fechou.
- **Não** modificar `state.resolved_labels` ou
  `state.figure_label_numbers` legacy — preservar walk
  arm legacy paralelo durante janela compat M5.
- API pública preservada — adição de variant é
  retrocompatível.
- Output observable em produção **inalterado** — variant
  novo sem produtor; stub no-op em `from_tags`.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `ElementPayload` actual:
   - 10 variants após P186B (incl. `Equation`).
   - `01_core/src/entities/element_payload.rs`.
   - Identificar onde inserir `Labelled` (provável: após
     `Equation`, conforme convenção cronológica per
     P186B §"Decisões"). Verificar empiricamente.

2. Confirmar L0 `entities/element_payload.md`:
   - Localizar estrutura actual.
   - Identificar onde adicionar entrada para variant
     novo.

3. Confirmar `Label` newtype (per P193A §2.2):
   - `01_core/src/entities/label.rs:12` —
     `pub struct Label(pub String)`.
   - Derives: `Eq, Hash, Clone, Debug` (necessários
     para HashMap key + Tag content).

4. Confirmar `from_tags` actual:
   - Match exhaustivo sobre `ElementPayload` (per P186B
     §"Decisões" - gate trivial força stub).
   - Identificar onde adicionar stub no-op
     `ElementPayload::Labelled { .. } => {}`.

5. Inventariar ADRs existentes:
   - `00_nucleo/adr/`.
   - Identificar próximo número livre.
   - Última ADR criada: ADR-0068 (P185E ACEITE).
   - Próxima provável: **ADR-0069**.

6. Confirmar shape ADR padrão:
   - Ler ADR-0068 ou ADR-0067 como template.
   - Identificar secções padrão (Estado, Contexto,
     Decisão, Alternativas, Consequências, Histórico).

7. Confirmar tests existentes em `mod tests` de
   `element_payload.rs`:
   - Padrão dos tests para variants existentes (per
     P186B `.F`).
   - Replicar para `Labelled`.

Output: tabela com item + estado + linhas exactas.

**Critério de saída**:
- `ElementPayload` localizado.
- Stub no-op em `from_tags` localizado.
- Próximo número ADR identificado.
- Template ADR identificado.

### .B Adicionar variant `ElementPayload::Labelled`

1. Em `01_core/src/entities/element_payload.rs`:
   - Adicionar variant após `Equation` (per `.A.1`):
     ```
     Labelled {
         label:         Label,
         resolved_text: Option<String>,
         figure_number: Option<usize>,
     },
     ```

2. Justificação dos campos (per P195A §3 cláusula 3
   Opção α):
   - `label`: chave para `intr.resolved_labels` +
     `intr.figure_label_numbers` populate.
   - `resolved_text`: texto pré-computed pelo walk arm
     (per P195A §11.1 — walk arm tem acesso a state).
   - `figure_number`: opcional; `Some(n)` quando target
     é figure numerada; `None` caso contrário.

3. Confirmar `@prompt-hash` actualiza após edit do L0 em
   `.C`.

**Critério de saída**:
- `cargo check --workspace` passa.
- Variant declarável e construível.

### .C Actualizar L0 `entities/element_payload.md`

1. Adicionar entrada para variant novo:
   - Nome: `Labelled`.
   - Campos: `label: Label`, `resolved_text:
     Option<String>`, `figure_number: Option<usize>`.
   - Propósito: payload emitido **post-recursion** pelo
     walk arm Labelled (per ADR-XXXX). Permite
     `from_tags` popular `intr.resolved_labels` +
     `intr.figure_label_numbers` sem
     `extract_payload` puro (arquitecturalmente
     impossível para state-dependent payload).
   - Cross-reference: ADR-XXXX, P195A §11.1.

2. Hash em branco aguarda recálculo manual em `.G`.

**Critério de saída**:
- L0 reflecte variant novo.
- Cross-references presentes.

### .D Adicionar stub no-op em `from_tags`

1. Em `01_core/src/rules/introspect/from_tags.rs`:
   - Localizar match sobre `ElementPayload`.
   - Adicionar arm:
     ```
     ElementPayload::Labelled { .. } => {},
     ```
   - Posicionar conforme convenção (provável: junto a
     `Equation` arm).

2. Comentário inline opcional (curto):
   ```
   // Stub no-op P195B — populate completo em P195C
   // (post-recursion pattern; vide ADR-XXXX).
   ```

3. Confirmar `@prompt-hash` actualiza após edit do L0
   `from_tags.md` em `.E`.

**Critério de saída**:
- `cargo check --workspace` passa (match exaustivo
  satisfeito).
- Stub no-op presente.

### .E Actualizar L0 `rules/introspect/from_tags.md`

1. Adicionar entrada Histórico:
   - P195B — stub no-op `ElementPayload::Labelled`
     adicionado (cláusula gate trivial).
   - P195C — populate completo (próximo passo).

2. Hash em branco aguarda recálculo manual em `.G`.

**Critério de saída**:
- L0 Histórico actualizado.

### .F Criar ADR PROPOSTO post-recursion-tag-emission

1. Localizar próximo número ADR per `.A.5`. Esperado:
   **ADR-0069**.

2. Criar
   `00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md`
   com 8 secções padrão:

   - **§1 Estado**: PROPOSTO.
   - **§2 Contexto**:
     - Walk arms cujo payload depende de state mutado
       durante walk recursivo do target não podem usar
       `extract_payload` (função pura, sem state).
     - Caso identificado: `Content::Labelled` com target
       Heading/Equation/Figure (P195A §11.1).
     - Padrão pré-existente (Bibliography, Equation,
       SetHeadingNumbering): payload independente de
       state; `extract_payload` chamado em walk top.
   - **§3 Decisão**:
     - Walk arm emite Tag **manualmente** em `tags` após
       recursão (não passa por `extract_payload`).
     - Payload pré-computed pelo walk arm com acesso ao
       state actual.
     - **Sem** `is_locatable=true` (não passa por
       mecanismo locatable).
     - **Sem** `ElementKind::*` (não conta para
       `kind_index`).
     - `from_tags` arm processa o payload normalmente.
   - **§4 Alternativas**:
     - Opção 1 padrão (locatable + extract_payload):
       impossível arquitecturalmente.
     - Opção 2 (StateUpdate): múltiplas Tags por
       Labelled; semântica não-natural; descartada per
       P195A §3 cláusula 1.
     - Opção 3 (variant não-locatable, sem ADR
       formalizado): essencialmente o que esta ADR
       formaliza.
   - **§5 Consequências**:
     - **Positivas**:
       - Permite migração de walk arms state-dependent
         para mecanismo Tag-based.
       - Sem refactor major do trait `extract_payload`.
       - Padrão aplicável a P196 Heading, P197 Figure,
         P198 SetHeadingNumbering+CounterUpdate.
     - **Negativas**:
       - Walk arm tem responsabilidade dupla (computar
         + emitir Tag).
       - Sem benefícios de `kind_index` (query por kind
         não funciona para Labelled).
       - Helper privado (`compute_labelled` per P195A
         §11.6) duplica lógica entre mutação legacy e
         populate Tag.
   - **§6 Critério de fecho**:
     - ADR transita PROPOSTO → ACEITE quando P195
       série fecha (P195E) com tests E2E confirmando
       paridade observable.
     - ADR transita PROPOSTO → REJEITADO se P195C/D
       revelarem bloqueador estrutural não previsto.
   - **§7 Aplicabilidade futura**:
     - P196 Heading walk arm — payload depende de
       counter formatting.
     - P197 Figure walk arm — payload depende de
       `figure_numbers`, `lang`.
     - P198 walks state-dependent.
   - **§8 Histórico**:
     - 2026-05-04: PROPOSTO em P195B.

3. Confirmar L0 `00_nucleo/adr/README.md` (ou
   equivalente) actualizado com entrada nova se
   convenção exigir.

**Critério de saída**:
- ADR-0069 (ou número real) criada com 8 secções.
- Status PROPOSTO.

### .G Tests unit do variant

3-4 tests obrigatórios. Padrão dos variants existentes
(per P186B `.F`).

#### Test 1 — `labelled_construivel`

1. Construir variant:
   ```
   let _ = ElementPayload::Labelled {
       label:         Label("intro".to_string()),
       resolved_text: Some("Capítulo 1".to_string()),
       figure_number: None,
   };
   ```
2. Compila ✅.

#### Test 2 — `labelled_equality`

1. 2 instâncias com mesmos campos.
2. Assert `==`.

#### Test 3 — `labelled_distincao_de_outras_variants`

1. `Labelled { ... } != Equation { block: true,
   counter_update: Step }`.
2. `Labelled { ... } != Heading { ... }` (se variant
   existe).

#### Test 4 — `labelled_hash_diferente_para_label_distinto`

1. Hash de `Labelled { label: A, ... }` ≠ Hash de
   `Labelled { label: B, ... }`.
2. Replica padrão P186B (hash via `format!("{:?}", self)`).

Tests co-localizados em `mod tests` de
`element_payload.rs`. Padrão P186B replicado.

**Critério de saída**:
- 3-4 tests novos passam.
- Tests existentes não regridem.

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P195A
   baseline (1.825): **+3 a +4** dependendo de cobertura
   `.G`.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Variant `ElementPayload::Labelled { label,
   resolved_text, figure_number }` construível.
5. Stub no-op `ElementPayload::Labelled { .. } => {}`
   presente em `from_tags`.
6. ADR-0069 (ou número real) criada com status PROPOSTO.
7. **`is_locatable(Content::Labelled)` continua `false`**
   — sem janela invariante quebrada.
8. **`extract_payload(Content::Labelled)` continua a
   retornar `None`** — catch-all intocado.
9. Walk arm Labelled **NÃO modificado** — preserva
   mutação legacy.
10. Trait `Introspector` **NÃO modificado** — P185B
    fechou.
11. `TagIntrospector` **NÃO modificado** — P193B fechou.
12. Consumer C4 **NÃO modificado** — P194B fechou.
13. Snapshot tests ADR-0033 verdes.
14. Linter passa final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-195b-relatorio.md`
com:

- Resumo: variant adicionado + stub no-op + ADR PROPOSTO;
  walk arm intacto.
- Confirmação `.H` (14 verificações).
- Δ tests vs baseline P195A (esperado +3 a +4).
- Hashes finais L0 modificados (2 — `entities/element_payload.md`
  + `rules/introspect/from_tags.md`).
- ADR-0069 (ou número real) criada — referência ao
  ficheiro.
- Decisões de execução notáveis:
  - Cláusula gate trivial em `from_tags` esperada
    (replica P186B).
  - ADR PROPOSTO formaliza pattern novo identificado
    P195A.
- Estado actual:
  - P195 série: A ✅ B ✅ | C-E pendentes.
  - `ElementPayload`: 10 → **11 variants**.
  - `ElementKind`: 9 (inalterado — sem locatable).
  - 66 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P195C (estender stub no-op com populate
  completo dos sub-stores).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Variant `ElementPayload::Labelled` adicionado.
3. L0 `entities/element_payload.md` actualizado.
4. Stub no-op em `from_tags`.
5. L0 `rules/introspect/from_tags.md` actualizado.
6. ADR-0069 PROPOSTO criada (8 secções).
7. 3-4 tests unit passam.
8. Tests existentes não regridem.
9. Verificações `.H` passam (14/14).
10. Relatório `.I` escrito.
11. Output observable em produção inalterado.

---

## O que pode sair errado

- **`Label` não tem `Hash` ou `Eq` derivados**: cláusula
  gate trivial — auditar em `.A.3`. Provável que tenha
  (P193A confirmou para sub-store).
- **`from_tags` não tem match exaustivo** (catch-all):
  cláusula gate trivial — stub no-op desnecessário; arm
  específico preferível. **Improvável** per P186B
  descoberta empírica.
- **Próximo número ADR ocupado** (alguma ADR criada
  entre P185E e P195B): cláusula gate trivial — usar
  número correcto.
- **Convenção L0 ADR diferente** do esperado: cláusula
  gate trivial — replicar shape empírica.
- **Tests existentes regridem por adição de variant**
  (algum lugar com match não-exaustivo): cláusula gate
  trivial — adicionar arms.
- **Snapshot tests divergem**: improvável (variant novo
  sem produtor). Se acontecer, investigar.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~30 LOC produção (variant + stub
  no-op + ADR) + ~50 LOC tests + ~150 LOC ADR + edits
  L0.
- **Sem dependências externas novas**.
- **Sem DEBT formal**.
- **Padrão replicado**: P186B (variant adition + stub
  no-op em from_tags por cláusula gate trivial).
- **ADR formal pela primeira vez desde P185E**.
- **Cláusula gate trivial**: aplicável a derives,
  posição do variant, convenção ADR.
- **Sem cláusula gate substancial esperada**.
- **Estado intermédio seguro**: variant existe; stub
  no-op em `from_tags` benigno; sem activação de
  caminho. Output observable preservado.
- **Próximo passo P195C**: estender stub no-op com
  populate completo dos sub-stores
  (`intr.resolved_labels` + `intr.figure_label_numbers`).
  Magnitude S esperada — replica P186E pattern com
  payload mais rico.
- **ADR-0069 transita PROPOSTO → ACEITE em P195E** (após
  tests E2E confirmarem paridade observable).
