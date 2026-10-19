# Independent structural evidence review

The review reconstructed every mutation from archived original sources and derived target locations independently. Raw compiler JSON, including primary spans, was inspected instead of trusting the worker's selected-diagnostic counter. No compiler was run during this review.

## Baseline

Status: complete.

core-extension: exact mutation verified; raw compiler error codes {'E0004': 195}; build-finished [False].
- V16-001: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:240 — no primary E0004 at the match.
- V16-002: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:322 — no primary E0004 at the match.
- V16-003: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:468 — no primary E0004 at the match.
- V16-004: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:581 — no primary E0004 at the match.
- content-field-closure: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:53 — no primary E0004 at the match.

shell-omissions: exact mutation verified; raw compiler error codes {}; build-finished [True].
- V16-083: 02_shell/src/cli.rs:1156 — no primary E0004 at the match.
- V16-084: 02_shell/src/cli.rs:1177 — no primary E0004 at the match.
- V16-085: 02_shell/src/cli.rs:1418 — no primary E0004 at the match.

## Green

Status: pending archived results.

## Scope

Compiler witness proves enum-domain review coverage, not language correctness.
Shell witnesses remove positive arms in actual shell matches; they are not future-variant cross-crate compilation witnesses.
Core baseline is an intentionally failing extension build with unrelated E0004s; it is not a successful unmutated build.

The worker restores every Rust source before each mutation group. The core group adds only the sentinel Content variant. The shell group removes only the selected positive arms, compiling against unchanged Content. Input hashes, exact mutated bytes and unchanged final freeze are verified in the JSON report.
