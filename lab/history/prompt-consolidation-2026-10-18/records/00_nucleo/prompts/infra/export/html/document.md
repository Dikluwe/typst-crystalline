# L0 — html/document
Hash do Código: 6f0012ea

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/document.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130


## Consumer evidence

`03_infra/src/export/html/document.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Document composition

Own export_html_with_frame_resolver, automatic doctype/envelope/metadata,
explicit_document_root and block_sequence with the initial StyleChain. A sole
explicit html root follows the doctype directly without automatic lang/head/metadata;
a sole body retains its attributes inside the automatic html/head envelope. html/body
with other relevant nodes returns `` `<{}>` element must be the only element in the document ``
with the offending tag substituted. Ignore only
the existing introspective/empty-space nodes in root classification; head/title are
ordinary nodes, not extracted metadata.

Buffer top-level paragraph-groupable content according to tags; flush at parbreaks,
block/non-groupable nodes, tables and HtmlFrame. Preserve trimming and paragraph
boundaries through whitespace helpers; do not insert paragraphs inside normal
HtmlElem bodies. Emit compact markup first and call pretty only afterwards.
Inline frames remain block-level without a paragraph wrapper; their markup belongs
to inline and their rendering to the external resolver.

## Verification

Check explicit html/body exclusivity, head as ordinary content, attributes/metadata,
mixed paragraph boundaries, tables/frames and equivalent compact/pretty document DOM.


## Realização com avaliação única

Antes de classificar blocos/raiz HTML, projetar visualmente os carriers internos para conservar tags e agrupamento DOM existentes. Não serializar origens consumidas; o índice semântico permanece separado.


## Heading sob estilos transparentes — 2026-10-06

Medição anterior à decisão: html-headings e html-show na baseline
`lab/results/construction/diagnostics/paridade-dominios-2026-10-06/measurement.json.gz`
retornam erro unsupported no Tekt e h2/h3 no vanilla pinado 9f2b6e871.
`document.rs:47-87,145-158` classifica apenas Heading direto e mapeia seu
nível sem offset; Styled cai em inline, que não aceita blocos. No oracle
`lab/typst-original/crates/typst-html/src/rules.rs:223-260`, nível 1 vira h2
e nível >=6 usa div role=heading aria-level=nível+1. Não inferir intenção.

Classificar fronteiras de bloco através de Styled/Sequence sem perder a chain
lexical. Converter blocos estilizados em seus blocos DOM; não englobar heading
em p nem fragmentar spans inline do mesmo parágrafo. Centralizar a emissão de
Heading neste owner, com offset de nível e fallback semântico para nível 6.
Apenas conteúdo visual é emitido; carriers internos continuam projetados na
entrada. Não alterar a classificação de raiz html/body nem seu diagnóstico.
Numbering e avisos de heading profundo não recebem certificação nesta rodada.
Correção interna de paridade ADR-0127, sem assinatura/fase/default/unsafe novo.
Testes RED→GREEN: heading em Styled/Sequence com parágrafos antes/depois, tag
por nível e inline Styled sem fragmentar o parágrafo; DOM bilateral e suíte L3.


### Raiz explícita sob Styles — refinamento medido

Na coleta intermediária html-headings/measurement.json.gz, explicit-body e
explicit-html em Styled produzem raízes duplicadas no Tekt; o vanilla mantém
a raiz declarada como única. `document.rs::explicit_document_root` examinava
somente filhos diretos e o caminho de Styled recém-corrigido revelou o root
preexistente oculto. Classificar raiz através de Styled/Sequence, transportando
a chain lexical com cada nó relevante até inline. Não inventar body automático
para raiz já explícita nem aceitar root com outros nós relevantes. Preservar
o diagnóstico de exclusividade e não projetar filhos de HtmlElem como raízes
do documento. Teste RED→GREEN de body estilizado e exclusividade sob wrappers.


## Aspa semântica e resolução tardia — ratificada em 2026-10-06

O contrato desta seção é vigente e supersede as cláusulas históricas conflitantes
das seções anteriores apenas para SmartQuote. A ratificação humana autoriza a
materialização em fluxo RED→GREEN, com ownership exclusivo e pins abaixo.

### Medição anterior à decisão

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
registra pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 dos binários,
HEAD/working tree, diff HEAD --stat, UTC, fontes, argv/cwd/ambiente e stdout/stderr.
PDF usa texto extraído, HTML usa DOM/texto e diagnósticos, não igualdade de bytes PDF.
A avaliação converte aspas de markup para Text; campos double/enabled são perdidos.
A variável contendo aspas mantém glyph inglês sob uso em alemão/francês ou
smartquote(enabled:false). Primes 6'/2" viram aspas no Tekt. Show da função
nativa e nome público foram corrigidos separadamente e passam antes deste gate;
markup ainda não é SmartQuote. Constructor vazio tem fields vazio no vanilla;
double:false é campo conhecido, mas double omitido é Unset. enabled:false
permanece SmartQuote no vanilla. set double:false e show filtrado têm positivos
novos. HTML literal funciona porque recebe Text; SmartQuote programática hoje
é rejeitada. Título literal renderiza aspas; script recebe string, rejeitando
Content antes da exportação nos dois binários.

Fontes: markup_stream.rs:79–128 decide glyph em eval; smartquote.rs de layout
alterna dois flags sem texto precedente; stdlib/text/smartquote.rs:112–117 apaga
elemento disabled; observation/elements.rs:242–250 compara os campos atuais.
Vanilla typst-eval/markup.rs:141–146 cria elemento com double explícito;
typst-layout/inline/collect.rs:196–211 resolve estilo/contexto na coleta;
typst-library/text/smartquote.rs:119–158 distingue prime/apóstrofo/nesting.

### Decisão ratificada e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. Confirmação humana recebida em 2026-10-06. O contrato ratificado abaixo
supersede cláusulas históricas conflitantes apenas para aspas; ADRs/núcleos
existentes e propostas alheias permanecem intactos.

Compartilhar contexto de aspas enquanto append_sequence/append_block acumulam
um único parágrafo, não criar contexto em cada folha inline. Resetar em
flush lógico/block boundary; heading/body Par possuem fluxo próprio. Passar
contexto ao helper inline stateful mantendo públicas export APIs intactas.
Preservar envelope/raízes/exclusividade, carriers visuais e estilos. A diferença
já existente de auto-root lang=en versus locale especificado fica registrada
fora do gate; não alegar HTML bytes idênticos quando só glyph/body coincidem.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

workspace-seventh/p1421 HTML contextual refuta perda de tag sob Source. Coleta/root, bloco-inline e render de documento/sequence delegam Source ao body, preservando tags, estilos, encoding e estado SmartQuote; Source não fabrica parágrafo/container.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.


## A1 — projeção de marcadores introspectivos sem saída visual

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/fontes/argv e diffstat em candidate-probe/target-controls/state-extended.json.gz
e candidate-probes-resources.json de
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17.
State reader-init, unused e at-prefix passam no PDF, mas HTML rejeita o marcador
com unsupported; vanilla pin bd5ebec285505889e8ff1d4f68804ab147a699f3 aceita.
Upstream foundations/styles.rs:1049–1050 registra StateUpdate/Metadata como empty
em ambos os targets. Tekt document.rs:28–35 já projeta carriers antes de blocos;
Label tem body (entities/content/construction.rs:391–398), que não pode sumir.
Isto é conteúdo/erro observável. Não inferir intenção ou exigir mecânica upstream.

### Responsabilidade e aceitação

Na projeção visual já owned, StateUpdate, Metadata e inicializador State legado
contribuem Empty; Label contribui seu body transformado. O índice semântico é
construído antes e preserva ações, valores e labels. Não avaliar callbacks no
exportador nem ocultar ContextBlock/StateDisplay não realizados. Preservar corpo
de Label, fronteiras Heading e whitespace. Não certificar links/âncoras de Ref
ou matemática HTML. Sem API pública, flag, fase ou default novo: correção de
paridade em fluxo contínuo ADR-0127, teste RED→GREEN no próprio owner e sondas
bilaterais PDF/HTML. Testar marcador entre texto, heading rotulado, metadado e
State legado, inclusive erro de conteúdo ainda não suportado.
