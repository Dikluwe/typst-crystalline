#let repeat-zero(n) = range(n).map(_ => 0)

#metadata((
  arrays: (
    empty: repr(()),
    singleton: repr((0,)),
    short: repr((0, 1, 2)),
    inner-length-49: repr(("x" * 47,)),
    inner-length-50: repr(("x" * 48,)),
    inner-length-51: repr(("x" * 49,)),
    zeros-body-49: repr(repeat-zero(17)),
    zeros-body-52: repr(repeat-zero(18)),
    internal-multiline: repr((repeat-zero(18), 0)),
    nested-long: repr((repeat-zero(18), repeat-zero(18))),
  ),
  math-ops: (
    csc: repr(math.csc),
    dim: repr(math.dim),
    lim: repr(math.lim),
    arcsin: repr(math.arcsin),
    sum-non-op-control: repr(math.sum),
    user-false-string: repr(math.op("foo", limits: false)),
    user-true-string: repr(math.op("foo", limits: true)),
    user-false-content: repr(math.op([foo], limits: false)),
    user-markup: repr(math.op([*f* oo], limits: false)),
    user-escaping-string: repr(math.op("a \"quote\" \\ slash", limits: false)),
    user-escaping-content: repr(math.op([a \# \*], limits: false)),
  ),
)) <p1290-contract>
