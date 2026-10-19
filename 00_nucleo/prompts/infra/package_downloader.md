# L0 — HTTP preview package downloader
Hash do Código: 22057f95

**Layer:** L3
**Exclusive consumer:** `03_infra/src/package_downloader.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:4b9f69ff41790a55a5b2d6a7b9982da858092e4bb2f0c503d6332a14a510e3ec
- 00_nucleo/prompts/_nuclei/packages/downloader-contract.toml sha256:46d1fc7a1a1c99a876e6cfcadf2e6ff1793eb840b944da823ba717d5a1a7adf2
```

## Current evidence

- `03_infra/src/package_downloader.rs:49`, `:54`, `:63`, `:73` — registry/cache/CA configuration.
- `03_infra/src/package_downloader.rs:132` — remote latest-version lookup.
- `03_infra/src/package_downloader.rs:170` — `PackageDownloader::download`.
- `03_infra/src/package_downloader.rs:272` — HTTPS proxy environment selection.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own synchronous network access, archive extraction and cache publication for `HttpPackageDownloader`, implementing the existing L1 PackageDownloader contract. The default registry is `https://packages.typst.org`; `with_url` configures a single alternative base. World package-root search belongs to `00_nucleo/prompts/infra/world/packages.md`, not here.

Download only `preview`; other namespaces return NotFound. Preserve GET `<base>/preview/<name>-<version>.tar.gz` and final `<cache>/preview/<name>/<version>/`. Reuse an existing package directory; otherwise create a randomized `.tmp-<version>-<suffix>` under the same package parent, unpack gzip/tar and rename to the final destination. No explicit interprocess lock, checksum or signature verification is added. A DirectoryNotEmpty rename race accepts the existing destination. Other filesystem errors retain path/cause. Cleanup before an unsuccessful publication remains guarded; the concurrent-race cleanup discrepancy is recorded in `lab/results/construction/diagnostics/sanitization-infra-wiring-history.md` rather than claimed fixed.

Use the existing ureq HTTPS agent, preferring `HTTPS_PROXY` over `https_proxy` and preserving invalid-proxy handling. `with_custom_ca` stores only an optional path. Read PEM immediately before the request, add valid CAs to normal webpki roots and keep hostname verification. Unreadable, empty, invalid PEM, no certificates or invalid DER fails locally without leaking the certificate path/bytes. The pinned custom-CA nucleus owns shared invariants.

`latest_version(name)` GETs `<base>/preview/index.json`, parses the JSON array, filters exact package names and valid three-component versions, and returns the numerical maximum or `None` for no valid matching version. Download/read/JSON errors remain distinguishable from absence. The consumer does not cache the index per World. `typst init` invokes this path for omitted preview versions; import syntax is unchanged.

Preserve PackageDownloadError distinctions and their existing L1 display contract. On archive 404, an index lookup distinguishes a missing package from a missing version when a different latest version is found; an unavailable index on this branch retains NotFound behavior. Other request failures propagate URL/cause; decompression/unpack failures retain the existing I/O classification. Diagnostic formatting belongs to L2/L4, not the downloader.

## Verification boundary

Existing tests check registry paths and custom-CA failures. The loopback TLS probe may report only PermissionDenied at bind as missing environmental capability; other bind errors fail. Once listening, handshake, trust-chain, hostname, timeout and HTTP assertions are mandatory. Do not substitute public network access for the local probe.
