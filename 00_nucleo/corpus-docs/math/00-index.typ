// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/#variables
// Citação: "In math, single letters are always displayed as is. Multiple letters, however, are interpreted as variables and functions. To display multiple letters verbatim, you can place them into quotes and to access single letter variables, you can use the hash syntax."
$ A = pi r^2 $
$ "area" = pi dot "radius"^2 $
$ cal(A) :=
    { x in RR | x "is natural" } $
#let x = 5
$ #x < 17 $

// Fonte: https://typst.app/docs/reference/math/#symbols
// Citação: "Math mode makes a wide selection of symbols like `pi`, `dot`, or `RR` available. Many mathematical symbols are available in different variants. You can select between different variants by applying modifiers to the symbol. Typst further recognizes a number of shorthand sequences like `=>` that approximate a symbol. When such a shorthand exists, the symbol's documentation lists it."
$ x < y => x gt.eq.not y $

// Fonte: https://typst.app/docs/reference/math/#line-breaks
// Citação: "Formulas can also contain line breaks. Each line can contain one or multiple _alignment points_ (`&`) which are then aligned."
$ sum_(k=0)^n k
    &= 1 + ... + n \
    &= (n(n+1)) / 2 $

// Fonte: https://typst.app/docs/reference/math/#function-calls
// Citação: "Math mode supports special function calls without the hash prefix. In these "math calls", the argument list works a little differently than in code:"
$ frac(a^2, 2) $
$ vec(1, 2, delim: "[") $
$ mat(1, 2; 3, 4) $
$ mat(..#range(1, 5).chunks(2)) $
$ lim_x =
    op("lim", limits: #true)_x $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "To write a verbatim comma or semicolon in a math call, escape it with a backslash. The colon on the other hand is only recognized in a special way if directly preceded by an identifier, so to display it verbatim in those cases, you can just insert a space before it."
$ f(a\, b) $
$ mat(1\; 2) $
$ f(x : y) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Functions calls preceded by a hash are normal code function calls and not affected by these rules."
$ #calc.max(1, 2) + #range(3).len() $

// Fonte: https://typst.app/docs/reference/math/#alignment
// Citação: "When equations include multiple _alignment points_ (`&`), this creates blocks of alternatingly right- and left-aligned columns."
$ (3x + y) / 7 &= 9 && "given" \
  3x + y &= 63 & "multiply by 7" \
  3x &= 63 - y && "subtract y" \
  x &= 21 - y/3 & "divide by 3" $

// Fonte: https://typst.app/docs/reference/math/#math-fonts
// Citação: "You can set the math font by with a show-set rule as demonstrated below. Note that only special OpenType math fonts are suitable for typesetting maths."
#show math.equation: set text(font: "Pennstander Math")
$ sum_(i in NN) 1 + i $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "All math functions are part of the `math` module, which is available by default in equations. Outside of equations, they can be accessed with the `math.` prefix."
#math.frac([a], [b])

// Fonte: https://typst.app/docs/reference/math/#accessibility
// Citação: "To make math accessible, you must provide alternative descriptions of equations in natural language using the `alt` parameter of `math.equation`."
#math.equation(
  alt: "d S equals delta q divided by T",
  block: true,
  $ dif S = (delta q) / T $,
)
