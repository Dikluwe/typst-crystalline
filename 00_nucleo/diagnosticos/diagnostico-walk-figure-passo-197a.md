# Diagnóstico — Walk arm Figure (P197A)

**Data**: 2026-05-04
**Pattern arquitectural relevante**: ADR-0069 — 3ª aplicação potencial.
**Variante candidata**: P196B (locatable, `emitted_loc` directo).
**Estado empírico**: confirmado por grep + leitura directa de `01_core/src/`.

---

## §1 Validação estado actual

### §1.1 Walk arm Figure — `01_core/src/rules/introspect.rs:490-519`

```rust
Content::Figure { body, caption, kind, numbering } => {
    // Excepção M5 E3 (Figure): walk muta state.figure_numbers
    // directamente porque `Labelled` arm lê figure_numbers
    // durante walk para popular state.figure_label_numbers
    // (cadeia chained com E2 — Reserva 2 alargada). Sub-store
    // existe (P184B figure_numbers + P168 figure_label_numbers)
    // mas Labelled arm precisa de migrar primeiro.
    if numbering.is_some() && caption.is_some() {
        let kind_key = kind.as_deref().unwrap_or("image").to_string();
        let counter = state.local_figure_counters
            .entry(kind_key.clone())
            .or_insert(0);
        *counter += 1;
        let figure_number = *counter;
        state.figure_numbers
            .entry(kind_key)
            .or_default()
            .push(figure_number);
    }
    walk(body, state, locator, tags, None);
    if let Some(cap) = caption {
        walk(cap, state, locator, tags, None);
    }
}
```

**2 mutações sob gate `numbering.is_some() && caption.is_some()`**:
1. `state.local_figure_counters.entry(kind_key).or_insert(0); *counter += 1;` — counter local per-kind.
2. `state.figure_numbers.entry(kind_key).or_default().push(figure_number);` — registo global per-kind.

### §1.2 Variant `Content::Figure` — `01_core/src/entities/content.rs:202`

```rust
Figure {
    body:      Box<Content>,
    caption:   Option<Box<Content>>,
    kind:      Option<String>,
    numbering: Option<EcoString>,
}
```

### §1.3 `is_locatable(Content::Figure) = true`

Confirmado em `01_core/src/rules/introspect/extract_payload.rs:27`:

```rust
Content::Figure { kind, numbering, caption, .. } => Some(ElementPayload::Figure {
    kind:           kind.clone(),
    counter_update: CounterUpdate::Step,
    is_counted:     numbering.is_some() && caption.is_some(),
}),
```

`extract_payload` retorna `Some(...)` → walk top emite `Tag::Start(loc, ElementInfo::new(payload))` antes do match arm; `emitted_loc: Option<Location>` é `Some(loc)` no escopo da arm Figure.

### §1.4 Variant `ElementPayload::Figure` — `01_core/src/entities/element_payload.rs:48-63`

```rust
Figure {
    kind:           Option<String>,
    counter_update: CounterUpdate,
    is_counted:     bool,  // P168: numbering.is_some() && caption.is_some()
}
```

**Variant existe + cobre semântica de gate**. `is_counted` propaga predicado para from_tags. **Cenário α confirmado** (cláusula 1).

### §1.5 `from_tags` arm Figure — `01_core/src/rules/introspect/from_tags.rs:72-113`

```rust
ElementPayload::Figure { kind, counter_update, is_counted, .. } => {
    intr.kind_index.entry(ElementKind::Figure).or_default().push(*loc);
    let kind_key = kind.as_deref().unwrap_or("image");
    intr.counters.apply_at(format!("figure:{}", kind_key), counter_update.clone(), *loc);
    intr.counters.apply_at("figure".to_string(), counter_update.clone(), *loc);
    if *is_counted {
        if let Some(label) = &info.label {
            let next_num = intr.figure_label_numbers.len() + 1;
            intr.figure_label_numbers.entry(label.clone()).or_insert(next_num);
        }
    }
}
```

**Já popula 4 sub-stores**:
- `kind_index[Figure]` — locations.
- `counters` (CounterRegistry) — chave `figure:{kind}` per-kind via `apply_at`.
- `counters` (CounterRegistry) — chave `figure` global via `apply_at` (write paralelo M6).
- `figure_label_numbers` — quando `is_counted && label`.

### §1.6 Sub-stores existentes para Figure

| Sub-store | Existência | Populado em produção |
|-----------|------------|---------------------|
| `intr.kind_index[Figure]` | ✅ existe | ✅ sim (from_tags arm Figure) |
| `intr.counters` chave `figure:{kind}` | ✅ existe (CounterRegistry P184B) | ✅ sim |
| `intr.counters` chave `figure` global | ✅ existe | ✅ sim (write paralelo M6) |
| `intr.figure_label_numbers` | ✅ existe (P168) | ✅ sim quando is_counted+label |
| `intr.figure_numbers` directo (HashMap separado) | ❌ não existe | — equivalente via CounterRegistry |
| `intr.local_figure_counters` | ❌ não existe | sem consumer → desnecessário |

**Achado crítico**: sub-store equivalente para `state.figure_numbers` **já existe** via CounterRegistry (`figure:{kind}`). Consumer C3 (`mod.rs:484`) já usa via `figure_number_at_index` (P184C). **Caminho Introspector já activo em produção**.

### §1.7 Consumer downstream `state.figure_numbers`

**`grep -rn "figure_numbers" 01_core/src/`** identifica 3 consumers cristalinos:

1. **`layout/mod.rs:484-491`** — substitution-with-fallback P184D:
   ```rust
   let figure_number = self.introspector
       .figure_number_at_index(kind_key, idx)
       .or_else(|| self.counter.figure_numbers.get(kind_key).and_then(|v| v.get(idx)).copied())
       .unwrap_or(idx + 1);
   ```
   **Caminho Introspector já activo**. Fallback legacy persiste mas raramente disparado.

2. **`introspect.rs:344-365` (`compute_labelled` Figure arm, P195D)** — lê `state.figure_numbers.last()` durante walk:
   ```rust
   Content::Figure { kind, numbering, caption, .. } => {
       let kind_key = kind.as_deref().unwrap_or("image");
       let n = if numbering.is_some() && caption.is_some() {
           state.figure_numbers.get(kind_key).and_then(|v| v.last()).copied().unwrap_or(0)
       } else { 0 };
       …
   }
   ```
   **Cadeia E2-E3 confirmada empiricamente**: `compute_labelled` precisa que `state.figure_numbers` esteja mutado pré-recursão por walk arm Figure.

3. **`layout/tests.rs:4895`** — sentinela E3 P189B:
   ```rust
   let nums = state.figure_numbers.get("image").cloned().unwrap_or_default();
   assert_eq!(nums, vec![1], "E3: figure_numbers[image] populado");
   ```

### §1.8 Consumer downstream `state.local_figure_counters`

**`grep -rn "local_figure_counters" 01_core/src/`** identifica apenas:
1. **Definição em `counter_state_legacy.rs:64-70`** com comentário explícito:
   > *Não exposto ao layouter; apenas `figure_numbers` é consumido externamente.*
2. **Walk arm Figure** (origem da mutação).

**Conclusão**: `local_figure_counters` é **walk-internal apenas**. Sem consumer downstream. Sem necessidade de sub-store novo.

### §1.9 Cadeia E2-E3 confirmada

`compute_labelled` Figure arm (P195D, introspect.rs:344) lê `state.figure_numbers.last()`. Walk arm Labelled chama `compute_labelled` durante walk pós-recursão. Sequência:
1. Walk arm Figure muta `state.figure_numbers` pré-recursão no body.
2. Walk recursivo no body do Figure (não muta figure_numbers).
3. Walk recursivo no caption do Figure (não muta figure_numbers).
4. Walk volta ao caller. Se caller é Labelled wrapper sobre o Figure → Labelled arm chama `compute_labelled(Figure, state)` → lê `state.figure_numbers.last()` → encontra número recém-inserido.

**Cláusula gate substancial**: remover mutação `state.figure_numbers.push` quebraria `compute_labelled` Figure arm. **Mutação legacy DEVE ser preservada como write paralelo durante M5**. Cleanup orgânico em M6 quando `compute_labelled` migrar para CounterRegistry.

### §1.10 Tests sentinela E3

- `layout/tests.rs:4895` (P189B): valida `state.figure_numbers["image"] == vec![1]`.
- `layout/tests.rs:4026` (P184D): valida fallback `or_else` caminho legacy.
- `from_tags.rs:412-465` (P168): testes `figura_numerada_com_label_popula_figure_label_numbers` etc.

Devem manter-se **inalterados** após P197B — cenário α não modifica payload nem from_tags arm.

### §1.11 Regra dos 2 eixos aplicada

| Eixo | Estado | Implicação |
|------|--------|------------|
| Eixo 1 (consumer precisa de valor durante walk ou snapshot final?) | Snapshot final (`mod.rs:484`); MAS `compute_labelled` precisa **durante walk** | Cadeia E2-E3 obriga preservar mutação legacy |
| Eixo 2 (sub-store equivalente existe?) | ✅ via CounterRegistry (`figure:{kind}`); + figure_label_numbers para is_counted+label | Caminho Introspector já activo |

**Conclusão**: caminho Introspector já existe e está activo para consumer C3. Mutação legacy `state.figure_numbers` mantida porque `compute_labelled` Figure arm (P195D) ainda lê dela. Mutação legacy `state.local_figure_counters` mantida apenas como walk-internal trivial — sem consumer.

### §1.12 Workspace baseline

- Tests: 1.843 verdes.
- Linter: zero violations.
- Hash L0 introspect: `3bc33823`.
- Hash código introspect: `73489ae5`.

---

## §2 Decisões cláusula 1–7 (formato O1–O5)

### Cláusula 1 — Forma do payload

- **O1 (input)**: variant `ElementPayload::Figure { kind, counter_update, is_counted }` já existe (element_payload.rs:48).
- **O2 (alternativas)**:
  - α — reuso directo (sem mudança).
  - β — expandir adicionando `number: Option<usize>` (post-recursion exigida).
  - γ — variant nova.
- **O3 (critério)**: variant cobre semântica de gate (is_counted) + counter_update. Número computado via CounterRegistry post-walk via `figure_number_at_index`.
- **Decisão**: **cenário α** — reuso directo. Sem expansão.
- **O4 (magnitude)**: zero LOC variant.
- **O5 (reversibilidade)**: trivial.

### Cláusula 2 — Helper `compute_figure`

- **O1 (input)**: walk arm Figure tem cómputo inline do número (lines 504-509).
- **O2 (alternativas)**:
  - α — extrair helper `compute_figure(state, kind, is_counted) -> Option<usize>` (replica padrão P195D `compute_labelled` / P196B `compute_heading_auto_toc`).
  - β — manter inline.
- **O3 (critério)**: consistência com pattern ADR-0069 (helper privado para isolamento). Permite reuso futuro se `compute_labelled` Figure arm migrar para chamar este helper.
- **Decisão**: **opção α** — extrair helper.
- **O4 (magnitude)**: ~15-20 LOC.
- **O5 (reversibilidade)**: refactor puro.

### Cláusula 3 — Tratamento de `local_figure_counters`

- **O1 (input)**: comentário em `counter_state_legacy.rs:69`: "*Não exposto ao layouter; apenas `figure_numbers` é consumido externamente*". Sem consumer downstream.
- **O2 (alternativas)**:
  - α — walk-internal apenas; preservar mutação legacy trivial; sem sub-store novo.
  - β — abrir sub-store dedicado.
- **O3 (critério)**: sem consumer downstream → sub-store novo é overhead injustificado. Mutação legacy preservada simplesmente.
- **Decisão**: **opção α** — walk-internal apenas; sem sub-store novo.
- **O4 (magnitude)**: zero LOC.
- **O5 (reversibilidade)**: trivial.

### Cláusula 4 — Cadeia E2-E3 (interacção com Labelled P195D)

- **O1 (input)**: `compute_labelled` Figure arm (introspect.rs:344-365) lê `state.figure_numbers.last()` durante walk pós-recursão.
- **O2 (alternativas)**:
  - α — preservar mutação legacy `state.figure_numbers.push` durante janela compat M5; `compute_labelled` continua a funcionar via legacy. Migração `compute_labelled` adiada para M6.
  - β — migrar `compute_labelled` para ler de CounterRegistry imediatamente. Risco: `value_at_index` exige idx; tracking de idx em walk recursivo é não-trivial.
- **O3 (critério)**: opção α minimiza acoplamento e risco de regressão. Cleanup orgânico em M6.
- **Decisão**: **opção α** — preservar mutação legacy. Cláusula gate substancial **resolvida sem disparar gate**.
- **O4 (magnitude)**: zero LOC mudança.
- **O5 (reversibilidade)**: trivial.

### Cláusula 5 — Locator handling

- **O1 (input)**: Figure é locatable; `emitted_loc: Option<Location>` é `Some(loc)` no escopo da arm.
- **O2 (alternativas)**:
  - α — variante P196B (`emitted_loc` directo).
  - β — variante P195D (snapshot+find_map) — desnecessária.
- **O3 (critério)**: Figure é locatable → variante P196B trivial.
- **Decisão**: **variante P196B** disponível mas **não utilizada** em P197B porque cenário α não emite Tag pós-recursão (sub-store equivalente já populated via from_tags arm Figure existing).
- **O4 (magnitude)**: zero LOC adição de Tag.
- **O5 (reversibilidade)**: trivial.

### Cláusula 6 — Mutação legacy preservada

- **O1 (input)**: padrão P195D + P196B preserva mutação legacy como write paralelo M5.
- **O2 (alternativas)**:
  - α — preservar (replica padrão).
  - β — remover (quebra cadeia E2-E3).
- **O3 (critério)**: cláusula 4 obriga preservação.
- **Decisão**: **opção α** — write paralelo M5; cleanup em M6.
- **O4 (magnitude)**: zero LOC remoção.
- **O5 (reversibilidade)**: trivial.

### Cláusula 7 — Critério de fecho de P197

- **O1 (input)**: E3 declarada activa em P189B §5; sub-stores destinos confirmados em §1.6.
- **O2 (alternativas)**:
  - α — E3 fecha **estruturalmente** porque ambas as 2 mutações têm caminho Introspector destino activo (Mut 2 via CounterRegistry; Mut 1 sem consumer downstream → trivial).
  - β — E3 fica activa até cleanup M6.
- **O3 (critério)**: alinhamento com declaração P189B §5 + estado empírico §1.6.
- **Decisão**: **opção α** — E3 fecha estruturalmente em P197B. Funcional fecha em M6.
- **O4 (magnitude)**: declaração em L0.
- **O5 (reversibilidade)**: trivial.

---

## §3 Plano de sub-passos sem condicionais

### P197B — Walk arm Figure refactor + helper + L0 + tests

Magnitude **S/M** (sem novo Tag, sem nova variant, sem novo sub-store; apenas extração de helper + declarações):

1. Adicionar helper `compute_figure(state, kind, is_counted) -> Option<usize>` em `01_core/src/rules/introspect.rs` antes da `walk` fn. Função pura sobre `(state, kind, is_counted)` retornando o número que vai ser inserido.
2. Refactor walk arm Figure (introspect.rs:490-519) para chamar helper:
   - Mutações legacy preservadas (chamam helper para obter `n`, mantêm `*counter += 1` + `figure_numbers.push(n)`).
   - Adicionar comentário declarando E3 fechada estruturalmente via existing extract_payload + from_tags.
3. Actualizar L0 `00_nucleo/prompts/rules/introspect.md`:
   - Tabela "Excepções M5": E3 → "Fechou estruturalmente em P197B" (análogo a E4 P195D).
   - Lista "Ordem inversa à mutação": passo 6 marcado ✅.
   - Nova secção "Walk arm Figure migrado (P197B, ADR-0069 cenário α)".
4. Adicionar 5 testes E2E em `mod tests`:
   - `figure_extract_payload_emite_is_counted_true_para_numbered_caption`.
   - `figure_extract_payload_emite_is_counted_false_para_sem_caption`.
   - `figure_from_tags_popula_counter_registry`.
   - `consumer_c3_recebe_some_via_figure_number_at_index`.
   - `walk_arm_figure_preserva_write_paralelo_legacy_para_compute_labelled`.
5. Hash L0 actualizado via `crystalline-lint --fix-hashes`.

### P197C — Relatório consolidado P197 + actualização DEBT

Magnitude **S puro**:

1. Auditoria empírica (12 verificações — replica P196C).
2. Relatório consolidado `typst-passo-197-relatorio-consolidado.md` (9 secções padrão).
3. Nota DEBT M5-residual actualizada (3 excepções activas + 1 residuo).
4. Verificação estrutural final.

---

## §4 Magnitude consolidada

| Sub | Magnitude | LOC produção | LOC teste | LOC L0 | Δ tests |
|-----|-----------|--------------|-----------|--------|---------|
| P197A | S | 0 | 0 | 0 | 0 |
| P197B | S/M | ~30 (helper + refactor) | ~80 (5 tests) | ~50 (secção nova) | +5 |
| P197C | S | 0 | 0 | 0 | 0 |

**Total agregado**: **S+ a M-** (significativamente menor que P196B+P196C devido a cenário α — variant + sub-store equivalente já existem).

---

## §5 ADR avaliação

- Pattern ADR-0069 disponível mas **não aplicado em P197B** (cenário α — variant cobre, sub-store via CounterRegistry).
- Sem decisão arquitectural nova.

**Conclusão**: **não cria ADR**.

---

## §6 DEBT M5-residual avaliação

| Excepção | Pré-P197 | Pós-P197B |
|----------|----------|-----------|
| E1 (Equation) | activa | activa (independente) |
| E2-residuo (Heading headings_for_toc) | residuo | residuo |
| **E3 (Figure)** | activa | **fechada estruturalmente** |
| E4 (Labelled) | fechada estruturalmente (P195D) | fechada estruturalmente |
| E5 (SetHeadingNumbering) | activa | activa |
| E6 (CounterUpdate) | activa | activa |

**Estado pós-P197B**:
- 3 excepções activas + 1 residuo (E1, E2-residuo, E5, E6).
- 2 pré-requisitos restantes (inalterado: `headings_for_toc` para E2-residuo; `SetEquationNumbering` para E1).

**Cenário B continua** (sem DEBT formal aberto).

---

## §7 Cadeia E2-E3 (interacção com Labelled P195D) — análise empírica

### Sequência durante walk

Considere `Content::Labelled { label: "fig1", target: Content::Figure { numbering: Some("1"), caption: Some(...), ... } }`:

1. Walk top em `Labelled` — não-locatable, `emitted_loc = None`.
2. `walk(target=Figure, state, locator, tags, Some(label))`.
3. Walk top em Figure — locatable, walk emite `Tag::Start(loc, ElementInfo { payload: Figure {kind, ..., is_counted: true}, label: Some("fig1") })`.
4. Match arm `Content::Figure`:
   - Mut 1: `local_figure_counters.entry(kind).or_insert(0); *counter += 1` → 1.
   - Mut 2: `figure_numbers.entry(kind).or_default().push(1)`.
   - Walk recursivo no body + caption.
5. Walk bottom em Figure — emite `Tag::End(loc, hash_content)`.
6. Volta ao walk arm `Labelled` em introspect.rs:521+.
7. `compute_labelled(Figure, state)` chamada (introspect.rs:344): lê `state.figure_numbers["image"].last() = Some(&1)`.
8. Walk arm `Labelled` insere `state.figure_label_numbers.insert("fig1", 1)` + emite Tag::Labelled pós-recursão (P195D).

### Risco se `state.figure_numbers.push` for removido

Step 7 (`compute_labelled`) lê `state.figure_numbers["image"].last()`. Se mutação legacy for removida, `state.figure_numbers["image"]` fica vazio → `.last() = None` → `n = 0` → `(Some(""), None)` retornado. Walk arm Labelled NÃO insere `figure_label_numbers`; Tag::Labelled emitida com `figure_number: None`. **Quebra paridade observable**: consumer de figure_label_numbers (referências para figura) recebe None onde deveria receber Some.

### Mitigação P197 cenário α

**Mutação `state.figure_numbers.push` preservada como write paralelo M5**. Cadeia E2-E3 continua funcional. `compute_labelled` Figure arm não é tocado.

### Cleanup M6 (futuro)

Em M6, `compute_labelled` Figure arm migra para ler de CounterRegistry via uma API location-aware (e.g., `flat_counter_at("figure:{kind}", current_location)`). Quando isso acontecer, mutação legacy `state.figure_numbers.push` torna-se redundante e pode ser removida.

---

## §8 Próximo sub-passo (P197B com escopo concreto)

**P197B — Walk arm Figure refactor (cenário α)**:

1. Adicionar helper `compute_figure(state, kind, is_counted) -> Option<usize>` antes da `walk` fn em `01_core/src/rules/introspect.rs`. Lógica: se `!is_counted`, retorna `None`. Caso contrário, projecta o próximo número que vai ser inserido em `state.figure_numbers[kind_key]` (1-based).

2. Refactor walk arm Figure (introspect.rs:490-519) para usar helper:
   ```rust
   Content::Figure { body, caption, kind, numbering } => {
       let kind_key = kind.as_deref().unwrap_or("image").to_string();
       let is_counted = numbering.is_some() && caption.is_some();
       if let Some(n) = compute_figure(state, &kind_key, is_counted) {
           // Mutações legacy preservadas (write paralelo M5) — cadeia E2-E3
           // exige que compute_labelled Figure arm continue a ler
           // state.figure_numbers durante walk.
           let counter = state.local_figure_counters.entry(kind_key.clone()).or_insert(0);
           *counter += 1;
           state.figure_numbers.entry(kind_key).or_default().push(n);
       }
       walk(body, state, locator, tags, None);
       if let Some(cap) = caption {
           walk(cap, state, locator, tags, None);
       }
   }
   ```

3. Actualizar L0 `00_nucleo/prompts/rules/introspect.md`:
   - Tabela "Excepções M5": linha E3 → "**Fechou estruturalmente em P197B**" (análogo a E4 P195D).
   - Lista "Ordem inversa à mutação": passo 6 marcado ✅.
   - Nova secção "Walk arm Figure migrado (P197B, ADR-0069 cenário α — sub-store via CounterRegistry)".

4. Adicionar 5 testes E2E em `mod tests` (sentinelas E3-fechada-estruturalmente):
   - `figure_extract_payload_emite_is_counted_true_para_numbered_caption`.
   - `figure_extract_payload_emite_is_counted_false_para_sem_caption`.
   - `figure_from_tags_popula_counter_registry_via_apply_at`.
   - `consumer_c3_recebe_some_via_figure_number_at_index_sem_fallback`.
   - `walk_arm_figure_preserva_write_paralelo_legacy_para_compute_labelled`.

5. Hash L0 actualizado via `crystalline-lint --fix-hashes`.

**Critério de fecho P197B**: 5 testes novos passam, tests sentinela E3 P189B (`layout/tests.rs:4895`) inalterados, lint zero violations, L0 hash actualizado, walk arm refactored com helper extraído + comentário declarando E3 fechada estruturalmente.
