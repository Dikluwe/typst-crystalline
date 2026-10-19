# Independent effect/display cross-review

State and inspected file hashes are recorded in `effects-cross-review-state.json`.
This is a source review, not a test run. Oracle measurements are the effects
agent's verified current-pin receipts, particularly `warning-additional-fields.json`
and `warning-additional-fields-controls.json`.

## Actionable finding: constructor marker does not prove supplement origin

The lab-only field repair correctly limits Styled inspection to the existing
constructor-heading view, but this is insufficient after style merging. In
`compiler/stdlib/structural/heading.rs:184–215`, a numbering-only constructor also
gets the `heading.constructor` marker. In
`compiler/eval/expression_dispatch.rs:17–33`, `style_code_tail` merges an outer
set-rule delta with the marked constructor delta. That merge occurs before the
warning scan (`:322–345`). `entities/content/fields.rs:136–154` only checks the
marker and Heading body; its supplement lookup (`:260–274`) then reads the merged
style map without identifying who supplied the supplement.

Concrete source to measure and test before accepting the candidate:

```typst
#{ set heading(supplement:state("k",0).update(1)); heading(numbering:"1")[A]; return "x" }
```

Source analysis predicts a false extra update hint in the candidate, although
this supplement belongs to the set rule. The existing negative control uses
unmarked `heading[A]` and cannot refute this case. A correct fix must preserve the
origin of the explicit supplement through merging, rather than treating every
supplement found beside a constructor marker as explicit. This finding has been
sent to the coordinator and effects owner; oracle and product confirmation remain
required. No public API expansion is recommended by this review.

## Bounded local corrections otherwise supported

The original shared visitor misses `RefElem.supplement`, despite its static
Content payload. The measured oracle warning confirms this omission. The local
Ref arm closes it without altering the shared visitor. The known RawLineElem
concrete downcast reaches its body without invoking extension field enumeration
or evaluating a context/function. Unknown dynamic extensions remain opaque.

PageRun must prune the shared visual traversal after explicitly scanning body:
the measured header/footer/supplement/background/foreground are deferred and
must not cause this static warning. The candidate's local recursion plus early
`return false` does this correctly. A false body result prunes only that page;
subsequent siblings are still visited. A true result is retained by the initial
`if found` guard, so later siblings cannot reset a previously found update.

Metadata's nested Value decision admits Content/LocatedContent and Array, while
Dict and callbacks remain opaque. This matches the existing measured controls.
The nested closed Value match is an independent future-variant obligation.

The static-fields audit correctly declines complete parity certification. Figure
supplement is lost at its producer, and Equation set/constructor supplement
origins share storage. Derived Outline/Table children and arbitrary extensions
remain unmeasured. These are open obligations, not accepted omissions.

## Display and paragraph limits

The reviewed display conversion preserves rich Content and unwraps LocatedContent
without executing content or rewriting it as a debug representation. No concrete
new regression was found. Constructed rich-content fixtures do not alone prove
all actual query-produced metadata paths or generic rich-value display parity.

The revised paragraph L0 correctly preserves the measured existing stage
recognizer and rejects an unsupported claim that all Located/Realized inputs are
unreachable. Carrier preservation tests and producer effect controls do not prove
universal stage reachability. Broader paragraph preparation differences remain
explicit follow-up work. Structural exhaustiveness must be checked at the final
predicate independently of these semantic controls.

## Follow-up: proposed explicit snapshot visibility

The effects owner confirmed the numbering/inherited-supplement discriminator
against the pinned oracle and proposed a separate private
`heading.constructor.supplement` snapshot, governed by the shared nucleus.
`heading-snapshot-review-state.json` freezes this subsequent read-only review.
The proposed origin snapshot addresses the ambiguity above without depending on
merged public supplement values. Runtime preservation still needs coordinator
validation.

No new public projection leak was found: content field classification exposes
only the named supplement/numbering fields and a fixed heading field list
(`field_access/content_methods.rs:44–52,277–285`); `func` retains the existing
marker/body check (`:477–479`). Content field lookup uses exact keys
(`entities/content/fields.rs:260–281`). Heading representation descends to its
child without serializing custom styles (`eval/repr/content.rs:554–615`).
JSON/TOML/YAML share `stdlib/loading/encoding.rs:18` and its Styled projection
serializes child plus the literal `styles(..)` (`:67–72`), not custom values.
Internal hash/merge storage still carries the snapshot by design. Existing
custom-only morphology remains transparent; the paged optimization classifier
already refuses the old constructor marker, so adding this key does not newly
change the heading's optimization classification. This is bounded source
evidence, not a claim that every serialization or runtime path was executed.
