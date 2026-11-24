# Recompressão dos objetos Git — sessão 2026-11-14/15

Repositório: 5.38 → 5.24 GiB; ganho líquido adicional de 143.04 MiB (0.14 GiB). Mesma medição `du -sx -B1 .`, incluindo novos recibos. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Estado anterior, diffstat e hashes em `lab/results/storage-git-packs-2026-11-14/before.json.gz`.

## Evidência e operação

O Git ocupava cerca de 2,2 GiB em 18 packs e objetos soltos. `git fsck --full --strict` passou antes da operação. Foram inventariados 226.272 IDs de objetos, com seus tipos e tamanhos; também foram capturados refs, reflogs, índice, demais metadados Git fora dos objetos e hashes de arquivos versionados/novos de trabalho.

Uma cópia independente de recuperação dos objetos foi criada em `/tmp/tekt-git-objects-before-repack-20261114` e conferida por hash antes da operação. Sua criação exigiu um diretório inexistente, protegendo outros temporários.

Comando executado:

```
git -c pack.windowMemory=512m -c pack.threads=1 repack \
  -a -d -f -n --keep-unreachable --window=50 --depth=50 --threads=1
```

A operação apenas recompactou o armazenamento: todos os objetos, inclusive não alcançáveis, foram mantidos. O Git ficou com um pack. Nenhum commit foi reescrito; não houve expiração de reflogs, reset, checkout ou mudança de branch. A operação de escrita em `.git` foi autorizada pela revisão automática no contexto da limpeza solicitada.

## Validação

O inventário completo de IDs/tipos/tamanhos antes/depois é idêntico. Refs, reflogs, índice e outros metadados Git permaneceram byte a byte iguais, assim como todos os arquivos de trabalho protegidos. `git fsck --full --strict` passou depois, incluindo a conferência de integridade dos objetos. Objetos dangling reportados nos logs permanecem preservados; não são objetos descartados.

O recibo ativo de Box, seu harness e o SHA do oracle fixado foram revalidados integralmente. Fontes, L0, alterações locais e baselines atuais ficaram intactos. Não houve rebuild ou integração ao produto. A cópia temporária de recuperação foi retirada somente depois das verificações completas, portanto o ganho não foi obtido apenas transferindo os mesmos packs para `/tmp`.

Os índices dos backups também foram comparados por hashes de payload durante a investigação: há repetição entre alguns conjuntos, mas o volume descompactado repetido não mede o ganho possível em contêineres já comprimidos. Esses arquivos foram preservados nesta rodada. O ganho aqui declarado vem somente da recompressão Git, descontados os novos recibos.

Recibos: `lab/results/storage-git-packs-2026-11-14/`, incluindo `recovery-manifest.json.gz`, `fsck-before.log`, `fsck.log`, `receipt.json` e `final.json`. A árvore local continua contendo as alterações e exclusões autorizadas das limpezas anteriores, sem commit.
