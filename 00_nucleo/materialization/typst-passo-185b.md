# Passo P185B — Trait methods location-aware

Primeiro passo de implementação P185 (após P185A diagnóstico
+ ADR-0068 PROPOSTO).
Magnitude **S**.

Adiciona dois métodos location-aware ao trait `Introspector`:
`is_numbering_active_at(key, location)` e
`flat_counter_at(key, location)`. Impl em `TagIntrospector`
delega a `StateRegistry::value_at` e
`CounterRegistry::value_at` respectivamente. Padrão P177
(`formatted_counter_at`) replicado para 2 métodos novos.

Após P185B:
- Trait `Introspector` ganha 2 métodos location-aware.
- `TagIntrospector` impl funciona contra dados populados
  via P171/P184B.
- Tests unitários cobrem populate + lookup por Location +
  re-update (a Location consultada determina o valor
  retornado).
- Layouter ainda **não** consulta estes métodos (P185C
  introduz `current_location` field).

**Pré-condição**: P185A concluído. Tests workspace 1.769
verdes; zero violations. ADR-0068 PROPOSTO criada
documentando mecanismo M3.

**Restrições**:
- **Não** modificar Layouter — P185C.
- **Não** modificar `Locator` — P185C usa o existente.
- **Não** modificar `StateRegistry` (`value_at` já existe
  per P171).
- **Não** modificar `CounterRegistry` (`value_at` já
  existe per P177).
- **Não** modificar walk arm, `extract_payload`,
  `from_tags`.
- **Não** migrar consumers C1+C2 — P187/P188.
- API pública preservada.
- Output observable em produção inalterado — métodos
  novos sem consumer ainda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar trait `Introspector` actual:
   - `01_core/src/entities/introspector.rs`.
   - Localizar definição do trait.
   - Identificar localização sugerida para inserir
     `is_numbering_active_at` e `flat_counter_at`:
     - Junto a `is_numbering_active` (P182B) para
       simetria.
     - Junto a `formatted_counter_at` (P177) para
       agrupamento por categoria location-aware.

2. Confirmar `StateRegistry::value_at`:
   - `01_core/src/entities/state_registry.rs` (ou
     similar).
   - Assinatura: `value_at(&self, key: &str, location:
     Location) -> Option<Value>` ou `Option<&Value>`.
   - Confirmar empiricamente.

3. Confirmar `CounterRegistry::value_at`:
   - `01_core/src/entities/counter_registry.rs`.
   - Assinatura: `value_at(&self, key: &str, location:
     Location) -> Option<&[usize]>` ou similar (per
     P177).
   - Confirmar empiricamente.

4. Confirmar `Value::Bool` matching:
   - P182B usa `matches!(self.state.final_value(key),
     Some(Value::Bool(true)))`.
   - P185B usa mesma forma com `value_at` em vez de
     `final_value`.

5. Confirmar L0 actual `entities/introspector.md`:
   - Localizar entradas existentes.
   - Verificar onde adicionar 2 entradas novas.

6. Confirmar `TagIntrospector` impl block:
   - Localizar onde adicionar os 2 métodos novos.
   - Confirmar field names dos sub-stores
     (`state`, `counters`, etc.).

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída e gate de decisão**:
- Se `value_at` em algum sub-store tem assinatura
  diferente do esperado: cláusula gate trivial —
  adaptar.
- Se `Location` precisa de import explícito: adicionar.
- Senão prosseguir.

### .B Actualizar L0 `entities/introspector.md`

1. Adicionar entrada para `is_numbering_active_at`:
   - Assinatura: `fn is_numbering_active_at(&self, key:
     &str, location: Location) -> bool`.
   - Propósito: consulta se numeração está activa para
     chave dada **na Location especificada**. Usa
     snapshot por Location, não snapshot final.
   - Default: `false` quando state ausente em Location
     ou valor não-Bool.
   - Implementação: delega a `state.value_at(key,
     location)` + match `Some(Value::Bool(true))`.

2. Adicionar entrada para `flat_counter_at`:
   - Assinatura: `fn flat_counter_at(&self, key: &str,
     location: Location) -> Option<usize>`.
   - Propósito: consulta valor do contador flat (1
     elemento) para chave dada **na Location
     especificada**.
   - Default: `None` quando counter ausente em Location.
   - Implementação: delega a `counters.value_at(key,
     location)?.last().copied()`.

3. Cross-reference: P177 (`formatted_counter_at`),
   P182B (`is_numbering_active`), P184C
   (`figure_number_at_index`). Padrão `*_at` para
   location-aware.

4. Hashes em branco aguardam recálculo manual.

**Critério de saída**:
- L0 contém 2 entradas novas.
- Coerente com convenção dos métodos existentes.

### .C Adicionar métodos ao trait

1. Em `01_core/src/entities/introspector.rs`:
   - Adicionar declarações ao trait `Introspector`:
     - `fn is_numbering_active_at(&self, key: &str,
       location: Location) -> bool;`
     - `fn flat_counter_at(&self, key: &str, location:
       Location) -> Option<usize>;`
   - Posicionar conforme `.A.1` (junto aos métodos
     location-aware existentes).

2. Documentação inline: 1-3 linhas por método explicando
   propósito + comportamento default + diferença face a
   versões snapshot-final (`is_numbering_active`,
   `formatted_counter`).

**Critério de saída**:
- `cargo check --workspace` falha com erros esperados em
  impls obrigados a implementar (será corrigido em
  `.D`).
- Linter passa.

### .D Implementar métodos em `TagIntrospector`

1. Em `impl Introspector for TagIntrospector`:
   - `is_numbering_active_at`: delega a
     `self.state.value_at(key, location)` + match
     `Some(Value::Bool(true))`.
   - `flat_counter_at`: delega a
     `self.counters.value_at(key, location)?.last().copied()`.

2. Forma exacta fica para Claude Code conforme
   convenção do projecto. Padrão P182B (`is_numbering_active`)
   + P184C (`figure_number_at_index`) replicado.

3. Confirmar cabeçalho de linhagem `@prompt-hash`
   actualiza após edit do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- `cargo build --workspace` passa.
- Linter passa.

### .E Tests unitários

8-10 tests obrigatórios (per P185A §14):

#### Para `is_numbering_active_at` (4-5 tests):

1. **Vazio devolve `false`** — `TagIntrospector::empty()`
   + chamada `is_numbering_active_at("numbering_active:heading",
   loc(0))` → `false`.

2. **Populate Bool(true) retorna `true`** — `state.init`
   com `Value::Bool(true)` em loc(10); chamada com loc(15)
   retorna `true` (location posterior à init).

3. **Re-update reflecte Location consultada** — populate
   `init(true, loc(10))` + `update(false, loc(20))`;
   chamada com `loc(15)` → `true`; chamada com `loc(25)`
   → `false`. **Caso central** que valida diferença face
   a `is_numbering_active` (snapshot-final).

4. **Bool(false) retorna `false`** — populate explícito
   `Value::Bool(false)`; retorna `false`.

5. **Non-Bool value retorna `false`** — populate
   `Value::Int(1)` ou similar; retorna `false`
   (graceful degradation).

#### Para `flat_counter_at` (4-5 tests):

6. **Vazio devolve `None`** — empty + chamada
   `flat_counter_at("figure:image", loc(0))` → `None`.

7. **Populate retorna valor por Location** — `apply_at`
   com `Step` em loc(10); chamada com loc(15) →
   `Some(1)`.

8. **Re-update reflecte Location consultada** —
   sequência `apply_at(Step, loc(10))` + `apply_at(Step,
   loc(20))` + `apply_at(Step, loc(30))`;
   chamada com `loc(15)` → `Some(1)`;
   chamada com `loc(25)` → `Some(2)`;
   chamada com `loc(35)` → `Some(3)`.
   **Caso central** que valida snapshot por Location.

9. **Keys distintas isoladas** — `apply_at` em
   `figure:image` e `figure:table` separadamente;
   chamadas retornam valores correctos por chave.

10. **Location anterior a qualquer apply** — chamada com
    Location anterior à primeira `apply_at` retorna
    `None` (snapshot vazio).

Tests co-localizados em `mod tests` dentro de
`introspector.rs`. Helpers replicam padrão P182B e P184C.

**Critério de saída**:
- 8-10 tests novos passam.
- Tests existentes não regridem (1.769 + 8-10 = 1.777
  a 1.779 esperado).

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P185A
   baseline (1.769): +8 a +10.
3. `crystalline-lint .` zero violations.
4. `is_numbering_active_at` accessível via trait.
5. `flat_counter_at` accessível via trait.
6. `TagIntrospector` impls delegam correctamente.
7. Re-update casos retornam valor por Location (não
   snapshot final) — caso central P182E §5.2 atendido.
8. Walk **NÃO modificado**.
9. Layouter **NÃO modificado** (esperado em P185C).
10. Snapshot tests ADR-0033 verdes.
11. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-185b-relatorio.md`
com:

- Resumo: 2 trait methods location-aware materializados;
  impls delegam a sub-stores (`StateRegistry`,
  `CounterRegistry`); 8-10 tests unit cobrem casos
  típicos incluindo re-update.
- Confirmação `.F` (11 verificações).
- Δ tests vs baseline P185A (esperado +8 a +10).
- Hashes finais de L0 modificado (`introspector.md`).
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P185 série: A ✅ B ✅ | C-E pendentes.
  - M9: 11/11 (inalterado — métodos novos são extensão
    location-aware, não M9 feature nova).
  - **Trait `Introspector`**: 16 → 18 métodos.
  - 45 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P185C (Layouter `locator` +
  `current_location` fields; gating em
  `layout_content`). **Magnitude M genuíno** — primeira
  introdução de Locator no Layouter.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. L0 `entities/introspector.md` actualizado com 2
   entradas novas.
3. 2 métodos declarados no trait.
4. 2 métodos implementados em `TagIntrospector`.
5. 8-10 tests novos passam.
6. Tests existentes não regridem.
7. Verificações `.F` passam (11/11).
8. Relatório `.G` escrito.
9. Output observable em produção inalterado (métodos
   novos sem consumer ainda).

---

## O que pode sair errado

- **`value_at` em sub-stores tem assinatura diferente**:
  cláusula gate trivial — adaptar matching.
- **`Location` precisa de import**: trivial.
- **`Value::Bool` ausente ou diferente**: improvável;
  mesma forma de P182B.
- **Trait method exige `&mut self`**: improvável; método
  é read-only.
- **Tests `value_at` em `StateRegistry` ou
  `CounterRegistry` exigem helpers específicos**:
  replicar padrão de tests existentes para os sub-stores
  (provavelmente já há helpers `init`/`apply_at` para
  setup de tests).
- **`flat_counter_at` retorna valor inesperado para
  contador hierárquico** (heading em vez de figure):
  caso edge — heading counter tem `Vec<usize>` com mais
  que 1 elemento; `.last()` retorna o nível mais
  profundo. Documentar inline que `flat_counter_at` é
  para counters flat (figure, equation); para heading
  usar `formatted_counter_at` (P177).
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~80-150 LOC (2 declarações + 2
  impls + ~80 LOC de tests).
- **Sem dependências externas novas**.
- **Sem novo sub-store**.
- **Sem cláusula gate substancial esperada**.
- **Pré-condição P185C**: este passo concluído.
- **Padrão replicado**:
  - P182B (`is_numbering_active`) com `value_at` em vez
    de `final_value`.
  - P177 (`formatted_counter_at`) + P184C (`figure_number_at_index`)
    no estilo de delegação.
- **Cláusula gate trivial**: aplicável a assinaturas,
  imports, helpers de tests.
- **Cláusula gate substancial**: aplicável apenas se
  `value_at` em sub-stores tiver semântica diferente do
  esperado (improvável dado P171/P177).
- **Test re-update é o caso central** que valida
  diferença entre snapshot-final (atendido por
  `is_numbering_active`/`formatted_counter`) e snapshot
  por Location (atendido pelos métodos `*_at`). Sem
  este test, a infra location-aware seria
  funcionalmente equivalente a snapshot-final — o que
  bloquearia C1+C2.
- **Métodos novos não são M9 feature nova** — são
  extensão location-aware de métodos existentes (P177
  já tinha estabelecido `*_at` como padrão). M9 contador
  permanece 11/11.
