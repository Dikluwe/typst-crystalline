# Controles gerados e caches de sondas — 2026-11-14

Repositório: 6.06 → 5.38 GiB; mais 0.68 GiB liberados pelo mesmo `du -sx -B1 .`. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. Estado anterior, diffstat exato, hashes e horários nos recibos `before.json.gz` dos diretórios abaixo.

## Medição e redução

Os controles gerados de `box-lifecycle-2026-11-14` somavam 652.436.501 bytes em 5.026 arquivos. Tinham somente 94 payloads/permissões distintos. PNG/PDF/XML/SVG de `e2e/` e `controls.json` foram preservados em `lab/results/storage-baseline-controls-2026-11-14/controls.tar.zst`, de 2.097.516 bytes. A leitura integral verificou todos os caminhos e SHA-256 antes de retirar as cópias soltas, liberando aproximadamente 623 MiB.

Também foram removidos quatro targets antigos de sondas JPEG, três diretórios de bytecode Python e seis executáveis de probes regeneráveis. Total: 235 arquivos e aproximadamente 76 MiB. Fontes dos probes, CLI `lab/target/product/release/typst`, linter e oracles permaneceram preservados. Esses caches precisam ser recompilados se as sondas antigas forem repetidas.

## Baselines e preservação

Nenhum executável de baseline foi alterado ou compartilhado. Fontes/L0 da candidata, arquivo de fontes completo, manifesto, fixtures `.typ` e telemetria permanecem nos caminhos originais. A execução do benchmark atual não depende de restaurar os controles visuais antigos. Para reanalisar esses controles antigos, restaurar somente os caminhos necessários pelo helper abaixo.

Foram conferidos todos os arquivos versionados/novos fora da seleção contra o estado anterior, preservando alterações locais. O recibo completo de `box-launch-bias` e seu harness foram revalidados, assim como o SHA do oracle fixado. O binário lifecycle segue SHA `474f47e24c1d17324c2d65eec60b1138b923964d5d1e37f9ec99ad5fc34f80dc`. Hashes de baseline anterior, fontes, manifesto e telemetria estão no recibo final. Não houve rebuild, integração ao produto, commit ou alteração de histórico Git.

## Recuperação e teste

```
python3 lab/tools/cleanup-results/pack-baseline-controls.py --restore CAMINHO_OU_PREFIXO
```

Consultar caminhos em `lab/results/storage-baseline-controls-2026-11-14/index.jsonl.gz`. Conservar o índice e `controls.tar.zst`. O teste real restaurou `controls.json` e duas imagens de payload igual, conferindo hashes, permissões, mtime e inodes independentes. As três cópias foram removidas depois da verificação; os conteúdos permanecem no arquivo. Os controles descompactados preservam os bytes exatos e permitem conferir os hashes dos recibos anteriores.

Recibos: `lab/results/storage-baseline-controls-2026-11-14/` e `lab/results/storage-probe-caches-2026-11-14/`. Resultado combinado em `final.json` do primeiro. As declarações de rodadas anteriores de que os controles visuais de lifecycle ficavam soltos são superadas por esta movimentação; baselines executáveis e fixtures atuais continuam diretamente acessíveis.
