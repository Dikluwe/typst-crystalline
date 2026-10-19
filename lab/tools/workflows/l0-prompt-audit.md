# Workflow — auditing an L0 prompt

**Purpose:** Review the quality and completeness of an L0 prompt under `00_nucleo/prompts/`. Use this when checking whether a prompt is ready to guide implementation or has gaps that must be resolved first.

**This is not:** a mechanical linter. `crystalline-lint` checks structural rules; this workflow reviews content and clarity that a regular expression cannot assess.

## How to use

Apply the four blocks below to each prompt, in order. Each block produces independent findings. Do not stop at the first issue; record all findings.

### Block 1 — Structural completeness

Check whether the prompt covers the five roles in `l0-prompt-template.md`: Context, Structural Constraints, Instruction, Verification Criteria, and Expected Result.

- Accept equivalent wording already used in the project (for example, “Purpose” for Context, “Struct” / “Public Interface” for Instruction, or “Invariants” / “Criteria” for Verification Criteria). Do not require literal headings; require the role to be covered somewhere in the document.
- If no part of the prompt covers one of the five roles, report the missing role.
- Review whether the L0 has one productive consumer and the consumer has one owner. Shared claims belong in pinned Tekt nuclei and do not change this 1:1 ownership rule.

### Block 2 — Content ambiguity

For the Instruction section or its equivalent, check:

1. **Is each behavior claim verifiable?** Phrases such as “must work correctly,” “handles normal cases,” or “reasonable behavior” are ambiguous unless “correct,” “normal,” or “reasonable” is defined. Report the phrase and what remains unspecified.
2. **Is each domain-specific term defined or self-evident?** If a Typst term (for example, “auto-scaling,” “fence,” or “stretchy”) is neither defined nor linked to official documentation and is not clear from context, report it.
3. **Are scope limits explicit?** When plausible edge cases are not covered, the reader should be able to tell whether they were deliberately excluded or not considered. Scope-outs need not enumerate every imaginable case.
4. **Are there internal contradictions?** Report incompatible claims about the same behavior in different sections.

### Block 3 — Evidence (when applicable)

For prompts describing Typst language behavior, rather than internal crystalline infrastructure:

- Claims about what Typst does should cite official documentation (`typst.app/docs/`) or, where documentation does not cover the case, a direct measurement against the pinned vanilla binary with provenance (`file:line` or command and result).
- The current vanilla parity target is pinned upstream commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Prefer the verified oracle at `lab/typst-original/target/release/typst`, built with explicit `TYPST_COMMIT_SHA` and verified provenance. The current crystalline product binary is `lab/target/product/release/typst`; do not use it as its own oracle. `/usr/local/bin/typst` is usable only after byte-identity with the ratified oracle is proved.
- Confirm commit, tree, build recipe, and binary SHA-256 before citing a measurement. A tag, version string, or `--version` alone does not prove identity. The ratified vanilla executable SHA-256 is `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
- Parity follows ADR-0107/0108: measure language semantics, syntax, and morphology, not Rust implementation mechanics, and do not copy vanilla defects as requirements.
- A claim without either a supporting citation or measurement is an evidence finding, whether or not the claim happens to be correct.
- Check that the cited source supports the full generalization. For example, documentation saying “X can happen” does not support a prompt that claims “X is the only case.” A known precedent is `compiler/eval.md`, which described `\\` in math as “only” a line break when the cited source only stated that formulas can contain line breaks.
- If an area has a documentation corpus under the current documentation-corpus path, compare against it directly; this is faster and more reliable than retrieving the same documentation again.

### Block 4 — Historical task/step references

Search the prompt for historical task or step identifiers (for example, `P123` or `Step 123`). A reference outside creation/revision history is a finding unless it is clearly presented as historical provenance and resolves to a retained record. Prompts must not depend on a permanent numbered step directory. When a referenced historical record is required to understand an obligation or measurement, retain it in the project’s recoverable history and point to its current location. Creation/revision history should record a date and description, not use a step number as a substitute for the content.

## Output

Report one finding per issue. Each finding includes:

- Source block (1–4).
- Exact location (section and, where useful, line).
- A concise description of the problem, without recommending a fix. This workflow audits; it does not perform the repair.

Do not assign B1/B2/B3 classifications here; those apply to code modules under ADR-0084. This workflow returns findings only. The task that requested the audit decides what to address first.
