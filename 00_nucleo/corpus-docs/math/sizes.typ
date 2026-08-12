// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/sizes/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/sizes/#functions-display
// Citação: "Forced display style in math. This is the normal size for block equations."
$sum_i x_i/2 = display(sum_i x_i/2)$

// Fonte: https://typst.app/docs/reference/math/sizes/#functions-inline
// Citação: "Forced inline (text) style in math. This is the normal size for inline equations."
$ sum_i x_i/2
    = inline(sum_i x_i/2) $

// Fonte: https://typst.app/docs/reference/math/sizes/#functions-script
// Citação: "Forced script style in math. This is the smaller size used in powers or sub- or superscripts."
$sum_i x_i/2 = script(sum_i x_i/2)$

// Fonte: https://typst.app/docs/reference/math/sizes/#functions-sscript
// Citação: "Forced second script style in math. This is the smallest size, used in second-level sub- and superscripts (script of the script)."
$sum_i x_i/2 = sscript(sum_i x_i/2)$

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "cramped — Whether to impose a height restriction for exponents, like regular sub- and superscripts do." (parâmetro de display, Default: false)
$ display(x^2, cramped: #true) = display(x^2, cramped: #false) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "cramped — Whether to impose a height restriction for exponents, like regular sub- and superscripts do." (parâmetro de inline, Default: false)
$ inline(x^2, cramped: #true) = inline(x^2, cramped: #false) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "cramped — Whether to impose a height restriction for exponents, like regular sub- and superscripts do." (parâmetro de script, Default: true)
$ script(x^2, cramped: #false) = script(x^2) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "cramped — Whether to impose a height restriction for exponents, like regular sub- and superscripts do." (parâmetro de sscript, Default: true)
$ sscript(x^2, cramped: #false) = sscript(x^2) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "body — The content to size."
$ display(x + y) + inline(x + y) + script(x + y) + sscript(x + y) $
