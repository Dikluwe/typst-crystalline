# Correção de tracking — 2026-09-29

O tracking agora atravessa fronteiras textuais de estilo uniforme no layout incremental e conserva sua parte fracionária na geometria/PDF. Espaços repetidos e literais iniciais recebem seus pares uma única vez. O controle `representation end` com 0.2pt caiu de 0.348pt de desvio para cerca de 0.000001pt nas caixas de palavras; o máximo entre os 36 controles de posições de glifos é 0.00003pt. Isso fecha os dois mecanismos isolados nessa matriz, não toda a paridade de tracking/paginação.

## Proveniência e estado

Artefatos em `correcao-tracking-2026-09-29/`:

- `initial.json`: UTC, HEAD, working tree não commitada, lista exata de arquivos alterados e `git diff HEAD --stat`, hashes completos das fontes/L0 e binários anteriores.
- `current-source.json` e `final-receipt.json`: estado final completo, fontes verificadas sem mudança durante as sondas finais, comandos/logs, hashes e escopo alterado. As contagens e os limites abaixo referem-se a esses recibos e aos JSONs de cada lote, não somente ao HEAD.
- HEAD: `17f13b1df79b87ea065ee62e78164b18e477da3d`; vanilla pinado: `9f2b6e8715237cb086899a42873660fe744622e8`.
- SHA-256 final Tekt: `f6cd7011742a146b37775d93de73539208ff90de438e619e03763fc50b0faaf7`.
- SHA-256 vanilla: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- Fontes Noto Sans solicitadas por CLI, seus caminhos/comandos e inventário SHA-256 estão no recibo final; o fallback RTL não é certificado integralmente nesta rodada.

`review.diff.txt`, `changed-source.json`, `scope.json`, `scoped-seals.json` e os scripts arquivados permitem revisar/reproduzir o escopo sem usar a árvore inteira de alterações preexistentes como se fosse desta frente. São dez pares L0↔consumer e um núcleo novo (21 arquivos produtivos/L0). Não houve novo contrato público, fase ou flag. Nenhuma referência PDF foi regenerada.

## Medição anterior à decisão

A auditoria `00_nucleo/diagnosticos/tracking-residual-2026-09-29/` separou a quantização interna e a perda de pares entre fragmentos. Em 12pt/Noto Sans, tracking 0.2pt virava 17 unidades da fonte (0.204pt). Treze pares de `representation` acumulavam +0.052pt. Um espaço perdia dois pares (−0.4pt), resultando em −0.348pt no começo de `end`. Com três espaços são quatro pares, e a adjacência sem espaços também tem um par. ±1.2pt refutou a hipótese de que tudo vinha de arredondamento.

O owner shaping legitimava manter o inteiro arredondado; text_measure conservava o adiamento da discrepância; space avançava só a largura nominal. A decisão atual altera esses limites explicitamente nos owners, antes do código, sob o fluxo contínuo ADR-0127. Não atribui motivação pessoal nem adota defeitos de vanilla como requisitos.

## Materialização por responsabilidade

`compiler/layout/text_measure` possui a medida privada da fronteira entre unidades textuais: mesma linha, estilos completamente iguais, direção ausente/LTR e fora de math. Marcadores diretos são neutros; objetos/semantic wrappers interrompem; TextReplacement encaminha sua última folha visual. O match mantém barreiras explícitas e exaustivas, sem despacho dinâmico ou exceção por fonte/palavra.

`word_layout` usa a fronteira no cursor projetado antes de decidir wrap/hifenização; soma somente no Keep/prefixo emitido na mesma linha. Wrap descarta o intervalo e admite a baseline da próxima linha. `space` mede o avanço nominal no estilo efetivo e soma o par anterior antes da posição do carrier; o carrier não reaplica esse intervalo. Isso resolve n+1 pares entre palavras separadas por n espaços.

A rodada adicional refutou a primeira implementação em espaços literais iniciais. `first-round/` conserva as sondas e o estado dessa implementação. `red-leading.log` registra o novo teste causal. O produtor agora conserva carriers para Literal com tracking não zero mesmo antes de material, sem demandar baseline. Quando o texto é admitido, o owner word_layout alinha esses carriers à baseline real, com o offset de cada estilo. Markup/Newline iniciais e o caminho sem tracking conservam sua política anterior.

O átomo L3 `shaper/tracking.rs` calcula somente o resíduo entre tracking exato e inteiro armazenado, antes do próximo cluster distinto. Não muda glyphs, texto, dados públicos, recursos ou cache. Shaping usa sua extensão ao posicionar sub-runs; reconciliation usa a mesma largura real e não reaplica as fronteiras de L1.

PDF single/coalescido/bitmap consome o mesmo resíduo. O TJ conserva o delta inteiro anterior de kerning e serializa a correção fracionária com cinco casas quando necessária; a fronteira de run com tracking compensa o cursor efetivamente serializado com a mesma precisão. Tc continua explicitamente zero em shaped. Text/Type1 conserva seu caminho anterior. Não foi corrigida nesta rodada a precisão dos carriers vazios no compacto.

O núcleo novo `00_nucleo/prompts/_nuclei/layout/text-tracking.toml` compartilha invariantes e está pinado pelos owners; ele não é consumer nem alvo de @prompt. O código permanece estático e as decisões de cada unidade ficam em sua camada.

## Validação

Os REDs estão em `red-core.log`, `red-word.log`, `red-infra.log` e `red-leading.log`; as receitas arquivadas reconstroem as etapas a partir de `before-source/`. Discriminam fronteira assinada, espaços repetidos, decisão de wrap e reinício de linha, cluster compartilhado/último glyph e interpretação independente dos operadores Tc/TJ. O teste de PDF cobre emissão individual e coalescida, incluindo fronteira absoluta fracionária. Os testes históricos de soma única continuam passando.

O único teste histórico cuja expectativa mudou exigia justamente ausência de tracking entre palavras e comparava tamanhos diferentes. Seu owner foi atualizado primeiro; a substituição usa mesmo tamanho e deslocamento exato de 2*tracking, inclusive negativo, com tolerância mais estrita. Não se removeu o controle.

- Núcleo: 6.369 testes unitários, oito de integração e três doc-tests aprovados; dois testes preexistentes ignorados.
- Infra: 1.137 testes unitários e 34 de integração aprovados. Total: 7.551 aprovados, zero falhas.
- Builds dev e release offline concluídos. `lint-final.sarif`: zero erros, 503 warnings e 1.557 notes; nenhuma V5/V6/V15/V26 no escopo. Os avisos/notas não são apresentados como zero violations.
- 36 controles principais novos contra vanilla, ambos os modos: glifos visíveis coincidem dentro de 0.001pt, máximo bruto 0.00003pt. `after/results.json` conserva traces e valores brutos, sem aumentar a tolerância para esconder a perda anterior.
- 72 comparações raster a 144dpi (36 contra vanilla e 36 entre modos): todas com pixels idênticos. PDFs/hashes e imagens estão em `visual/`.
- Matriz anterior de 30 controles reexecutada: todos compilam; quatro casos de tracking 1.2pt têm agrupamento de palavras Poppler diferente, por isso a comparação bruta não é declarada igual nem descartada como semântica certificada.
- 42 controles extras novos, com ambos os modos e vanilla: 37 coincidem em palavras/caixas e dimensões de página dentro de 0.015pt. Incluem fronteira de wrap, quebra forçada, espaços iniciais/finais, auto-leading, parágrafo anterior, grupo local, SHY, ligadura, bold e tamanhos 13/13.2pt. Os cinco abertos ficam enumerados em `extra-summary.json` e no recibo.
- Replay de 401 entradas: 397 preservam a geometria/diagnósticos previamente certificados. Quatro baselines com tracking mudam como esperado e continuam FAILED na comparação bruta com o comportamento antigo, sem adulteração do recibo. Três controles de tracking −0.333/0.333/3pt agora coincidem em glifos visíveis com vanilla compilado novamente com desvio máximo de cerca de 0.00001pt. O controle paginado permanece aberto, como abaixo. Observações vanilla do replay geral foram reutilizadas com entradas byte-idênticas; os controles principais/extras e as quatro verificações discriminantes recompilaram o oráculo.

## Limites e próxima frente

O controle paginado `after-pdf/tracking` tem as mesmas quatro linhas no Tekt e no vanilla, mas o Tekt coloca três linhas na primeira página e uma na segunda; o vanilla coloca duas em cada página. A preparação global ainda recusa tracking (`paragraph_prepare`), logo esse fallback não recebe a proteção de viúvas/órfãs que já existe para linhas preparadas. O máximo bruto de coordenadas de 32.736pt nessa comparação está conservado; não é residual de quantização a esconder. Próxima frente: certificar medição/seleção de parágrafos com tracking e sua proteção regional.

Tracking misturado positivo conserva uma diferença de 0.2pt; RTL explícito conserva 0.8pt. A equivalência entre estilos resolvidos e suas fronteiras lexicais precisa de estudo próprio. Dois controles auto com conteúdo exclusivamente de espaços conservam a página 612×792pt anterior, enquanto vanilla mede altura/largura do conteúdo; before/after prova que esse limite é preexistente. Nenhum desses cinco controles foi contabilizado como fechado.

O fallback genérico de medição ainda conta codepoints, portanto não se declara seleção completa em ligaduras/combining/scripts, mesmo quando a emissão e alguns controles fecham. A precisão de até 0.048pt dos carriers vazios no compacto continua independente. Não há alegação de desempenho, superioridade sobre vanilla, paridade global ou cobertura de outros targets nesta frente.
