# Terceira limpeza — artefatos versionados

Em 2026-11-14, por autorização explícita, a limpeza passou a incluir artefatos gerados versionados e sem alterações locais. Repositório: 24.45 → 18.59 GiB. Redução adicional: 5.86 GiB por `du -sx -B1 .`, incluindo os novos recibos.

HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Estado anterior e diffstat exato estão em `lab/results/storage-tracked-2026-11-14/before.json.gz`. As 5.419 remoções versionadas são intencionais e continuam apenas no working tree. Foram compactados dados brutos de campanhas antigas e removidos executáveis superados e saídas renderizadas. Também foram excluídas 4.169 imagens JPEG geradas em comparações/qualificações antigas; fixtures e assets primários ficaram preservados.

O backup de outubro passou de 2.402.053.440 para 1.964.800.396 bytes, mantendo 202.082 arquivos cujos bytes foram conferidos por SHA-256. Foram omitidas 32 entradas ELF/build. Novo caminho: `lab/results/project-cleanup-2026-10-22/source-copies.tar.zst`. O contêiner antigo foi retirado após validar o novo.

Fontes, L0, alterações locais, fixtures, oracle e baselines ativas foram preservados. Todos os arquivos versionados/novos que ficaram fora da seleção foram conferidos contra o snapshot anterior; também foram revalidados todos os hashes do recibo ativo de Box e seu harness e o SHA do oracle fixado. Sem rebuild ou integração ao produto.

O histórico Git continua intacto e ainda contém versões antigas; esta limpeza não reescreve commits. Os dados históricos agora em `.gz` sob `lab/results/` são retidos localmente, mas ignorados pelo Git. Um commit comum das exclusões não inclui automaticamente esses dados. Receitas antigas exigem descompactação ou regeneração das saídas.

Índices de ações, hashes, procedimentos e recibos em `lab/results/storage-tracked-2026-11-14/`; instruções em `lab/tools/cleanup-results/README.md`. O tamanho restante inclui histórico Git, snapshots de fontes, fixtures e medições compactadas, além do laboratório atual.
