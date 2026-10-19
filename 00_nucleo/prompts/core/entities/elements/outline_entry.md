# Prompt L0 — `entities/elements/outline_entry` — `OutlineEntryElem`
Hash do Código: 44be7b9b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/outline_entry.rs`

## Evidence before public entry

Historical upstream 586e1bd43 outline.rs:468-522,668-674 constructs
entry(level:1,element:heading(body:[H])); synthesized entries expose full
heading,level and default fill repeat(body:[.],gap:0.15em). body() returns
[H]; strong yields cannot outline strong.

## Contract and private causal data

OutlineEntryElem is static native identity outline.entry, never Dynamic.
It stores level:usize>0,element:Content,fill:Option<Content>; get_field exposes
these three. plain_text follows the outlinable body's text. Both maps recurse
into element/fill. body() supports Heading in this bounded slice, returning
semantic Content rather than flattening to text; other variants fail explicitly
in the call consumer. No scopes/eval logic lives here. Locatable kind is
OutlineEntry and participates in query.

The separately gated prefix/page contract adds private realized semantic
prefix and causal Location, not reflected fields. Direct entries initialize
both absent; introspector-synthesized entries receive them. prefix() returns
preserved Content or none; stdlib page() uses Location. Walkers preserve
metadata, transforming prefix Content only. The structural-outline owner explicitly records human approval on2026-09-14
for this prefix/page contract (archived original at
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/stdlib/structural/outline.md`, original lines 210–214).
This satisfies the bounded gate; it is documentary approval evidence, not an
inference from implementation. inner/indented remain separately owned and
outside this entity's original slices.

## Measured query/body repair and transparent carriers

`lab/results/construction/diagnostics/tekt-v16-introspect-walk-2026-10-16/before.json.gz`
records HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,uncommitted tree/UTC/exact
diffstat/source hashes/binaries. measurement-before.json/measurement-fields-before.json
at pin9f2b6e8715237cb086899a42873660fe744622e8 measured direct/synthesized/mixed
entry query counts1/1/2 upstream versus 0/1/1 local; direct body() failed
cannot outline heading. Source sites: outline_entry.rs:61 accepted only bare
Heading,99 gave NativeElement; extract_payload.rs:148 discarded OutlineEntry;
walk.rs:805 treated terminal; outline_entries.rs:38-61 synthesized after walk.
Generic map_content visits element/fill; realized.rs:139-281 uses it to certify
origins, requiring Located-heading controls without treating a referenced
heading as rendered descendants. Upstream outline.rs:472-481 is Locatable.
Refuters include wrong query/fields/body,phantom headings,duplicates or changed
synthesis/order/labels; this is language behavior, not PDF/Rust equality.

body() traverses only Source/Located by reference to the semantic variant,
returning cloned Heading.body or the same cannot outline <elem> error.
Keep stored element intact; never unwrap Realized/Strong/Emph/Styled,execute
callbacks,invent Location/prefix,expand outlinable types or walk referenced
fields as visual content. Preserve signatures/constructors/fields/payload/maps/
get_field. This supersedes earlier limited OutlineEntry caveats only within
the measured repair and pinned source-origin invariant.

## Acceptance

L0 precedes colocated real functional RED; retain existing controls and old CLI
measurement. Repeat direct/synthesized/mixed body()/field fixtures at language
level, not PDF bytes. Separately record RED for Located(Source(Heading)) and
Source(Located(Heading)), retaining Strong semantic body and unchanged element.
Real-workspace disposable Content sentinel must add E0004 at six W1–W6 sites
only after repair, without other errors; preserve logs/inputs. Reseal only five
pairs after valid V15/V26. Build/workspace suite/fmt/Clippy all-targets warnings+
unfulfilled expectations errors/strict Rustdoc/strict-reverse lineage/diffcheck/
differential SARIF required. Historical expectation: six V16 removed 239→233,
six informative V19 added; extractor complement loses one alternative and
needs_fresh_index moves OutlineEntry false→true. No new V20/other diagnostics
after span mapping. V19 uniform consequences are not per-alternative execution
coverage. No suppressions/global config;12GiB/no swap/one Cargo job,heavy jobs
sequential with state/UTC/hashes. Internal ADR-0127 repair adds no API/default/
phase/compatibility; other V16/global work remains open. The original repair
package excluded publication/commit/push.

## Borrowed prefix access

tekt-v16-bibliography-static-2026-10-17/before.json.gz under diagnostics records
the same HEAD/tree/source/hashes/status/diffstat/UTC; outline_entry.rs:73-75
returned cloned Option<Content>, preventing borrowed traversal across modules.
This measures navigation contract/cost, not language or heap improvement.
Add only crate-private prefix_ref(&self)->Option<&Content> using prefix.as_ref().
Preserve public prefix(),fields/construction/Location/body/maps/all behavior.
The static visitor borrows without navigation clones. Local tests check direct
entry None and a reference to stored realized prefix; pointer identity is not
parity evidence. Internal ADR-0127, no public/trait/default/phase change;
validate with the three-owner I2 package/restricted reseal,12GiB/no swap/jobs 1,
no publication.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/outline_entry.md` preserves the original evidence and chronology.
