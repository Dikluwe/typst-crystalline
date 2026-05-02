# Passo P182B — `Introspector::is_numbering_active` trait method

Primeiro passo de materialização P182 (após P182A diagnóstico).
Magnitude **S**.

Adiciona método `is_numbering_active(&self, key: &str) -> bool`
ao trait `Introspector` e impl correspondente em
`TagIntrospector`. Delega a `state.final_value(key)` (P171
StateRegistry) com matching `Value::Bool(true)`. Default OFF
quando state ausente ou valor não-Bool.

Após P182B:
- Trait `Introspector` expõe consulta `is_numbering_active`.
- `TagIntrospector` impl funciona contra StateRegistry vazio
  (retorna sempre false antes de P182C extract_payload).
- Tests unitários cobrem casos esperados.
- Layouter continua a ler de `state.numbering_active` legacy
  (até P182D).

**Pré-condição**: P182A concluído. 6 cláusulas fixadas:
mecanismo M1, default OFF, 2 consumers Layouter, API A2,
Opção 3 fecho. Tests workspace verde (1.738 ou superior).
`crystalline-lint .` zero violations.

**Restrições**:
- **Não** modificar `extract_payload` (P182C).
- **Não** modificar Layouter consumers (P182D).
- **Não** modificar walk arm `Content::SetHeadingNumbering`
  — continua a popular `state.numbering_active` legacy.
- **Não** modificar `StateRegistry`.
- API pública preservada (adição de método trait não-quebra
  callers actuais que não chamem o método).
- Output observable não muda — método novo retorna sempre
  false porque StateRegistry está vazio para chave
  `numbering_active:*`.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar trait `Introspector` actual:
   - `01_core/src/entities/introspector.rs`.
   - Localizar definição do trait.
   - Identificar métodos existentes (per P181F: `bib_entry_for_key`,
     `bib_number_for_key`; per outros passos: `state_value`,
     `final_value`, etc.).
   - Identificar localização sugerida para inserir
     `is_numbering_active` (P182A §13 sugere "após
     `bib_number_for_key`" — confirmar linha actual).

2. Confirmar L0 actual `entities/introspector.md`:
   - Localizar cabeçalho + lista de métodos do trait.
   - Verificar se já existe alguma referência a
     `numbering_active` (esperado: zero).

3. Confirmar API de `StateRegistry`:
   - `01_core/src/entities/state_registry.rs` (ou similar).
   - Método `final_value(&self, key: &str) -> Option<Value>`
     ou similar (P171 estabeleceu).
   - Confirmar assinatura exacta.

4. Confirmar `Value::Bool` existe:
   - `grep -n "Bool" 01_core/src/entities/value.rs` (ou
     similar).
   - Variant deve estar disponível.

5. Confirmar `TagIntrospector` impl bloco:
   - `01_core/src/entities/introspector.rs` ou ficheiro
     próprio de impl.
   - Localizar `impl Introspector for TagIntrospector { ... }`.
   - Identificar onde adicionar método novo.

6. Decidir nome do field StateRegistry no trait/impl:
   - P182A §13 usa `self.state.final_value(key)`.
   - Confirmar se field é `state` ou outro nome em
     `TagIntrospector`.

Output: tabela com item + estado confirmado / linha
actual / observação.

**Critério de saída e gate de decisão**:
- Se `final_value` em StateRegistry tem assinatura diferente
  da esperada: cláusula gate trivial — adaptar.
- Se `Value::Bool` ausente: gate substancial — recuar e
  decidir mecanismo alternativo (improvável; documentado em
  P182A §"O que pode sair errado").
- Senão prosseguir.

### .B Actualizar L0 `entities/introspector.md`

1. Adicionar entrada para método novo:
   - Nome: `is_numbering_active`.
   - Assinatura: `fn is_numbering_active(&self, key: &str) -> bool`.
   - Propósito: consulta se numeração está activa para chave
     dada (heading, equation, etc.).
   - Default: false quando state ausente ou valor não-Bool.
   - Implementação: delega a `state.final_value(key)` +
     match `Some(Value::Bool(true))`.
   - Posição na lista: após `bib_number_for_key` (per P182A
     §13).

2. Hash em branco aguarda recálculo manual após
   confirmação humana.

**Critério de saída**:
- L0 contém entrada nova.
- Texto coerente com convenção dos métodos existentes.
- Sem alteração de outros métodos do trait.

### .C Adicionar método ao trait

1. Em `01_core/src/entities/introspector.rs`:
   - Adicionar declaração ao trait `Introspector`:
     ```
     fn is_numbering_active(&self, key: &str) -> bool;
     ```
   - Posicionar após `bib_number_for_key` (per .A).

2. Documentação inline do método: 1-3 linhas explicando
   propósito + comportamento default.

**Critério de saída**:
- `cargo check --workspace` passa (impls obrigados a
  implementar; falha esperada se `TagIntrospector` ainda
  não tem o método — corrigido em `.D`).
- Linter passa.

### .D Implementar método em `TagIntrospector`

1. Em `impl Introspector for TagIntrospector`:
   - Adicionar método:
     - Delega a `self.state.final_value(key)`.
     - Match `Some(Value::Bool(true))` retorna true.
     - Qualquer outro caso (None, `Some(Value::Bool(false))`,
       `Some(Value::*)` não-Bool) retorna false.
   - Exacta forma do código fica para Claude Code conforme
     convenção do projeto. P182A §13 sugeriu uso de
     `matches!` mas é detalhe de implementação.

2. Confirmar cabeçalho de linhagem `@prompt-hash` está no
   estado correcto (recálculo manual se L0 mudou).

**Critério de saída**:
- `cargo check --workspace` passa.
- `cargo build --workspace` passa.
- Linter passa.

### .E Tests unitários

5 tests obrigatórios (per P182A §13):

1. **Vazio devolve false** — TagIntrospector novo (state
   vazio) + chamada `is_numbering_active("heading")` →
   false.

2. **Apply Set Bool(true) retorna true** — StateRegistry
   recebe set `Value::Bool(true)` para chave; chamada
   retorna true.

3. **Keys distintas isoladas** — set `numbering_active:heading`
   = true; chamada para `numbering_active:equation` retorna
   false (key isolation).

4. **Bool(false) retorna false** — set explícito
   `Value::Bool(false)`; chamada retorna false.

5. **Non-Bool value retorna false** — set `Value::Int(1)` ou
   similar; chamada retorna false (graceful degradation).

Tests co-localizados em `mod tests` dentro de
`introspector.rs` (ou ficheiro de impl). Helpers de
construção de StateRegistry replicam padrão dos tests
existentes para `state_value` / `final_value` (P171).

**Critério de saída**:
- 5 tests novos passam.
- Tests existentes não regridem (1.738 + 5 = 1.743
  esperado).

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P182A
   baseline (1.738): +5.
3. `crystalline-lint .` zero violations.
4. `is_numbering_active` accessível via `Introspector`
   trait.
5. `TagIntrospector` impl delega correctamente.
6. Documento sem `set state("numbering_active:*", _)`
   produz `is_numbering_active(*)` = false em todos os
   sítios.
7. Walk **NÃO modificado** (regressão evitada).
8. Layouter **NÃO modificado** (esperado em P182D).
9. Snapshot tests ADR-0033 verdes.
10. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181b-relatorio.md`
com:

- Resumo: trait method materializado; impl delega a
  StateRegistry.
- Confirmação `.F` (10 verificações).
- Δ tests vs baseline P182A (+5 esperado; reportar real).
- Hashes finais de L0s modificados (`introspector.md`).
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P182 série: A ✅ B ✅ | C-F pendentes.
  - M9: 10/11 features (inalterado — feature
    `numbering_active` só conta após P182F fechar lacuna #4).
  - 32 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P182C (`extract_payload` arm
  `Content::SetHeadingNumbering`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. L0 `entities/introspector.md` actualizado com entrada
   nova.
3. Método `is_numbering_active` declarado no trait.
4. Método implementado em `TagIntrospector`.
5. 5 tests novos passam.
6. Tests existentes não regridem.
7. Verificações `.F` passam (10/10).
8. Relatório `.G` escrito.
9. Output observable inalterado em produção (StateRegistry
   vazio para chave; método retorna sempre false até P182C+).

---

## O que pode sair errado

- **`final_value` tem assinatura diferente**: P171 pode ter
  `final_value(&self, key: &str) -> Option<&Value>` em vez
  de `Option<Value>`. Cláusula gate trivial — adaptar
  matching.
- **Field name diferente de `state`**: TagIntrospector pode
  ter o sub-store StateRegistry com outro nome (`state_registry`
  ou similar). Adaptar.
- **`Value::Bool` não existe ou é diferente**: improvável
  per P182A; mas se sim, adaptar match arm.
- **Trait method exige `&mut self`**: improvável; método é
  read-only. Confirmar.
- **Linter dispara V13/V14 por adição de método sem L0**:
  cláusula gate trivial — actualizar L0 antes do código (já
  previsto em `.B` antes de `.C`).
- **Tests de StateRegistry exigem helper de construção
  específico**: replicar padrão dos tests P171 existentes.
- **Cabeçalho `@prompt-hash` desactualizado por edit do L0**:
  recálculo manual obrigatório antes de fechar passo.

---

## Notas operacionais

- **Tamanho**: S puro. ~80-150 linhas (declaração + impl +
  5 tests + documentação inline).
- **Sem dependências externas novas**.
- **Pré-condição P182C**: este passo concluído. Sem
  alterações estruturais que afectem P182C.
- **Padrão replicado**: P181F (trait method
  `bib_entry_for_key` + `bib_number_for_key`).
- **Cláusula gate trivial**: aplicável a assinatura de
  `final_value`, field name, formato de tests.
- **Sem cláusula gate substancial esperada**.
