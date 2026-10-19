# Binário atual após memória5 versus vanilla pinado

## Proveniência e protocolo

Comparação nova, sem rebuild nem alteração de código/L0. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**. `candidate.json.gz` conserva as fontes/L0/configurações integrais, status incluindo untracked e `git diff HEAD --stat` no instante da preparação. Todas essas fontes permaneceram idênticas no fechamento; a identidade do release atual coincide com a execução validada de memória5.

- Tekt atual SHA-256: `89d01f159b7244d8684b8644400b7673fff34ee278894e906dabcde73c17b450`.
- Vanilla `lab/typst-original/target/release/typst`, SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- Pin vanilla: `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489. A string de versão não foi usada como prova.

UTC `2026-10-08T23:57:29.745085+00:00` → `2026-10-08T23:58:32.523747+00:00`. Quinze fixtures, três blocos de quatro pares, ordem alternada e um aquecimento por motor/caso/bloco: **360 execuções medidas**. Processos CLI novos com filesystem aquecido, CPU 0, Rayon 1, vanilla jobs 1, SOURCE_DATE_EPOCH=0, TZ UTC, LC_ALL C, fontes compartilhadas de `performance-baseline/fonts`, PDF sem tags/pretty. `%M` mede pico RSS do processo filho; wall inclui o wrapper `/usr/bin/time` comum. Monitor exclui builds/testes/lint/GDB concorrentes. `bench.json.gz` conserva todos os comandos, horários, stderr, SHA-256 de entradas/fontes/fontconfig/assets/config/código/L0/binários. Fingerprints conferidos antes/depois.

Não mede watch, recompilação incremental nem disco frio. Não atribui os ganhos a uma função isolada, e não soma percentuais das intervenções antigas. Os testes/build/linter do binário atual estão no relatório `value-compacto-memoria5-execucao-2026-10-08.md`; não foram repetidos porque nenhum código mudou.

## Medições

Antes da decisão, foram conferidos texto sem normalizar whitespace, palavras, dimensões/páginas, bounding boxes, família/tamanho/cor de fonte e raster a 36 dpi. Os resultados estão em `qualification.json.gz`. A tabela abaixo inclui todos os casos, inclusive os que têm saída diferente. Percentual negativo indica menor custo do Tekt. Valores absolutos são medianas por motor; percentuais são medianas das razões pareadas e não necessariamente a razão das medianas. `summary.json` conserva também as três variações por bloco.

| Caso | Wall ms Tekt / vanilla | Tempo pareado | RSS MiB Tekt / vanilla | RSS pareado | Qualificação |
|---|---:|---:|---:|---:|---|
| cloned-strokes | 117.90 / 139.55 | -15.52% | 43.86 / 32.60 | +34.55% | controles coincidem |
| context | 116.96 / 135.37 | -13.68% | 43.29 / 29.60 | +46.70% | ressalva de saída |
| context10 | 118.27 / 135.44 | -12.65% | 43.12 / 29.47 | +46.28% | controles coincidem |
| context100 | 135.06 / 137.55 | -1.34% | 52.79 / 29.76 | +77.35% | conteúdo/fluxo diverge |
| context300 | 221.46 / 144.43 | +57.50% | 124.97 / 30.87 | +304.85% | conteúdo/fluxo diverge |
| dictionary | 119.83 / 137.51 | -12.05% | 43.27 / 29.55 | +46.36% | controles coincidem |
| image | 11.81 / 13.34 | -14.34% | 18.48 / 21.14 | -12.73% | ressalva de saída |
| numeric-array | 122.20 / 135.50 | -10.21% | 47.55 / 30.14 | +57.97% | controles coincidem |
| pipeline-single-font | 116.19 / 134.90 | -13.87% | 41.76 / 29.17 | +43.16% | controles coincidem |
| plain300 | 138.85 / 135.38 | +2.16% | 46.34 / 29.38 | +57.72% | conteúdo/fluxo diverge |
| shapes | 14.36 / 14.00 | +1.15% | 19.62 / 21.20 | -7.18% | controles coincidem |
| table-alias | 123.19 / 137.66 | -10.82% | 44.19 / 30.77 | +43.61% | ressalva de saída |
| table-local | 134.70 / 138.03 | -2.55% | 44.38 / 30.84 | +43.79% | ressalva de saída |
| text-dense | 136.86 / 138.01 | -1.36% | 44.20 / 30.28 | +46.00% | ressalva de saída |
| unique-strokes | 123.33 / 145.55 | -15.35% | 43.59 / 34.31 | +27.55% | controles coincidem |

## Divergências e limites observados

`context100`, `context300` e `plain300` produzem números consecutivos sem separadores. No PDF vanilla, a extração enxerga uma única linha até o trecho próximo de 55, enquanto o Tekt distribui os números em múltiplas linhas e chega até 99/299. A inspeção da extração e do raster demonstra fluxo distinto; não demonstra por si só a intenção correta da linguagem. Esses três casos **não são trabalho equivalente para certificar ganho/perda de desempenho**. Context300 registrou aproximadamente 221 ms e 125 MiB no Tekt, contra 144 ms e 31 MiB no vanilla; estes números são diagnóstico com essa ressalva, não prova de inferioridade para saída equivalente.

`context`/`text-dense` conservam palavras/páginas/fontes, mas têm delta vertical até 0,658006 pt. O raster de context coincide na resolução examinada; isso não apaga a diferença geométrica. `table-alias` tem whitespace extraído e raster diferentes; palavras e bbox até 0,000020 pt coincidem. `table-local` coincide em texto/palavras/páginas/fontes/bbox até 0,000020 pt, mas difere no raster. `image` tem raster diferente e palavras vazias; seus tempos não certificam renderização equivalente. Essas diferenças não foram corrigidas nesta tarefa de medição.

Stderr diferente em 0 dos 180 pares; os recibos individuais estão em bench.json.gz. Raster 36 dpi e tolerância de bbox 0,0001 pt são controles locais; não certificam paridade global de semântica/sintaxe/morfologia nem exigem igualdade de bytes PDF.

## Recorte com controles coincidentes e decisão

Sete casos coincidiram em texto, palavras, páginas/dimensões, estilos e raster, com bbox até 0,0001 pt: cloned-strokes, context10, dictionary, numeric-array, pipeline-single-font, shapes, unique-strokes. Strokes únicos/repetidos e arrays avaliam construção/armazenamento de valores e imprimem o tamanho do array; não renderizam milhares de traços. Os gains dessas fixtures não representam exportação massiva de strokes.

Agregado construído = soma de uma compilação fresca por caso em cada ciclo pareado. Não é um lote cronometrado à parte, nem uma distribuição ponderada pelo uso do produto. Neste recorte, Tekt 739.04 ms versus vanilla 854.51 ms; variação pareada **-13.42%**, blocos -12.83%, -13.43%, -13.42%. Todos os casos desse recorte com texto tiveram menor wall nos três blocos. Shapes foi um pouco mais lento e é uma execução curta sensível a ruído. Não extrapolar o agregado aos quinze casos, nem tratá-lo como superioridade universal.

**Conclusão:** o Tekt atual tem vantagem medida de tempo em texto simples, arrays, dicionário e construção/clonagem de strokes neste corpus. A vantagem não é uniforme. Em memória, o Tekt continua pior nos casos com texto, mesmo após as otimizações: aproximadamente 42–48 MiB contra 29–34 MiB nos exemplos coincidentes. Shapes usa menos RSS nesta bateria, mas não autoriza concluir redução universal de memória. As grandes populações contextuais precisam de fixtures com fluxo equivalente antes de uma comparação conclusiva contra vanilla.

Artefatos em `atual-vanilla-memoria5-2026-10-08/`: state/candidate, scripts, logs, bench, summary, aggregate, qualification e completion. Nenhum commit, alteração produtiva ou nova decisão arquitetural foi feito.
