# Block — alocação e fronteiras, correção em andamento

Estado medido: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree
não commitado. Cada resultado preserva UTC, status, `git diff HEAD --stat`,
hashes das fontes e comando. Binário Tekt final SHA-256
`8075378030a4e47fb2216c9b884ec9f7542dc3eb7476efcca672631af2340784`.
Referência vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
árvore e receita permanecem as ratificadas em P1489.

## Correções e critérios

Block usa a própria alocação vertical, incluindo insets e altura explícita,
para backing, raiz de pintura e fim do fluxo. O corpo inicia uma cadeia
local. Outset expande pintura/recorte sem mover o corpo nem antecipar quebra.
Os testes iniciais RED→GREEN estão em `implementation/block-height-*`.

A continuação corrigiu a duplicação de top-edge após pagebreak e prefixos
fracionários, e passou a usar o fim causal após linha fechada/espaço forte.
A reserva fixa do prefixo foi ajustada antes da distribuição dos pesos;
compensar só a origem deixava a cauda curta. A primeira tentativa genérica
encurtava também floats em subframes de origem local, perdendo marcador no
checkpoint. O refutador foi conservado; a conversão agora respeita a origem
local do caller. Sem assinatura pública, nova fase ou default: ADR-0127 em
fluxo contínuo, L0 antes de código, resselo restrito aos owners.

P248 continha uma expectativa refutada: depois de v620 em A4 restam cerca
de80.157pt físicos. O teste agora compara Block80 e Block90 com outset0/30:
o primeiro cabe e o segundo quebra, independentemente de outset. Não copiar
um erro anterior do Tekt para satisfazer o teste, nem copiar um defeito do
vanilla apenas por divergir. A aceitação aqui é conteúdo, posição e paginação.

## Evidência final deste lote

`implementation/block-boundary-layout-v3-{pre,result}.json.gz` e `.log`:
2026-10-14T20:27:27.970982Z a20:28:05.445655Z,
**1198 passaram, 2 falharam, 3 ignorados** no filtro workspace
`compiler::layout`; não é a suíte completa. Os dois testes novos de
pagebreak e linha/espaço foram RED e agora são GREEN; P1292 voltou a passar.

Build offline/locked, fmt de todo workspace, V1/V5/V15/V26 estritos e
linhagem reversa passaram em `implementation/block-boundary-final-*`.
A reversa inspecionou970 consumers e encontrou zero erros. Guards de fontes
verdadeiros. `git diff --check` também passou. Não reexecutamos a suíte
inteira, Clippy ou o inventário arquitetural amplo neste estado; nenhum
crédito de CI/release verde é derivado destes filtros.

`block-boundaries-final/results.json.gz`:19 documentos bilaterais em PDF,
com páginas e bboxes textuais extraídos.17 coincidem dentro de0.0001pt,
incluindo origem após pagebreak, linhas fechadas, espaço forte, limites de
outset, checkpoint com todos os marcadores e divisão1fr/2fr. Os dois
restantes são o prefixo18pt isolado e Align(horizon). Ausência de palavras
não certifica sozinha a geometria de formas; essa obrigação também possui
os testes de backing e o corpus SVG/PNG.

`block-height-boundary-final/results.json.gz`:45 documentos em SVG/PNG
para cada binário,180 execuções, sem erro ou diferença de dimensão. Os45
SVGs rasterizados coincidem. Seis PNGs possuem diferenças de borda/cobertura,
conservadas em `boundary-summary.json`; não são prova isolada de erro de
layout nem de superioridade de algum rasterizador. Esta medição ocorreu
entre20:28:20.379499Z e20:28:57.351429Z, com guard de fontes verdadeiro.

A sonda exploratória anterior `block-height-extra` comparou páginas distintas
nos casos pagebreak SVG/PNG por causa do suporte incompleto a `{p}` no CLI
Tekt. Esses deltas NÃO são aceitos como evidência de layout. A comparação
PDF final lê todas as páginas e substitui essa inferência.

## Pendências que impedem encerrar

- P898: o fundo terminal do Block anterior ainda prevalece sobre Align
  vertical. No documento de referência Block200+short, página Tekt230pt
  contra250.438pt. A matriz `block-align-boundary-before` amplia o problema
  para Top/Horizon/Bottom, texto e formas após bloco ou linha. Fonte de
  investigação: `placement.rs` restringe gap/publicação do fim a
  `flow_aligned`, e o certificado textual não trata wrappers Source.
  Isso é hipótese causal a verificar, não uma correção implementada.
- P1381: o antigo teste exigia duas páginas para um prefixo18pt isolado;
  ambos os binários agora produzem uma, mas One ainda está8.4pt acima do
  pin. Trocar apenas a contagem esperada ocultaria o erro de espaçamento.
- Fragmentação de Block, inset/outset e paint_root por região continuam
  sem certificação; a rota legada permanece. Medida textual, dimensões
  automáticas de formas, RTL, transformações, cores/exportadores e os
  demais débitos do objetivo geral também permanecem abertos.
- CI/release não concluídos; não houve publicação, commit ou push.

## Preservação

Arquivo `boundary-evidence.json.gz`:2321 entradas com caminho, SHA-256 e
bytes base64, reaberto e comparado com os arquivos em disco. Inclui fontes
antes/depois, scripts, logs, resultados, documentos e renders, mesmo quando
ignorados pelo Git. SHA-256:
`0ed0d0364c8b70b9d1f97bdae216317fc9f7da072d711e0de3908da80ac841ba`.
Receipt: `boundary-archive-receipt.json`, UTC20:29:20.031102Z.
