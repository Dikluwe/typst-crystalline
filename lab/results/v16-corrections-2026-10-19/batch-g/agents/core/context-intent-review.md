# Context comparison: observation, source and limits

No new language behavior is implemented or authorized by this review. The current source/L0 snapshot and initial state are in source-before.json.gz. The first immutable29-expression receipt is equality-probes/receipt.json SHA-256 c03c9e8653b09d21fe0cd55ada2a920e6820b2efcf5a06a77454eca896b128a6. Additional explicit-parenthesis, opaque panic, captured environment, unlike-kind and rendered/query controls are in context-intent-probes/receipt.json SHA-256 3523208ad9be23fcc87960083db10fa7a1805910b758cdb44de83f2eb42fb86c. Oracle commit/tree/binary are the repository's current pinned identities; product is the immutable previously verified build4 binary, not a rebuild of newly added tests/L0.

## Exact observations before interpretation

`{ let a = (context [A]); let b = (context [B]); (equal: a == b, left: repr(a), right: repr(b), left_fields: a.fields(), right_fields: b.fields(), left_type: repr(type(a)), right_type: repr(type(b))) }` gives oracle equal:true, both repr context(), fields {}, type content; product equal:false with the same fields/types/repr. Explicit parentheses and variable assignment put the comparison outside either deferred expression.

`{let a = (context panic("must-not-run-A")); let b = (context panic("must-not-run-B")); (equal: a == b, repr: repr(a), fields: a.fields())}` succeeds without either panic; oracle equal:true/product false. Two closures created through `f(x) = context x` with captured "A" and "B" produce the same equality split. A context remains unequal to its body or empty content in both directions. No accidental eager body comparison or syntax swallowing is demonstrated.

The independent `distinct-rendered.typ` declares parenthesized Context A and B, records their comparison in labelled Metadata and emits both. Both products render AB; oracle records equal:true/product false. Thus the two Context bodies remain distinct emitted computations despite opaque pre-expansion comparison. The relation does not authorize merging occurrences or evaluating one fewer closure.

## Source and explicit intent

Pinned `lab/typst-original/crates/typst-library/src/foundations/context.rs:63–71` declares ContextElem's sole func field as required and internal. Its constructor rejects manual construction; :78–87 calls the closure later with location/styles. The field is deliberately internal, not accidentally missing a getter.

Pinned `lab/typst-original/crates/typst-macros/src/elem.rs:393–410,412–425` explicitly excludes internal fields from the native field table and field-name map. `foundations/content/vtable.rs:262–270` enumerates that table; `foundations/content/raw.rs:354–361` uses its equality hook or all-field comparison. Therefore no externally represented Context field remains to distinguish closures in the fallback. This is a mechanism explaining the measured result, not by itself proof of intended equality.

Pinned language documentation `lab/typst-original/docs/content/reference/language/context.typ:11,23–36` explicitly calls the created expression opaque content: callers cannot peek into its result, which depends on insertion surroundings and may execute zero, one or multiple times. This is affirmative intent to hide body/results and retain deferred insertion behavior. The docs do **not** explicitly state "all Context expressions compare equal". No such stronger quotation or normative equality test was found in the inspected Context docs/tests; intent for that exact boolean must not be fabricated.

## Bounded inference and refuter

The measured opaque equality is consistent with explicit field exclusion and documented opacity, while comparing IDs or executing bodies would add facts unavailable in the language morphology. It is therefore a defensible bounded parity inference to keep mechanical ContextBlock Eq/Hash by its owned id and treat raw Context morphology as the same opaque element in the language comparison owner only. This is not evidence that two closures are behaviorally interchangeable, that their emissions can be deduplicated, or that occurrence identity/observation equality can omit IDs/capabilities.

A source or language requirement making Context equality depend on function, identity or output would refute the inference. A comparison-only correction must preserve Context-versus-other-kind inequality, nested containers, fields, aliases, source wrappers, captured environments, zero execution during comparison and separate AB emission. It must not change public morph_canon, the Context entity's id equality/hash, occurrence reservation, query/ObservationEq, constructors or phase.

The root must classify and scope any such language-owner correction before implementation. With no such extension, report the measured independent-Context language gap explicitly instead of claiming equality closure from mechanical id delegation alone.
