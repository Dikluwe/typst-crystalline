# Infrastructure and export risk reproduction

## Scope and provenance

Diagnostic/reproduction only. No fixes, productive tests, L0 edits, ADR edits,
nucleus changes, formatter or hash reseal were performed. Repository instructions,
ADR-0107/0108/0121, the three requested sanitation diagnostics and the relevant
current owners were read. The PDF skill informed decoded-format, structure and
rendered-page verification, not a new output contract.

The final successful measurement ran from `2026-09-27T02:06:22.946553+00:00` to
`2026-09-27T02:06:28.881359+00:00`, on HEAD
`6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, **uncommitted working tree**.
The complete changed-file inventory and outside-task file fingerprints are saved
in `infra-export-risk-reproduction/results/provenance-before.json` and
`provenance-after.json`. Filename/status inventories substitute for diff-stat
line metrics under the explicit instruction not to count lines. The comparison
in `protected-changes.json` is empty: existing files outside this diagnostic scope
had unchanged content during the successful run.
A later handoff check detected concurrent changes to other diagnostics and L0s;
`results/provenance-handoff.json` records their complete paths and fingerprints.
Those files were left untouched, and no new measurements use their later state.
The handoff comparison found no productive source changes. Run the saved
`verify_handoff.py` to regenerate that separately timestamped read-only receipt.

The diagnostic executable was built offline from current path dependencies, using
the saved independent Cargo manifest/lockfile; it does not depend on an older
crystalline CLI executable. Its SHA-256 is
`c0fa9509fde8becb7fecfe0900fc92401760ba35b6495b3d6f793e87eba69782`.
The ICC dependency-substitution probe is
`638571da74af8823f6c1733878d5c842d211c2dd594471d1cf1630e020be983f`.
Compiler versions, dependency source fingerprints, all measurement commands,
timestamps, exit codes, tool paths and hashes are in the result receipts.
Earlier harness compilation errors were corrected only inside this diagnostic
directory; they were not product failures and are not classified as such.

### Verified vanilla

Only `lab/typst-original/target/release/typst` was used. Its measured SHA-256 is
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, matching
the build receipt in `00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md` for
pin `9f2b6e8715237cb086899a42873660fe744622e8`, tree
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4` and its explicit pinned build
recipe. The runner checks this digest before comparisons. `--version` was recorded
as auxiliary evidence only. Neither PATH's Typst nor the root release Typst was
used as a vanilla oracle. Vanilla was needed for BibTeX acceptance and image sizing;
the remaining findings have local carrier/format/filesystem witnesses.

## Classification after measurement

| Item | Classification | Measured boundary |
| --- | --- | --- |
| Semantic tagging in background/foreground | **Confirmed** | Visible Label carrier has no MCID or ParentTree node outside the body, in ordinary tagged PDF and UA1 |
| Raster group clipping | **Confirmed** | Clip-bearing group paints beyond its resolved rectangle; SVG control clips |
| Raster image clipping | **Confirmed** | Image clip_rect does not restrict raster pixels; SVG control clips |
| SVG Line caps | **Confirmed** | Round/Square carriers serialize and render as butt |
| ICC compression-failure envelope | **Inconclusive in production** | Forced dependency error confirms an invalid raw-under-FlateDecode envelope; no natural production error reproduced |
| Escaped quotes in BibTeX | **Confirmed** | Valid quoted LaTeX accent fails the local scan and is accepted by pinned vanilla |
| Signed numbers in BibTeX | **Confirmed local inconsistency; broader syntax-parity allegation refuted** | Minus dispatch consumes no sign; vanilla also rejects bare signed numbers |
| Zero DPI consequences | **Confirmed, format-specific** | PNG/EXIF zero yields inf/NaN dimensions; zero JFIF is rejected and falls back correctly |
| Losing-download temporary cleanup | **Confirmed on this Linux filesystem** | Real download returns the winner while its extracted .tmp remains |

“Confirmed” describes the bounded observed discrepancy, not approval of a repair
or a claim of global nonconformance. Where current L0s explicitly record a limit,
this result reproduces that limit; it does not silently expand the contract.

## 1. Semantic content in page layers

**Owners:** `00_nucleo/prompts/infra/export/stream.md` (MCIDs),
`00_nucleo/prompts/infra/export/stream/page.md` (layer painting),
`00_nucleo/prompts/infra/export/stream/draw.md` (marked envelopes), and
`00_nucleo/prompts/infra/export/builder/metadata.md` (structure/ParentTree).
The producer-side owners are `00_nucleo/prompts/compiler/layout/page_canvas.md`
and `00_nucleo/prompts/compiler/layout/page_commit.md`.

The minimal fixture moves a SemanticKind::Label containing visible Helvetica
text between `Page.items`, `background` and `foreground`. Tags stay enabled;
ordinary versus UA1 is varied independently. All generated PDFs pass `qpdf --check`,
and pypdf extracts the appropriate text from each. Poppler rendering confirms the
text is actually painted, not merely stored in an unused object.

The body control has `/Lbl << /MCID 0 >> BDC`, a `/Lbl` StructElem with `/K 0`,
and ParentTree key 0 pointing to that element on the correct page. Both decorative
layers have visible/extractable text but no `/MCID`, no Lbl StructElem and empty
ParentTree arrays. `pdf-observations.json` records decoded operators, roles,
MCIDs and page-reference relationships; no fixed global IDs are parity criteria.

**Causal path:** `stream.rs:145,208` numbers only `page.items`;
`stream/page.rs:58-60` paints all three layers; `stream/draw.rs:48` opens Label
marked content only when the assignment exists; `builder/metadata.rs:871` also
collects structure from body only. The loss is therefore aligned in both numbering
and structure, rather than a dangling marked reference. `page_canvas.rs:17`
wraps rich layer content in a Group; it does not automatically convert all such
content into an Artifact.

**Limit:** This is not proof that every background/header/footer belongs in the
semantic tree. Explicit artifacts should remain excluded. No assistive-technology
or full PDF/UA conformance claim follows from qpdf or text extraction.

**Next test:** Compile actual background/foreground Content through page_canvas,
including Label/Formula/table/link/UA text and explicit Artifact negative controls;
validate matching MCID/ParentTree/annotation relationships under selection and bleed
with an independent accessibility validator. A finding of an intentional Artifact
wrapper at the producer boundary would refute the semantic claim for that fixture,
but not the direct non-Artifact carrier loss measured here.

## 2. Raster group and image clips

**Owners:** `00_nucleo/prompts/infra/export/render.md`; resolved carriers/sizing
come from `00_nucleo/prompts/entities/layout_types.md` and
`00_nucleo/prompts/compiler/layout-image.md`. SVG positive controls use
`00_nucleo/prompts/infra/export/svg/groups.md` and
`00_nucleo/prompts/infra/export/svg/images.md`.

Fixtures use a 100pt square page at two pixels/pt. A group at (10,10) contains a
40pt red rectangle but declares a 10pt-square Rect clip. The image witness uses
the same painted box, a locally generated valid red PNG and a 10pt clip_rect.
The paired controls remove only the clip carrier.

Both clipped raster witnesses paint bbox `[20,20,100,100)` pixels, including red
at (70,70), outside the intended crop. Unclipped controls paint the same geometry.
The independently rendered exported SVG controls paint `[20,20,40,40)` with white
at (70,70); removing their clip restores the larger box. This validates the
fixture's spatial meaning without comparing complete PNG or SVG bytes.

**Causal path:** `render.rs:211` destructures Image with `..`, omitting clip_rect;
`render.rs:226` passes group clip_mask, but `render_group` at `:591` names it
`_clip_mask` and applies only position/matrix before recursion. No raster mask is
provided. `raster-observations.json` saves decoded pixel geometry; SVG documents
and `*.svg-render.png` retain the reference graph and independent render.

**Limits:** Rect clips with identity transforms are confirmed. Ellipse/path/fill-rule
and nested transforms are not newly validated. The local SVG control is a positive
spatial control, not a substitute for vanilla language parity.

**Next tests:** For groups, nested/transformed Rect and rounded/path clips with
finite local bounds; for images, layout-produced cover clips, EXIF orientations
5–8 and bleed. Require geometry outside the clip to remain unpainted, with inside
content preserved. Those are separate tests, not permission to implement masking.

## 3. SVG Line caps

**Owners:** `00_nucleo/prompts/infra/export/svg.md` (dispatch) and
`00_nucleo/prompts/infra/export/svg/geometry.md` (Line writer).
Raster cap mapping remains in `00_nucleo/prompts/infra/export/render.md`.

One FrameItem::Line from (20,30) to (40,30), thickness 10pt, varies only its cap.
The XML parser finds `stroke-linecap="butt"` for Butt, Round and Square alike.
Independent SVG rendering at two pixels/pt yields `[40,50,80,70)` for all three.
The same carriers sent to raster yield that bbox for Butt, but
`[30,50,90,70)` for Round/Square: their endpoint extent is observable.

**Causal path:** the SVG façade's Line match does not forward the cap to
`svg/geometry.rs:14`; the writer hardcodes butt. This is not a universal SVG
stroke defect: Shape's Stroke cap/join/dash attributes follow a different path.

**Next test:** Trace math.mat augmentation or another producer of capped
FrameItem::Line through compilation, compare endpoint geometry against its PDF/
raster witnesses and the verified vanilla SVG, with rotated/scaled groups and
Shape-stroke negative controls. This would distinguish a carrier transport gap
from a different language construct rendered through Shape.

## 4. ICC failure envelope

**Owners:** `00_nucleo/prompts/infra/export/images/icc.md`, with compression
dependency in `00_nucleo/prompts/infra/export/images.md` and binding in
`00_nucleo/prompts/infra/export/builder/resources.md`.

**Natural successful control:** The actual verbose PDF references `/c0` ICCBased;
pypdf decodes its Flate stream into a profile with `acsp` at offset 36. Its decoded
SHA-256 is `c56e1685d888f5edb92fe07f2750f387f8fe8e91b32ff8fb0b56bfbbb9458353`.

**Conditional fault witness:** `src/bin/icc_fault.rs` includes the unchanged current
ICC module in a diagnostic namespace and substitutes only compress_zlib with Err.
The real helper produces `/Length 480 ... /Filter /FlateDecode` followed by the
480 raw ICC bytes. Length matches stored data and the profile signature is valid,
but independent zlib decoding fails with “unknown compression method”. The filter
does not describe the payload: format validity fails under this supplied error.

**Causal path:** `images/icc.rs:107-114` uses raw data on compression error while
unconditionally declaring FlateDecode. The actual compressor in `images.rs:46-48`
writes to an in-memory Vec and exposes write/finish errors. No controlled natural
Err was reached through that production dependency; exhaustion may panic/abort
instead and was not induced. Disk-full, public-network faults or arbitrary invalid
ICC bytes would not demonstrate this Vec compressor failure.

**Classification:** Inconclusive for production failure reachability; confirmed
invalidity of the conditional fallback. This is not a reproduced ordinary-PDF failure.
The source was not instrumented or altered.

**Next test:** A sanctioned diagnostic dependency/fault boundary that preserves
the actual compressor error interface and embeds its output in a complete PDF for
qpdf/ICC validation; separately establish whether any natural backend failure can
return Err instead of terminating. Until then keep the two conclusions separate.

## 5. BibTeX quotes and signed numbers

**Owners:** `00_nucleo/prompts/infra/bibtex.md` for the pure parser;
`00_nucleo/prompts/compiler/eval/bibliography.md` and
`00_nucleo/prompts/infra/system-world/paths.md` for caller/diagnostic propagation.

The minimal accepted reference is a book with quoted title `G\"{o}del`, author
`Doe, Jane` and year 2024. The local parser rejects it with
`esperado ',' ou '}' após campo em 'k'`. A plain quoted title succeeds; the same
LaTeX accent inside braces succeeds and remains literal in the local BibEntry.
Pinned vanilla accepts both accent witnesses and its extracted bibliography text
contains “Gödel”. This establishes escaped-quote syntax acceptance, not a promise
that the deliberately literal local parser implements general LaTeX expansion.

**Quote cause:** `bibtex.rs:231` stops on any double quote, without considering
the preceding escape. `bibliography.rs:64` and `world/paths.rs:79` call this parser
and retain the failed-to-parse wrapper. No localized error text was changed.

**Signed witnesses:** Bare `volume=-1` and `year=-1` reach minus dispatch but fail
the same separator diagnostic. Bare `volume=+1` produces
`valor de campo inválido em posição 54`. Braced `volume={-1}` succeeds, while
braced `year={-1}` reaches the semantic `year inválido` check. Thus lexical sign
consumption and the separate u32 year restriction are different causal boundaries.

**Number cause:** `parse_value` dispatches minus at `bibtex.rs:200`;
`parse_number` at `:246` advances digits only and returns without consuming minus.
The following separator check fails; the fixture terminates normally, not in an
infinite loop. Plus has no numeric dispatch.

**Refuted broader allegation:** Pinned vanilla also rejects both bare-sign volume
witnesses and bare negative year, with its own BibLaTeX diagnostics. Therefore
“vanilla accepts signed bare numeric values, so accepting them is required parity”
is refuted by these fixtures. Vanilla does accept braced negative year and renders
2 BC; the local u32 restriction is an additional bounded semantic difference, not
evidence that the bare-minus branch works. Current local L0 still specifies u32.

**Next tests:** For quotes, backslash parity, escaped-backslash before closing
quote, nested accents, Unicode and World-level diagnostic spans. For numbers,
establish the intended field grammar with the pinned oracle for volume/pages/date,
separating braces/macros/concatenation from numeric tokens and the year domain;
verify precise failure diagnostics rather than inferring sign support from a match arm.

## 6. Zero DPI and image geometry

**Owners:** `00_nucleo/prompts/infra/image-sizer.md` (metadata) and
`00_nucleo/prompts/compiler/layout-image.md` (dimensions).

Locally generated, decodable 2×2 PNG/JPEG fixtures vary only density metadata.
PNG pHYs zero, PNG EXIF XResolution=0/1, and JPEG zero EXIF before positive JFIF
all return Some(0.0). Natural dimensions become `(inf,inf)`. With width=20pt alone,
they become `(20,NaN)`; cover/contain proportional fit also propagates NaN.
Explicit stretch with both width=20pt and height=10pt stays finite. The path is
therefore not universally broken for every override.

The density-free control yields 2pt square. Positive pHYs=2835 yields DPI 72.009
and size approximately 1.999750031pt square. Positive JFIF=72 yields 2pt square.
Zero JFIF is **not** reproduced as Some(0): it returns None and uses the 72-DPI
fallback. This refutes a uniform “all zero densities behave alike” claim.

**Causal path:** `image_sizer.rs:32,50` preserves zero PNG/EXIF values;
`layout/image.rs:61` uses supplied DPI or 72 only for None, then divides 72 by DPI.
Infinite intrinsic dimensions make their aspect quotient NaN, affecting overrides.
JPEG zero EXIF takes precedence over the valid JFIF fallback.

Pinned vanilla successfully compiles every local image fixture into SVG with finite
natural image dimensions (2pt for zero/absent density). Its current raster source
`lab/typst-original/crates/typst-library/src/visualize/image/raster.rs:367-374`
filters DPI to positive after metadata selection. Comparison used rendered geometry
attributes, not payload or complete SVG byte identity.

**Limit:** This run measured the actual metadata and dimension functions plus
vanilla export. It did not claim a crystalline CLI crash, infinite pagination or
the final downstream handling of nonfinite dimensions.

**Next test:** A bounded crystalline end-to-end compilation with natural/one-axis/
cover/contain/stretch variants, checking finite emitted geometry and exact diagnostic
or failure behavior in PDF/SVG/PNG; include zero/negative rational, missing metadata
and EXIF/JFIF precedence controls. Keep any renderer allocation risk bounded.

## 7. Losing download publication

**Owner:** `00_nucleo/prompts/infra/package_downloader.md`, consuming the existing
`00_nucleo/prompts/_nuclei/packages/downloader-contract.toml`.

The real HttpPackageDownloader requests only
`/preview/risk-probe-1.0.0.tar.gz` from a loopback server. Once that GET arrives,
the downloader has passed its initial destination check and created a temp directory.
Before releasing the valid local archive, the server-side controlled competing
publisher atomically installs a nonempty final directory containing “winner-payload”.
The downloaded archive contains “loser-payload”. No external service is changed.

The downloader returns Ok(final path), leaves the winner unchanged, and leaves a
`.tmp-1.0.0-<suffix>/marker.txt` containing loser-payload. The normal publication
control leaves only the final directory. The invalid-gzip control returns IoError
and leaves no temporary directory, demonstrating guard cleanup on the unsuccessful
path rather than a generic cleanup failure.

**Causal path:** `package_downloader.rs:249-266` accepts DirectoryNotEmpty without
removing temp; subsequent mem::forget disables TempDirGuard's Drop. The comment
claiming cleanup is not an action. Success plus both payload inventories witnesses
the losing branch; the destination was not returned by the initial fast path.

**Limit:** This is a real downloader racing a controlled filesystem publisher,
not a probabilistic two-client load test. It confirms the branch on this Linux
filesystem. Other platforms may classify failed rename differently.

**Next test:** Two real downloader processes with loopback response barriers and
distinct archives, plus destination disappearing/empty/partial cases and another
supported filesystem/OS. Assert successful winner reuse and no losing temp after
return; retain error classifications without using network timing as the oracle.

TemporaryDirectory removed each exclusive cache after its inventory was recorded.
Only diagnostic caches were discarded; no user cache or public package registry
was touched. Persistent archives, source scripts and observation receipts remain.

## Commands, artifacts and verification boundaries

Reproduce from the repository root:

```sh
/home/dikluwe/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/bin/python3 \
  00_nucleo/diagnosticos/infra-export-risk-reproduction/run.py
```

The runner records these families, with exact expanded paths in `commands.json`:

```sh
cargo build --offline --manifest-path <diagnostic>/Cargo.toml --target-dir <repo>/target --quiet
<repo>/target/debug/infra-export-risk-probe <diagnostic>/results
qpdf --check <tag-fixture.pdf>
pdftoppm -r 72 -png -singlefile <tag-fixture.pdf> <prefix>
rsvg-convert -w 200 -h 200 -o <control.png> <fixture.svg>
<repo>/target/debug/infra-export-risk-probe download <exclusive-cache> http://127.0.0.1:<port>
<repo>/target/debug/icc_fault <diagnostic>/results/icc-forced-error.stream
<verified-vanilla> compile <bib-or-density-fixture.typ> <vanilla-output>
```

Fixtures, scripts, independent Cargo files and receipts are all under
`00_nucleo/diagnosticos/infra-export-risk-reproduction/`. Relevant outputs include
`public-api-observations.json`, `pdf-observations.json`, `raster-observations.json`,
`svg-observations.json`, `icc-success-control.json`, `icc-observations.json`,
`download-observations.json`, `vanilla-observations.json`, valid image/BibTeX/package
fixtures, generated PDFs/SVGs/PNGs and `observation-preview.png`. The preview was
visually inspected: layer text paints; raster clips overpaint; SVG clips crop; SVG
Square retains the shorter butt geometry. Structure claims were checked separately.

Pillow emits a getdata deprecation warning; it does not invalidate pixel observations.
System Python lacks pypdf; the successful run uses the recorded bundled runtime.
The initial offline resolution issue and diagnostic harness type/signature errors
were resolved without productive edits or public network access. No behavioral
test was added to a productive layer. No line-count or generic full-byte parity
criterion was used. Source/fixture/binary hashes establish provenance only.

This report closes the requested **diagnostic reproduction pass**, not the
discrepancies, their L0 decisions, complete conformance, or global sanitization.
