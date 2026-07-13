# Achados adiados na cadeia P700-724 — lista de controlo

## Por resolver

| Achado | Onde foi encontrado | Prioridade | Estado |
|---|---|---|---|
| `curve` compila mas renderiza página em branco (bug de render, não de namespace) | P723 | Alta — namespace funciona, mas o resultado visual está errado; pode afectar `cetz` mesmo depois de P724 | Aberto |
| `Mul` com `Length` ausente — zero braços em `operators.rs` (só `Div`, P713); vanilla suporta as 4 combinações (medido: `2.0 * 1pt → 2pt`, `1pt * 2.0 → 2pt`, `2 * 1pt → 2pt`, `1pt * 2 → 2pt`). **Bloqueio actual do cetz**: operando medido `2.0 * 28.35pt` (= 2.0 × 1cm), consumidor `canvas.typ:146-147,182-186` (escala de coordenadas por `length`, default 1cm de `canvas.typ:25`) | P724 | Alta — bloqueia o critério final da cadeia; correção pequena (4 braços + testes, L0 `rules/eval/ops.md`) | Aberto |
| Argumento nomeado extra sem parâmetro (`f(1, z: 2)`) aceite silenciosamente | P708 | Média | Aberto |
| Formatação de `Float` diverge (`2.0` vs `2`) | P713 | Baixa | Aberto |

## Fechados

| Achado | Fechado em |
|---|---|
| `repr_value` usa Debug do Rust em vez de repr Typst | P721 |
| `Array * Int` (repetição) ausente | P722 |
| Mensagem de aridade do `for` divergia do vanilla | P723 (efeito colateral da delegação a `destructure_let`) |
| Destructuring em parâmetro de closure (`((i, segment)) => ...`) descartado silenciosamente (`closures.rs` braço `_ => None`); bloqueava `cetz` em `path-util.typ:453` | P724 (`ClosureParam.pattern: Option<SyntaxNode>` + bind via `destructure_let` em `apply_closure`, mirror do vanilla `call.rs:655-665`) |

## Scope-outs conscientes

- `Ratio`/`Relative` mistos em divisão de `Length` — não-alcançável por sintaxe actual (P713).
- Dupla avaliação de efeitos no fall-through de métodos mutantes — sem consumidor em `cetz` (P716/P717).
- `Dict * Int` — confirmado inexistente no vanilla, não é lacuna (P722).

## Regra

Antes de fechar qualquer passo desta cadeia, verificar esta lista. Se o passo resolver algum item, marcar como fechado, com o número do passo. Se um novo achado for adiado, entra aqui antes do relatório desse passo ser dado como terminado.
