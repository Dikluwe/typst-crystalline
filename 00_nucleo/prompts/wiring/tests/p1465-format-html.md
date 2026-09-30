# L0 — wiring/tests/p1465-format-html
Hash do Código: 58a60d1f

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1465_format_html.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
```

## Current evidence

- `04_wiring/tests/p1465_format_html.rs:136` — `Profile`.
- `04_wiring/tests/p1465_format_html.rs:158` — `ProcessResult`.
- `04_wiring/tests/p1465_format_html.rs:165` — `success`.
- `04_wiring/tests/p1465_format_html.rs:171` — `Checks`.
- `04_wiring/tests/p1465_format_html.rs:176` — `require`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1465_format_html.rs`.

## Owned contract

- Own format.html public surface and pretty transport, not general HTML parity. Cover default/a11y-extras/html/combined profiles.
- Without html, direct/alias/std/set-rule access fails with the exact gate, ordered hints and field span before evaluating a panic argument. Neither target, exporter nor a11y-extras activates html.
- With html, type/function short name/default false/manual-construction rejection and alias/std identity hold. Reuse elem/frame and the existing full tag namespace, including through with; unknown fields retain ordinary function errors.
- Cover true/false/empty/repeated rules, named/type/positional/duplicate diagnostics and gate-versus-argument evaluation order. Root pretty follows explicit CLI then document then false; at least two distinct container kinds reject format rules with `format set rules are not allowed inside of containers`.
- Compact/pretty preserve DOM/escaping and affect structural whitespace only; inline html.frame receives the external resolved boolean. Keep svg/png/pdf defaults, orthogonal PDF gates and Crystalline/Vanilla escaping. Bundle/new tags/targets remain outside this suite.
- Use serial bounded processes and isolated /dev/shm task roots with no disk fallback. OOM, capacity failure or timeout is inability to verify, never success; no total-byte/hash oracle.

## Verification and boundary

Run `cargo test -p typst --test p1465_format_html -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
