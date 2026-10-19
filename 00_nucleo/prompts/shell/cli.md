# L0 — Shell CLI parsing and semantic serialization
Hash do Código: a0874afb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:923f1c9e4c2fd5934316ae2eef3dbb461c3bef3009de1d8931d951e793dda342
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:79283c9047419768139e0227b9ed7e34146a85706ed47731655736545c95306b
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46
- 00_nucleo/prompts/_nuclei/shell/build-identity.toml sha256:a97f32705be8700feaa6a5c89440ffa49d15c11145ea83744307aefc65a50297
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

**Layer:** L2
**Exclusive consumer:** `02_shell/src/cli.rs`


## Evidence, ownership and classification

The current consumer defines color/output enums at `02_shell/src/cli.rs:199`, command arguments at `:415`, command variants at `:550`, public intents at `:698`, translation at `:830`, semantic serialization at `:1021` and resolution helpers at `:1550`. These observations precede the contracts below; they do not establish new product decisions or global vanilla parity.

L2 owns syntax, argument translation, presentation choices and memory-only serialization. It reads argument/environment/TTY configuration but does not inspect filesystem paths, discover fonts, compile, lay out or export. L4 consumes typed `RunIntent` values rather than clap's private `Args`. Raw tri-state/optional overrides must remain distinguishable so their productive owner can combine CLI and document options.

CLI acceptance, diagnostics and serialized public value morphology are observables. Internal enums, arithmetic, generators and pipeline mechanics need not match vanilla. An observed parser quirk is not, by itself, evidence of intentional language design. Unresolved normative/consumer disagreements are retained in `lab/results/construction/diagnostics/sanitization-shell-history.md`; this translation authorizes no change of behavior.

## Commands, compatibility, help and identity

The command tree is named `typst`, with about literal `Typst compiler (crystalline)`. It exposes `compile` (accepted alias `c`), `watch` (visible alias `w`), `eval`, `fonts`, `completions`, `info` and `init`. `query` remains accepted but hidden from principal help; it is deprecated, not an alias of eval.

Preserve `typst INPUT [OUTPUT] ...` as implicit compile. The normalizer scans argument tokens for recognized command/alias names and `-h`, `--help`, `-V`, `--version`; if any is present, it does not insert compile. Otherwise it inserts `compile` after the executable name. Empty argument vectors are left unchanged. Do not describe this implementation as inspecting only the first positional token; collisions with command-looking argument values are not permission to change it.

`parse() -> RunIntent` uses clap's parser and translates the selected command. Help/version terminate successfully with exit 0; malformed/unknown arguments and closed-enum values exit 2 with clap diagnostics before productive execution. Missing compile input has the diagnostic body `the following required argument was not provided: <INPUT>`.

`command() -> clap::Command` returns the same derived tree used by parsing, including hidden commands, for completion generation.

Public version derives from `PARITY_VERSION`, not the workspace crate version, and appends ` ({short_commit})`. Compile-time `TYPST_COMMIT_SHA` supplies the first eight bytes (or shorter full value); absent identity displays `unknown commit`. `build_commit() -> Option<&'static str>` preserves the complete compile-time identity. Build capture belongs to `00_nucleo/prompts/shell/build.md`; parity version belongs to `00_nucleo/prompts/entities/version.md`. No runtime Git lookup or assumption that a version string proves oracle provenance is permitted.

## Global options and configuration

`--color auto|always|never` is global, default auto. `ColorWhen` is the closed public enum Auto/Always/Never. `resolve_colored_with(choice, no_color, is_tty) -> bool` applies explicit always/never first; in auto, presence of `NO_COLOR`, even empty, disables color; otherwise stderr TTY status decides. The formatter itself never discovers this state.

`--cert PATH` is global. Resolution is explicit flag, then `TYPST_CERT` presence, then no custom CA. Preserve optional path presence rather than treating an empty value as automatically absent. Pass the selected path to commands that resolve packages/world configuration; do not make it an unused decorative flag or disclose certificate path/bytes in info. TLS interpretation belongs to the pinned custom-CA nucleus and productive I/O owners.

Features are explicit repeatable/comma-delimited `--features html,a11y-extras,bundle` on compile/watch and eval only. Unknown names exit 2. Default is an empty set; repetitions/order are idempotent. Each name enables only its corresponding typed feature. Selecting HTML/bundle output does not implicitly enable that feature. `TYPST_FEATURES` is not a CLI activation source.

## Compile and watch common arguments

`typst compile INPUT [OUTPUT] [OPTIONS]` and `typst watch INPUT [OUTPUT] [OPTIONS]` share:

| Argument | Current parsing/resolution contract |
| --- | --- |
| `-o FILE`, `--output FILE` | Flag wins over positional OUTPUT; otherwise derive input.with_extension("pdf"), except output-less explicit bundle uses input.with_extension(""). |
| `--root DIR` | Explicit flag wins over `TYPST_ROOT`, then nonempty input parent, then `.`. No path-existence validation in L2. |
| `--font-path DIR` | Repeatable; split each value and `TYPST_FONT_PATHS` on `:` on Unix or `;` on Windows. Explicit flag occurrences replace the whole environment list, not append to it. Filter empty slots; preserve order and nonempty duplicates. |
| `-f FORMAT`, `--format FORMAT` | Closed pdf/png/svg/html/bundle enum. Explicit format wins over the selected explicit output's case-insensitive pdf/png/svg/html extension, then PDF. A .bundle extension alone does not select bundle. |
| `--full-error` | Boolean, default false; pass it through for the extended show-rule recursion hint. |
| `--timings-json FILE` | Optional path, absent None; productive owner writes pipeline timings. |
| `--document-id UUID` | Explicit flag wins over `CRYSTALLINE_DOCUMENT_ID`, then None. Convert accepted UUID to 16 bytes in L2. |
| `--input chave=valor` | Repeatable raw key/value strings; split on the first equals sign. Nonempty key required; empty value and additional equals signs are allowed. Default empty vector. |
| `--compact` | Boolean, default false: verbose PDF remains production default; true selects compact PDF. Direct PNG/SVG/HTML outputs are unaffected; bundle PDF forwarding is recorded below. |
| `--pretty[=true|false]` | Tri-state Option<bool>: absent None, bare true, explicit equals value parsed as bool. Never erase absence in L2. |
| `--ppi PPI` | Option<f64>, default None, not a pre-applied 144. Ordinary f64 syntax; productive owner decides meaningful resolution/validation. |
| `--pdf-tagged[=true|false]` | Tri-state Option<bool>, absent None, bare true; explicit value uses equals. |
| `--no-pdf-tags` | Legacy boolean, absent false; explicit modern pdf-tagged choice wins when both occur. |
| `--pdf-standard LIST` | Closed comma-delimited PDF-standard names, translated to raw Vec<PdfStandard>; absent empty. Do not validate cross-name conflicts in L2. |
| `--oracle-pdf` | Boolean, absent false; diagnostic-only PDF path, not a production output format. |
| `--pages RANGES` | Optional comma-delimited physical one-based page ranges, converted to typed PageRanges; absence stays None. |

An explicitly selected non-bundle format does not change the default output filename's .pdf extension. Format selection and filename derivation are distinct contracts. Standard output/path interpretation belongs to the publication owner, not these pure path helpers.

Font-path empty values are legitimate slots, unlike a flag missing its required argument. An explicitly empty flag still replaces an environment list before filtering. L2 does not convert empty slots to cwd. Unreadable-path warning conformance is a recorded pending decision, not repaired here.

UUID's documented supported forms remain canonical 8-4-4-4-12 hexadecimal and unhyphenated hexadecimal. The consumer also strips all hyphens before checking length/digits, accepting noncanonical hyphen placement. Do not tighten or endorse that broader grammar without the recorded behavioral decision. Rejected values exit 2 with the exact literal:

```text
error: invalid document ID: expected a UUID (e.g., f81d4fae-7dec-11d0-a765-00a0c91e6bf6)
```

Malformed input pairs exit 2 with `error: invalid input '{raw}': expected the form key=value`. Do not parse values as numbers/booleans or silently deduplicate the raw vector; SystemWorld owns construction of sys.inputs.

For pretty/pdf-tagged, a following space-separated token is not the optional boolean value: it is parsed under ordinary positional rules. This differs from the prior unconditional claim that spaced pdf-tagged values are rejected; resolving that claim is pending, not a reason to alter the parser.

## Page ranges and PDF standards

Range tokens accept N, A-B, A- and -B, with endpoint whitespace trimmed and positive integer physical indices. A single page is an inclusive equal-bound range; no counter/label remapping occurs. Preserve these parser messages:

| Condition | Message |
| --- | --- |
| Empty token | `page range must not be empty` |
| Literal zero | `page numbers start at one` |
| Invalid endpoint | `not a valid page number` |
| Both endpoints absent | `page range must have start or end` |
| Descending interval | `page range must end at a page after the start` |
| Multiple hyphens | `page range must have a single hyphen` |

Accept standard names exactly: `1.4`, `1.5`, `1.6`, `1.7`, `2.0`, `a-1b`, `a-1a`, `a-2b`, `a-2u`, `a-2a`, `a-3b`, `a-3u`, `a-3a`, `a-4`, `a-4f`, `a-4e`, `ua-1`. Unknown names exit 2. Preserve typed versions, archival/accessibility labels and raw order.

PDF composition combines CLI overrides with document options: explicit CLI values precede document settings, which precede defaults. Preserve None/empty overrides so document settings remain observable. Modern tagged overrides legacy no-pdf-tags; absent both delegates to document/default. A nonempty standard override replaces the document's whole set rather than forming a union. With no document override, pretty is false, PNG PPI is 144, PDF version is 1.7 and automatic PDF tagging resolves true unless page selection or standard constraints require otherwise. Document-option precedence belongs to `00_nucleo/prompts/entities/document_format_options.md` and the format-specific pipeline owners, not L2.

For PDF, pages plus explicit tagged=true exits 1 before compilation with `error: cannot enable PDF tags when exporting a page range`. Page selection without an explicit tagging decision implies false. Accessible standards a-1a/a-2a/a-3a/ua-1 cannot resolve to untagged output; retain `error: cannot disable PDF tags when exporting a {name} document` and, when applicable, ` = hint: using --pages implies --pdf-tagged=false`. Conflict validation belongs to the domain standard resolver and productive composition, not clap.

This is a language/selection contract, not certification that every selected archival/accessibility profile satisfies full PDF/A or PDF/UA conformance. Pages/standards/tagging/oracle are neutral for direct PNG/SVG/HTML outputs; PNG/SVG multi-file page selection is not promised. The bundle branch forwards pages, tagged, compact, PPI and pretty to relevant generated assets, while its CLI standard override remains absent because the resolver runs only in the direct PDF branch. This exceeds older PDF-only descriptions and requires the recorded scope decision; it is not silently ratified here.

## Command-specific options

Compile alone accepts `--html-serialization crystalline|vanilla`, default crystalline. It selects the HTML escaping profile, not target or feature. The prior requirement limits effect to direct HTML output; current composition also forwards the choice to bundle HTML assets. That scope disagreement remains pending in the diagnostic, without granting a new default or changing either path. Vanilla selection is explicit; it is not a silent new default. Watch does not accept this option and carries Crystalline.

Watch adds `--no-serve` and `--no-reload` (false by default), plus `--port PORT` (Option<u16>, absent None). Serving/reload/port search are owned by `00_nucleo/prompts/shell/watch.md` and cycle composition by `00_nucleo/prompts/wiring/watch.md`; L2 starts no server.

`typst eval EXPRESSION [--in INPUT] [--format json|yaml|raw] [--pretty] [--features ...]`: expression is code text, format defaults json, pretty defaults false, input defaults None. Contextual input uses `--in`, not another positional; `--in -` lets L4 read stdin. Preserve ordinary JSON/Raw behavior with or without document context. No eval --target/root/font-path/sys-input options are accepted by this consumer.

`typst query INPUT SELECTOR [--field FIELD] [--one] [--format json|yaml] [--pretty]`: JSON default, absent field None, one/pretty false. INPUT `-` selects stdin in L4. It has no features or --in argument. Its deprecation warning is emitted by L4 after compilation diagnostics or successful serialization. The pure query_deprecation_hint function provides the specific eval replacement; acquisition/selection/serialization failures retain their measured boundaries. The warning message stays `the `typst query` subcommand is deprecated` and the generated hint follows the ratified contract below.

`typst fonts [--font-path DIR ...] [--ignore-system-fonts] [--variants]`: font-path handling is shared; include_system is the inverse of ignore-system-fonts, default true; variants false. Presentation belongs to `00_nucleo/prompts/shell/fonts-command.md`.

`typst completions SHELL` requires one of bash/elvish/fish/nushell/powershell/zsh. The typed public selector is `CompletionShell`; generation belongs to `00_nucleo/prompts/shell/completions.md`.

`typst info [-f json|--format json] [--pretty]` has no format by default (human); pretty false and requires format, otherwise exit 2. No unsupported info format or feature selection is advertised. Presentation belongs to `00_nucleo/prompts/shell/info.md`.

`typst init TEMPLATE [DIR]` carries template text, optional directory and selected cert path. Materialization belongs to `00_nucleo/prompts/shell/init.md`.

## Public carriers and pure helpers

Preserve the closed `RunIntent` variants Compile/Watch/Eval/Query/Fonts/Completions/Info/Init and their corresponding public structs. They remain data, not process handles, trait objects or executable callbacks.

`CompileIntent` fields are features, input, output, output_format, html_serialization, root, font_paths, colored, full_error, timings_json, document_id, inputs, compact, pretty, ppi, pdf_tagged, no_pdf_tags, oracle_pdf, cert_path, pages and pdf_standard. Preserve their current path/domain/boolean types, Option distinctions and vectors. `WatchIntent` wraps compile and no_serve/no_reload/port.

`EvalIntent` contains expression/input/format/pretty/colored/cert_path/features; `QueryIntent` input/selector/field/one/pretty/format/colored/cert_path; `FontsIntent` font_paths/include_system/variants; `CompletionsIntent` shell; `InfoIntent` format/pretty/cert_path; `InitIntent` template/directory/cert_path. Do not introduce a field or change its public type during documentation sanitation.

`OutputFormat` is Pdf/Png/Svg/Html/Bundle; `EvalFormat` Json/Yaml/Raw; `QueryFormat` Json/Yaml; `InfoFormat` Json; `HtmlSerialization` Crystalline/Vanilla. Preserve public Copy/Eq/Debug/ValueEnum traits where currently implemented.

Keep pure resolution APIs and their argument types: `resolve_colored_with`, `resolve_output_with`, `resolve_output_format_with` and `resolve_root_with`. The output-format helper takes an explicit default supplied by its caller; compile supplies PDF. Parsing/world validation stays separate.

## Eval/query serialization

Public serialization entrypoints remain `serialize_eval`, `serialize_query` and `serialize_query_with_format`, returning Result<Vec<u8>, String> without I/O. The legacy query wrapper delegates to JSON. Share semantic projection rather than formatting Rust Debug.

Raw accepts only Value::Str and Value::Bytes, returning their exact bytes without newline. Other values yield this exact body, with a newline between its lines:

```text
cannot print {public_type} in raw format
hint: `--format=raw` only supports strings and bytes
```

Structured projection preserves none as null, booleans, integers, finite floats, strings, effective symbol graphemes, recursive arrays and insertion-ordered dictionaries. Nonfinite floats become null in JSON and .nan/.inf/-.inf in YAML. Bytes become the string `bytes(N)`, not a byte array. Other nominal public values use the core's morphological repr serialization, never Rust Debug, nominal erasure or a shell-owned duplicate repr table. Stroke serialization is covered by this same public repr contract.

JSON is compact or pretty according to the selected flag and ends with newline. YAML recursively emits valid structured values with two-space indentation and a final newline; pretty does not change YAML values/output. String quoting must preserve values and distinguish scalar-like strings from booleans/null/numbers.

LocatedContent with fields present uses its authoritative ordered snapshot recursively, adding func with the element's public name. It must not synthesize extra render fields, fall back field-by-field, leak selector metadata as an explicit label, or serialize locations. Absent snapshots and raw Content use the existing legacy public projection; an incomplete legacy fallback is not a promise to expose every element's complete language fields.

Legacy Content projection preserves func and its public recursive fields. Label wrappers add the public label notation only to the projected object. Important cases:

- Text: text; Sequence/MathSeq: children; Paragraph/SmallCaps: body; Styled: body.
- Heading: level and depth equal level, offset 0, numbering null, supplement obtained from the core heading-language owner with None, outlined, bookmarked as bool or auto, hanging-indent auto, and body. Do not duplicate the English supplement table or infer another locale without a style chain.
- Figure: body, alt null, placement null, scope column, kind (default image), caption, supplement, numbering 1, gap 0.65em, outlined true and its kind-specific counter repr. Caption includes separator content `: `, body, kind, supplement, numbering and counter. Preserve the existing Figure/Table/Listing supplement selection.
- Equation: block, numbering null, number-align end + horizon, supplement Equation, alt null and body.
- MathAttach: func attach, base plus public t/b/tl/bl slots. Primary t/b use the corresponding right slot only when Omitted; ExplicitNone blocks fallback and serializes null; Present serializes even empty content. Omitted fields are absent, not null. Do not infer omission from empty content or collapse it with explicit none.
- Shape: its rect/ellipse/line/polygon func, width/height as auto or serialized lengths, fill and optional stroke. MathText has func text; MathIdent has func symbol.
- Metadata: value; Quote: block/quotes/body/nullable attribution. Other legacy elements expose the current body fallback only when available.

Heading supplement policy belongs to its L1 owner, not an English successor literal in L2. The pinned content-snapshot and math-slot nuclei remain authoritative; this prompt must not weaken them to bless legacy fallback gaps.

Query projects the same public values. With --one, require exactly one element before looking up --field; otherwise `expected exactly one element, found {N}`. A missing field for that single result is `no such field found for element`. Without --one, a field request filters out elements lacking the field and emits the remaining values as an array; without field, emit an array of projected elements. Apply JSON/YAML serialization after cardinality/field selection.

## Exit and output integration

Productive composition is referenced at `00_nucleo/prompts/wiring/compile.md`, `00_nucleo/prompts/wiring/eval.md`, `00_nucleo/prompts/wiring/query.md` and the command-specific shell/wiring owners; those are not additional consumers of this L0.

Successful finite commands exit 0. Language evaluation/compilation errors, invalid query cardinality/fields and structured/raw serialization errors exit 1. Argument errors and operational source/stdin/stdout/timings failures exit 2. Watch is continuous: failed compilation reports status/diagnostics and waits for another change rather than being described as a successful finite exit.

Preserve `error: {error}` for eval/query serialization failures; `error: failed to write eval output: {error}` and `error: failed to write query output: {error}` for stdout failures. Preserve existing source-load/stdin diagnostics under their productive owner.

HTML/bundle export warnings, feature-required errors, page/tag/standard conflicts and full-error hints retain their source-produced literals. No translation of specification prose authorizes translation, suppression or rewording of these messages.

Preserve these experimental warning bodies and their final blank line; each is emitted for its corresponding target, not as a combined warning:

```text
warning: html export is under active development and incomplete
 = hint: its behaviour may change at any time
 = hint: do not rely on this feature for production use cases
 = hint: see https://github.com/typst/typst/issues/5512 for more information

warning: bundle export is experimental
 = hint: its behaviour may change at any time
 = hint: do not rely on this feature for production use cases

```

Missing feature selection exits 1. Preserve the diagnostic bodies below, including literal backticks. HTML retains hints `html export is under active development and incomplete` and `see https://github.com/typst/typst/issues/5512 for more information`; bundle retains `bundle export is experimental`:

```text
html export is only available when `--features html` is passed
bundle export is only available when `--features bundle` is passed
```

Bundle stdout refusal remains `error: cannot write bundle to standard output`; watch stdout refusal remains `error: watch requires a file output; stdout is not supported`. Do not substitute implicit feature activation for these errors.

## Acceptance and exclusions

Check command/alias inventory, hidden query, legacy invocation, help/version status, each closed selector, global color/cert precedence, font empty-token behavior, output/root/format precedence and all optional/tri-state carriers. Cover accepted and rejected input/page/UUID forms without silently resolving the recorded grammar discrepancies.

Check structured nested values, nominal repr, symbol/bytes/nonfinite handling, authoritative snapshots, labels, typed math-slot presence, generic query fields/cardinality and raw byte preservation. Check successful outputs and failure channels/status through the independently owned command envelope tests.

Do not assert global shell, HTML/bundle, query fallback or PDF-standard conformance. Unsupported argument families remain unadvertised. Public contract/default/pipeline/compatibility changes require ADR-0127; ownership and canonical links follow ADR-0129/0130.

## `--pdf-standard` em PDF de bundle — proposta pendente de gate ADR-0127 (2026-09-30)

### Medição anterior à decisão

Com o binário cristalino debug construído da working tree de HEAD
`17f13b1df79b87ea065ee62e78164b18e477da3d`, a mesma ausência de título
compila sem a flag em PDF isolado e em `document("one.pdf")` dentro do bundle.
Com `--pdf-standard=ua-1`, o PDF isolado falha com
`PDF/UA-1 error: missing document title`, mas o bundle publica `one.pdf`.
A sonda com timestamp, SHA-256 do binário/entradas e comandos está em
`lab/results/construction/diagnostics/pdf-standard-bundle-2026-09-30/results.json`.
`04_wiring/src/compile.rs:155-219` só constrói o override em `OutputFormat::Pdf`,
enquanto `04_wiring/src/compile.rs:305-325` já o transporta para
`BundleCompileOptions` e `03_infra/src/pipeline/bundle.rs` já o resolve por
entrada PDF. A inferência é uma lacuna no fio CLI; seria refutada se a
flag alterasse o standard efetivo do PDF interno, mas a sonda UA-1 não o mostra.
A diferença é semântica de seleção do output, não igualdade de bytes PDF.

### Contrato proposto, ainda não vigente

Se o dono autorizar, `--pdf-standard LIST` não vazio seleciona o mesmo conjunto
explícito para todos os PDFs gerados dentro de bundle, precedendo o
`format.pdf(standard: ...)` de cada documento; lista ausente preserva a
seleção documental e os defaults por entrada. O override nunca atinge
HTML/SVG/PNG/assets. Conflitos entre nomes da própria lista são rejeitados
antes de criar `SystemWorld` nos targets PDF direto e bundle, mesmo quando
um bundle não contém PDF. O gate CLI-only de standard acessível versus tags
continua restrito ao target PDF direto; no bundle, cada entrada PDF resolve
as opções e valida o conflito na pipeline L3, para não rejeitar antecipadamente
entradas não PDF. O bundle publica tudo ou nada se uma entrada falhar.

Aceitação proposta: sem flag, conservar o resultado atual; com UA-1, PDF de
bundle sem título falha e nenhum arquivo é publicado; com título, o PDF
interno declara UA-1. Uma entrada não PDF não recebe esse standard. Esta é
mudança de comportamento explícito do produto e **não autoriza código** até
confirmação do dono pelo gate ADR-0127. Após confirmação, substituir a frase
anterior sobre override ausente e ressellar L0/consumer antes de RED→GREEN.

## Output obrigatório para stdin — 2026-10-06

### Medição antes da decisão

`lab/results/construction/diagnostics/plano-debitos-2026-10-06/before.json.gz` conserva
HEAD/working tree/UTC, fontes, comandos e hashes dos binários. Compile/c -
sem output aceita e escreve -.pdf no Tekt; o pin exige OUTPUT e sai 2.
`lab/typst-original/crates/typst-cli/src/args.rs:351-355` declara
required_if_eq(input, -), comprovando intenção. `02_shell/src/cli.rs:922-935`
deriva output mesmo nesse caso. O stdin implícito é extensão já estabelecida
do Tekt: vanilla rejeita o subcomando -, e essa forma não é sonda de paridade.

### Contrato e aceitação

Antes de resolver formato/output ou construir RunIntent, input exatamente -
sem output positional nem -o/--output sai 2 com corpo de diagnóstico
`the following required arguments were not provided:\n  <OUTPUT>`.
Usar a árvore clap existente para apresentar usage/help, sem fabricar opções
vanilla ausentes no Tekt. Nenhum stdin, World ou arquivo é adquirido em L2.
Isso inclui c, compile e a forma implícita já suportada; vale nos formatos
pdf/svg/html/bundle. Output positional ou flag continua suficiente; a flag
vence positional na mesma ordem atual. Entrada física continua derivando
output quando ausente. Não alterar assinaturas, flags ou helpers públicos.

Testar black-box rejeição sem -.pdf/árvore bundle e stdout vazio; aceitação
positional/flag/stdout e prioridade flag em ambas ordens; preservar o default
para entrada física. Correção de paridade ADR-0127 em fluxo contínuo.
Usage difere legitimamente pela superfície de flags já existente; conservar
stderr bruto na coleta, sem declarar identidade integral desse envelope.

### Envelope de usage do erro — refinamento medido

final.json.gz registra Usage: compile [OPTIONS] [INPUT] [OUTPUT] no primeiro
reparo. Essa apresentação perde o nome real do executável e indica opcionais
que o erro exige. Construir o erro a partir de clone do subcomando existente,
com bin_name typst compile e input/output required apenas nessa árvore de
apresentação do erro. Não alterar a árvore usada para parse/help/completions
ou aceitar/rejeitar novas formas. Exigir Usage: typst compile e <INPUT> <OUTPUT>.
As opções suportadas continuam as do Tekt; não fabricar creation-timestamp do
oracle nem declarar stderr integralmente igual enquanto o usage divergir.

## Hint específico de depreciação de query — gate confirmado ADR-0127

Evidência antes da decisão: plano-debitos-2026-10-06/before.json.gz compara
query-clean/query-warning/query-error no pin 9f2b6e871. O Tekt usa query(...)
e --in ... constantes; o pin fornece a expressão exata. Upstream
query.rs:126-161 compõe field/one e shell_escape::escape. É apresentação de
L2, não domínio de L1 nem algoritmo de L4.

Adicionar a função pública pura neste mesmo consumer:
`pub fn query_deprecation_hint(intent: &QueryIntent) -> String`.
Devolve `use ` seguido do comando entre backticks e ` instead`, sem prefixo
warning/hint, newline ou I/O. Constrói query(selector) com texto do selector.
Sem one/field, conserva expressão. Com one, acrescenta .first(); com field
sem one, acrescenta .map(it => it{access}); com ambos, .first(){access}.
access é .FIELD para compiler::lexer::is_ident(field), ou .at(REPR) caso
contrário; repr usa a representação existente de Value::Str, preservando
escapes. Shell-escape da expressão usa shell-escape =0.1.5 em L2, mesmo pin
upstream. Com input -, não acrescenta --in; com arquivo, acrescenta --in e
sua representação display vigente upstream. Não alterar parser, QueryIntent,
serialização dos valores, enums, ou ambiente. A sonda de caminhos com espaços
será conservada; este gate não autoriza corrigir ambiguidades do hint upstream
por hipótese. Função usa somente dados recebidos e utilitários puros.

### Pureza do escape — refinamento interno medido

shell-escape 0.1.5 src/lib.rs:19-24 consulta MSYSTEM em targets não Unix no
wrapper escape. Para preservar a função pura ratificada, escolher o helper
unix::escape em cfg(unix), e windows::escape nos demais targets, sem consultar
ambiente. Esta coleta certifica Unix; conformance MSYS/Windows não foi medida.
O repr de Value::Str é obtido pela fachada pura repr_value_for_serialization.

Gate confirmado pelo humano em 2026-10-06 (resposta confirmo), com candidatos
e hashes preservados em lab/history/construction/records/00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json.

## Reparos independentes do gate de aspas — 2026-10-06

Medição anterior à decisão: smartquote-2026-10-06/{before,fields-before}.json.gz,
com pin/HEAD/working tree/UTC e binários exatos registrados.

Adaptar o nome público Content::SmartQuote para smartquote na projeção de
JSON/YAML, inclusive o envelope de campos Some, sem mudar elem_name interno,
structs, campos ou fase. O snapshot before mede smart.quote em eval-native-default
contra smartquote. Nesta fatia só o nome é certificado; presença de double,
enabled e quotes pertence ao gate smartquote-semantica. Teste deve conferir
nome sem inventar campos ausentes. Correção de apresentação interna ADR-0127.


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

SmartQuote JSON/YAML usa nome canônico e só propriedades Some, preservando None
omitido. Double markup Some(bool) aparece; enabled:false programático aparece
sem transformar tipo. Quotes Value canônico via tipos existentes; map não
inclui propriedades herdadas de Styles. Some captured fields permanecem
exatos; não fabricar default. Não alterar Styled geral nesta rodada: a
morfologia pública completa dos wrappers Styled é dívida independente
medida em eval-markup-disabled, sem certificar igualdade global desse caso.

## D07 — correção de morfologia de matriz, fluxo ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
lab/results/construction/diagnostics/matrix-morfologia-2026-10-06/before.json.gz guarda
UTC/diff/fontes/binários e casos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Mat raw/query sai como {func:math.matrix} sem rows; getter/métodos perdem campos
e func-equality retorna false. Fonte upstream math/matrix.rs:91–221 declara
propriedades e rows. Tekt MathMatrixElem conserva rows/explicit em math_matrix.rs:45–52,
mas Element.get_field é default; content_methods.rs:151–191 não enumera mat;
cli.rs:1154–1348 cai no fallback body. É morfologia/semântica da linguagem,
não exigência de estrutura Rust ou bytes PDF; nenhuma intenção foi inferida.

### Responsabilidade exclusiva

Serializar MathMatrix com func mat e somente campos assentes na ordem delim,align,augment,row-gap,column-gap,rows. Usar getter do domínio e value_to_semantic existente recursivamente, conservando formas text/symbol/sequence de cada célula. Padding Empty projetado pelo getter torna-se sequence children []. Não serializar corpo por plain_text nem assar defaults/render; manter JSON/YAML, cardinalidade, snapshot Some e mensagens existentes.

### Aceitação

RED→GREEN e coleta bilateral de fields/has/at/func, JSON/query, padding e
presença; build/lint e regressões D05/D06/query. Gate contínuo: contratos
públicos de tipos/traits/assinaturas e fase ficam iguais. Não certificar
mat selectors, render, Styled geral ou outros elementos não medidos.

## D08B2 — presença lexical ratificada em 2026-10-07

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz e
presence-refuters.json conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree, diffstat, UTC, binários e argumentos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Upstream model/table.rs:734–778 possui campos Smart/Option; o Tekt
entities/elements/table_cell.rs:40–51 conserva apenas Option de valor.
X:auto e x omitido colapsam; fill:none e stroke:none perdem presença.
Os refutadores medem auto permitido em x/y/fill/align/inset/breakable,
rejeitado em colspan/rowspan/stroke; none permitido só em fill/stroke.
São observáveis da linguagem, sem obrigação de copiar a estrutura upstream.

### Contrato ratificado e responsabilidade exclusiva

Adicionar projeção TableCell JSON/YAML dos dez campos assentes na ordem body,x,y,colspan,rowspan,inset,align,fill,stroke,breakable. Usar Content.get_field e value_to_semantic recursivo; func cell. Omitted não sai, auto é repr canônico e none explícito é null. Snapshot Some permanece autoritativo; não inventar propriedades resolvidas.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

JSON/YAML/getters existentes projetam somente campos públicos conhecidos do owner; plano/source/stage não vazam como propriedades novas. Snapshots Some e func cell permanecem exatos.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.

### Refinamento interno medido — 2026-10-07

Medição anterior à decisão: lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second}.json.gz, pin/binários/HEAD/estado/fontes/UTC/argv/resultados completos.
after-second/resolved mostra body func:text sem text; CLI content_to_semantic:1153–1182 não desembrulha ShowRecipe, enquanto entities/content/fields.rs:21 já delega campos. JSON/YAML projetam corpos sem wrappers mecânicos ShowRecipe/Located/Realized; transparência não executa receita nem modifica corpo lexical.
A aceitação é bilateral, efeitos uma vez e geometria; não normalizar whitespace nem alterar contratos ratificados.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Serialização semântica Source publica somente body, sem carrier/span ou execução.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Serialização semântica e testes constroem o Value de Stroke pelo From existente. Ler o payload por referência; não imprimir Arc/endereço ou alterar campos, schema, stdout, flags, versões ou defaults.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.

Ratificação Memória 5: o dono confirmou o manifesto SHA-256
6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237
com “confirmo” em 2026-10-08. O gate ADR-0127 deste pacote está satisfeito;
suas cláusulas propostas passam a vigorar nesta materialização.

## Fixtures ativos e resíduos de CI — 2026-10-13

### Medição anterior à decisão

`02_shell/src/cli.rs:1858-1891` compara booleanos de resolve_colored_with com true/false em 14 asserções.

Fontes, owners, diagnósticos, HEAD, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/fixtures-ativos-ci-2026-10-13/before.json.gz`. O registro real da suíte antes da edição confirma ausência do teste matemático. Estas evidências não constituem nova comparação com upstream.

### Decisão e aceitação

Usar assert!(expressão) para true e assert!(!expressão) para false, conservando todos os argumentos, tabelas de casos, nomes e polaridades. A precedência de NO_COLOR, TTY e Never/Always/Auto permanece idêntica. Não introduzir novos testes que apenas espelhem esta substituição.

Manutenção interna de testes (ADR-0127), sem assinatura pública, default ou fase nova. L0 precede código; resselo limitado aos owners envolvidos. Executar suítes shell e infra, compilar todos os targets, verificar formato, linhagem e inventário de Clippy. Contagens de testes não certificam paridade.


## Conteúdo vazio e campos Raw disponíveis — 2026-10-14

### Medição antes da decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/content-serialization-2026-10-14/before/results.json.gz
registra UTC, fontes/owners, status/diffstat, argv e hashes dos executáveis,
contra pin 9f2b6e8715237cb086899a42873660fe744622e8. CLI publica func empty
sem children para []; raw-default/lang publica apenas func raw. JSON.encode
é medido separadamente e perde campos por Source em seu próprio owner.
cli.rs:1152–1192 já desembrulha Source; faltam os ramos Empty/Raw, não essa
transparência. O pin expõe sequência vazia e text/lang explicitamente assentes.

### Decisão e responsabilidade exclusiva

Empty representa sequence com children vazio, também aninhado, em JSON/YAML
e query. A projeção do nome de função deve atravessar os wrappers existentes
Source/ShowRecipe/Located/Realized para aplicar o mesmo nome canônico em
LocatedContent com snapshot Some. Snapshot presente, inclusive vazio, segue
autoritativo e não ganha children/text/lang reconstruídos a partir do render.

Para Raw, preservar text e lang quando Some, que já estão disponíveis no
modelo e foram medidos. Não inferir block:false como assente a partir do bool
nem lang:none como assente a partir de None: falta presença no RawElem atual.
Não completar esses campos com defaults. A etapa raw-presenca completa a
presença de block/lang e demais consumidores depois do gate público próprio;
esta primeira materialização permanece explicitamente incompleta em Raw.
Nenhuma mudança de estrutura pública, assinatura, flag, modo ou fase neste
reparo de apresentação ADR-0127. RED antes de código, revalidação JSON/YAML,
query/snapshot e defaults existentes; guardar saídas sem alegar igualdade de
bytes YAML quando a diferença for representação válida do mesmo valor.

## Presença lexical Raw — ratificada em 2026-10-14

### Medição anterior à decisão

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes/binários e pin 9f2b6e8715237cb086899a42873660fe744622e8. RawElem raw.rs:21–25 perde omissão; native_raw markup.rs:78–136 colapsa ausência/false/none. Campos, igualdade e encoders distinguem esses valores no pin. Trata-se de semântica/morfologia, não mecânica Rust.

### Responsabilidade e aceitação

Projeção CLI Raw usa getter para text,block,lang conhecidos nessa ordem em JSON/YAML/eval/query. Snapshot Some permanece exato, inclusive vazio.

Manifesto aprovado b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45, recibo em lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json. Esta seção completa apenas a presença anteriormente incompleta de Raw e supersede as cláusulas de ausência de máscara. Preservar contratos alheios; sem fase/default/render novo. Validar testes RED→GREEN, suíte, build e linhagem.


## Campos públicos de OutlineEntry na projeção legada — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/hashes/diffstat/fontes em before.json.gz e shell-before.json.gz do diagnóstico tekt-v16-introspect-walk-2026-10-16. A última medição contra vanilla pinado 9f2b6e871 confirma body(), índice bare/where, ordem e leituras state/counter embutidos corretos no Tekt. O subcomando query --field level ainda devolve [] para a entry direta e [1] no misto, versus [1] e [1,1]. cli.rs:1121 filtra valores sem campo; content_to_semantic em :1185–1400 não tem ramo OutlineEntry e o fallback body não fornece level/element/fill. Snapshots sintetizados Some já contêm esses campos; o valor direto sem snapshot usa a projeção legada incompleta. Isto é defeito de apresentação, não nova perda de cardinalidade no índice.

### Responsabilidade exclusiva e aceitação

Acrescentar apenas o ramo Content::OutlineEntry(entry) na projeção legada: level inteiro por entry.level as i64 (mesma representação já fornecida pelo getter do domínio), element por content_to_semantic(&entry.element), fill por conversão recursiva do Some e Null para None. Conservar o func canônico entry já existente, ordem level/element/fill, transparência Source/Located/Label dos caminhos atuais, JSON/YAML e seleção --field/--one. Não criar snapshot L1 só para contornar esta omissão nem alterar Some: snapshot presente, inclusive vazio, continua autoritativo e não recebe defaults/campos pelo fallback. Serialização aninhada conserva o converter vigente; esta correção não certifica todos os campos de outras famílias.

Testes co-localizados antes do ramo: valores crus e wrappers com body morfológico e fill:none; query LocatedContent(None) com label e --field level em JSON/YAML e --one; Some(empty)/Some(level distinto) permanecem exatos. RED funcional real deve falhar nos dois primeiros testes e manter o controle Some passando. Repetir as nove sondas CLI sem relaxar seus esperados, incluindo query --field e dados embutidos. API externa, structs, parser, flags/defaults, fases e política de snapshots ficam iguais; tabela interna/paridade em fluxo ADR-0127. Resselo somente deste par adicional (dez owners totais no lote). Gates completos: build/workspace/fmt/Clippy estrito/Rustdoc estrito/linhagem estrita/reversa/SARIF diferencial sem mudança extra; 12 GiB, swap zero, um job. Sete V19 prescritos e três mensagens existentes ajustadas, seis V16 removidos; os três V16 já presentes deste consumer não são silenciados. Sem novo V20, commit, push ou release.

## Completar projeção da caption existente — 2026-10-17

Medição anterior à decisão: caption-shape-before.json e caption-after.json em
lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17 fixam fontes, UTC,
binários, HEAD/working tree de validation-result.json e pin
9f2b6e8715237cb086899a42873660fe744622e8. Projeção de caption já produz o
carrier, mas sua CLI imprime só func:figure.caption; body/getters/fields estão
ausentes. json.encode tem o mesmo defeito. Metadata de kind rejeita a função
de caption válida. O pin em model/figure.rs:510 declara caption Locatable e
em :511-594 declara position,separator,body e kind/supplement/numbering/counter
sintetizados públicos, com figure_location interno. Não sintetizar esses dados
nesta correção; a função de elemento já existe desde a aprovação P1473.

Responsabilidade exclusiva e aceitação:

Mapear FigureCaption para func público caption; projetar position,separator,
body conhecidos pelo getter na ordem declarada, recursivamente com
value_to_semantic. Não usar figure_caption_to_semantic, que sintetiza defaults
do objeto Figure e tem outro contrato. Não alterar a projeção legada de Figure
nem snapshots Some. JSON/YAML partilham a projeção; não duplicar validação L1.

Correção interna/paridade em fluxo contínuo ADR-0127. Não há novo campo/trait/
assinatura Rust pública, fase, modo ou default. L0 antes de testes/código, RED
real de getters/metadados/encoder/CLI, GREEN e corpus externo sem relaxar
expectativas de corpo/nome. Revalidar todos os gates da etapa sob 12 GiB, swap
zero, jobs1. Resselo só dos owners após V15/V26. FG1/SY1 permanecem incompletos
conforme follow-ups.md; nenhuma API pendente está autorizada por esta seção.
