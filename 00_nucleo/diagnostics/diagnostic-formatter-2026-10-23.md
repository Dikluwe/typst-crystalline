# Diagnostic formatter: measured panic correction complete

The L2 human diagnostic formatter now validates principal ranges before passing
them to `codespan-reporting`: ordered, in bounds, and at UTF-8 boundaries. A
publicly constructible `9..1` span previously exited 101 through a productive
`expect`; it now preserves the diagnostic and omits only the invalid location.
Reversed and mid-codepoint ranges no longer produce artificial carets.

The two public formatter signatures and pipeline phase are unchanged. No alert,
hint, trace, healthy source block, ANSI behavior, or existing assertion was
suppressed. The productive formatter contains no `expect`, `unwrap`, `panic!`,
or `unreachable!`.

[Full report](../../lab/results/diagnostic-formatter-2026-10-22/README.md) and
[completion receipt](../../lab/results/diagnostic-formatter-2026-10-22/completion.json)
bind the before probe, tests-first RED, after probe, exact source, L0, candidate,
and receipts. Candidate SHA-256:
`4fd9a7688975956927bfb6f761543fcc5e48a624bc2ac5a2dd4788ad1c4eaa21`.

All 12 workspace validation jobs, full Tekt lint, forward/reverse lineage, and
all 21 finite parity stages pass on the same frozen candidate. Direct L3 export
warnings, init stdout, and broader productive panic candidates remain open; this
does not claim whole-project panic freedom.
