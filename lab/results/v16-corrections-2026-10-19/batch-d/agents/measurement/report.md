# Batch D measurement owners — phase 1

The five selected matches are V16-037 helpers::measure_content, V16-038
image::extract_pt, V16-040 measure_content_constrained, V16-046 dictionary font
name and V16-047 font entry. Exact finite domains/anchors are in anchors.json;
phase1.json records test names, state and source hashes. No implementation,
Cargo invocation or sealing has been performed by this agent at phase 1.

## Evidence and provenance

probes/png-before.json preserves all probe sources, UTC, argv, source/output
hashes, HEAD and exact git diff HEAD --stat at both ends. The product debug
binary is the batch C build, not a compilation of concurrent D changes; build
source provenance is batch-c/validation/validation-state.json.gz, build.log and
batch-c/agents/effects/accepted-replay/receipt.json, as verified by coordinator
against committed HEAD a9f030738. Binary SHA is independently measured in the
probe receipt. Oracle commit bd5ebec285505889e8ff1d4f68804ab147a699f3, tree
cac836c07e4dec2a89219370428292393a3216b5, SHA
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9 were verified;
recipe is the pinned vanilla-resync-2026-10-17 report and its evidence.

probes/geometry-before.json extracts red next-column marker x. In Auto columns:
Shape and nested Table are 40pt in both; Empty/Metadata are zero in both;
Image, Align(Image), Styled(Image), and captionless Figure are zero in product
versus 40pt in oracle; Equation is zero versus 23.124444444pt. Fixed40 controls
keep the next column at40pt. Hide(Shape) reserves40pt in both. These are causal
occupied-width probes, not SVG byte comparisons. Table's body y differs and is
not certified by the matching next-column x.

Initial SVG-input image attempts are preserved in before.json and failed only
in product (unsupported SVG input); PNG fixture replays are successful in both.
The axis-before.json run used relative argv and hit product sandbox-root errors;
it does not certify axis semantics. Corrected absolute argv and complete sources
are in axis-absolute-before.json: em compiles but product emits zero image size;
ratio compiles but product retains intrinsic40pt; boolean and Float axes compile
in product but are rejected in oracle. These are existing unresolved language
debts, not acceptable zero projection semantics. Auto preserves intrinsic size.

## Causal scope after measurement

The legacy helper is a last resort after transform.rs performs real subframe
layout and checks positive extent. Preserve its bounded projection and make the
nominal complement explicit. Shape/metadata/empty source transforms were probed;
no general transform-parity claim follows. A painted body reaching zero fallback
would refute the narrow caller justification.

Constrained Image must use existing calculate_dimensions and injected sizer,
returning target bounds for cover/contain. Equation may reuse the existing
isolated measure_inline_flow subframe path only after a positive certificate for
callback-free math. This path preserves chain/style/metrics and does not attach
emitter runtime, locator or callback stores. Reject effects hidden below math
composites, including MathCancel function angles, ContextBlock, metadata Content,
refs/supplements and every unapproved family. Figure without caption delegates
body measurement; numbered/captioned Figure remains unresolved, because replaying
its layout would use progress-dependent prefix and spacing. Do not invent a
parallel caption formula. Scoped tests must fail for real width loss and stay
stable for rejected effects, raw schema controls and existing wrappers.

The new certificate requires its own Content sentinel E0004 check. Existing
inline_flow is not broadened. Legacy zero fallback's visual complements remain
unsupported approximations, not valid semantic zero; further subtype discovery
must lead to evidence and owning correction. Schema matches in image/text style
are internal projections; font writers normalize Str/Regex names and Str/Dict
entries but direct invalid custom chains remain reachable.

## Explicit open debts and refuters

- Captioned/numbered Figure and contextual/callback-dependent Equation measurement:
  current bounded measurement cannot certify them. A callback-free transcript
  replay with correct ambient identity and unchanged requests could enable a
  future owner correction; do not declare semantic zero.
- Image em/relative axes and invalid source axis diagnostics: admitted by the
  stdlib writer; extract_pt has no font/region context. A writer-side resolution
  or API-preserving layout resolution with bilateral tests is required, beyond
  the selected schema classification. No new public signature is authorized.
- SVG image source support and nested Table y origin are separately observed
  and remain unresolved by this batch.
- The final source/effect-once replay, certificate review, baseline and GREEN,
  structural checks, seals and full validation belong to the coordinator.

Phase-1 final inventory has seven new test functions (not eight). The direct
recursive certificate supersedes the earlier visitor proposal; source/located
body and Realized.visual are inspected explicitly. Block equations and other
uncertified math composites remain unresolved approximations.


## Implementation ready

The coordinator observed three expected semantic RED failures after correcting
a test-only private-method call; that E0624 attempt was not counted as RED.
Final baseline receipt is batch-d/baseline-final/new-controls.log and its
classification JSON. The seven tests are preserved. Five selected matches now
name all current variants without suppression, and the additional direct
recursive certificate has a separate sentinel anchor in anchors.json.

Image measurement uses injected self.sizer and occupied target bounds. Only
inline Equation with finite positive budget and the exact simple-math certificate
reuses isolated measure_inline_flow; captionless Figure delegates body. No
public signature, default, phase, callback wiring or mutable global was changed.
Unsupported image source axes, caption/block/context math and other mathematical
families remain explicit debts, not claims of zero language extent. Parent owns
all candidate validation, post-build replay, final receipts and sealing.
