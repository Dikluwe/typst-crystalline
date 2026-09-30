# Vanilla-style session font retention protocol

This quarantined lab adapter models the pinned vanilla CLI SystemWorld::reset
behavior: invalidate file state while retaining a lazy FontStore and its loaded slots.
Mechanism reference: lab/typst-original/crates/typst-cli/src/world.rs (fonts field,
constructor, reset, book/font) and typst-kit/src/fonts.rs (FontStore/FontSlot).
Pin 9f2b6e8715237cb086899a42873660fe744622e8, ratified tree/binary provenance inherited
from the preserved baseline. This models one mechanism, not the entire upstream
cache system, watch loop, disk persistence or a data-layout/SoA transformation.

Only lab/vanilla-font-retention and this new diagnostic dataset/report may change.
Production APIs, L0s, ADRs, benchmarks and older evidence must remain byte-identical.
World file, library and source state is fresh each cycle. Fonts alone delegate to a
separate donor SystemWorld configured with existing embedded/system discovery and
no project paths. Each session compiles six distinct prewritten edited inputs in
sequence, with unchanged root/font/options. Metric and shaper cache lifetimes remain
those of the current pipeline. No suppression of font demand or geometry changes.

Policies: fresh donor per cycle; retained lazy donor per session; retained donor with
book preload before cycle zero. Include retained donor setup, preload and final drop
in compiled_session_ms. First cycle stays separately reported. This is not a cold
filesystem experiment, nor a claim that independent CLI processes share a store.
The direct untimed control uses production SystemWorld/pipeline without the adapter.

Fixtures: six revisions each of Latin text, dense text, and a placed rectangle.
Latin/dense change rendered revision numbers; rectangle width changes 80..85pt.
Page 160x180pt, margin 8pt; explicit Libertinus Serif 11pt for text. No explicit text
font declaration for the rectangle. This includes the known unnecessary nontext
Place font demand: it is preserved, not repaired. System fonts/fontconfig and sources
are fingerprinted. Existing Tekt-versus-vanilla output defects are not closed here.

Release opt-level 3, no debug assertions or overflow checks; empty Rust flags,
offline locked dependencies, one Cargo job. Cargo artifact profile and runtime guard
must reject debug measurements. Archive the actual executable. Capture exact HEAD,
dirty/untracked inventory, diff stat, UTC, machine/toolchain, inputs, source snapshots,
font files/config and executable SHA-256. No test/build/other benchmark during timing.
Known competitors are monitored every 100ms with own PIDs whitelisted. Five-second
quiet gate after untimed controls. Fingerprints before and after every complete round
must match; discard any changed/contaminated round, retain raw evidence.

Five independent rounds, each fresh processes for all 3 fixture x 3 policy sessions.
Alternate/reverse policy and fixture order by round. No measured selective retry.
OS caches are warmed by controls; do not flush them. Primary timer is compile-session
composition, not CLI end-to-end. Each cycle includes donor/file/source setup, eval,
stabilization/layout/shaping, complete PDF export, instrumentation bookkeeping, and
explicit source/World/PDF-result destruction. Output file writes, PDF/metadata/font
hashing and validation are excluded and recorded as controls_ms. Final retained-store
destruction is included separately. Session wall and launcher wall also include
controls/output/JSON/process overhead and are different observables. Do not merge them.

Book-call counters and first-book timer, font-index capture and existing phase timers
are present identically in all adapter policies: results are instrumented composition.
First-book duration is nested within pipeline and must not be added again. Phase
comparisons use the same cohort; export includes all builder/resource/serialization
work. Do not label it stream cost. Stable requires CV <=20% and every round mean
(for cycle populations) or session value within 10% of pooled mean. Cold cycle zero,
subsequent cycles 1..5 and session totals are classified separately. Retain all values,
outliers and failed stability cohorts. Five session samples still limit precision.

Untimed semantic/render controls: direct vs adapter policies, all revised output texts,
page count/dimensions, font selection, warnings, and raster pixels. Use independent
Poppler output; hashes alone are auxiliary and not full parity evidence. Compare
cycle outputs across policies/rounds. Changing source checks must prove fresh edits
are observed; retained catalog metadata and used-font identities must match. First
session use discovers normally; later calls reuse the same font store. No conclusion
about changing font files/config, eviction, concurrency or error recovery is allowed.
