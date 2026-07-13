# Achados adiados na cadeia P700-725 — lista de controlo

## Por resolver

| Achado | Onde foi encontrado | Prioridade | Estado |
|---|---|---|---|
| `curve` compila mas renderiza página em branco (bug de render, não de namespace) | P723 | Alta — namespace funciona, mas o resultado visual está errado; pode afectar `cetz` mesmo depois de P724 | Aberto |
| `block(fill: none)` e `block(stroke: none)` rejeitados — **bloqueio actual do cetz**: `block(fill)` só aceita `Value::Color` (`stdlib/layout.rs:858-865`); `extract_stroke` só aceita `Length/Color/Stroke` (`layout.rs:431-449`). Vanilla aceita `none` (= sem fill/stroke). Consumidor: `canvas.typ:111,129` de cetz (`block.with(breakable: false)` invocado com `fill: background, stroke: stroke`, defaults `none` em `canvas.typ:25`). Medido: erro `block(fill): espera Color, recebeu none` após ~54s de eval | P725 | Alta — bloqueia o critério final da cadeia; correção pequena (aceitar `Value::None` → sem fill/stroke, em block/box/grid/table conforme sonda de P726) | Aberto |
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

## Scope-outs conscientes

- `Ratio`/`Relative` mistos em divisão de `Length` — não-alcançável por sintaxe actual (P713).
- `Length * Ratio` / `Ratio * Length` — `Value::Ratio` não produzível por sintaxe de utilizador (P725, mesmo raciocínio de P713).
- NaN como valor de utilizador (`float.nan`/`calc.nan`) — inexistente no eval cristalino (`float` é só `Type::Float`, `eval/mod.rs:1086`; `calc` expõe `inf` mas não `nan`, `stdlib/calc.rs:108`); caminhos NaN testados só ao nível de `eval_binary_op` (P725).
- Dupla avaliação de efeitos no fall-through de métodos mutantes — sem consumidor em `cetz` (P716/P717).
- `Dict * Int` — confirmado inexistente no vanilla, não é lacuna (P722).

## Regra

Antes de fechar qualquer passo desta cadeia, verificar esta lista. Se o passo resolver algum item, marcar como fechado, com o número do passo. Se um novo achado for adiado, entra aqui antes do relatório desse passo ser dado como terminado.
