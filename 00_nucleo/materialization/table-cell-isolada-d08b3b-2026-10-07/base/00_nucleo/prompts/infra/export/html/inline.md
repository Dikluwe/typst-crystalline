# L0 — html/inline
Hash do Código: b60b7004

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/inline.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

## Consumer evidence

`03_infra/src/export/html/inline.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Inline content dispatch

Own static Content dispatch for text, sequences, Strong/Emph, accumulated Styled
chains, linebreak, table wrappers, HtmlElem and HtmlFrame. Preserve the unsupported
content fallback `HTML export does not yet support this content`. Delegate normal body whitespace,
table serialization and context-dependent escaping to their owners.

HtmlElem preserves typed tag names and attribute order. Empty attribute values emit
only the name; nonempty values are quoted/escaped by encoding. Void tags emit no
closing tag and reject Content bodies even when empty with
"HTML void elements must not have children". Raw/title handling delegates to encoding.
For HtmlFrame, insert the resolver's validated SVG markup unescaped at its occurrence;
without a resolver retain `html.frame must be resolved by the HTML compilation pipeline`. Do not render
layout, wrap frames in p, substitute a frame tag or change style/resolver contracts.

## Verification

Check ordered/empty attributes, void-body rejection, styled chains, raw/title
delegation, unsupported variants and resolved/unresolved inline frames.


## Realização com avaliação única

Serialização HTML emite a projeção visual dos carriers; nunca serializa origem consumida nem cria tags DOM de transporte.


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

Aceitar Content::SmartQuote via helper puro L1 resolve_smart_quote com chain
ativa/idioma; aplicar escape_text ao glyph no modo vigente. Compartilhar
SmartQuoteContext através dos irmãos e wrappers inline transparentes; usar
helper privado stateful e preservar wrapper inline existente para callers
isolados. Nenhum layout Paged para quote; HtmlFrame conserva resolver SVG.
Markup/nativo misturados, disabled/herança e nested tags funcionam no fluxo.
Raw/script continua string apenas e title delega ao owner encoding com chain.
