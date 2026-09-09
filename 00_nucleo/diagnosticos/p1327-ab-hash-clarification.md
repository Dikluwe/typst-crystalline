# P1327 A/B — raw and normative identities

Additive clarification before C, without changing any frozen oracle or expectation.
The raw SHA-256 values in `p1327-ab-freeze.json` identify exact files as received.
The parent manifest uses normative SHA-256 after removing the complete single
`Hash do Código: <8 lowercase hex>\n` line. These are distinct identity domains.

Independent recomputation from the two allowed L0 inputs confirms the manifest:

- modules.md normative SHA-256: `790a041521e4d358f9e1ba76d9128ba554d6050eeda5dda50573d2f371086abe`.
- tests.md normative SHA-256: `fa0c9d6c0d2f473cc16c281ebf4e780f1f2b511aac73d04a56a9a2353643f900`.

The reciprocal code-hash line may change upon integration/reseal as the parent
manifest specifies. Normative bytes remain frozen; no semantic change is allowed.
The CLI candidate runner checks its own frozen SHA and oracle identity; it does
not require unchanged reciprocal metadata or inspect runtime source.

The frozen receipt SHA remains
`f45fa705a4f0b1edb93c6f48d3c58adea159a983a296c161a3606000acf6c076`.
This clarification does not revise the corpus, classes, ranges, transcripts,
replacement blocks or comparator. Regime remains A/B without technical isolation
attestation.
