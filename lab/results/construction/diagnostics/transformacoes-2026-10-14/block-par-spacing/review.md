# Margens automáticas de Block — correção e limites

Estado: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt,
working tree não commitado. Esta etapa corrige resolução de margem e
admissão da baseline; o objetivo geral continua aberto.

## Evidência anterior e decisão

`before-edit.json.gz` conserva os quatro consumers e owners antes da edição,
com SHA-256, UTC, status e diffstat. Os lotes irmãos `block-par-spacing-before`
e `block-par-extra-before` conservam fontes, comandos, saídas e hashes de
toda a árvore produtiva/L0 no instante de cada medição. O primeiro foi
medido em 2026-10-14T20:44:09Z–20:44:32Z. Vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8`, árvore e receita P1489,
binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt anterior SHA-256 `a353e810dd526a9e974145ccebfa811600340afd4d714743a518d04638136ee4`.

Block usava sempre 1.2em como margem automática, ignorando par.spacing.
Com fonte10/top10/bottom0, PRE + Block20 resultava em altura62, contra
50/55/70 para spacing0/5/2em. Após Block20 below0, a linha B perdia seu
próprio top10: página40 contra50. Dois Blocks1fr reservavam também a
margem fixa12, deslocando B em relação às margens especificadas.

A fonte pinada `typst-layout/src/flow/collect.rs:253–258` confirma resolução
Auto por ParElem::spacing. Isso é comportamento da linguagem e geometria
observável. A correção interna segue ADR-0127 em fluxo contínuo; nenhuma
API pública, fase, modo ou flag nova.

## Alteração

- Block resolve acima/abaixo explícitos, depois spacing explícito, depois
  par.spacing da chain exterior e finalmente o default 1.2em. Captura o
  fallback antes do corpo para conservar o escopo exterior.
- A reserva fracionária usa o mesmo fallback, legitimado pelo núcleo
  `layout/block-spacing-resolution.toml`, com três owners consumidores.
- Cursor usa fim causal + margem + top-edge quando a cadeia tem margem zero
  e uma testemunha válida. A ausência dessa testemunha conserva o fallback;
  não infere altura pela tinta nem reconhece Block no despachante genérico.
- Três testes fonte→frame cobrem margens0/5/2em, overrides zero, alturas,
  posições e distribuição de dois Blocks1fr. `stage.diff` isola esta etapa
  das alterações anteriores presentes na working tree.

## Verificação

Todas as execuções abaixo incluem HEAD/status/diffstat/UTC e hashes de fontes
em `implementation/block-par-spacing-*-{pre,result}.json.gz`. Source guards
passaram; não houve edição produtiva/L0 durante cada execução.

- RED 20:48:49Z–20:49:23Z: três testes falharam pelas causas previstas.
- GREEN 20:49:51Z–20:50:49Z: os três passaram.
- Layout final 20:51:56Z–20:52:31Z: **1.207 aprovados, uma falha, três
  ignorados**. A única falha é P1381, já presente antes desta etapa:
  `p1381_primeiro_fragmento_maior_permanece_fail_closed`. Seu esperado não foi
  alterado; paginação e posição precisam de decisão conjunta com a medição.
- Build, cargo fmt, linhagem estrita V1/V5/V15/V26 e linhagem reversa
  passaram (970 consumers, zero erros). Linter amplo: zero erros, 380 avisos,
  1.664 notas. Isso não significa CI sem avisos. Clippy e suíte completa
  não foram repetidos nesta etapa; CI/release continuam sem certificação.
- PDF final 20:52:56Z–20:53:19Z: **23 de 24 documentos** coincidem em páginas,
  dimensões e caixas de texto até0.0001pt. Os18 controles de margem passam;
  só scope19 mantém delta9.6pt. O lote extra confirma os três controles de
  dois Blocks1fr; mantém três caudas com paginação incorreta e três erros
  de constructor para margem negativa. Não contar erros como paridade.
- SVG/PNG 20:53:37Z–20:54:22Z: 24 documentos,96 compilações. SVG:23 imagens
  idênticas; PNG:17 idênticas e seis com somente cinco pixels variando um
  nível de canal. Scope19 tem dimensão diferente nos dois formatos.
  As pequenas diferenças raster não motivam mudança de linguagem.

Binário Tekt final SHA-256
`a213bba9ff88da628489d80ae72f207aeed5b5977ed4e89c8e70ef83fe5dfecd`.
`summary.json` enumera os casos e deltas; outputs e scripts são incorporados
no arquivo `evidence.json.gz`, reaberto e conferido byte a byte. O recibo
`archive-receipt.json` identifica hash e quantidade de entradas.

## Pendências concretas

1. Escopo lexical: um único wrapper18 em Lead More usa margem21.6; dois
   wrappers18 adjacentes continuam usando o escopo exterior. Consenso de
   tamanhos de runs não resolve isso. P1381/prefixos fracionários dependem
   da mesma certificação de escopo e reserva antes do bloco.
2. Texto após Block1fr ainda passa para outra página nos três controles;
   investigar a reserva da cauda e admissão na borda, sem alterar esperados
   para esconder a perda do espaço da primeira página.
3. `compiler/stdlib/layout/block.rs:210` rejeita margem negativa que a fonte
   pinada aceita. Resolver com o owner L0 do constructor e testes próprios.
4. Força de colapso entre margens explícitas e automáticas, outset na reserva
   fracionária e fragmentação de Block continuam abertos. O novo núcleo
   declara expressamente esses limites.
5. Persistem as frentes anteriores de Align Place-only, fechamento textual,
   transformações, exportadores, avisos e CI/release. Nenhuma publicação feita.
