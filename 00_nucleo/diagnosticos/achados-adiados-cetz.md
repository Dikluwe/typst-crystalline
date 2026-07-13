# Achados adiados na cadeia P700-727 — lista de controlo

## Por resolver

| Achado | Onde foi encontrado | Prioridade | Estado |
|---|---|---|---|
| `and`/`or` sem short-circuit — `01_core/src/rules/eval/mod.rs:687-692` avalia ambos os operandos antes de despachar; vanilla faz short-circuit. Caso mínimo: `#let a = (1, 2)` + `type(a) == str and a.contains(".")` → erro "campo desconhecido em array: 'contains'" (vanilla: `false`). Bloqueia cetz em `draw/shapes.typ:608,611` (`type(first-elem) == str and not first-elem.contains(".")`): `line` sozinha ou após `circle` falha a compilar; `line` antes de `circle` compila mas a linha não aparece no PDF (anomalia por explicar — suspeita de memoização a esconder o erro). É o **bloqueio actual** do diff de pixels do cetz (1100 vs 1451 não-brancos; o círculo já renderiza via fallback P727) | P727 (validação cetz) | Alta — último bloqueio conhecido da paridade visual do cetz; candidato natural a P728 | Aberto |
| `polygon` sem fallback de stroke default — mesmo defeito de `curve` corrigido em P727 (`shapes.rs:348-351` não aplica `Smart::Auto` vanilla, `lab/typst-original/crates/typst-layout/src/shapes.rs:336-339`); sem caso medido com pixels (ADR-0108), registado por inspecção de código | P727 (sonda) | Média | Aberto |
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
| `curve`/path renderiza página em branco — `native_curve` (`01_core/src/rules/stdlib/shapes.rs:592-595`) não aplicava o fallback de stroke default (paridade vanilla `Smart::Auto`, `lab/typst-original/crates/typst-layout/src/shapes.rs:126-129`): o path chegava ao PDF sem operador de pintura. Medido: caso mínimo vanilla 523 px não-brancos vs cristalino 0 → após correcção 523 vs 523 (diff 0.048%, anti-aliasing) | P727 (fallback idêntico a `native_rect`: sem fill nem stroke → stroke preto 1pt; com fill sem stroke → sem stroke) |

## Scope-outs conscientes

- `Ratio`/`Relative` mistos em divisão de `Length` — não-alcançável por sintaxe actual (P713).
- `Length * Ratio` / `Ratio * Length` — `Value::Ratio` não produzível por sintaxe de utilizador (P725, mesmo raciocínio de P713).
- NaN como valor de utilizador (`float.nan`/`calc.nan`) — inexistente no eval cristalino (`float` é só `Type::Float`, `eval/mod.rs:1086`; `calc` expõe `inf` mas não `nan`, `stdlib/calc.rs:108`); caminhos NaN testados só ao nível de `eval_binary_op` (P725).
- `stroke(paint: none)` — erro no vanilla também ("expected color, gradient, tiling, or auto, found none", medido P726); o cristalino mantém o seu erro — paridade, não lacuna.
- Dupla avaliação de efeitos no fall-through de métodos mutantes — sem consumidor em `cetz` (P716/P717).
- `Dict * Int` — confirmado inexistente no vanilla, não é lacuna (P722).

## Regra

Antes de fechar qualquer passo desta cadeia, verificar esta lista. Se o passo resolver algum item, marcar como fechado, com o número do passo. Se um novo achado for adiado, entra aqui antes do relatório desse passo ser dado como terminado.
