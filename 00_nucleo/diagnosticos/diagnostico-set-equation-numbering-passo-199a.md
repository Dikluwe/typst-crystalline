# Diagnóstico — `Content::SetEquationNumbering` (P199A)

**Data**: 2026-05-04
**Pattern arquitectural relevante**: ADR-0069 — cenário α por construção (variante operacional consolidada).
**Template primário**: P182C (`Content::SetHeadingNumbering`).
**Estado empírico**: confirmado por leitura directa de `01_core/src/`.

---

## §1 Validação estado actual

### §1.1 `Content::SetEquationNumbering` — ausente

```bash
grep -rn "SetEquationNumbering" 01_core/src/
```

Retorna **apenas referências em comentários** (Reserva 1):
- `entities/element_payload.rs:114`
- `rules/introspect.rs:509,514`
- `rules/introspect.rs:2907` (test 6 P198C inline doc)
- `rules/introspect/locatable.rs:55`
- `rules/introspect/from_tags.rs:230,947`
- `rules/layout/equation.rs:26,101`
- `rules/layout/tests.rs:4465,4719`

**Variant não existe em produção** — confirmado.

### §1.2 Template P182C mapeado

#### 1.2.1 Variant — `entities/content.rs:176`

```rust
SetHeadingNumbering { active: bool },
```

#### 1.2.2 `is_locatable` — `rules/introspect/locatable.rs:49`

```rust
Content::SetHeadingNumbering { .. } => true,
```

#### 1.2.3 `extract_payload` — `rules/introspect/extract_payload.rs:63-66`

```rust
Content::SetHeadingNumbering { active } => Some(ElementPayload::StateUpdate {
    key:    "numbering_active:heading".to_string(),
    update: StateUpdate::Set(Box::new(Value::Bool(*active))),
}),
```

#### 1.2.4 Walk arm — `rules/introspect.rs:611-624` (pós-P198B)

```rust
Content::SetHeadingNumbering { active } => {
    // P198B — E5 fechada estruturalmente (cenário α). [comentário inline]
    state.numbering_active.insert("heading".to_string(), *active);
}
```

#### 1.2.5 `from_tags` arm StateUpdate — genérica

`from_tags.rs:172+` (P171/P173) — **genérica para qualquer key**. Sem hardcoded `"heading"`. Auto-init em primeira ocorrência. Processa `numbering_active:equation` transparentemente.

### §1.3 Consumers Equation downstream `state.numbering_active["equation"]`

#### 1.3.1 Walk arm Equation — `introspect.rs:517`

```rust
if *block && state.is_numbering_active("equation") {
    state.step_flat("equation");
}
```

Lê durante walk para gating do counter step.

#### 1.3.2 Layouter Equation — `layout/equation.rs:32-33`

```rust
&& (self.introspector.is_numbering_active("numbering_active:equation")
    || self.counter.is_numbering_active("equation"));
```

**Consumer já com substitution-with-fallback implementada** (analogia C5 ↔ SetHeadingNumbering P185B). Caminho Introspector dorme em produção até P199 materializar `Content::SetEquationNumbering` que emita Tag::StateUpdate sob chave canónica.

#### 1.3.3 `compute_labelled` Equation arm — `introspect.rs:337`

```rust
Content::Equation { block, .. } if *block => {
    let n = state.get_flat("equation");
    ...
}
```

Lê `state.get_flat("equation")` — **não** `is_numbering_active` directamente. Mas o counter `equation` só avança se `numbering_active["equation"] = true` (per walk arm Equation linha 517). **Cadeia indirecta**: SetEquationNumbering → state.numbering_active["equation"] → walk arm Equation → state.flat["equation"] → compute_labelled lê.

### §1.4 Parser sintáctico — ausente

```bash
grep -rn "set equation\|#set equation" 01_core/src/
```

Retorna vazio. **Sem parser sintáctico** para `#set equation(numbering: ...)` em cristalino. Cláusula 2 Opção α confirmada — P199 cobre apenas materialização interna; variant disponível programaticamente para tests + uso futuro.

### §1.5 Tests existentes que cobrem cadeia E1

- `layout/tests.rs:968`: `state.numbering_active.insert("equation".to_string(), true)` — bypass directo para tests.
- `layout/tests.rs:4472`: idem.
- `layout/tests.rs:4465-4470`: comentário cita Reserva 1.
- `entities/introspector.rs:492,517,636`: tests `is_numbering_active("numbering_active:equation")` retornam false em introspector vazio — confirmam dormência pré-P199.
- `rules/introspect.rs:2907-2912` (test 6 P198C): inline doc explicita Reserva 1 e usa CounterUpdate Step para bypass.

### §1.6 L0 alvos

- `00_nucleo/prompts/entities/content.md` — variant nova (se L0 existir; verificar).
- `00_nucleo/prompts/rules/introspect.md` — arms novos + walk arm.

### §1.7 Regra dos 2 eixos

| Eixo | Estado pré-P199 | Estado pós-P199B |
|------|-----------------|------------------|
| Eixo 1 (consumer durante walk?) | ✅ Sim (walk arm Equation; compute_labelled) | ✅ Sim (mutação legacy preservada) |
| Eixo 2 (sub-store activo?) | ❌ Não (Content::SetEquationNumbering ausente → from_tags arm StateUpdate nunca dispara para key "numbering_active:equation") | ✅ Sim (variant materializada → extract_payload emite Tag::StateUpdate → from_tags popula StateRegistry) |

**Cenário α por construção**: caminho Introspector activa **imediatamente** após materialização da variant; sem necessidade de promote a locatable separado (já existe arm StateUpdate genérica P171).

### §1.8 Workspace baseline

- Tests: 1.859 verdes.
- Linter: zero violations.
- Hash L0 introspect: `d25dfc47`.
- Hash código introspect: `f49ec9df`.

---

## §2 Decisões cláusula 1–7 (formato O1–O5)

### Cláusula 1 — Forma da materialização

- **O1 (input)**: `SetHeadingNumbering { active: bool }` em `content.rs:176` (template P182C).
- **O2 (alternativas)**:
  - α — replica literal `SetEquationNumbering { active: bool }`.
  - β — forma diferente (improvável).
- **O3 (critério)**: analogia directa. Replicação literal reduz incerteza arquitectural a zero.
- **Decisão**: **Opção α** — `SetEquationNumbering { active: bool }`.
- **O4 (magnitude)**: ~5 LOC variant.
- **O5 (reversibilidade)**: trivial (remoção da variant).

### Cláusula 2 — Scope do parser

- **O1 (input)**: `grep "#set equation"` retorna vazio. Sem parser sintáctico actual.
- **O2 (alternativas)**:
  - α — P199 cobre apenas materialização interna.
  - β — P199 cobre parser. Magnitude L+.
- **O3 (critério)**: parser fora do escopo M5; pode ser passo dedicado M6+ ou após.
- **Decisão**: **Opção α** — apenas materialização interna. Variant disponível programaticamente para tests + uso futuro.
- **O4 (magnitude)**: 0 LOC parser.
- **O5 (reversibilidade)**: trivial.

### Cláusula 3 — `is_locatable` + `extract_payload` arms

- **O1 (input)**: P182C arms confirmados (locatable.rs:49; extract_payload.rs:63).
- **O2 (alternativas)**: replica literal vs forma diferente.
- **O3 (critério)**: analogia directa.
- **Decisão**: **replica literal** com chave canónica `numbering_active:equation`:
  ```rust
  // locatable.rs
  Content::SetEquationNumbering { .. } => true,

  // extract_payload.rs
  Content::SetEquationNumbering { active } => Some(ElementPayload::StateUpdate {
      key:    "numbering_active:equation".to_string(),
      update: StateUpdate::Set(Box::new(Value::Bool(*active))),
  }),
  ```
- **O4 (magnitude)**: ~10 LOC.
- **O5 (reversibilidade)**: trivial.

### Cláusula 4 — Walk arm

- **O1 (input)**: P182C walk arm em `introspect.rs:611-624`.
- **O2 (alternativas)**: replica literal com chave equation.
- **O3 (critério)**: analogia directa.
- **Decisão**:
  ```rust
  Content::SetEquationNumbering { active } => {
      // P199B — E1 fechada estruturalmente (cenário α por construção).
      // Caminho Introspector activado por construção desde materialização:
      // extract_payload → ElementPayload::StateUpdate sob chave
      // numbering_active:equation → from_tags arm StateUpdate (P171)
      // popula StateRegistry.
      //
      // Mutação legacy preservada como write paralelo M5: walk arm
      // Equation (introspect.rs:517) + Layouter equation.rs:32 lêem
      // state.is_numbering_active("equation") durante walk para gating
      // do counter step. Cleanup orgânico em M6.
      state.numbering_active.insert("equation".to_string(), *active);
  }
  ```
- **O4 (magnitude)**: ~15 LOC (incluindo comentário).
- **O5 (reversibilidade)**: trivial.

### Cláusula 5 — Reuso `from_tags` arm StateUpdate

- **O1 (input)**: arm StateUpdate em `from_tags.rs:172+` é genérica (sem hardcoded keys).
- **O2 (alternativas)**: confirmar não-modificação vs adicionar caso especial (improvável).
- **O3 (critério)**: P171 design genérico.
- **Decisão**: **sem modificação** — arm processa `numbering_active:equation` transparentemente (auto-init em primeira ocorrência).
- **O4 (magnitude)**: 0 LOC.
- **O5 (reversibilidade)**: trivial.

### Cláusula 6 — Cadeia E1 ↔ helpers

- **O1 (input)**: walk arm Equation `introspect.rs:517` lê `state.is_numbering_active("equation")` durante walk; `compute_labelled` Equation arm lê `state.get_flat("equation")` (cadeia indirecta).
- **O2 (alternativas)**: preservar mutação legacy vs migrar consumers para Introspector imediato.
- **O3 (critério)**: padrão P195D/P196B/P198B preserva mutação legacy.
- **Decisão**: **preservar** mutação legacy `state.numbering_active.insert("equation", *active)`. Cleanup orgânico em M6.
- **O4 (magnitude)**: 0 LOC mudança.
- **O5 (reversibilidade)**: trivial.

### Cláusula 7 — Critério de fecho de P199

- **O1 (input)**: análogo a P198B critério de fecho.
- **O2 (alternativas)**: cenário α completo vs parcial.
- **O3 (critério)**: analogia directa.
- **Decisão**: P199 fecha quando:
  - Variant `Content::SetEquationNumbering` adicionada.
  - 3 arms novos (`is_locatable`, `extract_payload`, walk arm) activos.
  - `from_tags` arm StateUpdate processa em produção sem modificação.
  - Tests E2E confirmam paridade observable + activação Introspector path.
  - **E1 fecha estruturalmente** (cenário α por construção).
  - Mutação legacy preservada.
- M5 universal **NÃO fecha** ainda — sub-store `headings_for_toc` (E2-residuo) ainda activo.

---

## §3 Plano de sub-passos sem condicionais

### P199B — Materialização `Content::SetEquationNumbering` (cenário α por construção)

Magnitude **M**:

1. Adicionar variant `Content::SetEquationNumbering { active: bool }` em `entities/content.rs` após `SetHeadingNumbering` (alfabeticamente próximo).
2. Adicionar match arm onde `Content` é matched exhaustivamente (e.g., método `to_string`, comparação, etc.) — analogia P198C com novas variants.
3. Activar `is_locatable(Content::SetEquationNumbering) = true` em `rules/introspect/locatable.rs`.
4. Adicionar arm em `rules/introspect/extract_payload.rs` retornando `Some(ElementPayload::StateUpdate { key: "numbering_active:equation", update: Set(Bool(*active)) })`.
5. Adicionar walk arm em `rules/introspect.rs` após `Content::SetHeadingNumbering` (linha 611-624) com mutação legacy + comentário inline P199B declarando E1 fechada estruturalmente.
6. Confirmar `from_tags` arm StateUpdate (P171) processa transparentemente — **sem modificação**.
7. Actualizar L0 `rules/introspect.md`:
   - Tabela "Excepções M5": linha **E1** → "**Fechou estruturalmente em P199B (cenário α por construção)**".
   - Lista "Ordem inversa à mutação": passo 9 marcado ✅.
   - Nova secção "Variant `Content::SetEquationNumbering` materializada (P199B, cenário α por construção)" (ou actualizar "Walk arm SetHeadingNumbering migrado" para mencionar paralelo Equation).
8. Adicionar 5 tests E2E em `mod tests` (sentinelas E1 fechada estruturalmente):
   - `set_equation_numbering_extract_payload_emite_state_update`.
   - `set_equation_numbering_from_tags_popula_state_registry`.
   - `set_equation_numbering_paridade_legacy_vs_introspector`.
   - `walk_arm_equation_le_numbering_active_legacy`.
   - `consumer_equation_layouter_recebe_some_via_introspector`.
9. Hash L0 actualizado via `crystalline-lint --fix-hashes`.

### P199C — Encerramento série P199

Magnitude **S puro**:

1. Auditoria empírica final (12-13 verificações).
2. Relatório consolidado `typst-passo-199-relatorio-consolidado.md` (9 secções padrão).
3. Nota DEBT M5-residual actualizada (0 excepções activas + 1 residuo).
4. Verificação estrutural final.

---

## §4 Magnitude consolidada

| Sub | Magnitude | LOC produção | LOC teste | LOC L0 | Δ tests |
|-----|-----------|--------------|-----------|--------|---------|
| P199A | S | 0 | 0 | 0 | 0 |
| P199B | M | ~50 (variant + 3 arms + walk arm + comentário + match arms exaustivos) | ~120 (5 tests) | ~60 | +5 |
| P199C | S | 0 | 0 | 0 | 0 |

**Total agregado**: **M** (~50 LOC produção + ~120 LOC tests + ~60 LOC L0 + relatórios).

---

## §5 ADR avaliação

- Pattern ADR-0069 cenário α por construção aplicável directamente.
- Sem decisão arquitectural nova.
- Analogia directa com P182C.

**Conclusão**: **não cria ADR**.

---

## §6 DEBT M5-residual avaliação

| Excepção | Pré-P199 | Pós-P199B |
|----------|----------|-----------|
| E1 (Equation / SetEquationNumbering) | activa | **fechada estruturalmente** |
| E2-residuo (Heading headings_for_toc) | residuo | residuo |

**Estado pós-P199**: **0 excepções activas + 1 residuo** (E2-residuo); 1 pré-requisito restante (sub-store `headings_for_toc`).

**Cenário B continua** (sem DEBT formal aberto).

---

## §7 Cadeia E1 com consumers Equation — análise empírica

### Sequência durante walk

Considere documento:
```
Content::Sequence(vec![
    Content::SetEquationNumbering { active: true },  // ← futura variant P199B
    Content::Equation { body: ..., block: true },
    Content::Labelled {
        label: Label("eq1"),
        target: Box::new(/* equation block */),
    },
])
```

1. Walk top em `SetEquationNumbering` — locatable (após P199B).
2. `extract_payload` retorna `Some(StateUpdate { key: "numbering_active:equation", update: Set(Bool(true)) })`.
3. Walk emite `Tag::Start(loc, ElementInfo { payload: StateUpdate {...}, label: None })`.
4. Match arm `Content::SetEquationNumbering`:
   - Mutação: `state.numbering_active.insert("equation", true)` (write paralelo M5).
5. Walk top em `Equation` — locatable (P186D).
6. Match arm `Content::Equation`:
   - Lê `state.is_numbering_active("equation")` → **true** (legacy populado pelo passo anterior).
   - Avança counter: `state.step_flat("equation")` → `state.flat["equation"] = 1`.
7. Walk top em `Labelled` — não-locatable.
8. Match arm `Content::Labelled` (P195D pós-recursão):
   - `compute_labelled(target=Equation, state)` → lê `state.get_flat("equation") = 1` → produz `(Some("Equação (1)"), None)`.
   - Mutação legacy: `state.resolved_labels.insert("eq1", "Equação (1)")`.
   - Tag::Labelled emit pos-recursão.
9. `from_tags` post-walk:
   - Arm StateUpdate processa Tag::Start de SetEquationNumbering → `intr.state` recebe `numbering_active:equation = Bool(true)`.
   - Arm Equation processa Tag de Equation → counter "equation" populated em CounterRegistry (gate `is_numbering_active_at` retorna true por causa do StateRegistry populado).
   - Arm Labelled processa Tag::Labelled → `intr.resolved_labels["eq1"] = "Equação (1)"`.

### Caminho Introspector activado pós-P199B

| Sub-store | Estado pré-P199 | Estado pós-P199B |
|-----------|----------------|------------------|
| `intr.state["numbering_active:equation"]` | nunca populated | populated via Tag::StateUpdate |
| `intr.counters["equation"]` | nunca populated (gate dormente em P186E) | populated (gate activado) |
| `intr.resolved_labels["eq1"]` | populated via P195D Tag (lê legacy) | populated via P195D Tag |
| Layouter equation.rs:32 fallback | sempre fallback legacy | primeira branch Introspector activa |

### Risco se mutação legacy for removida

- Walk arm Equation (introspect.rs:517) lê `state.is_numbering_active("equation")`. Se removida, counter "equation" não avança em legacy → `compute_labelled` retorna `(None, None)`.
- Quebra paridade observable.

### Mitigação P199B

**Mutação legacy preservada como write paralelo M5**. Cadeia E1 funcional. Cleanup orgânico em M6 quando walk arm Equation migrar para `is_numbering_active_at` (Introspector path location-aware).

---

## §8 Próximo sub-passo (P199B com escopo concreto)

**P199B — Materialização `Content::SetEquationNumbering` (cenário α por construção)**:

1. Adicionar variant `Content::SetEquationNumbering { active: bool }` em `01_core/src/entities/content.rs` (próximo a SetHeadingNumbering linha 176).

2. Cobrir match arms exhaustivos onde `Content` é matched (analogia P198C — novas variants em `to_string`, `eq`, `materialize_time` em introspect, etc.). Auditor descobre via `cargo check` quais arms exigem cobertura.

3. Activar `is_locatable(Content::SetEquationNumbering) = true` em `01_core/src/rules/introspect/locatable.rs` (próximo a SetHeadingNumbering linha 49).

4. Adicionar arm em `01_core/src/rules/introspect/extract_payload.rs` (próximo a SetHeadingNumbering linha 63):
   ```rust
   Content::SetEquationNumbering { active } => Some(ElementPayload::StateUpdate {
       key:    "numbering_active:equation".to_string(),
       update: StateUpdate::Set(Box::new(Value::Bool(*active))),
   }),
   ```

5. Adicionar walk arm em `01_core/src/rules/introspect.rs` (próximo a SetHeadingNumbering linha 611):
   ```rust
   Content::SetEquationNumbering { active } => {
       // P199B — E1 fechada estruturalmente (cenário α por construção).
       // [comentário inline completo per cláusula 4 §2]
       state.numbering_active.insert("equation".to_string(), *active);
   }
   ```

6. Actualizar L0 `00_nucleo/prompts/rules/introspect.md`:
   - Tabela "Excepções M5": E1 → "**Fechou estruturalmente em P199B (cenário α por construção — variant materializada)**".
   - Lista "Ordem inversa à mutação": passo 9 marcado ✅.
   - Secção nova ou actualização da secção P198B para mencionar paralelo Equation.

7. Adicionar 5 tests E2E em `mod tests`.

8. `crystalline-lint --fix-hashes` para actualizar hash L0.

**Critério de fecho P199B**:
- Tests workspace 1.859 + 5 = 1.864 verdes.
- Lint zero violations.
- L0 hash actualizado.
- Variant disponível programaticamente.
- Caminho Introspector activado (Layouter equation.rs:32 first branch retorna Some quando construído com Tag).
