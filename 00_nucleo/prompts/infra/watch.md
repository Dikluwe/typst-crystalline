# L0 — Filesystem watch capability and local preview server
Hash do Código: 10e1aa2a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/watch.rs`

## Evidence and ownership

`03_infra/src/watch.rs:56`, `:63`, `:67`, `:134` and `:173` define immediate fingerprint snapshots, consuming finalization, waiting and an in-memory HTTP preview server. Parsing belongs to `00_nucleo/prompts/shell/cli.md`, dependency inventory to `00_nucleo/prompts/infra/world.md`, and cycle composition to `00_nucleo/prompts/wiring/watch.md`. `BundleFile` belongs to `00_nucleo/prompts/infra/pipeline/bundle.md`.

Filesystem transitions and published preview output are observables. The causal constraint is capture before finalization: otherwise recovery may become the baseline and not trigger recompilation. Inference: a capability containing an already captured snapshot prevents this inversion. Lazy capture, an externally callable raw finalizer or baseline recapture would refute that property. Scheduling delays or observation of a later deletion alone do not prove ordering.

## Snapshot and consuming finalization APIs

```rust
pub struct WatchSnapshot { /* private baseline fields */ }
pub struct ArmedWatch { /* only an already captured WatchSnapshot */ }
pub fn snapshot(paths: &[PathBuf]) -> WatchSnapshot;
pub fn arm(paths: &[PathBuf]) -> ArmedWatch;
impl ArmedWatch {
    pub fn publish(self, staging: &Path, destination: &Path) -> io::Result<WatchSnapshot>;
    pub fn abandon(self, staging: &Path) -> WatchSnapshot;
    pub fn publish_directory(self, staging: &Path, destination: &Path) -> io::Result<WatchSnapshot>;
    pub fn abandon_directory(self, staging: &Path) -> WatchSnapshot;
}
pub fn wait_for_change_since(snapshot: WatchSnapshot, interval: Duration);
pub fn wait_for_change(paths: &[PathBuf], interval: Duration);
```

Capture each supplied path immediately, before returning. Fingerprints observe modified time where available, length and content hash. Missing/unreadable paths have an absent fingerprint; later creation remains observable. Creation, removal, changes and different content with unchanged length must be detectable.

`ArmedWatch` stores only the complete captured snapshot: no raw paths, closures or representation allowing lazy capture at finalization. Baseline fields remain opaque outside L3, with no external mutation/getters.

File publication consumes the capability and renames the completed staging file to the destination in a single atomic rename. Success returns the same captured baseline by move. Rename failure attempts staging cleanup but returns the original rename error, never a cleanup substitute.

Abandonment consumes the capability, attempts staging cleanup, leaves the valid destination untouched and returns the same baseline. Raw rename/remove finalizers remain private, so L4 cannot bypass capture.

Directory publication backs up an existing destination to an exclusive sibling, renames completed staging into place, and cleans the backup after success. On replacement failure, attempt to restore the previous destination and clean staging, returning the original replacement error. Restoration/cleanup are best-effort, not an unconditional successful-rollback or single-rename atomicity guarantee for the directory sequence. Directory abandonment removes only staging and returns the captured baseline.

Only specific validated staging/backup/destination paths are in scope; never recursively target a generic filesystem root.

`wait_for_change_since` polls after sleeping for the supplied interval, comparing current fingerprints to the passed baseline without recapturing or replacing it. Public `snapshot` and the compatibility `wait_for_change` remain available; the latter is immediate capture followed by waiting, not a productive finalization bypass.

## Local server API and defaults

```rust
pub struct BundleHttpServer { /* private state */ }
impl BundleHttpServer {
    pub fn new(title: &str, port: Option<u16>, live: bool) -> io::Result<Self>;
    pub fn addr(&self) -> SocketAddr;
    pub fn publish(&self, files: &[BundleFile]);
    pub fn publish_html(&self, html: &str);
}
```

Bind only `127.0.0.1`. An explicit port is tried alone, including ephemeral port 0. Without one, try 3000 through 3005 in order; exhaustion reports `no available local port in 3000..=3005`. The current title parameter does not add content to served documents.

Composition creates this server only for watch HTML/bundle; `--no-serve` prevents binding and `--no-reload` keeps HTTP without injecting the script. These flags do not enable compiler features. Successful output publication permits replacing the HTTP snapshot; a failed compilation preserves the last successful snapshot.

Serve a caller-validated in-memory snapshot, never arbitrary filesystem paths. `publish_html` maps the document to index.html; `publish` replaces the bundle route map. Readers see a published snapshot; successful publications notify active reload subscribers.

## Routes and observable output

Ignore query strings, percent-decode UTF-8 paths and normalize dot segments without escaping the root. Reject malformed decoding, backslash, NUL and above-root traversal. Root/trailing directory routes try index.html; a directory missing its final slash redirects with status 301 and a slash-terminated Location. Found files return 200; invalid or missing paths return 404 with `Not Found`.

Preserve common extension MIME values: html/htm `text/html; charset=utf-8`, css `text/css`, js `text/javascript`, svg `image/svg+xml`, pdf `application/pdf`, png `image/png`, jpg/jpeg `image/jpeg`, json `application/json`, txt `text/plain; charset=utf-8`; unknown types use `application/octet-stream`. Assets are byte-exact unless designated HTML documents receive live injection.

`/__events` is SSE, initially sending `: connected\n\n`; publish notification is `event: reload\ndata: now\n\n`. The endpoint remains available when live injection is disabled. With live mode, inject this literal script before the first case-sensitive `</body>`, or at the end when absent:

```html
<script>new EventSource("/__events").addEventListener("reload",()=>location.reload())</script>
```

Inject only into served HTML documents; never modify written disk output.

## Acceptance

Assert capture-before-removal/publication with an observed staging path, unchanged returned baseline, absent-path creation, unchanged-size content changes, original rename errors and best-effort cleanup. Waiting must not recapture.

Capability and snapshot integration owners are `00_nucleo/prompts/infra/tests/armed_watch_contract.md` and `00_nucleo/prompts/infra/tests/watch_snapshot_contract.md`. External CLI tests have their own owner at `00_nucleo/prompts/wiring/tests/cli.md`.

Check loopback binding, explicit/default ports, 200/301/404, asset bytes/MIME, directory output safety, HTML-only served injection, SSE reload and preservation after failure. Use an explicit ephemeral port and independent readiness signals. Timeouts only bound failure; sleeps, repetition and scheduler luck are not readiness proof. No external server or mechanically identical vanilla threading/headers/locking is required.


## Redundant preview state

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`
retains sources/L0/hashes/HEAD/tree/UTC/diffstat for watch.rs:170; consumer/use
inspection precedes selected Clippy RED. Internal Rust representation, no new intent/
parity. Remove only private redundant BundleHttpServer.live field/initialization.
Keep new(...,live), worker capture and serve_connection argument unchanged; bool
has no Drop/resource effects. Acquisition/HTTP/SSE/injection/API/defaults unchanged.
Continuous ADR-0127: L0 first/scoped seals/no suppression/exact inventory reduction/
no new warnings/selected RED→GREEN/unfulfilled expectations as error/workspace
suite/build/fmt/Rustdoc/lineage under12GiB/zero swap/one job. No derive/Entry/
module-placement mirror tests; real bbox controls elsewhere remain observations.

## Percent-decoder decision

### Evidence

tekt-ranges-2026-10-16/before.json.gz retains UTC/sources/L0/hashes/manifests/lock/
toolchain/config/status/diffstat at HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
dirty tree. Hash-reconfirmed prior six V18: four call_spans.rs:52,161,206,226 and
two watch.rs:287,288. Read current owner/branches first: Rust decision structure,
messages/spans/routes must stay, no global-parity claim.
The isolated probe extracts actual decode_url_path, changes only digit closure;
probe-details.json records argv/rustc1.92/SHA/UTC/results, probe-result.json records
12GiB/swap0/job1/events. All256bytes/65, 536pairs compare equal:22ASCII hex bytes/
484accepted pairs. Complete functions also match pair strings/13 UTF8-query-dot-
truncated-invalid-NUL-backslash controls. Probe counts are not HTTP/RSS benchmarks
or moving-main data.

### Exact contract and acceptance

Only replace digit table with char::from(value).to_digit(16).map(|digit|digit as u8).
Keep hi/lo/to_ascii_lowercase/get(index+1/2)?/hi*16+lo/advance/from_utf8/query split/
normalization/rejections. No wider alphabet/API/double decoding/filesystem/MIME/
status/headers/injection/SSE/loopback/watch/publication/layer move. Pure std conversion
preserves measured digits. Internal continuous ADR-0127 no public field/signature/
trait/default/dependency/phase/compatibility. Functional preservation tests pass
baseline; real RED is --checks v18 --fail-on warning, GREEN zero V18/exactly six
Tekt warnings removed/no new rules.

Retain joint-batch T1 float zero/one/two/many/bound/spread/named/missing-position
anchors (named first, unbound empty call even spread, bound empty/spread args.span),
T2 Attach/Mono/Script arity/type/unknown-name/cramped priority with/without spread,
unchanged Binom/HTML; T3 all valid percent octets/case/query/dots/root/trailing
slash and NUL/backslash/invalid-UTF8 rejection; T4 real P1293/HTTP-CLI spans/messages/
200-301-404/output; T5 build/strict workspace-all-target Clippy/unfulfilled errors/
fmt/workspace suite/strict Rustdoc/four indexes/diff/strict-reverse lineage under
12GiB/zero swap/job1. Freeze source/L0 during commands, two-pair-only seals, no
renames/expect-suppression/linter downgrade/expected relaxation/unmeasured performance
or AI-readability claim/release.

## Ratified L3 publication carrier

tekt-v12-2026-10-16/before.json.gz retains HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
dirty tree/UTC/sources/L0/hashes/status/diffstat:19V12, one productive Published at
main.rs:22 and18test helpers. Html(String)/Bundle(Vec<BundleFile>) produced by
compile.rs:90,99,363,539,541 and consumed by watch.rs:103,104. Former L4-local
permission is explicitly superseded; Rust ownership, no new parity/intent.
Owner approval `Aprovar a proposta` at2026-10-16T22:00:02.716079+00:00 in
`lab/results/construction/diagnostics/tekt-v12-2026-10-16/approval-receipt.json`,
manifesto `7862b7aeb7272a8b75822f9180b11a7681689eac20f823bebf3eb980526c36e9`,
covers Published in L3/four L0/native verification scope/no release. This satisfies
the former ADR-0127 §2.1/§2.4 before-code gate only for this additive public type.
Future removal is compatibility break.

Own here, without new module/reexport:

```rust
pub enum Published {
    Html(String),
    Bundle(Vec<BundleFile>),
}
```

BundleFile remains owned by `00_nucleo/prompts/infra/pipeline/bundle.md`. No new
derives/methods/fields/variants/constructor I/O/payload copy/flags/defaults. Preserve
move/drop of String/Vec and all watch/server APIs.

Real watch HTML/bundle HTTP-reload/success-error/no-serve-no-reload/last-snapshot
and compile full-tree/byte-exact-assets/stdout-warnings-exit/no-publication-on-error
controls specified in `00_nucleo/prompts/wiring/main.md` remain required, including
named watch tests and L3 HTTP controls. Real V12 RED→zero productive, transfer+
import-only diff/no extension inference, no syntax-mirror tests. Build/strict
workspace-all-target Clippy -Dwarnings/-Dunfulfilled_lint_expectations/workspace
suite/fmt/strict Rustdoc/four indexes/V1-V5-V15-V26/reverse/SARIF/diff --check;
heavy sequential12GiB/swap0/job1, no release.
