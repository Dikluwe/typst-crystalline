# Semantic review of explicit structural complements

Prepared before implementation while source/L0 are frozen. This is a domain
review, not a claim that a diagnostic count establishes correctness. V19 remains
enabled; broad explicit arms require the following owner-specific reasons.

| Case | Why the complement shares this disposition | What it does not mean / preservation boundary |
|---|---|---|
| 005 | This predicate identifies Matrix/Cell strict-field presence through Source, ShowRecipe, Located and the Realized visual. All other nominal nodes are outside that narrow family. | It is not a global field-access classifier. Styled remains opaque here, the broader presence predicate remains separate, and a supplied snapshot stays authoritative even when its key is absent. |
| 006 | Kind, Label, Location and Regex terminal selectors carry no native constructor metadata for this validator to inspect. Native constructors without catalogue metadata also retain existing acceptance. | Acceptance by this metadata validator does not admit these selectors at every language boundary. Recursive compositions still validate every operand; known non-locatable native metadata still errors with its original span. |
| 008 | Void and Normal are different arity domains and are each handled explicitly; rejected length/type guards retain their previous anchor fallback within that domain. | This is diagnostic attribution, not HTML argument acceptance. Spread precedence, missing capture fallback and named full-span/value-span precedence remain unchanged. No broad nominal complement is needed. |
| 009 | Only Content and LocatedContent can project an OutlineEntry body for this dispatch gate. Every other Value has no content representation at this boundary. | None means dispatch defers, not an invented method error. It must preserve the cached receiver and avoid evaluating arguments on refusal. Other content shapes also defer via the existing body predicate. |
| 010 | Explicit Value families retain their owning method handler; the residual Value families have no handler in this instance-dispatch stage. Within Type, only Counter has the existing at/display static-method exception. | Unsupported method names stay inside their nominal family and defer without evaluating arguments. State/Counter and other general handlers may diagnose themselves. Other Type values are not globally forbidden method receivers: this stage returns control to later dispatch. |
| 017 | Values other than Str/Path cannot supply a concrete include trace path and share the preexisting diagnostic placeholder. | This arm does not authorize an include or construct a new type policy. The immediately following exhaustive source dispatcher rejects invalid values before loading files; error text/timing/span and trace absence stay intact. |
| 018 | Int/Float/Decimal/Length/Relative/Ratio/Angle have the existing pre-dispatch numeric zero checks. Every other Value has no zero-check policy in this preliminary gate. | No-op means continue to the existing typed binary operation, not accept division. Duration zero remains the later operation's responsibility; IEEE values, negative zero, and both Relative components preserve their existing predicates. |
| 020 | Auto/Bool/Length/Func are the defined inherited outline-indent representations. Other Values do not supply an inherited indent at this capture point. | No inherited value must not override an explicit constructor argument or execute a Func. This does not change later constructor casts or invent automatic conversion. |
| 022 | Dict is the per-side bleed representation and Auto has its established rejection. Every other Value belongs to the scalar-cast route and must reach extract_rel_length. | This complement is deliberately not a neutral default: it preserves both valid scalar conversions and existing invalid-value diagnostics. It cannot be replaced with a blanket error or acceptance. Side/axis/rest precedence remains in the Dict owner. |
| 023 | A literal font warning can inspect Str directly or a Dict whose name field is Str. Other values and non-string names provide no literal family name. | Absence of a warning name is not font acceptance. Earlier cast/validation remains authoritative. Regex family selection produces no invented literal-name warning; missing Dict names remain distinct from present non-string values. |
| 024 | A static set target path is an Ident or recursively a FieldAccess over another static path. Other Expr forms require evaluation rather than pure name-path validation. | False does not reject the expression globally. It preserves the owning evaluator route, target-before-argument diagnostic ordering, and the false-condition bypass. |

The grouping criterion is the meaning of the decision at that call site, not a
variant-count threshold. Future variants must cause an exhaustiveness failure at
each selected decision, including the nested Type and font-name Value matches,
until their owner classifies them. Compile-negative evidence is coordinated by
root; this document is not a replacement for it. Existing warnings, accepted
values, refused guards, spans and unevaluated argument behavior remain part of
the tests and owning L0 contracts.
