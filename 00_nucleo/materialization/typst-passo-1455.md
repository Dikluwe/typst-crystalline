# Passo 1455 — eliminar a paleta string hardcoded de paint

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


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

Vanilla ratificado: identificador administrativo
[dado administrativo removido].
Cristalino medido: identificador administrativo
[dado administrativo removido].

| Expressão | Vanilla | Cristalino |
|---|---|---|
| `rect(fill: "red")` | erro de tipo string | aceita |
| `line(stroke: "red")` | erro de tipo string | aceita |
| `underline(stroke: "red", [x])` | erro de tipo string | aceita |
| `highlight(fill: "red", [x])` | erro de tipo string | aceita |

Hashes dos owners medidos:

- L0 shapes: [dado administrativo removido];
- código shapes: [dado administrativo removido];
- L0 deco: [dado administrativo removido];
- código deco: [dado administrativo removido].

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

## 6. Confirmação humana

O dono confirmou explicitamente a quebra de compatibilidade e autorizou a
materialização em 2026-09-18, após o commit de proposta `e5505f17c`.

## 7. Execução e resultado

Execução sem atestação de isolamento. O RED focal rejeitou o baseline porque
`parse_color(Value::Str("red"))` ainda retornava vermelho. A implementação
removeu integralmente os 19 spellings string, manteve a paleta canônica no
avaliador tipado, tornou `fill` fail-closed e alinhou os diagnósticos públicos
de string em shapes e decorações.

Em 2026-09-19T00:57:04Z, no working tree não commitado derivado de
`e5505f17ceb342b94424c2d1e5627cc523eb469a`, a campanha congelada executou 20
casos contra vanilla identificador administrativo [dado administrativo removido]
e candidato identificador administrativo [dado administrativo removido]:
zero falhas. Os testes focais de parse, rect, polygon, curve e highlight
passaram; `cargo build`, `cargo fmt --all -- --check`, `crystalline-lint .` e
`git diff --check` passaram.

`cargo test --workspace` executou 5.919 testes: 5.911 passaram e 8 falharam
fora do recorte P1455, em `eval::repr`, delimitadores matemáticos, align e
contagens HTML. Nenhuma das oito falhas toca os ficheiros ou comportamentos
alterados neste passo; por isso o gate global fica registado como não-verde,
sem ser apresentado como aprovação do produto.
