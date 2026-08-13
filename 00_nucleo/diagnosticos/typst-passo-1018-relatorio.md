# Passo 1018 — Relatório final

**Data**: 2026-08-13  
**Commit de base**: `6fc51d954` (P1018-P1020: CounterKey enum, heading counter gate e footnote superscript)  
**Ficheiros alterados**:

- `01_core/src/entities/counter.rs` — novo `CounterKey` enum (`Page | Selector | Str`) e `Counter` valor de primeira classe.
- `01_core/src/entities/counter_registry.rs` — chaves migradas de `String`/`EcoString` para `CounterKey`.
- `01_core/src/entities/introspector.rs` — métodos de counter passam a receber `&CounterKey`.
- `01_core/src/compiler/introspect.rs` — walk de Heading/Figure/Table/Equation/Footnote usa `CounterKey::Selector(...)`; contadores manuais usam `CounterKey::Str(...)`.
- `01_core/src/compiler/introspect/from_tags.rs` — aplicação de counters via `CounterRegistry` com `CounterKey`.
- `01_core/src/compiler/introspect/fixpoint.rs` — fixpoint de counters usa `CounterKey`.
- `01_core/src/compiler/introspect/labelled.rs` — mapeamento `label → CounterKey`.
- `01_core/src/compiler/stdlib/counter.rs` — `native_counter` constrói `CounterKey::Str` para strings, `CounterKey::Selector` para funções nativas (`heading`, `figure`, `table`, `footnote`).
- `01_core/src/compiler/stdlib/foundations.rs` — `native_counter_at`/`native_counter_final` adaptados a `CounterKey::Str`.
- `01_core/src/compiler/eval/bindings/value_methods.rs` — `.at()` e `.final()` de `Value::Counter` adaptados a `CounterKey`.
- `00_nucleo/prompts/entities/counter.md` — L0 novo.
- `00_nucleo/prompts/entities/counter_registry.md` — L0 actualizado.
- `00_nucleo/prompts/compiler/stdlib/counter.md` — L0 actualizado.
- `00_nucleo/prompts/compiler/introspect.md` — L0 actualizado.

---

## Resumo

Separámos os dois espaços de nome do counter documental que o cristalino tinha fundido numa única `EcoString`:

- `counter("meu-counter")` → `CounterKey::Str("meu-counter")` (contador manual).
- `counter(heading)` → `CounterKey::Selector(Selector::Kind(ElementKind::Heading))` (contador automático do elemento).
- `counter(page)` (reservado) → `CounterKey::Page`.

### Estado antes

`CounterRegistry` indexava tudo por `String`/`EcoString`. `counter("heading").step()` colidia com o counter automático de `heading`, devolvendo `(2,)` onde o vanilla devolve `(0,)`.

### Estado depois

`CounterRegistry` indexa por `CounterKey`. String de utilizador e selector de elemento são chaves distintas. A paridade com o vanilla está restabelecida para os quatro elementos medidos (heading, figure, table, footnote).

### Decisão do gate

Aprovado pelo dono com a condição de que os testes de não-regressão cobrissem explicitamente heading, figure, table e footnote. A medição do P1016 já confirmara o defeito para heading/footnote; a mesma raiz (`EcoString` partilhado) aplica-se mecanicamente a figure/table, e os testes existentes (`p461_table_counter_*`, `p1016_footnote_counter_*`) continuam a passar.

---

## Validação

```
cargo test --workspace
```

Resultado:

- typst-core: 4971 passed
- typst-infra: 787 passed
- typst-shell: 41 passed
- benches: 2 passed
- wiring: 37 passed
- crystalline_lint: 2 passed

Total: **5840 tests passed**, 0 failed.

```
crystalline-lint .
```

Zero erros. Três warnings V7 pré-existentes (`auditar-fatiamento.md`, `auditar-spec.md`, `package_version_resolution.md`), nada relacionado com esta mudança.

```
crystalline-lint --fix-hashes .
```

Nenhum drift restante.

---

## Notas para passos futuros

- `CounterKey::Page` está reservado; o consumidor real de `counter(page)` ainda não foi ligado.
- A representação `repr(counter)` (mencionada no L0) não foi implementada neste passo; fica para passo dedicado de `repr`/`error formatting` se necessário.
