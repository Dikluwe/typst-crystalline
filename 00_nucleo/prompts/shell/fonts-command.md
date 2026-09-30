# L0 — Font inventory presentation
Hash do Código: 7c6204a0

**Layer:** L2
**Exclusive consumer:** `02_shell/src/fonts.rs`

## Evidence and ownership

`02_shell/src/fonts.rs:11` defines the presentation DTO and `:21` formats an inventory supplied in memory. Discovery and deterministic inventory ordering belong to `00_nucleo/prompts/infra/fonts.md`; parsing belongs to `00_nucleo/prompts/shell/cli.md`; publication belongs to `00_nucleo/prompts/wiring/fonts.md`. This prompt owns none of those consumers.

Family names, face details and display order are public presentation. Discovery mechanics and machine-specific absolute paths are not byte-parity requirements.

## Public contract

`FontDisplayEntry` has `family: String`, `path: PathBuf`, `index: u32`, `style: String`, `weight: u16`, `stretch: u16` and `embedded: bool`. No `fontdb` or `ttf-parser` type crosses this L2 contract.

`pub fn format_fonts(entries: &[FontDisplayEntry], variants: bool) -> String` preserves supplied order. It prints a family name followed by newline whenever the family differs from the preceding entry. Thus a grouped inventory yields one line per family; the formatter does not sort or globally deduplicate arbitrary input. Empty input yields an empty string.

Without variants, print only family lines. With variants, insert a blank line between family groups and append for each face:

```text
  └ {origin}
     Style: {style}, Weight: {weight}, Stretch: {percentage}%
```

Both lines end with newline. Origin is `(Embedded)` for an embedded face, otherwise the displayed path, with `#{index}` only when index is nonzero. Percentage is the normal numeric display of `stretch as f64 / 10.0`; do not impose fixed decimal padding.

## Command integration and preserved obligation

`typst fonts [--font-path DIR ...] [--ignore-system-fonts] [--variants]` includes system fonts by default. Ignoring system fonts retains explicit paths and any genuinely available embedded fonts. Font paths follow the CLI's flag/environment precedence and empty-token filtering. L3 supplies a stable inventory and ignores invalid font data; L2 formats and L4 publishes.

The existing requirement that unreadable font paths produce a warning rather than a panic remains an unresolved conformance obligation. Current discovery silently skips these paths. Choosing silence or introducing a warning requires a behavioral decision; this documentation change authorizes neither.

Successful publication is exit 0. A stdout write failure is exit 2 with `error: failed to write font list: {error}`.

## Acceptance

Use grouped families and a multi-face collection to check stable order, deduplication supplied by discovery, variants, system-font exclusion and empty/invalid paths. Compare names and face fields, not machine-specific absolute paths. Assert literal indentation, origin/index notation and stretch presentation. The unreadable-path discrepancy is tracked in `00_nucleo/diagnosticos/sanitization-shell-history.md`.
