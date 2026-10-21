# Direct descendant dispatch: per-site routing classification

Source-only review at clean HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`. All118 sites from the immutable direct inventory are classified below. This does not imply all56 owners require edits. Full source/L0 hashes and per-site mandatory proof obligations are in `descendant-site-classification.json`.

| Source | Line | Category | Typed route | Required unresolved proof |
|---|---:|---|---|---|
| `bibliography.rs` | 58 | deferred_replay_or_move | BibliographyEntry(index).Prefix through generated Place wrapper | Carry generated/borrowed prefix provenance with PendingPrefix; establish whether existing CSL plan contains admitted source children before choosing None. |
| `bibliography.rs` | 170 | source_child_edge | BibliographyTitle | None at this direct site; preserve stated evidence. |
| `bibliography.rs` | 189 | generated_wrapper_with_source_child | BibliographyPlanEntry(index).Body or Prefix under generated entry Block | Resolve immutable CSL/legacy generated-entry plan boundary against its current L0. Do not consume BibliographyTitle or next raw child. Any generated-only None claim must prove all nested semantic entries were already admitted or remain outside existing discovery. |
| `bibliography.rs` | 197 | generated_wrapper_with_source_child | BibliographyPlanEntry(index).Body or Prefix under generated entry Block | Resolve immutable CSL/legacy generated-entry plan boundary against its current L0. Do not consume BibliographyTitle or next raw child. Any generated-only None claim must prove all nested semantic entries were already admitted or remain outside existing discovery. |
| `bibliography.rs` | 222 | generated_wrapper_with_source_child | BibliographyPlanEntry(index).Body or Prefix under generated entry Block | Resolve immutable CSL/legacy generated-entry plan boundary against its current L0. Do not consume BibliographyTitle or next raw child. Any generated-only None claim must prove all nested semantic entries were already admitted or remain outside existing discovery. |
| `bibliography.rs` | 273 | generated_wrapper_with_source_child | BibliographyPlanEntry(index).Body or Prefix under generated entry Block | Resolve immutable CSL/legacy generated-entry plan boundary against its current L0. Do not consume BibliographyTitle or next raw child. Any generated-only None claim must prove all nested semantic entries were already admitted or remain outside existing discovery. |
| `bibliography.rs` | 287 | generated_wrapper_with_source_child | BibliographyPlanEntry(index).Body or Prefix under generated entry Block | Resolve immutable CSL/legacy generated-entry plan boundary against its current L0. Do not consume BibliographyTitle or next raw child. Any generated-only None claim must prove all nested semantic entries were already admitted or remain outside existing discovery. |
| `block.rs` | 364 | source_child_edge | BlockBody | None at this direct site; preserve stated evidence. |
| `boxed.rs` | 231 | source_child_edge | BoxedBody | None at this direct site; preserve stated evidence. |
| `boxed.rs` | 497 | source_child_edge | BoxedBody | None at this direct site; preserve stated evidence. |
| `cite.rs` | 41 | generated_only_render_output | GeneratedCitationText | Formatted cached Content may be rich; prove its existing semantic boundary before setting None. Never treat e.supplement as generated. |
| `cite.rs` | 43 | source_child_edge | CitationSupplement | None at this direct site; preserve stated evidence. |
| `cite.rs` | 70 | generated_only_render_output | GeneratedCitationText | None at this direct site; preserve stated evidence. |
| `cite.rs` | 72 | source_child_edge | CitationSupplement | None at this direct site; preserve stated evidence. |
| `cite.rs` | 94 | generated_only_render_output | GeneratedCitationText | None at this direct site; preserve stated evidence. |
| `cite.rs` | 96 | source_child_edge | CitationSupplement | None at this direct site; preserve stated evidence. |
| `cite.rs` | 140 | generated_only_render_output | GeneratedCitationText | None at this direct site; preserve stated evidence. |
| `cite.rs` | 142 | source_child_edge | CitationSupplement | None at this direct site; preserve stated evidence. |
| `columns/flow.rs` | 119 | source_child_edge | ColumnsBody | None at this direct site; preserve stated evidence. |
| `convergence.rs` | 138 | deferred_replay_or_move | PreparedBodyRoot for current layout attempt | Root cursor and trace generation reset per accepted attempt; do not accumulate previous candidate events. |
| `counter_display.rs` | 32 | generated_only_render_output | GeneratedCounterText | None at this direct site; preserve stated evidence. |
| `counter_display_callback.rs` | 19 | generated_only_render_output | GeneratedCounterDisplayResult | Arbitrary rich callback result is not a certified textual tree. Existing runtime/semantic boundary must be proven; no automatic None or new callback realization phase. |
| `decorations.rs` | 49 | source_child_edge | DecorationBody | None at this direct site; preserve stated evidence. |
| `dynamic.rs` | 40 | generated_only_render_output | OpaqueDynamicBody | Retain current Dynamic opaque semantic discovery contract. No generic dynamic child plan or guessed next child; mandatory owner proof for rich rendering result. |
| `dynamic.rs` | 45 | generated_only_render_output | GeneratedDynamicPlainText | None at this direct site; preserve stated evidence. |
| `enum_item.rs` | 94 | source_child_edge | EnumItemBody | None at this direct site; preserve stated evidence. |
| `figure.rs` | 123 | source_child_edge | FigureBody | None at this direct site; preserve stated evidence. |
| `figure.rs` | 127 | generated_only_render_output | GeneratedCaptionLinebreak | None at this direct site; preserve stated evidence. |
| `figure.rs` | 131 | generated_wrapper_with_source_child | Sequence(GeneratedPrefix[FigureSelectedSupplement?,GeneratedNumberText],FigureCaption) | Map selected supplement and cap to separate declared source edges; only spacing/number suffix/Sequence wrapper are generated. Whole-call None loses caption and supplement descendants. |
| `figure.rs` | 133 | source_child_edge | FigureCaption | None at this direct site; preserve stated evidence. |
| `flow_suffix.rs` | 42 | deferred_replay_or_move | Same occurrence packet during first trial/replay | Store cursor with replay item and checkpoint event/resolution suffix; rejected first trial consumes nothing permanently. |
| `flow_suffix.rs` | 88 | deferred_replay_or_move | Same occurrence packet during first trial/replay | Store cursor with replay item and checkpoint event/resolution suffix; rejected first trial consumes nothing permanently. |
| `footnote.rs` | 48 | generated_only_render_output | GeneratedFootnoteMarker | None at this direct site; preserve stated evidence. |
| `footnote_flush.rs` | 142 | deferred_replay_or_move | FootnoteBody(queued occurrence) through generated marker/body composition | Queue original body cursor with n/body; route body inside combined wrapper. Admit at actual accepted physical placement; do not re-identify from clone or geometry. |
| `grid/cell_body.rs` | 335 | generated_wrapper_with_source_child | SelectedCellVisual under optional generated alignment/Block | Transport typed selected-cell cursor both into fallback composition and alongside cached measured product; publish its event block only when reused for actual emission. |
| `grid/tracks.rs` | 224 | measurement_only_trace_none | No admission during auto-width visual measurement | None at this direct site; preserve stated evidence. |
| `grid/tracks.rs` | 543 | deferred_replay_or_move | Cell(index) measured product retained for actual composition | Retain child cursor and unpublished event block with cached geometry. Do not call this pure None measurement if later reuse skips actual source admission. |
| `grid_cell.rs` | 19 | source_child_edge | GridCellBody | None at this direct site; preserve stated evidence. |
| `grid_footer.rs` | 19 | source_child_edge | GridFooterBody | None at this direct site; preserve stated evidence. |
| `grid_header.rs` | 19 | source_child_edge | GridHeaderBody | None at this direct site; preserve stated evidence. |
| `heading.rs` | 117 | generated_only_render_output | GeneratedHeadingNumberPrefix | None at this direct site; preserve stated evidence. |
| `heading.rs` | 126 | source_child_edge | HeadingBody | None at this direct site; preserve stated evidence. |
| `hide.rs` | 22 | source_child_edge | HideBody | None at this direct site; preserve stated evidence. |
| `html_frame.rs` | 100 | source_child_edge | HtmlFrameBody | None at this direct site; preserve stated evidence. |
| `html_frame.rs` | 227 | measurement_only_trace_none | Independent HTML intrinsic-layout target, Paged trace None | None is only for the proposed Paged running trace, not a claim that HTML intrinsic rendering is semantically empty. Preserve existing HTML positioning protocol separately. |
| `line_commit.rs` | 790 | generated_only_render_output | GeneratedParLineCallbackResult | Rich callback Content boundary must remain current; prove no admitted semantic source cursor is present. Do not make line-number output a new source insertion phase. |
| `link_capture.rs` | 213 | source_child_edge | LinkBody or caller-declared DestinationBody | None at this direct site; preserve stated evidence. |
| `list_item.rs` | 78 | source_child_edge | ListItemBody | None at this direct site; preserve stated evidence. |
| `measurement.rs` | 242 | measurement_only_trace_none | No source admission in isolated measurement Layouter | None at this direct site; preserve stated evidence. |
| `measurement.rs` | 746 | measurement_only_trace_none | No source admission in isolated measurement Layouter | None at this direct site; preserve stated evidence. |
| `mod.rs` | 1228 | deferred_replay_or_move | Same current root packet across dispatcher gates | Preserve exact current cursor without consuming twice; source span peeling must select SourceBody in plan without changing raw classifiers. |
| `mod.rs` | 1251 | source_child_edge | SourceBody | None at this direct site; preserve stated evidence. |
| `mod.rs` | 1274 | source_child_edge | LocatedBody | None at this direct site; preserve stated evidence. |
| `mod.rs` | 1283 | source_child_edge | ShowRecipeBody | None at this direct site; preserve stated evidence. |
| `mod.rs` | 1411 | deferred_replay_or_move | Same current root packet across dispatcher gates | Preserve exact current cursor without consuming twice; source span peeling must select SourceBody in plan without changing raw classifiers. |
| `mod.rs` | 1545 | deferred_replay_or_move | Same current root packet across dispatcher gates | Preserve exact current cursor without consuming twice; source span peeling must select SourceBody in plan without changing raw classifiers. |
| `outline.rs` | 63 | source_child_edge | OutlineRealizedBody | Use owning realized-outline plan; do not traverse original/raw semantic references again. |
| `outline.rs` | 93 | generated_wrapper_with_source_child | GeneratedHeading(OutlineTitle or generated localized default) | Conditional edge must preserve custom title under generated Heading while default localized text has None. Existing outline indexing rules decide whether title is a new semantic Heading; do not invent new membership. |
| `outline.rs` | 146 | generated_only_render_output | GeneratedReadonlyOutlineLine with borrowed frozen heading body | Prove existing borrowed-reference/no-repeat-action boundary; preserve readonly semantics and do not consume original HeadingBody cursor a second time. |
| `outline.rs` | 162 | generated_wrapper_with_source_child | GeneratedHeading(OutlineTitle or generated localized default) | Conditional edge must preserve custom title under generated Heading while default localized text has None. Existing outline indexing rules decide whether title is a new semantic Heading; do not invent new membership. |
| `outline.rs` | 174 | generated_only_render_output | GeneratedLOFOrLOTLineText | None at this direct site; preserve stated evidence. |
| `outline.rs` | 187 | generated_wrapper_with_source_child | GeneratedHeading(OutlineTitle or generated localized default) | Conditional edge must preserve custom title under generated Heading while default localized text has None. Existing outline indexing rules decide whether title is a new semantic Heading; do not invent new membership. |
| `outline.rs` | 199 | generated_only_render_output | GeneratedLOFOrLOTLineText | None at this direct site; preserve stated evidence. |
| `outline_entry.rs` | 15 | generated_only_render_output | BorrowedReferencedHeadingBody | Existing OutlineEntry semantic partition treats reference fields specially; require owner proof, no fresh source packet by structural clone equality. |
| `pad.rs` | 64 | source_child_edge | PadBody | None at this direct site; preserve stated evidence. |
| `page_canvas.rs` | 28 | source_child_edge | SelectedPhysicalBackgroundOrForeground | Slot root owns independent instantiated plan; trace body parent must not leak into canvas; retained request matching supplies exact slot. |
| `page_run.rs` | 89 | source_child_edge | PageRunBody | None at this direct site; preserve stated evidence. |
| `page_running.rs` | 56 | source_child_edge | SelectedPhysicalHeaderOrFooter | Caller supplies exact retained physical slot plan, not current source body cursor or raw PageConfig template pointer. |
| `page_running.rs` | 136 | generated_only_render_output | GeneratedPageNumberingContent | Can be rich callback formatting output; certify current numbering discovery boundary, not blanket fresh admission or automatic None based on plain_text width. |
| `paragraph.rs` | 287 | measurement_only_trace_none | No source admission during isolated text/prefix/first-fragment measurement | None at this direct site; preserve stated evidence. |
| `paragraph.rs` | 342 | measurement_only_trace_none | No source admission during isolated text/prefix/first-fragment measurement | None at this direct site; preserve stated evidence. |
| `paragraph.rs` | 422 | measurement_only_trace_none | No source admission during isolated text/prefix/first-fragment measurement | None at this direct site; preserve stated evidence. |
| `paragraph.rs` | 662 | source_child_edge | ParagraphBody | Fast plan must account for any semantic packets, or prove its certified pure-text input has none; fallback cannot be the only cursor route. |
| `pdf_artifact.rs` | 100 | source_child_edge | PdfArtifactBody | None at this direct site; preserve stated evidence. |
| `place.rs` | 47 | deferred_replay_or_move | PlaceBody retained with float/placement product | Carry cursor/event block with pending float and commit at accepted placement; do not infer semantic page from initial measuring cursor. |
| `placement.rs` | 160 | deferred_replay_or_move | PlaceBody through actual geometric placement subframe | Commit exact source child once at final accepted placement; preserve rollback when flow changes. |
| `placement.rs` | 476 | deferred_replay_or_move | PlaceBody through actual geometric placement subframe | Commit exact source child once at final accepted placement; preserve rollback when flow changes. |
| `quote.rs` | 45 | generated_only_render_output | GeneratedQuotePunctuation | None at this direct site; preserve stated evidence. |
| `quote.rs` | 47 | source_child_edge | QuoteBody | None at this direct site; preserve stated evidence. |
| `quote.rs` | 49 | generated_only_render_output | GeneratedQuotePunctuation | None at this direct site; preserve stated evidence. |
| `quote.rs` | 54 | generated_only_render_output | GeneratedQuotePunctuation | None at this direct site; preserve stated evidence. |
| `quote.rs` | 55 | source_child_edge | QuoteAttribution | None at this direct site; preserve stated evidence. |
| `quote.rs` | 62 | generated_only_render_output | GeneratedQuotePunctuation | None at this direct site; preserve stated evidence. |
| `quote.rs` | 64 | source_child_edge | QuoteBody | None at this direct site; preserve stated evidence. |
| `quote.rs` | 66 | generated_only_render_output | GeneratedQuotePunctuation | None at this direct site; preserve stated evidence. |
| `quote.rs` | 69 | generated_only_render_output | GeneratedQuotePunctuation | None at this direct site; preserve stated evidence. |
| `quote.rs` | 70 | source_child_edge | QuoteAttribution | None at this direct site; preserve stated evidence. |
| `realized.rs` | 573 | source_child_edge | RealizedVisual | Parent plan must retain original-only and visual memberships; alternate paragraph path also needs same visual cursor. |
| `references.rs` | 38 | source_child_edge | LabelBody | None at this direct site; preserve stated evidence. |
| `repeat.rs` | 34 | source_child_edge | RepeatBodyOnce | None at this direct site; preserve stated evidence. |
| `sequence.rs` | 332 | source_child_edge | SequencePart(index) | Use original typed index through lookahead/preflight/segmented slices; do not guess next child after paragraph fast paths. |
| `stack.rs` | 169 | source_child_edge | StackChild(index) | Carry source index through composed subframes and merge accepted events in actual owner order. |
| `stack.rs` | 234 | source_child_edge | StackChild(index) | Carry source index through composed subframes and merge accepted events in actual owner order. |
| `state_display.rs` | 19 | generated_only_render_output | GeneratedStateDisplayResult | Arbitrary rich result requires current display semantic-boundary proof before None; do not add State display realization phase or duplicate already recorded effects. |
| `style_scope.rs` | 143 | source_child_edge | StyledBody | None at this direct site; preserve stated evidence. |
| `style_scope.rs` | 160 | source_child_edge | StyledBody | None at this direct site; preserve stated evidence. |
| `style_scope.rs` | 198 | source_child_edge | StrongBody | None at this direct site; preserve stated evidence. |
| `style_scope.rs` | 215 | source_child_edge | EmphBody | None at this direct site; preserve stated evidence. |
| `style_scope.rs` | 227 | source_child_edge | SmallCapsBody | None at this direct site; preserve stated evidence. |
| `sub_frame.rs` | 76 | deferred_replay_or_move | Caller-supplied exact child cursor or explicit None | Transport caller cursor without advancing a guessed edge, isolate stack, checkpoint event suffix and restore caller. Actual emission returns an event block; measurement returns none. Inline/cell paths obey same rule. |
| `sub_frame.rs` | 90 | deferred_replay_or_move | Caller-supplied exact child cursor or explicit None | Transport caller cursor without advancing a guessed edge, isolate stack, checkpoint event suffix and restore caller. Actual emission returns an event block; measurement returns none. Inline/cell paths obey same rule. |
| `sub_frame.rs` | 263 | deferred_replay_or_move | Caller-supplied exact child cursor or explicit None | Transport caller cursor without advancing a guessed edge, isolate stack, checkpoint event suffix and restore caller. Actual emission returns an event block; measurement returns none. Inline/cell paths obey same rule. |
| `sub_frame.rs` | 490 | deferred_replay_or_move | Caller-supplied exact child cursor or explicit None | Transport caller cursor without advancing a guessed edge, isolate stack, checkpoint event suffix and restore caller. Actual emission returns an event block; measurement returns none. Inline/cell paths obey same rule. |
| `table.rs` | 87 | generated_wrapper_with_source_child | GeneratedTableCaptionPrefix plus TableCaption | Only prefix and wrapper are generated; route cap to exact TableCaption edge. Whole-call None loses nested source caption semantics. |
| `table.rs` | 89 | source_child_edge | TableCaption | None at this direct site; preserve stated evidence. |
| `table.rs` | 91 | generated_only_render_output | GeneratedTableCaptionLinebreak | None at this direct site; preserve stated evidence. |
| `table_cell.rs` | 40 | generated_wrapper_with_source_child | GeneratedBlock/Align around CellSelectedVisual | Wrapper nodes have None; exact embedded visual carries CellSelectedVisual edge. Do not consume dormant raw/resolved alternative. |
| `table_cell.rs` | 44 | source_child_edge | CellSelectedVisual | None at this direct site; preserve stated evidence. |
| `table_footer.rs` | 19 | source_child_edge | TableFooterBody | None at this direct site; preserve stated evidence. |
| `table_header.rs` | 19 | source_child_edge | TableHeaderBody | None at this direct site; preserve stated evidence. |
| `term_item.rs` | 35 | source_child_edge | TermItemTerm | None at this direct site; preserve stated evidence. |
| `term_item.rs` | 38 | generated_only_render_output | GeneratedTermSeparator | None at this direct site; preserve stated evidence. |
| `term_item.rs` | 39 | source_child_edge | TermItemDescription | None at this direct site; preserve stated evidence. |
| `terms.rs` | 24 | source_child_edge | TermsItem(index) | None at this direct site; preserve stated evidence. |
| `tiling.rs` | 164 | measurement_only_trace_none | Tiling paint-cell composition, outside source occurrence admission | Mandatory owner proof: preserve one source semantic admission if existing contract requires it, outside this repeated paint-cell composition; do not infer that every tiled paint is a fresh source occurrence or silently drop source metadata. |
| `title.rs` | 42 | source_child_edge | TitleBody | None at this direct site; preserve stated evidence. |
| `transform.rs` | 49 | source_child_edge | TransformBody | Preserve exact typed body cursor through native fallback/compose paths; no event replication when frame is transformed. |
| `transform.rs` | 649 | source_child_edge | TransformBody | Preserve exact typed body cursor through native fallback/compose paths; no event replication when frame is transformed. |

Counts are an inventory metric, not language parity. Rich display/callback/bibliography/outline output, cached Grid products, repeat/tiling and HTML target isolation require the explicit owner-specific proofs listed above; no blanket generated-output None or unconditional fresh semantic admission is authorized.

## Operational PageRun template boundaries outside direct layout calls

`compiler/realize/recipes.rs:38,70` uses `map_content_pre_with_origin`; PageRun remains in its ordinary declared traversal rather than an isolated physical-template boundary. `entities/content/traversal.rs` explicitly reconstructs static PageRun background/header/body/footer/foreground children. This means a raw PageRun marginal can be prepared once before physical repetition, unlike the deliberately opaque SetPage delta. A captured ShowRecipe by itself does not make its descendant immune to generic maps.

`compiler/realize/mod.rs:50-60` applies bottom-up map_content and locates every payload-bearing descendant that is not already Located/Realized. `realize_document` calls recipes → finish_occurrences → normalize_insertions at419-420; normalization itself maps Content at88. `observe_occurrences` also maps and reserves existing identities at234-235. Therefore the selected new PageRun template channel must keep operational marginal templates outside these preparation/identity/normalization passes until actual physical insertion, while continuing to prepare the PageRun body normally. It must not blindly strip genuine preexisting Located aliases from user input, or alter global map transparency. The exact owning changes belong to realize/mod, recipes and the proposed feature-local preparation driver, not an unapproved generic entities mapper contract change.

`realize_math_insertions` at433-470 re-enters recipes::realize with captured causal environment. `realize_table_insertions` at580-585 delegates the existing table_cells feature; recipes.rs:263-280 may run that early. These are concrete re-entry boundaries to audit for PageRun marginal protection. No measured independent defect in table_cells or bare_math is claimed here, and no blanket edit to their traversal/catalogue is authorized. If operational templates reach these entries, the owning preparation driver must carry the explicit protected-template boundary; otherwise demonstrate their inputs cannot contain it. Raw Context IDs remain fresh per actual raw insertion and retained across retries in the existing arena; one eagerly prepared Located marginal repeated per page is not equivalent.

The draft public PageRunningBody/preparation surface must declare this protection before code. Existing source/visual reflection snapshots and PageRun lexical restoration remain unchanged; parent-owned actual PageRun contrast already proves its static body-page store is not a complete passing control.
