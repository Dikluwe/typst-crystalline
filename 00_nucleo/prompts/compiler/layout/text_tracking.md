# L0 — text_tracking
Hash do Código: 71f46335

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text_tracking.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `00_nucleo/diagnosticos/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir exclusivamente boundary(items, style), puro e privado ao módulo pai. Buscar o último item, ignorando apenas marcadores HTML diretos. Text/TextShaped não vazios e TextSeparator fornecem estilo; TextReplacement consulta recursivamente somente o último filho. Texto vazio, glyphs, objetos, grupos, links, linhas e wrappers semânticos interrompem a fronteira. Manter match exaustivo.

Retornar tracking resolvido, inclusive negativo, apenas para estilos completamente iguais, não matemáticos e com direção ausente/LTR. Linha vazia ou fronteira recusada retorna zero. Não inspecionar Layouter nem emitir, alterar cursor ou escolher quebra. Word e Space aplicam o resultado por suas próprias operações; a continuidade lexical adicional pertence ao State privado descrito abaixo, sem multiplicadores para espaços.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

Possuir State privado ao módulo pai, clonável, com identidade opaca do escopo ativo e do último produtor textual. enter cria identidade nova e retorna a anterior; leave restaura somente identidade ativa, conservando a proveniência do último item emitido. Escopo vazio, sem emissão, não interrompe a continuidade. record associa o último produtor ao escopo atual. interrupt cria barreira de objeto até a próxima emissão. boundary no State retorna zero entre identidades distintas e delega ao cálculo puro existente quando contínuas. Identidades podem ser Arc<()> locais, sem contador global, ponteiro serializado ou mudança de contrato público. Preservar filtros existentes de TextStyle/LTR/math, fronteiras de objetos, valor assinado e seleção. Não generalizar para RTL ou habilitar seleção global. Testes devem distinguir escopo repetido, entrada/saída, vazio, restauração, barreira, clone/checkpoint e wrap.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.

## Fronteiras entre unidades em RTL explícito — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29/before-pdf/results.json`, `summary.json`, `provenance.json` e `initial.json` registram 66 entradas e 198 PDFs contra o vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, com fontes, comandos, binários, HEAD e working tree completos. Em `dir:rtl`, três termos latinos deixam de receber quatro intervalos: com ±0.2pt o primeiro termo difere 0.8pt; 0pt coincide. Texto árabe explícito difere 0.4pt nos dois intervalos entre palavras para ±0.2pt. `text_tracking.rs:65` recusa indiscriminadamente RTL; `word_layout.rs:34,72` e `space.rs:25` já projetam/adicionam a fronteira assinada; `cursor.rs:150-186` ancora a linha pelo limite direito real. No vanilla `inline/shaping.rs:1269-1295`, tracking aplica-se antes de cada próximo cluster distinto também em direção negativa. Isso mede posições e escolha de linha, não a intenção histórica.

Hebraico (`dir:rtl`) e espaço literal inicial apresentam diferenças mesmo em 0pt; não são prova de defeito da fronteira. Ligaduras, mistura de direção e paginação global ainda exigem suas próprias sondas.

### Contrato da correção

Para fronteira textual uniforme no caminho incremental, admitir dir explícita RTL além de direção ausente/LTR, com o mesmo filtro de estilo lexical, math, objetos, linha vazia e último item. Devolver um único tracking assinado anterior à próxima unidade; direção não muda o sinal do intervalo. O alinhamento RTL posterior usa a largura real da linha e permanece no owner cursor. Não adicionar tracking a clusters internos de outra unidade, nem reproduzir diferença hebraica independente de tracking. O Núcleo já condiciona expansão a controles; os controles desta rodada autorizam esta fatia, não paridade geral RTL.

Correção interna de paridade ADR-0127 em fluxo contínuo: sem contrato público, default, quebra de compatibilidade ou fase nova. A restrição histórica a LTR fica substituída somente por esta fatia medida. Aceitação RED→GREEN, controles bilaterais em ambas as direções/valores, preservação LTR, build e lint sem erros, resselo restrito.

## Alinhamento da linha RTL continuada — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29/after-pdf/results.json` e `summary.json` medem 66 fontes/198 PDFs com fonte, pin e estado em `provenance.json`; `wrap-initial.json` registra o estado exato antes desta segunda correção. Em latin-wrap ±0.2/0.333/1.2pt, a primeira/segunda linha do Tekt deslocam-se relativamente ao vanilla por exatamente um intervalo ±t, enquanto a última coincide. O texto e as linhas escolhidas coincidem. `line_commit.rs:241-248` conhece se há continuação, mas `:477` chama `cursor.rs:150` sem essa informação; `cursor.rs:164-177` ancora pela direita real. A inferência de um intervalo terminal pendente é refutada se controles de wrap/hifenização mostrarem outro deslocamento. Esta medição não prova uma regra universal para bidi misto ou espaços iniciais.

### Contrato interno

Possuir helper privado de intervalo terminal para linha RTL continuada. Examinar a última unidade textual material, ignorando carrier TextSeparator final e marcador HTML direto, e respeitar objeto/barreira, math e direção. Devolver tracking resolvido do seu próprio estilo, assinado; TextReplacement consulta último filho. Não oferecer ajuste para hebraico cujo desvio em 0pt vem de outra causa nem para espaços literais iniciais.

Mudança interna ADR-0127 em fluxo contínuo, sem assinatura pública, default ou fase. Teste RED→GREEN e controles bilaterais, preservação dos terminais e LTR, build/lint sem erros e resselo restrito.
