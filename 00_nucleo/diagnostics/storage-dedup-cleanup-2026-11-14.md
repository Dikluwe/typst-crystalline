# Deduplicação do laboratório — 2026-11-14

Repositório: 18.59 → 16.50 GiB. Mais 2.08 GiB liberados, pelo mesmo `du -sx -B1 .`. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Estado anterior, diffstat exato e hashes em `lab/results/storage-dedup-2026-11-14/before.json.gz`; horários de cada troca em `actions.jsonl.gz`.

Foram deduplicados 60113 caminhos de resultados históricos. Arquivos normais exigiram bytes idênticos; gzip exigiu conteúdo descompactado idêntico, com SHA-256. Os caminhos foram mantidos por hardlinks. Fontes editáveis, patches e fixtures Typst não entraram no compartilhamento, assim como as baselines e campanhas ativas. Todos os arquivos versionados/novos fora da seleção foram conferidos contra o snapshot anterior; arquivos previamente modificados ficaram fora da seleção.

O backup `lab/results/project-cleanup-2026-10-22/source-copies.tar.zst` foi deduplicado internamente: 160.668 entradas repetidas passaram a referências tar; os mesmos 202.082 arquivos/caminhos foram verificados por SHA-256 antes/depois. Embalagem: 1.964.800.396 → 1.487.134.042 bytes. A substituição ocorreu somente depois da leitura integral e validação do novo arquivo. Recibo: `source-archive-dedup.json`.

Revalidados integralmente o recibo ativo de Box e seu harness, e o SHA do oracle fixado. Não houve rebuild, integração, commit ou alteração do histórico Git.

## Uso dos resultados preservados

Hardlinks compartilham dados e metadados de inode. Os resultados históricos são para consulta; antes de editar/regenerar caminhos, executar `python3 lab/tools/cleanup-results/detach.py CAMINHO`. A ferramenta foi verificada com dois hardlinks descartáveis: após separar e editar um deles, o outro manteve seu conteúdo. Fontes extraídos do backup também devem passar por detach antes de servirem como checkout editável.

No caso gzip, cabeçalhos e hash bruto podem mudar; o conteúdo descompactado permanece idêntico. Índices registram hashes anteriores/posteriores e mtime anterior. Isso não conserva todos os bytes da embalagem antiga. O hash novo do backup está no recibo; hashes de embalagens antigas foram superados, sem reescrever relatórios históricos.

O restante inclui histórico Git, snapshots de fontes/medidas compactados e muitas fixtures pequenas. Essas fixtures foram mantidas nos caminhos originais; removê-las ou compartilhar fontes editáveis exigiria um tratamento diferente. O tamanho por caminho de grupos com hardlinks não deve ser somado como se cada caminho ocupasse blocos independentes. A medida final usa o diretório inteiro para contar cada inode uma única vez.
