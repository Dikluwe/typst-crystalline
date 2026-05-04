# Passo P198C — Promote `Content::CounterUpdate` (β-promote)

Segundo passo de implementação P198 (após P198A
diagnóstico, P198B declaração formal cenário α para E5).
Magnitude **M genuína** — primeira aplicação do **cenário
β-promote** (variante operacional nova consolidada em
P198A).

P198A confirmou empiricamente que `Content::CounterUpdate`
não tem nada do mecanismo locatable:
- Variant `ElementPayload::CounterUpdate` ❌ não existe.
- `is_locatable(CounterUpdate)` ❌ false.
- `extract_payload` arm ❌ não existe.
- `from_tags` arm ❌ não existe.
- CounterRegistry ❌ não populated via CounterUpdate.

P198C **promove** `Content::CounterUpdate` a locatable +
adiciona variant + adiciona 2 arms (`extract_payload` +
`from_tags`). Diferente de P196B/P195D porque
CounterUpdate é **leaf content sem body recursivo** —
Tag emit pré-recursão é trivial; sem necessidade de
helper estilístico ou snapshot+find_map.

Trabalho concreto:
1. Adicionar variant
   `ElementPayload::CounterUpdate { key, action }` ao
   enum.
2. Activar `is_locatable(Content::CounterUpdate) = true`.
3. Adicionar arm em `extract_payload(Content::CounterUpdate)`
   que retorna `Some(ElementPayload::CounterUpdate { ... })`.
4. Adicionar arm em `from_tags` que processa
   `ElementPayload::CounterUpdate` aplicando
   `apply_at` (flat counter) ou
   `apply_hierarchical_at` (hierarchical counter)
   conforme `action`.
5. Walk arm preservado — mutação legacy continua como
   write paralelo M5.
6. Comentário inline P198C no walk arm.
7. L0 actualizado: tabela Excepções M5 (E6 → fechada
   estruturalmente); secção nova "Walk arm CounterUpdate
   migrado (P198C, β-promote)".
8. 5-6 tests E2E cobrir promotion + paridade + cadeia.

Após P198C:
- E6 fechada estruturalmente — caminho Introspector
  activo para CounterUpdate.
- `ElementPayload`: 11 → 12 variants.
- `ElementKind`: 9 → 10 (adicionar `ElementKind::CounterUpdate`
  se convenção exigir).
- `is_locatable(Content::CounterUpdate)`: false → true.
- Mutação legacy preservada — `compute_*` helpers
  continuam a ler counters durante walk.
- DEBT M5-residual: 2 excepções + 1 residuo → **1
  excepção + 1 residuo** (E1, E2-residuo); 2
  pré-requisitos restantes inalterados.

**Pré-condição**: P198B concluído. Tests workspace 1.853
verdes; zero violations. E5 declarada fechada
estruturalmente.

**Restrições**:
- **Manter mutação legacy** —
  `state.step_hierarchical/step_flat/update_flat`
  continuam (write paralelo M5; consumers `compute_*`
  helpers dependem).
- **Não** modificar trait `Introspector` (P185B
  fechou).
- **Não** modificar `TagIntrospector` (P193B fechou).
- **Não** modificar consumer C3/C4 — Introspector path
  para counters já existe.
- **Não** modificar `compute_*` helpers (P195D, P196B,
  P197B) — continuam a ler legacy.
- **Não** materializar `SetEquationNumbering` — passo
  paralelo independente.
- **Não** abrir sub-store novo — CounterRegistry
  (P184B) cobre.
- API pública preservada (adição de variant é
  retrocompatível).

---

## Sub-passos

### .A Auditoria L0

1. Confirmar walk arm `Content::CounterUpdate` em
   `01_core/src/rules/introspect.rs:625-642` (per
   P198A §3):
   - Re-verificar empiricamente.
   - Localizar 3 caminhos:
     - `CounterAction::Step` para `key == "heading"`:
       `state.step_hierarchical("heading", 1)` ou
       similar.
     - `CounterAction::Step` para outras keys:
       `state.step_flat(key)`.
     - `CounterAction::Update(val)`:
       `state.update_flat(key, *val)`.

2. Confirmar `Content::CounterUpdate` variant:
   - Campos: `key: String` + `action: CounterAction`.
   - `CounterAction` enum: `Step`,
     `Update(usize)` ou similar.

3. Confirmar `is_locatable(Content::CounterUpdate)`:
   - Estado actual: **false** (per P198A §5).
   - Após P198C: **true**.
   - Localizar arm em `01_core/src/rules/locatable.rs`
     (ou similar).

4. Confirmar `extract_payload(Content::CounterUpdate)`:
   - Estado actual: catch-all retorna `None` (per
     P198A §5).
   - Após P198C: arm específico retorna
     `Some(ElementPayload::CounterUpdate { ... })`.
   - Localizar match em `01_core/src/rules/extract_payload.rs`
     (ou similar).

5. Confirmar `ElementPayload` actual:
   - 11 variants após P195B.
   - Identificar onde adicionar
     `CounterUpdate { key, action }` (provavelmente
     após `Labelled` per convenção cronológica).

6. Confirmar `ElementKind` actual:
   - 9 variants após P186B.
   - **Decidir**: adicionar `ElementKind::CounterUpdate`
     ou não? Per convenção cristalino, todo
     `ElementPayload` tem `ElementKind` correspondente
     se locatable. Auditor confirma empiricamente.

7. Confirmar `from_tags` actual:
   - Match exhaustivo per P186B descoberta.
   - Adição de variant força arm explícito.
   - Localizar `01_core/src/rules/introspect/from_tags.rs`
     (ou similar).

8. Confirmar API CounterRegistry:
   - `apply_at(key, location)` para flat counters?
   - `apply_hierarchical_at(key, depth, location)`?
   - Verificar empiricamente em P184B sub-store.

9. Confirmar consumer downstream:
   - `compute_*` helpers (P195D Equation, P196B
     Heading, P197B Figure) leem counters mutados via
     legacy. Mutação legacy preservada obrigatória.
   - Per P189A inventário: walk arm Equation lê
     `state.get_flat("equation")` — pode ser afectado
     por CounterUpdate("equation").
   - Walk arm Figure lê `state.local_figure_counters`.
   - Walk arm Heading lê
     `state.format_hierarchical("heading")`.

10. Confirmar tests existentes:
    - Sentinela E6 P189B (per `.D` ponto 3 do P189B).
    - Tests existentes podem regridir se ordem de
      tags mudar (per padrão pragmático auditor #1
      M4-residual; vide P196B 5 tests adaptados).

11. Aplicar regra dos 2 eixos:
    - Eixo 1: snapshot final (consumer Layouter lê
      contadores após walk completo).
    - Eixo 2: CounterRegistry populated em produção
      (após P198C — via novo arm `from_tags`).

Output: tabela com item + estado + linhas exactas.

**Critério de saída**:
- Walk arm CounterUpdate localizado (3 caminhos).
- Estado pré-P198C confirmado (não-locatable).
- API CounterRegistry confirmada.
- Convenção `ElementKind` para CounterUpdate decidida.

### .B Adicionar variant `ElementPayload::CounterUpdate`

1. Em `01_core/src/entities/element_payload.rs` (ou
   similar):
   - Adicionar variant após `Labelled` (per
     convenção cronológica P195B):
     ```
     CounterUpdate {
         key:    String,
         action: CounterAction,
     },
     ```
   - Importar `CounterAction` se necessário.

2. Confirmar `@prompt-hash` actualiza após edit do L0.

3. Adicionar tests unit em `mod tests` de
   `element_payload.rs`:
   - `counter_update_construivel_e_compara`.
   - `counter_update_distincao_de_outras_variants`.
   - `counter_update_distingue_por_action`.

**Critério de saída**:
- Variant declarável.
- 3 tests unit passam.
- `cargo check --workspace` passa.

### .C Adicionar `ElementKind::CounterUpdate` (se aplicável)

Conforme `.A.6`:

**Se convenção exigir** (todo locatable tem ElementKind):
1. Adicionar variant `ElementKind::CounterUpdate` ao
   enum.
2. Actualizar L0 `entities/element_kind.md`.

**Se convenção não exigir** (CounterUpdate sem
ElementKind):
1. Skip este sub-passo.
2. Cross-reference em `.D` justifica ausência.

Output: variant adicionada ou justificação registada.

**Critério de saída**:
- Decisão materializada per `.A.6`.
- `cargo check --workspace` passa.

### .D Activar `is_locatable(Content::CounterUpdate) = true`

1. Em `01_core/src/rules/locatable.rs` (ou similar):
   - Localizar match arm `Content::CounterUpdate`
     actual (esperado: `_ => false` catch-all).
   - Adicionar arm específico:
     ```
     Content::CounterUpdate { .. } => true,
     ```

2. Tests existentes em `locatable.rs` podem regridir
   (counters totais por documento mudam — paralelo a
   P186D). Aplicar padrão pragmático auditor #1
   (ajustar fixture).

**Critério de saída**:
- `is_locatable(Content::CounterUpdate)` retorna
  `true`.
- Tests existentes adaptados se regridirem.
- `cargo check --workspace` passa.

### .E Adicionar arm em `extract_payload`

1. Em `01_core/src/rules/extract_payload.rs` (ou
   similar):
   - Adicionar arm:
     ```
     Content::CounterUpdate { key, action } => Some(
         ElementPayload::CounterUpdate {
             key:    key.clone(),
             action: action.clone(),
         },
     ),
     ```
   - Forma exacta replica P186C pattern (Equation
     promote).

2. Confirmar `@prompt-hash` actualiza após edit do L0.

**Critério de saída**:
- Arm adicionada.
- `cargo check --workspace` passa.

### .F Adicionar arm em `from_tags`

1. Em `01_core/src/rules/introspect/from_tags.rs`:
   - Adicionar arm:
     ```
     ElementPayload::CounterUpdate { key, action } => {
         match action {
             CounterAction::Step => {
                 if key == "heading" {
                     intr.counters.apply_hierarchical_at(
                         "heading", *depth, loc,
                     );
                 } else {
                     intr.counters.apply_at(key, loc);
                 }
             }
             CounterAction::Update(val) => {
                 intr.counters.apply_update_at(key, *val, loc);
             }
         }
     }
     ```
   - Forma exacta depende de API CounterRegistry per
     `.A.8`.
   - Auditor adapta a assinatura real.

2. Comentário inline opcional curto:
   ```
   // P198C — populate CounterRegistry via Tag
   // (cenário β-promote ADR-0069). Mutação legacy
   // preservada como write paralelo M5; cleanup em
   // M6 quando compute_* helpers migrarem para
   // sub-store.
   ```

3. Confirmar `@prompt-hash` actualiza após edit do L0.

**Critério de saída**:
- Arm adicionada com 3 caminhos
  (Step+heading, Step+other, Update).
- `cargo check --workspace` passa (match exhaustivo
  satisfeito).

### .G Adicionar comentário inline P198C no walk arm

1. Em `01_core/src/rules/introspect.rs:625-642` (per
   `.A.1`):
   - **Não modificar** mutações legacy (3 caminhos
     preservados).
   - Adicionar comentário inline curto antes da match:
     ```
     // P198C — E6 fechada estruturalmente
     // (β-promote ADR-0069). Caminho Introspector
     // activado: extract_payload arm + from_tags arm
     // populam CounterRegistry via apply_at /
     // apply_hierarchical_at. Mutação legacy
     // preservada como write paralelo M5: compute_*
     // helpers (P195D Equation, P196B Heading,
     // P197B Figure) lêem counters durante walk.
     // Cleanup orgânico em M6.
     ```

2. Confirmar `@prompt-hash` actualiza.

**Critério de saída**:
- Comentário inline presente.
- Mutações legacy preservadas (3 caminhos intactos).
- `cargo check --workspace` passa.

### .H Actualizar L0 `rules/introspect.md`

1. Tabela "Excepções M5" — actualizar entrada E6:
   - Estado: **fechada estruturalmente** (P198C).
   - Cenário β-promote — primeira aplicação.
   - Mutação legacy preservada como write paralelo M5.
   - Cleanup em M6.

2. Adicionar secção "Walk arm CounterUpdate migrado
   (P198C, cenário β-promote)":
   - Variant `ElementPayload::CounterUpdate` adicionado.
   - `is_locatable` activado.
   - 2 arms novos (`extract_payload` + `from_tags`).
   - CounterRegistry populated via `apply_at` /
     `apply_hierarchical_at`.
   - Mutação legacy preservada para `compute_*`
     helpers.
   - Cross-references: P184B (CounterRegistry), P186C
     (precedente promotion Equation), ADR-0069
     (cenário β-promote stylesheet).

3. Actualizar lista "Ordem inversa à mutação":
   - Passos 1-8 marcados ✅ (P193B, P194B, P195D,
     P196B, P197B, P198B, **P198C**).
   - Pré-requisitos paralelos (E1, E2-residuo)
     registados como passos finais.

4. Hash em branco aguarda recálculo manual em `.J`.

**Critério de saída**:
- L0 reflecte promotion.
- Tabela Excepções M5 actualizada.
- Secção nova presente.

### .I Tests E2E

5-6 tests obrigatórios per P198A §7.

#### Test 1 — `counter_update_extract_payload_emite_payload`

1. Construir `Content::CounterUpdate { key:
   "equation".into(), action: CounterAction::Step }`.
2. Chamar `extract_payload(content)`.
3. Asserções:
   - Retorna `Some(ElementPayload::CounterUpdate { key:
     "equation", action: Step })`.

#### Test 2 — `counter_update_is_locatable_true`

1. Confirmar `is_locatable(Content::CounterUpdate)
   = true`.
2. **Confirma promotion P198C**.

#### Test 3 — `counter_update_walk_popula_counter_registry`

1. Documento com CounterUpdate:
   ```
   Content::Sequence(vec![
       Content::CounterUpdate { key: "equation".into(), action: CounterAction::Step },
       Content::CounterUpdate { key: "equation".into(), action: CounterAction::Step },
   ])
   ```

2. Pipeline: walk + from_tags → TagIntrospector
   populated.

3. Asserções:
   - `intr.counters.flat_counter_at("equation", loc)`
     retorna valor correcto após N steps.
   - **Confirma caminho Introspector activo**.

#### Test 4 — `counter_update_paridade_legacy_vs_introspector`

1. Mesmo documento.
2. Asserções de paridade:
   - `state.get_flat("equation") == intr.counters.flat_counter_at("equation", final_loc)`.
   - Confirma write paralelo legacy + Introspector.

#### Test 5 — `counter_update_action_update_apply_correctly`

1. Documento com CounterUpdate Update:
   ```
   Content::CounterUpdate { key: "page".into(), action: CounterAction::Update(42) }
   ```

2. Asserções:
   - `intr.counters.flat_counter_at("page", loc)
     == 42`.
   - `state.flat["page"] == 42`.
   - Confirma 3º caminho da match.

#### Test 6 — `counter_update_compute_helpers_continuam_funcionais`

Sentinela cláusula gate substancial cadeia E6 ↔
helpers.

1. Documento com CounterUpdate("equation") + Equation
   block:
   ```
   Content::Sequence(vec![
       Content::CounterUpdate { key: "equation".into(), action: CounterAction::Step },
       Content::Equation { block: true, body: ... },
       Content::Labelled {
           label: Label("eq1".into()),
           target: Box::new(/* equation */),
       },
   ])
   ```

2. Asserções:
   - `compute_labelled` Equation arm produz
     `(Some("Equação (1)"), None)` (lê
     `state.get_flat("equation")`).
   - `intr.resolved_labels[Label("eq1")]` retorna
     `Some("Equação (1)")` via Tag P195D.
   - **Confirma cadeia E6 ↔ E4 funcional após
     promotion**.

Tests co-localizados em submódulo
`p198c_counter_update` em `tests.rs`.

**Critério de saída**:
- 5-6 tests passam.
- Tests existentes não regridem.

### .J Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P198B
   baseline (1.853): **+5 a +6**.
3. `crystalline-lint .` zero violations (após
   `--fix-hashes`).
4. Tests `p198c_counter_update::*` passam isoladamente.
5. Tests existentes (incluindo sentinela E6 P189B,
   tests `compute_labelled` P195D, tests P186) **não
   regridem** — mutação legacy preservada.
6. Tests potencialmente adaptados (se `is_locatable`
   ou `kind_index` totals mudaram) per padrão
   pragmático auditor #1.
7. Variant `ElementPayload::CounterUpdate` presente.
8. (Se aplicável) `ElementKind::CounterUpdate`
   presente.
9. `is_locatable(Content::CounterUpdate)` retorna
   `true`.
10. `extract_payload(Content::CounterUpdate)` retorna
    `Some(ElementPayload::CounterUpdate { ... })`.
11. `from_tags` arm CounterUpdate funcional (popula
    CounterRegistry via apply).
12. **3 mutações legacy preservadas** —
    `step_hierarchical`, `step_flat`, `update_flat`
    continuam.
13. **Comentário inline P198C** presente em walk arm.
14. **L0 secção nova "Walk arm CounterUpdate migrado
    (P198C, cenário β-promote)"** presente.
15. **Tabela Excepções M5** com E6 marcada
    "fechada estruturalmente".
16. `compute_*` helpers (P195D, P196B, P197B) **NÃO
    modificados**.
17. Trait `Introspector` NÃO modificado.
18. `TagIntrospector` NÃO modificado.
19. Consumer C3 (P184D) / C4 (P194B) NÃO modificados.
20. Snapshot tests verdes.
21. Linter passa final.

### .K Encerramento

Escrever
`00_nucleo/materialization/typst-passo-198c-relatorio.md`
com:

- Resumo: promotion CounterUpdate; cenário β-promote
  primeira aplicação; E6 fechada estruturalmente.
- Confirmação `.J` (21 verificações).
- Δ tests vs baseline P198B (esperado +5 a +6).
- Hashes finais L0 (`rules/introspect.md` +
  possivelmente `entities/element_payload.md` +
  `entities/element_kind.md`).
- Decisões de execução notáveis:
  - Decisão `ElementKind::CounterUpdate` (per `.A.6`).
  - Tests existentes adaptados (per padrão pragmático
    auditor #1 se necessário).
- Estado actual:
  - P198 série: A ✅ B ✅ C ✅ | D pendente.
  - **E6 fechada estruturalmente** (cenário
    β-promote).
  - 78 passos executados.
- Pendências cumulativas: 1 excepção activa + 1
  residuo (E1, E2-residuo).
- Próximo passo: P198D — relatório consolidado P198 +
  actualização DEBT M5-residual.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate
   substancial.
2. Variant `ElementPayload::CounterUpdate` adicionado
   (`.B`).
3. `ElementKind::CounterUpdate` adicionado se aplicável
   (`.C`).
4. `is_locatable(Content::CounterUpdate) = true`
   (`.D`).
5. `extract_payload` arm adicionada (`.E`).
6. `from_tags` arm adicionada (`.F`).
7. Comentário inline P198C no walk arm (`.G`).
8. L0 actualizado (`.H`).
9. 5-6 tests E2E passam (`.I`).
10. Verificações `.J` passam (21/21).
11. Tests existentes não regridem ou são adaptados.
12. Mutação legacy preservada (3 caminhos).
13. Output observable em produção inalterado
    (caminho Introspector activado mas legacy fornece
    valores idênticos).
14. Relatório `.K` escrito.

---

## O que pode sair errado

- **API CounterRegistry diverge** do esperado (apply_at
  / apply_hierarchical_at): cláusula gate trivial —
  ajustar arm `from_tags` per assinatura real.
- **`ElementKind::CounterUpdate` não exigido** pela
  convenção: cláusula gate trivial — skip `.C`.
- **Tests `is_locatable_total_count` regridem** porque
  CounterUpdate passa a ser locatable: cláusula gate
  trivial — adaptar fixture per padrão pragmático
  auditor #1.
- **Tests `kind_index` regridem** se ElementKind
  adicionada: cláusula gate trivial.
- **`from_tags` arm produz valor diferente do legacy
  para CounterUpdate(equation, Step)**: cláusula gate
  substancial — investigar API CounterRegistry vs
  semântica legacy.
- **Test 6 (cadeia E6 ↔ E4) falha**: indica que
  promotion afectou paridade observable. Cláusula gate
  substancial.
- **Snapshot tests divergem**: indica que ordem de
  Tags ou contagem mudou. Investigar.
- **Linter divergência V13/V14**: cláusula gate
  trivial.

---

## Notas operacionais

- **Tamanho**: M genuíno. ~80 LOC produção (variant +
  arms + comentário) + ~120 LOC tests + ~80 LOC L0.
- **Sem dependências externas novas**.
- **Sem ADR; sem DEBT formal**.
- **Padrão materializado**: cenário β-promote primeira
  aplicação. **5ª aplicação ADR-0069 stylesheet**
  (mas sem helper extraído porque arm é trivial).
- **Cláusula gate trivial**: aplicável a API
  CounterRegistry, ElementKind convenção, tests
  adaptação.
- **Cláusula gate substancial**: aplicável a:
  - Paridade `from_tags` arm vs legacy diverge.
  - Cadeia E6 ↔ E4 quebra.
  - Tests sentinela E6 P189B regridem.
- **Distinção de P186C** (Equation promote):
  - P186C: Equation tinha `Content::SetEquationNumbering`
    como pré-requisito (Reserva 1) — gate dormente
    em P186E.
  - P198C: CounterUpdate **não tem pré-requisito**
    análogo — caminho Introspector activa
    imediatamente.
- **Próximo passo P198D**: relatório consolidado P198
  (9 secções) + actualização DEBT M5-residual.
  Magnitude S puro.
- **Após P198D**: M5 universal a 2 pré-requisitos do
  fecho — `SetEquationNumbering` (E1) + sub-store
  `headings_for_toc` (E2-residuo). Ambos paralelos
  fora série P198.
