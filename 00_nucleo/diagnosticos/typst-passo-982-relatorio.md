# Relatório — Passo 982 (oráculo: residual TJ e q/Q vs cm)

**Data:** 2026-08-05
**Proveniência**: HEAD no início = `2d884829d` (P981). Passo de
investigação sobre o oráculo — **nenhum código alterado** (as duas partes
concluíram-se por medição), logo sem benchmark novo: o binário é
idêntico ao de P981 (a única escrita foi a secção §P982 do L0
`oracle.md` + resselo de hash em comentário de header). Suíte corrida por
disciplina em P981: 5768 testes, 0 falhas; o resselo não muda código.

## Parte A — o residual TJ é estrutural (não é tolerância)

Medição sobre a saída do oráculo (`/tmp/p980-oracle.pdf`, doc de 30
secções): 263 arrays `TJ` com ajuste real; **só 2 de 663 ajustes têm
|a| ≤ 3 milésimos de em** — não é ruído de arredondamento. A
distribuição dos valores: −278 (×139), −222 (×105), −348 (×83), −167
(×57), −500 (×23)… — os três primeiros são exactamente os espaços de
classe math a 11pt: **THICK (5/18em = 278‰), MEDIUM (2/9em = 222‰),
THIN (1/6em = 167‰)**.

**Causa confirmada** (a terceira hipótese do passo — diferença genuína de
codificação): o cristalino escreve o espaçamento de classe como ajustes
TJ dentro de um run de texto; o vanilla nunca o evita — **codifica-o
como posições de blocos separados** (cada fragmento math é um TextItem
com posição absoluta própria; daí os seus 1919 blocos BT no documento
contra os nossos 1293 pós-P979). Colapsar estes TJ para Tj **mudaria
posições** — proibido; a tolerância pedida pelo passo (tratar quase-zero
como zero) não se aplica porque os valores não são quase-zero.

**Caminho de convergência adicional** (registado, não implementado — é
decisão de desenho para o dono): uma transformação de *split* no oráculo
que parta runs nos ajustes reais e re-posicione cada pedaço com Tm
absoluto, à imagem do vanilla. Exige os avanços nominais da fonte
(o stream sozinho não os tem — o oracle teria de receber os mapas
`glyph_to_nominal` do builder). Não é trabalho de tolerância; é um
segundo emissor posicional dentro do oráculo.

## Parte B — q/Q sem cm: explicado, sem código

Medição: cristalino q=1361 vs cm=1293 — delta **68**, exactamente o
número de traços vectoriais (`l S` = 68) e de padrões `q … w … m` sem
cm (≈68). O braço `FrameItem::Line` (`stream.rs`) emite
`q {w} w … m … l S Q`: o `q/Q` guarda o estado gráfico contra a mudança
de line width; o `cm` não é necessário porque as coordenadas do traço já
são absolutas (não há transformação a aplicar). Vanilla: delta 4
(1992/1988) porque o krilla embrulha quase tudo em `q/cm/Q`
uniformemente — diferença de estilo de emissão, não de semântica.

**Veredicto: comportamento correcto e intencional — nada a corrigir.**

## Resultado

- Parte A: causa do residual TJ confirmada como diferença genuína de
  codificação (espaços de classe em ajustes TJ vs blocos posicionados),
  com os valores exactos medidos; a proporção 79.7% **não** deve subir
  por colapso — só por split posicional (decisão registada para o dono).
- Parte B: descasamento q/Q vs cm explicado (68 = traços vectoriais com
  guarda de estado, sem transformação) — sem código.
- Saída principal inalterada (nenhum código tocado; prova de P980
  mantém-se válida — o binário é o mesmo).
