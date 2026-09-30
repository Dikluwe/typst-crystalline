# Place font acquisition: controlled protocol

Scope: diagnostic helpers and evidence only. Do not change production, owners,
ADRs or existing benchmarks. Read repository instructions, current subframe,
placement and world font owners, parity ADRs and provenance rules first.

Hypothesis H1: a nontext placed body triggers the first font catalog acquisition
through the typography-based subframe origin. Refutation: no first book demand,
or a captured first-demand stack identifying a different cause. H2: repeated
accessors are repeated scans. Refutation: first acquisition dominates and later
accessors on the same world are small. This does not establish persistence
between processes. H3: vanilla avoids catalog discovery on these nontext inputs.
Refutation: a `discover fonts` event appears in its corresponding CLI trace.

Fixtures: empty page, standalone/placed rectangle, standalone/placed image,
standalone/placed text. Page 160 x 180pt, margin 8pt; explicit Libertinus Serif;
placed bodies use top+left, dx=3pt, dy=5pt. Same image/font bytes, default system
and embedded fonts, PDF output, one worker. No claim of identical internal
catalog ordering or byte-identical PDF exports.

Build the diagnostic public-API World decorator and current CLI sequentially in
release, offline, one Cargo job. Preserve Cargo JSON artifact profiles and hash
executables. Clear Rust flags and disable debug assertions. Keep stack captures
outside admitted timing runs. Decorator counts/time every book accessor; its
overhead is included. It must not force book access merely to report catalog
size after an otherwise font-free compilation.

Decorator interval: public compile-to-PDF call, including eval, stabilization,
layout, resources and complete export. World/source setup, optional catalog
preload, writing PDF and output destruction are outside this interval and are
reported separately where timed. Repeated calls reuse world/source and may also
benefit from unrelated process caches; their difference is not assigned wholly
to catalog reuse. Preloading moves work outside the interval, so compare the
preload-plus-pipeline sum as well. It is not a production optimization.

CLI interval: subprocess startup through exit, including setup, parsing,
compilation, export, output I/O and destruction. Collect plain runs separately
from instrumented trace runs. Never subtract a diagnostic component time from
a plain CLI median or add timings across different fixtures.

Three rounds; four samples per fixture/variant in each round. Two untallied
warmup processes per fixture/engine. Alternate pair order across rounds and
samples. Each decorator sample starts a new world and performs three consecutive
compilations; compare cold vs explicitly preloaded catalog in separate samples.
OS file caches remain warm; no cache-drop operation. Check source, input, font
and executable fingerprints before/after each round. Discard the entire round
if any changes or another build/test/benchmark is detected. Monitor process
names every 100ms; this cannot detect all shorter processes or desktop load.

Stability: coefficient of variation <=20% across admitted samples, and each
round's median within 10% of the overall median, independently per cohort. Keep
unstable results as exploratory; do not retry until a convenient result appears.
Keep raw stdout/stderr, PDFs, traces, timestamps, commands, monitoring receipts
and distributions. Capture HEAD and exact dirty/untracked state. Record current
source preservation separately from stability/admission.

No new speedup claim follows from source inspection or accessor counts. A
coordinate-origin repair requires geometry controls for text/nontext, nested
placement, floats, alignment, page transitions and auto dimensions before L0
and implementation changes.
