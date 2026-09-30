# L0 — html/tags
Hash do Código: aaf9a4d6

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/tags.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/tags.rs:63` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Tag classification

Own the single static void/raw/phrasing/block/replaced tables and
should_group_into_paragraph. Preserve the current sets, including summary as block,
script/style as raw and custom tags as non-groupable by default. Keep datalist's
existing inline-body classification distinct from its top-level nongroupable status.
Share these tables with internal consumers without duplicating them or inferring
CSS/display, scripting, parent validation or mutable global state.

## Verification

Exercise class boundaries, summary/datalist, raw and replaced tags and unknown tags.
