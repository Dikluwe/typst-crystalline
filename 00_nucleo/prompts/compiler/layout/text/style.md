# L0 — Effective inline text style
Hash do Código: 702707c4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

Layer: L1. Exclusive consumer: `01_core/src/compiler/layout/text/style.rs`.

## Evidence before decision

`01_core/src/compiler/layout/text.rs:29-222` resolves TextStyle by typed and
custom chain precedence, including script scaling. Space previously only
used cached style at `01_core/src/compiler/layout/space.rs:14`; this is not
proof of the effective font at a lexical boundary. The approved carrier
requires actual font identity. No new parity measurement is claimed here.

## Contract

Own pure effective-style resolution extracted unchanged from Text. Preserve
field-specific precedence, typed size, custom font override, folded variations,
semantic strong/emph provenance, math fields, script scaling 0.65, explicit
script size and baseline offsets. Do not mutate layouter, do I/O, choose installed
fonts, or emit text. Text invokes once before its temporary style scope. Markup
and Newline separators invoke once at their current chain. Literal separators
already inside the resolved scope reuse that style to avoid scaling twice.
This internal descendant function introduces no public data or trait contract.
