# Prompt L0 — `entities/elements/raw` — `RawElem`
Hash do Código: 91c80bf8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/raw.rs`
**Architecture:** ADR-0105 model D; Raw is nonlocatable.

## Evidence before lexical-presence contract

`lab/results/construction/diagnostics/content-serialization-2026-10-14/{before,raw-controls}/results.json.gz`
records HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, uncommitted tree/source/
UTC/argv/binary hashes and upstream 9f2b6e8715237cb086899a42873660fe744622e8.
The old RawElem text/lang/block lacked presence: raw("x") and explicit
block:false compared equal locally but differed upstream; explicit lang:none
was lost. fields/has/at and JSON/YAML confirmed the distinction. Pinned
text/raw.rs:289/324/361 declares fields; markup/display establish block and
supplied lang. This is language behavior, not mandated upstream storage/Packed/
Rust equality. Human approval is recorded at
`lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json`,
manifest b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45.

## Raw contract

RawElem derives Debug/Clone/PartialEq/Hash with public text:EcoString,
lang:Option<EcoString>, block:bool, set_fields:u8. Public RAW_SET_BLOCK:u8=1
and RAW_SET_LANG:u8=2 belong here; other bits are reserved and repository
constructors leave them zero. Content::Raw carries Arc.

text is always known. A block bit makes even false known. A lang bit makes
Some a Str and explicit None a Value::None; a missing bit means Unset.
Existing Element::get_field returns text/known fields and None for omitted/
unknown fields, superseding the earlier default getter. No trait extension
or field-type change. plain_text returns text; is_empty defaults false;
both maps are terminal and preserve the mask; kind/payload retain defaults.
Equality/hash/clone and ObservationEq preserve presence because getters and
encoders observe it; measured language-value equality is not mandated upstream
Rust equality.

Content::raw(text,lang,block) preserves signature/visual behavior, marking block
known and lang known only for Some. native_raw sets block presence for supplied
Bool; omission and legacy block:none conversion stay absent. Supplied lang
is known including none. Keep legacy raw() compatibility/casts/diagnostics
as separate unresolved parity issues. No AST-name search or double evaluation.
Content delegates field lookup. content_methods distinguishes Set/Unset/
Undeclared and enumerates known fields in text/block/lang order; CLI/encoding
use the mask/lexical projection and retain exact Some snapshots.

## Separately ratified raw-line carrier

At baselinee7b2ddd44 raw.line was absent; upstream synthesized each inline/
block line including empty lines, with four required constructor arguments.
RawLineElem has number:i64,count:i64,text:EcoString,body:Content.
plain_text returns text; both maps traverse body; get_field exposes exactly
number/count/text/body. Stable dynamic identity is raw.line through
Content::Dynamic, without another closed Content variant. Equality/hash are
structural; incompatible cardinality/types reject.

`lab/history/construction/records/00_nucleo/materialization/typst-passo-1413.md:91`
records explicit human approval 2026-09-15 after contract commit
ed595a305dd6cdd651faf06d91292538a78d95cf. This supersedes the stale pending
RawLine heading. The later Raw-presence receipt itself neither ratifies nor
changes RawLine, layout, fonts, themes or realized-field enrichment.

## Acceptance and limits

Require omitted/false/true, lang omitted/none/string, fields/has/at,
aliases/Source/maps/equality, CLI/stdlib JSON/YAML and Some/None snapshot
RED→GREEN. Check inline/block markup and Bool/Auto presentation without losing
existing rendering or evaluating show twice. Full suite/build,1:1 lineage,
nucleus pins and warning inventory remain required. Real highlighting remains
separately scoped under ADR-0054. Never promote Raw to locatable.

The public mask breaks external struct literals, which explains the original
ADR-0127§2.1/2.4 gate. Its approval permits canonical L0/nucleus/pins then RED
and code within this scope; construction drafts are never active L0 or @prompt
owners. No broader compatibility/parity closure is claimed.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/raw.md` preserves the original evidence and chronology.
