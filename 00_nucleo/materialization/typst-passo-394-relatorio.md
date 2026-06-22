# Passo 394 — relatório: materialização de `eval(string)`

**Tipo:** materialização (L1 — stdlib runtime de re-eval; sem tipo Rust novo; zero I/O).
**Data:** 2026-06-22. **HEAD:** pós-`6e29fbaba`.
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente
em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Materializou-se `eval(source)` — função nativa que re-parseia e re-avalia uma string como
código Typst no contexto actual.

- `01_core/src/entities/func.rs` — adiciona `FuncRepr::NativeWithEngine(NativeFuncWithEngine)`
  e `Func::native_with_engine`, permitindo que uma nativa receba `&mut Scopes<'_>` e
  `&mut Engine<'_>` sem alterar o ABI das restantes nativas.
- `01_core/src/rules/eval/closures.rs` — `apply_func` passa a receber `&mut Scopes` e
  despacha a variante `NativeWithEngine`, propagando scopes/engine para `native_eval`.
- `01_core/src/rules/eval/rules.rs`, `rules/eval/closures.rs`, `rules/introspect/from_tags.rs` —
  callers de `apply_func` actualizados (closures, state/counter callbacks usam scope vazio).
- `01_core/src/entities/source.rs` — `Source::detached_with_parser` para parse de blocos de
  código não-markup.
- `01_core/src/rules/stdlib/eval.rs` — `native_eval`:
  - valida argumentos (string única, sem named args);
  - rejeita strings sintacticamente inválidas;
  - parseia com `parse_code`;
  - avalia as expressões numa engine local (clone de `styles`/`show_rules`/`sink`), confinando
    `#set`/`#show` internos;
  - devolve o `Value` da última expressão.
- `01_core/src/rules/eval/mod.rs` — regista `eval` em `make_stdlib` via
  `Func::native_with_engine`.
- `00_nucleo/prompts/rules/stdlib/eval.md` — L0 novo/revisado.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` — `eval(string)`
  reclassificado de `ausente` para `implementado`; `mode: "markup"` documentado como scope-out.

`cargo test --workspace` + `crystalline-lint .` verdes; **15** testes de `eval` passam
(7 em `rules/eval/tests.rs` + 8 em `rules/stdlib/mod.rs`).

## Protocolo de Nucleação cumprido

1. Redigiu-se o L0 (`eval.md`) e propagaram-se hashes.
2. Implementou-se o runtime de re-eval com TDD (variante ABI + `native_eval` + testes).
3. Linhagem `@prompt`/`@prompt-hash` actualizada via `crystalline-lint --fix-hashes`.

## Decisão de engenharia

A paridade (ADR-0107) é semântica: `eval(source)` avalia o string como código Typst e devolve
o valor resultante. Cristalino adopta **modo código por default** (`parse_code` + `eval_expr`)
porque é o substrato mínimo que satisfaz os casos de teste declarados (`1 + 2`, `x * 2`,
`[*bold*]`, `123`). O vanilla default é `mode: "markup"`; essa diferença é declarada
explicitamente como **scope-out** e reflectida no inventário.

A variante `FuncRepr::NativeWithEngine` foi escolhida em vez de alargar o ABI de todas as
nativas, minimizando a intrusão: apenas o despacho em `apply_func` e os callers directos
precisaram de ajuste. Não foi criado nenhum tipo `Value` novo; `eval` devolve os tipos já
existentes (`Int`, `Content`, etc.).

A engine local garante que efeitos laterais de `#set`/`#show` dentro do string avaliado não
vazam para o chamador, espelhando o confinamento de content block (`[]`).

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `eval("1 + 2")` | `3` | ✓ |
| `#let x = 5; eval("x * 2")` | `10` | ✓ |
| `eval("[*bold*]")` | `Value::Content` com strong "bold" | ✓ |
| `eval("123")` | `123` | ✓ |
| Sintaxe inválida (`"let x ="`) | erro | ✓ |
| Identificador desconhecido (`"hello"`) | erro | ✓ |
| Tipo errado (`eval(123)`) | erro | ✓ |
| Argumento nomeado (`mode:`) | erro | ✓ |
| Aridade errada (`eval()`) | erro | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `eval("1 + 2")` devolve `Value::Int(3)` | ✓ |
| 2 | `eval("x + 2")` vê a variável `x` do scope actual | ✓ |
| 3 | Zero tipo `Value` novo | ✓ |
| 4 | Tests verdes; lint zero; hashes propagados | ✓ 15/15; `✓ No violations` |
| 5 | Inventário 148 actualizado | ✓ A.8 + Tabela C |
| 6 | L0 salvo e hashado antes do código | ✓ `eval.md` |

## Artefactos

- Código: `01_core/src/entities/func.rs`, `entities/source.rs`, `rules/eval/closures.rs`,
  `rules/eval/rules.rs`, `rules/eval/mod.rs`, `rules/introspect/from_tags.rs`,
  `rules/stdlib/eval.rs`.
- L0: `00_nucleo/prompts/rules/stdlib/eval.md`.
- Testes: `01_core/src/rules/eval/tests.rs` (7), `01_core/src/rules/stdlib/mod.rs` (8).
- Inventário 148 — `eval(string)` implementado; `mode: "markup"` scope-out.
- este relatório.

## Nota sobre o Tekt

Este passo encerra uma dívida genuína acidental (balde D) identificada na sonda 389. O
aumento de escopo M foi controlado: a única alteração estrutural foi a nova variante de
`FuncRepr`; o resto é wiring de parser/eval já existente. O ciclo manteve-se limpo porque o
substrato (`Engine`, `Scopes`, `parse_code`, `eval_expr`) já estava materializado — a sonda 389
tinha razão.
