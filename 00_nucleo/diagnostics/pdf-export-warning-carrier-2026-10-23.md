# PDF export warnings no longer depend on stderr

The approved PDF export contract now returns successful output and structured
warnings in `PdfExport<T>` across all 34 public facades. The image-omission and
variable-font fallback paths no longer write to productive stderr. PDF and
bundle pipelines merge the returned warnings into their existing diagnostic
channels.

A nine-case public API probe covered empty, unknown-image, and malformed-image
documents with healthy, full, and closed stderr. Every case completed without a
panic or stderr output; each returned the expected PDF and warning. The full
workspace tests, strict Clippy, rustdoc, formatting, build, Hash366, mature-path,
and forward/reverse lineage gates passed.

At this PDF batch's closure, the full architectural linter reported nine V16
findings. They were subsequently resolved in the
[V16 coverage correction](v16-option-exhaustiveness-2026-10-23.md); the full
linter now passes. This PDF correction is bounded to
the three productive PDF-export stderr macros and does not claim repository-wide
panic freedom.

The complete commands, hashes, working-tree provenance, evidence limits, and
remaining V16 locations are in the
[completion report](../../lab/results/pdf-export-warning-carrier-2026-10-23/README.md).
