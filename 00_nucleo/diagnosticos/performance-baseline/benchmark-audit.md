# Audit of timed boundaries

The inspected sources are archived and fingerprinted with the build. This audit
classifies the actual API and Criterion loop, not a fixture's name. All Rust
entrypoints call `support/profile.rs::require_optimized` before measurements.
Observation and stdlib additionally verify the actual release core test artifact
through `run_core_benchmark.py::executable_from_messages`; raw ignored test
invocation is not used as the performance entrypoint.

| Target/group | Classification | Inside the timer | Outside the timer / cache and destruction limits |
|---|---|---|---|
| scanner | Isolated lexer operation | Lexer construction, complete token consumption, black boxes, transient token/lexer destruction | Corpus filesystem reads and input String construction excluded. Source text reused, OS/cache warm. ns/byte is tokenization, not parsing/eval. |
| content/construction | Public constructor operation | Content construction, wrappers/boxes/Arc allocations; Criterion's batch output collection overhead | `iter_batched(SmallInput)` builds Strings/children outside timing and destroys returned outputs after timing. Consumed-input/internal temporary destruction inside API remains timed. |
| content/sequence | Public normalization operation | Normalization, representation allocation and internal destruction | Vector/leaf construction and returned output destruction excluded by batching. No parse/layout. |
| content/representation | Representation operations | Clone + immediate destruction, Rust equality, fresh DefaultHasher and hash traversal; catalog cases loop and often allocate output | Fixture trees and separate equal/different inputs excluded. Arc identity/sharing is whatever fixture constructs. Rust equality/hash are mechanical costs, not language parity. |
| content/traversal | Public traversal with callback composition | Traversal, callback work, allocations, clones and output destruction; catalog Vec collection included | Fixture setup excluded. Identity callback does `to_owned`, transformation does uppercase. These operations are not bare pointer traversal. |
| content/morphology | Public canonicalization | Canonicalization and output allocation/destruction; catalog collection included | Mixed tree/catalog prepared outside timing. |
| content/projections | Public projections | Emptiness/plain-text/get-field calls and owned results/destruction; aggregate loops/collection included | Prepared wide/deep/catalog inputs reused. Plain text and named-field costs include returned values. |
| content/identity | Public identity/debug projections | Element-name lookup aggregate or complete Debug formatting + Vec/String destruction | Catalog classification/coverage gate excluded. Debug is formatting, not element dispatch alone. |
| content/page_structure | Public structural transformation | wrap_page_columns with its own cloning/traversal/allocation plus batch collection | Input clone is the untimed setup; output destruction is excluded by batching. Fast path/rewrite inputs differ, so subtraction is not a phase cost. |
| layout/measurement | Component composition | `measure_content_real`, temporary Layouter, sub-frame layout, temporary destruction | Content/StyleChain/FixedMetrics fixture and finite-result checks excluded. No real fonts, HarfBuzz/rustybuzz or disk. |
| layout/style_scope | Component composition | Complete public `layout`, introspection/layout, document destruction | Styled Content and morphology check excluded. Fixed metrics; does not isolate style helper or production shaping. |
| PDF builder cohorts | Complete export composition | Resource discovery, builder/object construction, streams, font/image work when used, serialization, temporary and output destruction | PagedDocument, image bytes and font vectors prepared outside timing. No eval/layout/source I/O. Empty-page and serialization sweeps still export entire PDFs. Internal exporter caches may remain warm. |
| PDF stream cohorts | Complete export composition | Full PDF export with Pretty serialization: discovery, builder, emission, serialization and destruction | page/text/shape/draw are fixture cohorts. No isolated emitter timer. Source fixtures/clone setup and PDF header/trailer smoke checks excluded. Most text cases deliberately use Type1 fallback, not real font subsetting. |
| pipeline | Warm-process pipeline entrypoint | Eval, introspection/context stabilization, layout where applicable, metric/font lookup, shaping, resource resolution, export, warnings/results/temporary destruction | `SystemWorld::for_eval`, embedded-font setup, `Source::new` parsing, source construction and smoke compile excluded. World/source reused; comemo/font/shaping caches persist. Any I/O requested *inside* measured APIs remains included; fixtures have no imports/downloads. Not CLI end-to-end. |
| observation | Isolated relation calls | observation_relation over prebuilt values/Content/style/math/etc; no construction or destruction of source fixtures | Private ignored test has short fixed Criterion windows. Existing guarded launcher validates optimization/debug profile. No count of relation calls in real documents is measured here. |
| stdlib/registry and inputs | Registry composition | Full make_stdlib_with_features, scopes/namespaces and destruction; sys.inputs projection/copies inside registry construction | Feature vectors and input maps prebuilt. Costs are registry assembly, not a complete document. |
| stdlib/lookup | Isolated lookup operation | Lookup in already constructed Scope; calc.abs lookup uses already resolved calc namespace | Scope and namespace resolution/setup excluded. |
| stdlib/native | Isolated pre-resolved native call | Native function, result/diagnostic creation and destruction | Parser, dispatcher, Args/EvalContext/NoIoWorld setup and lookup excluded. Context persists; these fixtures must not request I/O. |
| CLI comparison | End-to-end fresh process | Process startup, arg parsing, world/font discovery, source I/O/parse, compilation/export, output writes and teardown | Parent setup, input/output hashing and report serialization excluded; process cache is fresh, filesystem cache warm. Traced runs are a separate population. |

Criterion 0.5.1 `Bencher::iter` drops each returned value within its timed loop.
`iter_batched` performs setup before timing and drops returned outputs afterward;
when batching, `outputs.extend(inputs.into_iter().map(routine))` itself is timed.
This distinction is material: adding batched constructor timings to `b.iter`
export/pipeline timings does not produce a consistent full lifecycle total.

Missing isolated production costs: real font metrics, pure shaping, parser,
resource acquisition, stream emitters, serialization and CLI startup are not each
independently measured by the current suite. Existing phase counters/vanilla trace
spans provide broader envelopes with documented gaps. No new production API is
exposed to bridge these gaps.
