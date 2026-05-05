# Passo P199B — Materialização `Content::SetEquationNumbering`

Primeiro passo de implementação P199 (após P199A
diagnóstico). Magnitude **M genuína** — replicação
literal do template P182C com chave `equation`.

P199A confirmou empiricamente:
- Variant `Content::SetEquationNumbering` ausente
  (apenas referenciada em comentários como Reserva 1).
- Template P182C totalmente mapeado.
- `from_tags` arm StateUpdate (P171) é genérica — sem
  hardcoded `"heading"`.
- **Layouter Equation** (`equation.rs:32-33`) **já
  tem substitution-with-fallback implementada** —
  caminho Introspector dorme à espera da variant.
- Sem parser sintáctico — Opção α confirmada (apenas
  materialização interna).

P199B materializa a variant. Caminho Introspector
activa **imediatamente em produção real** após
materialização (cenário α por construção — sub-variante
de cenário α).

Trabalho concreto:
1. Adicionar variant `Content::SetEquationNumbering
   { active: bool }` em `entities/content.rs`.
2. Cobrir match arms exaustivos induzidos pela adição
   (per padrão P198C — adaptação de match exhaustivos
   em vários ficheiros).
3. Adicionar arm `is_locatable(Content::SetEquationNumbering)
   = true` em `locatable.rs`.
4. Adicionar arm `extract_payload(Content::SetEquationNumbering)`
   retornando `Some(ElementPayload::StateUpdate { key:
   "numbering_active:equation", update:
   Set(Bool(active)) })`.
5. Walk arm em `introspect.rs` mutando
   `state.numbering_active.insert("equation".to_string(),
   *active)` + comentário inline P199B.
6. **Não modificar** `from_tags` arm StateUpdate (P171
   genérica).
7. Comentário DEBT-10 no variant per sugestão do
   auditor (alinhamento com vanilla StyleChain futuro).
8. L0 actualizado: tabela Excepções M5 (E1 → fechada
   estruturalmente); secção nova "Walk arm
   SetEquationNumbering migrado (P199B, cenário α por
   construção)".
9. 5 tests E2E cobrir activação + paridade + cadeia E1.

Após P199B:
- E1 declarada formalmente fechada **estruturalmente**.
- Caminho Introspector activa em produção real
  (Layouter Equation first branch start firing).
- `intr.state["numbering_active:equation"]` populated
  via Tag::StateUpdate.
- Mutação legacy preservada — walk arm Equation +
  `compute_labelled` Equation arm continuam
  funcionais.
- DEBT M5-residual: 1 excepção + 1 residuo → **0
  excepções + 1 residuo** (E2-residuo); 1 pré-requisito
  restante (`headings_for_toc`).
- **`Content` enum**: + 1 variant.
- **Marco arquitectural**: M5 universal a 1 passo
  paralelo do fecho.

**Pré-condição**: P199A concluído. Tests workspace
1.859 verdes; zero violations. 7 cláusulas P199A
fechadas. Cenário α por construção identificado.

**Restrições**:
- **Não** materializar parser sintáctico — fora de
  escopo per P199A cláusula 2 Opção α.
- **Não** modificar `from_tags` arm StateUpdate (P171
  genérica).
- **Não** modificar Layouter `equation.rs:32-33`
  (substitution-with-fallback já implementada;
  activa por construção).
- **Não** modificar trait `Introspector` (P185B
  fechou).
- **Não** modificar `TagIntrospector` (P193B fechou).
- **Não** modificar `compute_labelled` Equation arm
  (P195D) — continua a ler legacy.
- **Não** modificar walk arm Equation (P186) —
  continua a ler legacy.
- **Manter mutação legacy** —
  `state.numbering_active.insert("equation", ...)`
  necessária durante janela compat M5.
- **Não** abrir sub-store `headings_for_toc` — passo
  paralelo independente.
- API pública preservada (adição de variant é
  retrocompatível).

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `Content::SetHeadingNumbering` (template):
   - `01_core/src/entities/content.rs` — localizar
     variant.
   - Forma exacta (campo `active: bool`).

2. Confirmar `Content::SetEquationNumbering` ausente:
   - `grep -rn "SetEquationNumbering" 01_core/src/`
     retorna zero (excepto comentários Reserva 1).

3. Confirmar arm `is_locatable(SetHeadingNumbering)`:
   - `01_core/src/rules/introspect/locatable.rs:49`
     (per P199A §3).
   - Linha exacta para inserir arm análogo.

4. Confirmar arm `extract_payload(SetHeadingNumbering)`:
   - `01_core/src/rules/introspect/extract_payload.rs:63`
     (per P199A §3).
   - Forma exacta com chave `numbering_active:heading`.

5. Confirmar walk arm `SetHeadingNumbering`:
   - `01_core/src/rules/introspect.rs:611` (per
     P199A §3).
   - Comentário inline P198B presente — usar como
     template.

6. Confirmar `from_tags` arm StateUpdate (P171):
   - `01_core/src/rules/introspect/from_tags.rs` (ou
     similar).
   - Genérica — não tem hardcoded keys.

7. Identificar match arms exaustivos induzidos pela
   adição (per padrão P198C):
   - `Content::to_string` ou método similar.
   - Comparações `eq` / `partial_cmp`.
   - `materialize_time` em `introspect.rs`.
   - Lista de "terminais sem effect em counters" no
     walk match.
   - `cargo check` revelará outros sítios via
     non-exhaustive match warnings.

8. Confirmar Layouter Equation
   `equation.rs:32-33`:
   - Substitution-with-fallback implementada.
   - Caminho dorme — first branch retorna `None` até
     `intr.state["numbering_active:equation"]`
     populated.
   - **Não modificar**.

9. Confirmar `compute_labelled` Equation arm (P195D):
   - Lê `state.is_numbering_active("equation")` +
     `state.get_flat("equation")` durante walk.
   - **Não modificar**.

10. Confirmar walk arm Equation (P186):
    - Lê `state.is_numbering_active("equation")` para
      gating.
    - **Não modificar**.

11. Confirmar L0 alvos:
    - `entities/content.md` — variant nova.
    - `rules/introspect.md` — tabela Excepções M5;
      ordem inversa; secção nova.

Output: tabela com item + estado + linhas exactas.

**Critério de saída**:
- Template P182C totalmente mapeado.
- Match arms induzidos identificados.
- Cadeia E1 confirmada (mutação legacy obrigatória).

### .B Adicionar variant `Content::SetEquationNumbering`

1. Em `01_core/src/entities/content.rs`:
   - Adicionar variant após `SetHeadingNumbering`
     (per convenção cronológica):
     ```
     /// Activa ou desactiva a numeração automática
     /// de equations. Análoga a SetHeadingNumbering
     /// (P57). Materializada em P199B.
     /// DEBT-10: substituir por StyleChain quando
     /// motor de introspecção completo for
     /// implementado.
     SetEquationNumbering { active: bool },
     ```
   - Forma exacta replica `SetHeadingNumbering`.

2. Confirmar `@prompt-hash` actualiza após edit do L0
   `entities/content.md` em `.H`.

**Critério de saída**:
- Variant declarável.
- `cargo check --workspace` falha (esperado — match
  exhaustivos não cobertos ainda).

### .C Cobrir match arms exaustivos induzidos

Conforme `.A.7` + `cargo check` warnings:

1. Adicionar arms para `SetEquationNumbering` em todos
   os match exaustivos relevantes que `cargo check`
   reportar.

2. Padrão típico (replica `SetHeadingNumbering`):
   - `Content::to_string`: retorna `"set equation
     numbering"` ou similar.
   - Comparações `eq`/`partial_cmp`: replica
     SetHeadingNumbering.
   - `materialize_time`: replica.
   - "Terminais sem effect em counters" no walk match:
     incluir SetEquationNumbering como
     SetHeadingNumbering.

3. Confirmar `cargo check --workspace` passa após
   adições.

**Critério de saída**:
- `cargo check --workspace` passa.
- Match arms exaustivos cobertos.
- Sem regressão em comportamento existente
  (SetEquationNumbering ainda não tem semântica
  activa — apenas declarada).

### .D Adicionar arm `is_locatable`

1. Em `01_core/src/rules/introspect/locatable.rs`:
   - Adicionar arm após `Content::SetHeadingNumbering
     { .. } => true`:
     ```
     Content::SetEquationNumbering { .. } => true,
     ```

2. Tests existentes em `locatable.rs` podem regridir
   (totals mudam — paralelo a P198C `.D`). Aplicar
   padrão pragmático auditor #1 (ajustar fixture).

**Critério de saída**:
- `is_locatable(Content::SetEquationNumbering)` retorna
  `true`.
- Tests existentes adaptados se necessário.
- `cargo check --workspace` passa.

### .E Adicionar arm `extract_payload`

1. Em `01_core/src/rules/introspect/extract_payload.rs`:
   - Adicionar arm após
     `Content::SetHeadingNumbering` arm:
     ```
     Content::SetEquationNumbering { active } => Some(
         ElementPayload::StateUpdate {
             key:    "numbering_active:equation".to_string(),
             update: StateUpdate::Set(Value::Bool(*active)),
         },
     ),
     ```
   - Forma exacta replica P182C com chave `equation`.

2. Confirmar `@prompt-hash` actualiza após edit do L0.

**Critério de saída**:
- Arm adicionada.
- `cargo check --workspace` passa.

### .F Adicionar walk arm `SetEquationNumbering`

1. Em `01_core/src/rules/introspect.rs`:
   - Adicionar arm após walk arm `SetHeadingNumbering`
     (linha ~624 per P198B):
     ```
     Content::SetEquationNumbering { active } => {
         // P199B — E1 fechada estruturalmente
         // (cenário α por construção).
         //
         // Caminho Introspector activado por
         // construção desde a materialização:
         // extract_payload → ElementPayload::StateUpdate
         // sob chave numbering_active:equation →
         // from_tags arm StateUpdate (P171) popula
         // StateRegistry. Layouter equation.rs:32-33
         // first branch (substitution-with-fallback)
         // activa em produção real.
         //
         // Mutação legacy preservada como write
         // paralelo M5: walk arm Equation (P186) +
         // compute_labelled Equation arm (P195D)
         // lêem state.is_numbering_active("equation")
         // durante walk para gating + format.
         // Cleanup orgânico em M6.
         state.numbering_active.insert("equation".to_string(), *active);
     }
     ```
   - Mutação legacy obrigatória — write paralelo M5.

2. Confirmar `@prompt-hash` actualiza.

**Critério de saída**:
- Walk arm presente.
- Mutação legacy obrigatória aplicada.
- Comentário inline P199B presente.
- `cargo check --workspace` passa.

### .G Confirmar `from_tags` arm StateUpdate sem modificação

1. Confirmar empiricamente que arm StateUpdate
   processa `numbering_active:equation`
   transparentemente (per P199A §3 — genérica).

2. Sem modificação a `from_tags.rs`.

**Critério de saída**:
- `from_tags` arm StateUpdate inalterado.
- Pipeline empírico processa Tag::StateUpdate com chave
  `numbering_active:equation` correctamente.

### .H Actualizar L0

1. `entities/content.md`:
   - Adicionar entrada para variant
     `SetEquationNumbering`.
   - Cross-reference: P57 (template), P182C
     (SetHeadingNumbering precedente), P199B
     (este passo), DEBT-10 (futuro).

2. `rules/introspect.md`:
   - Tabela "Excepções M5" — actualizar entrada E1:
     - Estado: **fechada estruturalmente** (P199B —
       cenário α por construção).
     - Variant materializada.
     - Mutação legacy preservada como write paralelo
       M5.
     - Cleanup em M6.
   - Lista "Ordem inversa à mutação" — passo 9
     marcado ✅ (P199B).
   - Secção nova "Walk arm SetEquationNumbering
     migrado (P199B, cenário α por construção)" —
     análoga a "Walk arm SetHeadingNumbering migrado
     (P198B, cenário α)".
   - Cross-references: P57, P171, P173, P182C,
     P186E, P195D, P198B, ADR-0069.

3. Hash em branco aguarda recálculo manual em `.J`.

**Critério de saída**:
- L0s actualizados.
- Cross-references presentes.

### .I Tests E2E

5 tests obrigatórios per P199A §12.

#### Test 1 — `set_equation_numbering_extract_payload_emite_state_update`

1. Construir `Content::SetEquationNumbering { active:
   true }`.
2. Chamar `extract_payload(content)`.
3. Asserções:
   - Retorna `Some(ElementPayload::StateUpdate { key:
     "numbering_active:equation", update: Set(Bool(true)) })`.
   - **Confirma materialização P199B**.

#### Test 2 — `set_equation_numbering_from_tags_popula_state_registry`

1. Construir Tag manualmente:
   ```
   Tag::Start(loc(0), ElementInfo::new(
       ElementPayload::StateUpdate {
           key: "numbering_active:equation".to_string(),
           update: StateUpdate::Set(Value::Bool(true)),
       },
   ))
   ```

2. Pipeline: `from_tags(vec![tag, ...])`.

3. Asserções:
   - `intr.state.get("numbering_active:equation")`
     retorna valor correcto.
   - **Confirma `from_tags` arm StateUpdate genérica
     (P171) processa chave `equation` transparentemente**.

#### Test 3 — `set_equation_numbering_paridade_legacy_vs_introspector`

1. Documento com SetEquationNumbering + Equation:
   ```
   Content::Sequence(vec![
       Content::SetEquationNumbering { active: true },
       Content::Equation { /* ... */ },
   ])
   ```

2. Pipeline: walk + from_tags → state legacy +
   TagIntrospector populated.

3. Asserções de paridade:
   - `state.is_numbering_active("equation") == true`.
   - `intr.is_numbering_active(...)` ou similar
     retorna idêntico.
   - Confirma write paralelo legacy + Introspector.

#### Test 4 — `walk_arm_equation_le_numbering_active_legacy_apos_set`

Sentinela cláusula gate substancial cadeia E1 ↔
walk arm Equation.

1. Documento com SetEquationNumbering + Equation
   numerada.
2. Asserções:
   - Walk arm Equation lê
     `state.is_numbering_active("equation")` durante
     walk (per P186 mecanismo).
   - Counter `state.flat["equation"]` avança quando
     `numbering_active:equation = true`.
   - **Confirma cadeia E1 funcional após
     materialização** — mutação legacy obrigatória.

#### Test 5 — `consumer_layouter_equation_recebe_some_via_introspector`

1. Documento com SetEquationNumbering + Equation +
   Ref(eq1):
   ```
   Content::Sequence(vec![
       Content::SetEquationNumbering { active: true },
       Content::Labelled {
           label: Label("eq1".into()),
           target: Box::new(Content::Equation { /* ... */ }),
       },
       Content::Ref(Label("eq1".into())),
   ])
   ```

2. Pipeline real (walk + from_tags + layout).

3. Asserções:
   - Layouter `equation.rs:32-33` first branch
     retorna `Some(...)` via Introspector path.
   - Fallback legacy não chamado mas continua
     funcional como backup.
   - **Confirma activação Introspector path em
     produção real para Equation** —
     substitution-with-fallback implementada
     anteriormente activa por construção.

Tests co-localizados em submódulo
`p199b_set_equation_numbering` em `tests.rs`.

**Critério de saída**:
- 5 tests passam.
- Tests existentes não regridem.

### .J Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P199A
   baseline (1.859): **+5**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p199b_set_equation_numbering::*` passam
   isoladamente.
5. Tests existentes (incluindo sentinela E1 P189B,
   tests P171, P182C, P186, P195D Equation arm) **não
   regridem** — mutação legacy preservada.
6. Tests adaptados via padrão pragmático auditor #1
   (se necessário em `.D`).
7. Variant `Content::SetEquationNumbering` presente
   no enum.
8. Comentário DEBT-10 presente no variant.
9. `is_locatable(Content::SetEquationNumbering) =
   true`.
10. `extract_payload(Content::SetEquationNumbering)`
    retorna `Some(ElementPayload::StateUpdate { ... })`
    com chave canónica.
11. Walk arm presente com **comentário inline P199B**.
12. Mutação legacy preservada
    (`state.numbering_active.insert("equation", ...)`).
13. `from_tags` arm StateUpdate **NÃO modificado**.
14. Layouter `equation.rs:32-33` **NÃO modificado**.
15. `compute_labelled` Equation arm (P195D) **NÃO
    modificado**.
16. Walk arm Equation (P186) **NÃO modificado**.
17. **L0 entries novas** presentes
    (`entities/content.md` + `rules/introspect.md`).
18. **Tabela Excepções M5** com E1 marcada "fechada
    estruturalmente".
19. Trait `Introspector` NÃO modificado.
20. Snapshot tests verdes.
21. Linter passa final.

### .K Encerramento

Escrever
`00_nucleo/materialization/typst-passo-199b-relatorio.md`
com:

- Resumo: variant materializada; cenário α por
  construção; E1 fechada estruturalmente; Layouter
  Equation activa em produção real.
- Confirmação `.J` (21 verificações).
- Δ tests vs baseline P199A (esperado +5).
- Hashes finais L0 (`entities/content.md` +
  `rules/introspect.md`).
- Decisões de execução notáveis:
  - DEBT-10 introduzida no comentário do variant.
  - Match arms exaustivos induzidos identificados e
    cobertos.
  - Tests existentes adaptados se necessário.
- Estado actual:
  - P199 série: A ✅ B ✅ | C pendente.
  - **E1 fechada estruturalmente** (cenário α por
    construção).
  - 82 passos executados.
- Pendências cumulativas: 0 excepções activas + 1
  residuo (E2-residuo).
- Próximo passo: P199C — relatório consolidado P199 +
  actualização DEBT M5-residual.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. Variant `Content::SetEquationNumbering` adicionada
   (`.B`).
3. Match arms exaustivos cobertos (`.C`).
4. `is_locatable` arm adicionada (`.D`).
5. `extract_payload` arm adicionada (`.E`).
6. Walk arm adicionada com comentário inline (`.F`).
7. `from_tags` arm StateUpdate inalterado (`.G`).
8. L0s actualizados (`.H`).
9. 5 tests E2E passam (`.I`).
10. Verificações `.J` passam (21/21).
11. Tests existentes não regridem ou são adaptados.
12. Mutação legacy preservada.
13. Output observable em produção alterado **apenas
    para activar** Layouter Equation first branch
    (substitution-with-fallback antes adormecida).
14. Relatório `.K` escrito.

---

## O que pode sair errado

- **`from_tags` arm StateUpdate tem hardcoded keys**
  (improvável per P199A): cláusula gate substancial —
  adicionar caso `numbering_active:equation`.
- **Layouter `equation.rs:32-33` tem expectations
  diferentes do esperado** (formato chave, semântica):
  cláusula gate substancial — investigar
  empiricamente.
- **Match arms exaustivos divergem do esperado**
  (ficheiros não previstos): cláusula gate trivial —
  cobrir conforme `cargo check` warnings.
- **Test 4 (cadeia E1 ↔ walk arm Equation) falha**:
  indica que mutação legacy não foi preservada
  correctamente. Cláusula gate substancial.
- **Test 5 (Layouter activação) falha**: indica que
  substitution-with-fallback antes adormecida não
  activa em produção real apesar de Tag emitida.
  Cláusula gate substancial — investigar
  `equation.rs:32-33` empiricamente.
- **Tests existentes regridem por mudança em
  is_locatable totals**: cláusula gate trivial —
  ajustar fixture.
- **Snapshot tests divergem**: pode acontecer se
  Layouter Equation activa first branch e produz
  output ligeiramente diferente. Investigar — pode
  ser regressão ou correcção esperada.
- **Linter divergência V13/V14**: cláusula gate
  trivial — `--fix-hashes`.
- **Comentário DEBT-10 conflita com convenção
  existente**: cláusula gate trivial — adaptar formato.

---

## Notas operacionais

- **Tamanho**: M genuíno. ~50 LOC produção (variant +
  3 arms + walk arm + match arms induzidos) + ~120
  LOC tests + ~60 LOC L0.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **DEBT-10 introduzida** (StyleChain futuro per
  vanilla typst) — auditor pode formalizar como entry
  no `m1-lacunas-captura.md` ou similar.
- **Padrão materializado**: cenário α por construção —
  sub-variante de cenário α (P197B/P198B). Variant
  nova com infraestrutura downstream pronta; activação
  imediata.
- **Cláusula gate trivial**: aplicável a match arms
  induzidos, recálculo de hashes, formato comentário.
- **Cláusula gate substancial**: aplicável a:
  - `from_tags` arm StateUpdate diverge da expectativa.
  - Layouter `equation.rs` diverge da expectativa.
  - Cadeia E1 quebra.
  - Test 5 (activação em produção real) falha.
- **Distinção arquitectural notável**: Layouter
  Equation tem **substitution-with-fallback antes
  adormecida** — caminho Introspector planeado
  antecipadamente (provavelmente em P186E ou similar)
  e activa por construção quando variant materializa.
  Padrão arquitectural reusável para casos futuros
  análogos.
- **Próximo passo P199C**: relatório consolidado P199
  (9 secções) + actualização DEBT M5-residual.
  Magnitude S puro.
- **Estado pós-P199C**: M5 universal a 1 passo
  paralelo do fecho — sub-store `headings_for_toc`
  fecha E2-residuo. Após esse passo, M5 universal
  completo desbloqueia M6 (P190A reescrita do zero).
