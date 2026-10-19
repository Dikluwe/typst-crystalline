# L0 — Shared text costs
Hash do Código: 8d75e7c9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/text_costs.rs`

## Evidence before decision

`correcao-custos-paragrafo-2026-09-28/before-pdf/` records 66 entries in
initial.json and before/ against historical vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`. text/mod.rs:1507–1564 validates
Dictionary, Ratio per field and partial composition; inline/mod.rs:240 reads
shared-scope rather than per-fragment costs; inline/linebreak.rs:29–30,961–962
multiplies defaults 135/100 by the ratio. Tekt discards costs in eval/rules and
accepts them without constructor transport; paragraph_hyphen/paragraph_select
use fixed defaults. Vanilla stderr retains errors/spans, including accepted
negative costs. Do not infer intention.

The first cost binary's after-first-resultados.json/proveniencia.json records
66 entries; only restore and inline diverge for valid inputs. inline/mod.rs:240
reads shared.get(TextElem::costs); eval/rules.rs retains factoring of Styled
covering all flow material before Par. paragraph_prepare.rs:45–78 rejects the
new channels although they change no glyph attribute, causing incremental
fallback choices to diverge. This refutes absolute exclusion of those channels,
without promoting partial fragments to outer scope or changing pipeline phase.
These historical readings do not establish parity against a newer pin.

## Exclusive cost contract

Own only LineCosts and pure private parse/resolve helpers. FIELDS defines the
four field/channel names. Parsing returns entries for Styles/chain and validates
Ratio in hyphenation/runt/widow/orphan order before remaining keys. Diagnostics
map int/str/bool to integer/string/boolean and retain measured list quoting and
Oxford commas. SourceDiagnostic uses the injected span. No World, Sink,
Layouter, I/O or global state. Resolve each channel independently and multiply
defaults 135/100; do not clamp negatives. Test partial/empty/restored composition,
types, priority and unknown keys. Original cast/transport tests do not claim
widow/orphan influence or performance by themselves.

Global hyphenation/runt consult configuration. Widow/orphan are validated and
transported, with influence limited to the certified regional subset below.
Partial-fragment costs never replace shared costs. The four validated channels
are glyph-neutral in the Styled preparation whitelist. Existing realization
factors a whole wrapper before Par; variable-width support remains limited.
Do not derive cost from glyphs; preserve defaults. This is ADR-0127 internal
parity correction without a new public contract, flag or phase.

Preparation admits only text.costs.hyphenation/runt/widow/orphan. Planning reads
the outer shared scope: preparation's temporary push is restored and does not
change that lookup. Partial inline style does not choose paragraph costs;
whole Styles is factored by existing realization and enters the chain before
Par. Cost differences create no space or word boundary. Preserve physical
widow/orphan limitations and contexts outside the certified subset.

Private is_channel consults only FIELDS. Preparation reuses that table rather
than maintaining a second list or admitting arbitrary prefixes.

## Certified regional widow/orphan boundary

### Evidence before decision

`lab/results/construction/diagnostics/correcao-viuvas-orfas-2026-09-28/before-pdf/`
records 78 entries and before-extra/ records 24 controls, with complete state in
initial.json and before/, against historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`.
page-65-100%-0%-False leaves three lines and one in Tekt, two and two in vanilla;
zero costs agree. Tiny/large-positive show the same vanilla protection.
`lab/typst-original/crates/typst-layout/src/flow/collect.rs:196–229` uses strict
positivity, first two/last two and whole protection of three lines;
flow/distribute.rs:343–364 advances for grouped need only when it fits the next
region. paragraph_emit.rs emits each line without that need. Observable
semantics is line/word distribution across regions, not vanilla frames/Child.
Do not infer intent from the old scope-out.

### Partial supersession and owner boundaries

This ADR-0127 internal parity correction adds no public field/trait/signature,
flag or phase. It supersedes the widow/orphan exclusion only for prepared
paragraphs with uniform vertical metrics, no baseline offset, constant paginated
geometry, and no columns/cells/subframes/notes/floats or active regional
transaction. Other contexts retain fallback and explicit limits. Do not promise
a complete algorithm or performance.

`00_nucleo/prompts/core/compiler/layout/paragraph_pagination.md` owns vertical
certification, grouping and regional admission. It receives already selected
ranges, does not select them again, emit glyphs or duplicate new_page logic.
Certify the whole paragraph before emission: all fragments have size/edges/
leading equal to the outer style (bold/italic allowed when metrics agree),
with finite nonnegative top/descent/leading. Auto bounds need no protection.
Respect initial baseline and existing block-chain collapse before comparing
real need with page_bottom_limit. Calculate next-page space from its resolved
margins. Advance only with material in the current region and a group fitting
the next. Overheight groups retain ordinary progress and complete content;
never loop new_page. These are integration obligations on that exclusive owner,
not permission to duplicate its algorithm here.

This owner's private helpers resolve widow/orphan independently of
hyphenation/runt weights, reuse FIELDS, default Ratio to 1 and test strictly >0.
Planning's default strategy becomes Simple for ordinary paragraphs without
linebreaks to permit preparation for default protection; automatic justify
selection and refused contexts remain. Emission preserves justification,
carriers, the last open line, boundaries and lexical restoration. Notes/columns
are preserved by avoiding incomplete protection on those paths.

### Acceptance

RED→GREEN with the same words distributed 2+2 versus 3+1, whole three-line
protection, independently disabled orphan protection, overheight groups,
restoration and default behavior without a flag. Bilateral boxes/words/pages
probes retain prior types/spans and fallback controls. XML groups are not a
physical line count. Restricted owner resealing, build and crystalline-lint must
pass without errors. The regional subset does not expand general pagination
certification. Original chronology is preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/text_costs.md`.
