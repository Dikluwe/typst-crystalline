# L0 — Relative quantity
Hash do Código: 485e934f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/rel.rs`

## Contract

`Rel<T>` stores public `rel: f64` (fraction of a contextual quantity) and
`abs: T` (absolute offset), with Debug/Clone/Copy/PartialEq as supported by T.
Preserve generic Default, Add/Sub/Neg and multiplication/division by f64 with
the corresponding T bounds. Mixed Length operations remain:
Rel+Length, Length+Rel, Rel-Length and Length-Rel.

`from_percent(pct)` uses `rel = pct / 100` and an absolute default. For Length,
`zero`, `resolve(&self, context: Length) -> Length` and `is_abs_zero` remain
available. Resolution is fraction × context + absolute offset; a 50% relative
value plus 2cm resolves to 7cm against 10cm. Multiplying 50% by 2 yields 100%.
Mixed addition is commutative; subtraction preserves operand order.

Layout supplies the context; this type does not choose container dimensions
or define context-free ordering. Rel<Abs>/Rel<Fr> behavior is outside this
specified scope. Structural equality is not a contextual `<`/`>` operation.
