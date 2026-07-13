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
| Mensagem de aridade do `for` diverge do vanilla (falta hint) | P540 (pré-existente), reconfirmado P719 | Baixa — comportamento correcto, só a mensagem difere | Aberto |
| Formatação de `Float` diverge (`2.0` vs `2`) | P713 | Baixa — cosmética, não afecta valor | Aberto |
| Argumento nomeado extra sem parâmetro (`f(1, z: 2)`) aceite silenciosamente | P708 | Média — categoria de "aceitação silenciosa incorrecta", mesma família do bug de binding que P708 corrigiu para posicionais | Aberto |

## Scope-outs conscientes (não precisam de passo próprio, salvo mudança de contexto)

- `Ratio`/`Relative` mistos em divisão de `Length` — não-alcançável por sintaxe actual (P713).
- Dupla avaliação de efeitos no fall-through de métodos mutantes — sem consumidor em `cetz` (P716/P717).

## Regra a partir de agora

Antes de fechar qualquer passo desta cadeia, verificar esta lista. Se o passo resolver algum item, marcar como fechado aqui, com o número do passo. Se um novo achado for adiado, entra aqui antes do relatório desse passo ser dado como terminado — não só na prosa do relatório em si.
