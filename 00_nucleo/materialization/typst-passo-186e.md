# Passo P186E — `from_tags` arm `Equation` com gate

Quarto passo de implementação P186 (após P186A diagnóstico,
P186B variants, P186C `is_locatable`, P186D `extract_payload`).
Magnitude **S**.

Adiciona arm em `from_tags` para processar
`ElementPayload::Equation { block, counter_update }`. Aplica
gate `block && state.is_numbering_active("equation")` per
cláusula 4 P186A. Quando gate dispara, chama
`counters.apply_at("equation", CounterUpdate::Step,
location)`.

Per achado P186A §11.2: `Content::SetEquationNumbering` não
existe em cristalino, logo `state.is_numbering_active("equation")`
é sempre `false` em produção. Gate **dorme** — counter nunca
populado em produção real. Estado correcto por construção;
infra pronta para activação futura.

Após P186E:
- Walk → Tag → `from_tags` → `CounterRegistry` populado
  **se gate dispara** (apenas em testes E2E que injectam
  state ou em produção futura com `SetEquationNumbering`).
- `flat_counter_at("equation", current_location)` retorna
  valor correcto quando gate disparou; `None` caso
  contrário.
- C2 (P188) vai migrar para usar Introspector path; em
  produção fallback legacy continua activo (gate dormente).

**Pré-condição**: P186D concluído. Tests workspace 1.793
verdes; zero violations. Variant `ElementPayload::Equation`
produzida por `extract_payload`. Walk emite Tag locatable
para Equation. `from_tags` arm Equation populates
`kind_index[Equation]` (P186D estendeu stub via cláusula
gate trivial — vide P186D §"Decisões"). Falta apenas
counter logic. Invariante `is_locatable ↔
extract_payload.is_some()` intacta.

**Restrições**:
- **Não** modificar walk arm legacy
  (`introspect.rs:377-382`).
- **Não** migrar consumer C2 — P188.
- **Não** modificar `state.is_numbering_active` API.
- API pública preservada.
- Output observable em produção inalterado — gate dormente
  garante que counter permanece vazio em prática.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `from_tags` actual:
   - `01_core/src/rules/introspect/from_tags.rs`.
   - Localizar match sobre `ElementPayload::*`.
   - Identificar arms existentes (Heading, Figure,
     Bibliography, Outline, etc.).
   - Identificar onde inserir arm para
     `ElementPayload::Equation`.

2. Confirmar API `CounterRegistry::apply_at`:
   - P184B padrão: `apply_at(key: String, update:
     CounterUpdate, location: Location)`.
   - Confirmar empiricamente.

3. Confirmar API `state.is_numbering_active`:
   - Trait `Introspector` ou método em `StateRegistry`.
   - Per P182B: trait method `is_numbering_active(key:
     &str) -> bool` — usa snapshot final. **Não-at**.
   - Per P185B: trait method `is_numbering_active_at(key:
     &str, location: Location) -> bool` —
     location-aware.
   - **Decisão pendente**: gate em `from_tags` usa qual
     versão? Cláusula gate substancial em `.B` decide.

4. Confirmar acesso a state durante `from_tags`:
   - `from_tags` constrói `TagIntrospector` iterando
     tags. Em que ponto state está disponível?
   - Se state é construído antes das tags Equation
     serem processadas (ordem de chamadas em walk):
     `state.is_numbering_active(key)` retorna o valor
     correcto na altura.
   - Se state é construído **junto** com counter
     (ambos em `from_tags`): pode haver problema de
     ordem.

5. Confirmar `kind_index` push para outros arms locatable:
   - Verificar arm Figure em `from_tags.rs` (per P184B/P186A).
   - Confirmar padrão `intr.kind_index.entry(ElementKind::*).or_default().push(*loc)`.
   - P186E **deve replicar** este push para
     `ElementKind::Equation` (consumers que iteram
     `kind_index` precisam de ver Equations).
   - `mod tests` em `from_tags.rs` — padrão.
   - Tests de outros arms locatable como referência.

6. Confirmar que P186A §11.5 está correcto:
   - Gate `block && state-active` é literal em sintaxe
     ou tem variantes (`is_block_equation_numbered`
     método helper)?

Output: tabela com item + estado + decisão pendente
sobre versão do gate.

**Critério de saída**:
- `from_tags` localizado.
- `apply_at` confirmado.
- Decisão sobre versão do gate (ver `.B`).

### .B Decisão sobre versão do gate

Gate usa `state.is_numbering_active(key)` (snapshot final)
ou `state.is_numbering_active_at(key, location)`
(location-aware)?

**Opção A** — Gate usa snapshot final:
```
if *block && intr.state.is_numbering_active("numbering_active:equation") {
    intr.counters.apply_at("equation", counter_update.clone(), *loc);
}
```

Vantagem: simples; replica forma legacy
(`introspect.rs:377-382` usa `state.is_numbering_active`).

Desvantagem: se `SetEquationNumbering` for materializado
no futuro com toggle on/off por location, snapshot final
não captura state intermédio.

**Opção B** — Gate usa location-aware:
```
if *block && intr.state.is_numbering_active_at("numbering_active:equation", *loc) {
    intr.counters.apply_at("equation", counter_update.clone(), *loc);
}
```

Vantagem: futureproof — quando `SetEquationNumbering` for
adicionado, gate funciona correctamente para toggle.

Desvantagem: ligeiramente mais complexo; requer state
construído antes do counter (ordem em `from_tags`
verificar).

Critério de escolha:
- Em produção actual gate é dormente (state nunca
  populado). Opção A vs B é equivalente em comportamento
  hoje.
- Para testes E2E em P186F que injectam state: ambos
  funcionam para teste estático. Opção B funciona melhor
  para teste com toggle.
- Coerência com P185 (location-aware é a direcção
  arquitectural): Opção B alinhada.

Sugestão: **Opção B** (location-aware). Replica decisão
P185 sobre direcção arquitectural; futureproof; sem
custo em produção (gate dormente é dormente em ambos).

Output: decisão fixada.

### .C Actualizar L0 `from_tags.md`

1. Adicionar entrada para arm novo:
   - Variant: `ElementPayload::Equation { block,
     counter_update }`.
   - Gate: `block && state.is_numbering_active_at(
     "numbering_active:equation", location)` (Opção B).
   - Acção quando gate dispara: `counters.apply_at(
     "equation", counter_update, location)`.
   - Justificação: replica padrão P184B Figure (com
     `apply_at`); gate adicional simétrico com walk
     legacy `introspect.rs:377-382`.
   - **Nota explícita**: gate dormente em produção
     actual (sem `Content::SetEquationNumbering`).

2. Hash em branco aguarda recálculo.

**Critério de saída**:
- L0 contém entrada nova com gate dormente documentado
  honestamente.

### .D Estender arm `from_tags::Equation` com counter logic

1. Em `01_core/src/rules/introspect/from_tags.rs:222-226`:
   - Localizar arm actual (P186B introduziu stub no-op;
     P186D estendeu com populate de `kind_index`).
   - **Manter** populate de `kind_index` que P186D adicionou:
     ```
     intr.kind_index.entry(ElementKind::Equation)
         .or_default().push(*loc);
     ```
   - **Estender** com counter logic:
     - Destructure `block` e `counter_update` do payload
       (pode requerer mudar pattern de `{ .. }` para
       `{ block, counter_update }`).
     - Adicionar gate `if *block && intr.state.is_numbering_active_at(
       "numbering_active:equation", *loc)`.
     - Quando gate dispara: `intr.counters.apply_at(
       "equation".to_string(), counter_update.clone(),
       *loc)`.
   - Forma exacta fica para Claude Code.

2. Confirmar `@prompt-hash` actualiza após edit do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- Linter passa.
- Arm tem populate `kind_index` + gate counter
  funcional.
- Sem regressão face a P186D (populate `kind_index`
  preservado).

### .E Tests unit do arm

3-4 tests obrigatórios.

1. **Gate dispara: state activo + block**:
   - Setup: `state.init("numbering_active:equation",
     Bool(true), loc(0))`.
   - Tag: `ElementPayload::Equation { block: true,
     counter_update: Step }` em `loc(10)`.
   - Após `from_tags`: `counters.value_at("equation",
     loc(10))` = `Some([1])` ou similar.

2. **Gate dorme: state activo + inline**:
   - Setup: `state.init("numbering_active:equation",
     Bool(true), loc(0))`.
   - Tag: `ElementPayload::Equation { block: false,
     counter_update: Step }`.
   - Após: counter **não** populado para essa Tag.

3. **Gate dorme: state inactivo + block**:
   - Sem state init.
   - Tag: `ElementPayload::Equation { block: true,
     counter_update: Step }`.
   - Após: counter **não** populado. **Caso central da
     produção** — confirma gate dormente.

4. **Múltiplas equations sequencializam**:
   - Setup: state activo.
   - 3 tags sequenciais (block: true).
   - Após: `counters.value_at("equation", loc_n)`
     retorna `[1]`, `[2]`, `[3]` para cada location.
   - Confirma sequencialização do counter.

Tests co-localizados em `mod tests` de `from_tags.rs`.
Padrão dos tests P184B replicado.

**Critério de saída**:
- 3-4 tests passam.
- Tests existentes não regridem.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P186D
   baseline: +3 a +4.
3. `crystalline-lint .` zero violations.
4. Arm `ElementPayload::Equation` adicionado em
   `from_tags`.
5. Gate `block && state-active` (location-aware)
   funcional.
6. `counters.apply_at("equation", ...)` chamado quando
   gate dispara.
7. Em produção (state dormente): counter permanece
   vazio.
8. `flat_counter_at("equation", loc)` retorna valor
   quando gate disparou; `None` em produção.
9. Walk arm legacy intocado.
10. Snapshot tests ADR-0033 verdes.
11. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-186e-relatorio.md`
com:

- Resumo: arm `from_tags::Equation` materializado com
  gate location-aware; gate dormente em produção
  documentado honestamente; counter populado apenas em
  testes que injectam state.
- Confirmação `.F` (11 verificações).
- Δ tests vs baseline P186D (esperado +3 a +4).
- Hashes finais L0 (`from_tags.md`).
- Decisão arquitectural notável: gate location-aware
  (Opção B) escolhido por futureproofing.
- Estado actual:
  - P186 série: A ✅ B ✅ C ✅ D ✅ E ✅ | F pendente.
  - Eixo 2 do bloqueio P183C **resolvido
    estruturalmente** — counter populável; gate dormente
    em produção até `SetEquationNumbering` materializar.
  - 52 passos executados.
- Pendências cumulativas: inalteradas + nova entrada:
  - **`Content::SetEquationNumbering` ausente** —
    documentado em P186A §11.2 e P186E `.B/.C`. Não é
    DEBT P186 mas é trabalho identificado para passo
    futuro fora da série.
- Próximo passo: P186F (tests E2E + relatório
  consolidado série P186).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria com decisão sobre versão do
   gate.
2. Decisão `.B` fixada (Opção B — location-aware).
3. L0 `from_tags.md` actualizado.
4. Arm adicionado.
5. 3-4 tests passam (incluindo caso central de gate
   dormente).
6. Tests existentes não regridem.
7. Verificações `.F` passam (11/11).
8. Relatório `.G` escrito.
9. Output observable em produção inalterado.

---

## O que pode sair errado

- **`apply_at` exige assinatura diferente**: cláusula
  gate trivial — adaptar.
- **`is_numbering_active_at` não acessível em
  `from_tags`** (state ainda não construído quando
  Equation tag é processada): cláusula gate substancial.
  Se confirmar empiricamente em `.A.4`: recuar para
  Opção A (snapshot final, que assume state-completo).
  Possível problema de ordem em `from_tags`.
- **Tests E2E falham por gate dormente em produção**:
  esperado e documentado. Caso 3 do `.E` é precisamente
  este. Não é falha — é confirmação.
- **`counter_update` não é `Clone`**: cláusula gate
  trivial — usar referência ou adicionar derive.
- **Snapshot tests divergem inesperadamente**: walk emite
  Tag para Equation; `from_tags` agora popula counter
  (em testes); algum consumer pode iterar counter e ver
  entry nova. Investigar empiricamente. Per P186A Q3,
  esperado raro.
- **Linter divergência V13/V14**: cláusula gate trivial.

---

## Notas operacionais

- **Tamanho**: S puro. ~10 LOC arm + ~50 LOC tests +
  edit L0.
- **Sem dependências externas novas**.
- **Pré-condição P186F**: este passo concluído.
- **Padrão replicado**: P184B Figure arm (com gate
  adicional para Equation).
- **Cláusula gate trivial**: aplicável a assinatura de
  `apply_at`, `Clone` em `counter_update`, ordem.
- **Cláusula gate substancial**: aplicável apenas se
  state não estiver disponível em `from_tags` na altura
  certa (problema de ordem). Recurso: Opção A (snapshot
  final) em vez de Opção B.
- **Gate dormente em produção honestamente registado**:
  não é defeito; é design intencional dado `SetEquationNumbering`
  ausente. P186E não fixa essa ausência.
- **Eixo 2 do bloqueio P183C resolvido estruturalmente**:
  C2 pode migrar em P188 com `flat_counter_at("equation",
  loc)`. Em produção retorna `None` (gate dormente);
  fallback legacy continua activo. Estado paralelo a
  inversão observable que P184D fez para C3 — mas com
  fallback como caminho funcional permanente até
  `SetEquationNumbering` ser materializado.
