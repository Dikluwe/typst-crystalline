# P1339 — bounded style r3 engineering handoff

Role: observer implementer, executed without technical-isolation attestation. This is engineering evidence, not an independent binding audit, final phase-F acceptance or overall P1339 verdict.

After the independent mechanical acceptance `p1339-verifier-seal-F-style-supersession-r2.json` (`1c0fb2adc50ecc88ee418f60b0062616214c561c7776f2c7288601da887eb0f6`), the implementer created `p1339-implementation-observer-core-wrapper-r3.rs` (`e7ec2da32f4172d6fa937fcc567ebcfe918c306f8779ca8135b906c66722cb5d`). Its only difference from the original protected wrapper is the style include basename changing to the accepted r3. The observer's test-only include now selects this thin wrapper. One test-only focal entrypoint calls the existing `run_style_matrix` with existing real StyleHooks; no expectation, productive API or unrelated semantic code was changed in this resumption.

The parent held productive edits during the build/runtime window. The complete raw receipt is `p1339-implementation-observer-style-r3-engineering.json`, SHA-256 `a68fa1c321e001216348bd4bcf83145d5c5bf2edf6014dc88bb898ae789aec80`. It records HEAD/dirty-tree stat, exact argv/environment, UTC, streams compressed losslessly, the transitive observer include closure plus both supplements and ancestral seal, and a compressed manifest of tracked build inputs.

- Window: 2026-09-10T10:23:11.203Z–10:24:39.461Z.
- Instrumented compilation: exit 0.
- Exact focal test: `compiler::eval::p1339_frozen_observation_binding::p1339_frozen_style_bound_matrix`; exit 0, one test passing, twelve emitted style cells covering four profiles and three order labels.
- All fifteen include/supplement pins and all 520 tracked build-input pins remained equal before compile, after compile and after runtime. This explicitly describes tracked inputs, not an attestation about arbitrary untracked files or external toolchain/cache contents.
- Actual compiled test executable: `/tmp/p1339-implementation-observer-instrumented.Y0Zc3f/release/deps/typst_core-af52440717336ad7`; SHA-256 before and after execution `5788769d98376ee1a3452157b2e79dbdfa3a890f42aaa762012d738b8194c51e`.
- Observer source at this build: `b1ea3ba00af4ed9c2dfcc1bc9c7f80d26c48589931e833c35b74d5f5b59c9578`.

The original/r2 harnesses, original/r2 wrappers, prior compile failures and prior wrapper-mutation disclosure remain preserved. Unlike that earlier compile, no include was edited during this window. The current result does not silently supersede the earlier E0499/E0599 evidence or establish independent runtime credit.

No full core matrix, pipeline telemetry, full F run or commit was performed. Broader observer completeness and scope items in the earlier handoff remain open; this bounded task only establishes that the accepted mechanical adapter compiles and its focal real-product style diagnostic passes with the recorded inputs. The productive build window has been released to the parent, and this owner has no running process or further productive edit pending for this assignment.
