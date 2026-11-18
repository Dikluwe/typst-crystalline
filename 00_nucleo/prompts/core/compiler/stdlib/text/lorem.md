# L0 — stdlib/text/lorem
Hash do Código: 3e5b12a4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/lorem-chain.toml sha256:99b63160c9d2306e1be6dac120c3b856572140f907af893724409093422bd6ee


Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/lorem.rs`


## Evidence and classification

`01_core/src/compiler/stdlib/text/lorem.rs:33`, `01_core/src/compiler/stdlib/text/lorem.rs:73` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

lorem requires exactly one positional Int and no named arguments, returning Value::Str. Negative n fails `lorem() não aceita números negativos`; wrong type `lorem() espera um inteiro, recebeu {type}`; wrong arity `lorem() requer 1 argumento inteiro`; zero yields "". Exact generated text is language-observable acceptance, not approximate similarity.

The private generator uses the permitted lipsum order-2 Markov chain trained with LOREM_IPSUM then LIBER_PRIMUS, starting ("Lorem","ipsum"), deterministic ChaCha20Rng seed 97. Separate words by one space before each subsequent word. Token "--" emits U+2013 without counting as a word. After . ! ? capitalize the next word's first char through Unicode uppercase while preserving its remainder. If the final sentence lacks . ! ?, remove pending trailing ASCII punctuation and append ".". Preserve 1 -> "Lorem." and 10 -> "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do.".

Initialize the context-owned chain lazily on the first positive call; no mutable global LazyLock/cache. The existing license attribution (upstream MIT port based on lipsum, © 2017 Martin Geisler) remains applicable. lipsum usage within text is local to this unit and its whitelist. Keep lorem_impl private; no World/file reads or new public generator API; the sole context access is the internal training slot.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.


## Laboratory structural reuse, 2026-11-12

### Evidence before decision

`lab/results/construction/diagnostics/watch-residual-2026-11-12/fontdemand-v2-cpu-stacks/analysis.json` and its freeze bind the source, observer and UTC. Native lorem owns 247/1190 and 205/976 direct outline samples; these are sample shares, not exact exclusive CPU times. `compiler/stdlib/text/lorem.rs:80-83` reconstructs identical training on every positive call; the pinned upstream text/lorem.rs:45-52 retains training globally. Generated text is language-observable; the training lifetime is mechanics.

### Decision and exclusive responsibility

Laboratory only, no integration. Internal RAM correction under ADR-0030 and ADR-0127 continuous flow; no public field, function, trait, default, phase or compatibility change.



This owner performs training and generation using the existing argument validation and exact joining/punctuation algorithm. Use the compiler-private context slot only after successful positive validation. Keep the private generator private. Tests must refute repeated training, shared iterator progress, eager zero/error training and cross-context retention; compare full strings against the previous generator across boundary and long counts. Existing diagnostics remain unchanged.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.
