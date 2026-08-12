# Relatório — Passo 997: `=` (e o resto) ancorado à linha de baixo em `(n \ k) = …`

**Tipo**: verificação primeiro → bug confirmado → fix (fluxo contínuo,
ADR-0127 — correcção de âncora, sem contrato novo).
**Estado do código das medições**: HEAD `137d57b7f` (P996) + alterações
deste passo. Commit final: ver rodapé.

## Passo 1/2 — reprodução e medição própria

Repro mínimo `/tmp/p997.typ` (`$ (n \ k) = n!/(k!(n-k)!) $`), compilado com
o cristalino do HEAD e com o vanilla ratificado (`/usr/local/bin/typst`,
main `a51e02804`, string registada em P995):

- **Vanilla**: `(𝑛` na linha de cima (y 27.71); `𝑘)` e `=` na linha de
  baixo (ambos y 52.91) — o conteúdo após o grupo ancora na **última**
  linha.
- **Cristalino (pré-fix)**: `=` em y 34.81 — na linha de **cima** (com
  `(n`) — **bug confirmado no nosso binário**, reproduzindo o achado
  externo (não assumido por fé, per a nota de proveniência do passo).

## Passo 3 — causa (leitura, `file:line`)

O desemparelhamento de P996 emitia `MathSequence["(", n, Linebreak, k, ")"]`
como nó **aninhado** na sequência exterior — o `Linebreak` ficava invisível
para `partition_grid` (`math/layout/mod.rs:283`, que só lê o nível do
topo); o `=`/fracção eram concatenados **depois** da grelha interior, cuja
baseline é a da linha 1. No vanilla, `expand_multiline_fence`
(`ir/multiline.rs:56-107`) devolve os `RawMathItem::Linebreak` **no nível
da run** — a quebra divide a run inteira: linha 1 = `(n`, linha 2 =
`k) = n!/(…)`.

## Passo 4 — fix (TDD)

L0 primeiro (`engine/eval.md` §P997). RED confirmado:
`p997_linebreak_sobe_para_o_nivel_do_run` (eval: quebra aninhada) e
`p997_conteudo_apos_grupo_ancora_na_linha_de_baixo` (layout: `=` na linha
errada); a guarda P996 verde desde o início.

Implementação (`eval/math.rs`, `eval_math_content`): uma sub-`MathSequence`
com `Linebreak` ao nível do topo é **especialada** na sequência pai (itens
entram em linha — a quebra sobe para o run); sem `Linebreak`, aninhamento
preservado. ~12 linhas.

GREEN: **5828 testes, 0 falhas** (5825 + 3 novos). Lint: 0 violations (só
V7 órfão pré-existente).

## Passo 5 — Revalidação

- Repro mínimo: `(𝑛` em cima; `𝑘)`, `=` e a fracção na linha de baixo —
  estrutura idêntica ao vanilla (mesma ancoragem; a distância entre linhas
  difere ~5pt — aproximação de leading já registada em P923b, fora de
  âmbito).
- Canónico secção 7 completa: render confere com o vanilla
  (`temp/revisao/s7-c997-1.png`); `binom` e o resto da secção inalterados.
- Benchmark canónico: ratios 0.958–1.012, **média 0.993 — sem regressão**.

## Nota de escopo

O achado 4/secção 36 (transformações `rotate`/`scale`) continua pendente,
próximo passo separado, per indicação do dono.

---
