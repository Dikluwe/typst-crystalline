# Final independent extra controls

All six immutable runners used final `candidate-build-4/receipt.json` (SHA-256 `6015e9d8b3ab7836a199b321c9e41165bead3ffc44d1d90fc218570b0ce42602`), binary SHA-256 `47773b808cf589b7e0f31602b2d791856196ae92ee49d0af24184ca4cfd90f76`, and the pinned bd5ebec2 oracle. HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` has uncommitted changes; the companion JSON retains the exact dirty-file stat, build time, per-run UTC intervals, all source/asset/oracle/runner receipt hashes and current-input checks.

There are35 actual source compilations (27 succeed;8 are intended rejection controls) and7 additional successful language query processes. This is process accounting, not35 language-parity passes. Every runner verified full final build inputs, binary and frozen corpus before and after. All current2142 build inputs still match. No productive/L0 source, fixture, Cargo build or old receipt was changed by these replays.

| Matrix | Source cases | Required/observed accounting |
|---|---:|---|
| final-captionless-controls | 10 | {"full_fixed_case_checks_passed": 10, "full_fixed_case_checks_failed": 0, "runner_exit": 0} |
| final-reference-extra-controls | 3 | {"full_fixed_case_checks_passed": 3, "full_fixed_case_checks_failed": 0, "runner_exit": 0} |
| final-ancillary-controls | 5 | {"full_fixed_case_checks_passed": 4, "full_fixed_case_checks_failed": 1, "runner_exit": 1} |
| final-math-style-controls | 8 | {"required_checks": {"top_level_callback_compiles": true, "top_level_callback_matches_explicit_zero": true, "top_level_callback_differs_explicit_ninety": true, "ordinary_outside_context_rejects": true}, "required_checks_passed": 4, "required_checks_failed": 0, "source_exit_matches": 8, "known_script_contract_observation": {"callback_exit": 0, "matches_existing_Tekt_effective14_zero": true, "matches_pinned_oracle_lexical20_ninety": false, "classification": "Existing Tekt rendered-script14 contract versus pinned target lexical20 style observation remains a separately recorded debt; no all-callback-parity claim."}, "runner_exit": 0} |
| final-math-capability-controls | 5 | {"source_exit_matches": 5, "source_exit_mismatches": 0, "message_gaps_retained": ["here-no-location", "counter-get-no-location"], "observed_final_branch": {"exit": 1, "matches_seven_zero": false, "matches_other_ninety": false, "oracle": "Rejects unknown context before any angle branch; do not reinterpret as expected7 or0."}, "observation_complete": true, "runner_exit": 0} |
| final-counter-owned-controls | 4 | {"required_cases_passed": 4, "required_cases_failed": 0, "runner_exit": 0} |

## Required fixed checks and retained failures

Captionless Block controls pass10/10; bibliography/non-Figure/Grid extras pass3/3; ancillary fixed comparisons pass4/5. The remaining ancillary query is exactly actual `[[1,0]]` versus oracle `[[1,1]]`: the reached Heading child appears once, while ordinary Block own-label membership remains missing. Its runner exits1 and the failed full query is retained. These18 primary cases therefore have17 fixed-check passes and1 full fixed-check failure.

Modern bound/static counter.final controls pass4/4 required cases: both style-only/no-Location calls reject the exact message and both ordered hints with no artifact; both real located contexts render `(7,)`. The capability probes match all5 exit outcomes and the former false-zero final branch now rejects. Plain math19 style checks pass all4 required predicates; outside-context style access still rejects.

## Language differences remain visible

The script callback continues to select Tekt's effective14pt/0deg branch; pinned vanilla selects lexical20pt/90deg. The existing Tekt script contract and explicit returned-Context extension are preserved, with no full callback-parity claim.

All6 error-source cases in the math/boundary matrices retain diagnostic differences on at least one measured axis. Ordinary outside-context text.size has only the first of vanilla's two hints. here/get still have different error text and missing hints; here also has an extra calling trace. The math callback owner anchors here/get/final and modern bound/static failures at the cancel request, while the oracle anchors the inner call. The newly repaired modern final message/hints match; its independent source-anchor gap remains. Complete payloads, field-by-field differences and all raw stderr receipts are retained in JSON. These differences are not suppressed or counted as complete diagnostic success.

The fixed requirements, source-process outcomes and known semantic/diagnostic differences use separate denominators. There is no blended all-green score, whole-parity assertion, F closure or all-alternative runtime coverage claim. Final12 workspace validation is recorded separately by the orchestrator.
