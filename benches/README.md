# Benchmark suite

Compile every target without measuring:

```sh
cargo bench --manifest-path benches/Cargo.toml --no-run
```

Check fixtures and runners without collecting performance samples:

```sh
cargo bench --manifest-path benches/Cargo.toml -- --test
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s benches/tests -p 'test_*.py' -v
```

All Rust entry points reject builds with disabled optimization or enabled debug
assertions before registering any measurements. Use `cargo bench` for smoke
checks too; the default `cargo test --bench` profile is rejected.

| Target | Timed operation and limits |
| --- | --- |
| `scanner_bench` | Lexer construction and complete token consumption; corpus I/O excluded. |
| `content_bench` | Public Content operations. Rust equality/hashing measure representation mechanics, not Typst language parity. Catalog family classification is an untimed coverage check. |
| `layout_bench` | Public `measure_content_real` with FixedMetrics, and complete `layout` over style fixtures. Includes temporary layouter/sub-frame work for measurement and introspection/layout for styled documents; does not isolate style helper costs or real font shaping. |
| `pdf_builder_bench` | Complete PDF exports by builder-oriented fixture. Discovery, object construction, resources, streams, serialization and output destruction are included. |
| `stream_bench` | Complete PDF export cohorts organized by page/text/shape/draw fixtures, with Pretty serialization. Resource discovery, builder and serialization remain timed; these are not isolated emitters. |
| `pipeline_bench` | Warm-process public compilation entry points; eval, stabilization, layout where applicable, resources and export included. Source parsing/world/font setup excluded. |
| `observation_bench` | Existing private ObservationEq test; its Criterion closures alone measure relation calls. |
| `stdlib_bench` | Existing private stdlib test; registry construction, lookups and pre-resolved native calls are separate groups. |

Observation and stdlib use Rust launchers plus `run_core_benchmark.py` (Python 3,
standard library only). The runner builds core library tests with
`cargo test --release --lib --no-run --message-format=json`, verifies the
artifact's optimization and debug-assertion settings, checks the exact ignored
test exists, then executes that artifact. No production API is added or source
implementation copied. Build/startup overhead is outside Criterion's sampling.

The runner clears encoded rustflags for its child build so hidden compiler
flags cannot contradict Cargo's reported profile. Release profile overrides
are still checked and rejected if unoptimized or debug assertions are enabled.
Dependencies must be cached: this child build is offline. The private tests
keep their existing short Criterion settings and do not accept forwarded
Criterion tuning arguments. Their `--test` mode verifies compilation, profile
and test presence without running Criterion fixtures or collecting samples.

Supported timing commands:

```sh
cargo bench --manifest-path benches/Cargo.toml --bench observation_bench
cargo bench --manifest-path benches/Cargo.toml --bench stdlib_bench
```

Do not invoke the raw ignored observation test for performance reports: it has
no internal debug guard. The guarded launcher is the supported entry point.

PDF cohort names now start with `pdf_export_cohorts/` to prevent interpreting
whole-export timings as private phase timings. Historical `pdf_stream/` and
`pdf_builder/` results retain their original names in diagnostics. Do not
silently mix their Criterion baselines with the renamed groups or subtract
different fixtures to infer an emitter's cost.

Fixture creation and smoke checks stay outside measured closures. Allocations,
traversals and output destruction belonging to the measured API remain part of
its cost, subject to Criterion's batching policy. A smoke check proves execution,
not PDF conformance, language parity, performance stability or improvement.

Performance decisions require exact commit/dirty-tree provenance (including
untracked sources), timestamp, command/profile, executable identity, machine
and repeated-run dispersion. This review collects no new performance timings.
