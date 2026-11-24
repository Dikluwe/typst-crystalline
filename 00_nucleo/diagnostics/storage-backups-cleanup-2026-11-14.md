# Backup de outubro — compactação conjunta

Em 2026-11-14, o repositório passou de 9.68 para 8.72 GiB: mais 0.97 GiB liberados, medidos por `du -sx -B1 .`, incluindo os índices e recibos. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Estado anterior, diffstat e hashes em `lab/results/storage-backups-2026-11-14/before.json.gz`.

## Investigação e resultado

O backup tinha cerca de 1,45 GB de gzip independentes, além de fontes e registros. Contêineres já comprimidos isoladamente impediam aproveitar repetições entre seus dados. A nova embalagem descompacta esses payloads e os comprime em conjunto, sem eliminar os conteúdos.

`lab/results/project-cleanup-2026-10-22/source-copies.tar.zst` foi substituído por `source-copies.packed.tar.zst`: 1.487.134.042 → 436.310.806 bytes. Todos os 202.082 arquivos permanecem representados, com 30.741 payloads distintos; 3.445 payloads gzip foram descompactados para a compactação conjunta. Também foram preservados e conferidos os metadados de 16.501 diretórios e um symlink.

Cada arquivo não gzip conserva seus bytes originais, incluindo fontes e L0. Para gzip, conserva-se exatamente o conteúdo descompactado, não o cabeçalho/stream comprimido original. O índice registra ambos os hashes. A leitura integral do novo arquivo verificou os caminhos, os hashes de todos os payloads, modos e metadados não regulares antes de retirar o contêiner antigo. O novo SHA-256 está em `archive-receipt.json`.

## Restauração e controles

A embalagem usa PAX `tekt.encoding=gzip`; os membros com essa marca contêm dados descompactados apesar do nome antigo terminar em `.gz`. Não extrair o backup inteiro como se fosse um checkout utilizável. Restaurar seletivamente por:

```
python3 lab/tools/cleanup-results/repack-backup.py --restore CAMINHO_OU_PREFIXO
```

Consultar caminhos no índice `lab/results/storage-backups-2026-11-14/index.jsonl.gz` e conservar esse índice com o contêiner. O helper reconstrói gzip válidos e arquivos independentes; fontes repetidos não compartilham inodes na restauração. Metadados de diretórios e symlink continuam no tar; o helper se destina aos arquivos indexados.

Teste real: duas cópias de um fonte Rust, um L0 e um gzip foram restaurados, verificando conteúdos, permissões, mtime e inodes independentes. As cópias de teste, criadas em caminhos previamente ausentes, foram removidas depois de conferir seus hashes. Comandos e resultados em `restore-check.json` e `restore-real.log`.

Todos os arquivos versionados e novos não ignorados foram conferidos contra o estado inicial; alterações locais permaneceram intactas. O recibo ativo de Box e seu harness foram revalidados integralmente, assim como o SHA do oracle fixado. Não houve rebuild, mudança no produto, commit ou reescrita do histórico Git. As baselines atuais continuam diretamente acessíveis.

Recibos definitivos em `lab/results/storage-backups-2026-11-14/`, incluindo `final.json`. Os hashes e os nomes de embalagens anteriores ficam como registros históricos, superados pelo novo contêiner e seu índice.
