# Passo P198B — Walk arm SetHeadingNumbering (cenário α)

Primeiro passo de implementação P198 (após P198A
diagnóstico). Magnitude **S puro** — declaração formal
cenário α para E5; sem refactor de código produção.

P198A confirmou que **E5 está em cenário α** desde
P182C (StateUpdate via `extract_payload` →
`from_tags` arm popula StateRegistry). Walk arm
SetHeadingNumbering apenas muta legacy `numbering_active`
para preservar consumers `compute_heading_auto_toc`
(P196B) e walk arm Equation. Caminho Introspector já
activo em produção.

P198B **não modifica walk arm** — diferente de P197B
que extraiu helper. Razão: walk arm SetHeadingNumbering
é trivial (1 linha de mutação); helper estilístico não
agrega valor.

Trabalho concreto:
1. Adicionar comentário inline P198B no walk arm
   declarando E5 fechada estruturalmente via cenário α.
2. Actualizar L0 (tabela Excepções M5: E5 → fechada
   estruturalmente; secção nova).
3. 5 tests sentinela cobrir cenário α.

Após P198B:
- E5 declarada formalmente fechada **estruturalmente**.
- Caminho Introspector inalterado (já activo desde
  P182C).
- Mutação legacy preservada — `compute_heading_auto_toc`
  + walk arm Equation continuam funcionais.
- DEBT M5-residual: 3 excepções + 1 residuo → **2
  excepções + 1 residuo** (E1, E6, E2-residuo); 2
  pré-requisitos restantes inalterados.

**Pré-condição**: P198A concluído. Tests workspace
1.848 verdes; zero violations. 9 cláusulas P198A
fechadas. Cenário α confirmado para E5.

**Restrições**:
- **Não** emitir Tag pós-recursão — pattern ADR-0069
  dispensado per cenário α.
- **Não** modificar variant `ElementPayload::StateUpdate`
  (P171 fechou).
- **Não** modificar `from_tags` arm StateUpdate (P171
  cobre).
- **Não** modificar trait `Introspector` (P185B
  fechou).
- **Não** modificar `TagIntrospector` (P193B fechou).
- **Não** modificar consumer C3/C4.
- **Não** modificar `compute_heading_auto_toc` (P196B)
  ou outros helpers — continuam a ler legacy.
- **Manter mutação legacy** —
  `state.numbering_active.insert("heading", ...)`
  necessária durante janela compat M5.
- **Não** migrar walk arm CounterUpdate — P198C.
- API pública preservada.
- Output observable em produção **inalterado** —
  caminho Introspector já activo desde P182C.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar walk arm SetHeadingNumbering em
   `01_core/src/rules/introspect.rs:611-623` (per
   P198A §3):
   - Re-verificar empiricamente.
   - Localizar mutação `state.numbering_active.insert("heading".to_string(), *active)`.

2. Confirmar `Content::SetHeadingNumbering` variant:
   - Campo: `active: bool` ou similar.

3. Confirmar `is_locatable(Content::SetHeadingNumbering)
   = true` (P182C).

4. Confirmar `extract_payload(Content::SetHeadingNumbering)`
   retorna `Some(ElementPayload::StateUpdate { ... })`
   (P182C).

5. Confirmar `from_tags` arm StateUpdate (P171) popula
   StateRegistry.

6. Confirmar consumer downstream:
   - `compute_heading_auto_toc` (P196B) lê
     `state.is_numbering_active("heading")` —
     mutação legacy obrigatória.
   - Walk arm Equation lê
     `state.is_numbering_active("equation")` —
     mutação legacy obrigatória.

7. Confirmar L0 `rules/introspect.md`:
   - Tabela "Excepções M5" — actualizar entrada E5.
   - Secção nova "Walk arm SetHeadingNumbering migrado
     (P198B, cenário α)" — análoga a "Walk arm Figure
     migrado (P197B, cenário α)".

8. Confirmar tests existentes:
   - Sentinela E5 P189B (per `.D` ponto 3 do P189B).
   - Tests P171/P182 que cobrem StateUpdate flow.
   - Identificar quais devem manter-se inalterados.

Output: tabela com item + estado + linhas exactas.

**Critério de saída**:
- Walk arm SetHeadingNumbering localizado.
- Cenário α confirmado.
- Mutação legacy preservada estabelecida obrigatória.

### .B Adicionar comentário inline P198B no walk arm

1. Em `01_core/src/rules/introspect.rs:611-623` (per
   `.A.1`):
   - **Não modificar** mutação legacy.
   - Adicionar comentário inline curto:
     ```
     // P198B — E5 fechada estruturalmente (cenário α).
     // Caminho Introspector já activo desde P182C
     // (extract_payload → ElementPayload::StateUpdate
     // → from_tags arm popula StateRegistry).
     // Mutação legacy preservada como write paralelo
     // M5: compute_heading_auto_toc (P196B) + walk
     // arm Equation lêem state.numbering_active
     // durante walk. Cleanup orgânico em M6.
     ```

2. Confirmar `@prompt-hash` actualiza após edit do L0
   em `.C`.

**Critério de saída**:
- Comentário inline presente.
- `cargo check --workspace` passa.
- Sem regressão (zero código mudado).

### .C Actualizar L0 `rules/introspect.md`

1. Tabela "Excepções M5" — actualizar entrada E5:
   - Estado: **fechada estruturalmente** (P198B).
   - Cenário α — caminho Introspector activo desde
     P182C.
   - Mutação legacy preservada como write paralelo M5.
   - Cleanup em M6.

2. Adicionar secção "Walk arm SetHeadingNumbering
   migrado (P198B, cenário α)":
   - Caminho Introspector já activo desde P182C.
   - Sem helper extraído (mutação trivial — 1 linha).
   - Mutação legacy preservada para
     `compute_heading_auto_toc` (P196B) + walk arm
     Equation.
   - Cross-references: P171 (StateUpdate variant),
     P182C (locatable + extract_payload), P196B
     (consumer), ADR-0069 (cenário α stylesheet).

3. Actualizar lista "Ordem inversa à mutação":
   - Passos 1-7 marcados ✅ (P193B, P194B, P195D,
     P196B, P197B, **P198B**).
   - Passo 8 novo (P198C — CounterUpdate).

4. Hash em branco aguarda recálculo manual em `.E`.

**Critério de saída**:
- L0 reflecte declaração formal.
- Tabela Excepções M5 actualizada.
- Secção nova presente.

### .D Tests sentinela cenário α

5 tests obrigatórios per P198A §12.

#### Test 1 — `set_heading_numbering_extract_payload_emite_state_update`

1. Construir `Content::SetHeadingNumbering { active:
   true }`.
2. Chamar `extract_payload(content)`.
3. Asserções:
   - Retorna `Some(ElementPayload::StateUpdate { key,
     update })` com `key == "numbering_active:heading"`
     ou similar.
   - **Confirma caminho Introspector activo desde
     P182C** (não dependente de P198B).

#### Test 2 — `set_heading_numbering_from_tags_popula_state_registry`

1. Construir Tag manualmente:
   ```
   Tag::Start(loc(0), ElementInfo::new(
       ElementPayload::StateUpdate {
           key: "numbering_active:heading".to_string(),
           update: StateUpdate::Set(true),
       },
   ))
   ```

2. Pipeline: `from_tags(vec![tag, ...])`.

3. Asserções:
   - `intr.state.get("numbering_active:heading")`
     retorna `Some(true)` ou similar.
   - **Confirma `from_tags` arm StateUpdate funcional
     desde P171**.

#### Test 3 — `set_heading_numbering_paridade_legacy_vs_introspector`

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

2. Pipeline: walk + from_tags → state legacy +
   TagIntrospector populated.

3. Asserções de paridade:
   - `state.is_numbering_active("heading") == true`.
   - `intr.state.get("numbering_active:heading")`
     retorna `Some(true)` ou similar.
   - Confirma write paralelo legacy + Introspector.

#### Test 4 — `compute_heading_auto_toc_le_numbering_active_legacy`

1. Mesmo documento.
2. Confirmar empiricamente que
   `compute_heading_auto_toc` (P196B) chama
   `state.is_numbering_active("heading")` durante walk.
3. Asserção:
   - Quando `state.numbering_active = false`
     (SetHeadingNumbering NÃO emitido):
     `compute_heading_auto_toc` retorna `(label,
     "")` per P196B §3 paridade legacy.
   - Quando `state.numbering_active = true`:
     `compute_heading_auto_toc` retorna `(label,
     "Secção 1")`.
   - **Confirma mutação legacy obrigatória — consumer
     compute_heading_auto_toc depende**.

#### Test 5 — `walk_arm_preserva_write_paralelo_legacy_para_compute_helpers`

Sentinela cláusula gate substancial cadeia E5 ↔
helpers.

1. Documento com SetHeadingNumbering + Heading + Ref:
   ```
   Content::Sequence(vec![
       Content::SetHeadingNumbering { active: true },
       Content::Heading {
           level: 1,
           body: Box::new(Content::Text("Intro".into())),
       },
       Content::Ref(Label("auto-toc-1".into())),
   ])
   ```

2. Pipeline real (walk + from_tags + layout).

3. Asserções:
   - `state.numbering_active.get("heading")` retorna
     `Some(&true)` (mutação legacy preservada).
   - `intr.resolved_labels.get(&Label("auto-toc-1".into()))`
     retorna `Some("Secção 1")` (auto-toc P196B
     funcional).
   - Consumer C4 (P194B) recebe `Some("Secção 1")`.
   - **Confirma cadeia E5 ↔ E2 (Heading auto-toc)
     funcional após declaração formal**.

Tests co-localizados em submódulo `p198b_set_heading_numbering`
em `tests.rs`.

**Critério de saída**:
- 5 tests passam.
- Tests existentes não regridem.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P198A
   baseline (1.848): **+5**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p198b_set_heading_numbering::*` passam
   isoladamente.
5. Tests existentes (incluindo sentinela E5 P189B,
   tests P171, P182C) **não regridem** — mutação
   legacy preservada; sem mudança a código produção.
6. Walk arm SetHeadingNumbering com **comentário
   inline P198B** presente.
7. **Mutação legacy preservada** —
   `state.numbering_active.insert("heading", ...)`
   continua.
8. `extract_payload` arm SetHeadingNumbering NÃO
   modificado (P182C).
9. `from_tags` arm StateUpdate NÃO modificado (P171).
10. Variant `ElementPayload::StateUpdate` NÃO
    modificado.
11. **L0 secção nova "Walk arm SetHeadingNumbering
    migrado (P198B, cenário α)"** presente.
12. **Tabela Excepções M5** com E5 marcada "fechada
    estruturalmente".
13. `compute_heading_auto_toc` (P196B) NÃO modificado.
14. Trait `Introspector` NÃO modificado.
15. Consumer C3 (P184D) / C4 (P194B) NÃO modificados.
16. Snapshot tests verdes.
17. Linter passa final.

### .F Encerramento

Escrever
`00_nucleo/materialization/typst-passo-198b-relatorio.md`
com:

- Resumo: declaração formal cenário α; comentário inline
  + L0 + 5 tests sentinela; sem refactor de código.
- Confirmação `.E` (17 verificações).
- Δ tests vs baseline P198A (esperado +5).
- Hashes finais L0 (`rules/introspect.md`).
- Decisões de execução notáveis.
- Estado actual:
  - P198 série: A ✅ B ✅ | C-D pendentes.
  - **E5 fechada estruturalmente** (cenário α).
  - 77 passos executados.
- Pendências cumulativas: 2 excepções activas + 1
  residuo (E1, E6, E2-residuo).
- Próximo passo: P198C — promote CounterUpdate
  (cenário β-promote, magnitude M).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. Comentário inline P198B presente no walk arm
   (`.B`).
3. L0 `rules/introspect.md` actualizado (`.C`).
4. 5 tests sentinela passam (`.D`).
5. Verificações `.E` passam (17/17).
6. Tests existentes não regridem (paridade observable
   preservada por construção).
7. Output observable em produção inalterado (sem
   código produção tocado além do comentário).
8. Relatório `.F` escrito.

---

## O que pode sair errado

- **Site walk arm SetHeadingNumbering mudou** entre
  P198A e P198B (improvável): cláusula gate trivial.
- **Test 1 (`extract_payload`) falha** porque assinatura
  do StateUpdate diverge: cláusula gate trivial —
  ajustar key/update format.
- **Test 4 (`compute_heading_auto_toc` lê legacy)
  falha**: cláusula gate substancial — re-verificar
  comportamento empírico de P196B helper.
- **Test 5 (cadeia E5↔E2) falha**: indica que
  alteração afectou paridade. Cláusula gate
  substancial. **Risco baixo** (sem código produção
  modificado).
- **Tests existentes regridem por mudança a comentário
  apenas**: improvável — investigar se acontecer.
- **Snapshot tests divergem**: improvável.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~10 LOC comentário + ~80 LOC
  tests + ~50 LOC L0 + relatório.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão materializado**: cenário α aplicado pela
  segunda vez (após P197B). Refactor estilístico
  dispensado porque mutação trivial (1 linha) não
  beneficia de helper.
- **Cláusula gate trivial**: aplicável a forma de
  comentário, formato L0, recálculo de hashes.
- **Cláusula gate substancial**: aplicável a:
  - Cadeia E5 ↔ helpers `compute_*` quebrar
    (mutação legacy não preservada).
  - Tests sentinela E5 P189B regridem.

  **Risco baixo** (sem código produção modificado).
- **Próximo passo P198C**: cenário β-promote (M
  genuíno) — adicionar variant `ElementPayload::CounterUpdate`,
  promover Content::CounterUpdate a locatable, adicionar
  arms `extract_payload` + `from_tags`. Magnitude
  similar a P195B + P195C combinados.
- **Distinção de P197B**: P197B extraiu helper
  estilístico (`compute_figure`); P198B não — mutação
  trivial dispensa. Cenário α aceita ambas formas (com
  ou sem helper).
