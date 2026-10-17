# Fluxo de Block e tabelas: diferenças de largura, ancoragem e bidi

As oito diferenças herdadas da matriz de espaço terminal foram corrigidas.
O lote ampliado passou de **154/168 para 168/168**, sem regressões; seis controles
adicionais também foram corrigidos. O modo compacto passou de **26/40 para 40/40**.
Estes resultados certificam texto e posições pelo método abaixo, não igualdade
de bytes de PDF nem igualdade visual exata.

## Proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`.
Estado anterior UTC `2026-10-06T12:06:29.725716+00:00`;
verificação final iniciada em `2026-10-06T12:44:01.520131+00:00`.
[before.json](block-tabela-fluxo-2026-10-06/before.json) conserva fontes/L0 anteriores
e stat/status completos; [execution.json.gz](block-tabela-fluxo-2026-10-06/execution.json.gz)
conserva sete consumers, sete owners, dois núcleos, hashes, stat/status, UTC e logs.
O escopo exclusivo está em scope.json; ownership/pins/linhagem finais em seals.json
mais lint.sarif. Outros arquivos da árvore já estavam alterados antes desta frente.

Tekt anterior SHA-256 `bc072887ed0178966ac08b217c8623b1a1876f3d1856299d05361f565c350bad`;
final `f6820eeea52476b22ef837be276f3b11d263fa5caae5b4e73cb1e2c07b3c9750`. Binários congelados foram usados nas coletas.
Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`,
conforme a proveniência ratificada P1489. Fontes em font-provenance.json.
CLI: SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1, tags desativadas, pretty=false.
Cada fonte, comando, texto bruto e XML/bbox está em
[verification.json.gz](block-tabela-fluxo-2026-10-06/verification.json.gz).
Compacto possui coleta e script próprios, sem passar --compact ao vanilla.

## Medição antes da decisão

1. `measurement.rs:141`: Sequence limitada usava a maior largura de um filho e
   somava alturas, mesmo para filhos que formavam um fluxo inline. A célula Auto
   mista recebia largura insuficiente e quebrava o árabe. O teste
   `inline_sequence_is_measured_as_one_flow` falhou antes da correção.
2. `block.rs:317`: o reposicionamento RTL do Block Auto usava apenas a extensão
   visual dos caracteres. Espaços literais terminais emitidos ficavam fora da
   largura usada para ancorar o frame. O teste
   `rtl_block_places_literal_terminal_space_inside_its_frame` falhou.
3. `grid/tracks.rs`: uma coluna era sempre ancorada na margem esquerda, mesmo
   quando o alinhamento Start resolvia à direita. O teste
   `one_column_rtl_uses_the_outer_right_anchor` falhou.
4. A primeira candidata resolveu os dois Blocks e três tabelas LTR, mas as três
   tabelas RTL ainda divergiam na ordem. A hipótese de que largura/âncora bastavam
   foi refutada por candidate-verification.json.gz. `bidi_origin.rs:115` rejeitava
   qualquer célula/sub-frame; `grid/cell_body.rs:229` rebatia o texto mas excluía
   TextSeparator. No novo teste causal, texto tinha y=27,7pt e espaço y=20pt.
   `text_and_separator_share_a_certified_cell_baseline` confirmou RED antes do reparo.

São diferenças observáveis de língua/posição, não requisitos para copiar a
estrutura interna ou memoização do vanilla. A origem histórica destes caminhos
não foi demonstrada nesta coleta; não inferir intenção a partir deles.

## Decisão e implementação

- Sequence recebe medição de fluxo somente com certificado estrutural positivo
  de conteúdo inline e largura finita positiva. Um Layouter isolado usa estilos
  e métricas injetados; o estado de emissão do pai é preservado. Quebras explícitas,
  parágrafos e conteúdo estrutural mantêm seus caminhos próprios.
- A extensão intrínseca inclui avanço de separadores Literal emitidos. O cálculo
  geral de tinta/alinhamento de linha continua separado; espaços fracos de markup
  e newline não ampliam automaticamente o frame terminal.
- Uma coluna em região finita respeita alinhamento físico ou Start/End resolvido
  pela direção. Esta frente não generaliza a regra a grades de várias colunas.
- A origem bidi de um corpo textual completo pode atravessar o sub-frame da
  célula certificada de uma coluna. A capacidade privada começa desativada,
  é restaurada pelo owner e suspensa em sub-frames aninhados/inline. As demais
  exclusões e o certificado textual existentes permanecem.
- TextSeparator acompanha a mesma transformação vertical dos itens de texto.

Os L0 foram atualizados antes da implementação, com dois Núcleos Tekt para os
invariantes compartilhados e owners 1:1. Correções internas de paridade seguem
ADR-0127 em fluxo contínuo. Sem novo unsafe, API pública, dependência ou mudança
entre fases do pipeline.

## Validação e limites

Quatro testes causais RED→GREEN; os logs de compilação exploratória não são
contados como RED. Suítes finais: **6.540 testes L1 passaram, 5 ignorados;
1.178 testes L3 passaram**. Builds debug/release e diff-check passaram.
Lint: **0 erros, 531 avisos, 1.578 notas**, nenhum diagnóstico acrescentado ou
removido comparando regra/nível/mensagem/arquivo; V15/V26 íntegros.

Paridade medida com pdftotext: texto sem formatação de direção/espaçamento e
caixas de palavras adjacentes agrupadas, tolerância <0,05pt. A comparação bruta
é conservada: 156/168 textos coincidem exatamente; diferenças restantes de
formatação do extrator não foram ocultadas. O maior delta agrupado no lote é
0,016206pt; no compacto 0,042600pt. Os controles com um e dois espaços literais
preservam também a extração bruta. Nada permaneceu divergente nesse critério.
Ver [summary.json](block-tabela-fluxo-2026-10-06/summary.json).

Rasterização dos oito casos a 600dpi, limiar 180, em raster.json e scripts.
Os dois Blocks têm zero pixels de tinta sem correspondência dentro de 1px.
As seis tabelas conservam resíduo de 42–49 pixels do vanilla dentro desse
critério, concentrado nos cantos das bordas; inspeção visual confirma pequena
diferença de junção de traços. A borda superior tem extensão raster diferente
em 1px (0,12pt). **Não declarar igualdade visual exata nem fechar a paridade
geral de bordas/junções.** Texto, quebra e ancoragem pedidos estão corrigidos.

## Tempo: sem ganho consistente demonstrado

Medição da CLI inteira, UTC `2026-10-06T12:46:03.087675+00:00`, mesmo estado/hash/fontes,
duas rodadas de aquecimento e quinze rodadas pareadas em ordem aleatória.
Tempos incluem catálogo/fontes, compilação e exportação; não atribuir a L1.

| Documento | Antes ms | Depois ms | Vanilla ms | Mediana do delta pareado ms |
|---|---:|---:|---:|---:|
| grid320 | 195.90 | 195.99 | 148.99 | -0.46 |
| aligned-80 | 169.84 | 169.82 | 135.42 | +0.11 |
| blocks-80 | 173.71 | 173.28 | 137.25 | -0.16 |

A tabela de 320 células permaneceu praticamente igual. As pequenas diferenças
não sustentam conclusão de aceleração. A medição de fluxo continua construindo
itens isolados; reaproveitar esse produto é uma frente separada de memoização,
com igualdade de largura, estilos, região e contexto a demonstrar. Dados brutos
em [benchmark.json](block-tabela-fluxo-2026-10-06/benchmark.json).
