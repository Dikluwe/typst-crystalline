# L0 — Dynamic callout and badge fixtures
Hash do Código: 30bbe264

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/test_callout.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:eddb0b1b680907d9db343169884ebcc6e668d4d46a7e94146c6ddc018d4fe4f7

## Evidence and test-only contract

The consumer defines CalloutElem and BadgeElem implementing the same Element
trait as native elements and entering through the blanket dynamic bridge.
This module is test-only under `#[cfg(test)]`, not a native production element.

- Callout uses kind `callout`, exposes body/title/tone, recurses into body and
  produces text `title: body`.
- Badge uses kind `badge`; label is required. Explicit note wins over the chain,
  which fills only an absent note.
- Callout hashes Debug manually because Content does not implement Hash.

Verify text, fields, distinct kinds, body maps and settable precedence.
