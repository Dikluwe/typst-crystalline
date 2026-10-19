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
- `03_infra/src/integration_tests.rs` — testes de regressão `p1018_counter_string_figure_nao_colide_com_selector_figure` e `p1018_counter_string_table_nao_colide_com_selector_table`.
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

`CounterRegistry` indexava tudo por `String`/`EcoString`. Medido no commit anterior (`c311ad868`, Passo 1017), antes do fix:

| Expressão | Resultado |
|-----------|-----------|
| `#context counter("heading").get()` (sem `numbering:`) | `(2,)` |
| `#context counter(heading).get()` | `(2,)` |
| `#context counter("figure").get()` (com `#set figure(numbering: "1.")`) | `(2,)` |
| `#context counter(figure).get()` | `(2,)` |
| `#context counter("table").get()` (com `#set table(numbering: "1.")` e caption) | `(2,)` |
| `#context counter(table).get()` | `(2,)` |

A string de utilizador `"figure"` e o selector `figure` partilhavam a mesma entrada no registry, logo devolviam o mesmo valor. O mesmo acontecia para `table`.

### Estado depois

Medido no commit `6fc51d954` (HEAD após o fix):

| Expressão | Resultado |
|-----------|-----------|
| `#context counter("figure").get()` | `(0,)` |
| `#context counter(figure).get()` | `(2,)` |
| `#context counter("table").get()` | `(0,)` |
| `#context counter(table).get()` | `(2,)` |

`CounterRegistry` indexa por `CounterKey`. String de utilizador e selector de elemento são chaves distintas. A paridade com o vanilla está restabelecida para os quatro elementos medidos (heading, figure, table, footnote).

### Decisão do gate

Aprovado pelo dono com a condição de confirmar que `figure`/`table` tinham o mesmo defeito antes de assumir que o fix os cobre. A medição directa acima confirma a hipótese: com o setup que activa o contador automático (`figure.numbering` / `table.numbering` + caption para table), tanto `counter("figure")` como `counter("table")` devolviam o valor do contador automático `(2,)` em vez de `(0,)`. Após o fix, os dois espaços de nome estão isolados.

---

## Validação

```
cargo test --workspace
```

Resultado:

- typst-core: 4971 passed
- typst-infra: 789 passed
- typst-shell: 41 passed
- benches: 2 passed
- wiring: 37 passed
- crystalline_lint: 2 passed

Total: **5842 tests passed**, 0 failed.

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
