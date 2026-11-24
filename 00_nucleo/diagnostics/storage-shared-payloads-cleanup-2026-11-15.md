# Payloads compartilhados — 2026-11-15

Repositório: 5.24 → 4.97 GiB. Mais 272.98 MiB (0.27 GiB) liberados pelo mesmo `du -sx -B1 .`, incluindo novos índices, metadados, ferramentas e recibos. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Estado anterior e diffstat exato em `lab/results/storage-shared-migration-2026-11-15/before.json.gz`.

## Medição e ação

Os três contêineres de backup, snapshots e dados históricos somavam 1.210.419.438 bytes. Havia repetição entre eles apesar da deduplicação interna de cada um. Foram reunidos em um armazenamento compartilhado por SHA-256 em `lab/results/storage-shared-payloads-2026-11-15/`, preservando 202.082 caminhos de backup, 1.396 de snapshots e 20.464 de dados: 223.942 arquivos, com 43.549 payloads distintos.

Cada payload foi lido do arquivo original e conferido contra o índice do respectivo conjunto. Depois, o novo arquivo foi lido integralmente e todos os conteúdos foram verificados novamente por SHA-256. Metadados TarInfo/PAX de todos os membros foram preservados em `*-headers.jsonl.gz`, inclusive 16.501 diretórios e um symlink do backup. Os índices anteriores permaneceram intactos e são referenciados pelo catálogo.

A substituição só ocorreu após medir que o armazenamento verificado era menor. Foram criadas cópias independentes dos três arquivos originais em um diretório temporário exclusivo e conferidas por hash antes da retirada dos originais. A recuperação temporária só foi removida após repetir os testes com os originais ausentes e revalidar as baselines. O ganho não corresponde a transferência de arquivos para fora do repositório.

## Restauração e testes

Os três comandos existentes foram atualizados para detectar o catálogo, mantendo argumentos e seleção por caminho/prefixo. As versões anteriores desses helpers ficaram no recibo, e suas alterações foram explicitamente registradas. Nenhum fonte produtivo ou L0 foi modificado. O novo helper verifica hashes dos índices, metadados, contêiner e payload selecionado antes de gravar arquivos independentes; recusa sobrescrever conteúdo divergente.

Teste isolado: deduplicação entre conjuntos, alias, restauração gzip, cópias independentes, recusa de sobrescrita e detecção de corrupção no contêiner. Testes reais antes/depois da retirada dos originais: dois fontes Rust repetidos, um L0, um gzip do backup com payload também presente nos dados, um snapshot e a cópia correspondente nos dados. Foram verificados conteúdos, modos, mtime e inodes independentes. As cópias de teste foram retiradas. Logs e comandos em `store-tests.log`, `restore-check*.json` e `real-restores*.log`.

Os conteúdos não gzip permanecem byte a byte idênticos. Para gzip, permanece idêntico o conteúdo descompactado, conforme o formato já utilizado antes desta migração; cabeçalhos/streams gzip não têm garantia de bytes iguais. Os metadados de diretórios e links continuam nos registros de cabeçalhos; o helper restaura arquivos indexados em cópias independentes.

## Preservação e operação

Todos os arquivos versionados/novos fora dos três helpers de restauração, do teste novo desta tarefa e da documentação de limpeza explicitamente atualizados foram conferidos contra o estado inicial. Alterações locais e trabalho produtivo ficaram intactos. O recibo ativo de Box e seu harness foram revalidados integralmente, assim como o SHA do oracle fixado. As baselines atuais, fixtures e controles de Box não foram migrados. Não houve alteração do Git, rebuild, integração ao produto ou commit.

Conservar todo `storage-shared-payloads-2026-11-15/` e os índices referenciados em `catalog.json`. Os caminhos `archive` do catálogo identificam contêineres antigos retirados; `payloads.tar.zst` é o arquivo de dados atual. As instruções anteriores de abrir os três tar diretamente são superadas pelo catálogo. Os comandos de restauração continuam:

```
python3 lab/tools/cleanup-results/repack-backup.py --restore CAMINHO_OU_PREFIXO
python3 lab/tools/cleanup-results/restore-snapshots.py CAMINHO_OU_PREFIXO
python3 lab/tools/cleanup-results/restore-historical-data.py CAMINHO_OU_PREFIXO
```

Recibos definitivos em `lab/results/storage-shared-migration-2026-11-15/`; resultado combinado em `final.json`. Não há cópias de recuperação temporárias retidas fora do repositório.
