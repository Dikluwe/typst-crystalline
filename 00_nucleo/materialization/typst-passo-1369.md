# Passo 1369 — medição bilateral do vetor efetivo S02

## Objetivo

Consumir o observador P1368 numa matriz bilateral real e produzir um novo
veredito focal para precedência de PDF tagueado, sem alterar o produto.

## Entradas congeladas

- vanila ratificado: `lab/typst-original/target/release/typst`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino: `target/release/typst`, SHA-256
  `60851811dca31074eca38f5d4e397ce04a89eccb7bf49c9d6e28f6759ba761f9`;
- `pdfinfo` 26.05.0, SHA-256
  `fee70ade670fb025343aca2b5c3a2aacacb8ed9edce1b716b233b5de924b6bf5`;
- observador P1368, SHA-256
  `fbd1a49c5389e0d469001083852216fd40c6bd223c8c14356ccdcdf8e96b0a06`;
- contrato P1368, SHA-256
  `c40baa32bb636185946f5fab0c41784fc0d11cbdeb6127570ab3c6c7c7356164`.

## Matriz e observável

Executar os três casos `top_level`, `document_over_cli` e `cli_only` para
vanila e cristalino nos perfis `default`, `html`, `a11y-extras` e
`html+a11y`: 24 compilações e, somente após cada sucesso, no máximo 24 chamadas
`pdfinfo`. Cada tripla alimenta `classify_s02`; `Unknown` nunca vira match.

O resultado bilateral é `MATCH` por perfil somente quando os dois produtos
obtêm `Preserved` e o mesmo vetor canônico `(true,false,true)`. Resultado
incompleto é `Unknown`; vetores completos diferentes são `DIFFERENCE`.

## Segurança de execução

O runner usa um processo por vez, diretório temporário isolado por caso,
timeout de 45 s, `RLIMIT_AS=2 GiB`, `RLIMIT_FSIZE=16 MiB`, `RLIMIT_CPU=45 s`,
canais em arquivos temporários e leitura máxima de 1 MiB por canal. Não usa
`PIPE`, paralelismo, Cargo, watcher, rede deliberada ou suíte ampla.

Falha de limite, sinal, timeout, output ausente ou observador inválido produz
`Unknown` com reason code. O runner não repete automaticamente uma falha.

## L0, regime e alegação

Somente `00_nucleo/diagnosticos/` é alterado; nenhum Prompt L0 nem L1–L4 muda.
Protocolo completo executado sem atestação de isolamento, porque os papéis
compartilham workspace e autoridade desta sessão. O veredito cobre apenas S02.

