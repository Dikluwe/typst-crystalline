# Passo 1455 — eliminar a paleta string hardcoded de paint

**Estado:** PROPOSTO — BLOQUEADO NO GATE ADR-0127
**Data da medição:** 2026-09-18T21:40:56-03:00
**Commit medido:** `3360722a3eb3b63371d2ad9e2d1758d85cd9f98d`
**Árvore medida:** `bf84d89cc34bd58bb00c56b1a7daeda9d5125adf`, limpa
**Regime:** `executado sem atestação de isolamento`

## 1. Próxima unidade

P1454 eliminou a única pós-reescrita produtiva de cor nas camadas L1–L4. A
auditoria ampliada encontrou uma segunda classe de hardcode: uma paleta CSS
duplicada dentro de `stdlib/shapes::parse_color`, consumida também por
`stdlib/text/deco`.

Não é a tabela Typst canônica de `stdlib/color`; é um catálogo separado de 19
spellings, com valores diferentes (`red = #ff0000`, por exemplo), usado para
aceitar strings onde a linguagem exige paint tipado.

## 2. Medição

Vanilla ratificado: SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
Cristalino medido: SHA-256
`1ae2028599b9e9ec2c5de9dc303b47f1efb184117ed6bfb6716d217c44e62d84`.

| Expressão | Vanilla | Cristalino |
|---|---|---|
| `rect(fill: "red")` | erro de tipo string | aceita |
| `line(stroke: "red")` | erro de tipo string | aceita |
| `underline(stroke: "red", [x])` | erro de tipo string | aceita |
| `highlight(fill: "red", [x])` | erro de tipo string | aceita |

Hashes dos owners medidos:

- L0 shapes: `335af3fc7e890f0c01b94819e9f793ed2a0e0ba5fb051514574f39810e0d8ce9`;
- código shapes: `f8adf97bc7e6857c6f159f7d2e6f256f81c48624d934b899e3d502984393248f`;
- L0 deco: `33d7957a8c3ffcbe5aab4100d7bf1156107b5aac5a2540c7ff281cf42679fec2`;
- código deco: `991752687b93b9d3ac93aa768033a029e3fd0b4be4c2179f66eab784359b40e4`.

## 3. Escopo proposto

1. Remover os 19 braços string de `parse_color` sem criar catálogo sucessor.
2. Manter cores nomeadas somente pelo scope canônico, como `Value::Color`.
3. Fazer fill/stroke/deco rejeitarem string de forma explícita e fail-closed,
   sem convertê-la em `None` e acionar fallback.
4. Preservar `none`, tipos paint já suportados, omissão e fallback legítimo.
5. Escrever testes bilaterais RED→GREEN para shapes, line, underline e
   highlight; atacar aliases e promoção de erro a sucesso.

Fora de escopo: alterar os valores da paleta Typst, remover constantes
nomeadas, completar unions de paint ainda não materializadas ou refatorar
layout/export.

## 4. Gate obrigatório

A mudança remove sintaxe pública cristalina antes aceite. É quebra de
compatibilidade, uma das quatro classes que exigem parada no ADR-0127.
Portanto este passo atualiza L0 e é commitado, mas **não autoriza testes ou
código** até confirmação humana explícita posterior ao selo da proposta.

## 5. Execução após confirmação

Congelar manifesto, contrato, oráculos positivos/negativos/opacos e mutantes;
obter RED; implementar a coerção tipada; ressellar hashes; exigir mutation
score `1.0`, zero promoção de `Unknown`, testes focais e completos, `cargo
build`, `cargo fmt --all -- --check`, `crystalline-lint .` e `git diff
--check`. Produto e certificado ficam em commits separados.
