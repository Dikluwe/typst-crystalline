# CLI stderr and watch: measured correction complete

CLI-owned stderr/status writes now propagate actual I/O failures and terminate
with2, preserving healthy messages/order/status. Watch cleans exact owned staging
paths before arming/publication after terminal reporting failure, and before
reporting a fatal publication error. Destination and live HTTP controls preserve
prior valid content; cleanup/report errors remain explicit.

[Full report](../../lab/results/stderr-watch-errors-2026-10-22/README.md) and
[completion](../../lab/results/stderr-watch-errors-2026-10-22/completion.json)
bind HEAD/uncommitted dirty stat, UTC, source/recipe and candidate SHA-256:
`d066a9a1dc50da488ff80e6ac7193835b3f7c0a6954dc04784bc858071f3f57f`. Eleven owner pairs, zero external public API changes.

Full workspace tests, all-target build/Clippy, rustdoc, hashes/lineage/paths,
Hash366, R7, F/G, all21 finite parity stages and CMYK/rounding/Box pass. Real stream
probes and24 post-implementation held-outs pass; every historical failure and
all lint notes remain. No alerts or contracts were weakened.

This closes CLI stderr/watch failures, **not all panic risks**. Direct L3 image
export warnings, init stdout and broader productive candidates remain; the L2
formatter expects were subsequently corrected in
[diagnostic formatter](diagnostic-formatter-2026-10-23.md). Successful
SVG-template watch publication is still open.
The inventory is not proof of reachability or universal panic freedom.
