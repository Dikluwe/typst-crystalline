# L0 — html/encoding
Hash do Código: c64e7859

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/encoding.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

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
