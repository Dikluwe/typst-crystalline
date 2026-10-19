# Reúso das métricas na reconciliação final — 2026-10-03

## Hipótese e intervenção

O estudo anterior (`grid-cache-real-2026-10-03.md`) observou um terceiro cache de
larguras em `shaper/reconciliation.rs`, depois dos candidatos de layout paginado.
O `C000` tinha a mesma chave de texto, tamanho e fonte nas instâncias. A hipótese
era que manter as métricas do layout comum até `fix_line_positions` eliminaria
medições repetidas sem alterar texto nem posições. O caminho contextual selecionado
e o chamador bundle continuam com métricas novas. A assinatura pública permanece.
Os quatro owners L0 foram editados antes do código e ressellados de forma restrita.

## A/B real

Runner: `00_nucleo/diagnosticos/grid-cache-real-2026-10-03/compare_cache_share.py`,
20 pares alternados por fixture, duas rodadas de aquecimento, um CPU fixo,
`RAYON_NUM_THREADS=1`, `SOURCE_DATE_EPOCH=0`.

- UTC da medição: `2026-10-03 03:02:17 UTC`.
- HEAD: `13c08765ccd2a4665b2f0338d71c648aef53e8f4`.
- Antes: binário SHA-256 `acedd6624e8f370cc2b622976b6fb44ae5f04ca47b84b1414718236400965ff9`.
- Depois, working tree não commitado: binário SHA-256 `f258f380c976183441eb83575acbc41b90fc95967084df8134847d21a3c7d247`.
- `git diff HEAD --stat` no instante da medição: quatro prompts L0, `03_infra/src/pipeline/paged.rs`, `03_infra/src/shaper.rs`, `03_infra/src/shaper/reconciliation.rs`, `03_infra/src/shaper/tests.rs`; **8 arquivos, 39 inserções, 4 remoções**. As edições posteriores foram hashes/linhas `@updated` de linhagem e formatação do teste novo, sem mudar o caminho medido.

| Fixture (320 células) | Antes, mediana | Depois, mediana | Diferença pareada mediana | Pares favoráveis |
| --- | ---: | ---: | ---: | ---: |
| textos distintos | 237,72 ms | 228,89 ms | −8,58 ms (−3,7% pelas medianas) | 20/20 |
| texto repetido | 213,66 ms | 213,61 ms | +0,05 ms | 9/20 |

O segundo caso já desfrutava de hits no cache; sua diferença fica na faixa de
ruído. O ganho distinto soma-se aos cerca de 9–10 ms da rodada anterior de
reúso entre layouts, mas não se deve somar porcentagens de bases diferentes.

`pdftotext -bbox` produziu hashes idênticos antes/depois:

- Distinto: `bf41d63dd7b695589e4bfcd1916afb72fddb31cf837826f67858260fe89edf1e`.
- Repetido: `d720c2bca3ef7e3b64607c4d8d7e4321094733d1e9635d56565af24f4f96d80b`.

Isso cobre texto extraído e coordenadas; não prova igualdade de todos os bytes
ou estruturas do PDF. Não houve nova medição bilateral com o vanilla nesta rodada.

## Verificação e limite

O teste novo foi RED por ausência do helper interno e GREEN depois da mudança.
`cargo test -p typst-infra --lib shaper::tests` passou 56 testes;
`SOURCE_DATE_EPOCH=0 cargo test -p typst-infra --lib pipeline::` passou 121;
`cargo build` e `crystalline-lint .` retornaram código zero. O primeiro run
do pipeline, sem fixar o relógio, falhou em uma asserção de bytes PDF entre
dois entry points; a repetição determinística passou. O linter global
`--fix-hashes` encontrou cópias diagnósticas de código com metadados ambíguos;
os quatro pares L0/código desta mudança foram ressellados por escopo e não
apresentam V5. A sonda GDB adicional não pôde executar porque o sandbox
bloqueou `ptrace`; não atribuímos contagem nova de misses a esta rodada.

Próxima investigação: medir separadamente o cache usado por bidi e os custos
de agrupamento/ordenação da reconciliação em documentos com muitas linhas,
com fixtures que exercitem RTL e geometria, antes de alterar o algoritmo.
