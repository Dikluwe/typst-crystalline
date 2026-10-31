# Configurações das entradas de notas — R12, 2026-10-25

**As divergências originais de `footnote.entry(indent:)`, `gap:` e `separator: none` foram corrigidas.** `clearance` também passou a governar reserva, admissão e paginação, além do caso curto que já coincidia por não testar esse limite. Os **58 controles positivos** passam em PDF e SVG, em debug e release, a **0,001 pt**. Dos 32 refutadores anteriores, três melhoram e os outros 29 preservam seus resultados geométricos: restam **17 casos não qualificados** naquele conjunto.

Há ainda um limite geral de `line(length: 50%)`, demonstrado em três fontes, mantido aberto e separado da configuração de entradas. A rodada começou em 24/10 e foi encerrada em 25/10, America/Sao_Paulo; o diretório de evidências conserva a data inicial e os recibos registram horários UTC. [R11](long-footnotes-r11-2026-10-24.md) e suas evidências foram preservados.

## Estado e proveniência

HEAD mantido: `212aed0449150dd296ceea13fc2be665cc5db01a`, **working tree não commitado recebido de R11**. [baseline.json](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/baseline.json) e [received-source.json.gz](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/received-source.json.gz) conservam o estado inicial, os 2.163 inputs recebidos, os documentos e a lista exata de alterações (`git diff HEAD --stat` e status). [closing-audit.json](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/closing-audit.json) e [completion.json](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/completion.json) vinculam os números finais aos **2.164 inputs** congelados e à lista final de alterações. Não houve commit, reset ou substituição das mudanças recebidas.

Oracle: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore `cac836c07e4dec2a89219370428292393a3216b5`, executável SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`, conferido contra a [receita ratificada](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md). Não houve resync. Os binários recebidos são os executáveis selados em R11, com inputs idênticos ao snapshot inicial desta rodada.

Os testes Rust, os builds e o pipeline final usam um job, teto de 12 GiB, `MemoryHigh=8G` e swap zero; Cargo offline/locked. O release mantém `opt-level=s`, ThinLTO e uma unidade de geração. [Ambiente](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/execution-environment.json), [pipeline de 20 etapas](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/pipeline.json), [build release](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/revalidation/final-build/receipt.json) e [arquivo integral do código](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/revalidation/final-build/source.json.gz) registram comandos, horários e hashes. Debug `4c0a55c69401ffabb20ba6315288a75b189c24ec9023176aefb183d15307ef38`; release `2822a54fe3375c8bdd3e4a12042e4524cc286fbe6722b92522126e24c84c9bd2`. As medições de recursos ocorreram após os builds, sem compilação concorrente.

## Medição, decisão e correção

As quatro fontes originais são mantidas byte a byte em [original-fixtures.json](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/original-fixtures.json). A repetição recebida confirmou que os três setters não produziam o resultado pedido; `clearance:15pt` sozinho era um controle insuficiente porque a nota curta já cabia. O novo caso a 80 pt com `clearance:30pt` exige três páginas no vanilla — chamada, página sem texto com separador e entrada completa — enquanto o produto recebido emitia tudo em uma página. [Antes](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/before/receipt.json), [SVG recebido](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/before-svg/receipt.json), [matriz inicial](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/before-expanded/receipt.json) e [tipos no oracle](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/type-probes.json).

O código fixado em `typst-layout/src/rules.rs:400–410` resolve a indentação dentro da entrada reduzida a 85%; `flow/mod.rs:275–282` lê separador, folga e gap dos estilos compartilhados do fluxo. Os testes de bloco local e declaração tardia refutaram usar os estilos da última chamada como configuração da página. A correção preserva essa distinção:

| Configuração | Comportamento corrigido |
|---|---|
| `indent` | Resolve `pt + em` no tamanho reduzido da entrada. |
| `gap` | Usa a distância declarada entre entradas, no tamanho externo. |
| `clearance` | Participa da reserva, admissão, continuação e progresso limitado. |
| `separator` | Omissão conserva a linha padrão; `none`/vazio removem sua pintura; conteúdo fornecido conserva altura e estilo externo. |

O setter reconhece a função nativa por identidade, inclusive alias léxico, valida os quatro argumentos e usa o canal custom existente da cadeia de estilos. Um snapshot privado do fluxo compartilhado impede que restaurações de estilo descartem a configuração ou que um override local a substitua. A quebra explícita estabelece o novo escopo; paginação automática conserva o anterior. Os frames capturados carregam os valores resolvidos através das divisões e deferimentos. Os corpos e cursores originais das notas continuam com sua avaliação/captura única.

O primeiro candidato expôs dois deslocamentos exclusivos do texto do separador: **0,121005 pt** em Nimbus Sans e **0,424306 pt** em Liberation Serif. O controle de altura de 9,9 pt passou e foi mantido como controle positivo, sem ser contado como RED. A asserção independente de baseline mediu **9,9 pt versus 7,7 pt**. O probe puro e a pintura passaram a usar a origem geométrica já existente do subframe; a origem das notas de R11 ficou preservada. Os recibos do candidato, a tentativa positiva e o RED de baseline permanecem disponíveis.

Os sete L0 foram atualizados antes do código e resselados, com o [núcleo compartilhado de configurações](../prompts/_nuclei/layout/footnote-entry-settings.toml). A classificação é correção interna de paridade pelo fluxo contínuo de ADR-0127: nenhum campo de entidade, trait, assinatura pública, modo ou fase foi alterado. Os proprietários são `eval/rules/set`, `layout/mod`, `footnote`, `footnote_fragments`, `footnote_flush`, `pagebreak` e `set_page`. V15/V26 foram aprovados antes dos reparos de hashes.

A primeira validação completa e os 56 controles iniciais passaram, mas dois refutadores adicionais de `#set page(height:)` mostraram que essa transição material ainda conservava o snapshot anterior: SECOND ficou em 33,97989 pt em vez de 43,329887 pt, ou em 43,32989 pt em vez de 24,629885 pt ao remover a indentação herdada. O [recibo dos refutadores](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/further-scope/receipt.json) e a [primeira tentativa arquivada](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/first-scope-attempt/receipt.json) conservam seus próprios inputs e resultados. Dois novos REDs precederam o hook privado no proprietário `set_page`, após sua transição real e fora de subframes. O código final recebeu uma nova validação completa, sem reutilizar os gates do estado anterior.

## RED → GREEN e validação completa

- [RED funcional](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/red-core.json): sete falhas de asserção, sem mudança de inputs durante o teste.
- [Controle positivo da altura](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/red-origin.json): passou; não foi usado como testemunha RED. [RED da baseline](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/red-baseline.json): uma falha, com altura correta e baseline errada.
- Dez testes novos aprovados no candidato e novamente na suíte final. O aviso de visibilidade privada foi corrigido e o [Clippy inicial](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/pre-clippy.json) conservado; a soma atribuída foi ajustada sem supressão. [Clippy final](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/pre-clippy-scope-final.json).
- **58 PDFs + 58 SVGs por build**: 54 documentos positivos e quatro fronteiras explícitas de fluxo, com três fontes, unidades afins/zero, aliases, separadores textuais/linhas absolutas, escopo compartilhado, continuação literal e folga impossível. Palavras completas por página, dimensões físicas, caixas, glifos e regras a 0,001 pt. [Debug](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/r12-final-debug-evaluation.json), [release](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/r12-final-release-evaluation.json) e [fronteiras](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/flow-boundaries/final-release-evaluation.json).
- [RED de SetPage](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/red-set-page.json): duas falhas de escopo após a transição material, mantendo como premissas positivas as duas páginas e cada corpo emitido uma vez.
- Sete controles de tipo/resultado da CLI concordam com o oracle nos dois builds, inclusive rejeição de inteiros para comprimentos, argumentos desconhecidos e posicionais; [type-final](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/type-final/receipt.json).
- **12 gates completos**, com **9.936 testes Rust aprovados**, sete ignorados e **42 testes Python**: workspace, integração, doctests, all-targets, Clippy, formatação, rustdoc e linhagens direta/reversa. Linter completo sem violações. [Comandos](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/validation-complete/validation-commands.json), [congelamento](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/validation-complete/validation-freeze.json), [SARIF](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/full-lint-final.sarif).

| Preservação por build | PDF | SVG |
|---|---:|---:|
| R8 | 28 | 22 |
| R9 | 75 | 75 |
| R10 | 69 | 69 |
| R11 | 68 | 68 |

Os dois controles bitmap de R10 conservam zero pixels diferentes a 72/600 dpi, e os 16 controles legados permanecem qualificados. Os **21 replays e cinco suplementos** reconfirmam A–H/R1–R7; [audit de aceitação](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/revalidation/audit.json). Os warnings regionais de `text.costs` de R11 permanecem idênticos, sem alteração do classificador ou da tolerância.

O observador adicional de linhas relativas parou antes de salvar sua primeira comparação por falta dos imports do parser SVG retido. A [tentativa](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/pipeline-relative-attempt1.json) e o [registro de correção](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/observer-import-correction.json) conservam o erro, os artefatos parciais e o script original. Apenas esses imports foram corrigidos; a retomada manteve os 13 gates/etapas já aprovados do mesmo código final, sem alterar inputs produtivos, fontes, classificadores ou tolerâncias.

## Pintura, recursos e preservação

A pintura das entradas foi observada independentemente a 72/600 dpi. Debug e release têm os mesmos resultados; os valores abaixo são pixels diferentes frente ao oracle e não substituem o gate geométrico. Resíduos não nulos permanecem explícitos no [recibo de pintura](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/entry-paint/receipt.json).

| Documento | PDF 72 | PDF 600 | SVG 72 | SVG 600 |
|---|---:|---:|---:|---:|
| note-entry-indent | 0 | 2 | 1 | 0 |
| note-entry-gap | 0 | 2 | 2 | 0 |
| note-entry-no-separator | 0 | 0 | 3 | 0 |
| note-entry-clearance | 0 | 2 | 3 | 0 |
| DejaVu Sans-separator-text | 0 | 0 | 2 | 0 |
| DejaVu Sans-separator-line-absolute | 0 | 0 | 2 | 0 |
| Nimbus Sans-separator-text | 0 | 0 | 0 | 0 |
| Liberation Serif-separator-text | 0 | 0 | 0 | 0 |

Nas oito amostras, PDF a 72 dpi e SVG a 600 dpi têm zero pixels diferentes. PDF a 600 dpi conserva dois pixels diferentes com delta máximo de canal 11 em `indent`, `gap` e `clearance`; SVG a 72 dpi tem no máximo três pixels diferentes e delta máximo 1. A ausência da regra em `separator:none` é confirmada independentemente pelo SVG. Esses resíduos não foram classificados como pintura exata nem usados para afrouxar a geometria.

Os gates de R6 continuam aprovados: tamanho normalizado atual/R5 **0.855634**, média geométrica RSS atual/R5 **0.952139**, limites por workload e repetibilidade preservados. Os três testes de estresse também passam. [Recursos](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/revalidation/final-resources/receipt.json) e [estresse](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/revalidation/legacy-resources/receipt.json). Esses controles usam as referências históricas de R5, sem alegação de ganho causal de R12 sobre R11.

Os **2.149 inputs recebidos fora dos sete pares de proprietários** permanecem intactos; apenas um núcleo foi acrescentado. Os corpos dos testes recebidos de R11 também foram conferidos. A [auditoria de preservação](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/preservation.json) reconfere **72.996 arquivos históricos**, 1.853 assets e os fixtures de fontes/bitmap. A licença DejaVu mantém a normalização já recebida de HEAD212; seu hash histórico anterior não foi restaurado. Os relatórios anteriores conservam seus corpos e arquivos; o índice passa a apontar para esta rodada.

## Limites que continuam abertos

Nos 32 refutadores anteriores, apenas `note-entry-indent`, `note-entry-gap` e `note-entry-no-separator` melhoram. `clearance` curto continua qualificado; a nova prova de paginação está no corpus R12. Os outros 29 preservam as observações geométricas, restando **17 casos não qualificados**, listados nos recibos [debug](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/refuters-debug/receipt.json) e [release](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/refuters-release/receipt.json). Não equivalem a 17 defeitos independentes.

A matriz inicial de 54 documentos incluía três `separator: line(length:50%)`. O setter recebido ignorava o argumento inteiro e desenhava a regra padrão errada; ao avaliar corretamente o argumento, aparece a rejeição já existente do construtor `line` relativo, declarada no L0 de shapes. Esses três documentos **continuam não qualificados**; a mudança de resultado de compilação está registrada, sem alegação de preservá-la. Os controles positivos usam a linha absoluta já aceita de 130 pt, equivalente a 50% da região de 260 pt. As 54 fontes iniciais, três linhas absolutas e quatro fronteiras totalizam 61 documentos de geometria: **58 qualificados e três refutadores de construtor abertos**. Nenhuma expressão falha foi apagada ou reclassificada como aprovada. [Classificação dos corpora](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/cohort-classification.json) e [refutadores relativos](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/relative-line-refuters/receipt.json).

As divergências anteriores de scripts, linhas vazias, hifenização, matemática, pintura e recursos frente ao vanilla mantêm seus escopos e continuam abertas onde já estavam documentadas. A rodada não certifica todas as formas de separador opaco, fontes ou resoluções. Hashes e bytes são proveniência; a aceitação continua sendo a linguagem e a saída observável. [Encerramento](../../lab/results/construction/diagnostics/footnote-entries-r12-2026-10-24/completion.json).
