# Passo P182C — `extract_payload` arm `Content::SetHeadingNumbering`

Segundo passo de materialização P182 (após P182A diagnóstico,
P182B trait method).
Magnitude **S**.

Adiciona arm em `extract_payload` para `Content::SetHeadingNumbering`
produzir `ElementPayload::StateUpdate { key: "numbering_active:heading",
update: StateUpdate::Set(Value::Bool(active)) }`. Walk passa a emitir
tag para este variant; `from_tags` arm `StateUpdate` (já existente)
popula `StateRegistry`. Walk arm canonical `introspect.rs:455–457`
**não modificado** — continua a popular `state.numbering_active`
legacy paralelamente (M6 elimina).

Após P182C:
- Tags de `Content::SetHeadingNumbering` produzem
  `ElementPayload::StateUpdate` durante walk.
- `TagIntrospector` ao construir-se via `from_tags` popula
  `StateRegistry` com chave `numbering_active:heading`.
- `Introspector::is_numbering_active("numbering_active:heading")`
  passa a retornar valor real (não mais sempre false).
- Layouter continua a ler de `self.counter.is_numbering_active(…)`
  legacy (até P182D).

**Pré-condição**: P182B concluído. Tests workspace 1.743 verdes;
zero violations. Trait method `is_numbering_active` em
`Introspector`; impl em `TagIntrospector` delega a
`StateRegistry`.

**Restrições**:
- **Não** modificar walk arm `introspect.rs:455–457` — continua
  write canonical legacy.
- **Não** modificar `from_tags` arm `StateUpdate` — já cobre.
- **Não** modificar Layouter consumers (P182D).
- **Não** modificar `StateRegistry`.
- **Não** modificar trait `Introspector` (P182B fechou).
- API pública preservada.
- Output observable não muda em produção — Layouter ainda lê
  legacy, e `numbering_active:heading` em `StateRegistry`
  passa a estar populado mas sem consumer activo.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `extract_payload` actual:
   - `01_core/src/rules/introspect/extract_payload.rs` (ou
     localização equivalente; verificar).
   - Localizar match sobre `&Content::*` ou `content`.
   - Identificar arms existentes (esperado: `Content::Heading`,
     `Content::Figure`, `Content::Bibliography`, `Content::Outline`,
     `Content::Metadata`, `Content::State`, `Content::StateUpdate`,
     etc.).
   - Identificar onde inserir arm para `Content::SetHeadingNumbering`
     (sugestão: junto aos arms relacionados a state).

2. Confirmar variant `Content::SetHeadingNumbering`:
   - `01_core/src/entities/content.rs:176` (per P182A).
   - Forma exacta: `SetHeadingNumbering { active: bool }`.

3. Confirmar `ElementPayload::StateUpdate`:
   - `01_core/src/entities/element_payload.rs` ou similar.
   - Forma esperada: `StateUpdate { key: String, update:
     StateUpdate }` ou `StateUpdate { key: String, update:
     StateUpdateOp }` (verificar nome exacto do enum interno).
   - Variant `Set(Value)` esperado em `StateUpdate`/
     `StateUpdateOp` (per P171/P173).

4. Confirmar `is_locatable` actual para `Content::SetHeadingNumbering`:
   - `01_core/src/rules/introspect/locatable.rs` (ou
     localização equivalente).
   - **Decisão crítica**: se `is_locatable` retorna `false`
     para este variant, walk não chama `extract_payload`
     mesmo após este passo — arm fica silencioso. Tem que
     mudar para `true`.
   - Se já `true`: nada a fazer aqui.
   - Se `false`: cláusula gate trivial — adicionar/modificar
     arm para retornar `true`.

5. Confirmar `from_tags` arm `StateUpdate`:
   - `01_core/src/rules/introspect/from_tags.rs:154–166`
     (per P182B §5).
   - Comportamento: `init` na primeira ocorrência, `update`
     nas seguintes. Cobre o caminho deste passo sem
     modificação.

6. Confirmar L0s relevantes:
   - `00_nucleo/prompts/rules/introspect/extract_payload.md`
     (se existir) — entrada para arm novo.
   - `00_nucleo/prompts/rules/introspect/locatable.md` (se
     existir e mudança em .A.4) — entrada para variant
     locatable.

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída e gate de decisão**:
- Se `is_locatable(Content::SetHeadingNumbering)` é `false`:
  cláusula gate trivial — sub-passo `.B` adiciona arm
  locatable + L0.
- Se `ElementPayload::StateUpdate` tem forma diferente
  (campo `update: StateUpdate` vs `update: Value` directo,
  por exemplo): cláusula gate trivial — adaptar.
- Se variant `StateUpdate::Set` não existe (improvável per
  P171): cláusula gate substancial — investigar P171 antes
  de prosseguir.
- Senão prosseguir.

### .B Actualizar L0 `locatable.md` (se necessário per .A.4)

Apenas se .A.4 confirmar que `is_locatable(Content::SetHeadingNumbering)`
é `false`.

1. Adicionar `Content::SetHeadingNumbering` à lista de
   variants locatable em
   `00_nucleo/prompts/rules/introspect/locatable.md`.

2. Justificação documentada: variant produz `StateUpdate`
   que tem efeito sobre `Introspector` — precisa de tag.

**Critério de saída**:
- L0 reflecte mudança.
- Hash em branco aguarda recálculo.

### .C Modificar `is_locatable` (se necessário per .A.4)

Apenas se .A.4 confirmar mudança necessária.

1. Em `01_core/src/rules/introspect/locatable.rs`:
   - Modificar arm `Content::SetHeadingNumbering` de `false`
     para `true`.
   - Ou adicionar arm explícito se actualmente está em
     fallback `_ => false`.

2. Test unitário cobre `is_locatable(&Content::SetHeadingNumbering
   { active: true })` retorna `true`.

**Critério de saída**:
- `cargo check --workspace` passa.
- Test novo passa.
- Linter passa.

### .D Actualizar L0 `extract_payload.md`

1. Adicionar entrada para arm novo:
   - Variant: `Content::SetHeadingNumbering { active }`.
   - Output: `ElementPayload::StateUpdate { key:
     "numbering_active:heading", update: StateUpdate::Set(
     Value::Bool(active)) }`.
   - Justificação: replica padrão P171/P173 onde State
     userspace produz mesmo payload kind. Aqui o state é
     interno (chave com prefixo `numbering_active:`).

2. Documentar convenção de chave `numbering_active:<feature>`
   estabelecida em P182B.

3. Hash em branco aguarda recálculo.

**Critério de saída**:
- L0 contém entrada nova.
- Coerente com convenção dos arms existentes.

### .E Adicionar arm a `extract_payload`

1. Em `01_core/src/rules/introspect/extract_payload.rs`:
   - Adicionar arm `Content::SetHeadingNumbering { active }`
     que retorna `Some(ElementPayload::StateUpdate { ... })`
     conforme L0 .D.
   - Posicionar junto aos arms relacionados a state
     (vizinhança de `Content::State` / `Content::StateUpdate`).

2. Confirmar cabeçalho de linhagem `@prompt-hash` actualiza
   após edit do L0.

**Critério de saída**:
- `cargo check --workspace` passa.
- `cargo build --workspace` passa.
- Linter passa.

### .F Test unitário ou E2E mínimo

1. Test em `mod tests` de `extract_payload.rs` (unitário):
   - Input: `Content::SetHeadingNumbering { active: true }`.
   - Esperado: `Some(ElementPayload::StateUpdate { key:
     "numbering_active:heading", update: StateUpdate::Set(
     Value::Bool(true)) })`.
   - Caso simétrico: `active: false` produz `Bool(false)`.

2. Test E2E em `tests` ou similar (cobertura caminho
   completo):
   - Construir `Content` com `SetHeadingNumbering { active:
     true }` algures.
   - Correr walk + `from_tags` para construir
     `TagIntrospector`.
   - Verificar `intr.is_numbering_active("numbering_active:heading")`
     retorna `true`.
   - Caso simétrico: `active: false` retorna `false`.

3. Se .C tocou `is_locatable`: test adicional cobrindo isso
   (já incluído em .C).

**Critério de saída**:
- 2-4 tests novos passam (1-2 unitários + 1-2 E2E).
- Tests existentes não regridem.

### .G Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P182B baseline
   (1.743): +2 a +4 dependendo de cobertura `.F`.
3. `crystalline-lint .` zero violations.
4. `extract_payload(&Content::SetHeadingNumbering { .. })`
   retorna `Some(...)`.
5. Walk produz tag com payload `StateUpdate` para este
   variant.
6. `from_tags` popula `StateRegistry` com chave
   `numbering_active:heading` quando tag aparece.
7. `is_numbering_active("numbering_active:heading")` reflecte
   estado real (não mais sempre `false` quando documento
   contém `SetHeadingNumbering`).
8. Walk arm `introspect.rs:455–457` **NÃO modificado**.
9. Layouter **NÃO modificado** (esperado em P182D).
10. Snapshot tests ADR-0033 verdes.
11. Linter passa final.

### .H Encerramento

Escrever
`00_nucleo/materialization/typst-passo-182c-relatorio.md`
com:

- Resumo: arm `extract_payload` materializado; tag passa a
  ser emitida; `StateRegistry` populado paralelamente a
  legacy.
- Confirmação `.G` (11 verificações).
- Δ tests vs baseline P182B (esperado +2 a +4).
- Hashes finais de L0s modificados (`extract_payload.md` +
  eventualmente `locatable.md`).
- Decisões de execução notáveis (se houver, em particular
  se .A.4 disparou cláusula gate trivial).
- Estado actual:
  - P182 série: A ✅ B ✅ C ✅ | D-F pendentes.
  - M9: 10/11 features (inalterado).
  - 33 passos executados.
- Pendências cumulativas: inalteradas (legacy continua;
  Layouter ainda em legacy até P182D).
- Próximo passo: P182D (Layouter heading-arm + equation-arm
  via Introspector com substitution-with-fallback).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. `is_locatable(Content::SetHeadingNumbering)` retorna `true`
   (modificado em `.C` se necessário).
3. L0 `extract_payload.md` actualizado.
4. L0 `locatable.md` actualizado (se aplicável).
5. Arm em `extract_payload` produz payload correcto.
6. Tests novos passam (unitário + E2E).
7. Tests existentes não regridem.
8. Verificações `.G` passam (11/11).
9. Relatório `.H` escrito.
10. Output observable em produção inalterado (Layouter
    continua legacy).

---

## O que pode sair errado

- **`is_locatable` retorna `false` para `SetHeadingNumbering`**:
  cláusula gate trivial documentada (`.B` + `.C`). Padrão
  estabelecido em P181D para Bibliography (modificou `is_locatable`
  como passo dedicado). Aqui pode ser inline em P182C.
- **Forma de `ElementPayload::StateUpdate` diferente**: P171
  estabeleceu mas verificar empiricamente. Se enum interno
  for `StateUpdateOp` em vez de `StateUpdate`, ajustar nome.
  Cláusula gate trivial.
- **`Value::Bool` requer construção diferente** (`Value::bool(active)`
  vs `Value::Bool(active)`): cláusula gate trivial.
- **Arm já existe** (improvável, mas verificar): se sim,
  P182C reduz-se a verificação de coerência. Não emitir
  duplicado.
- **Walk arm `introspect.rs:455–457` toca tag emission**:
  improvável — walk arm canonical é separado de tag
  emission. Se walk arm precisar de modificação para emitir
  tag, gate trivial — mas é regressão a evitar (P182C
  pretende manter walk arm intocado).
- **Test E2E exige helper de pipeline completo**: replicar
  padrão P181 tests E2E. Helper `build_introspector_from_content`
  ou similar.
- **`from_tags` arm `StateUpdate` não cobre o caso**: P182B
  §5 documentou que cobre. Se inventário .A.5 revelar
  divergência: cláusula gate substancial — recuar.
- **Hashes desactualizados**: recálculo manual obrigatório
  após edits dos L0s.

---

## Notas operacionais

- **Tamanho**: S. ~30-80 LOC (1 arm em `extract_payload`,
  eventualmente 1 arm em `is_locatable`, 2-4 tests, edits L0s).
- **Sem dependências externas novas**.
- **Pré-condição P182D**: este passo concluído. Layouter
  passa a ter dados reais em `Introspector` para consultar.
- **Padrão replicado**: P171/P173 (State userspace produz
  `StateUpdate` payload). Aqui é state interno com prefixo
  de chave.
- **Cláusula gate trivial**: aplicável a forma de payload,
  nome de variant interno, `is_locatable` actual, helpers
  de tests.
- **Sem cláusula gate substancial esperada** se P171/P173
  estão estáveis.
- **Convenção de chave** `numbering_active:<feature>`
  documentada em P182B; este passo aplica-a literalmente
  para `heading`. Equation usa mesma convenção (chave
  `numbering_active:equation`) mas P182 actual cobre
  apenas heading via variant `SetHeadingNumbering`. Equation
  numbering não tem set rule equivalente em cristalino —
  Layouter equation-arm em P182D consulta a chave dele
  separadamente.
