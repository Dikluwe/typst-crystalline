# P1339 — actual replay-chain telemetry correction

Role: observer implementer; executed without technical-isolation attestation. This is bounded engineering evidence, not a final binding verdict or phase-F/overall acceptance.

The independent review `p1339-verifier-style-r3-engineering-review-r1.json` (`85c08dd60780f10d2228f89af73ebc07ce09ded31c4b1185b6e1274a22ab424a`) correctly identified that the earlier raw DTO reconstructed replay chain identity and size from the original read. The old compile/runtime receipt remains intact; its passing predicates did not prove use of that chain. No old result is erased or promoted retroactively.

The verifier reviewed the correction design before edits. The root recorded owner-local authority before adding the cfg-only StyleChain backing-identity accessor. The observer now retains a clone of `transaction.styles` immediately before actual read-owner dispatch. Each `ContextReplayObservation` holds the original read identity, this actual retained chain, and the real replay result. Retaining the clone pins the shared backing allocation until the observation is consumed.

The recorded DTO uses the captured chain's backing identity. The replay DTO separately uses the actual dispatch chain's backing identity and effective size; the final Engine DTO uses the final actual chain. All three identity fields have the same backing-resource meaning, not an address of an Engine or an external wrapper. Empty chains are represented canonically as empty. No identity is inferred from size, hash, representation, expected values or fixture labels. The source mapping remains the previously audited exact input FileId allocation mapping.

Counterexample rationale, not an executed mutation: substituting ambient chain30 into `transaction.styles` before the observation/dispatch point changes the retained backing and size reported by replay even when request identity and counter result `[0]` remain unchanged. No mutation score or dynamically measured rejection is claimed here. The independent verifier owns assessment of this connection and any additional adversarial run.

## Exact new measurement

Raw receipt: `p1339-implementation-observer-style-r4-actual-chain-engineering.json`, SHA-256 `77dbd832fc95721563de3d194952035fec0a5877a71dfe114c8144c54ddac63d`.

- UTC: 2026-09-10T10:33:10.175Z–10:34:42.367Z.
- Actual instrumented compile exit 0; exact focal style test exit 0.
- One Rust test passed and emitted twelve cells, four profiles by three labels for the single fixture. This does not claim a reordered multi-case experiment.
- Twenty-one pins covering transitive includes, authorities, supplements, relevant owner and L0, plus 520 tracked build inputs, matched before compilation, after compilation and after runtime. Exact manifest, argv, dirty-tree provenance and lossless streams are in the receipt. Arbitrary untracked files, external toolchain and caches are not thereby attested.
- Actual executable `/tmp/p1339-implementation-observer-instrumented.Y0Zc3f/release/deps/typst_core-af52440717336ad7` retained SHA-256 `e0a5ca4ee6187aba21eecabb58e8d07cc799afca67bca646af78d828b7b10c41` before/after runtime.
- Observer source SHA-256 `3b608e291143b6feff07101fdfdb32b4782f11f12adbc877a90ad703841064c7`; root-owned StyleChain source SHA-256 `3f77f12991d4038b3c695606aec5d18934869c3a17a23bbe45e3c60f594287ed`.

The implementer changed only cfg-only observer telemetry in its authorized productive owner and its own diagnostics. The root separately owns the cfg-only StyleChain accessor under `p1339-style-observation-owner-extension.md` and its pins. Frozen oracle/harness/fixture predicates and semantic L0 text were not changed. No full F run, new productive API, unrelated semantic correction, commit or own final verdict was performed. Raw evidence and source pins were sent to the independent verifier. The coordinated productive build window has been released; this owner has no remaining process for this bounded assignment.
