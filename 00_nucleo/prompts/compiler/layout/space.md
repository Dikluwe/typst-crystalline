# L0 — space
Hash do Código: 044b5479

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/space.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `00_nucleo/diagnosticos/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir layout e advance para Markup/Newline/Literal. Markup/Newline resolvem estilo efetivo pelo owner `00_nucleo/prompts/compiler/layout/text/style.md`; Literal reutiliza estilo já resolvido. Medir avanço nominal por text_measure::space.

layout suprime Markup/Newline iniciais ou depois de apenas marcadores HTML. advance emite carrier quando há material ou quando Literal tem tracking resolvido diferente de zero. Aplicar text_tracking::boundary ao cursor antes do carrier, uma vez. Emitir TextSeparator append-only com origem, estilo, posição e advance nominal; advance não inclui tracking nem deve ser somado novamente pelo exportador. Aplicar largura nominal ao cursor e registrar oportunidade de justificação.

A posição y inicial usa cursor_y mais offset do estilo, sem demandar baseline. text_baseline pertence ao alinhamento posterior quando chega texto material. Literal sem tracking conserva avanço sem carrier inicial. Não inferir espaços a partir de distâncias ou texto adjacente. Em layout, quando par.justify não é true, flush somente após cruzar limite direito; com justify true, a unidade material seguinte escolhe continuação. advance literal não introduz esse flush imediato.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

Usar boundary do State antes do carrier e record depois de emitir TextSeparator. Espaço suprimido ou avanço inicial sem carrier não registra produtor. Conserva advance nominal, origem, estilo, baseline e justificação.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.

## Fronteiras entre unidades em RTL explícito — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29/before-pdf/results.json`, `summary.json`, `provenance.json` e `initial.json` registram 66 entradas e 198 PDFs contra o vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, com fontes, comandos, binários, HEAD e working tree completos. Em `dir:rtl`, três termos latinos deixam de receber quatro intervalos: com ±0.2pt o primeiro termo difere 0.8pt; 0pt coincide. Texto árabe explícito difere 0.4pt nos dois intervalos entre palavras para ±0.2pt. `text_tracking.rs:65` recusa indiscriminadamente RTL; `word_layout.rs:34,72` e `space.rs:25` já projetam/adicionam a fronteira assinada; `cursor.rs:150-186` ancora a linha pelo limite direito real. No vanilla `inline/shaping.rs:1269-1295`, tracking aplica-se antes de cada próximo cluster distinto também em direção negativa. Isso mede posições e escolha de linha, não a intenção histórica.

Hebraico (`dir:rtl`) e espaço literal inicial apresentam diferenças mesmo em 0pt; não são prova de defeito da fronteira. Ligaduras, mistura de direção e paginação global ainda exigem suas próprias sondas.

### Contrato da correção

Em RTL explícito, avançar para a próxima unidade textual pelo mesmo State::boundary e conservar advance do TextSeparator exclusivamente nominal. Origin/style/baseline/justificação e supressão de espaço inicial permanecem. Não somar outro intervalo em PDF ou inverter seu sinal pela direção.

Correção interna de paridade ADR-0127 em fluxo contínuo: sem contrato público, default, quebra de compatibilidade ou fase nova. A restrição histórica a LTR fica substituída somente por esta fatia medida. Aceitação RED→GREEN, controles bilaterais em ambas as direções/valores, preservação LTR, build e lint sem erros, resselo restrito.

## Geometria auto de separadores literais — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-auto-espacos-2026-09-29/before/results.json`, `provenance.json` e `initial.json` medem 30 fontes/60 PDFs, font-path, comandos, UTC, HEAD, working tree e binários contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Só espaços literais (`#text("   ")`) com tracking 0/±0.2 produzem no Tekt `/MediaBox [0 0 0 0]` (Poppler apresenta 612×792), enquanto o vanilla mede respectivamente 9.36/9.76/8.96×8.568pt. Separadores literais finais após texto perdem a sua largura em páginas auto; os iniciais seguidos de texto já coincidem. Escopo com tamanho 18pt termina em 14.44×12.852pt vanilla para +0.2pt. Texto vazio e markup sem tinta revelam outra falta de baseline/mínimo; newline final tem divergência vertical própria e não é fechado por esta fatia.

Causa fonte: `space.rs:22-35` não transporta Literal inicial sem tracking; `page_geometry.rs:104-135` usa `helpers::line_content_right` que filtra todos os TextSeparator, por necessidade correta do alinhamento RTL, e a altura de linha só com carrier não recebe topo tipográfico; `builder.rs:291` e `builder/fonts.rs:633,1167` escrevem MediaBox zero. Vanilla `crates/typst-pdf/src/convert.rs:108-118` impõe mínimo físico de 3×3 na saída PDF. Isso justifica projeção auto separada do alinhamento RTL e mínimo compartilhado entre os três caminhos exportadores. Não inferir intenção histórica do escopo antigo.

### Contrato interno da correção

Transportar TextSeparator Literal desde o primeiro espaço mesmo com tracking zero, sem criar baseline ou tinta. O avanço/carrier continua nominal; tracking anterior é aplicado apenas quando existe unidade anterior elegível, sem duplicação. Markup/Newline iniciais continuam suprimidos. Preservar estilo literal resolvido para a projeção auto, sem introduzir estado global nem contagens codificadas.

Correção interna de paridade em fluxo contínuo ADR-0127, sem campo público, assinatura/flag/fase nova. Aceitação RED→GREEN para espaços 0/± e PDF mínimo, casos causais bilaterais, corpus anterior, build e lint sem erros, resselo estritamente no manifesto. Texto vazio explícito e newline final permanecem para rodada própria.
