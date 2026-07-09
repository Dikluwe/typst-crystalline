# Relatório de Paridade — P655

**Passo:** 655  
**Data:** 2026-07-09  
**Foco:** Terceira ronda de deteção de falhas silenciosas, usando avisos nativos do compilador e do `clippy` em vez de padrões de texto.  
**Dependências:** P633, P650 (rondas anteriores por padrões de texto).  
**Hash do commit com o relatório:** `fa4d5fd1d`

---

## 1. Sonda

### 1.1 `cargo build --workspace` do zero

```bash
cargo clean -p typst-core -p typst-infra -p typst-shell -p typst-wiring 2>/dev/null
cargo build --workspace 2>&1 | tee /tmp/p655-build-warnings.txt
```

Resultado: **43 warnings**.

Distribuição por tipo (todos em `typst-core` ou `typst-infra`):

| Tipo de aviso | Contagem | Origem |
|---------------|----------|--------|
| `unused_imports` | ~30 | 01_core, 03_infra |
| `dead_code` | 4 | 01_core, 03_infra |
| `deprecated` | 8 | `FrameItem::Text` e `ttf_parser::Face::table_data` |

Nenhum aviso de `unused_must_use` / `must_use` / `unused_results`. Nenhum `Result` ou `Option` criado e descartado sem tratamento foi detectado pelo compilador.

### 1.2 Revisão de `#[allow(...)]` fora de testes

Encontrados 41 atributos `#[allow(...)]` / `#![allow(...)]` fora de testes:

- **22× `#![allow(deprecated)]`** — `FrameItem::Text` fallback path (P483). Documentado como legítimo em ficheiros de layout e exportação.
- **7× `#[allow(dead_code)]`** — APIs de reparse incremental ou funções usadas internamente; comentadas como migração futura.
- **13× `#[allow(clippy::disallowed_methods)]`** em `rules/stdlib/calc.rs` — uso de `f32::powf`, `f64::powi`, etc., centralizado no helper `trig_op`.
- **1× `#[allow(clippy::needless_lifetimes)]`** em `entities/world_types.rs`.
- **1× `#[allow(clippy::match_like_matches_macro)]`** em `rules/parse/parser.rs`.
- **2× `#[allow(unused_imports)]`** em `entities/content.rs`.
- **1× `#[allow(unused_macros)]`** em `utils.rs`.
- **1× `#[allow(dead_code)]`** em `04_wiring/src/eviction.rs` — `P204E` expõe via CLI.

Nenhum `#[allow(unused_must_use)]`, `#[allow(unused_results)]` ou similar que escondesse descarte de `Result`.

### 1.3 `cargo clippy` com lints para falhas silenciosas

Comando com `--all-targets`:

```bash
cargo clippy --workspace --all-targets -- \
  -W clippy::unwrap_used \
  -W clippy::expect_used \
  -W clippy::result_unwrap_used \
  -W clippy::option_unwrap_used \
  -W clippy::let_underscore_must_use \
  -W clippy::let_underscore_future \
  -W clippy::unused_result_ok \
  2>&1 | tee /tmp/p655-clippy-warnings.txt
```

Resultado: **2535 warnings** + **4 erros**.

Sem `--all-targets` (só lib/bin):

Resultado: **211 warnings** + **3 erros**.

#### 1.3.1 Avisos dos lints pedidos (só lib)

| Lint | Contagem | Notas |
|------|----------|-------|
| `unwrap_used` | 35 | Invariantes locais; verificados abaixo |
| `expect_used` | 13 | Invariantes locais; verificados abaixo |
| `result_unwrap_used` | 2 | Renamed lint (gera aviso de renamed lint) |
| `option_unwrap_used` | 2 | Renamed lint (gera aviso de renamed lint) |
| `let_underscore_must_use` | 0 | — |
| `let_underscore_future` | 0 | — |
| `unused_result_ok` | 1 | `Route::within` em `world_types.rs:346` |

#### 1.3.2 Erros do clippy (sem `--all-targets`)

| Erro | Local | Natureza |
|------|-------|----------|
| `deprecated_semver` | `entities/layout_types.rs:203` | `since = "P483"` não é semver |
| `never_loop` | `rules/stdlib/structural.rs:1856` | Loop `for k in args.named.keys()` com `return` imediato |
| `never_loop` | `rules/stdlib/structural.rs:1895` | Idem para `cancel()` |

O quarto erro (`approx_constant`) apareceu só com `--all-targets` em `rules/stdlib/mod.rs:1557`, dentro de testes.

---

## 2. Classificação

### 2.1 Categoria 1 — Já coberto por P633-P654

Nenhum. Os avisos nativos do compilador e estes lints do clippy não repetem os casos encontrados nas rondas anteriores (bibliografia, grid, erros de parser, `sorted()`).

### 2.2 Categoria 2 — Novo, inofensivo (com razão documentada)

#### `cargo build`

- Todos os 43 avisos: `unused_imports`, `dead_code` e `deprecated`. Nenhum indica descarte de `Result`/`Option` nem falha silenciosa.

#### `unwrap_used` / `expect_used`

Verificados os casos não-triviais:

- `rules/eval/bibtex.rs:96` — `from_utf8` sobre bytes ASCII alfabéticos previamente validados. Sempre válido.
- `rules/eval/mod.rs:457` — `Option<char>::unwrap()` em função auxiliar que trata `None` primeiro. Invariante local.
- `rules/eval/mod.rs:541` — `parts.pop().unwrap()` após `matches!(parts.last(), ...)`. Sempre seguro.
- `rules/eval/rules.rs:1213` — `f.element_name().unwrap()` após `is_some()`. Invariante local.
- `rules/eval/math.rs:254` — `cols.last_mut().unwrap()`; `cols` inicializado com `vec![vec![]]` e nunca esvaziado.
- `rules/stdlib/foundations.rs:399` — `char::from_digit(digit, base).unwrap()`; `digit < base` e base validada (2-36).
- `entities/source.rs`, `entities/span.rs`, `entities/syntax_node.rs` — unwraps em construtores internos de IDs/spans; invariantes de domínio.

#### `unused_result_ok`

- `entities/world_types.rs:346` — `compare_exchange(...).ok()` em `Route::within`. O resultado descartado é aceitável: se o valor atual já mudou, não se atualiza o `upper bound`. Comportamento intencional, comentado.

#### Erros do clippy

- `deprecated_semver` — metadados de `#[deprecated]`. Não afecta runtime.
- `never_loop` (×2) — `for k in args.named.keys() { return Err(...) }`. Reporta erro no primeiro named arg e sai. Semântica intencional (P296 scope-out cosméticos), embora o estilo possa ser melhorado.

### 2.3 Categoria 3 — Novo, suspeito

Nenhum. Todos os casos acima têm justificação documentada ou invariante local.

### 2.4 Categoria 4 — Novo, confirmado (falha real)

Nenhum.

---

## 3. Validação

```bash
cargo test --workspace
```

Resultado: todos os crates passaram.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 4. Decisão

- A terceira ronda, usando avisos do compilador e lints do clippy, **não encontrou falhas silenciosas novas** além das já tratadas em P633-P654.
- Os `#[allow(...)]` existentes são justificados e não escondem descarte de `Result`.
- Não houve alterações de código neste passo. O resultado é um reforço de confiança nas rondas anteriores, registado com os números brutos.
