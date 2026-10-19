# L0 — Compile-time shell build identity
Hash do Código: 24e7f743

**Layer:** L2 build tooling
**Exclusive consumer:** `02_shell/build.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shell/build-identity.toml sha256:a97f32705be8700feaa6a5c89440ffa49d15c11145ea83744307aefc65a50297

## Evidence and boundary

`02_shell/build.rs:6` declares rebuild inputs; its compile-time environment check and Git fallback implement the identity contract. The pinned build-identity nucleus supplies the shared invariants. This owner is the build script alone, not the runtime CLI. Runtime presentation belongs to `00_nucleo/prompts/shell/cli.md`.

## Contract

Emit these Cargo directives unchanged:

```text
cargo:rerun-if-env-changed=TYPST_COMMIT_SHA
cargo:rerun-if-changed=../.git/HEAD
cargo:rerun-if-changed=../.git/refs
```

If `option_env!("TYPST_COMMIT_SHA")` is absent when the build script is compiled, run `git rev-parse HEAD`. Successful execution with UTF-8 stdout publishes its trimmed value as `cargo:rustc-env=TYPST_COMMIT_SHA={value}`. A supplied compile-time value takes precedence. Spawn failures, unsuccessful Git status and non-UTF-8 output are silent; they leave the runtime consumer's fallback intact. Do not substitute a parity-reference commit for the crystalline build commit.

Build-time process and filesystem access are allowed here; the executable performs no Git lookup at runtime. Changes to Git HEAD or refs must invalidate the captured identity.

## Acceptance

Check explicit identity precedence, successful Git capture, unavailable Git/repository fallback and declared rebuild inputs. This is build provenance, not proof of language parity; a displayed version alone does not identify an oracle binary.
