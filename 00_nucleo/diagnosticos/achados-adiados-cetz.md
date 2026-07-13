# Achados adiados na cadeia P700-726 — lista de controlo

## Por resolver

| Achado | Onde foi encontrado | Prioridade | Estado |
|---|---|---|---|
| `curve`/path renderiza página em branco — **bloqueio actual da paridade visual do cetz** (a cadeia de eval fechou em P726: cetz compila exit 0; medido: vanilla 1451 px não-brancos vs cristalino **0** no canvas, 1043 vs 0 em `#curve` directo). `rect`/`circle` renderizam bem (979 vs 972 px) — bug específico de paths. Render em `01_core/src/rules/layout/curve.rs` | P723, confirmado P726 | Alta — único bloqueio restante para paridade de pixels no cetz; candidato natural a P727 | Aberto |
| hline/vline `stroke: none` rejeitado — vanilla aceita (linha não desenhada, medido exit 0); cristalino guarda `Stroke` não-opcional em `GridHLineElem`/`TableHLineElem` (+vlines) e o render desenha sempre (`rules/layout/grid.rs:668-689`); correcção exige entidade `Option<Stroke>` + salto no render | P726 | Baixa — sem consumidor em cetz; não usar zero-thickness (width 0 em PDF é hairline) | Aberto |
| `line(end:)` rejeitado ("argumento nomeado inesperado em line(): 'end'") — vanilla aceita | P726 | Baixa — sem consumidor em cetz | Aberto |
| Argumento nomeado extra sem parâmetro (`f(1, z: 2)`) aceite silenciosamente | P708 | Média | Aberto |
| Formatação de `Float` diverge (`2.0` vs `2`) | P713 | Baixa | Aberto |
| `Length / Float` com NaN (P713) propaga NaN — vanilla saneia para 0 via `Scalar::new` (medido em P725 no caminho `Mul`: `repr(1pt * float.nan)` → `0pt`; mesmo mecanismo aplica-se à divisão). P725 saneou só o caminho `Mul` (um-bug-por-passo) | P725 | Baixa — NaN é inalcançável por sintaxe de utilizador no cristalino (ver scope-outs) | Aberto |

## Fechados

| Achado | Fechado em |
|---|---|
| `repr_value` usa Debug do Rust em vez de repr Typst | P721 |
| `Array * Int` (repetição) ausente | P722 |
| Mensagem de aridade do `for` divergia do vanilla | P723 (efeito colateral da delegação a `destructure_let`) |
| Destructuring em parâmetro de closure (`((i, segment)) => ...`) descartado silenciosamente (`closures.rs` braço `_ => None`); bloqueava `cetz` em `path-util.typ:453` | P724 (`ClosureParam.pattern: Option<SyntaxNode>` + bind via `destructure_let` em `apply_closure`, mirror do vanilla `call.rs:655-665`) |
| `Mul` com `Length` ausente — zero braços em `operators.rs`; bloqueava `cetz` em `canvas.typ:146-147,182-186` | P725 (4 braços em `eval_binary_op` sobre `Length: Mul<f64>`, paridade `foundations/ops.rs:238-243`, + saneamento NaN → 0 por componente, paridade `Scalar::new`) |
| `block(fill/stroke: none)` rejeitado — bloqueava `cetz` em `canvas.typ:111,129` | P726 (braços `Some(Value::None)` em block/box/grid/table + table.cell/grid.cell — 12 pontos; `extract_stroke` inalterado; scope-out hline/vline) |

## Scope-outs conscientes

- `Ratio`/`Relative` mistos em divisão de `Length` — não-alcançável por sintaxe actual (P713).
- `Length * Ratio` / `Ratio * Length` — `Value::Ratio` não produzível por sintaxe de utilizador (P725, mesmo raciocínio de P713).
- NaN como valor de utilizador (`float.nan`/`calc.nan`) — inexistente no eval cristalino (`float` é só `Type::Float`, `eval/mod.rs:1086`; `calc` expõe `inf` mas não `nan`, `stdlib/calc.rs:108`); caminhos NaN testados só ao nível de `eval_binary_op` (P725).
- `stroke(paint: none)` — erro no vanilla também ("expected color, gradient, tiling, or auto, found none", medido P726); o cristalino mantém o seu erro — paridade, não lacuna.
- Dupla avaliação de efeitos no fall-through de métodos mutantes — sem consumidor em `cetz` (P716/P717).
- `Dict * Int` — confirmado inexistente no vanilla, não é lacuna (P722).

## Regra

Antes de fechar qualquer passo desta cadeia, verificar esta lista. Se o passo resolver algum item, marcar como fechado, com o número do passo. Se um novo achado for adiado, entra aqui antes do relatório desse passo ser dado como terminado.
