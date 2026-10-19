# Context occurrence task: checkpoint read-before report

**Scope:** read-only context sufficiency audit for the representative task: “When a Context-produced body revisits realization and returns `Result::Err`, what identity must stay distinct or be reused, who owns arena/frame responsibility, what restoration is required, what is explicitly excluded for panic, and which acceptance cases must not weaken?”

**Checkpoint:** `tekt-construction-2026-10-18`, commit `8ec827fd975bf14cd849bebb9ede245e49033337`. All source text below was obtained with `git show <checkpoint>:<path>`. No current-tree prompt prose, implementation source, tests, pilot reviews, or post-checkpoint files were read. No code was changed and no tests were run.

## Minimum task-relevant excerpts and findings

The owning L0, `00_nucleo/prompts/compiler/realize/context_occurrences.md`, ratifies a pure Location arena shared by contextual producers in a Session. `new(origin)` reserves Locations carried by that origin and those assigned by its canonical introspection. Reservation observes the maximum used ID; it does not create an identity. Existing `Located` aliases keep their Location and do not call `next`. No identity may be derived from morphology, span, text, hash, Context ID, or an address.

Each real producer owns its frame across its own re-executions. A frame reuses its private slot for each Location allocation actually reached on replay; a new reached slot is allocated by the arena's shared Locator. On contraction, unused slots remain reserved and emit no tags or contributions. Growth appends at the shared free tail. Different producers must remain disjoint even when their bodies or fields are equal; within one producer, stable occurrences reuse their allocated slot identity. The parent `compiler/realize/mod` owner says `locate` keeps an existing `Located` intact and delegates only new Location allocation to the private `next` helper; without an active frame, the existing Locator path remains.

`evaluate` temporarily installs the arena Locator and frame slots in the existing `EvalContext`, resets the frame cursor, invokes the operation once, recovers Locator/slots, and restores the context's previous state on every normal return, including `Result::Err`. **Panic handling is explicitly out of scope:** do not catch panic and do not alter the compiler's unwind policy. This is a normal-return restoration guarantee, not a panic-safety guarantee.

The task-specific acceptance cases are: reserve the real origin's identities; disjoint identities for producers; slot reuse, growth, and contraction; restoration after `Result::Err`, using real locatables and introspection; plus integrated alias, nesting, and selective-replay controls. The prompt also requires the count and `it.location` RED/GREEN comparison bilaterally against the pinned oracle. These cases should not be removed, combined into weaker synthetic-only checks, or relaxed. The prompt’s build/suite/strict Clippy and Rustdoc/format/lineage/SARIF gates remain part of the full materialization acceptance, but were not run in this context audit.

The L0 records a measured language-identity defect: repeated context producers could reset `EvalContext`/Locator, while introspection deduplicated by Location; probes showed duplicate `EQ` values with differing query identities and a `Heading` prefix scenario. Under ADR-0107, the acceptance target is occurrence semantics and morphology, not a Rust representation, memory address, algorithm copy, or output bytes. No cognitive or performance improvement is claimed.

## Canonical owners

- Arena/frame API and its direct tests: `00_nucleo/prompts/compiler/realize/context_occurrences.md` → exclusive consumer `01_core/src/compiler/realize/context_occurrences.rs`.
- Parent realization integration (`locate` and delegation to `next`): `00_nucleo/prompts/compiler/realize/mod.md` → exclusive consumer `01_core/src/compiler/realize/mod.rs`.
- The `EvalContext` itself and surrounding context-production lifecycle remain owned by their own prompts; these were not inspected here. The Context occurrence prompt only borrows the existing `EvalContext` to install and restore transient allocator state.

## Read-set, hashes, portions, and reasons

SHA-256 values are computed over the exact bytes emitted by `git show` at the checkpoint.

| Checkpoint path | SHA-256 | Portions consulted and reason |
|---|---|---|
| `00_nucleo/prompts/compiler/realize/context_occurrences.md` | `df318f1037aece205dc47b93d3699ccf3984e314a181122a35dc16d7f5d21625` | Full file; exclusive owner, arena/frame contract, error/panic guarantee, and task-specific acceptance. |
| `00_nucleo/prompts/compiler/realize/mod.md` | `48129363572ae92b8eb31a5d091b9cee6acd3f70d76b41bb3620685a78994195` | Header and pinned-nucleus list (lines 1–16); general realization identity clause (lines 24–28); `Context — identidade de ocorrências ratificada` measurement/gate and responsibility (lines 551–580). Used to establish parent integration ownership and shared identity context. |
| `00_nucleo/prompts/_nuclei/content/map-content-change.toml` | `96f8917eb9e198a3230e27770a800b4dc13f58fde8fc33390bd2d64f3bb76c93` | All claims; checked causal walk/error interruption and the prohibition on inferring change from equality/hash/address. |
| `00_nucleo/prompts/_nuclei/content/sequence-realization.toml` | `696102b0d120e8a02f568ad07a9bcec55120625717e478dc755eb75a5a4fa858` | All claims; checked alias occurrence/guard distinctions and the “no fabricated Location” boundary. |
| `00_nucleo/prompts/_nuclei/content/source-origin.toml` | `11fedbd7980d61194d253cb6d7b07fc060db1a1225f01b2a44a735ce943ea717` | All claims; checked single-occurrence/source transparency and that Source cannot create an independent Location. |
| `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md` | `39cf82738584006ca154d3cff6aa835a297f0e2ceba9d5dfc7ea2521f8752f1c` | Decision and measurement sections; distinguishes language-level occurrence behavior from Rust equality, internal shape, algorithm, and output bytes. |
| `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md` | `741460cf71a2defc826ab2b53aeef1a3c80eefa11b6c1fc8000792a531807a86` | Gate criteria and continuous-flow criteria; confirmed the recorded approval applies only to the specified additive API and cannot be generalized to unrelated contract/phase changes. |
| `00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md` | `bc741c4e61ea9683bf483bb075009d4b763d7d7c82fd4f79b549d0238846d24e` | Ownership and nucleus rules; establishes one owning prompt per consumer and that owner-specific identity/algorithm/acceptance cannot be delegated to nuclei. |
| `00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md` | `006280aea1d828005c85a421638515d3d76cd31aa9259885b31e97693dd9c8bc` | Canonical prompt reference rules; confirmed the full repository-relative owner paths above. |

## Minimum excerpt set versus implementation closure

For answering this bounded task, the minimum decisive text is the complete `context_occurrences.md` owner plus the `Context — identidade de ocorrências ratificada` subsection and ownership header of `realize/mod.md`. ADR-0107 supplies the language-versus-mechanics boundary; ADR-0129/0130 establish ownership and canonical owner references; ADR-0127 confirms the scope of the recorded gate. The three inspected nuclei are supporting constraints on causal Content traversal, aliases, and Source, not substitutes for the owner’s contract.

This is not a sufficient implementation packet. Before actual implementation, audit the then-current (not merely checkpoint) L0 owners and hashes, the complete `realize/mod.md` owner, all six nuclei pinned there (including `textual-recipe-frontier`, `bare-insertion`, and `table/cell-realization`, not read in this experiment), and the full applicable ADR closure listed by both owners. Then inspect the owned consumer code and existing tests, confirm ownership/hash/nucleus integrity, and follow the repository’s RED-before-code and final validation gates. This experiment intentionally did not inspect those sources or establish whether the checkpoint contracts are still current.

## Unresolved implementation questions

None for the representative contract questions above. Integration details—where real Context producers obtain/persist their frames, how `EvalContext` currently stores the relevant Locator/frame state, and exact existing test locations—remain unassessed by design. They require the complete current owner/code/test closure before implementation; no speculative answer is made here.
