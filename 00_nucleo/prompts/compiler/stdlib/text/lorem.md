# L0 — stdlib/text/lorem
Hash do Código: 84ed3780

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/lorem.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/text/lorem.rs:33`, `01_core/src/compiler/stdlib/text/lorem.rs:73` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

lorem requires exactly one positional Int and no named arguments, returning Value::Str. Negative n fails `lorem() não aceita números negativos`; wrong type `lorem() espera um inteiro, recebeu {type}`; wrong arity `lorem() requer 1 argumento inteiro`; zero yields "". Exact generated text is language-observable acceptance, not approximate similarity.

The private generator uses the permitted lipsum order-2 Markov chain trained with LOREM_IPSUM then LIBER_PRIMUS, starting ("Lorem","ipsum"), deterministic ChaCha20Rng seed 97. Separate words by one space before each subsequent word. Token "--" emits U+2013 without counting as a word. After . ! ? capitalize the next word's first char through Unicode uppercase while preserving its remainder. If the final sentence lacks . ! ?, remove pending trailing ASCII punctuation and append ".". Preserve 1 -> "Lorem." and 10 -> "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do.".

Build the chain per call; no mutable global LazyLock/cache. The existing license attribution (upstream MIT port based on lipsum, © 2017 Martin Geisler) remains applicable. lipsum usage within text is local to this unit and its whitelist. Keep lorem_impl private; no World/context/file reads or new public generator API.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
