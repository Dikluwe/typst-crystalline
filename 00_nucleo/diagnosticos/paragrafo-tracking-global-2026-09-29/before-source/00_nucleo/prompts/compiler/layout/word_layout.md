# L0 — word_layout
Hash do Código: d3c65be1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:363fc473f68d7a9faea59460d462eae77c4eb7c6df1360b91a30f6b6f0dd92ef

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/word_layout.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `00_nucleo/diagnosticos/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir word, chunk, prepared e o produtor privado push_text com highlight. Word/chunk pedem admissão a text_baseline::admit; word preserva delegação ICU e ambos medem uma vez por text_measure::word. Resolver hyphenate lexicalmente: Bool explícito prevalece; auto/ausência usam par.justify Bool, default false. Não modificar chain ou persistir política no estilo.

Consultar State::boundary do owner text_tracking e projetar cursor_x + intervalo para break_selection. Keep aplica o intervalo uma vez antes de emitir. Wrap zera o intervalo, fecha com flush_continued_line, admite baseline e emite com a largura previamente medida. Hyphenate aplica intervalo, emite prefixo com substituição causal, avança sua largura, fecha continuação e recorre para o restante. Chunk não hifeniza e conserva seleção por largura. Não inserir espaço automático entre fragmentos.

push_text conserva estilo, baseline/offset, ordem da tinta e highlight: extent resolvido no tamanho atual, radius positivo determina rounded rect, demais casos rect, ascender e altura vêm das métricas injetadas. Substituição opcional envolve exclusivamente a folha Text em TextReplacement, deixando highlight fora. Texto ordinário/chunk usam None; palavra usa replacement do seletor.

prepared aplica temporariamente o TextStyle fornecido, chama ensure_initial_baseline e push_text com largura e substituição pré-medidas, avança e restaura estilo. Não seleciona quebras ou acrescenta tracking; a certificação global continua recusando tracking. As fórmulas de fronteira e alinhamento de carriers pertencem aos respectivos owners, não a este emissor.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

Usar boundary do State no cursor projetado de word/chunk. Registrar produtor após push_text, inclusive prepared, para associar texto realmente emitido ao escopo atual. Keep/wrap/hifenização, widths pré-medidas, SHY, highlight e estilo permanecem; fim/início de linha continua regido pelo buffer real.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.

## Fronteiras entre unidades em RTL explícito — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29/before-pdf/results.json`, `summary.json`, `provenance.json` e `initial.json` registram 66 entradas e 198 PDFs contra o vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, com fontes, comandos, binários, HEAD e working tree completos. Em `dir:rtl`, três termos latinos deixam de receber quatro intervalos: com ±0.2pt o primeiro termo difere 0.8pt; 0pt coincide. Texto árabe explícito difere 0.4pt nos dois intervalos entre palavras para ±0.2pt. `text_tracking.rs:65` recusa indiscriminadamente RTL; `word_layout.rs:34,72` e `space.rs:25` já projetam/adicionam a fronteira assinada; `cursor.rs:150-186` ancora a linha pelo limite direito real. No vanilla `inline/shaping.rs:1269-1295`, tracking aplica-se antes de cada próximo cluster distinto também em direção negativa. Isso mede posições e escolha de linha, não a intenção histórica.

Hebraico (`dir:rtl`) e espaço literal inicial apresentam diferenças mesmo em 0pt; não são prova de defeito da fronteira. Ligaduras, mistura de direção e paginação global ainda exigem suas próprias sondas.

### Contrato da correção

A fronteira calculada por State::boundary pode ser não nula em RTL explícito. Projetá-la no cursor usado por break_selection; Keep/Hyphenate aplicam uma vez, Wrap zera antes de emitir na linha seguinte. Conservar largura nominal, hifenização, SC, prepared (tracking recusado) e identidade textual. Não espelhar coordenadas ou deslocar manualmente glyphs: owner cursor ancora a linha pelo limite direito.

Correção interna de paridade ADR-0127 em fluxo contínuo: sem contrato público, default, quebra de compatibilidade ou fase nova. A restrição histórica a LTR fica substituída somente por esta fatia medida. Aceitação RED→GREEN, controles bilaterais em ambas as direções/valores, preservação LTR, build e lint sem erros, resselo restrito.
