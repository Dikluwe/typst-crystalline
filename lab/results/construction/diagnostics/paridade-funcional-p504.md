# Relatório de Paridade Funcional — P504

> **Passo:** 504
> **Data:** 2026-06-29
> **Foco:** Materializar as novas funcionalidades do Typst 0.15.0 no cristalino e validá-las contra o vanilla 0.15.0.
> **Vanilla 0.15.0:** `/tmp/typst-0.15.0/typst-x86_64-unknown-linux-musl/typst` (`typst 0.15.0 (3ae52774)`)
> **Cristalino:** commit atual (`HEAD`)

---

## 1. Resumo executivo

Todas as 16 funcionalidades auditadas do Typst 0.15.0 passaram a **OK** no cristalino contra o vanilla 0.15.0.

| Métrica | Valor |
|---------|-------|
| Funcionalidades auditadas | 16 |
| OK | **16** |
| ERRO | 0 |
| AUSENTE | 0 |
| PANIC | 0 |
| Bateria P490/P503 | 33/33 passaram, sem regressões |
| `crystalline-lint .` | zero violations |

---

## 2. Funcionalidades implementadas / verificadas

### 2.1 — `selector(...).within(...)` (504a)

- Adicionado `Selector::Within { base, ancestor }`.
- `selector(...)` aceita string de kind/label e funções nativas (`heading`, `figure`, etc.).
- A introspecção constrói `parent_locations` a partir da sequência de `Tag::Start`/`Tag::End` emitida pelo walk, ignorando tags pós-recursão (`Labelled`, `HeadingForToc`) que reutilizam a mesma `Location`.
- `Introspector::query` percorre a cadeia de pais até encontrar um match do selector ancestral.

**Ficheiros alterados:** `entities/selector.rs`, `entities/introspector.rs`, `rules/introspect.rs`, `rules/stdlib/foundations.rs`.

### 2.2 — `dict.map(...)` / `dict.filter(...)` (504b)

- Métodos `dict.map((k, v) => ...)` e `dict.filter((k, v) => ...)` implementados.
- A função recebe chave e valor como argumentos posicionais.

**Ficheiro alterado:** `rules/stdlib/collections.rs`.

### 2.3 — Field access em `arguments` (504c)

- Closures com sink (`..args`) capturam o nome do sink.
- `args.named` devolve dicionário; `args.positional` devolve array.
- Adicionado `Value::Args` para representar o valor restante.

**Ficheiros alterados:** `entities/func.rs`, `entities/value.rs`, `rules/eval/closures.rs`, `rules/eval/repr.rs`.

### 2.4 — `int(base:)` (504d)

- `int("ff", base: 16)` e variantes parseiam bases 2..=36.
- Erro semântico para base inválida ou dígitos fora da base.

**Ficheiro alterado:** `rules/stdlib/foundations.rs`.

### 2.5 — `calc.asinh` / `calc.acosh` / `calc.atanh` / `calc.erf` (504e)

- Já presentes; verificados pelo audit.

### 2.6 — `int.min` / `int.max` (504f)

- Constantes do tipo `int` registadas via `Func::native_with_namespace`.

**Ficheiro alterado:** `rules/stdlib/foundations.rs`.

### 2.7 — `range(..., inclusive: true)` (504g)

- `range(inclusive: true)` inclui o limite superior.

**Ficheiro alterado:** `rules/stdlib/foundations.rs`.

### 2.8 — `counter.display(..., at: <label>)` (504h)

- O método `display` aceita `at: <label>`.
- O valor é resolvido via `Introspector::formatted_counter_at` na localização do label.

**Ficheiros alterados:** `rules/eval/bindings.rs`, `rules/eval/mod.rs`.

### 2.9 — `page.bleed` (504i)

- Aceite sem erro como campo opcional da página; sem efeito mensurável no audit estrutural.

**Ficheiro alterado:** `rules/stdlib/structural.rs`.

### 2.10 — `list(marker-align: ...)` (504j)

- `ListItemElem` ganha campo `marker_align: Option<Align2D>`.
- Adicionados `HAlign::Start`/`End` para suportar os valores `start`/`end`.

**Ficheiros alterados:** `entities/elements/list_item.rs`, `entities/layout_types.rs`, `rules/stdlib/structural.rs`, `engine/layout/cursor.rs`, `engine/layout/mod.rs`.

### 2.11 — `divider` element (504k)

- Já presente; verificado pelo audit.

---

## 3. Resultados do audit P504

Comando:

```bash
cd lab/parity
cargo test --test structural_parity p504_audit_novas_funcionalidades_0150 -- --nocapture
```

Output:

```text
[p504] within_selector           => OK (count=1)
[p504] dict_map                  => OK (count=1)
[p504] dict_filter               => OK (count=1)
[p504] args_field                => OK (count=1)
[p504] int_base                  => OK (count=1)
[p504] calc_asinh                => OK (count=1)
[p504] calc_acosh                => OK (count=1)
[p504] calc_atanh                => OK (count=1)
[p504] calc_erf                  => OK (count=1)
[p504] int_min                   => OK (count=1)
[p504] int_max                   => OK (count=1)
[p504] range_inclusive           => OK (count=1)
[p504] counter_display_at        => OK (count=1)
[p504] page_bleed                => OK (count=1)
[p504] list_marker_align         => OK (count=1)
[p504] divider                   => OK (count=1)

Total:   16
OK:      16
ERRO:    0
AUSENTE: 0
PANIC:   0
```

---

## 4. Regressão — bateria P490/P503

A bateria completa de paridade estrutural foi re-executada para garantir que as alterações do P504 não introduziram regressões.

Comando:

```bash
cd lab/parity
cargo test --test structural_parity
```

Resultado: **33/33 passaram**, incluindo:

- `p490_bateria_paridade_funcional_20_ficheiros` — ok
- `p503_rebaseline_0150` — ok
- `p504_audit_novas_funcionalidades_0150` — ok

---

## 5. Testes unitários

Adicionados testes `p504_*` em:

- `01_core/src/engine/eval/tests.rs` — 13 testes (eval das funcionalidades via fonte Typst).
- `01_core/src/engine/introspect.rs` — 2 testes (`p504_within_selector_query`, `p504_parent_locations_index`).
- `01_core/src/engine/stdlib/collections.rs` — testes de dict methods (embutidos na suite existente).
- `01_core/src/entities/layout_types.rs` — `p504_align2d_start_end_parse`.
- `01_core/src/entities/selector.rs` — `p504_selector_within_estrutural`.

Comando:

```bash
cargo test -p typst-core p504_ -- --nocapture
```

Resultado: **17/17 passaram**.

---

## 6. Validação arquitetural

```bash
cargo build
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 7. Ficheiros alterados

```text
01_core/src/entities/content.rs
01_core/src/entities/elements/list_item.rs
01_core/src/entities/func.rs
01_core/src/entities/introspector.rs
01_core/src/entities/layout_types.rs
01_core/src/entities/selector.rs
01_core/src/entities/value.rs
01_core/src/engine/eval/bindings.rs
01_core/src/engine/eval/closures.rs
01_core/src/engine/eval/mod.rs
01_core/src/engine/eval/repr.rs
01_core/src/engine/eval/tests.rs
01_core/src/engine/introspect.rs
01_core/src/engine/layout/cursor.rs
01_core/src/engine/layout/mod.rs
01_core/src/engine/stdlib/collections.rs
01_core/src/engine/stdlib/foundations.rs
01_core/src/engine/stdlib/mod.rs
01_core/src/engine/stdlib/structural.rs
lab/parity/tests/structural_parity.rs
```

---

## 8. Decisão para o passo seguinte

Com o P504 fechado (16/16 OK, zero regressões), o próximo passo deve focar:

1. Continuar o fecho de gaps pendentes do P500 (`list/enum` indent, `state/counter/context`).
2. Expandir o audit P504 para cenários combinados (e.g., `selector(heading).within(figure).or(...)`).
3. Documentar L0 das novas entradas (`selector.md`, `introspector.md`) caso ainda não existam.

---

## A. Apêndice — Comando de re-execução completa

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Testes unitários P504
cargo test -p typst-core p504_ -- --nocapture

# Bateria de paridade estrutural
cd lab/parity
cargo test --test structural_parity -- --nocapture

# Linter arquitetural
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline
crystalline-lint .
```
