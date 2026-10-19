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

Status: complete.

core-extension: exact mutation verified; raw compiler error codes {'E0004': 202}; build-finished [False].
- V16-001: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:351 — E0004 primary 351:45 bytes 13978..13979 (`c`).
- V16-002: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:565 — E0004 primary 565:11 bytes 21497..21504 (`content`).
- V16-003: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:840 — E0004 primary 840:38 bytes 31353..31354 (`c`).
- V16-004: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:1062 — E0004 primary 1062:11 bytes 39859..39860 (`c`).
- content-field-closure: 01_core/src/compiler/eval/bindings/field_access/content_methods.rs:53 — E0004 primary 53:11 bytes 2367..2368 (`c`).
- equation-getter-recognizer: 01_core/src/entities/content/fields.rs:10 — E0004 primary 10:11 bytes 241..248 (`content`).
- equation-identity-recognizer: 01_core/src/entities/content/identity.rs:13 — E0004 primary 13:11 bytes 298..305 (`content`).
- equation-encoding-recognizer: 01_core/src/compiler/stdlib/loading/encoding.rs:138 — E0004 primary 138:11 bytes 5511..5518 (`content`).

shell-omissions: exact mutation verified; raw compiler error codes {'E0004': 4}; build-finished [False].
- V16-083: 02_shell/src/cli.rs:1293 — E0004 primary 1293:11 bytes 45670..45677 (`content`).
- V16-084: 02_shell/src/cli.rs:1429 — E0004 primary 1429:11 bytes 50175..50182 (`content`).
- V16-085: 02_shell/src/cli.rs:1775 — E0004 primary 1775:11 bytes 63882..63889 (`content`).
- equation-shell-recognizer: 02_shell/src/cli.rs:1156 — E0004 primary 1156:11 bytes 41067..41074 (`content`).

## Scope

Compiler witness proves enum-domain review coverage, not language correctness.
Shell witnesses remove positive arms in actual shell matches; they are not future-variant cross-crate compilation witnesses.
Core baseline is an intentionally failing extension build with unrelated E0004s; it is not a successful unmutated build.

The worker restores every Rust source before each mutation group. The core group adds only the sentinel Content variant. The shell group removes only the selected positive arms, compiling against unchanged Content. Input hashes, exact mutated bytes and unchanged final freeze are verified in the JSON report.
