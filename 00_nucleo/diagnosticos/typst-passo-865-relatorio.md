# Relatório — typst-passo-865: `#text(...)` aceita argumentos nomeados de `#set text`

**Data:** 2026-07-23
**Passo:** P865
**Base:** commit `06a336b0c3ae42949c2bced6c4c6511b9d89bebd`
**Estado:** working tree não commitado (ver `git diff HEAD --stat` no §Proveniência).

---

## Proveniência da medição

```text
commit: 06a336b0c3ae42949c2bced6c4c6511b9d89bebd
working tree: alterações não commitadas (inclui alterações preexistentes de outros passos)
```

`git diff HEAD --stat` (estado final do working tree):

```text
 .../typst-passo-859-realize-proposito.md           |  41 ---
 00_nucleo/prompts/engine/eval.md                   |  24 +-
 00_nucleo/prompts/engine/layout.md                 |  28 +-
 00_nucleo/prompts/engine/stdlib/text.md            |  46 ++-
 00_nucleo/prompts/shell/cli.md                     |  50 +++-
 01_core/src/engine/eval/bibliography.rs            |   2 +-
 01_core/src/engine/eval/bindings.rs                |   2 +-
 01_core/src/engine/eval/closures.rs                |   2 +-
 01_core/src/engine/eval/control_flow.rs            |   2 +-
 01_core/src/engine/eval/flow.rs                    |   2 +-
 01_core/src/engine/eval/markup.rs                  |   2 +-
 01_core/src/engine/eval/math.rs                    |   2 +-
 01_core/src/engine/eval/mod.rs                     |   2 +-
 01_core/src/engine/eval/modules.rs                 |   2 +-
 01_core/src/engine/eval/rules.rs                   |  48 ++-
 01_core/src/engine/eval/tests.rs                   | 130 +++++++-
 01_core/src/engine/layout/cursor.rs                |  74 ++++-
 01_core/src/engine/layout/dynamic.rs               |   2 +-
 01_core/src/engine/layout/grid.rs                  |   2 +-
 01_core/src/engine/layout/grid_placement.rs        |   2 +-
 01_core/src/engine/layout/helpers.rs               |   2 +-
 01_core/src/engine/layout/hyphenation.rs           |   2 +-
 01_core/src/engine/layout/metrics.rs               |   2 +-
 01_core/src/engine/layout/mod.rs                   |  74 ++++-
 01_core/src/engine/layout/placement.rs             |   2 +-
 01_core/src/engine/layout/set_page.rs              |  20 +-
 01_core/src/engine/layout/slicing.rs               |   2 +-
 01_core/src/engine/layout/sub_frame.rs             |   2 +-
 01_core/src/engine/layout/tests.rs                 |  45 ++-
 01_core/src/engine/stdlib/text.rs                  | 332 +++++++++++++++++++--
 01_core/src/entities/content.rs                    |   7 +-
 01_core/src/entities/layout_types.rs               |  25 +-
 02_shell/build.rs                                  |   2 +-
 02_shell/src/cli.rs                                | 104 ++++++-
 04_wiring/src/main.rs                              |  18 +-
 04_wiring/tests/cli.rs                             | 175 +++++++++++
 36 files changed, 1131 insertions(+), 148 deletions(-)
```

> Nota: as alterações de 2 linhas em ficheiros como `eval/*.rs`, `layout/*.rs`,
> `entities/*.rs`, `02_shell/*` e `04_wiring/*` resultam principalmente da
> actualização de `@prompt-hash` pelo `crystalline-lint --fix-hashes` após
> alterações nos respectivos prompts L0. As alterações semânticas de P865
> concentram-se em `text.md`, `rules.rs`, `text.rs` e na secção P865 de
> `tests.rs`.

---

## Achado (medição de P861)

`#text("hello", size: 20pt)` (chamada de função, não `#set`) era rejeitada no cristalino com:

```text
error: text() argumento nomeado desconhecido: 'size'
```

O vanilla 0.15.0 aceita a chamada e produz texto em 20pt.

A sonda de P865 estendeu o alcance a outros argumentos de `#set text(...)`:
`font:`, `weight:`, `style:`, `fill:`, `tracking:`, `lang:`, `dir:`, `top-edge:`,
`bottom-edge:` e `variations:`. Antes da correção, todos eram rejeitados pela
validação rígida de `native_text`, que só aceitava `fill:` e `variations:`.

---

## Causa raiz

A lista de argumentos nomeados válidos era mantida em dois sítios distintos:

1. **`#set text(...)`** — `eval_set_rule` em `01_core/src/engine/eval/rules.rs`
   valida cada argumento contra `VANILLA_TEXT_SET_PROPS` e aplica-o na chain
   de estilos.
2. **`#text(...)`** — `native_text` em `01_core/src/engine/stdlib/text.rs`
   rejeitava qualquer named arg que não fosse `fill` ou `variations`.

`size:` estava implementado para `#set` mas não fora sincronizado com a
chamada de função.

---

## Implementação

Ficheiros alterados por P865:

- `00_nucleo/prompts/engine/stdlib/text.md` — L0 actualizado.
- `01_core/src/engine/eval/rules.rs` — `VANILLA_TEXT_SET_PROPS`,
  `expected_length_error`, `edge_cast_error` e `type_mismatch` tornados
  `pub(crate)` para reutilização.
- `01_core/src/engine/stdlib/text.rs` — `native_text` passa a aceitar todos os
  argumentos nomeados válidos em `#set text(...)`, com a mesma validação de
  tipo.
- `01_core/src/engine/eval/tests.rs` — testes P865.

### Detalhe técnico

`native_text` itera agora `args.named` e, para cada chave:

- `fill:` / `variations:` — comportamento existente (P492/P836).
- `size:`, `weight:`, `style:`, `tracking:`, `lang:`, `font:`, `top-edge:`,
  `bottom-edge:`, `dir:` — validação e transporte idênticos aos de
  `eval_set_rule`.
- `bold:` / `italic:` — rejeitados com `unexpected argument: {name}`
  (paridade `#set text`).
- Outros nomes dentro de `VANILLA_TEXT_SET_PROPS` mas ainda não capturados
  (scope-out) — aceites sem efeito.
- Nomes fora da lista — erro hard `unexpected argument: {name}`.

A validação partilhada usa os helpers:

- `require_length` — `expected length, found integer` com hint `...pt?`.
- `parse_text_weight` — `Int` ou nome simbólico.
- `parse_text_style` — `"normal" | "italic" | "oblique"`.
- `parse_text_lang` — BCP-47.
- `parse_text_font` — string, array de strings ou dict (named/legacy).
- `parse_text_edge` — métrica nomeada ou `Length`.

Os valores viajam pelo mesmo mecanismo que `#set text(...)`: `Style` tipado
(`Size`, `Weight`, `Tracking`, `Lang`, `Font`) ou canal custom
(`"text.style"`, `"text.top-edge"`, `"text.bottom-edge"`, `"text.dir"`).

---

## Testes adicionados

Local: `01_core/src/engine/eval/tests.rs`, secção P865 (a seguir aos testes
P492 de `text(...)`).

| Teste | Descrição |
|-------|-----------|
| `p865_text_size_named` | `#text("hello", size: 20pt)` produz `Content::Styled` com `StyleDelta.size == Some(20.0)`. |
| `p865_text_named_args_comuns` | Conjunto de 10 chamadas (`weight`, `style`, `tracking`, `lang`, `font`, `dir`, `top-edge`, `bottom-edge`, `fill+size`, `fill posicional+size`) que devem compilar. |
| `p865_text_size_int_erro` | `size: 12` dá `expected length, found integer`. |
| `p865_text_arg_desconhecido_erro` | `foo: 1` dá `unexpected argument: foo`. |
| `p865_text_bold_erro` | `bold: true` dá `unexpected argument: bold`. |
| `p865_text_weight_int_valido` | `weight: 700` reflete-se em `StyleDelta.weight`. |
| `p865_text_variations_continua_a_funcionar` | `variations:` continua a validar e aceitar. |
| `p865_text_scope_out_aceite_sem_erro` | `hyphenate: true` (scope-out) aceite sem erro. |

---

## Validação

### `cargo test -p typst-core --lib p865`

```text
running 8 tests
test engine::eval::tests::tests::p865_text_bold_erro ... ok
test engine::eval::tests::tests::p865_text_arg_desconhecido_erro ... ok
test engine::eval::tests::tests::p865_text_size_named ... ok
test engine::eval::tests::tests::p865_text_size_int_erro ... ok
test engine::eval::tests::tests::p865_text_variations_continua_a_funcionar ... ok
test engine::eval::tests::tests::p865_text_scope_out_aceite_sem_erro ... ok
test engine::eval::tests::tests::p865_text_weight_int_valido ... ok
test engine::eval::tests::tests::p865_text_named_args_comuns ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 4664 filtered out
```

### `cargo test --workspace`

A suíte completa foi executada com sucesso. Contagens por crate:

| Crate / Suite | Passaram | Falharam | Ignorados |
|---------------|----------|----------|-----------|
| `typst-core` (lib) | 4670 | 0 | 2 |
| `typst-infra` (lib) | 717 | 0 | 5 |
| `typst-shell` (lib) | 41 | 0 | 0 |
| `typst` (bin) | 2 | 0 | 0 |
| `cli` (integration) | 36 + 2 | 0 | 0 |
| `crystalline_lint` (integration) | 0 | 0 | 3 |

**Total: 5468 passaram; 0 falharam; 10 ignorados.**

### `crystalline-lint --fix-hashes .`

```text
Fixed 3 files:
  ./01_core/src/engine/eval/rules.rs            → 552d1ead
  ./01_core/src/engine/eval/tests.rs            → 552d1ead
  ./01_core/src/engine/stdlib/text.rs           → d55f9c11

Re-running analysis... ✅ 0 drift warnings remaining
```

`crystalline-lint .` posterior reporta apenas o prompt órfão
`infra/package_version_resolution.md` (não relacionado com P865).

---

## Diff resumido (P865)

```text
 00_nucleo/prompts/engine/stdlib/text.md            |  46 ++-
 01_core/src/engine/eval/rules.rs                   |  48 ++-
 01_core/src/engine/eval/tests.rs                   | 130 +++++++-
 01_core/src/engine/stdlib/text.rs                  | 332 +++++++++++++++++++--
```

---

## Decisão

P865 está implementado segundo o L0 actualizado. A funcionalidade de
`#text(...)` como chamada fica sincronizada com `#set text(...)` quanto aos
argumentos nomeados válidos. A suíte de testes foi adicionada e validada, e
tanto `cargo test --workspace` como `crystalline-lint .` (exceto o órfão
preexistente) reportam zero falhas.
