# SetPage running-content admission

**Status:** freshly reproduced; repair not implemented. The [exact API/phase proposal](../../lab/results/set-page-header-2026-10-21/review.md) is pending the ADR-0127 decision.

Measurements ran on `codex/tekt-mature-layout`, clean productive HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`. The baseline build and bilateral receipts record UTC, exact inputs, diffstat, argv, observer output and binary identities under [the evidence directory](../../lab/results/set-page-header-2026-10-21/README.md). The oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.

The preserved single-page header/body fixture still renders Figure 1 twice, with final counter 1, instead of Figure 1/Figure 2 and final counter 2. Two physical pages with a static header require four distinct queryable Figures in the oracle; the product records only the two body Figures. Footer, contextual running content and captured-style probes expose the same missing admission boundary. An overridden template remains inert in both binaries; eagerly visiting every SetPage field would violate that control. Repeated-label references must diagnose multiple occurrences, while the product currently reports a missing label.

Source inspection shows SetPage remains an opaque progressive delta during preparation/indexing, while page commit draws its raw Content later. Native PageRun has separate lexical boundaries and restoration; rewriting SetPage as PageRun is not equivalent. A one-time marginal traversal does not establish physical occurrence identity, ordered actions or retained diagnostics.

All twelve existing validation jobs pass on the unchanged productive inputs: complete tests/builds, canonical paths, strict forward/reverse lineage, Clippy, formatting and Rustdoc. Complete pinned lint has zero errors and warnings; every V19/V20 note matches the prior G multiset. This structural/test preservation does not close the measured language failure. Historical F/G receipts and expectations remain unchanged.

The proposed repair must preserve L1 purity, incremental source-derived introspection, per-store sealing, original/visual membership, actual page order and the existing contextual transaction budget. It must not execute callbacks in layout, reconstruct global introspection from frames, suppress diagnostics or invent a Location. No new active L0 or productive code is authorized by this diagnostic.
