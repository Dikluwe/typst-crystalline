# Testes observáveis na CI

Retirados sete casos com `assert!(true)` e um sentinel redundante que
ainda aceitava texto descartado em grupos. Nenhuma asserção dos testes
restantes foi alterada, nenhum teste foi ignorado para contornar falhas e
nenhuma implementação produtiva mudou.

## Proveniência e decisão

Branch Tekt, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Medição inicial em `2026-10-14T00:58:05.319634+00:00`. `before.json.gz` guarda fonte e
L0 completos, trechos retirados com linhas, status e `git diff HEAD --stat`.
A comparação mecânica em `preservation-proof.json` exclui somente os oito
blocos retirados, cabeçalho de linhagem e linhas vazias. Não é uma prova
de paridade. O L0 foi atualizado antes do código e resselo foi restrito
a `infra/export/tests/mod`.

Casos retirados:

- `p273_6_pattern_debt37_replicado_n3`.
- `p273_7_template_passo_replicado_literal_n1`.
- `p273_10_l3_only_parent_bbox_inaugural_n1`.
- `p273_12_dedup_arc_as_ptr_resources_n3_smoke`.
- `p273_13_l3_only_parent_bbox_n2_smoke`.
- `p279_render_real_groups_n2_smoke`.
- `p281_render_real_groups_n3_smoke`.
- `p279_text_em_group_continua_stub_documentado`.

Os sete primeiros não executavam o sistema: seus comentários narravam
reaplicações históricas de padrões. Preservamos esse histórico no snapshot.
O último verificava apenas `cm`; o teste existente
`p281_text_em_group_helvetica` verifica `(hello) Tj` e `cm`. Também permanecem
os casos CIDFont, multifont, glyph, linha e grupo aninhado.
Os casos p273_6 e p273_7 de bbox em Block/Boxed, p273_10 de bbox de Group,
p273_12 de deduplicação/Coords, p273_13 de emissão de patterns e p279 de
Image/XObject continuam com suas asserções originais. Sua existência
não certifica sozinha toda a semântica de exportação.

## Validação

Checagens concluídas em `2026-10-14T01:00:30.115296+00:00`; suíte de infraestrutura
concluída em `2026-10-14T01:02:19.327315+00:00`. Fontes não mudaram durante as execuções.

- Registro infra: 1292 → 1284 nomes; exatamente os oito
  acima saíram, sem adições.
- Suíte infra: 1316 passaram, 0 falhas, 2 ignorados; os ignorados
  não contam como execução. Cargo offline/locked, execução serial, tmpfs
  isolado, linter publicado e oráculo pinado, com ambiente e hashes em
  `workspace-pre.json.gz`. Apesar do nome herdado do arquivo, esta execução
  usa `-p typst-infra`, e não `--workspace`.
- Build, check workspace/all-targets, formato, Rustdoc com `-D warnings`,
  linhagem estrita, linhagem reversa e diff check passaram.
- Linter arquitetural: 0 erros, 380 avisos e 1611 notas.
- Inventário Clippy workspace/all-targets: 340 → 333 diagnósticos distintos;
  única diferença: sete `assertions_on_constants` removidos. Distribuição:
  core 37, shell 14, infra 269, wiring 13.

O inventário usa `--cap-lints warn` somente para coletar todos os crates.
O gate real permanece `-D warnings` e ainda não passa. A suíte completa
anterior, de 8.361 aprovados, está documentada no relatório
`infra-imports-docs-ci-2026-10-13.md`; não foi repetida nem atribuída ao
estado posterior à remoção dos oito testes. A matriz remota e release
continuam pendentes. Nenhuma release foi publicada.

Logs, receitas, diffs e inventários estão em `evidence.json.gz`, com
hashes conferidos após descompressão. Os snapshots comprimidos são
preservados separadamente.
