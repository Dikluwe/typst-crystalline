# L0 — Full-paragraph bidi resolution in L3
Hash do Código: f5281c19

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425

**Layer:** L3.
**Exclusive consumer:** `03_infra/src/bidi_context.rs`.

## Evidence before decision

`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/probe.json.gz`
refutes isolated line/shaping resolution; initial.json.gz retains provenance and
thirteen bilateral cases. Upstream inline/collect.rs:156–178 preserves explicit
opposite-direction embeddings. L1 API was ratified by manifesto 1ae7513e;
World/FontMetrics do not expand.

verification.json.gz for the second candidate observes numeric-explicit Tekt
456/789 versus vanilla 789/456 on the second line, while soft-wrap 456/789 agrees.
Upstream inline/collect.rs:191–194 inserts LF for nonjustified Linebreak and U+2028
only for justify=true. unicode-bidi 0.3.18 src/lib.rs:546–560, 669–676 clones all
self.levels on every visual_runs; line-view-before.json.gz retains source/SHA/
HEAD/stat/UTC and the previous resolver. These facts support the contracts below;
no runtime gain is inferred from copied-size reduction alone.

## Registry and resolution contract

A crate-private registry holds Vec entries with Arc input, an auxiliary string and
ranges for each original segment. Discover inputs from document carriers/envelopes/
layers without text/geometric identity inference. Arc ptr_eq only deduplicates the
same immutable input locally; it is not a language observable. Build auxiliary text
without normalization: original Text, Space U+0020 and Linebreak LF. Wrap Text of
opposite direction to the base in LRE/RLE and PDF only in the auxiliary string.
This unit's Linebreak carrier certifies only nonjustified breaks; its producer
rejects justify=true rather than inferring absent state. This correction keeps
phase/API unchanged.

Resolved borrows the registry and owns BidiInfo with ordinary Rust lifetimes per
string and explicit base. A span query maps original to auxiliary range, locates
its paragraph and uses visual_runs, returning relative ranges/RTL only for author
bytes, never auxiliary embeddings. Line queries include the interval from minimum
to maximum span for end-of-line rules; consumers intersect exact runs. No midpoint/
substring origin estimate. Reject spans whose text does not match their carrier.
Preserve author-supplied controls.

Cache lifetime is local to compilation and may be shared by internal reorder/
shaper helpers; public wrappers construct a local equivalent. Retain no World/
watch state, mutable globals or unsafe.

## Bounded line views

Do not clone all input levels per word/line. Construct temporary ParagraphBidiInfo
for only the interval text/classes/levels, copying levels already resolved from
whole input and preserving paragraph_level. Local visual_runs applies only Unicode
L1/L2 to the slice; translate ranges back. No additional BidiInfo::new or automatic
detection on fragmented text.

## Acceptance

RED→GREEN: `456 789` within `ABC 123 456 789 مرحبا` is a single LTR run under RTL
base, not neutral-reversed like an isolated fragment. Auxiliary controls never
appear in original runs; author embeddings survive. Prefix/suffix/UTF-8/override/
partial-line controls preserve identity. Validate bilateral source→PDF, full-input/
numeric/break tests and CLI. Measure cost rather than assuming a time benefit
from a smaller copied interval.
