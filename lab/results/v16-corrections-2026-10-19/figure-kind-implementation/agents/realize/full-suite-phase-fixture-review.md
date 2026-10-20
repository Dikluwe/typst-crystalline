# Full-suite Ref phase fixture review

The three failures are obsolete phase assumptions, not evidence of a productive non-Figure Ref rendering failure. No production change is warranted from these receipts.

The failed full-suite generation is bound to `validation-final/validation-state.json.gz` and `validation-final/tests-domain-infra.log`. Exact old test bodies were recovered from `structural-candidate-final/original-sources.json.gz`; their complete source hashes match the failed validation inputs. The JSON review includes those exact source excerpts, raw failures, hashes, HEAD/dirty provenance and approval binding.

The approved Ref phase captures an emitted normal Ref as a classified contextual producer while retaining its canonical original Ref. The current owning L0 explicitly distinguishes eval-only capture from actual retained Paged contextual execution. `do_eval` returns only the evaluated module. The p788 tests then call the pure L1 `layout(content)` entry point, which introspects/layouts but executes no contextual producers. Their missing Section/Cap text therefore follows from bypassing the newly approved execution boundary. The core test similarly expects a flattened raw Ref from a phase that now returns the producer transport.

Migrate only the fixtures:

- Keep `@meu_label`; verify the exact canonical original Ref fields/source and classified producer. Its missing target must not be fabricated or silently executed as success.
- Keep both p788 source strings byte-for-byte and their `Section 1`/`Cap 1` text assertions, including the English-versus-Portuguese negative control. Compile through the actual retained Paged facade.
- Preserve context execution, traces, errors and assertions. Do not strip/neutralize transport, weaken helpers, execute callbacks in pure layout or manufacture target/counter histories.

The failed generation remains preserved. Migration needs targeted and complete revalidation with the new exact input hashes; this review does not claim those runs have passed. Agent C owns the test and corresponding L0 edits; no productive/API change is authorized by this classification.
