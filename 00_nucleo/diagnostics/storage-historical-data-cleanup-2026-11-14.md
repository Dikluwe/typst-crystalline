# Dados históricos e assets — 2026-11-14

Repositório: 8.72 → 6.06 GiB. Mais 2.66 GiB liberados, medidos por `du -sx -B1 .` após compactar os índices e retirar as cópias de teste. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Diffstat exato, estado anterior e hashes de proteção em `lab/results/storage-historical-data-2026-11-14/before.json.gz`.

## Investigação e redução

A contagem por inode único encontrou 2,93 GiB alocados em gzip, além de imagens e fontes binárias repetidas. Somar tamanhos por caminho superestimaria o uso por causa dos hardlinks anteriores.

20.464 gzip históricos de dados, logs e patches foram reunidos em `data.tar.zst` de 414.286.549 bytes, com 15.996 payloads distintos. Os conteúdos descompactados foram conferidos por SHA-256 antes de retirar as cópias originais. A compactação conjunta liberou 2,23 GiB nesta etapa, com snapshots de fontes e dados já arquivados nas rodadas anteriores mantidos em seus contêineres.

935 imagens e fontes binárias idênticas de resultados históricos passaram a compartilhar inodes; os bytes foram verificados. Essa etapa liberou cerca de 435 MiB. Fontes de código/L0 não foram compartilhados e campanhas atuais ficaram fora. Metadados anteriores constam no índice de ações; hardlinks podem compartilhar mtime. Separar cópias por `detach.py` antes de editar ou regenerar esses assets.

## Preservação e validação

Arquivos com alterações locais e arquivos novos não ignorados ficaram fora da seleção. Todos os arquivos versionados/novos fora das ações autorizadas foram conferidos contra os respectivos snapshots anteriores. As campanhas `box-launch-bias`, `box-launch-control`, `box-lifecycle` e o oracle resync permaneceram diretamente acessíveis e fora das duas seleções. Foram revalidados integralmente os hashes do recibo ativo de Box, seu harness e o SHA do oracle fixado. Arquivos `.tar.gz` de fontes ficaram fora do novo arquivamento.

Teste real: duas cópias históricas de dados e um patch foram restaurados em gzip independentes. Hashes descompactados, permissões, mtime e inodes independentes foram verificados. As cópias criadas para teste foram retiradas após a verificação. Logs e comandos em `restore-check.json` e `restore-real.log`.

## Recuperação

```
python3 lab/tools/cleanup-results/restore-historical-data.py CAMINHO_OU_PREFIXO
```

Consultar os caminhos em `lab/results/storage-historical-data-2026-11-14/index.jsonl.gz`. Conservar índice e `data.tar.zst` juntos. Os membros internos contêm payloads descompactados apesar dos nomes `.gz`; não extrair o tar inteiro como se fossem gzip prontos. O helper recria gzip válidos com conteúdo idêntico, embora cabeçalhos/hashes brutos possam mudar. Para aplicar um patch restaurado, usar depois `gzip -dk CAMINHO.patch.gz`.

Assets deduplicados mantêm os caminhos originais. Antes de alterá-los: `python3 lab/tools/cleanup-results/detach.py CAMINHO`. Índice em `lab/results/storage-assets-2026-11-14/actions.jsonl.gz`.

Recibos definitivos nos dois diretórios acima; `final.json` registra o ganho combinado. Sem rebuild, integração ao produto, commit ou reescrita do histórico Git. Os cerca de 2,2 GiB de objetos Git continuam preservados, junto com backups, fontes e baselines necessários.
