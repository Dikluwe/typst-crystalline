# Achados adiados na cadeia P700-720 — lista de controlo

**Motivo:** `repr_value` (formatação Debug em vez de repr Typst) foi
prometido como "próximo passo" em P710 e outra vez em P711, mas P712 foi
reatribuído a um bug mais urgente (`measure()`) e o assunto nunca mais
apareceu até P720. Esta lista existe para que isso não se repita — cada
item aqui tem de ser explicitamente marcado como fechado, com o número do
passo que o fechou, antes de ser removido.

## Por resolver

| Achado | Onde foi encontrado | Prioridade | Estado |
|---|---|---|---|
| `repr_value` usa `{:?}` (Debug do Rust) em vez do repr Typst para `Length`/`Ratio`/`Angle`/`Color`/`Stroke`/`Align` embutidos em markup | P710, reconfirmado P711 | Alta — afecta qualquer documento que embuta estes valores directamente | **Fechado em P721** (relatório: `paridade-producao-p721.md`; divergência residual documentada: componente auto de `stroke` indistinguível do default — limitação do modelo P227, scope-out no L0) |
| `Array * Int` (repetição) ausente | P720 | Média-alta — mesmas linhas de `hobby.typ` que motivaram P720, vai bloquear `cetz` outra vez em breve | **Fechado em P722** (relatório: `paridade-producao-p722.md`; ambas as ordens, paridade `Array::repeat`; `Dict * Int` scope-out — inexistente no vanilla) |
| Mensagem de aridade do `for` diverge do vanilla (falta hint) | P540 (pré-existente), reconfirmado P719 | Baixa — comportamento correcto, só a mensagem difere | **Fechado em P723** (o `for` passou a delegar em `destructure_let`; mensagens de `wrong_number_of_elements` com hint, mirror do vanilla) |
| Formatação de `Float` diverge (`2.0` vs `2`) | P713 | Baixa — cosmética, não afecta valor | Aberto |
| Argumento nomeado extra sem parâmetro (`f(1, z: 2)`) aceite silenciosamente | P708 | Média — categoria de "aceitação silenciosa incorrecta", mesma família do bug de binding que P708 corrigiu para posicionais | Aberto |
| `curve(...)` compila (exit 0) mas renderiza página em branco — sonda `#curve(curve.move(...), curve.line(...), curve.cubic(...), curve.close())` no cristalino vs vanilla que desenha a forma (PNGs medidos em `/tmp/p723-{vanilla,cristalino}.png`) | P723 | Média — bug de render, separado do namespace (este existe desde P513); sem consumidor directo no caminho actual do cetz | Aberto |
| Destructuring em parâmetro de closure (`((i, segment)) => ...`) silenciosamente descartado — `eval_closure_expr` (`closures.rs:345`, braço `_ => None`) cria a closure com 0 params; na chamada o argumento extra dá `unexpected argument`. **É o bloqueio actual do cetz** (`path-util.typ:453`: `segments.enumerate().filter(((i, segment)) => ...)`) | P723 | Alta — bloqueia o critério final da cadeia; requer mudança no modelo `ClosureParam` (guardar o pattern, não só o nome) + bind via `destructure_let` em `apply_closure` | Aberto |

## Scope-outs conscientes (não precisam de passo próprio, salvo mudança de contexto)

- `Ratio`/`Relative` mistos em divisão de `Length` — não-alcançável por sintaxe actual (P713).
- Dupla avaliação de efeitos no fall-through de métodos mutantes — sem consumidor em `cetz` (P716/P717).

## Regra a partir de agora

Antes de fechar qualquer passo desta cadeia, verificar esta lista. Se o passo resolver algum item, marcar como fechado aqui, com o número do passo. Se um novo achado for adiado, entra aqui antes do relatório desse passo ser dado como terminado — não só na prosa do relatório em si.
