# SetPage header: measured omission and architectural review

**Status: repair open; exact API/phase proposal pending approval.** Productive source and L0 remain unchanged. The [review package](review.md) binds the frozen [proposal](agents/core/proposal.md), public declarations and finite owner domains. The [current diagnostic](../../../00_nucleo/diagnostics/set-page-header-2026-10-21.md) distinguishes structural preservation from the language failure.

## Provenance and reproduction

The baseline is productive HEAD `63b7b96b99109b61655b373dc0d248beb7838f86` on `codex/tekt-mature-layout`, with no productive changes. Every command receipt records UTC and exact state. [The build receipt](baseline-build/receipt.json) freezes all 2142 productive inputs and binds the actual product binary SHA-256 `257c153b1044bf8bb773b05dad1a83c6a978ba6f9ce83506e929e4da3c11b89f`. Bilateral probes use an identical-byte copy under `/tmp`, separately from subsequent validation builds. Its bytes are checked before and after observation.

The ratified oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. The ratified recipe/source verification remains in `lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md`. Moving branches and version text are not oracle identities.

The [fresh preserved-control replay](baseline-controls/receipt.json) passes ten controls and reproduces the one known header mismatch. The process returns 1 deliberately because that mismatch remains a real failure. Historical F/G sources, expectations and receipts are pinned by [history-baseline.json](history-baseline.json); no old failure has been converted into a pass.

## Bilateral language measurements

The [frozen matrix](agents/parity/matrix.json) defines fifteen exact sources and observation properties before any candidate. [The raw receipt](baseline-parity/receipt.json) retains actual compile/query commands, stdout/stderr, source hashes, physical page counts, per-page PDF text and word bounds, and both binary identities. Oracle compilation succeeds for thirteen sources and refuses two for observed language reasons. A nonzero exit or missing query JSON is never acceptance by itself.

| Observation | Oracle | Baseline product |
|---|---|---|
| Original single-page header/body | Figure 1 / Figure 2 / final 2 | Figure 1 / Figure 1 / final 1 |
| Header repeated on two physical pages | Four distinct Figures; header states 1 and 3, body states 2 and 4 | Two body Figures; headers absent from query/history |
| Footer repeated on two physical pages | Body states 1 and 3; footer states 2 and 4 | Two body Figures; footers absent from query/history |
| Three-page contextual header | Header 1, Header 2, Header 3; three metadata occurrences | No header output or metadata |
| Header superseded before material | Dormant panic stays inert; one body Figure | Same observed language result |
| Background/header/body/footer/foreground | Five members and states 1–5 in that semantic order | Only body is a query member |
| Reference to repeated header label | Duplicate-label errors at both references | Missing-label errors at both references |

These counts and states come from the raw receipt's exact source/binary/state/UTC. PDF extraction order for overlapping canvas content does not establish geometry or semantic order. Complete typed query values are decisive for the stated temporal observations. Captured-style and footer page-counter-update probes are retained separately; neither callback invocation counts nor numeric Location representations are acceptance criteria.

The separate [ambient-style extension](agents/core/ambient-matrix.json) and [observations](ambient-baseline/receipt.json) refute freezing creation-time ambient styles: a template created at 10pt observes 20pt on the first page and 30pt on the next page after the corresponding body style changes. The independent [typed requalification](agents/contracts/verified-ambient-observations-1.json) retains exact hashes, source/binary identities and observer validity. Operational capture must preserve the actual page-effective environment as well as explicit inner styles and original source/rule provenance.

The [source/results analysis](agents/parity/baseline-summary.md) and [contract audit](agents/contracts/audit.md) separate inference from observation. [Independent acceptance requirements](agents/contracts/acceptance-v2.md) preserve dormant-template behavior, actual repeated identity, source/lexical scopes, styles, effects, original/visual membership and target boundaries.

## Verifier qualifications and inherited gaps

The observation worker deliberately has no global PASS assertion. Its raw equality telemetry has documented weaknesses; [the independent verifier review](agents/contracts/baseline-verifier-review.md) and [guarded requalification](agents/contracts/verified-baseline-observations-1.json) preserve those facts without rewriting the raw measurement. The downstream verifier rejects boolean/integer coercion, invalid JSON, nonzero empty observers and dual timeouts; its selfchecks are recorded separately. HTML is examined as actual document content, not solely artifact bytes.

Native PageRun preserves the measured text/counter ordering, but its second-page body metadata reports page 1 in the product. The same stale-page observation occurs in the two-page SetPage sources. This is an inherited position/context obligation, not proof that page count alone fixes queries. Direct Func marginals are refused by both binaries, with a diagnostic-word-order difference; this task does not authorize a new Func marginal mode. HTML keeps BODY and does not execute the Paged header panic, but the product omits the oracle's ignored-page-rule warning. These differences remain visible and unwaived.

## Validation of the baseline

[The command receipt](validation-baseline/validation-commands.json) and [freeze receipt](validation-baseline/validation-freeze.json) record all twelve existing jobs passing, with no changed productive inputs: CI verifier tests; canonical paths; strict forward lineage/ownership/Nuclei; reverse lineage; workspace build; domain/infra/shell tests; wiring tests; doctests; all-target build; strict Clippy; formatting; strict Rustdoc.

Full pinned lint has zero errors and warnings. Its [complete SARIF multiset comparison](lint-delta.json) is identical to G, retaining all 1339 V19 and 905 V20 notes. Tests and structural gates passing do not close the measured header failure. No repair, hash resealing, suppression, expectation relaxation or productive test change has occurred.

## Architecture boundary

SetPage is a progressive configuration delta; PageRun owns lexical page boundaries and restoration. Generic visitors currently leave SetPage opaque while page commit renders its raw running Content. Eagerly preparing every installed header would execute unused templates; opening one visitor cannot supply per-page identities and temporal order. Existing layout renderers remain callback-free, and ADR-0073/0074 preserve tracked per-store sealing rather than global frame introspection.

The frozen proposal specifies a typed physical running-content channel and a source-backed traversal plan, preserving original Content and existing legacy façades. [The source index](agents/core/source-evidence-index.json), [index-driver review](agents/parity/index-driver.md) and independent proposal reviews record implementation obligations and rejected earlier drafts. New public contracts and pipeline order require ADR-0127 review; no earlier Figure/Ref approval is represented as authority for this new boundary. Private implementation details must pass the declared owner-first gates; no unknown public API is included in the requested decision.
