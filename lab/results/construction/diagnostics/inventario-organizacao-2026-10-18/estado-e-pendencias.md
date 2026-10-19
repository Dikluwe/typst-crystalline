# Estado consolidado: o que foi feito e o que falta

A situação abaixo resulta da leitura dos relatórios e da comparação do freeze
produtivo atual com a revisão 10 de R3-T1, não de uma nova campanha de paridade.
HEAD, working tree, horários e hashes estão no [inventário](README.md). “Entregue”
significa o recorte comprovado na evidência indicada. Os defeitos herdados sem
nova medição permanecem como pendências a revalidar, sem alegar que a saída atual
foi novamente observada. Nenhum diagnóstico desta pasta é Prompt L0.

## Entregas anteriores já incorporadas

Os marcos de 13–17/10 foram consolidados nos commits `e2d7f6cdc` e `b1b486f3d`.
As alterações posteriores continuam locais; os lotes se sobrepõem em vários owners.
O histórico Git e `git-before.json.gz` distinguem esses estados.

| Frente | O que já existe | Limite / evidência |
|---|---|---|
| Linhagem, CI e distribuição | Selos diretos/reversos, ownership/núcleos, linter pinado, gates de testes/Clippy/docs, oracle pinado, pacote com BUILD.json e Docker/release dependentes do CI. | [CI/release](../../../tools/ci/README.md); configuração pronta não significa release liberada ou matriz remota recém-aprovada. |
| Contratos Raw, tiling e transformações | Presença explícita em Raw, transporte de paint/tiling nos frames e operações de transformação tipadas materializados após aprovações. | Propostas/recibos em `00_nucleo/materialization/{raw-presenca,tiling-transporte,transformacoes-tipadas}-2026-10-14/`; preservar esses contratos, não arquivar os L0s vigentes. |
| Clippy | Correções locais, type_complexity, inicialização com Default, traits e enums; aridade triada com refatorações privadas e exceções nominais prescritas nos L0s. | [Aridade](../clippy-aridade-final-2026-10-16/relatorio.md), [locais](../clippy-locais-2026-10-16/relatorio.md), [enums](../clippy-enums-publicos-2026-10-16/relatorio.md); Clippy estrito passou no último fechamento. |
| V12 e organização das camadas | Published em L3 e suporte de testes extraído dos harnesses L4. | [V12](../tekt-v12-2026-10-16/relatorio.md); não há pendência V12 no SARIF final consultado. |
| V16 / linter | Lotes por owner, complementos nominais e parser None corrigido; pin atual do linter aplicado. | [Parser/pin](../tekt-v16-none-parser-2026-10-16/relatorio.md); ainda existem erros V16 reais/pendentes, não confundir com notas V19. |
| Texto relativo TS1 | Length conserva abs+em até a resolução; callers migrados e controles de Heading. | [TS1](../text-size-length-2026-10-17/relatorio.md); não fecha RM1, Paint ou toda a geometria de Heading. |
| Igualdade Styled | Igualdade da linguagem corrigida nos controles de estilos de render, sem copiar PartialEq Rust. | [Igualdade](../text-styled-equality-2026-10-17/relatorio.md); não fecha todos os campos de Heading, Styles ou identidade std. |
| Referência vanilla | Pin atualizado para bd5ebec28 com fonte, árvore, receita e SHA do binário. | [Re-sync](../vanilla-resync-2026-10-17/relatorio.md); relatórios contra 9f2b6e871 continuam históricos. |

As contagens históricas de cada relatório não foram somadas ou promovidas a
resultados atuais. Os limites particulares continuam valendo.

## Entregas locais posteriores ao último commit

| Lote | Feito e validado no recorte | O que não está fechado |
|---|---|---|
| A1 callbacks/reflexão/seletores | State resolvido por leitor e sob demanda; init opcional; suplementos Heading antes de consultas; campos já transportados; seletores por identidade; projeção HTML de marcadores invisíveis. | Box.body/presença, equações aninhadas, HTML Ref/math/StateDisplay legado e diagnósticos. [Relatório](../lote-a1-callbacks-reflexao-seletores-2026-10-17/relatorio.md). |
| Pilha/concatenação | Sequências artificiais de +/join removidas; isolamento de frames recursivos; ensaio 64×2048 conclui. | Não é promessa de profundidade arbitrária. [Relatório](../state-stack-2026-10-17/relatorio.md). |
| R1/R2/R2-S Sequence | Reflexão de children e schema, construtor real, selector/where/show/counter por identidade, preservação de alias versus reconstrução fresca e decisão da Sequence antes dos filhos ocultáveis. | Reflexão Styled/Styles/Symbol e casos além do corpus. [R1](../sequence-reflection-2026-10-17/relatorio.md), [R2](../sequence-constructor-2026-10-17/relatorio.md). |
| R3 markup/slots | Filhos imediatos de ContentBlock, slots consumidos e None; certificados de prefixo antes de 1fr. | TextualGroups e reflexão adicionais exigem reconciliação das sondas antigas; não usar só repr como prova. [Relatório](../sequence-r3-2026-10-18/relatorio.md). |
| A2 bibliografia | Estilo CSL preservado, fronteiras de parágrafo/título, estrutura/colunas/continuação e prioridade de margens. | A2-Multi, clusters adjacentes e resíduos RTL não foram encerrados. [CSL](../bibliography-a2-2026-10-18/relatorio.md), [estrutura](../bibliography-structure-2026-10-18/relatorio.md), [margens](../bibliography-margins-2026-10-18/relatorio.md). |
| Hanging-indent | Recuos válidos, default/override causal, direção e parágrafos comuns; origens LTR de decorações e ausência de traço em corpos vazios. | Containers, estratégias Plain+optimized e geometria completa de decorações. [Relatório](../bibliography-hanging-2026-10-18/relatorio.md). |
| Faces mistas/admissão regional | Extremos de métricas independentes; altura negativa preservada; entrada após bloco/nova página e casos regionais certificados. | Regiões fora do certificado, fontes e recortes residuais. [Relatório](../mixed-style-metrics-2026-10-18/relatorio.md). |
| R3-T1 textual | Precedência literal/regex, regras locais/herdadas, produtos protegidos, show-set textual e pai antes dos filhos; alias bibliográfico corrigido. | Marco parcial: Context e coerções de retorno continuam abertos. [Fechamento](../r3-t1-materializacao-2026-10-18/relatorio.md). |

A1-S1 foi retirada e substituída por A1-S2: não implementar a adição de init em
StateUpdateElem a partir do pacote antigo. RG1 continua proposta, baseada no pin
anterior, sem implementação autorizada. Sua existência não é aprovação.

## Backlog organizado por dependência

| Ordem proposta | Trabalho | Resultado necessário antes de fechar |
|---|---|---|
| O0 | Proteger os 26 novos arquivos produtivos/L0/núcleos e recibos ignorados; preparar commits por lote. | Conteúdo recuperável e commits revisáveis; não executar git clean no estado atual. |
| O1 | Limpeza de caches e arquivamento histórico conforme lista própria. | Aprovação dos caminhos, cópia conferida quando necessária e reconstrução documentada; nenhum arquivo removido nesta etapa. |
| P1 | Context: herança global/local de regras textuais. | Sondas global/local hoje documentadas como A versus R/L devem passar; L0 primeiro, preservar execução adiada e escopo. |
| P1 | Context: identidade de equações e query. | Múltiplos contextos e prefixo Heading devem contar ocorrências reais corretamente, sem Location fabricada; separar de equação aninhada. |
| P2 | R3-T2: retornos de callbacks show. | Coerção comum para valores além de Content/String, com None, Raw, Module e morfologia corretos; produto segue as regras pertinentes. |
| P2 | Reflexão/presença | Box.body, Styled/Styles, labels e TextualGroups; revalidar as fontes antigas no freeze atual e propor contratos quando necessário. |
| P2 | V16 por lote e L0 | Auditar os 105 findings por consumer, separar exaustividade e fallback intencional; manter V19/V20 visíveis e critérios de testes. |
| P3 | Bibliografia/layout residual | A2-Multi, clusters adjacentes, CSL custom Plain+optimized, containers, métricas/fontes, checkpoint com prefixo em current_line, RTL e decorações. |
| P3 | Reparos internos E09–E13/E16–E20 | Revalidar cada origem do inventário, escrever RED e corrigir sem copiar mecânica indevida. |
| P4 | Contratos RM1/RG/FG/Paint/Symbol/document | Rebasear as propostas no L0 e pin atuais, medir e obter os gates ADR-0127 aplicáveis antes do código. |
| P4 | RAM e prontidão de produção | Benchmark release pareado, corpus alvo, matriz CI/targets e empacotamento no checkout aprovado; não extrapolar debug versus release. |

P1–P4 são uma sugestão de sequência, não execução já iniciada. Não há necessidade
de aprovar exclusões ou contratos novos para concluir este inventário.

## Reconciliação do inventário E01–E20

Fonte de base: [inventário contra bd5ebec28](../paridade-novo-vanilla-2026-10-17/inventario.md).
O estado abaixo combina essa medição com os reparos posteriores documentados.
“Pendente herdado” não significa reprodução nova nesta sessão.

| ID | Estado consolidado | Próximo escopo / owners indicados na base |
|---|---|---|
| E01 | Pendente herdado: percentuais/relativos em h/box; TS1 não fecha esses carriers. | RM1, casting de layout e entidades correspondentes. |
| E02 | Parcial: getters transportados corrigidos em A1; presença Box.body e campos Figure continuam. | content/fields, field_access, carriers donos. |
| E03 | Parcial: State e suplemento Heading corrigidos; compatibilidade legada e casos de contexto/nesting delimitados. | state/resolve, introspect/headings e Context. |
| E04 | Catálogo/seletores reparados no recorte A1/R2; diagnósticos e seletores compostos ainda têm diferenças. | counter, selector_matching e catálogo nominal. |
| E05 | Parcial: CSL, fronteiras, hanging, margens e alias avançaram; bibliografia universal não fechada. | bib_csl/plan, bibliography, convergence e lotes A2 restantes. |
| E06 | Pendente herdado: document format/string/auto/reflexão e construção versus export. | Owners de document/format e pipeline semântico. |
| E07 | RG1/RG2/RG3/Table ainda dependem de contrato e validação. | Proposta table-row-groups; atualizar pin/L0s antes de ratificar. |
| E08 | Pendente herdado: Paint textual rico. | Texto, estilos/paint e resolução correspondente; transporte FramePaint não o substitui. |
| E09 | Parcial: TS1 tratou tamanho relativo; sub/super, altura de filhos e substituição de Heading ainda a revalidar/corrigir. | Layout de Heading/parágrafos e scripts. |
| E10 | Parcial: R3-T1 tratou precedência, Unicode e show-set no corpus; Context permanece aberto. | textual/matching e realização; reconciliar cada sonda original antes de fechar E10 inteiro. |
| E11 | Pendente herdado: overflow de literal inteiro e forma negativa. | lexer/code, AST e expression_dispatch. |
| E12 | Pendente herdado: include de arquivo só com definições deixa separador. | modules/markup; revalidar após as mudanças compartilhadas de realização. |
| E13 | Parcial: controles de igualdade Styled foram preservados; supplement de Heading e identidade std seguem na fila. | Igualdade da linguagem/morfologia/modules, sem redefinir igualdade Rust. |
| E14 | Pendente herdado: Figure kind/caption/reflexão. | Contrato FG e entidades/constructor de Figure. |
| E15 | Pendente herdado: Symbol ainda perde identidade ao virar Text. | Carrier de Symbol/Content; gate público se adicionar variante. |
| E16 | Pendente herdado: stretch/attachments e callback em argumentos matemáticos. | math/layout/stretch, attach e eval/math/arguments. |
| E17 | Pendente herdado: export SVG/PNG multipágina e template de nomes. | pipeline/raster e CLI; compatibilidade de nomes exige classificação do gate. |
| E18 | Pendente herdado: junções PDF de tiling, subpixel e alpha variável. | pdf_tiling e controle analítico; preservar composição correta do Tekt. |
| E19 | Pendente herdado: html.elem/span com body String. | stdlib/html/elem e projeção HTML. |
| E20 | Parcial: spans/erros primários tiveram reparos, rastros e mensagens ainda divergem. | Owners de lexer, modules, call_dispatch e show; medir o erro como observável. |

Escopos adicionais continuam: image SVG/PDF, MathML em HTML, padding negativo,
mappers genéricos de Cell, Table multipágina/PDF-UA, rich properties e formatos.
São lacunas/dívidas classificadas na base, sem nova prova universal aqui.

## Diferenças deliberadas a preservar

Conforme fonte e controles do inventário base, manter a composição de overlap de
tiling, tratamento seguro do caso subpixel e validade do SVG onde o vanilla
falha. Qualidade de amostragem/junções permanece uma dívida separada. Preservar
L1 puro, despacho estático e compartilhamento Arc; a paridade é de linguagem.

Compatibilidades legadas (como números absolutos em callers e format.html) pedem
política explícita; não recebem crédito de paridade nem devem ser removidas em
massa. Mensagem primária correta do Tekt não deve copiar mascaramento upstream.
Não se deduz “vantagem Tekt” de suporte ausente, erro não medido ou RAM sem benchmark.

## Arquivos novos que precisam ser preservados

Estes sete pares são produtivos e ainda não rastreados no estado coletado. Não
classificar como rascunhos por estarem fora de um commit. Os hashes dos 26
arquivos protegidos estão no inventário de produção.

| Rust | Owner L0 |
|---|---|
| `01_core/src/compiler/introspect/from_tags/headings.rs` | `00_nucleo/prompts/compiler/introspect/from_tags/headings.md` |
| `01_core/src/compiler/layout/bib_csl/plan.rs` | `00_nucleo/prompts/compiler/layout/bib_csl/plan.md` |
| `01_core/src/compiler/layout/paragraph_indent.rs` | `00_nucleo/prompts/compiler/layout/paragraph_indent.md` |
| `01_core/src/compiler/realize/recipes.rs` | `00_nucleo/prompts/compiler/realize/recipes.md` |
| `01_core/src/compiler/realize/sequences.rs` | `00_nucleo/prompts/compiler/realize/sequences.md` |
| `01_core/src/compiler/stdlib/state/resolve.rs` | `00_nucleo/prompts/compiler/stdlib/state/resolve.md` |
| `03_infra/src/pipeline/runtime.rs` | `00_nucleo/prompts/infra/pipeline/runtime.md` |

Núcleos novos associados:

- `00_nucleo/prompts/_nuclei/content/markup-sequence-children.toml`
- `00_nucleo/prompts/_nuclei/content/sequence-realization.toml`
- `00_nucleo/prompts/_nuclei/eval/pending-text.toml`
- `00_nucleo/prompts/_nuclei/eval/textual-consumed-slots.toml`
- `00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml`
- `00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml`
- `00_nucleo/prompts/_nuclei/layout/bibliography-prefix-admission.toml`
- `00_nucleo/prompts/_nuclei/layout/block-inline-region.toml`
- `00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml`
- `00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml`
- `00_nucleo/prompts/_nuclei/layout/prepared-text-carriers.toml`
- `00_nucleo/prompts/_nuclei/paragraph-hanging-validation.toml`

Há ainda oito documentos no namespace de prompts sem referência direta nos
headers Rust coletados (incluindo templates e especificações históricas):
`template-prompts.md`, `package-spec-dto.md`, `auditar-fatiamento.md`,
`auditar-spec.md`, `infra/package_version_resolution.md`,
`compiler/show-regex.md`, `compiler/style/font-dict.md` e
`compiler/stdlib/text/regex.md`. Essa busca não substitui V15/V26 e não é prova
de violação ou inutilidade; revisar classificação e referências antes de qualquer
proposta de movimentação. Não estão na lista de descarte.
