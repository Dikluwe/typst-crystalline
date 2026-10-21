# Standalone Cell block-flow cause and bounded repair

The frozen candidate receipt proves the remaining morphology error: the isolated Cell source emits `body/p/h2(H)` rather than oracle `body/h2(H)`. Exit and diagnostics match. Bundle conversion now supplies the correct h2, but the document composer sees the nominal Cell wrapper and applies its generic paragraph fallback before inline emission reveals the h2.

The local correction belongs to document composition. Inspect the same selected visual as the inline Cell renderer (standalone visual, otherwise body) for block presence, direct body flow and sequence accumulation. Do not peel Cells in the entry map: that would erase Table row cells before the Table renderer owns td/th output. Preserve nominal fields/snapshots, inherited styles, quote state and plain inline grouping.

L0 must precede two functional block-flow controls and two preservation controls. The existing semantic DOM source remains immutable and must pass. Explicit html/body root recognition and all unsupported kinds remain unchanged. This is a continuous correction inside the existing semantic HTML renderer, with no new API/phase/default. No source, L0, test, policy or script was edited by this plan.
