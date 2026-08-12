// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/lr/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/lr/#functions-mid
// Citação: "Scales delimiters vertically to the nearest surrounding `lr()` group."
$ { x mid(|) sum_(i=1)^n w_i abs(f_i (x)) < 1 } $

// Fonte: https://typst.app/docs/reference/math/lr/#functions-abs
// Citação: "Takes the absolute value of an expression."
$ abs(x/2) $

// Fonte: https://typst.app/docs/reference/math/lr/#functions-norm
// Citação: "Takes the norm of an expression."
$ norm(x/2) $

// Fonte: https://typst.app/docs/reference/math/lr/#functions-floor
// Citação: "Floors an expression."
$ floor(x/2) $

// Fonte: https://typst.app/docs/reference/math/lr/#functions-ceil
// Citação: "Ceils an expression."
$ ceil(x/2) $

// Fonte: https://typst.app/docs/reference/math/lr/#functions-round
// Citação: "Rounds an expression."
$ round(x/2) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "size — The size of the brackets, relative to the height of the wrapped content. Default: The current value of `lr.size`."
// (descrição do parâmetro `size`, comum a abs/norm/floor/ceil/round)
$ abs(x/2, size: #150%) $
$ norm(x/2, size: #50%) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "While matched delimiters scale by default, this can be used to scale unmatched delimiters and to control the delimiter scaling more precisely."
$ lr(]x/2]) $
$ lr(]x/2], size: #200%) $

// Fonte: https://typst.app/docs/reference/math/lr/
// Citação: bloco "Example" da página (verbatim, inclui o `#set math.lr(size: 1em)`);
// "To prevent a delimiter from being matched by Typst, and thus auto-scaled, escape it with a backslash. To instead disable auto-scaling completely, use `set math.lr(size: 1em)`."
// Nota: este bloco contém um `#set` que afecta o resto do documento, por isso fica no fim.
$ [a, b/2] $
$ lr(]sum_(x=1)^n], size: #50%) x $
$ abs((x + y) / 2) $
$ \{ (x / y) \} $
#set math.lr(size: 1em)
$ { (a / b), a, b in (0; 1/2] } $
