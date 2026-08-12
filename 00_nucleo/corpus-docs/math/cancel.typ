// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/cancel/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/cancel/
// Citação: "Displays a diagonal line over a part of an equation. This is commonly used to show the elimination of a term."
Here, we can simplify:
$ (a dot b dot cancel(x)) /
    cancel(x) $

// Fonte: https://typst.app/docs/reference/math/cancel/#parameters-length
// Citação: "The length of the line, relative to the length of the diagonal spanning the whole element being “cancelled”. A value of `100%` would then have the line span precisely the element’s diagonal."
$ a + cancel(x, length: #200%)
    - cancel(x, length: #200%) $

// Fonte: https://typst.app/docs/reference/math/cancel/#parameters-inverted
// Citação: "Whether the cancel line should be inverted (flipped along the y-axis). For the default angle setting, inverted means the cancel line points to the top left instead of top right."
$ (a cancel((b + c), inverted: #true)) /
    cancel(b + c, inverted: #true) $

// Fonte: https://typst.app/docs/reference/math/cancel/#parameters-cross
// Citação: "Whether two opposing cancel lines should be drawn, forming a cross over the element. Overrides `inverted`."
$ cancel(Pi, cross: #true) $

// Fonte: https://typst.app/docs/reference/math/cancel/#parameters-angle
// Citação: "How much to rotate the cancel line. - If given an angle, the line is rotated by that angle clockwise with respect to the y-axis. - If `auto`, the line assumes the default angle; that is, along the rising diagonal of the content box. - If given a function `angle => angle`, the line is rotated, with respect to the y-axis, by the angle returned by that function. The function receives the default angle as its input."
$ cancel(Pi)
  cancel(Pi, angle: #0deg)
  cancel(Pi, angle: #45deg)
  cancel(Pi, angle: #90deg)
  cancel(1/(1+x), angle: #(a => a + 45deg))
  cancel(1/(1+x), angle: #(a => a + 90deg)) $

// Fonte: https://typst.app/docs/reference/math/cancel/#parameters-stroke
// Citação: "How to stroke the cancel line."
$ cancel(
  sum x,
  stroke: #(
    paint: red,
    thickness: 1.5pt,
    dash: "dashed",
  ),
) $
