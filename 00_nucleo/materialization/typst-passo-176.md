# Passo P176 — `counter.final(key)` (M9 sub-passo 6)

Sexta feature M9. Capitaliza P170 (`CounterRegistry`
hierarquia) + P174 (fixpoint mechanism) + P175 (entry
point `introspect_to_fixpoint`).

Stdlib que durante eval retorna o valor final de um counter
— acessível via `ctx.introspector` da iteração anterior do
fixpoint.

**Pré-condição**: P175 concluído. `Selector` minimal,
`Introspector::query`, `introspect_to_fixpoint`,
`compute_tags_hash` disponíveis. `CounterRegistry`
hierárquico (P170).

**Restrições**:
- Walk em `rules/introspect.rs::walk` **NÃO modificado**.
- API pública existente preservada.
- Stdlib `counter_final` retorna forma minimal — não
  `Value::Counter` rich type.
- Output observable não muda; snapshot tests passam
  inalterados.
- Não migrar consumers M5.

---

## Sub-passos

### .A Inventário e decisões locais

1. **Confirmar estado actual de `CounterRegistry`**:
   - `01_core/src/entities/counter_registry.rs`.
   - Métodos existentes (P165 + P170):
     - `empty()`.
     - `apply(key, update)` (P165).
     - `apply_hierarchical(key, level)` (P170).
     - `value(key)` ou similar — confirmar.
     - `format(key) -> Option<String>` (P170, paridade
       com `format_hierarchical` legacy).
   - Verificar se existe método `final_value` ou se
     `value(key)` já retorna o valor actual (que após
     `from_tags` completar é o valor final).
   - Se ausente: adicionar `final_value(key) -> Option<&[usize]>`
     (ou nome similar). Mas se `value(key)` é equivalente:
     reutilizar.

2. **Verificar `Introspector` trait**:
   - Métodos actuais (P165 + P170 + P175):
     - `query_by_kind`, `query_by_label`, `query_first`,
       `query_unique`, `position_of` (P165).
     - `formatted_counter(key)` (P170).
     - `query(selector)` (P175).
     - `state_value(key, location)`,
       `state_final_value(key)` (P171).
   - Adicionar:
     - `counter_final_value(&self, key: &str) -> Option<Vec<usize>>`
       OU
     - `counter_final_formatted(&self, key: &str) -> Option<String>`
   - Decisão sobre forma — ver `.A.3`.

3. **Decisão sobre tipo de retorno da stdlib**:

   - **Opção α** — array de inteiros:
     ```
     counter_final("heading") → Value::Array([Value::Int(1), Value::Int(2), Value::Int(3)])
     ```
     - Cliente formata como precisar.
     - Mais informação preservada.
     - Mais cascade: precisa `Value::Array` (existe em
       cristalino?).

   - **Opção β** — string formatada:
     ```
     counter_final("heading") → Value::Str("1.2.3")
     ```
     - Reutiliza `formatted_counter` (P170).
     - Cliente recebe pronto para mostrar.
     - Menos informação (perde estrutura hierárquica).
     - Cascade trivial — `Value::Str` certamente existe.

   - **Opção γ** — count (último nível):
     ```
     counter_final("heading") → Value::Int(3)
     ```
     - Para heading hierarquia [1,2,3] retorna 3.
     - Útil só para casos simples.
     - Mais simples.

   Recomendação: **β** (string formatada). Reutiliza
   trabalho P170; menos cascade; útil directamente.
   Refino futuro pode adicionar outras formas.

4. **Verificar `Value::Array` em cristalino** (apenas se
   Opção α):
   - `grep -rn "Value::Array\|Array(Vec" 01_core/src/entities/value*.rs`.
   - Se ausente e Opção α escolhida: gate trivial — adoptar
     β.

5. **Confirmar padrão stdlib pós-P175**:
   - `query(kind_str)` em P175 aceita string.
   - `counter_final(key_str)` segue mesmo padrão — string
     como argumento.

6. **Tests com fixpoint**:
   - Padrão estabelecido em P175 (closure programática).
   - Para `counter.final`: documento com 3 headings.
     Iter 1: counter vazio → resultado vazio. Iter 2:
     counter populado → resultado correcto.

Output: notas internas + decisões registadas:
- Forma de retorno (α/β/γ).
- Método novo ou existente em `CounterRegistry`.

**Critério de saída e gate de decisão**:
- Se Opção β viável (`formatted_counter` retorna
  `Option<String>`): prosseguir.
- Se método `final_value` ou equivalente em
  `CounterRegistry` ausente: criar trivialmente.
- Se cascade adicional inesperada: cláusula gate trivial.

### .B Estender `Introspector` trait

1. L0 `00_nucleo/prompts/entities/introspector.md`:
   - Adicionar método (forma decidida em `.A`):
     ```rust
     fn counter_final_value(&self, key: &str) -> Option<...>;
     ```
   - Documentar comportamento:
     - Retorna valor final do counter (após todas as
       updates aplicadas).
     - `None` se key não tem counter (nunca foi
       actualizado).

2. L1 `01_core/src/entities/introspector.rs`:
   - Adicionar método ao trait + impl em
     `TagIntrospector` (delega para `CounterRegistry`).
   - Tests co-localizados:
     - Vazio → `None`.
     - Após 3 headings hierárquicos → resultado correcto
       conforme forma escolhida.
     - Key inexistente → `None`.

3. (Apenas se necessário) Estender `CounterRegistry`:
   - Se método `final_value` ou equivalente ausente:
     adicionar.
   - Tests co-localizados.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .C Stdlib `counter_final(key_str)`

1. Identificar registry de stdlib (P169/P175 padrão).

2. Adicionar função stdlib:
   - `counter_final(key_str: Str) -> Value::Str` (Opção β
     sugerida).
   - Implementação: consulta
     `ctx.introspector.counter_final_value(&key_str)`;
     mapeia para `Value`.

3. Comportamento:
   - Iter 1 (introspector vazio): retorna
     `Value::Str("")` (ou `Value::None` se mais
     idiomático em cristalino).
   - Iter 2 (populado): retorna string formatada.

4. Update L0 stdlib se aplicável.

5. Tests:
   - `counter_final("heading")` em introspector vazio.
   - `counter_final("heading")` em introspector populado.
   - `counter_final("inexistente")` retorna vazio/None.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .D Tests E2E com fixpoint

1. Em `rules/introspect/fixpoint.rs::tests` ou módulo
   dedicado:

   - **`p176_counter_final_em_doc_estavel`**:
     Closure de eval que retorna Content fixo com 3
     headings (níveis [1, 2, 1] → counter `[2]`).
     `introspect_to_fixpoint(closure, engine, ctx)`.
     Verifica:
     - Resultado `Ok(...)`.
     - `introspector.counter_final_value("heading")` →
       valor correcto.
     - Convergência em 2 iter.

   - **`p176_counter_final_evolui_entre_iters`**:
     Closure que constrói Content baseado em
     `ctx.introspector.counter_final_value("heading")`.
     - Iter 1: vazio → "".
     - Iter 2: "1.2.1" → confirma esta string presente.
     Convergência em iter 3.

   - **`p176_counter_final_inexistente`**:
     Doc sem headings. `counter_final_value("heading")`
     → `None` ou string vazia.

2. Opcional: test E2E via stdlib se viável construir
   pipeline de eval em test.

**Critério de saída**:
- 3 tests E2E passam.
- Linter passa.

### .E Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P175 baseline (1668). Estimativa: +6 a +10.
3. `crystalline-lint`: zero violations.
4. `Introspector::counter_final_value` no trait.
5. `TagIntrospector` impl.
6. Stdlib `counter_final(key_str)` registada.
7. Walk **NÃO modificado**.
8. `introspect()` legacy preservada.
9. `introspect_with_introspector` preservada.
10. Snapshot tests ADR-0033 verdes.
11. Linter passa final.

### .F Encerramento

Escrever
`00_nucleo/materialization/typst-passo-176-relatorio.md` com:

- Resumo: `counter.final(key)` materializado;
  capitalizando P170 + P174 + P175.
- Confirmação de cada verificação `.E`.
- Hashes finais de L0s modificados.
- Decisões registadas em `.A`:
  - Forma de retorno (α/β/γ).
  - Método novo/existente em `CounterRegistry`.
- Δ tests vs baseline P175.
- **Estado de M9**: 6/11 features.
- **Estado de M7**: ainda 2 sub-passos (mecanismo + 2
  clientes).
- Pendências cumulativas + actualização.
- Estado pós-passo: P176 concluído. P177 desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário sem disparar gate substancial.
2. `Introspector::counter_final_value` no trait + impl.
3. Stdlib `counter_final(key_str)` registada.
4. Tests E2E confirmam fixpoint funciona com `counter.final`.
5. Verificações `.E` 1-11 passam.
6. Relatório `.F` escrito.
7. Output observable não muda.
8. M9 6/11 features.
9. Padrão "feature consulta introspector via fixpoint"
   replicado de P175.

---

## O que pode sair errado

- **`CounterRegistry::final_value` ausente**: criar
  trivialmente com `value(key).cloned()` ou similar.
  Cláusula gate trivial.
- **`Value::Array` ausente** (apenas se Opção α): fallback
  para Opção β. Documentar.
- **Stdlib forma incompatível**: cláusula gate trivial,
  adoptar forma do cristalino.
- **Tests E2E com fixpoint complicados**: closures
  programáticas funcionam mas podem não simular bem o
  fluxo real. Aceitar limitação; tests mais ricos
  vêm quando pipeline é validado.
- **Counter "vazio" representação ambígua**: `None` vs
  string vazia vs array vazio. Decisão local em `.A`.
  Cláusula gate trivial.
- **Linter divergência**: ajustar conforme erro.

---

## Notas operacionais

- **Tamanho**: S-M. Menor que P175 (sem Selector novo,
  sem entry point novo). Trabalho concentra-se em método
  trait + stdlib + tests.
- **Pré-condição P177**: feature seguinte M9. Candidatas
  pós-P176: `counter.at(label)` (capitaliza P176 +
  LabelRegistry); `here()` com pré-requisitos; Outline
  cascade.
- **Cláusula gate trivial**: aplicável a forma de
  retorno, método existente vs novo em CounterRegistry.
- **Padrão replicado de P175**: feature stdlib que
  consulta `ctx.introspector` durante eval; usa
  fixpoint via `introspect_to_fixpoint` em testes.
- **Sem variant Content novo**: `counter.final` é stdlib
  func; não cria Content variant. Cascade arms zero.
  Por isso magnitude S-M.
- **Reutilização de `formatted_counter` (P170)** se
  Opção β: trabalho mínimo, máximo aproveitamento.
