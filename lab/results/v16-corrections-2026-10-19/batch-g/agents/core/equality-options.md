# V16-074 — bounded equality/hash decision dependency

This supplements audit.json; it does not authorize implementation. Baseline is HEAD4382891f0d5f0839669597cb5c90a98f24cfb8bd, clean productive tree at the initial audit. Source SHA-256 and exact full bytes are preserved in source-before.json.gz. No oracle command, Cargo, source, L0 or test mutation has run in this audit.

## Evidence before the options

- `01_core/src/entities/content/equality.rs:11–211` has four transparent carrier projections, Space equivalence, existing same-kind delegation, and one false wildcard. That wildcard also covers same-kind HtmlElem/HtmlFrame/Title/ContextBlock and the three historically documented mechanical exceptions.
- `01_core/src/entities/html.rs:13–47` derives entity equality. Its attrs are IndexMap, whose locked2.13.0 implementation compares membership independent of insertion order (`map.rs:1866–1881`), while its Debug iterates insertion order (`map.rs:121–128`).
- `00_nucleo/prompts/core/entities/html.md` explicitly classifies Rust equality as mechanics; its frame clause explicitly says equality preserves body. `00_nucleo/prompts/core/entities/elements/title.md` preserves body and no hub always-false requirement. `00_nucleo/prompts/core/entities/elements/context_block.md` explicitly requires equality/hash by id. Their omissions cannot be justified by the unrelated historical Metadata/State/StateUpdate clause.
- `01_core/src/compiler/eval/operators/equality.rs:149–173` morph-projects raw content, then compares it with Rust PartialEq. `:378–421` separately compares located snapshots by linguistic field sets. A change in hub equality can consequently affect raw and nested language equality; these layers are different responsibilities, not operationally isolated.
- `01_core/src/entities/content_hash.rs:25–29` hashes the complete Content Debug string. `01_core/src/entities/content/rust_hash.rs:15–147` only specializes carrier transparency and MathMatrix/MathCases; all other Content roots use that Debug hash.
- `00_nucleo/prompts/core/entities/content/rust_hash.md` requires compatible Eq/Hash. Its historical no-algorithm-change clause describes an older bounded projection repair, not permission to violate equal-values/equal-hash. `00_nucleo/prompts/core/entities/content_hash.md` also requires equal Content, clones and equal sequences to hash equally.

## Candidate A: entity delegation with a compatible transitive hash correction

HtmlElem/HtmlFrame/Title delegate to entity equality; ContextBlock delegates to its documented id equality. Preserve explicit historical false cases and all existing projections. This is the most direct reconciliation with current entity owners, subject to independent language probes.

A top-level HtmlElem Hash specialization alone is insufficient: equal HTML trees inside Sequence/Title/Block still reach parent Debug fallback. Existing Eq-relevant Value fields, selector filters and style entries can also contain Content. An implementation based only on generic Content child traversal must first prove it covers those representation facts; current traversal intentionally leaves several arbitrary Value slots opaque. Do not claim complete hash compatibility by testing only ordinary body trees.

The narrow hash obligation is canonical attrs order in every newly equality-relevant enclosing representation, while preserving attrs presence and HtmlBody tri-state. Preserve Content Debug and language repr, and do not sort/mutate the actual attrs object. A private normalization/serialization approach requires complete traversal of the relevant stored representation or a proof that omitted fields cannot introduce equal HTML differences. No regex/text replacement of Debug output, constant hash, opaque skip, top-level-only repair, or changed public type is acceptable. This may exceed the current three-source ownership and requires an explicit owner/scope decision before code.

## Candidate B: separate order-sensitive mechanical hub rule

One bounded alternative can compare HtmlElem tag, attrs presence, ordered key/value entries and HtmlBody/body mechanically; HtmlFrame/Title body and ContextBlock id still delegate. It avoids introducing the attrs-order mismatch with existing Debug hash and fixes the unexplained always-false same-kind omission. Entity equality would remain a distinct owner rule, as Metadata already demonstrates.

This is a possible mechanical partition, not a measured language requirement. If the pinned oracle says opposite attrs insertion orders are linguistically equal, raw language comparisons would retain a gap under this rule because the operator currently uses hub Eq after projection. Do not adopt B merely because it avoids hash work, label it full equality parity, or silently ratify that gap as intended behavior. It would require a documented bounded decision and language controls; it is not the preferred final outcome on static evidence alone.

## Required controls before choosing

Measure actual pinned language output for title equal/different bodies, HTML elem attrs None/empty/ordered permutations and body Unset/None/Content, frame body equality, Context aliases versus two independent values, and array/dictionary nesting. Retain fields/repr/morphology output separately from the result of language comparison. Numeric Context id controls remain mechanical only. Then add actual semantic RED before implementation for each behavior accepted for repair.

For compatible hash acceptance, independently test equal reordered-attrs content at root, Sequence, nested Title/Block/HtmlFrame and transparent carriers; also inspect/test Content inside Eq-relevant Value/selector/style fields. Include unlike-kind symmetry and body/attrs/tag/presence differences. Treat hash equality as the Rust contract, not a parity observable, and do not require distinct content to have distinct hashes.

## Gate

The four traversal repairs remain continuous internal catalogue corrections. V16-074's explicit preservation portion can also flow after L0 update. New same-kind behavior must be classified from actual language evidence: a bounded parity correction with unchanged public signatures/default mode/phase can flow under ADR-0127, while a deliberate public equality/compatibility change or unresolved class requires approval. No public API, entity field, phase, default, generic visitor, hash owner, or suppression change is currently authorized by this read-only audit.
