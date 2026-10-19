# Prompt organization audit

The recommended convention derives each owning L0 path from its consumer's
layer and module path, omitting the leading `src/`. Names describe the tested or
implemented behavior, rather than the construction step. Path migration and
semantic consolidation are separate operations: changing a filename does not
resolve contradictory or superseded clauses inside a specification.

## Provenance and coverage

The audited commit is `d330c717eb06c86a560c4cf3d8d0ba40224609b9` on
`codex/tekt-mature-layout`. The tracked working tree was clean. Three delegated
agents read numbered test suites, compiler prompts, and other owner categories
and tooling constraints. The orchestration checked the complete mechanical
owner inventory and reconciled the reports. No active L0 or source was changed.

[inventory.json](inventory.json) records the timestamp, Git state, all current
owner/consumer pairs, SHA-256 identities, numbered filenames, and candidate
paths. It contains 991 unique owners/consumers, 84 numbered prompt filenames,
85 numbered Rust filenames, and nine numbered fixture assets. The filename
predicate is a leading `p` followed by digits or `passo-` followed by digits,
including the optional `typst-` prefix. Historical lab records are excluded.

Content reading covered the generic numbered test families and selected full
architectural owners. It was not a complete semantic audit of every paragraph
in all 991 prompts. [semantic-name-candidates.json](semantic-name-candidates.json)
contains 37 content-reviewed name candidates, including the requested outline
example. These are proposals, not applied renames.

The pinned executable `lab/target/linter/debug/crystalline-lint`, SHA-256
`421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`, passed the
delegated V5/V15/V26 checks. A prior agent check used the installed executable;
its result was repeated with the pinned executable before consolidation.
Passing ownership and seals does not certify the semantic freshness of prose.

## Directory convention

```text
00_nucleo/prompts/
  _nuclei/                  shared invariants, not direct code owners
  core/
    lib.md
    contracts/
    entities/
    compiler/
      eval/
        mod.md
      layout/
        mod.md
        outline.md
        set_page.md
    tests/
      outline_entry_indented.md
  shell/
    lib.md
    build.md
    cli.md
    tests/
  infra/
    lib.md
    world.md
    project_init.md
    watch.md
    export/
    tests/
  wiring/
    main.md
    watch.md
    tests/
```

For a consumer under `01_core`, `02_shell`, `03_infra`, or `04_wiring`, replace
the layer directory with `core`, `shell`, `infra`, or `wiring`, remove only an
immediately following `src/`, and replace `.rs` with `.md`. Retain remaining
module names and underscores. Thus `mod.rs` maps to `mod.md`; `lib.rs`,
`main.rs`, and `build.rs` retain their distinct identities. Integration tests
remain under `tests/`; internal test modules retain their actual module path.
The examples above are illustrative; the inventory defines actual consumers.

This mechanical mapping has no duplicate destinations among the 991 current
owners. It would relocate 852 owner paths before semantic source renames.
Combining the 37 reviewed source-name candidates with that mapping also has no
duplicate destinations. These checks establish uniqueness, not completed
reference migration or semantic approval of every final name.

The alternative is to retain the existing functional categories and repair
individual anomalies. That costs fewer moves but still requires learning
exceptions. Layer-plus-module mirroring fits the user's preference for a
predictable tree. Its cost is that future source moves also move their L0 owners.
Feature-oriented discovery can use a navigation index outside materializable
L0; it should not create a second owner or duplicate normative text.

## Findings from actual ownership and content

| Current artifact | Finding | Recommended treatment |
|---|---|---|
| `01_core/tests/p1405_outline_indented.rs:132` and `00_nucleo/prompts/compiler/stdlib/structural/p1405_outline_indented_tests.md:5` | A test harness for `outline.entry.indented`, including fixed/automatic/function indentation, RTL, fallback and rejection cases. | Rename the source to `01_core/tests/outline_entry_indented.rs` and its owner to `00_nucleo/prompts/core/tests/outline_entry_indented.md`. Preserve its exclusive test scope. |
| `00_nucleo/prompts/compiler/layout.md:31` | Owns `01_core/src/compiler/layout/mod.rs`, alongside flat `layout_figure`, `layout-image`, `layout_counters`, `layout_outline`, and `layout_references` prompts. | Put the owners together under `00_nucleo/prompts/core/compiler/layout/`, using the actual source basenames, including `mod.md`. |
| `00_nucleo/prompts/shell/init.md` and `00_nucleo/prompts/shell/watch.md` | Own L3 `project_init.rs` and `watch.rs`, not L2 shell files. | Put them under `00_nucleo/prompts/infra/` according to their actual source paths. |
| `00_nucleo/prompts/infra/bibtex.md` | Owns L1 `01_core/src/compiler/eval/bibtex.rs`. | Map to `00_nucleo/prompts/core/compiler/eval/bibtex.md`. |
| `00_nucleo/prompts/world.md` | Owns `01_core/src/contracts/mod.rs`. | Map to `00_nucleo/prompts/core/contracts/mod.md`; retain the distinct `World` trait owner. |
| `00_nucleo/prompts/passo-537b-set-page-columns.md:5` | Lists seven target files and still uses imperative authorization language, despite its sole consumer being `compiler/layout/set_page.rs`. | Audit and consolidate the owner contract before presenting it as `00_nucleo/prompts/core/compiler/layout/set_page.md`. Preserve old cross-module rationale in history and verify obligations with each current owner. |

The P537b discrepancy is substantive: its old proposal describes changes to
Content, PageConfig, eval, layout and tests; current `set_page.rs:16` accepts a
`SetPageDelta`, while the page-column transformation has a dedicated owner at
`00_nucleo/prompts/entities/content/page-structure.md`. A clean seal does not
make that old multi-file instruction a current specification for one consumer.

The generic numbered test suites have identifiable subjects. For example,
`01_core/src/compiler/eval/p1426_tests.rs` covers root evaluation and
`01_core/src/compiler/stdlib/structural/p1445_tests.rs` covers `math.round`.
The latter uses an LR carrier, but naming it as generic LR tests would obscure
the public function it verifies. API, evaluation and layout suites remain
separate owners even when they share a feature or construction-step number.

`04_wiring/tests/p1286_contract.rs` is an explicit exclusion in
`crystalline.toml:43`; its missing owner is not a newly discovered V15 failure.
The numbered font and certificate fixtures need separate provenance review.
Do not erase their identities merely because the basename includes a step.

## Content consolidation requires clause accounting

The agents read three long layout specifications in full:

- `00_nucleo/prompts/compiler/layout.md`: the repeated realization sections at
  lines 261–283 specify different obligations, including identity reservation
  and service scope. The SmartQuote section at lines 342–399 includes a
  ratified contract and phase decision. Neither is disposable simply because
  it is dated or appears after an earlier summary.
- `00_nucleo/prompts/compiler/layout/paragraph.md`: later empty-paragraph and
  `empty_text_style` sections partially supersede earlier formulas while
  retaining margin and fallback obligations. A rule that treats the last
  section as replacing every earlier clause would lose valid requirements.
- `00_nucleo/prompts/compiler/layout/paragraph_select.md`: historical evidence
  about overhang refutes a proposed selector diagnosis, while subsequent
  pagination obligations delegate to a different owner. Consolidating by
  keyword alone could assign requirements to the wrong module.

Each clause needs a disposition: retained current requirement, specifically
superseded requirement, active limitation, or supporting historical evidence.
Keep a compact measurement and its provenance link before the resulting
decision, preserving ADR-0108. Move full run logs and obsolete coordination
into the existing evidence/history areas. Retain current acceptance criteria,
fallbacks, public approvals and unresolved scope in the owning prompt.

## Migration sequence and checks

1. Record the path rule and a complete old/new source/owner manifest. Establish
   semantic names from the test bodies and L0 contract, not only the step ID.
2. Move unambiguous owner/source pairs in reviewable batches. Keep the historical
   files and checkpoint unchanged. Record exceptions explicitly.
3. Update source `@prompt` headers, Rust `mod` and `#[path]` declarations,
   affected parent prompts, canonical prose references, includes, test filters,
   CI/tool scripts and any explicit Cargo target paths. Renaming Cargo-discovered
   integration files also changes the target used by `cargo test --test`.
4. Resolve ownership and nucleus issues before resealing. Keep `_nuclei` paths
   and pinned invariant content unchanged for a path-only migration; revalidate
   the graph rather than inventing new shared claims.
5. Run the path/link checker, strict lineage, reverse seals, affected tests and
   all-target compilation. Compare full lint against the known V16 baseline.
6. Consolidate semantically stale or cumulative L0 prose separately, with a
   clause map and independent review. Do not merge or split Rust modules simply
   to make a directory visually uniform.

ADR-0130 fixes the canonical `00_nucleo/prompts/` prefix. Its statement that
that earlier decision did not rename prompts is not a universal ban on a
subsequent authorized migration. ADR-0129 still requires one owner per consumer
and distinct shared nuclei. The existing validators accept nested paths below
the canonical prefix and do not mandate the current functional taxonomy.
