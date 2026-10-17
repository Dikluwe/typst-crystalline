# L0 candidato — extração exata de cobertura cmap por intervalos
Hash do Código: pendente

Camada L3. Consumer produtivo exclusivo previsto: `03_infra/src/fonts/coverage.rs`. Candidato completo; instalar no caminho canônico `00_nucleo/prompts/infra/fonts/coverage.md` somente após ratificação do pacote público. Este documento não legitima código enquanto permanecer em materialization.

## Medição anterior à decisão

`00_nucleo/diagnosticos/cmap-intervalos-2026-10-05/{measurement-direct,measurement,execution}.json.gz` conserva programa/receita, fontes, binários, HEAD/dirty tree, UTC, argv e amostras. O experimento reduziu extração/união isoladas de53.19 para10.85ms; sete testes sintéticos passaram. Nas1112 faces instaladas daquele estado, havia1520598 intervalos/3582 subtabelas nativas e nenhum fallback. Não são contagens atuais nem ganho da CLI. `00_nucleo/diagnosticos/coverage-ranges-preparacao-2026-10-07/audit.json` confirma o consumer puro byte-idêntico à medição e o extrator antigo L3 byte-idêntico ao mesmo HEAD limpo. O resto da árvore e o release atual diferem; precisam de nova medição após implementação.

Fontes primárias lidas: ttf-parser0.25.1, `src/tables/cmap/format4.rs:103–115`, `format12.rs:68–75`, `format13.rs:41–48`; callbacks enumeram limites declarados, sem filtrar glyph0. `cmap/mod.rs:196–219,251–260` usa get tipado, offsets relativos à cmap e encerra o iterador no primeiro registro não parseável. Format4 parse usa segCountX2/2 e exige seus arrays completos; o campo length não autoriza aqui uma validação incompatível adicional. Vanilla pin9f2b6e8715237cb086899a42873660fe744622e8, `text/font/info.rs:119–134`, enumera e une pontos; não tem o mecanismo direto que será implementado.

## Decisão e fronteira

Usar segmentos nativos somente nas variantes4/12/13 aceitas pelo parser tipado. Manter classificação Unicode, ordem/terminação de subtabelas e todos os membros declarados do extrator existente. Bytes, validação e seleção do caminho são exclusivos desta unidade L3; união/codificação do conjunto é exclusivamente de `00_nucleo/prompts/entities/font_book.md`, via Coverage::from_ranges. Normalização neutra não importa ttf-parser em L1. Não inferir intenção upstream nem trocar a semântica por suporte aproximado de blocos Unicode.

Não criar API pública, counters de telemetria, flags, imports de lab, unsafe, globals ou política nova de catálogo/caches. Parent `00_nucleo/prompts/infra/fonts.md` conserva o facade extract_coverage(&Face)->Coverage e delega ao filho privado. I/O/catalog/discovery/shaping/FontInfo permanecem com seus owners; o adapter só lê bytes já recebidos da Face.

## Contrato privado do adapter

1. Face sem cmap produz Coverage vazia, como o caminho antigo. Consultar cmap.subtables.get(i), em ordem; se falhar, terminar ali, sem saltar para um registro posterior. Ignorar subtables não Unicode pela classificação tipada vigente.
2. Em cada subtable Unicode, selecionar formato nativo pela enumeração tipada. Ler a tabela raw cmap da mesma Face e seu record offset correspondente; validar formato bruto, acesso e aritmética com checked_add/checked_mul e slices.get. Não tratar os bytes de uma subtable diferente como sua origem.
3. Formato4 lê start/end dos segmentos declarados; converter limites inclusivos para Range semiaberto. start>end contribui vazio. Parar no singleton FFFF, inclusive quando há segmentos posteriores. Glyph0/idDelta/idRangeOffset não filtram membros: são campos de mapping, não o critério do callback codepoints existente.
4. Formatos12/13 leem grupos declarados em sua ordem, preservando overlap/duplicatas e buracos. Grupo start>end é vazio. End inclusive converte por checked_add(1), sem saturação, clamp ou wrapping.
5. Guardar checkpoint por subtable. Qualquer falha de leitura/validação nativa reverte integralmente o que aquela tentativa anexou e usa seu callback tipado existente. Outros formatos Unicode usam esse callback; não virar suporte nativo adicional sem nova medição.
6. Compactar pontos de fallback em intervalos conectados com acumulador local; ordem arbitrária/duplicatas não alteram a união. Um endpoint exclusivo não representável sinaliza que esta Face precisa do extrator legado inteiro; descartar a tentativa parcial e enumerar a Face na fronteira antiga. Não fabricar intervalo vazio ou perder membros. O domínio Unicode suportado do point constructor fica inalterado; valores inválidos fora desse domínio não recebem promessa nova.
7. Ao terminar coleta válida, usar uma chamada Coverage::from_ranges. O resultado preserva membership/iter/fallback order/canonical equality/hash e não muda indexação de slots. Não guardar referências à Face nos valores L1.
8. Todo output é Coverage própria. Não executar shaping/export, reler arquivo, iniciar World ou forçar catálogo dentro deste adapter.

## Aceitação e testes antes de código

Escrever testes no próprio módulo contra um oráculo independente de membros/BTreeSet e, como controle adicional, o callback legado. Cobrirem4 com glyph0/sentinela/múltiplos segmentos/invertidos;12/13 com buracos/overlap/duplicatas/zero/Unicode máximo; não Unicode; formatos6 e outro callback-only; cmap ausente; registro inválido seguido de válido; truncamento/count/offset/formato incoerente com rollback; endpoint exclusivo não representável no coletor privado. Não usar velocidade como threshold de unit test nem exigir nova política para fonte malformada.

Comparar Coverage de cada face instalada/embutida/fixtures com o caminho antigo fora dos timers. Registrar arquivos e índices, exact hashes e casos de fallback. Preservar seleção de fontes/fallback/estilo, textos/glifos/posições e mensagens de erro na CLI/PDF, incluindo fontes com cobertura ausente e texto multiescrita. Sem font asset equivalente, classificar a lacuna em vez de alegar paridade.

Após GREEN e resselo restrito: builds, suites core/infra/shell/query, linter sem erros e owners sem V5/V6/V15/V26; benchmark de extração/catalog/CLI com executáveis congelados atual/anterior/vanilla pinado, três rodadas com gate/monitor e estado/fontes/argv/UTC completos. Critério de performance é ganho repetível em trabalho/saída equivalentes, incluindo RSS, não extrapolação dos42ms do experimento. Preservar resultados negativos e a árvore recebida; sem commit/reset ou otimização de leitura seletiva da prioridade3 neste pacote.
