# Achados adiados na cadeia P700-732 — lista de controlo

## Por resolver

| Achado | Onde foi encontrado | Prioridade | Estado |
|---|---|---|---|
| `polygon` aceita coordenadas Int/Float que o vanilla **rejeita** — vanilla: "expected relative length, found integer"/"found float" (medido em P732); o cristalino mantém a aceitação via `coord_component` (helper partilhado com a interface documentada de `curve`, `shapes.md`). Vértices com `Ratio` (`50%`) também scope-out — vanilla aceita (medido exit 0) | P732 (sonda) | Baixa — aceitação silenciosa sem consumidor em cetz | Aberto |
| Namespaces `emoji` e `pdf` ausentes no cristalino — `#type(emoji)` e `#type(pdf)` dão "unknown variable"; vanilla devolve `module` (medido, binário `lab/typst-original/target/release/typst`; `html` devolve unknown no vanilla compilado sem a feature — paridade acidental, não lacuna) | P731 (sonda) | Média — sem consumidor em cetz | Aberto |
| `color` e `gradient` expostos como `dictionary` no cristalino; vanilla expõe como `type` (medido). Distinto de P731: vanilla não os trata como módulos mas como tipos com constructors | P731 (sonda) | Média — sem consumidor em cetz | Aberto |
| `counter` e `state` expostos como `function` no cristalino; vanilla expõe como `type` (medido) | P731 (sonda) | Baixa — sem consumidor em cetz | Aberto |
| `FlowEvent::Return` não marcado como condicional no fim de `while`/`for` — o vanilla marca (`typst-eval/src/flow.rs:105-108,183-185`; em `if`/`else` também, `flow.rs:54-57` — este o cristalino já faz, P635); o cristalino não marca em `eval_while`/`eval_for` (`control_flow.rs`). Sem caso medido com comportamento divergente (encontrado por inspecção durante a auditoria de join); mecanismo distinto de join, fora do scope de P729 | P729 (auditoria de join, fora da cadeia cetz) | Baixa — sem caso medido | Aberto |
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
| `and`/`or` sem short-circuit + anomalia de ordem do cetz (era **dois** bugs) — (1) `eval/mod.rs` avaliava ambos os operandos antes de despachar; (2) a "anomalia de ordem/suspeita de memoização" era o braço `Expr::CodeBlock` devolver só a última expressão, sem `join` (paridade vanilla `typst-eval/src/code.rs:57` + `foundations/ops.rs:24-45`): `{ line(...); circle(...) }` valia só `(closure_circle)` — a primeira expressão perdia-se. Hipótese de memoização **refutada** (sem `#[comemo::memoize]` no eval de closures; casos puros erravam consistentemente). Medições: `{ (1,); (2,) }` vanilla `(1, 2)` vs cristalino `(2)`; `false and (1/0==0)` vanilla `false` | P728 (braço dedicado And/Or em `eval/mod.rs` + `operators::join` com tabela completa — None identidade, str/content/array/dict/args/bytes, resto erro "cannot join X with Y") |
| `array.slice` ausente — `01_core/src/rules/stdlib/collections.rs` só tinha `slice` para `Str`; o cetz usa `array.slice` em ≥10 locais. Medições vanilla: `.slice(1,3)` → `(2, 3)`, `.slice(-2)` → `(3, 4)`, `.slice(1)` → `(2, 3, 4)`, `.slice(0, count: 2)` → `(1, 2)`, `.slice(1, count: -1)` → `()`, `.slice(10)` → erro "array index out of bounds (index: 10, len: 4)", `.slice(1, 2, count: 2)` → erro "`end` and `count` are mutually exclusive" | P730 (`array_slice` + braço no dispatcher, mirror de `foundations/array.rs:279-300` sobre `locate(index, end_ok: true)`, mensagens de erro exactas do vanilla) |
| Namespaces embutidos `calc`/`sys`/`math`/`sym` expostos como `dictionary` em vez de `module` — bloqueava `cetz` em `aabb.typ:18` (`import calc: min, max` em corpo de função; `eval/modules.rs:187` exige `Value::Module`). Medições vanilla: os 4 devolvem `module`; cristalino devolvia `dictionary` | P731 (4 builders convertidos de `Value::Dict` para `Value::Module` — `make_calc_module`, `build_sym_module` (renomeada de `build_sym_dict`), `make_sys_module`, `make_math_module`; `lookup_math_op` em `eval/math.rs` actualizado para `m.scope()`; field access em `Value::Module` já existia desde P679) |
| `polygon` sem fallback de stroke default (mesmo defeito de `curve` pré-P727) **+ coordenadas `Length` rejeitadas** — a sonda mediu dois defeitos: (1) `extract_coordinate` só aceitava `cast_float()` (Int/Float), rejeitando `(0pt, 0pt)` com "argumento 0 não é uma coordenada válida" — o vanilla exige `Rel<Length>` e rejeita Int/Float ("expected relative length, found integer/float"); (2) sem fill nem stroke o polígono renderizava página em branco — medido via o caminho numérico: cristalino 0 px não-brancos vs vanilla 898 px. Após correcção: diff 0.0834% (sem cores), 0.1358% (fill red), 0.0797% (stroke blue) — anti-aliasing | P732 (`coord_component` aceita `Length` via `abs.to_pt()`; fallback `Smart::Auto` idêntico a `native_curve` P727: sem fill nem stroke → stroke preta 1pt; com fill sem stroke → sem stroke) |

## Scope-outs conscientes

- `Ratio`/`Relative` mistos em divisão de `Length` — não-alcançável por sintaxe actual (P713).
- `Length * Ratio` / `Ratio * Length` — `Value::Ratio` não produzível por sintaxe de utilizador (P725, mesmo raciocínio de P713).
- NaN como valor de utilizador (`float.nan`/`calc.nan`) — inexistente no eval cristalino (`float` é só `Type::Float`, `eval/mod.rs:1086`; `calc` expõe `inf` mas não `nan`, `stdlib/calc.rs:108`); caminhos NaN testados só ao nível de `eval_binary_op` (P725).
- `stroke(paint: none)` — erro no vanilla também ("expected color, gradient, tiling, or auto, found none", medido P726); o cristalino mantém o seu erro — paridade, não lacuna.
- Dupla avaliação de efeitos no fall-through de métodos mutantes — sem consumidor em `cetz` (P716/P717).
- `Dict * Int` — confirmado inexistente no vanilla, não é lacuna (P722).

## Regra

Antes de fechar qualquer passo desta cadeia, verificar esta lista. Se o passo resolver algum item, marcar como fechado, com o número do passo. Se um novo achado for adiado, entra aqui antes do relatório desse passo ser dado como terminado.
