# Limpeza do laboratório — 2026-11-14

Repositório: 84.65 → 43.17 GiB; redução de 41.48 GiB pelo mesmo `du -sx -B1 .`. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Estado anterior, diffstat exato, hashes, horários e caminhos removidos em `lab/results/storage-2026-11-14/`.

Por autorização explícita do usuário, foram excluídos 583 ELFs históricos de relatórios anteriores a 2026-11-12 e o arquivo de retenção de builds de outubro. Não foram arquivados: receitas antigas podem exigir recompilação. Relatórios e medições brutas permanecem disponíveis. O arquivo de cópias de fontes de outubro foi preservado, pois não contém apenas executáveis.

Conferidos integralmente os hashes do recibo ativo de Box, seu harness, o oracle fixado e os arquivos de trabalho previamente modificados. Fonte produtiva, L0 e Git intactos; README do laboratório atualizado com apontadores. Nenhum rebuild foi necessário para uma limpeza de artefatos.

Ferramenta e escopo: `lab/tools/cleanup-results/README.md`. O recibo intermediário usa a chave legada `archived_files` para a contagem de exclusões; `objects: 0` confirma que não houve armazenamento de executáveis. O resultado definitivo está em `final.json`.
