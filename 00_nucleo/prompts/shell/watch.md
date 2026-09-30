# L0 — Filesystem watch capability and local preview server
Hash do Código: a91cc852

**Layer:** L3
**Exclusive consumer:** `03_infra/src/watch.rs`

## Evidence and ownership

`03_infra/src/watch.rs:56`, `:63`, `:67`, `:134` and `:173` define immediate fingerprint snapshots, consuming finalization, waiting and an in-memory HTTP preview server. Parsing belongs to `00_nucleo/prompts/shell/cli.md`, dependency inventory to `00_nucleo/prompts/infra/system-world.md`, and cycle composition to `00_nucleo/prompts/wiring/watch.md`. `BundleFile` belongs to `00_nucleo/prompts/infra/pipeline/bundle.md`.

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

Capability and snapshot integration owners are `00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md` and `00_nucleo/prompts/infra/tests/p1295_watch_contract.md`. External CLI tests have their own owner at `00_nucleo/prompts/wiring/tests/cli.md`.

Check loopback binding, explicit/default ports, 200/301/404, asset bytes/MIME, directory output safety, HTML-only served injection, SSE reload and preservation after failure. Use an explicit ephemeral port and independent readiness signals. Timeouts only bound failure; sleeps, repetition and scheduler luck are not readiness proof. No external server or mechanically identical vanilla threading/headers/locking is required.
