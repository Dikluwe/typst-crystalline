# Layout and stdlib V16 cause analysis — plan only

Baseline: `23bcda80d8348b08e5199312fddeec8ece2eb0a5`. Static review completed at `2026-10-19T10:13:35.549706+00:00`. All 37 inventory rows and their frozen source/L0 hashes were checked. No Rust, L0 or configuration changes and no tests, build, lint or bilateral probes were run.

V16 identifies a missing nominal future-variant decision. It does not establish that a present fallback is a language bug. Several warnings are valid negative projections, cache refusals, forwarding/delegated operations or slice/product domains. Others expose approximations or potential parity debt requiring pinned evidence. The JSON contains the full current enum complements, residual guards and the complete25-pair line-number alignment product.

## Evidence and decision discipline

- Every corrective implementation first updates its existing1:1 owning L0; shared claims use existing pinned nuclei only when genuinely shared. V15/V26 must remain valid before any later reseal.
- Preserve L1 purity, injected interfaces, data dimensions/units, phase/effect order, static exhaustive dispatch and ADR-0109 form B. Do not add entity render logic, dynamic dispatch or duplicate algorithms.
- No blind complement expansion: use each row’s semantic domain/payload guards/caller invariants to decide current variants. No wildcard relocation into if-let/matches!/helper, linter/config weakening, allow/expect or omitted test observation.
- Preserve intentional neutral/unsupported/delegated domains visibly. New variants must require a new decision rather than default success, cache admission, zero geometry or no-op.
- V19/V20 informational notes can legitimately grow from explicit OR groups; retain and review each alternative with relevant functional tests, never suppress notes to improve counts.
- Separate nominal repair from possible parity debt. Unknowns require pinned source and bilateral probes before behavior classification. Any public signature/field/default/phase/compatibility expansion gets its own ADR-0127 decision.
- All proposed tests/probes are NOT RUN. Future heavy validation must preserve the project resource constraints and exact HEAD/dirty paths/UTC/source/binary/oracle provenance; use current pinned bd5ebec285505889e8ff1d4f68804ab147a699f3, not historical9f2b6e871 as current certificate.

Structural mutation precedent: `00_nucleo/prompts/core/entities/value.md:154-160`. A later disposable-copy sentinel must produce attributable E0004 at each selected enum match, not merely fail compilation elsewhere. Option/nested/product matches require the inner enum decision site; cfg(test) finders require test compilation. V16-056 is a slice cardinality partition and needs zero/one/two/many controls, not an enum-sentinel claim. None of these checks has been run.

## Priority and dependencies

1. Preserve behavior while documenting justified current partitions: cell overrides (029/030/033/034), safe cache boundaries (031/032), projection traversal (035), placement policies (036), optional numbering/quote observers (039/041/042), finite alignment domains (043/044), reference and counter default keys (045/054), validated font/outline schemas (046/047/070), HTML union validators (057/058), dash validation (061), alignment search (062), field delegation/YAML classes (063/064), arity (056) and target-finder test boundaries (068/069). Each still needs its own L0 and functional proof before nominal implementation.
2. Keep documented approximation boundaries explicit: legacy/constrained measurements (037/040), image dimension projection (038) and collection constructor adapter subset (053). Investigate actual reachable language extent/constructor loss rather than pronounce the complement intrinsically zero/noncallable.
3. Do not repair possible semantic debts from lint alone: Figure inference is explicitly open FG1 (055); invalid track filtering (059), Place dx/dy (060), Circle radius (065), legacy Line dx/dy (066) and bibliography locale (067) require pinned probes and gate classification. Current fallbacks are observations to preserve until those decisions are supported.

## Per-row findings

### V16-029 — cell_effective_stroke

Source: `01_core/src/compiler/layout/grid/borders.rs:49`; complete function reviewed: `01_core/src/compiler/layout/grid/borders.rs:39-52`. Scrutinee: cell: &Content; enum/domain: Content. Classification: **negative_projection_missing_nominal_coverage**.

**Evidence.** Non-cell means no cell-specific stroke, then grid_stroke is inherited; prepared TableCell returns resolved.stroke directly, including known None. References: `01_core/src/compiler/layout/grid/borders.rs:43-51`, `01_core/src/compiler/layout/grid/borders.rs:191,239-241`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/grid/borders.md` (SHA-256 `384ac906a0051868c9462eb22d487c04e64f8b41973ef11f3360bf0ec17b0ea3`, effective seal `2b729452`): Exclusive ownership and current contract: prepared external stroke, known None, Grid/raw priority; paint/border coalescing unchanged..

**Complement and invariant.** All non-cell bodies may occupy a raw grid cell but have no outer cell override. They are not skipped from rendering. TableCell resolved/unresolved branches are both covered; the guard cannot fall into raw inheritance. Callers pass PlacedCell.body; raw content is explicitly allowed by placement. Do not unwrap arbitrary styled/raw bodies as cell metadata.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Cell stroke versus inherited grid stroke; prepared known None must defeat fallback even with replaced/hidden body. Raw Text/Shape/Empty/wrapper bodies inherit grid stroke without changing body geometry; future Content sentinel must break the classifier compile.

**Dependencies.** grid_placement::PlacedCell, TableCellResolved, grid/cell_body external-fill boundary. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-030 — Layouter::emit_grid_cell_body

Source: `01_core/src/compiler/layout/grid/cell_body.rs:103`; complete function reviewed: `01_core/src/compiler/layout/grid/cell_body.rs:47-526`. Scrutinee: cell = &placed.body; enum/domain: Content. Classification: **negative_projection_missing_nominal_coverage**.

**Evidence.** Raw non-cell content yields five absent local overrides and still proceeds through layout_sub_frame/emission. Prepared plan fields bypass container fallback for known None. References: `01_core/src/compiler/layout/grid/cell_body.rs:72-139,168-228,261-525`, `01_core/src/compiler/layout/grid/cell_emission.rs:87`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/grid/cell_body.md` (SHA-256 `1843854d5c5aa500dab2c6571a9ee201be5e558edc4e07f4466ff9f6e3044872`, effective seal `a839e62f`): Exclusive current contract: positioned cell emission; prepared external props versus surviving-default inset/alignment; Grid/raw fallback..

**Complement and invariant.** Raw content has no outer cell stroke/fill/align/inset/breakable; Grid/raw fallback must inherit while prepared TableCell uses its plan. This classification is not a license to erase wrapper contents. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Prepared replacement/hidden/default snapshot: paint independent, inset/alignment only when surviving default; known None wins. Raw Text/Shape/Sequence retains grid defaults, body bounds, clipping/tails/decor and state restoration on cache hit/miss.

**Dependencies.** prepared_default, measurement_cache::take, TableCellResolved, cell_bidi scope. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-031 — simple_placement

Source: `01_core/src/compiler/layout/grid/measurement_cache.rs:59`; complete function reviewed: `01_core/src/compiler/layout/grid/measurement_cache.rs:40-61`. Scrutinee: content.source_body(); enum/domain: Content. Classification: **positive_admission_predicate_needs_nominal_boundary**.

**Evidence.** Cells are simple only with unit spans/no explicit coords; every other outer Content currently returns true. References: `01_core/src/compiler/layout/grid/measurement_cache.rs:40-61,150-169,185-266`, `01_core/src/entities/content/source.rs:11`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/grid/measurement_cache.md` (SHA-256 `6e0c89e609f2a655b7e06e50d3b0f33d148ac1e0d2806971823885abea8fec96`, effective seal `2947ebae`): Exclusive ownership and eligibility: whole-set simple placement; source_body removes only Source; no context/identity/unknown-carrier expansion..

**Complement and invariant.** True means absence of outer placement fields only, not cache eligibility. token/capture/take separately refuse context, arbitrary wrappers and non-single-leaf products. Source is stripped transparently; Styled is not stripped here. Do not promote this true into a generic purity certificate. MeasuredCells::new combines simple_placement with ambient checks; capture/take require token, one exact Text leaf, style/region/coordinates/unit spans. Source cannot reach the nominal fallback after source_body. Guard residuals: Source is normalized out; all other complement variants are only placement-field-neutral, not cache-approved..

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Each cell x/y/span variation disables the whole cache; raw token remains eligible only if all later guards pass. Source(special cell) stays refused; Styled wrapping special placement/context never obtains a cache hit merely because simple_placement is true.

**Dependencies.** token, MeasuredCells::new/capture/take. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-032 — token

Source: `01_core/src/compiler/layout/grid/measurement_cache.rs:100`; complete function reviewed: `01_core/src/compiler/layout/grid/measurement_cache.rs:63-102`. Scrutinee: content.source_body(); enum/domain: Content. Classification: **safety_refusal_missing_nominal_coverage**.

**Evidence.** None refuses cached-product reuse; it does not suppress layout. Miss returns to normal cell sublayout. References: `01_core/src/compiler/layout/grid/measurement_cache.rs:63-102,104-146,185-266`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/grid/measurement_cache.md` (SHA-256 `6e0c89e609f2a655b7e06e50d3b0f33d148ac1e0d2806971823885abea8fec96`, effective seal `2947ebae`): Eligibility: graphic ASCII; one-material Sequence; local-cell-only Styled exception; every refusal/environment/product guard retained..

**Complement and invariant.** The positive certificate is intentionally tiny: one graphic ASCII Text through exact admitted wrappers. The complement includes actual semantic/locatable/callback/style contexts that cannot safely reuse a measured product. Styled guard failure must remain refusal, not recursive fallback. source_body strips only Source; local_cell_transport_only requires every typed delta absent and only the specific local custom keys. Token is checked in capture and again at consumption. Guard residuals: Styled when local_cell_transport_only is false -> None; Text empty/non-graphic -> None inside its arm; GridCell/TableCell special placement/alignment or resolved alignment -> None inside their arms.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Allowed punctuation/decimal ASCII and Source/local-key Styled produce the same single leaf and once-only consumption. Unicode, spaces/controls, multiple material nodes, locatables/context/Realized/Located/ShowRecipe, changed style/width/origin or forbidden Styled keys refuse and preserve ordinary output/effects.

**Dependencies.** simple_placement, local_cell_transport_only, normal layout_sub_frame miss path. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-033 — Layouter::resolve_grid_columns

Source: `01_core/src/compiler/layout/grid/tracks.rs:87`; complete function reviewed: `01_core/src/compiler/layout/grid/tracks.rs:11-199`. Scrutinee: cell/item outer Content in inset projection; enum/domain: Content. Classification: **negative_projection_missing_nominal_coverage**.

**Evidence.** Non-cell inset None selects grid inset; a prepared TableCell with no surviving default uses zero inset rather than inherited grid inset. References: `01_core/src/compiler/layout/grid/tracks.rs:77-97`, `01_core/src/compiler/layout/grid/cell_body.rs:18-45`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/grid/tracks.md` (SHA-256 `c706646541fe2d7e4aefcfa18a5c45c6c7fe1fe2cdaec31248abed4d10c744b0`, effective seal `888e96e9`): Exclusive current contract: prepared Complete visual at physical coordinates; surviving-default properties; preserve legacy Grid formulas/cache/phase..

**Complement and invariant.** The complement has no outer cell-specific inset. Nested content still participates in measurement. The physical plan controls placement; do not invent metadata from the visual body or reexecute recipes. Guarded resolved TableCell is handled within its own unconditional TableCell arm; negative raw class is not a total prepared-cell fallback.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Raw and GridCell custom inset preserve Auto width/row height; prepared replacement has zero inset while surviving prepared default retains its known inset. Composite visual/nested Table/source-style/explicit columns/fractions/cache controls retain actual width, row-height, wrapping and once-only effects.

**Dependencies.** prepared_default, measurement_visual, constrained measurement, measurement_cache. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-034 — Layouter::resolve_grid_rows

Source: `01_core/src/compiler/layout/grid/tracks.rs:298`; complete function reviewed: `01_core/src/compiler/layout/grid/tracks.rs:203-416`. Scrutinee: cell/item outer Content in inset projection; enum/domain: Content. Classification: **negative_projection_missing_nominal_coverage**.

**Evidence.** Non-cell inset None selects grid inset; a prepared TableCell with no surviving default uses zero inset rather than inherited grid inset. References: `01_core/src/compiler/layout/grid/tracks.rs:288-308`, `01_core/src/compiler/layout/grid/cell_body.rs:18-45`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/grid/tracks.md` (SHA-256 `c706646541fe2d7e4aefcfa18a5c45c6c7fe1fe2cdaec31248abed4d10c744b0`, effective seal `888e96e9`): Exclusive current contract: prepared Complete visual at physical coordinates; surviving-default properties; preserve legacy Grid formulas/cache/phase..

**Complement and invariant.** The complement has no outer cell-specific inset. Nested content still participates in measurement. The physical plan controls placement; do not invent metadata from the visual body or reexecute recipes. Guarded resolved TableCell is handled within its own unconditional TableCell arm; negative raw class is not a total prepared-cell fallback.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Raw and GridCell custom inset preserve Auto width/row height; prepared replacement has zero inset while surviving prepared default retains its known inset. Composite visual/nested Table/source-style/explicit columns/fractions/cache controls retain actual width, row-height, wrapping and once-only effects.

**Dependencies.** prepared_default, measurement_visual, constrained measurement, measurement_cache. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-035 — measurement_visual

Source: `01_core/src/compiler/layout/grid/tracks.rs:469`; complete function reviewed: `01_core/src/compiler/layout/grid/tracks.rs:461-473`. Scrutinee: node in Content::map_content_pre closure; enum/domain: Content. Classification: **traversal_continue_missing_nominal_coverage**.

**Evidence.** None means no replacement now: map_content_pre continues the ordinary recursive walk; it does not drop the node. Table returns itself atomically; staged TableCell projects visual, ShowRecipe projects body. References: `01_core/src/compiler/layout/grid/tracks.rs:461-491`, `01_core/src/entities/content/traversal.rs:678-695`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/grid/tracks.md` (SHA-256 `c706646541fe2d7e4aefcfa18a5c45c6c7fe1fe2cdaec31248abed4d10c744b0`, effective seal `888e96e9`): measurement_visual projects known stage; preserve Styled/Source, raw unstaged fallback and complete nested Table boundary..

**Complement and invariant.** Other variants must keep their existing reconstruction/traversal, especially Styled/Source and unstaged cells. Table is a deliberate recursion barrier so nested cells/coords are not extracted. A nominal no-replacement class preserves traversal, not zero measurement. map_content_pre handles Source before the transform closure; Some replacement terminates traversal at this node; None permits existing recursion. Guard residuals: TableCell with stage None -> no replacement and ordinary traversal.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Staged positioned/standalone cell and ShowRecipe project once; unstaged TableCell is not forcibly realized. Source/Styled around text survives; a nested complete Table is atomic; aliases/composite visuals preserve measured geometry/effects.

**Dependencies.** Content::map_content_pre, TableCell::stage/visual. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-036 — place / fields closure

Source: `01_core/src/compiler/layout/grid_placement.rs:106`; complete function reviewed: `01_core/src/compiler/layout/grid_placement.rs:85-276`. Scrutinee: c: &Content with PlacementPolicy; enum/domain: Content. Classification: **policy_guarded_negative_projection**.

**Evidence.** Only TableCell under TableDocumentOrder or GridCell under GridExplicitFirst supplies coordinates/spans/span; all other policy-content combinations are automatic unit cells. References: `01_core/src/compiler/layout/grid_placement.rs:65-83,98-121,143-275`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/grid_placement.md` (SHA-256 `226145e575b3b8ac4ff66155c148640dbac8491f64a0322511cfcd396ff297d8`, effective seal `3e409803`): Ownership/current contract: one occupied-slot engine, closed policies; Grid raw fallback; Table adapter refines only its policy..

**Complement and invariant.** Wrong-policy cell variants are legitimate raw bodies, not another policy’s placement instructions. Raw Empty holes/text must remain unit-span and detached, while Table cells keep source-span errors. Need two complete policy partitions including guarded residuals, not an unguarded combined cell arm. table is derived once from the closed PlacementPolicy. The two adapters call the same engine; no assumption that every input is a matching cell is valid. Guard residuals: TableCell when !table -> automatic raw; GridCell when table -> automatic raw.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Grid explicit-first ordering and Table document-order occupancy/conflict/first-origin semantics remain distinct. Wrong-policy TableCell/GridCell bodies behave as raw unit cells; checked overflow, padding, coordinates/hints and source spans stay exact.

**Dependencies.** PlacementPolicy, TableCell::placement, Grid and Table adapters. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-037 — measure_content

Source: `01_core/src/compiler/layout/helpers.rs:481`; complete function reviewed: `01_core/src/compiler/layout/helpers.rs:450-483`. Scrutinee: content: &Content; enum/domain: Content. Classification: **documented_approximation_requires_bounded_nominal_policy**.

**Evidence.** Only shapes/sequences and transparent realization/source wrappers are measured; other content returns zero. References: `01_core/src/compiler/layout/helpers.rs:450-483`, `01_core/src/compiler/layout/transform.rs:38-68`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/helpers.md` (SHA-256 `6f6c7ba2f6539cfa273fa1ddf2a2ba76c1c19d29047e5fc0c434d452513cefbc`, effective seal `521a7355`): Ownership: narrow legacy shape/sequence measurement fallback; project Realized/Located/ShowRecipe/Source; preserve other-arm approximations..

**Complement and invariant.** Current L0 explicitly preserves this narrow legacy fallback, and transform::layout_matrix calls it only after actual sublayout has no measurable width/height. Nonempty renderable variants are not intrinsically zero; do not label them universally nonmeasurable. Distinguish true Empty/metadata from unsupported approximations in L0. Caller first lays out the body and uses real item bounds/height when either is positive. This narrows reachability but does not prove every other variant is semantically zero.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Keep documented unsupported zero approximation initially; add no general measurement algorithm in this cleanup. If a reachable fallback loses language extent, open a separate measured correction in this owner rather than copy the real-measure dispatcher.

**Tests/refuters — not run.** Transform with empty/metadata/degenerate shapes preserves current fallback; Realized visual and Source/Located/ShowRecipe around shape preserve dimensions. Probe renderable fallback bodies under arbitrary transforms; nonzero actual geometry with zero fallback refutes sufficiency. Pinned bilateral probe needed and not run.

**Dependencies.** transform::layout_matrix, actual subframe extent, helpers owner approximation boundary. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No probe establishes that every currently renderable complement is unreachable at this fallback. Any extension must keep dimensions/phase/purity and avoid callback/locator replay.

### V16-038 — extract_pt

Source: `01_core/src/compiler/layout/image.rs:133`; complete function reviewed: `01_core/src/compiler/layout/image.rs:129-135`. Scrutinee: val: &Value; enum/domain: Value. Classification: **documented_dimension_projection_missing_nominal_coverage**.

**Evidence.** Float and the absolute part of Length become points; other Values mean no usable axis override, so natural/aspect dimensions are used. References: `01_core/src/compiler/layout/image.rs:51-104,129-135`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/image.md` (SHA-256 `624442193036b74e69b96bbd95c362f4480493e5a327715efa330f1c6dcc0272`, effective seal `4e17da98`): Ownership: preserve current Float/absolute-Length override projection; DPI72/aspect/EXIF/unknown100pt fallback unchanged..

**Complement and invariant.** The image L0 explicitly retains Float/absolute-Length projection. Int/Ratio/Relative are not silently promoted; Length em semantics are not repaired here. Public calculate_dimensions accepts raw Values, so enum-wide invalid inputs are possible even if constructor commonly narrows them. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Float/absolute Length one/two-axis overrides and fit cover/contain/stretch retain target/natural/aspect dimensions. For each refused type, assert unchanged fallback rather than invented zero dimension; pinned probes for Int/Ratio/em behavior are separate and not run.

**Dependencies.** native_image input carrier, calculate_dimensions, ImageSizer. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-039 — flush_line_with_outer_singleton

Source: `01_core/src/compiler/layout/line_commit.rs:720`; complete function reviewed: `01_core/src/compiler/layout/line_commit.rs:356-904`. Scrutinee: numbering: Option<Value> after admission and overlay suppression; enum/domain: Value. Classification: **guarded_optional_dispatch_missing_nominal_coverage**.

**Evidence.** Pattern formats a string; Func resolves/records a callback request; suppressed None produces no number. No Func runs in layout. References: `01_core/src/compiler/layout/line_commit.rs:660-723,782-834`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/line_commit.md` (SHA-256 `d5d818ec929808fbee4f17750bff557599051c9d79210e0b20fb178d1e76166e`, effective seal `1a259f13`): Number only material non-subframe lines after stable baseline/page; absent/none neutrality; Func request consumption without execution; overlay suppression..

**Complement and invariant.** At this match, Some is Str/Func because of the preceding admission. Suppression can replace it with None after the counter is advanced. Other Some variants are impossible under current local assignments, not generic accepted no-number inputs. Some Str/Func admitted at675-681; overlay may set None at693-697. Deferred patterns set None before admission and therefore bypass the block. Guard residuals: Outer None is reachable by overlay suppression; Some(complement) is locally unreachable at this match.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Model the outer None explicitly and the admitted Some payload with nominal decisions; preserve current no-number result for excluded present types without widening admission or fabricating a provisional number.

**Tests/refuters — not run.** Pattern and callback unresolved/resolved modes preserve page/baseline/scope and request count. Overlay suppression advances counting but emits no number; disabled/invalid custom values never enter admitted counting. A future Value variant must require a decision.

**Dependencies.** ParLineCallbackPass, segmented pattern consolidation, overlay and counter scope. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-040 — Layouter::measure_content_constrained

Source: `01_core/src/compiler/layout/measurement.rs:426`; complete function reviewed: `01_core/src/compiler/layout/measurement.rs:138-428`. Scrutinee: content: &Content; enum/domain: Content. Classification: **documented_approximation_requires_bounded_nominal_policy**.

**Evidence.** Unhandled content currently measures zero; some handled sequences invoke a certified isolated inline-flow sublayout. References: `01_core/src/compiler/layout/measurement.rs:138-428`, `01_core/src/compiler/layout/grid/tracks.rs:92-136,309-327`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/measurement.md` (SHA-256 `57be786ad9bb1fade138783fa533cef504308026f70f900554cad383ecd75272`, effective seal `dbc0c9d4`): Constrained dispatch: listed supported families; unsupported current fallback; isolated certified inline Sequence and no emitter mutation/callbacks; nonfractional Box contract..

**Complement and invariant.** Empty/metadata/control content and unsupported renderable Figure/Image/Equation/Table/etc. cannot share an explanatory claim that all have zero dimensions. Current L0 explicitly preserves unsupported fallback. Prepared/nested Table callers separately select physical subframes. Preserve this boundary, classify every complement, and require probes before expanding the supported domain. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Record true-neutral versus unsupported approximation in L0; retain existing zero values initially. A proved reachable width/height loss requires an owner-specific real-layout/projection correction, not mechanically forwarding every node through emission.

**Tests/refuters — not run.** Existing text/Sequence inline, lexical style, Box explicit totals, Block dimensions, spaces and columns tests remain unchanged. Pinned source→Auto-grid probes for Image/Equation/Table/Figure and wrappers distinguish documented fallback from language extent loss; guard callback/locator/effect-once and nested-Table physical routes. Not run.

**Dependencies.** tracks physical fallback, inline_flow certificate, metrics, Block/Box resolution. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** Zero fallback alone is not evidence of a language bug or legitimate zero extent for every complement. Needed bilateral probes are not run; no broad measurement redesign authorized.

### V16-041 — Layouter::layout_content_once

Source: `01_core/src/compiler/layout/mod.rs:1256`; complete function reviewed: `01_core/src/compiler/layout/mod.rs:1219-1263`. Scrutinee: content before locator gate; enum/domain: Content. Classification: **forwarding_partition_missing_nominal_coverage**.

**Evidence.** Four transport variants are routed specially and return; fallback continues to advance_locator_if_locatable then exhaustive layout_content_body. References: `01_core/src/compiler/layout/mod.rs:1214-1279`, `01_core/src/compiler/layout/mod.rs:1150-1171`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/mod.md` (SHA-256 `cd64e452a1a51bed7e4dec6983bb587e0e03face25e49f3ae184f7b766c59f81`, effective seal `a1f48fb6`): Causal dispatcher and refinements: one canonical locatable gate, Source transparency, Located explicit identity, Realized static delegation, ShowRecipe body..

**Complement and invariant.** Complement is not ignored: every normal/native leaf or container must pass through the canonical locator gate exactly once. Source/Located/Realized/ShowRecipe have distinct identity/phase semantics and must not be unified into a generic recursive wrapper. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Raw locatable gets one Location/position; explicit Located retains identity without a new body Location; Realized consumed origin plus visual events remain causal. Source/ShowRecipe transparent emission, reserved explicit IDs, nested wrappers, context/rollback and no duplicate effects.

**Dependencies.** is_locatable, reserve_locations, realized::layout, layout_content_body. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-042 — Layouter::layout_content_body

Source: `01_core/src/compiler/layout/mod.rs:1273`; complete function reviewed: `01_core/src/compiler/layout/mod.rs:1265-1752`. Scrutinee: content in pre-dispatch SmartQuote observation; enum/domain: Content. Classification: **side_observer_negative_partition**.

**Evidence.** Only direct text/separators update quote context in this prepass; all content is then processed by a separate exhaustive dispatch. Equation/Image/Boxed trigger object observation afterward. References: `01_core/src/compiler/layout/mod.rs:1265-1279,1747-1751`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/mod.md` (SHA-256 `cd64e452a1a51bed7e4dec6983bb587e0e03face25e49f3ae184f7b766c59f81`, effective seal `a1f48fb6`): SmartQuoteContext: observe logically emitted text/separators/inline objects causally; descendants share flow; checkpoint restore; sublayout sees object..

**Complement and invariant.** Containers must let emitted descendants update context rather than observe a second synthetic text/object at the wrapper. No-op metadata must not become punctuation. The complement is not discarded layout. New text-bearing/object variants need a deliberate quote-context policy. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Markup/programmatic quotes across Styled/Strong/Emph and text/space/newline preserve one causal context. Equation/Image/Boxed update object context at the existing post-dispatch point; independent sublayout, rollback, empty metadata and nested wrappers do not consume context twice.

**Dependencies.** SmartQuoteContext, smartquote::layout, child static dispatch, checkpoint. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-043 — Layouter::apply_pending_par_line_fixups

Source: `01_core/src/compiler/layout/par_line_callbacks.rs:138`; complete function reviewed: `01_core/src/compiler/layout/par_line_callbacks.rs:130-146`. Scrutinee: (margin, align): (HAlign,HAlign); enum/domain: HAlign. Classification: **finite_product_negative_partition**.

**Evidence.** Only Left margin with Left/Start moves left by max_width-width; Right margin with Right/End moves right. All remaining pairs retain their original placement. References: `01_core/src/compiler/layout/par_line_callbacks.rs:130-146`, `01_core/src/compiler/layout/line_commit.rs:788-835`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/par_line_callbacks.md` (SHA-256 `41428537723f4aca46d3008e91c835d98de391cbbae245b8b1d07e1e176704e4`, effective seal `4efd6ec5`): Final region-local maximum-number-width fixup; line_commit owns physical page/counter/margin selection..

**Complement and invariant.** This is a product enum, not one missing HAlign arm. Current producer stores only physical Left/Right margins; Center/Start/End margin states are unreachable there but retain legacy zero behavior for manual/internal inputs. Center and opposite-edge alignments already have correct per-item placement and need no width-difference correction. line_commit normalizes configured/last-column margins to Left or Right; align can be any present HAlign. Existing item position is already correct for its own width. Guard residuals: Product complement is all25 HAlign pairs except (Left,Left),(Left,Start),(Right,Right),(Right,End); exact list supplied in product_domains below..

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Use a closed product partition (four shifted pairs and the enumerated unchanged pairs). Do not generalize right/left shifts by direction or turn invalid internal state into a new panic.

**Tests/refuters — not run.** Differing number widths cover Left×Left/Start, Right×Right/End and unchanged Center/opposite alignment. Verify producer-normalized margin domain; retain no-shift behavior for manual complement pairs and all item indices/counts/order.

**Dependencies.** line_commit pending-number producer. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-044 — Layouter::translate_deferred_float

Source: `01_core/src/compiler/layout/place.rs:309`; complete function reviewed: `01_core/src/compiler/layout/place.rs:289-521`. Scrutinee: f.alignment.h: Option<HAlign>; enum/domain: HAlign. Classification: **finite_optional_negative_partition**.

**Evidence.** Some Center centers, Some Right/End uses remaining width; None/Some Left/Some Start return zero, then offset is clamped nonnegative. References: `01_core/src/compiler/layout/place.rs:289-310,324-520`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/place.md` (SHA-256 `f0f44d8823bb65a5923b89cf1f8916876d009f7867b6b0122aed2a3b451ad0be`, effective seal `726933c5`): Place adapter float fitting/reservation/final emission; current DeferredFloat excludes dx/dy/scope; reference frame shared with orphan/decor metadata..

**Complement and invariant.** This is complete present horizontal domain behavior, not unhandled Content. Preserve physical Start/End legacy behavior; resolving RTL differently is a separate geometry decision. The fallback must name None and Left/Start rather than accept future alignments. No additional caller restriction is assumed beyond the cited source. Guard residuals: Outer None ->0; Some Left/Start ->0.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** All six Option<HAlign> states, body wider/narrower than region, negative free space, top/bottom float and orphan/decor/link translations. No changed dx/dy/scope support, bidi certificate, clip policy or universal RTL claim from this local repair.

**Dependencies.** DeferredFloat, float admission, current coordinate-origin contract. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-045 — default_supplement_for_key

Source: `01_core/src/compiler/layout/references.rs:369`; complete function reviewed: `01_core/src/compiler/layout/references.rs:349-371`. Scrutinee: key: &CounterKey plus nested Selector/Kind and string prefix; enum/domain: CounterKey. Classification: **guarded_semantic_default_missing_nominal_coverage**.

**Evidence.** Heading/Equation/Table Kind selectors and figure:* string keys receive language supplements; other keys receive no implicit supplement. References: `01_core/src/compiler/layout/references.rs:278-315,349-371`, `01_core/src/entities/counter.rs:22-26`, `01_core/src/entities/selector.rs:27-69`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/references.md` (SHA-256 `e82643f595d841b5937ef7c2b59b84fe86c236a66276b34fe50be1d16f7288f9`, effective seal `fb1a0c78`): References: explicit/empty supplement wins; causal Equation, Figure language delegation, Heading suffix extraction/current-style fallback; no new catalogs..

**Complement and invariant.** A user counter string is not automatically a figure/kind; Page and arbitrary filtered/native selectors cannot be assigned a default label from appearance. Explicit supplements and causal realized Equation/Heading prefixes take precedence in callers. Classify nested Selector and ElementKind complements as well as Page/Str residuals. No additional caller restriction is assumed beyond the cited source. Guard residuals: Str not starting figure: ->None; Selector::Kind other than Heading/Equation/Table ->None; All non-Kind Selector variants ->None; Page ->None.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Retain language-owner delegation and explicit-none distinction. Do not synthesize labels for every element/native selector or add bilingual catalogs.

**Tests/refuters — not run.** Heading causal prefix versus style fallback, equation realized supplement, table/figure language and explicit empty supplement precedence. User key/page/native/filtered/boolean selector keys retain no invented implicit label; NBSP joins exactly once when nonempty.

**Dependencies.** language supplement owner, introspector causal fields, counter-key encoding. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-046 — effective_style

Source: `01_core/src/compiler/layout/text/style.rs:89`; complete function reviewed: `01_core/src/compiler/layout/text/style.rs:13-212`. Scrutinee: text.font family element / dict.name Value; enum/domain: Value. Classification: **schema_negative_projection_missing_nominal_coverage**.

**Evidence.** Invalid Dict.name returns None, skipping that family rather than choosing an installed font. References: `01_core/src/compiler/layout/text/style.rs:71-137,145-212`, `01_core/src/compiler/eval/rules/set/text.rs:305-347`, `01_core/src/entities/font_list.rs:38`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/text/style.md` (SHA-256 `ec1d7a0ac4f8edc41b30bdbce4883767ee7c38df5a9574b8bc12b5bfb62b8c06`, effective seal `0ccd7ea6`): Effective-style ownership: preserve custom-font override/field precedence; no Layouter mutation, I/O or installed-font choice..

**Complement and invariant.** Only Literal(Str) and Regex are valid FontNamePattern representations; missing/wrong schema is not a family name. The L0 preserves effective-style precedence and never gives the layouter responsibility for re-parsing or installed-font selection. The custom chain is Value-based, not a sealed typed container. The ordinary setter normalizes arrays, but invalid direct custom chains exist in style-chain tests; do not assume them impossible.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Literal and Regex dict names, required variants, optional variant/weight/style preserve ordering, overrides and all style fields. Malformed/raw custom-chain Values preserve fallback; integration through validated set/text writer must not lose a supported pattern or alter diagnostics. Refute a claimed unreachable schema with a real writer that admits it.

**Dependencies.** font argument validation owner, FontNamePattern/FontList, StyleChain custom transport. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-047 — effective_style

Source: `01_core/src/compiler/layout/text/style.rs:131`; complete function reviewed: `01_core/src/compiler/layout/text/style.rs:13-212`. Scrutinee: text.font family element / dict.name Value; enum/domain: Value. Classification: **schema_negative_projection_missing_nominal_coverage**.

**Evidence.** Other array elements are skipped; an empty resulting list falls back through FontList::new to the ambient font. References: `01_core/src/compiler/layout/text/style.rs:71-137,145-212`, `01_core/src/compiler/eval/rules/set/text.rs:305-347`, `01_core/src/entities/font_list.rs:38`.

**Current L0.** `00_nucleo/prompts/core/compiler/layout/text/style.md` (SHA-256 `ec1d7a0ac4f8edc41b30bdbce4883767ee7c38df5a9574b8bc12b5bfb62b8c06`, effective seal `0ccd7ea6`): Effective-style ownership: preserve custom-font override/field precedence; no Layouter mutation, I/O or installed-font choice..

**Complement and invariant.** Only string family entries and structured pattern dictionaries encode font families here; Content/numbers/etc. are not silently new family descriptors. The L0 preserves effective-style precedence and never gives the layouter responsibility for re-parsing or installed-font selection. The custom chain is Value-based, not a sealed typed container. The ordinary setter normalizes arrays, but invalid direct custom chains exist in style-chain tests; do not assume them impossible.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Literal and Regex dict names, required variants, optional variant/weight/style preserve ordering, overrides and all style fields. Malformed/raw custom-chain Values preserve fallback; integration through validated set/text writer must not lose a supported pattern or alter diagnostics. Refute a claimed unreachable schema with a real writer that admits it.

**Dependencies.** font argument validation owner, FontNamePattern/FontList, StyleChain custom transport. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-053 — type_as_callable

Source: `01_core/src/compiler/stdlib/collections/validation.rs:100`; complete function reviewed: `01_core/src/compiler/stdlib/collections/validation.rs:89-102`. Scrutinee: t: Type; enum/domain: Type. Classification: **legacy_constructor_subset_needs_nominal_audit**.

**Evidence.** The listed nine Types become native Func; all other Type values return None and caller reports non-callable/type error. References: `01_core/src/compiler/stdlib/collections/validation.rs:89-102,122-137`, `01_core/src/compiler/stdlib/collections/array_predicate.rs:30-40,317`, `01_core/src/compiler/stdlib/collections.rs:1535-1548`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/collections/validation.md` (SHA-256 `09a4b0a18beb785efd9a7f3b15b732da84038b05502a4473fa6f60ebe955e4f1`, effective seal `66e39c33`): Shared validation/casts: type_as_callable preserves a type function; preserve measured identity/errors; parent owns bound/type exposure..

**Complement and invariant.** Current test expectations explicitly reject Bool/Array/Length. The broad Type enum also has later callable-looking classes, so a nine-type whitelist is not proof that the entire complement lacks a constructor. Preserve current subset during nominal repair and separately audit main-call constructor identity before any extension. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Document supported-constructor versus unsupported-in-this-adapter, never claim all complement Types intrinsically non-callable. If pinned map/sorted/filter probes show missing supported constructors, update this owner with measured native identity/ABI and correction scope.

**Tests/refuters — not run.** map(str/int/etc.) and key/by adapters retain native identity, errors and effect counts. Compare each present Type to actual call_dispatch constructor admission and pinned higher-order calls, especially later Decimal/Duration/Version/Stroke types; no invented constructors. Not run.

**Dependencies.** main Type call dispatch, native constructors, array predicate/key/by behavior. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** Whether later constructor Types should be supported by this adapter needs pinned functional evidence; whitelist presence is not intent.

### V16-054 — counter::display::render

Source: `01_core/src/compiler/stdlib/counter/display.rs:132`; complete function reviewed: `01_core/src/compiler/stdlib/counter/display.rs:11-195`. Scrutinee: counter.key: CounterKey with nested Selector; enum/domain: CounterKey. Classification: **nested_key_policy_missing_nominal_coverage**.

**Evidence.** Str uses key-specific numbering.pattern, Kind selector uses lowercased kind pattern, Page uses page pattern; non-Kind selector uses generic counter.numbering.pattern. References: `01_core/src/compiler/stdlib/counter/display.rs:117-154`, `01_core/src/entities/counter.rs:22-26`, `01_core/src/entities/selector.rs:27-69`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/counter/display.md` (SHA-256 `1e673ffc46c895965fc71aabb7ba99ddbab6690b1afb5520f3925269597b7961`, effective seal `664e311c`): Exclusive display contract: same default/explicit numbering ordering; filtered history and snapshot/replay semantics unchanged..

**Complement and invariant.** Outer CounterKey already has only three variants. The wildcard catches payload complements under Selector, not a fourth current CounterKey. Full filtered/Element/boolean keys remain distinct for reading; this default-style lookup must not normalize their counter identity or copy their base kind. No additional caller restriction is assumed beyond the cited source. Guard residuals: Selector::NativeElement -> generic counter.numbering.pattern; Selector::Element -> generic counter.numbering.pattern; Selector::Label -> generic counter.numbering.pattern; Selector::Location -> generic counter.numbering.pattern; Selector::And -> generic counter.numbering.pattern; Selector::Or -> generic counter.numbering.pattern; Selector::Regex -> generic counter.numbering.pattern; Selector::Where -> generic counter.numbering.pattern; Selector::Within -> generic counter.numbering.pattern; Selector::Before -> generic counter.numbering.pattern; Selector::After -> generic counter.numbering.pattern.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Make Selector Kind versus the eleven explicit non-Kind forms visible inside the Selector case and preserve the generic key for each. No new style inheritance or default discovery on replay.

**Tests/refuters — not run.** Implicit Auto versus explicit string/Func pattern; both/current/final and page snapshots retain exact text/effect/observation order. Each non-Kind Selector keeps generic pattern lookup and its complete read key; homonyms/aliases/empty where do not collapse into Kind.

**Dependencies.** filtered_reads, CounterKey/Selector full identity, Engine.styles captured pattern. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-055 — infer_kind_from_body

Source: `01_core/src/compiler/stdlib/figure_image.rs:69`; complete function reviewed: `01_core/src/compiler/stdlib/figure_image.rs:60-71`. Scrutinee: body: &Content; enum/domain: Content. Classification: **explicit_open_semantic_scope_not_lint_authority**.

**Evidence.** Finds first Image/Table/Raw recursively only through ordinary Sequence; wrappers and other content yield no inferred kind. References: `01_core/src/compiler/stdlib/figure_image.rs:60-71,128-145`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/figure_image.md` (SHA-256 `7a0adf6e21580440065bb70fa5f0f2ae9b8276d5c9f7a331026a9d35b9eb67ba`, effective seal `0de4787b`): Figure/caption contract: infer_kind_from_body V16/recursion/phase remains open; typed-kind FG1 and Symbol SY1 separate..

**Complement and invariant.** Current Figure L0 explicitly leaves inference recursion/phase open under FG1. Styled/Source/Located/Realized etc. can hide inferable bodies, but deciding which morphologically transparent carrier should forward, and whether typed explicit kind survives, requires the measured Figure evolution; broad recursion could inspect hidden/consumed/nested unrelated material. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** First partition genuine non-inferable leaves, potential transparent carriers and semantic containers in the Figure owner. Preserve current results while making that bounded negative policy nominal if pursued separately. For a semantic inference correction, obtain pinned source/probes, specify FG1/phase/type-carrier implications in L0 and classify ADR-0127 before implementation; do not add fabricated neutral leaves just to satisfy V16.

**Tests/refuters — not run.** Pinned figure bodies Image/Table/Raw direct and under Sequence/Source/Styled/Strong/Located/Realized/Hide/nested Figure; inspect language kind/getters/counter/supplement, not Rust wrapper equality. Explicit Str/Auto/valid Element Func, alias/homonym, once-only body/caption/kind validation and FG1/SY1 controls; no constructor execution in inference. Not run.

**Dependencies.** FG1 Figure kind carrier/synthesis, Content wrapper semantics, native_figure validation. **ADR-0127:** unclassified_pending_pinned_evidence. Do not change behavior as a V16 cleanup. First measure pinned vanilla acceptance/errors/precedence and source typing, compare explicit current L0/legacy compatibility, then update the owning L0. A proved internal parity correction with unchanged public Rust contract/phase follows continuous flow; an intentional compatibility break/public signature/default/phase change needs ADR-0127 confirmation. Uncertainty must be classified before implementation.

**Unknowns.** Current code/lint does not decide a complete inference contract. Bilateral/source classification and any needed public-kind/phase gate remain pending.

### V16-056 — parse_selector_arg

Source: `01_core/src/compiler/stdlib/foundations/selector.rs:162`; complete function reviewed: `01_core/src/compiler/stdlib/foundations/selector.rs:128-164`. Scrutinee: items: &[Value]; enum/domain: Value. Classification: **slice_arity_negative_partition**.

**Evidence.** Singleton accepted forms/types are handled earlier (including all Func/Str guard residuals); [other] diagnoses every other singleton. Final wildcard handles zero or >=2 items only. References: `01_core/src/compiler/stdlib/foundations/selector.rs:128-164`, `01_core/src/compiler/stdlib/foundations/query.rs:61,93`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/foundations/selector.md` (SHA-256 `2f481b18b28712f4689da800c681609d4b8041fba610171313ee0a4affbc1484`, effective seal `09802fb4`): Own parse_selector_arg used by query/locate; preserve arity/types/diagnostics, full selector and static native identity..

**Complement and invariant.** The warned scrutinee is a slice. It does not lose a Value variant: every singleton is covered by [other]. Replacing it with a Value OR-list would address the wrong domain. [other] exhausts all length-one values; guarded Func is followed by unconditional Func, guarded label Str by unconditional Str. Guard residuals: Actual fallback domain: [] or [first,second,rest..], independent of Value variant. nominal_complement below describes singleton unsupported types, not the final warned arm..

**Proposed correction.** Keep semantic length partition explicit: empty and at-least-two patterns report the same existing arity diagnostic, singleton dispatch remains unchanged. This is causal arity coverage with a testable domain, not syntax evasion; do not rewrite as if-let or move the wildcard elsewhere. Audit any adjacent Regex/string parity debt separately.

**Tests/refuters — not run.** query/locate with zero, one supported, one unsupported, and >=2 args retain exact detached diagnostics/arity count. Func Cell/non-element/alias, full Element/Selector cloning, guarded label strings and ordinary kind strings retain identity/errors. Test Regex discrepancy separately against pinned native_selector/query contract; not run.

**Dependencies.** query/locate adapters, native_selector catalog. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** L0 Regex acceptance versus this legacy parser should be reconciled separately; no new Regex correction is authorized by the arity warning.

### V16-057 — cast_attr

Source: `01_core/src/compiler/stdlib/html/attr_cast.rs:108`; complete function reviewed: `01_core/src/compiler/stdlib/html/attr_cast.rs:9-210`. Scrutinee: value under AttrKind::OnOffOrNoneOrEnum; enum/domain: Value. Classification: **delegated_validator_union_missing_nominal_coverage**.

**Evidence.** Bool→on/off, None→none, Str must be an allowed token; all other types are sent to require_enum and rejected, not accepted by fallback. References: `01_core/src/compiler/stdlib/html/attr_cast.rs:105-109`, `01_core/src/compiler/stdlib/html/attr_values.rs:16-34`, `01_core/src/compiler/stdlib/html/global_attrs.rs:194-214`, `01_core/src/compiler/stdlib/html/attr_specs.rs:25-29`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/html/attr_cast.md` (SHA-256 `229d9e9efe54e796817ec2fb25f942598beee1683bd9fe85d69dc53f8bfd3d9e`, effective seal `282c1f75`): attr_cast contract: exhaustive AttrKind coercion; preserve hub public union types, omissions, diagnostics/order/morphology..

**Complement and invariant.** The delegated fallback is a string-union validator, not a neutral default. Preserve Str allowed/invalid token split and exact wrong-type diagnostics; never route new Value variants to success automatically. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Keep Str delegation explicit and classify every non-union Value as the same existing require_enum error path, retaining full span/formatting. Do not broaden casts or replace diagnostics with generic errors.

**Tests/refuters — not run.** Autocapitalize or hidden: every Bool branch, valid/invalid Str, None where allowed/forbidden, Int and Content wrong types, omitted versus false versus empty output and order. Compare full diagnostic/span to pinned source→HTML attribute cases before any behavior correction; these probes are not run.

**Dependencies.** require_enum, GLOBAL_ATTRS, public HTML union contract. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-058 — cast_attr

Source: `01_core/src/compiler/stdlib/html/attr_cast.rs:124`; complete function reviewed: `01_core/src/compiler/stdlib/html/attr_cast.rs:9-210`. Scrutinee: value under AttrKind::PresenceOr; enum/domain: Value. Classification: **delegated_validator_union_missing_nominal_coverage**.

**Evidence.** Bool true→empty attribute, false→omit, Str must be an allowed token; all other types are sent to require_enum and rejected, not accepted by fallback. References: `01_core/src/compiler/stdlib/html/attr_cast.rs:121-125`, `01_core/src/compiler/stdlib/html/attr_values.rs:16-34`, `01_core/src/compiler/stdlib/html/global_attrs.rs:194-214`, `01_core/src/compiler/stdlib/html/attr_specs.rs:25-29`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/html/attr_cast.md` (SHA-256 `229d9e9efe54e796817ec2fb25f942598beee1683bd9fe85d69dc53f8bfd3d9e`, effective seal `282c1f75`): attr_cast contract: exhaustive AttrKind coercion; preserve hub public union types, omissions, diagnostics/order/morphology..

**Complement and invariant.** The delegated fallback is a string-union validator, not a neutral default. Preserve Str allowed/invalid token split and exact wrong-type diagnostics; never route new Value variants to success automatically. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Keep Str delegation explicit and classify every non-union Value as the same existing require_enum error path, retaining full span/formatting. Do not broaden casts or replace diagnostics with generic errors.

**Tests/refuters — not run.** Autocapitalize or hidden: every Bool branch, valid/invalid Str, None where allowed/forbidden, Int and Content wrong types, omitted versus false versus empty output and order. Compare full diagnostic/span to pinned source→HTML attribute cases before any behavior correction; these probes are not run.

**Dependencies.** require_enum, GLOBAL_ATTRS, public HTML union contract. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-059 — parse_track_sizing

Source: `01_core/src/compiler/stdlib/layout/grid.rs:18`; complete function reviewed: `01_core/src/compiler/stdlib/layout/grid.rs:11-20`. Scrutinee: val: &Value; enum/domain: Value. Classification: **type_projection_with_probe_dependent_caller_loss**.

**Evidence.** Unsupported Values return None; extract_tracks uses filter_map and silently drops invalid array entries, with an empty result defaulting to one Auto track in native_grid. References: `01_core/src/compiler/stdlib/layout/grid.rs:11-34,62-72`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/layout/grid.md` (SHA-256 `2e8260431ab16e9b1ace106cee0e3b546d4ccaabcc3ed6552a9fb78355b56676`, effective seal `4b2e9ad8`): Grid/track contract: auto/length/fraction/integer expansion, validation and current compatibility; hub boundary criteria..

**Complement and invariant.** A parser returning None on a non-track is legitimate. The caller currently conflates invalid supplied tracks with omitted/empty tracks; this can change language geometry/cardinality. Non-auto Str is a guarded residual. Current L0 says preserve behavior, not proof of intended rejection/acceptance. No additional caller restriction is assumed beyond the cited source. Guard residuals: Str other than auto ->None.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Preserve parser rejection initially. Separately probe invalid scalar/array track input and source typing. If rejection is required, make caller validation distinguish absent/empty from invalid without dropping entries, update each owning L0 (Grid/Table callers), keep legacy Float/auto-string extensions unless separately decided, and preserve error precedence/spans.

**Tests/refuters — not run.** Mixed valid/invalid track array must not be silently called equivalent to a shorter array; pinned valid/invalid scalar/empty/integer-count probes needed, not run. Preserve Length/Fraction/Auto/legacy Float and auto string semantics, ordering, grid/table defaults and geometry.

**Dependencies.** extract_tracks, native_grid, structural Table track caller. **ADR-0127:** unclassified_pending_pinned_evidence. Do not change behavior as a V16 cleanup. First measure pinned vanilla acceptance/errors/precedence and source typing, compare explicit current L0/legacy compatibility, then update the owning L0. A proved internal parity correction with unchanged public Rust contract/phase follows continuous flow; an intentional compatibility break/public signature/default/phase change needs ADR-0127 confirmation. Uncertainty must be classified before implementation.

**Unknowns.** Whether invalid-track omission is parity debt or retained legacy compatibility needs pinned evidence and L0 classification. Length relative/em projection is a separate dimensional debt, not resolved by nominal coverage.

### V16-060 — native_place::extract_pt

Source: `01_core/src/compiler/stdlib/layout/place.rs:38`; complete function reviewed: `01_core/src/compiler/stdlib/layout/place.rs:33-40`. Scrutinee: val: &Value; enum/domain: Value. Classification: **possible_parity_debt_unvalidated_zero_coercion**.

**Evidence.** Float/Int/absolute Length project to points; every other Value becomes0. named dx/dy passed directly without type validation. References: `01_core/src/compiler/stdlib/layout/place.rs:33-52`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/layout/place.md` (SHA-256 `f886990ac7f282fce9c035cbf3215da1570167f379a0084a2b60cad38fd07c70`, effective seal `6529b9d5`): place preserves dx/dy and extraction compatibility; no new default/phase.

**Complement and invariant.** Omission already has its own default; invalid supplied dx/dy is currently indistinguishable from explicit zero. This is observed behavior, not an established bug: the current L0 preserves fallback/compatibility. A blind nominal negative group would hide the question; rejecting everything now would also change language behavior without evidence. named dx/dy passed directly without type validation

**Proposed correction.** Retain current output while planning. Measure pinned accepted types/errors and numeric/relative forms, compare source typing and current legacy contract. If a parity defect is established, update this owner L0 before making conversion fail through a private fallible path at the actual argument read, preserving omission defaults and validation order. Do not add region/font resolution in eval or change a public Rust signature merely for lint.

**Tests/refuters — not run.** Pinned probes for omitted, explicit0, negative, Float/Int/pt/em/Ratio/Relative, None/Auto/Bool/Str/Array/Content dx/dy, with conflicting/invalid options; compare complete diagnostics and real geometry. Not run. Preserve existing valid legacy forms and effect-once/argument order; malformed explicit input producing zero is a refuter to any claim that caller validation makes the fallback unreachable.

**Dependencies.** canonical dimensional cast owner, native argument validation, pinned oracle/source typing. **ADR-0127:** unclassified_pending_pinned_evidence. Do not change behavior as a V16 cleanup. First measure pinned vanilla acceptance/errors/precedence and source typing, compare explicit current L0/legacy compatibility, then update the owning L0. A proved internal parity correction with unchanged public Rust contract/phase follows continuous flow; an intentional compatibility break/public signature/default/phase change needs ADR-0127 confirmation. Uncertainty must be classified before implementation.

**Unknowns.** No pinned probe was run; neither universal invalidity nor a new default/error contract is asserted. A dimensional correction may require carrier/phase/public contract beyond this local projection; classify before expanding.

### V16-061 — native_stroke::dash_item

Source: `01_core/src/compiler/stdlib/layout/stroke.rs:151`; complete function reviewed: `01_core/src/compiler/stdlib/layout/stroke.rs:145-153`. Scrutinee: value: &Value; enum/domain: Value. Classification: **guarded_validation_refusal_missing_nominal_coverage**.

**Evidence.** Only Length with abs>=0 and exact string dot become a DashLength; None from any failed item propagates through collect and the outer native emits its existing dash diagnostic. References: `01_core/src/compiler/stdlib/layout/stroke.rs:145-162,178-207`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/layout/stroke.md` (SHA-256 `14a1e13df5ab55d98c6b65041c2da749882ea7dc3fdbe87857744fd750764b14`, effective seal `9c7dc942`): Stroke contract: preserve parsed fields/specified/default/validation/error priority; no extra cast/default semantics..

**Complement and invariant.** Guard-false Length and non-dot Str are actual rejected states, not two nominal successes. Current abs/em/nonfinite behavior must not be silently changed. Refusal is validation, never an omitted dash item or default. No additional caller restriction is assumed beyond the cited source. Guard residuals: Length with !(abs>=0) ->None; Str != dot ->None.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Name guard-false Length and non-dot Str explicitly, then classify remaining variants as unsupported dash items. Preserve caller atomic failure of the entire array.

**Tests/refuters — not run.** Valid mixed Length/dot arrays and dictionary phase preserve order/LineWidth resolution. Negative/NaN Length, non-dot Str, wrong type and empty array reject through existing full diagnostic; infinity/em retain current conditions unless separately measured.

**Dependencies.** dash_array collect<Option<_>>, native_stroke outer error. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-062 — extract_alignment

Source: `01_core/src/compiler/stdlib/layout.rs:124`; complete function reviewed: `01_core/src/compiler/stdlib/layout.rs:118-127`. Scrutinee: each positional Value; enum/domain: Value. Classification: **search_projection_missing_nominal_coverage**.

**Evidence.** Finds first Align/legacy Str and defaults if absent, ignoring body and other non-alignment values. References: `01_core/src/compiler/stdlib/layout.rs:90-127`, `01_core/src/compiler/stdlib/layout/place.rs:43-52`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/layout.md` (SHA-256 `b3e3d9e06e9921db079590ae80a48cf22a7d137d35c797f219359695ca240f64`, effective seal `35fda480`): Façade/alignment contract: Align or legacy Str, default when omitted, existing body error and no named args..

**Complement and invariant.** Content body is deliberately a negative alignment candidate: refusing the whole call on this complement would break omitted alignment. This function is a search projection, not the full public argument validator. Preserve first-match and legacy string conversion. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression.

**Tests/refuters — not run.** Align+body, body-only/default, legacy alignment Str and positional order preserve chosen alignment and body. Non-alignment values remain search-neutral under existing callers; any stricter arity/type policy is separate pinned/L0 work, not this V16 repair.

**Dependencies.** native_align, native_place, Align2D::from_string. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-063 — textual_content_fields

Source: `01_core/src/compiler/stdlib/loading/encoding.rs:110`; complete function reviewed: `01_core/src/compiler/stdlib/loading/encoding.rs:18-131`. Scrutinee: normalized Content when no authoritative snapshot; enum/domain: Content. Classification: **delegated_projection_missing_nominal_coverage**.

**Evidence.** Fallback continues to eval_content_method(fields); it does not erase content or accept a blank result. Success must be Dict, otherwise error. A Some snapshot bypasses reconstruction entirely. References: `01_core/src/compiler/stdlib/loading/encoding.rs:18-131`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/loading/encoding.md` (SHA-256 `d6b5930974dad201217766fe4fd8e61a5bc80b4efa6417f4a2ff8c3d5d5bd6e5`, effective seal `9795dd45`): Exclusive contract: ordered public projection; snapshot Some authoritative even empty; Source/body and Realized visual; no synthesis/World/callback..

**Complement and invariant.** Explicitly implemented legacy field schemas differ from general getter-backed projection. Remaining public elements must continue delegated fields, including Raw/caption presence, without synthesizing defaults. Source/ShowRecipe/Located/Realized are normalized out before this match and cannot be blindly redelegated as public wrappers. Value must be Content or LocatedContent; wrappers normalize first, snapshot Some takes authoritative branch. Guard residuals: Source/ShowRecipe/Located/Realized impossible at this normalized match; No direct-schema arm means delegation, not no-op.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Record direct-schema versus delegated-field classes and normalized-out variants. Keep getter errors and authoritative empty snapshot; do not manufacture an empty map to make a new variant pass.

**Tests/refuters — not run.** Some empty/full snapshot stays exact and ordered; raw absent snapshot and Source/Styled/Sequence/HTML projections preserve func/field order/presence. Actual JSON/YAML/CLI Raw/caption controls distinguish delegation errors from silent field loss; new element without fields must fail rather than serialize func alone by assumption.

**Dependencies.** eval_content_method(fields), getter/field owners, content-snapshot/raw-presence/caption nuclei. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** General Content serialization and upstream parity remain excluded by the current L0. Parent loading retains an unresolved historical encoder/snapshot approval-provenance gap; nominal preservation does not ratify it.

### V16-064 — write_yaml_value

Source: `01_core/src/compiler/stdlib/loading/yaml_encode.rs:54`; complete function reviewed: `01_core/src/compiler/stdlib/loading/yaml_encode.rs:31-83`. Scrutinee: value: &Value; enum/domain: Value. Classification: **two_stage_serialization_partition_missing_nominal_coverage**.

**Evidence.** Outer fallback sends scalars/empty containers/opaque values to an inner scalar-or-repr match; None is null, Bool/Int/Float preserve exact YAML tokens and opaque values use canonical serialization repr. References: `01_core/src/compiler/stdlib/loading/yaml_encode.rs:31-83`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/loading/yaml_encode.md` (SHA-256 `154662ee329df40b78f14c4976921e288ebeeb88017d579c64d16e72a746a49c`, effective seal `724f6aee`): Emit sequences/maps/projected Content/special scalars; preserve quoting/indent/newline/errors; snapshot authoritative; use shared projection..

**Complement and invariant.** Nonempty Array/Dict are guarded; their empty values deliberately enter scalar []/{} branches. Existing Str/Symbol/Content paths cannot reach the inner repr fallback. Flattening must preserve signed zero, nan/inf, newline/quote/indent/error behavior, not apply repr universally. No additional caller restriction is assumed beyond the cited source. Guard residuals: Array empty -> inner []; Dict empty -> inner {}.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Make three semantic classes visible: nonempty containers, direct textual/content routes, scalar/empty-container/opaque-repr routes. Explicitly name guard-false empty Array/Dict and the complete opaque Value complement; preserve one final newline only where currently emitted.

**Tests/refuters — not run.** Nested/empty Array/Dict, Content Some/None snapshots, Bytes repr, Unicode/quoting/block scalars, finite signed-zero/nan/inf outputs preserve exact public strings. Round-trip is additional only; fixed YAML strings/errors and no callback/read behavior refute accidental repr/newline/indent changes.

**Dependencies.** textual_content_fields, canonical_numeric_token, repr_value_for_serialization, YAML emit helpers. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** Broader encoder/snapshot historical approval gap in parent loading is not resolved by this report; no new semantics or approval inferred.

### V16-065 — native_circle::extract_pt

Source: `01_core/src/compiler/stdlib/shapes/ellipse.rs:89`; complete function reviewed: `01_core/src/compiler/stdlib/shapes/ellipse.rs:84-91`. Scrutinee: val: &Value; enum/domain: Value. Classification: **possible_parity_debt_unvalidated_zero_coercion**.

**Evidence.** Float/Int/absolute Length project to points; every other Value becomes0. named radius passed directly, doubled into explicit Float dimensions. References: `01_core/src/compiler/stdlib/shapes/ellipse.rs:84-100`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/shapes/ellipse.md` (SHA-256 `9f4dcf8c5686c963e645db07f90ba56b4e62017c5e0e7af802aef5d212ad49d4`, effective seal `5d69dfe0`): ellipse/circle preserve radius-to-diameter and fallback; optional-stroke correction excludes dimensions/radius.

**Complement and invariant.** Omission already has its own default; invalid supplied radius is currently indistinguishable from explicit zero. This is observed behavior, not an established bug: the current L0 preserves fallback/compatibility. A blind nominal negative group would hide the question; rejecting everything now would also change language behavior without evidence. named radius passed directly, doubled into explicit Float dimensions

**Proposed correction.** Retain current output while planning. Measure pinned accepted types/errors and numeric/relative forms, compare source typing and current legacy contract. If a parity defect is established, update this owner L0 before making conversion fail through a private fallible path at the actual argument read, preserving omission defaults and validation order. Do not add region/font resolution in eval or change a public Rust signature merely for lint.

**Tests/refuters — not run.** Pinned probes for omitted, explicit0, negative, Float/Int/pt/em/Ratio/Relative, None/Auto/Bool/Str/Array/Content radius, with conflicting/invalid options; compare complete diagnostics and real geometry. Not run. Preserve existing valid legacy forms and effect-once/argument order; malformed explicit input producing zero is a refuter to any claim that caller validation makes the fallback unreachable.

**Dependencies.** canonical dimensional cast owner, native argument validation, pinned oracle/source typing. **ADR-0127:** unclassified_pending_pinned_evidence. Do not change behavior as a V16 cleanup. First measure pinned vanilla acceptance/errors/precedence and source typing, compare explicit current L0/legacy compatibility, then update the owning L0. A proved internal parity correction with unchanged public Rust contract/phase follows continuous flow; an intentional compatibility break/public signature/default/phase change needs ADR-0127 confirmation. Uncertainty must be classified before implementation.

**Unknowns.** No pinned probe was run; neither universal invalidity nor a new default/error contract is asserted. A dimensional correction may require carrier/phase/public contract beyond this local projection; classify before expanding.

### V16-066 — native_line::extract_pt

Source: `01_core/src/compiler/stdlib/shapes/line.rs:35`; complete function reviewed: `01_core/src/compiler/stdlib/shapes/line.rs:30-37`. Scrutinee: val: &Value; enum/domain: Value. Classification: **possible_parity_debt_unvalidated_zero_coercion**.

**Evidence.** Float/Int/absolute Length project to points; every other Value becomes0. legacy dx/dy passed directly; polar length branch validates supported types before calling. References: `01_core/src/compiler/stdlib/shapes/line.rs:30-37,106-119,136-140`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/shapes/line.md` (SHA-256 `0d263ed22249964b718f3d0628c8422c0f1d271b4b7c6ce8896ad36615d18abf`, effective seal `d3c27c4e`): line preserves dx/dy extension, polar precedence and causal dimensions; numeric backend only corrects cos/sin.

**Complement and invariant.** Omission already has its own default; invalid supplied legacy dx/dy is currently indistinguishable from explicit zero. This is observed behavior, not an established bug: the current L0 preserves fallback/compatibility. A blind nominal negative group would hide the question; rejecting everything now would also change language behavior without evidence. legacy dx/dy passed directly; polar length branch validates supported types before calling

**Proposed correction.** Retain current output while planning. Measure pinned accepted types/errors and numeric/relative forms, compare source typing and current legacy contract. If a parity defect is established, update this owner L0 before making conversion fail through a private fallible path at the actual argument read, preserving omission defaults and validation order. Do not add region/font resolution in eval or change a public Rust signature merely for lint.

**Tests/refuters — not run.** Pinned probes for omitted, explicit0, negative, Float/Int/pt/em/Ratio/Relative, None/Auto/Bool/Str/Array/Content legacy dx/dy, with conflicting/invalid options; compare complete diagnostics and real geometry. Not run. Preserve existing valid legacy forms and effect-once/argument order; malformed explicit input producing zero is a refuter to any claim that caller validation makes the fallback unreachable.

**Dependencies.** canonical dimensional cast owner, native argument validation, pinned oracle/source typing. **ADR-0127:** unclassified_pending_pinned_evidence. Do not change behavior as a V16 cleanup. First measure pinned vanilla acceptance/errors/precedence and source typing, compare explicit current L0/legacy compatibility, then update the owning L0. A proved internal parity correction with unchanged public Rust contract/phase follows continuous flow; an intentional compatibility break/public signature/default/phase change needs ADR-0127 confirmation. Uncertainty must be classified before implementation.

**Unknowns.** No pinned probe was run; neither universal invalidity nor a new default/error contract is asserted. A dimensional correction may require carrier/phase/public contract beyond this local projection; classify before expanding.

### V16-067 — native_bibliography / locale closure

Source: `01_core/src/compiler/stdlib/structural/bibliography.rs:334`; complete function reviewed: `01_core/src/compiler/stdlib/structural/bibliography.rs:266-356`. Scrutinee: locale named Value; enum/domain: Value. Classification: **legacy_optional_cast_probe_dependent**.

**Evidence.** Str locale is retained; None or any other Value is treated as no locale. Omission also means no locale. References: `01_core/src/compiler/stdlib/structural/bibliography.rs:266-356`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/structural/bibliography.md` (SHA-256 `e79c574582c080a30c54b5f49110eec821753f7af2030a77d4493f8b260f17ab`, effective seal `5e0ac712`): Bibliography construction preserves title/style/locale, source identity and existing casts/errors; CSL/rendering elsewhere..

**Complement and invariant.** Locale is optional but malformed explicit values and absent values currently collapse. L0 says preserve locale/casts/diagnostics; it does not prove arbitrary invalid Values should be discarded. Do not change this into a new error as a mechanical cleanup. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Keep current negative projection if only restoring nominal coverage. Separately probe pinned locale casts and diagnostics; if wrong-type omission is a proved parity defect, update bibliography L0 and native validation in existing order before code, preserving read/parse/style effects.

**Tests/refuters — not run.** Pinned missing/None/valid-language-string/invalid-string/Bool/Int/Content locale with valid/missing bibliography and style conflicts; not run. Preserve Path/Str versus pre-parsed entries, rooted identity, title/style/ieee default and registered-entry/cite behavior.

**Dependencies.** bibliography loader, locale/style resolution, validation precedence. **ADR-0127:** unclassified_pending_pinned_evidence. Do not change behavior as a V16 cleanup. First measure pinned vanilla acceptance/errors/precedence and source typing, compare explicit current L0/legacy compatibility, then update the owning L0. A proved internal parity correction with unchanged public Rust contract/phase follows continuous flow; an intentional compatibility break/public signature/default/phase change needs ADR-0127 confirmation. Uncertainty must be classified before implementation.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

### V16-068 — find_accent

Source: `01_core/src/compiler/stdlib/structural/math_accent_tests.rs:343`; complete function reviewed: `01_core/src/compiler/stdlib/structural/math_accent_tests.rs:334-345`. Scrutinee: content: &Content in test-only target finder; enum/domain: Content. Classification: **test_finder_negative_partition_needs_scope_proof**.

**Evidence.** Finds MathAccent through a narrow set of wrappers and first matching sequence child; None causes the calling eval helper to panic on missing target, not report success. References: `01_core/src/compiler/stdlib/structural/math_accent_tests.rs:334-345`, `01_core/src/compiler/stdlib/structural/math_accent_tests.rs:347-361`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/structural/math_accent_tests.md` (SHA-256 `b84bd592c31fc30103abe7696a469a1e6396436799bb1e091b64d91376978010`, effective seal `8440aed7`): Test-only obligations: published API/Content carrier and exact existing observations; no productive ownership or altered expected values..

**Complement and invariant.** This is a test observation boundary. Unsupported leaves can legitimately lack the target, but Source/realization/new math wrappers may hide it. Do not widen traversal to arbitrary descendants that could find an unrelated target and weaken the oracle. Classify intended fixture wrapper routes first; preserve missing-target failure. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. In this test owner, record the fixture-supported traversal and explicit non-target complement. If real eval results include a newly transparent wrapper, add a specific route only with a discriminating fixture that fails the old finder and still rejects an unrelated nested target. No productive code change follows.

**Tests/refuters — not run.** Actual published bare/qualified/wrapper fixtures must find the intended MathAccent and retain repr/field expectations. Negative fixture with no target must fail the observation; nested unrelated targets/wrappers do not become success merely to reduce V16. Any structural sentinel check is test-only.

**Dependencies.** eval fixture output morphology, test-only L0 owner. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No eval/test was run, so reachability of omitted transparent wrappers in these exact fixtures is not newly demonstrated.

### V16-069 — find_cases

Source: `01_core/src/compiler/stdlib/structural/math_cases_tests.rs:327`; complete function reviewed: `01_core/src/compiler/stdlib/structural/math_cases_tests.rs:318-329`. Scrutinee: content: &Content in test-only target finder; enum/domain: Content. Classification: **test_finder_negative_partition_needs_scope_proof**.

**Evidence.** Finds MathCases through a narrow set of wrappers and first matching sequence child; None causes the calling eval helper to panic on missing target, not report success. References: `01_core/src/compiler/stdlib/structural/math_cases_tests.rs:318-329`, `01_core/src/compiler/stdlib/structural/math_cases_tests.rs:331-346`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/structural/math_cases_tests.md` (SHA-256 `e3564a1aa018323f80094f801bd5695a27d14475d40c76151f828c478ec025a3`, effective seal `d38a1616`): Test-only obligations: published API/Content carrier and exact existing observations; no productive ownership or altered expected values..

**Complement and invariant.** This is a test observation boundary. Unsupported leaves can legitimately lack the target, but Source/realization/new math wrappers may hide it. Do not widen traversal to arbitrary descendants that could find an unrelated target and weaken the oracle. Classify intended fixture wrapper routes first; preserve missing-target failure. No additional caller restriction is assumed beyond the cited source.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. In this test owner, record the fixture-supported traversal and explicit non-target complement. If real eval results include a newly transparent wrapper, add a specific route only with a discriminating fixture that fails the old finder and still rejects an unrelated nested target. No productive code change follows.

**Tests/refuters — not run.** Actual published bare/qualified/wrapper fixtures must find the intended MathCases and retain repr/field expectations. Negative fixture with no target must fail the observation; nested unrelated targets/wrappers do not become success merely to reduce V16. Any structural sentinel check is test-only.

**Dependencies.** eval fixture output morphology, test-only L0 owner. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No eval/test was run, so reachability of omitted transparent wrappers in these exact fixtures is not newly demonstrated.

### V16-070 — native_outline_entry_indented

Source: `01_core/src/compiler/stdlib/structural/outline.rs:434`; complete function reviewed: `01_core/src/compiler/stdlib/structural/outline.rs:390-524`. Scrutinee: engine.styles custom outline.parent.indent Value; enum/domain: Value. Classification: **private_schema_negative_projection**.

**Evidence.** Only Auto/Bool/Length/Func become OutlineIndent; absent/other value yields must-be-within-outline error, not Auto. References: `01_core/src/compiler/stdlib/structural/outline.rs:390-524`, `01_core/src/compiler/eval/rules/outline.rs:106-142`.

**Current L0.** `00_nucleo/prompts/core/compiler/stdlib/structural/outline.md` (SHA-256 `ec4f46cb478a5736eec4f6ab2faa6fb6e028fe693792fb46d09fe89a2af066d3`, effective seal `3ae7b7fd`): indented: actual parent context/key, private injection, Auto/fixed/Func policy, prefix metrics and replay; no public Entry parent..

**Complement and invariant.** The producer serializes all four current OutlineIndent variants into exactly these Values during actual outline realization. Other Values are invalid/no-parent context, not a request for a new default. Parent key is independently required; neither field alone fabricates parenthood. realize_heading_outline builds parent_styles using exhaustive OutlineIndent match, then restores lexical state. Raw custom styles are not a proof of valid outline context.

**Proposed correction.** After the semantic partition below is recorded in this owner L0, replace the open fallback with an exhaustive nominal decision over the present variants, preserving the existing operation for each class. This is justified negative/forwarding classification, not blind wildcard expansion. Keep guards and their false branches visible; future variants must require a new decision. Do not evade the match through if-let, matches!, another helper or a suppression. Preserve context validation/error order and normalize only the four admitted private schema values. Do not convert None/unsupported Values to Auto, run the indent Func early or infer parent from the receiver.

**Tests/refuters — not run.** Direct/query Entry without parent and malformed private indent/key retain exact context errors. Real Auto/Bool true/false/Length/Func parent, multiple outline keys, prefix None, RTL/hanging widths and callback once(level-1) retain measured composition/replay.

**Dependencies.** eval outline realizer, OutlineIndent, context_reads replay, private parent style schema. **ADR-0127:** continuous_flow_internal_nominal_coverage. Update the current owning L0 first. Preserve every existing public signature, accepted/rejected language case, default, phase, effect and diagnostic; then run meaningful functional controls plus structural future-variant compile-negative checks. No approval or implementation is performed by this diagnostic.

**Unknowns.** No new bilateral probe was run. Static source inspection is not a claim of current vanilla parity.

## Coverage and limits

37/37 assigned rows analyzed; no missing IDs. All37 source hashes and37 owning-L0 hashes match the inventory. These are static review counts at the recorded baseline, not functional success measurements. Needed bilateral and structural compile-negative probes remain not run. No implementation is authorized or certified by this diagnostic.
