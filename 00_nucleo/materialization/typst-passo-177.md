# Passo P177 — `counter.at(label)` (M9 sub-passo 7)

Sétima feature M9. Capitaliza P165 (LabelRegistry) +
P170 (CounterRegistry hierarquia) + P176 (forma stdlib β).

Stdlib que durante eval retorna o valor do counter na
Location associada a um label — útil para referenciar
valor de counter em ponto específico do documento.

**Pré-condição**: P176 concluído. `Introspector::query_by_label`
e `formatted_counter` disponíveis. Padrão fixpoint
estabelecido por P175 + P176.

**Restrições**:
- Walk em `rules/introspect.rs::walk` **NÃO modificado**.
- API pública existente preservada.
- Sem variant Content novo — feature stdlib pura.
- Output observable não muda; snapshot tests passam
  inalterados.
- Stdlib forma minimal — sem rich `Value::Counter`.

---

## Sub-passos

### .A Inventário e decisões locais

1. **Confirmar `CounterRegistry::value_at(key, location)`**:
   - L0 `00_nucleo/prompts/entities/counter_registry.md`
     P165 declarou `value_at(key, location) -> Option<&[usize]>`.
     P170 manteve ou modificou?
   - `grep -rn "fn value_at" 01_core/src/entities/counter_registry.rs`.
   - Se ausente: **criar** em `.B`. Algoritmo similar a
     `StateRegistry::value_at` (P171):
     - Iterar updates da key ordenadas por Location.
     - Retornar valor da última update com `loc <= target_location`.
     - Se nenhuma update precede target_location: `None`.
   - Se existe mas não é hierárquico (P170 trabalho):
     verificar comportamento.

2. **Confirmar `LabelRegistry::lookup(label) -> Option<Location>`**:
   - P165 estabeleceu. Confirmar interface actual.

3. **Verificar método trait `Introspector::query_by_label`**:
   - P165 estabeleceu. Confirma retorno
     `Option<Location>`.

4. **Decisão sobre stdlib forma**:

   - **`counter_at(key_str, label_str)`** — 2 args strings.
   - Decisão sobre ordem dos argumentos:
     - **Opção A**: `(key, label)` — paridade com
       `counter_final(key)` que tem key como 1º arg.
     - **Opção B**: `(label, key)` — possível leitura
       natural ("at this label, counter X").
     - Sugestão: A — consistência com `counter_final`.
   - Cláusula gate trivial.

5. **Decisão sobre formato de retorno**:

   Manter Opção β estabelecida em P176:
   `counter_at(...) -> Value::Str("1.2.3")`.

   Comportamentos para casos de borda:
   - Label inexistente → `Value::Str("")` (defensive,
     consistente com P176 vazio).
   - Counter não tem update prévia à location do label →
     `Value::Str("")`.
   - Counter populado → string formatada hierárquica.

6. **Forma de formatação hierárquica**:

   `formatted_counter(key)` (P170) retorna string final.
   Para `counter.at`: precisa formatar valor **na**
   location específica.

   Opções:
   - **F1**: novo método trait
     `formatted_counter_at(key, location) -> Option<String>`.
     Implementação: `value_at(key, location)`, formata via
     mesma lógica de `format` em CounterRegistry.
   - **F2**: reutilizar `CounterRegistry::format` em
     versão genérica que aceita slice e formata.
   - F1 é mais limpo. Sugestão.

7. **Tests com fixpoint**:
   - Padrão P175/P176 — closure programática.
   - Documento com:
     - Heading 1 (label "intro").
     - Heading 1 (sem label).
     - Heading 2 (label "subsec").
   - Esperado:
     - `counter_at("heading", "intro")` → "1" (counter
       depois do 1º heading).
     - `counter_at("heading", "subsec")` → "2.1".
     - `counter_at("heading", "nonexistent")` → "".

   **Detalhe importante**: `value_at(key, location)`
   retorna o valor **antes ou depois** da update na
   location? Convenção:
   - Updates ordenadas por Location.
   - `value_at(key, loc)` retorna o valor do counter
     **após** todas as updates com Location ≤ loc.
   - Para heading com label "intro", a Location do label é
     a mesma do tag start; após esta update, counter = 1.
   - Confirmar com tests E2E que esta convenção dá
     resultado esperado.

Output: notas internas + decisões registadas:
- `value_at` existe ou criado.
- Ordem de args (sugestão A: key, label).
- Método novo `formatted_counter_at` ou inline.
- Convenção sobre value_at antes/depois da update.

**Critério de saída e gate de decisão**:
- Se `value_at` ausente em `CounterRegistry` e algoritmo
  é straightforward (similar a `StateRegistry::value_at`):
  cláusula gate trivial — criar.
- Se `value_at` existe mas com semântica diferente do
  esperado: gate trivial — adaptar.
- Senão prosseguir.

### .B Estender `CounterRegistry` se necessário

Apenas se `.A` confirmou ausência de `value_at`.

1. L0 `00_nucleo/prompts/entities/counter_registry.md`:
   - Adicionar método `value_at(key: &str, location: Location) -> Option<&[usize]>`.
   - Documentar convenção: retorna valor **após** todas
     as updates com Location ≤ target.

2. L1 `01_core/src/entities/counter_registry.rs`:
   - Implementar.
   - Tests co-localizados:
     - Vazio → `None`.
     - 1 update na location L1, value_at(L1) → valor; value_at(L0) → `None`.
     - Múltiplas updates ordenadas → valor correcto em
       cada location.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .C Estender `Introspector` trait

1. L0 `00_nucleo/prompts/entities/introspector.md`:
   - Adicionar método
     `formatted_counter_at(&self, key: &str, location: Location) -> Option<String>`.
   - Documentar: formata valor do counter na location
     especificada via mesma convenção que
     `formatted_counter` (P170).

2. L1 `01_core/src/entities/introspector.rs`:
   - Adicionar método ao trait + impl em
     `TagIntrospector`:
     ```rust
     fn formatted_counter_at(&self, key: &str, location: Location) -> Option<String> {
         self.counters.value_at(key, location).map(format_hierarchical)
     }
     ```
   - Tests co-localizados.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .D Stdlib `counter_at(key_str, label_str)`

1. Em registry stdlib (mesmo local que P175/P176):
   - Adicionar função:
     ```rust
     fn native_counter_at(args) -> SourceResult<Value> {
         let key: Str = args.expect("key")?;
         let label: Str = args.expect("label")?;
         let label_typed = Label::from(label);
         let result = ctx.introspector
             .query_by_label(&label_typed)
             .and_then(|loc| ctx.introspector.formatted_counter_at(&key, loc))
             .unwrap_or_default();
         Ok(Value::Str(result.into()))
     }
     ```
   - Adaptar API real conforme P175/P176 padrão.

2. Tests:
   - `counter_at` em introspector vazio → `Str("")`.
   - `counter_at` com label populado → string correcta.
   - `counter_at` com label inexistente → `Str("")`.
   - Argumentos não-string → `Err`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa.

### .E Tests E2E com fixpoint

1. Em `rules/introspect/fixpoint.rs::tests`:

   - **`p177_counter_at_em_doc_estavel`**:
     Closure constrói Content com 3 headings:
     - heading lvl 1 com label "intro".
     - heading lvl 1 sem label.
     - heading lvl 2 com label "subsec".
     Verifica via `introspect_to_fixpoint`:
     - `formatted_counter_at("heading", loc_intro)` → "1".
     - `formatted_counter_at("heading", loc_subsec)` → "2.1".

   - **`p177_counter_at_label_inexistente`**:
     Doc com 1 heading. Lookup label que não existe →
     `None`.

   - **`p177_counter_at_via_stdlib`** (se viável):
     Simular chamada stdlib `counter_at("heading", "intro")`
     em pipeline de eval programática.

**Critério de saída**:
- 2-3 tests E2E passam.
- Linter passa.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace` — todos os tests passam.
   Δ vs P176 baseline (1675). Estimativa: +7 a +12.
3. `crystalline-lint`: zero violations.
4. `CounterRegistry::value_at` existe (criado se ausente).
5. `Introspector::formatted_counter_at` no trait.
6. `TagIntrospector` impl.
7. Stdlib `counter_at(key_str, label_str)` registada.
8. Walk **NÃO modificado**.
9. `introspect()` legacy preservada.
10. Snapshot tests ADR-0033 verdes.
11. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-177-relatorio.md` com:

- Resumo: `counter.at(label)` materializado via padrão
  P175/P176 + LabelRegistry P165 + value_at em
  CounterRegistry.
- Confirmação de cada verificação `.F`.
- Hashes finais de L0s modificados.
- Decisões registadas em `.A`:
  - `value_at` criado ou existente.
  - Ordem de args.
  - Método `formatted_counter_at` no trait vs inline.
- Δ tests vs baseline P176.
- **Estado de M9**: 7/11 features.
- **Estado de M7**: mecanismo + 3 clientes (P175 query,
  P176 counter.final, P177 counter.at).
- Pendências cumulativas + actualização.
- Estado pós-passo: P177 concluído. P178 desbloqueado.

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu inventário sem disparar gate substancial.
2. `CounterRegistry::value_at` existe (criado se
   necessário).
3. `Introspector::formatted_counter_at` no trait + impl.
4. Stdlib `counter_at(key_str, label_str)` registada.
5. Tests E2E confirmam comportamento.
6. Verificações `.F` 1-11 passam.
7. Relatório `.G` escrito.
8. Output observable não muda.
9. M9 7/11 features.
10. Padrão "feature consulta introspector via fixpoint"
    aplicado pela 3ª vez (consistência).

---

## O que pode sair errado

- **`value_at` em CounterRegistry tem semântica diferente
  do esperado**: e.g. retorna valor *antes* da update em
  vez de *depois*. Cláusula gate trivial — adaptar
  formatação ou ajustar tests.
- **`Locations` em CounterRegistry não estão ordenadas
  por inserção**: P170 mudou estrutura interna? Verificar.
  Se necessário ordenar internamente.
- **`Label::from(Str)` não existe**: criar conversão
  trivial ou usar construtor existente.
- **Tests E2E precisam de Locations específicas**:
  closure programática constrói tags directamente —
  controlar Locations é factível mas precisa atenção.
- **Stdlib `counter_at` falha em type-check de args**:
  ajustar conforme padrão P175/P176.
- **`value_at` complica fixpoint convergência**: improvável
  — `value_at` é determinístico se updates ordenadas o são.
- **Linter divergência**: ajustar conforme erro.

---

## Notas operacionais

- **Tamanho**: S. Replica padrão P175/P176; trabalho
  concentra-se em método trait + stdlib + verificação de
  `value_at`. Sem variant Content novo, sem cascade.
- **Pré-condição P178**: feature seguinte M9. Candidatas
  pós-P177:
  - `here()` (precisa pré-requisitos).
  - `ElementKind::Outline` cascade (fecha lacuna #7).
  - `locate(callback)` (depende de Position).
  - Bib state (lacuna #6, magnitude desconhecida).
- **Cláusula gate trivial**: aplicável a decisões locais
  (ordem args, criar `value_at` ou usar existente).
- **Padrão fixpoint replicado pela 3ª vez**:
  P175 (query) → P176 (counter.final) → P177 (counter.at).
  Sinal que infraestrutura está madura.
- **`formatted_counter_at` é generalização de
  `formatted_counter`**: P170 retorna valor final;
  P177 retorna valor numa location. Convergência
  conceitual.
- **Magnitude S confirmada**: P176 entregou em S-M sem
  L0 modificado. P177 pode também não modificar L0 se
  `value_at` já existe.
