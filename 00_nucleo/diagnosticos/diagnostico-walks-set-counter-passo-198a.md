# Diagnóstico — Walks SetHeadingNumbering + CounterUpdate (P198A)

**Data**: 2026-05-04
**Pattern arquitectural relevante**: ADR-0069 — 4ª e 5ª aplicações.
**Variantes operacionais avaliadas**: P195D (não-locatable + snapshot), P196B (locatable + emitted_loc), P197B (cenário α).
**Estado empírico**: confirmado por leitura directa de `01_core/src/`.

---

## §1 Validação estado actual

### §1.1 Walk arm `Content::SetHeadingNumbering` (E5)

Localização: `01_core/src/rules/introspect.rs:611-623`.

```rust
Content::SetHeadingNumbering { active } => {
    // Excepção M5 E5 ... Tag emitido paralelamente via P182C
    // extract_payload arm — from_tags arm StateUpdate popula
    // StateRegistry independentemente; legacy mutation aqui é
    // write paralelo durante janela compat.
    state.numbering_active.insert("heading".to_string(), *active);
}
```

**1 mutação**: `state.numbering_active.insert("heading", *active)`.

### §1.2 Variant `Content::SetHeadingNumbering` (content.rs:176)

```rust
SetHeadingNumbering { active: bool }
```

Leaf content — sem body recursivo.

### §1.3 `is_locatable(SetHeadingNumbering) = true`

Confirmado em `locatable.rs:49`:
```rust
Content::SetHeadingNumbering { .. } => true,
```

P182C — locatable desde 2026-04-30.

### §1.4 `extract_payload(SetHeadingNumbering)` arm (extract_payload.rs:63-66)

```rust
Content::SetHeadingNumbering { active } => Some(ElementPayload::StateUpdate {
    key:    "numbering_active:heading".to_string(),
    update: StateUpdate::Set(Box::new(Value::Bool(*active))),
}),
```

Reuso de `ElementPayload::StateUpdate` (P171/P173) sob chave canónica `numbering_active:heading`. **Sem variant nova necessária**.

### §1.5 `from_tags` arm StateUpdate (from_tags.rs:172+)

Já existe — popula `intr.state` (StateRegistry P182). Auto-init em primeira ocorrência. **Caminho Introspector activo desde P182C**.

### §1.6 Consumer downstream `state.numbering_active`

`grep -n "is_numbering_active\|numbering_active" introspect.rs`:

1. **`compute_heading_auto_toc` (P196B helper, linha 384)**:
   ```rust
   let resolved_text = if state.is_numbering_active("heading") { ... }
   ```
2. **Walk arm `Content::Equation` (linha 517)**:
   ```rust
   if *block && state.is_numbering_active("equation") { state.step_flat("equation"); }
   ```

Cadeia E5 — `compute_heading_auto_toc` lê durante walk. **Mutação legacy DEVE ser preservada** (cláusula gate substancial análoga a cadeia E2-E3 P197A).

### §1.7 Walk arm `Content::CounterUpdate` (E6)

Localização: `introspect.rs:625-642`.

```rust
Content::CounterUpdate { key, action } => match action {
    CounterAction::Step => {
        if key == "heading" {
            state.step_hierarchical("heading", 1);
        } else {
            state.step_flat(key);
        }
    }
    CounterAction::Update(val) => {
        state.update_flat(key, *val);
    }
},
```

**3 caminhos de mutação** sob match de action:
- `Step + key="heading"` → `state.step_hierarchical("heading", 1)`.
- `Step + key!="heading"` → `state.step_flat(key)`.
- `Update(val)` → `state.update_flat(key, *val)`.

### §1.8 Variant `Content::CounterUpdate` (content.rs:190-193)

```rust
CounterUpdate {
    key:    String,
    action: CounterAction,
}
```

Onde `CounterAction` é re-export de `CounterUpdate` enum em `entities/counter_update.rs`:
```rust
pub enum CounterUpdate {
    Step,
    Update(usize),
}
```

Leaf content — sem body recursivo.

### §1.9 `is_locatable(CounterUpdate) = false`

Confirmado em `locatable.rs:84`:
```rust
| Content::CounterUpdate { .. }
... => false,
```

### §1.10 `extract_payload(CounterUpdate)` — **arm não existe**

Sem arm em `extract_payload.rs`. Cai no `_ => None` final.

### §1.11 `from_tags` arm para CounterUpdate — **não existe**

Sem arm correspondente. CounterRegistry **não é populated via CounterUpdate** actualmente. CounterRegistry é populated via:
- Heading arm (`apply_hierarchical_at("heading", level, loc)`).
- Figure arm (`apply_at("figure:{kind}", Step, loc)`).
- Equation arm (gate em P186E).

CounterUpdate é um caminho **paralelo independente** que muta legacy state directamente sem populating CounterRegistry.

### §1.12 Consumer downstream `state.flat / state.hierarchical / numbering_active`

- `compute_labelled` Equation arm (introspect.rs:337): `state.get_flat("equation")`.
- `compute_heading_auto_toc` (introspect.rs:386): `state.format_hierarchical("heading")`.
- `compute_figure` (P197B): lê `state.local_figure_counters`.
- Layouter consumers (mod.rs em outros lugares).

Mutações via CounterUpdate alimentam estes consumers. Mutação legacy **DEVE ser preservada** durante M5.

### §1.13 Regra dos 2 eixos por arm

| Arm | Eixo 1 (consumer durante walk?) | Eixo 2 (sub-store activo?) |
|-----|--------------------------------|---------------------------|
| **SetHeadingNumbering** | ✅ Sim (compute_heading_auto_toc + Equation walk arm) | ✅ Sim (StateRegistry P182C) |
| **CounterUpdate** | ✅ Sim (compute_labelled, compute_heading_auto_toc, compute_figure) | ❌ Não (CounterRegistry não populated via CounterUpdate) |

**E5**: caminho Introspector activo → cenário α aplicável.
**E6**: caminho Introspector NÃO activo → promoção a locatable necessária (cenário β).

### §1.14 Workspace baseline

- Tests: 1.848 verdes.
- Linter: zero violations.
- Hash L0 introspect: `b9f78ff9`.
- Hash código introspect: `c938c001`.

---

## §2 Decisões cláusula 1–9

### Cláusula 1 — Variante para SetHeadingNumbering (E5)

- **Decisão**: **cenário α** (paralelo a Figure P197B).
- Caminho Introspector activo desde P182C (StateRegistry populated via StateUpdate Tag).
- Walk arm muta `state.numbering_active.insert("heading", ...)` em paralelo a Tag emission via `extract_payload`.
- Refactor estilístico opcional — declarar fechada estruturalmente em L0.
- Mutação legacy preservada (write paralelo M5) porque `compute_heading_auto_toc` lê durante walk.
- Magnitude P198B: **S puro** (declaração + 5 tests sentinela; sem helper extraído porque arm é trivial — 1 linha).

### Cláusula 2 — Variante para CounterUpdate (E6)

- **Decisão**: **cenário β-promote** — promover a locatable + nova ElementPayload variant + from_tags arm popular CounterRegistry.
- Trabalho concreto:
  1. Mover `Content::CounterUpdate` de não-locatable para locatable em `locatable.rs`.
  2. Adicionar arm em `extract_payload.rs` retornando `Some(ElementPayload::CounterUpdate { key, action })`.
  3. Adicionar variant nova `ElementPayload::CounterUpdate { key: String, action: CounterUpdate }` em `element_payload.rs`.
  4. Adicionar arm em `from_tags.rs` para `ElementPayload::CounterUpdate` aplicando `counters.apply_at` ou `apply_hierarchical_at` conforme key/action.
  5. Walk arm preserva mutação legacy (write paralelo M5).
- Magnitude P198C: **M** (5 ficheiros tocados; nova variant; novo from_tags arm; promoção locatable).

### Cláusula 3 — Helper(s) privado(s)

- **E5**: sem helper. Walk arm é 1 linha; extracção não acrescenta valor.
- **E6**: sem helper centralizado. Lógica de mapeamento `(key, action) → CounterRegistry::apply_at | apply_hierarchical_at` é o `from_tags` arm próprio (não duplicada com walk).

**Decisão**: 0 helpers novos na família ADR-0069 stylesheet em P198. Distinção do padrão P195D/P196B/P197B.

### Cláusula 4 — Ordem de execução dos 2 arms

- **Decisão**: **Opção β — sub-passos separados** (variantes diferentes; magnitudes divergentes).
- P198B: SetHeadingNumbering (cenário α — S).
- P198C: CounterUpdate (cenário β-promote — M).
- P198D: relatório consolidado P198 + DEBT (S).

### Cláusula 5 — Cadeia E5/E6 com outros arms

- **E5 ↔ Heading auto-toc + Equation**: `compute_heading_auto_toc` (P196B) lê `state.is_numbering_active("heading")`; walk arm Equation (linha 517) lê `state.is_numbering_active("equation")`. Mutação legacy `state.numbering_active.insert` preservada.
- **E6 ↔ compute_labelled + compute_heading_auto_toc + compute_figure**: helpers lêm counters mutados via CounterUpdate. Mutação legacy `state.step_*` / `state.update_flat` preservada.

**Decisão**: preservar mutações legacy em ambos arms durante janela compat M5. Cleanup orgânico em M6 quando consumers (helpers `compute_*`) migrarem para sub-stores Introspector.

### Cláusula 6 — Interacção com E1 (Equation)

- E1 bloqueada por `Content::SetEquationNumbering` ausente — independente de P198.
- **Sem trabalho em E1 dentro de P198**.
- Nota: walk arm Equation (linha 517) usa `state.is_numbering_active("equation")` — chave que **só será populada** quando `Content::SetEquationNumbering` materializar e emitir Tag análogo a SetHeadingNumbering. Ortogonal a P198.

### Cláusula 7 — Mutação legacy preservada

Replica padrão P195D/P196B/P197B. Write paralelo M5 → cleanup orgânico em M6.

### Cláusula 8 — Critério de fecho de P198

- E5 fecha estruturalmente em P198B (cenário α — declaração formal).
- E6 fecha estruturalmente em P198C (promoção a locatable + new variant + from_tags arm).
- M5 universal **NÃO fecha** ainda. Pré-requisitos restantes:
  - E1 ↔ `SetEquationNumbering` materialização (passo paralelo).
  - E2-residuo ↔ sub-store `intr.headings_for_toc` (passo paralelo).

### Cláusula 9 — Granularidade

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| **P198B** | Walk arm SetHeadingNumbering — declaração formal cenário α + L0 + 4-5 tests sentinela | **S** |
| **P198C** | Promote `Content::CounterUpdate` a locatable: locatable.rs + extract_payload.rs + element_payload.rs + from_tags.rs + walk arm preservation + L0 + 5-6 tests E2E | **M** |
| **P198D** | Auditoria + relatório consolidado P198 + DEBT M5-residual actualizada | **S** |

Total agregado: **M-** (S + M + S).

---

## §3 Plano de sub-passos sem condicionais

### P198B — Walk arm SetHeadingNumbering cenário α

1. Confirmação empírica final estado actual (replica P197B .A).
2. Sem alteração ao walk arm (mutação legacy preservada como está).
3. Adicionar comentário inline declarando E5 fechada estruturalmente via cenário α (paralelo a Figure P197B).
4. Actualizar L0 introspect.md:
   - Tabela "Excepções M5": linha E5 → "**Fechou estruturalmente em P198B (cenário α — caminho Introspector activo desde P182C)**".
   - Lista "Ordem inversa à mutação": passo 7 marcado parcialmente ✅ (E5 portion).
   - Nova secção "Walk arm SetHeadingNumbering migrado (P198B, cenário α)".
5. Adicionar 5 tests sentinela em `mod tests`:
   - `set_heading_numbering_extract_payload_emite_state_update`.
   - `set_heading_numbering_from_tags_popula_state_registry`.
   - `set_heading_numbering_paridade_legacy_vs_introspector`.
   - `compute_heading_auto_toc_le_numbering_active_legacy`.
   - `walk_arm_preserva_write_paralelo_legacy_para_compute_helpers`.
6. Hash L0 actualizado via `crystalline-lint --fix-hashes`.

### P198C — Promote CounterUpdate (cenário β-promote)

1. Adicionar variant `ElementPayload::CounterUpdate { key: String, action: CounterUpdate }` em `01_core/src/entities/element_payload.rs`.
2. Mover `Content::CounterUpdate` para lista locatable em `01_core/src/rules/introspect/locatable.rs`.
3. Adicionar arm em `01_core/src/rules/introspect/extract_payload.rs`:
   ```rust
   Content::CounterUpdate { key, action } => Some(ElementPayload::CounterUpdate {
       key: key.clone(),
       action: action.clone(),
   }),
   ```
4. Adicionar arm em `01_core/src/rules/introspect/from_tags.rs` para `ElementPayload::CounterUpdate` aplicando `counters.apply_at` (Update/Step flat) ou `counters.apply_hierarchical_at` (Step + key="heading"). Indexar em `kind_index[ElementKind::CounterUpdate]` (nova ElementKind variant — ou reuso de existente).
5. Walk arm `Content::CounterUpdate` preservado (write paralelo M5) — mutações legacy continuam.
6. Adicionar comentário inline P198C declarando E6 fechada estruturalmente.
7. Actualizar L0 introspect.md:
   - Tabela "Excepções M5": linha E6 → "Fechou estruturalmente em P198C (cenário β-promote)".
   - Lista "Ordem inversa": passo 7 marcado ✅ (E6 portion).
   - Nova secção "Walk arm CounterUpdate migrado (P198C, cenário β-promote)".
8. Adicionar 5-6 tests E2E:
   - `counter_update_step_heading_popula_counter_registry`.
   - `counter_update_step_flat_popula_counter_registry`.
   - `counter_update_update_flat_popula_counter_registry`.
   - `counter_update_paridade_legacy_vs_introspector`.
   - `walk_arm_preserva_write_paralelo_legacy_compute_labelled_equation`.
9. Hash L0 actualizado via `crystalline-lint --fix-hashes`.

### P198D — Encerramento série P198

1. Auditoria empírica final (12-14 verificações).
2. Relatório consolidado `typst-passo-198-relatorio-consolidado.md` (9 secções padrão).
3. Nota DEBT M5-residual actualizada (1 excepção activa + 1 residuo).
4. Verificação estrutural final.

---

## §4 Magnitude consolidada

| Sub | Magnitude | LOC produção | LOC teste | LOC L0 | Δ tests |
|-----|-----------|--------------|-----------|--------|---------|
| P198A | S | 0 | 0 | 0 | 0 |
| P198B | S | ~5 (comentário) | ~80 | ~50 | +5 |
| P198C | M | ~80 (variant + arms) | ~120 | ~80 | +6 |
| P198D | S | 0 | 0 | 0 | 0 |

**Total agregado**: **M-** (~85 LOC produção + ~200 LOC tests + ~130 LOC L0 + relatórios).

---

## §5 ADR avaliação

- 3 variantes ADR-0069 cobrem ambos arms (cenário α P197B aplicado em P198B; cenário β-promote = P196B variante operacional aplicada em P198C com leaf content).
- Sem decisão arquitectural nova.
- ElementPayload nova variant é decisão de implementação — não arquitectural.

**Conclusão**: **não cria ADR**.

---

## §6 DEBT M5-residual avaliação

| Excepção | Pré-P198 | Pós-P198 |
|----------|----------|----------|
| E1 (Equation) | activa | activa (independente; passo paralelo `SetEquationNumbering`) |
| E2-residuo | residuo | residuo (passo paralelo sub-store `headings_for_toc`) |
| E5 (SetHeadingNumbering) | activa | **fechada estruturalmente** |
| E6 (CounterUpdate) | activa | **fechada estruturalmente** |

**Estado pós-P198**: **1 excepção activa + 1 residuo** (E1, E2-residuo); 2 pré-requisitos restantes (inalterado).

**Cenário B continua** (sem DEBT formal aberto).

---

## §7 Cadeia E5/E6 com outros arms — análise empírica

### E5 — SetHeadingNumbering ↔ Heading + Equation

**Sequência**:
1. `Content::SetHeadingNumbering { active: true }` percorrida.
2. Walk top emite `Tag::Start(loc, ElementInfo { payload: StateUpdate { key: "numbering_active:heading", update: Set(Bool(true)) }, label: None })`.
3. Walk arm muta `state.numbering_active.insert("heading", true)` (write paralelo).
4. `from_tags` arm StateUpdate posteriormente popula `intr.state` com chave "numbering_active:heading" → Bool(true).
5. Heading subsequente: walk arm chama `compute_heading_auto_toc(state, n)` → lê `state.is_numbering_active("heading")` (legacy) → resolve auto-toc text.
6. Equation block subsequente: walk arm lê `state.is_numbering_active("equation")` (legacy).

**Risco se mutação legacy for removida**: `compute_heading_auto_toc` retornaria string vazia para auto-toc (ainda popula sub-store mas com texto vazio); Equation arm não avançaria counter. Quebra paridade observable.

**Mitigação P198B**: mutação legacy preservada como write paralelo M5. Cadeia E5 preservada.

### E6 — CounterUpdate ↔ compute_* helpers

**Sequência**:
1. `Content::CounterUpdate { key: "equation", action: Step }` percorrida.
2. Walk arm muta `state.step_flat("equation")`.
3. (Pré-P198C) — sem Tag emitida; sem populate em CounterRegistry.
4. (Pós-P198C) — walk top emite `Tag::Start(loc, ElementInfo { payload: CounterUpdate { key, action }, ... })`; from_tags arm popula CounterRegistry via `apply_at`.
5. Equation labelled subsequente: `compute_labelled` Equation arm lê `state.get_flat("equation")` (legacy).
6. Heading subsequente: `compute_heading_auto_toc` lê `state.format_hierarchical("heading")` (legacy).

**Risco se mutação legacy for removida em P198C**: helpers `compute_*` quebrariam (lêm legacy). Quebra paridade observable.

**Mitigação P198C**: mutação legacy preservada. Caminho Introspector activado adicionalmente para futuro consumer (M6 cleanup).

---

## §8 Próximo sub-passo (P198B com escopo concreto)

**P198B — Walk arm SetHeadingNumbering cenário α**:

1. Confirmar via grep que walk arm em `introspect.rs:611-623` invariante.
2. Adicionar comentário inline declarando E5 fechada estruturalmente:
   ```rust
   // P198B — walk arm SetHeadingNumbering cenário α.
   // Caminho Introspector activo desde P182C (extract_payload
   // emite ElementPayload::StateUpdate; from_tags arm popula
   // StateRegistry sob chave numbering_active:heading).
   // Mutação legacy preservada como write paralelo M5 porque
   // compute_heading_auto_toc P196B lê state.is_numbering_active
   // durante walk; cadeia E5 preservada. Cleanup orgânico em M6.
   ```
3. Actualizar L0 introspect.md:
   - Tabela Excepções M5: E5 fechada estruturalmente em P198B.
   - Secção nova "Walk arm SetHeadingNumbering migrado (P198B, cenário α)".
4. Adicionar 5 tests sentinela em `mod tests`.
5. Hash L0 actualizado.

**Critério de fecho P198B**: tests workspace 1.848 + 5 = 1.853 verdes; lint zero violations; L0 hash actualizado; comentário inline presente.
