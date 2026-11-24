# Limpeza de contêineres históricos — 2026-11-14

Repositório: 10.94 → 9.68 GiB. Mais 1.26 GiB liberados, medidos por `du -sx -B1 .`. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Estado anterior, diffstat e hashes de proteção em `lab/results/storage-containers-2026-11-14/before.json.gz`.

## Medição e ação

A inspeção de três contêineres mostrou executáveis ainda escondidos em arquivos históricos. `math-fields-r4-2026-10-23/evidence.tar.gz` tinha oito arquivos sem extensão somando 780.429.544 bytes, incluindo múltiplos builds. `text-size-length-2026-10-17/candidate-1.tar.gz` continha `binaries/typst` de 462.855.360 bytes. A remoção usou assinatura ELF, sem presumir que todo arquivo sem extensão era executável.

Os dois contêineres foram reempacotados em `.tar.zst`, retirando ELF e saídas PDF/SVG/XML/HTML/PPM antigas. Tamanhos: 322.519.313 → 47.692.529 bytes e 143.603.031 → 6.669.674 bytes. Todos os arquivos retidos foram verificados por SHA-256 antes de substituir os contêineres; links internos foram conferidos. Fontes, L0 e scripts internos permaneceram preservados. Hashes antigos/novos e lista das saídas retiradas constam nos recibos individuais.

499 patches históricos foram compactados sem perda, com 349 payloads distintos; os repetidos compartilham os gzip de consulta. Todos os payloads descompactados foram conferidos novamente. Patches atuais e arquivos com alterações locais ficaram fora da seleção. Restaurar um patch independente antes de uso com `gzip -dk CAMINHO.patch.gz`.

## Preservação e espaço restante

Todos os arquivos versionados/novos fora da seleção foram conferidos contra o estado anterior. Alterações locais ficaram preservadas. Revalidados integralmente o recibo ativo de Box, seu harness e o SHA do oracle fixado. Nenhum rebuild ou integração ao produto; sem commit.

O backup maior `source-copies.tar.zst` foi inspecionado e preservado: mistura fontes primários, snapshots de fontes e registros de campanhas. Retirar dados internos sem classificação mais fina poderia eliminar conteúdo de código. Não foi reduzido nesta rodada. Cerca de 2,2 GiB pertencem ao Git; `git count-objects -v` registrou zero garbage e zero prune-packable. Não se reescreveu o histórico.

Recibos definitivos em `lab/results/storage-containers-2026-11-14/`. As receitas antigas devem trocar os dois nomes `.tar.gz` pelos novos `.tar.zst` e usar `tar --zstd`. Saídas geradas removidas exigem reconstrução. As baselines necessárias ao laboratório atual continuam disponíveis diretamente.
