# Independent measurement candidate review

Read-only review of helpers/image/measurement/text/style productive changes and current L0. Exact HEAD, UTC, changed-file diffstat and reviewed source/L0/dependency hashes are in `measurement-peer-review.json`. No Cargo, source edit or oracle run was performed. No actionable regression found in the inspected candidate. This is a static audit, not a replacement for the coordinator's semantic RED→GREEN, conservation controls or current-pin geometry replay.

## Certificate, transport and callbacks

`measurement.rs:54` uses a direct exhaustive root match. Source/Located inspect their body, Realized inspects visual without consuming its original, sequences inspect every child, fractions inspect numerator/denominator and roots inspect optional index plus radicand. Styles inspect body. MathCancel, ShowRecipe, Ref, Metadata (including opaque Content payload), delayed/context/state/counter/dynamic families, nested Equation and unaudited math all reject. There is no generic visitor silently bypassing a transport root. The new tests include callback-bearing descendants under Realized/Located, metadata, reference supplement and ShowRecipe, and permit an already-consumed unsafe original only when visual is safe.

`measure_inline_flow` constructs a separate Layouter with empty introspector and copies only the active chain/TextStyle while borrowing injected metrics. The emitter's locator, runtime and request passes are not copied. Equation emission builds its ordinary math style and uses its own fallback callback transcript; MathCancel, the callback-producing math arm, cannot pass the certificate. Styled mathematical bodies may use the existing external-layout bridge; that bridge also creates a fresh Layouter and invokes its subframe. This preserves the existing rendering route instead of adding a separate math formula.

The suspected ambient `par.line.numbering` leak is refuted by the current call chain: `measurement.rs:228` calls isolated `layout_sub_frame`; `sub_frame.rs:204` sets `is_sub_frame = true` before dispatch, and `line_commit.rs:663` gates all line-number counting, callback requests and realization on `had_items && !self.is_sub_frame`. The final subframe drain also does not execute the ordinary counting block. Nested external-style layout is a subframe too. No body style can introduce an admitted MathCancel because the certificate rejects the Content family before emission. A direct ambient Func regression control would improve durability, but the absence of that additional control is not evidence of a present leak.

## Geometry and scope

Image uses the injected image sizer and existing dimension calculator, preserving intrinsic/DPI/orientation and fit behavior. Returning target width/height when both exist matches `image.rs` occupied-region selection (`used_height`, target_w/target_h and `record_flow_width_at`); cover ink can exceed that target without increasing occupied width. Single-axis/intrinsic cases use calculated dimensions. The implementation deliberately leaves explicit fixed dimensions unclamped by the grid budget as specified by the new L0.

Captionless Figure delegates body measurement without running figure progress/prefix/page registration. `figure.rs` draws that same body and only appends caption/prefix when a caption exists, so this bounded projection does not drop caption geometry in an admitted case. Captioned figures remain explicit debt, with no invented body-plus-caption formula.

Inline Equation admits only finite positive width and the closed certificate. It uses the existing equation style/math machinery and isolated subframe, preserving the current width cap. Block and contextual equations remain unsupported approximations; the L0 does not call those semantic zero. The certificate's narrow list is explicit and its source traversal matches the written list. This review does not certify every mathematical family or general wrapper geometry against vanilla.

## Other owners

helpers preserves its last-resort Shape/transport behavior while separating Empty/Metadata absence from unsupported visual approximation. image::extract_pt and both effective_style projections retain their current positive branches and explicitly name the negative Value categories. No accepted cast, font-family schema, style precedence, public signature, default or pipeline phase changes were found in those nominal edits.

Final runtime, structural mutation and bilateral results remain coordinator-owned.
