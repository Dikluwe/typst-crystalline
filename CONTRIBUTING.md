# Contributing to Typst Crystalline

Thank you for contributing to this experimental Typst fork. The project preserves Typst attribution and licensing; see [`LICENSE`](LICENSE) and the [upstream Typst repository](https://github.com/typst/typst). Contributions here follow this fork's architecture and review process. If you intend to submit work upstream, follow the [upstream contribution policy](https://github.com/typst/typst/blob/main/CONTRIBUTING.md) separately.

## Before you start

For a bug fix or feature that spans responsibilities, open an issue or discuss the design before implementation. Start with a focused change and identify the owning layer and its current L0 prompt. Read [`CLAUDE.md`](CLAUDE.md), the relevant ADRs in [`00_nucleo/adr/`](00_nucleo/adr), and the complete owner prompt under [`00_nucleo/prompts/`](00_nucleo/prompts).

L0 is the causal specification for productive code. Keep the one-prompt-to-one-consumer ownership rule; shared obligations belong in pinned Tekt Nuclei. Follow ADR-0127: public contracts, default product behavior, pipeline phase changes, and compatibility breaks require an explicit human decision. Internal corrections within an authorized scope follow continuous flow with L0 updated first, then RED→GREEN evidence and revalidation.

## Make and validate a change

1. Create a focused branch from the intended project state.
2. Inspect the current implementation, governing ADRs, owner prompt, pinned Nucleus closure, and relevant evidence before choosing a design.
3. Update L0 before product code. Preserve its ownership, hash lineage, evidence scope, and acceptance criteria.
4. Add or update meaningful tests in the owning layer, confirm the regression case fails, then implement the smallest change that satisfies the specification.
5. Validate the affected workspace and the required project gates. Product build output belongs under `lab/target/product/`; project tools and CI commands are under `lab/tools/`.
6. In the pull request, describe the user-visible result, architectural owner, evidence, validation, and any remaining limits. Keep claims tied to what was measured.

Do not claim language parity from matching Rust structures, algorithm steps, or output bytes unless those are the specified observable. Identify the pinned oracle and the covered semantic, syntactic, and morphological cases. Historical measurements against older pins remain historical.

## Review expectations

A contribution should address one coherent change, keep interfaces and dependencies within the owning layer, include appropriate regression coverage, and update documentation where the user-facing behavior changes. Review checks the implementation against the current L0, ADRs, tests, lineage, and stated evidence. Passing CI is necessary for the covered gates; it does not certify behavior outside their scope.

Use the exact commands and resource limits in [`lab/tools/ci/README.md`](lab/tools/ci/README.md). Do not publish or release a build without the project's release approval.
