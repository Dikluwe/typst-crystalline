# L0 — stdlib/text/smartquote
Hash do Código: 0ee33c4d

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/smartquote.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

## Evidence and classification

`01_core/src/compiler/stdlib/text/smartquote.rs:37` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

smartquote accepts zero positionals and named double: Bool=true, enabled: Bool=true, alternative: Bool=false and quotes: Auto|string|array|dictionary=Auto. Positional error retains `smartquote() não aceita argumentos posicionais (recebeu {n})`; unknown named `smartquote(): argumento nomeado inesperado '{name}'`; wrong boolean preserves the parameter-specific bool error.

When enabled, emit a Content::SmartQuote leaf retaining explicitly supplied optional fields, not Content::Styled: public content.func() must retain smart.quote identity. When disabled, emit literal ASCII double/single quote according to double, ignoring alternative/quotes and never updating the layout quoter. Preserve omission versus explicit quotes:auto so inherited overrides can be erased. Set rules continue using Styles. Direct-field/set-chain/default resolution and localized glyph/open-close state belong to the quote/layout owners, not this native.

Use the localized-quotes owner's validation/canonicalization: strings contain exactly two Unicode grapheme clusters, arrays exactly two items, dictionaries only single/double, each Auto/string/array. Preserve `expected 2 characters, found {n} character{s}`, `expected 2 quotes, found {n} quote{s}` and `unexpected key "{key}", valid keys are "double" and "single"`. Explicit quotes take priority over alternative; absent/auto dictionary members select localized pairs. No language read, EvalContext quoter counter or mutable global state. Do not infer unmeasured content.fields()/BCP-47-region behavior from a repr wrapper that happens to be transparent. The historical public-carrier gate is archived, not newly approved by this sanitation.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.


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

Manter assinatura native_smartquote, named-only e diagnósticos/tipos existentes.
Double/enabled omitidos ficam None; explícitos Some(bool). Validar alternative
/quotes com helpers vigentes. enabled:false devolve SmartQuote, não Text.
Nunca ler engine/style no constructor para preencher campos omitidos. Layout
resolve herança; fields expõe só o que foi assente. Sem forçar localização.
