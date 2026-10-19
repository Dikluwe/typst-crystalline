# Diagnóstico P1140.10 — superfície pública de `linebreak`

**Data:** 2026-08-24  
**Estado:** fechado para superfície; efeito visual reservado ao P1140.11  
**Baseline vanilla:** `a51e02804`  
**Commit base cristalino:** `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`

## Resultado

O cristalino agora expõe `linebreak` como função global e preserva tanto o
valor quanto a presença explícita do campo `justify`. Isso fecha binding,
morfologia, `repr` e reflexão sem alterar ainda o algoritmo de layout.

| observável | vanilla ratificado | cristalino P1140.10 |
|---|---|---|
| `type(linebreak)` | `function` | `function` |
| `repr(linebreak())` | `linebreak()` | `linebreak()` |
| `repr(linebreak(justify: false))` | `linebreak(justify: false)` | igual |
| `repr(linebreak(justify: true))` | `linebreak(justify: true)` | igual |
| `repr([\ ])` | `sequence(linebreak(), [ ])` | igual |
| `linebreak().func() == linebreak` | `true` | `true` |
| `linebreak().fields()` | `(:)` | `(:)` |
| `linebreak(justify: false).fields()` | `(justify: false)` | igual |
| `linebreak().has("justify")` | `false` | `false` |
| `linebreak(justify: false).has("justify")` | `true` | `true` |

Os observáveis foram convertidos em testes P1140.10 e passaram em conjunto
com toda a suíte L1. Argumento posicional, named desconhecido e valor não bool
também são rejeitados.

## Preservação

`Content::linebreak()` continua produzindo a forma omitida (`false/false`),
portanto a sintaxe markup e consumidores internos mantêm a quebra simples. Os
testes P584, P996 e P997 passaram, assim como as suítes completas L1 e L3.

## Proveniência do fecho

Medição em `2026-08-24T11:38:04-03:00`, working tree não commitado:

- `git diff HEAD --stat`: **46 ficheiros alterados, 789 inserções, 140 remoções**;
- `git status --short`: **50 entradas**;
- L1: **5150 passed, 0 failed**;
- L3: **828 passed, 0 failed**;
- build, fmt, lint e diff-check: exit 0.

Essas contagens abrangem alterações acumuladas dos passos anteriores na mesma
árvore; não devem ser atribuídas isoladamente ao P1140.10.

## Divergência restante

`justify: true` é armazenado e refletido, mas ainda executa o mesmo
`flush_line()` de uma quebra não justificada. O P1140.11 deve medir posições e
implementar a distribuição visual do espaço restante, incluindo shaped text,
RTL, links e linhas sem oportunidades de expansão. Até isso ocorrer, não há
paridade completa do comportamento de layout de `linebreak`.
