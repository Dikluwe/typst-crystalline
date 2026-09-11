# Frozen style adapter: actual compilation blocker

Engineering compilation (not a phase-F run):

```
CARGO_TARGET_DIR=/tmp/p1339-implementation-observer-instrumented.Y0Zc3f RUSTFLAGS='--cfg p1339_observation --check-cfg=cfg(p1339_observation)' cargo test -p typst-core --lib --release --locked --offline --no-run --message-format short
```

The actual Rust compiler rejected the protected adapter:

```
p1339-mutant-closed-state-style-harness.rs:222:35: error[E0499]: cannot borrow `validation_tracked` as mutable more than once at a time: `validation_tracked` was mutably borrowed here in the previous iteration of the loop
```

The assignment `engine.sink = &mut validation_tracked` occurs inside the operations loop. Engine's existing single lifetime ties this mutable borrow to the retained engine rather than the current iteration. This is not an oracle mismatch or a candidate result. The frozen harness had not been compiled before sealing, as its provenance already discloses. No protected file has been changed by the observer implementer.

Parent informed immediately. A correction requires the authorized adapter owner's mechanical revision and the independent chain's explicit treatment of its new pin. Until then this binding has no runtime credit; neither a fake Engine nor changed productive API is an acceptable workaround.

Concurrent integration errors (state replay not yet present, selector repr test argument type) are distinct and were also reported to their owners. This diagnostic is not a verifier verdict.
