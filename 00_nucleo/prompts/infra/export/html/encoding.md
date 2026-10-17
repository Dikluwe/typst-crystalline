# L0 — html/encoding
Hash do Código: 44c74b92

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/encoding.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

## Consumer evidence

`03_infra/src/export/html/encoding.rs:12` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Contextual escaping

Own raw_text/title_text, escape_text and escape_attribute, not the public mode enum
or export entrypoints. Crystalline retains conservative escaping; Vanilla normal
text escapes &/< and normal attributes &/double-quote. Preserve replacement order.

Raw accepts only Empty/Text/Space/SpaceFromNewline/Linebreak/Sequence/Styled as textual
content; script/style are not HTML-escaped. Title uses escapable-raw encoding:
escape only &/<, preserve > and quotes, keep repeated spaces/newlines, and turn
Linebreak into newline while absorbing its immediately following Space or
SpaceFromNewline. Nontext children retain
"HTML raw text element cannot have non-text children". SpaceFromNewline projects
identically to Space here; perform no additional DOM/whitespace normalization.

## Verification

Check both modes independently in text/attributes, raw/title contexts, nontext
diagnostics, repeated spaces and linebreak separators.


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

Title suporta folha SmartQuote e a resolve via helper L1 com chain/contexto
próprio ao título, antes do escape escapable-raw existente. Preservar repeated
spaces/newlines/br absorption e erro para demais não-textos. Não aceitar
Content para script/style: typed constructor e raw_text guard medidos seguem
vigentes. Wrapper privado de título antigo pode delegar com chain default;
produção fornece chain ativa. Nunca escapar glyph duas vezes.


## Carrier transparency repair — 2026-10-13

### Measurement before decision

`03_infra/src/export/html/encoding.rs:12` and `:31` reject carriers before
reaching their textual children. The verified vanilla accepts the literal title
fixture that Tekt rejects; evidence is in
`diagnosticos/auditoria-producao-tekt-2026-10-13/html-probes.json.gz`.

Raw/title projection must traverse Source.body, ShowRecipe.body, Located.body
and Realized.visual without executing recipes or serializing original provenance.
These are internal carriers, not nontext HTML children. Styled retains the active
chain in titles. Raw/title nontext-child rejection remains strict after traversal.
Title's absorption of the separator after Linebreak also recognizes wrapped
Linebreak/Space/SpaceFromNewline. Preserve contextual quote state and escaping;
acceptance compares title text, escaping and diagnostics rather than transport
shape. Existing script/style validation is unchanged. Unit REDs cover each
carrier, nested carriers, visual-versus-original, escaped text and wrapped
linebreak separators. This changes no API/default/pipeline phase (ADR-0127).
