# Prompt L0 — `entities/region`
Hash do Código: bf2ac371

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/region.rs`
**References:** ADR-0078,ADR-0029,ADR-0036; later region extension ADR-0081.

## Domain and current interface

Region aggregates layout geometry previously scattered in Layouter:
public cursor_x,cursor_y,line_start_x,width,height:f64;
current_items,current_line:Vec<FrameItem>;
pending_fr:Vec<(usize,f64)> for insertion index/fraction of pending horizontal
fr spacing. Derive Debug/Clone, no required PartialEq/Eq/Hash. Pure L1,
no I/O; single Layouter ownership permits mutation, with owned Vec rather than
EcoVec. Width/height remain derived copies of PageConfig (the rejected design
moved them exclusively here).

new(width,height) zeros cursor/line_start_x and empties buffers.
reset() returns cursor to(line_start_x,0), clears buffers including pending_fr,
and preserves dimensions. Fractional spacing expands in flush_line/finish and
is cleared after expansion. has_pending() detects nonempty item buffers.
Insertion order remains stable; normal-layout cursor_x≥line_start_x,cursor_y≥0.

Regions derives Debug/Clone with public current:Region,backlog:Vec<Region>,
last:Option<Region>,cell:Option<Region>. single(width,height) constructs one
current region with empty backlog and absent last/cell. reset_current delegates
to current.reset. advance()->Option<Region>: with backlog, move current to
last, consume first backlog as current and return previous current; without
backlog return None so caller creates the next external/page region.
effective()->&Region selects active cell else current.
enter_cell(Region)->Option<Region> returns the saved cell for nesting;
exit_cell(saved) restores it, including None at outermost exit.

## Evidence and staged boundaries

The original single-region aggregation and minimal Regions were extended by
backlog/last, then transient cell. Those explicit later definitions supersede
old 'wrapper deferred' and 'no backlog/last' clauses. Initial empty production
backlog/last was an infrastructure stage, not a permanent restriction. Actual
column-flow population and breakable cell behavior remain separately owned.
The abstraction itself does not prove full multiregion or cell-break closure.

Upstream regions.rs uses size/expand/full separately from cursors/buffers;
crystalline intentionally combines state, uses f64 dimensions, leaves expansion
to flush helpers and fullness to cursor_y/height. No expand/full additions are
authorized without a real consumer. Cell height/width replace old
cell_available_h/cell_origin_w; absolute cell_origin_x/y remain in Layouter
because Region has no origin Point. Their future migration needs a contract.

## Acceptance

Keep p216a_region_new_inicia_cursor_zero,
p216a_region_reset_preserva_dimensoes,
p216a_region_has_pending_false_apos_new,p216a_region_clone_funciona.
Preserve zero initialization,reset dimensions/left edge,empty-state detection,
clone,both advance branches and nested enter/exit restoration. Block/Boxed/
TableCell overflow may consult effective geometry but enabling breakability
is a separate semantic step.

## Deferred cell-tail handoff constraints

The original later annotation concerns layout owners, not entity implementation.
Its pending_cell_tails Vec<DeferredCellTail> stores items:Vec<FrameItem> with
rebased y,origin_x:f64,width:f64,fill:Option<Color>,stroke:Option<Stroke>,
forwarded_count:u32. Auto/Fraction row overflow slices at body_h: emit head
now and queue tail. Next new_page flushes floats, closes/setup page, then
flushes cell tails at the top with fill→rebased items→stroke order.
Fixed rows retain implicit clipping and do not use this break path.

Atomic Group/Shape items do not split internally; complete items move to tail.
Reemit fill/stroke per fragment; forwarding is capped at 3 iterations.
Other cells of the original row do not continue, an explicit row-level
limitation. Pure slicing/rebase helpers preserve exhaustive FrameItem handling;
the historical six-variant list is not permission to ignore later variants.
These constraints and the original per-cell-only/multiregion debt remain
traceable in the archive; this owner neither implements layout nor certifies
those handoffs complete.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/region.md`. Historical observations are not a new parity certification.
