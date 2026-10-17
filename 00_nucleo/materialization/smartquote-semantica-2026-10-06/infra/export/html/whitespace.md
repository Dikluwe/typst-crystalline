# L0 — html/whitespace
Hash do Código: 088d8a43

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/whitespace.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

## Consumer evidence

`03_infra/src/export/html/whitespace.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Structural space handling

Own inline_html_body, ruby recognition, space_between/top_level_space_between and
SpaceSupport classification. Traverse Styled and inline wrappers without modifying
text/raw payloads. Trim only structural Sequence edges and spaces adjacent to block
or br boundaries; preserve supported inline spaces. Distinguish Boundary,
EmptyInline and Supportive, with replaced tags supporting visible space.

An isolated ASCII space lacking supportive content on both sides uses exactly
`<span style="white-space: pre-wrap">&#x20;</span>`. Top-level lookup reaching a
non-groupable node drops the boundary space rather than making a paragraph/span.
Within ruby, drop a space only when both immediate non-space neighbors are rp/rt;
base text or ordinary inline siblings still preserve spaces. Use tags' static
classification rather than duplicating display tables or implementing CSS.

SpaceFromNewline has the same local projection as Space in these helpers. CJK
normalization and textual show matching belong upstream; do not inspect neighboring
CJK characters to hide an upstream language error. Do not indiscriminately trim
raw/title or change preformatted content.

## Verification

Check edges, br/block adjacency, supportive/replaced versus empty inline nodes,
top-level boundaries, ruby counterexamples and direct Space/SpaceFromNewline pairs.


## Aspa semântica e resolução tardia — proposta aguardando ADR-0127

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

### Decisão proposta e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. PARAR
antes de testar/materializar este pacote até o dono confirmar. Candidatos
são cópias completas para revisão, não os L0s vigentes. Após confirmação,
substituir cláusulas conflitantes do contrato histórico apenas para aspas;
ADRs/núcleos existentes e propostas alheias permanecem intactos.

Encaminhar o contexto de aspas compartilhado ao helper inline stateful para
irmãos em um único fluxo; skip/trim de spaces observa só separators efetivos,
sem reset em spans inline. Manter APIs privadas wrappers atuais onde caller
precisa de fluxo isolado. Ruby/br/block continuam seguindo catálogos vigentes,
sem CSS/table de display nova e sem mudar preformatted raw/title.
