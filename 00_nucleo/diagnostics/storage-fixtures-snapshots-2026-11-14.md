# Fixtures e snapshots históricos — 2026-11-14

Repositório: 16.51 → 10.94 GiB. Mais 5.57 GiB liberados pelo mesmo `du -sx -B1 .`, já incluindo índices compactados e três exemplos restaurados para validação. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada; diffstat e hashes anteriores nos recibos `before.json.gz`.

## Fixtures

539.290 fixtures Typst ignoradas de campanhas históricas correspondiam a 6.827 conteúdos/permissões distintos. Seus 252.065.661 bytes lógicos tinham custo muito maior em blocos por serem arquivos pequenos. Foram arquivadas em `lab/results/storage-fixtures-2026-11-14/fixtures.tar.zst` (5.725.825 bytes), com referências internas para conteúdos repetidos. Caminhos, conteúdo, permissões e timestamps ficam preservados no índice. A retirada das cópias soltas só começou depois da leitura integral e conferência de todos os SHA-256 do arquivo concluído.

Não foram arquivadas fixtures versionadas, arquivos novos não ignorados, alterações locais ou fixtures das campanhas ativas de Box/oracle. Fontes e L0 produtivos permanecem nos caminhos originais. Fixtures históricas ignoradas precisam ser restauradas para executar as receitas antigas.

## Snapshots

1.396 snapshots de fontes em JSON gzip foram reunidos em `lab/results/storage-snapshots-2026-11-14/snapshots.tar.zst` (359.822.083 bytes). A compactação conjunta permite reutilizar trechos entre snapshots distintos; 1.112 payloads completos eram únicos. Todos os JSONs descompactados foram conferidos byte a byte por SHA-256. A troca só ocorreu depois da validação integral e da comparação com os blocos efetivamente ocupados pelos inodes originais, sem somar aliases como cópias independentes.

O contêiner gzip antigo não é preservado byte a byte: cabeçalhos e hash bruto poderão diferir na reconstrução. Os dados JSON exatos e metadados de caminho/permissão/timestamp estão preservados. O índice registra hash do contêiner original e dos dados. Os arquivos `.tar.gz` de fontes identificados separadamente (23, cerca de 329 MiB por caminho) não foram migrados nesta rodada.

## Verificação e uso

Todos os arquivos versionados/novos fora da seleção foram conferidos contra o snapshot anterior. Alterações locais ficaram fora da seleção. Foram revalidados integralmente o recibo ativo de Box, seu harness e o SHA do oracle fixado. O laboratório atual continua acessível diretamente. Não houve rebuild, integração ao produto ou commit.

Teste isolado: conteúdo exato, restauração em arquivos independentes e recusa de sobrescrita divergente. Teste real: duas fixtures do arquivo completo foram restauradas com SHA correto e inodes diferentes; um snapshot real voltou como gzip independente com JSON de hash correto. Os três exemplos ficaram disponíveis. Comandos, horários e logs em `restore-check.json`, `real-restore.log` e `restore-unit-check.log`.

Restauração seletiva:

```
python3 lab/tools/cleanup-results/pack-fixtures.py --restore CAMINHO_OU_PREFIXO
python3 lab/tools/cleanup-results/restore-snapshots.py CAMINHO_OU_PREFIXO
```

Consultar os caminhos nos respectivos `index.jsonl.gz`. As ferramentas restauram cópias independentes e recusam sobrescrever conteúdo divergente. Não extrair diretamente os membros do arquivo de snapshots como se já fossem gzip: eles contêm JSON sem compressão interna.

Um inventário inicial foi interrompido antes de exclusões por uma busca de conjuntos repetida no laço; foi otimizado e reiniciado com recibos separados. Recibos definitivos: `lab/results/storage-fixtures-2026-11-14/` e `lab/results/storage-snapshots-2026-11-14/`. O resultado final está em `final.json` do segundo diretório.
