# L0 — lang/quotes
Hash do Código: 190ff365

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lang/quotes.rs`

## Consumer evidence

`01_core/src/compiler/lang/quotes.rs:57` locates `fn pair(open: &str, close: &str) -> SmartQuotePair`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Localization and validation

Own pure quote-pair lookup, validation and resolution, using the public types
owned by `00_nucleo/prompts/entities/elements/smartquote.md`.
`DEFAULT_QUOTES` is `“`/`”`; `DEFAULT_SINGLE_QUOTES` is `‘`/`’`.

Primary double pairs: pt/es/it `«`/`»`, en `“`/`”`, de `„`/`“`,
fr `«\u{00A0}`/`\u{00A0}»`. Single pairs: en `‘`/`’`, de `‚`/`‘`.
Uncovered or missing language uses the corresponding English curly pair.
German alternative pairs are single `›`/`‹` and double `»`/`«`.
Exact language lookup does not infer BCP47 regions.

`parse_smartquote_quotes(value, span)` accepts auto/none, string, array or
dictionary. Auto/none selects automatic localization. A string must have
exactly two grapheme clusters; an array must have exactly two strings.
A bare string/array overrides the double pair only. A dictionary accepts only
`single` and `double`; each member is auto/none or a valid pair.
Explicit overrides affect only their member; auto preserves its localized
alternative. Pair resolution remains independent of context; resolve_smart_quote owns
nesting, opening/closing context, apostrophes and primes as ratified below.

Preserve span-anchored diagnostics, including singular/plural forms:
`expected 2 characters, found {n} character{s}`,
`expected 2 quotes, found {n} quote{s}`,
`expected string, found {type}`,
`unexpected key "{key}", valid keys are "double" and "single"`,
`expected auto, string, or array, found {type}` and
`expected auto, string, array, or dictionary, found {type}`.
Regional and nesting parity is not established by this owner.


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

Adicionar helper puro público compartilhado por Paged/HTML:
```rust
#[derive(Debug, Clone, Default)]
pub struct SmartQuoteContext { /* campos privados: stack e caractere precedente */ }
impl SmartQuoteContext {
    pub fn observe_text(&mut self, text: &str);
    pub fn observe_object(&mut self);
    pub fn reset_paragraph(&mut self);
}
pub fn resolve_smart_quote(
    context: &mut SmartQuoteContext,
    element: &SmartQuoteElem,
    chain: &StyleChain,
    effective_lang: Option<&Lang>,
) -> SourceResult<EcoString>;
```
Resolver Option explícita > chain > defaults (double/enabled true, alternative
false, quotes auto). Reusar parse_smartquote_quotes/resolve_smartquote_pair,
idioma efetivo explícito/chain e catálogo vigente; sem locale do processo.
Contexto observa texto lógico final após show, não bytes Rust da fonte original
ou último frame pintado. Manter nesting por tipo, abertura/fechamento,
apóstrofo após letra/objeto e primes após número conforme controles.
Disabled emite ASCII sem alterar stack de aspas habilitadas; observar o texto
emitido como contexto seguinte. Soft wrap não reseta; parbreak reseta;
linebreak representa newline mas mantém nesting. Não implementar bidi novo,
limite arbitrário copiado ou novas tabelas de língua/região. Erros de config
já validados preservam SourceDiagnostic, não panic ou falha silenciosa.
A cláusula histórica 'não implementa contexto' é supersedida para este helper;
par resolver/validação atual mantém sua assinatura/comportamento.
