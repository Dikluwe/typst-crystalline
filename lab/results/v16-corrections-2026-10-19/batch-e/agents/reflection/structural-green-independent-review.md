# Independent green structural evidence review


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


Exact archived mutations were reconstructed independently before deriving UTF-8 diagnostic locations. Each selected target has exactly one raw primary E0004. Both compiler groups ended unsuccessfully as intended; no other compiler error codes occur.

All archived structural input hashes match the current source state. Shared source inputs and every frozen owner match the accepted candidate-final source receipt, whose freeze is unchanged. Exact state timestamps, HEAD, changed-file stat, receipt hashes and primary diagnostic spans appear in the JSON companion.

Core adds the sentinel Content variant. Shell removes positive arms and is not a cross-crate future-variant witness. Additional helper targets are green-only; no selected baseline result is claimed for them. This proves local closed-domain coverage, not language parity.

The original baseline-only report was preserved as structural-baseline-independent-review.json/md. No compiler, Cargo or sealing command was run during this review.
