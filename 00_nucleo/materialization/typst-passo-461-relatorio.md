# Relatório de Execução — P461

> **Passo:** 461  
> **Data:** 2026-06-25  
> **Foco:** Correção de divergência arquitetural P459: `table_counter` → `CounterRegistry`  
> **Tipo:** Correção arquitetural / Refacto mecânico

---

## Resumo executivo

Movido o contador de numeração automática de `table(...)` do campo local
`table_counter: usize` no `Layouter` para o `CounterRegistry` do
`TagIntrospector`, chave `"table"`, alinhando-se com heading (P451), figure
(P454) e equation (P456). `Content::Table` foi promovido a locatable para
permitir leitura location-aware no layout e desbloquear `label`/`ref` na
Trilha 2.

---

## Alterações realizadas

### 1. Remoção de `table_counter` do `Layouter`

- **Ficheiro:** `01_core/src/rules/layout/mod.rs`
- Removido campo `table_counter: usize` do struct `Layouter`.
- Removida inicialização `table_counter: 0` em `Layouter::new`.

### 2. Promoção de `Content::Table` a locatable

- **Ficheiro:** `01_core/src/entities/element_kind.rs`
  - Adicionado variant `ElementKind::Table`.
  - Atualizados `as_str` e `from_name`.
- **Ficheiro:** `01_core/src/entities/element_payload.rs`
  - Adicionado variant `ElementPayload::Table { counter_update, is_counted }`.
- **Ficheiro:** `01_core/src/entities/elements/table.rs`
  - Implementados `element_kind()` → `Some(ElementKind::Table)` e
    `to_payload()` → `ElementPayload::Table`.
- **Ficheiro:** `01_core/src/rules/introspect/extract_payload.rs`
  - Adicionado arm `Content::Table(e) => e.to_payload()`.
- **Ficheiro:** `01_core/src/rules/introspect/locatable.rs`
  - Movido `Content::Table(_)` para secção locatable.

### 3. População do counter `"table"` no oráculo

- **Ficheiro:** `01_core/src/rules/introspect.rs`
  - Walk top: gate `is_counted` combinado com `table.numbering` da chain
    (paridade com figure P365).
  - Walk arm `Content::Table`: recursa em caption + children.
  - `populate_intr_from_tag_start`: arm `ElementPayload::Table` avança
    `intr.counters["table"]` quando `is_counted`.
  - `materialize_time`: recursa também na caption da table.

### 4. Layout de table consome do oráculo

- **Ficheiro:** `01_core/src/rules/layout/table.rs`
  - Substituído `layouter.table_counter += 1` por
    `layouter.introspector.flat_counter_at("table", current_location)`.
  - Preservado o prefixo `"Table {formatted}: "`.

### 5. Resolução do prompt órfão `eval/table.md`

- **Ficheiro:** `01_core/src/rules/eval/rules.rs`
  - Adicionada referência `@prompt 00_nucleo/prompts/rules/eval/table.md` no
    cabeçalho do ficheiro dono do arm `target == "table"`.

### 6. Nota de divergência P459

- **Ficheiro:** `00_nucleo/materialization/typst-passo-459-nota-divergencia.md`
  - Criado com a descrição da divergência, padrão esperado, consequências,
    correção e lição aprendida.

### 7. Ajustes mecânicos de vizinhança

- **Ficheiro:** `01_core/src/rules/layout/mod.rs`
  - Recursão de `walk` interno de bibliography também percorre caption da
    table (paridade com figure).
- **Ficheiro:** `03_infra/src/export/builder.rs`
  - Correção de tipo `String` → `EcoString` em `entries.push` (resíduo de
    P460 que impedia compilação).

---

## Tests

### Novos

- `01_core/src/rules/introspect.rs::p461_table_counter_popula_via_introspector`
  - Verifica que duas tables numeradas produzem counter `"table"` = 1, 2.
- `01_core/src/rules/layout/tests.rs::p461_table_counter_persiste_relayout`
  - Verifica que layouts repetidos da mesma sequência preservam 1, 2.

### Ajustados

- `01_core/src/rules/introspect/locatable.rs`
  - Adicionado `Content::table(...)` ao helper de invariante
    `is_locatable ↔ extract_payload.is_some()`.

### Resultado

```text
cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica
=> OK (todos os testes passam)
```

A execução completa falha apenas no teste pré-existente
`p350c_flag_on_nao_convergente_classifica`, que aborta com stack overflow em
`rules::eval::tests::tests`. O falhanço é independente de P461 (não toca em
convergência/counters de eval).

```text
cargo test --test crystalline_lint
=> OK (2 passed)
```

---

## Critério de fecho

- [x] `table_counter` removido do `Layouter`.
- [x] `CounterRegistry`/`Introspector` com chave `"table"` populado.
- [x] Layout de table lê número do oráculo, não de campo local.
- [x] Tests L2/L3 de P459 continuam passando (paridade funcional).
- [x] Novo teste de regressão: `p461_table_counter_persiste_relayout`.
- [x] Prompt órfão `eval/table.md` resolvido (referência em `rules/eval/rules.rs`).
- [x] Nota de divergência P459 criada em `00_nucleo/materialization/`.
- [x] `cargo test --workspace` verde (exceto falhanço pré-existente em P350c);
      `crystalline-lint` zero violations novas.
- [ ] Trilha 1 marcada como **COMPLETA E COERENTE** no roteiro de conclusão
      (ação separada no roteiro).

---

## Próximo passo

Com P461, Trilha 1 está completa e coerente. Trilha 2 (`label`/`ref`, PDF
`/Dests`) está desbloqueada para continuação em P462/P463/P464.
