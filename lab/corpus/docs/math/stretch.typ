// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/stretch/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/stretch/
// Citação: "Stretches a glyph."
$ H stretch(=)^"define" U + p V $
$ f : X stretch(->>, size: #150%)_"surjective" Y $
$ x stretch(harpoons.ltrb, size: #3em) y
    stretch(\[, size: #150%) z $

// Fonte: https://typst.app/docs/reference/math/stretch/#parameters-size
// Citação: "The size to stretch to, relative to the maximum size of the glyph and its attachments."
// Citação: "In the example below, when the size parameter is increased from 101% to 200%, the selected glyph remains the same, so the actual size does not change."
#for size in (
  100%, // short
  101%, 200%, // tall
  201%, 300%, 400%, 500%, 600%, // taller
) {
  $stretch(integral, size: #size)$
}

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "This function can also be used to automatically stretch the base of an attachment, so that it fits the top and bottom attachments."
$ stretch(=)^"a very long top attachment" $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Note that only some glyphs can be stretched, and which ones can depend on the math font being used."
$ stretch(a, size: #200%) b $

// Caso derivado da prosa (tabela de parâmetros), sem exemplo na doc:
// Citação (parâmetro size): "Default: 100% + 0pt"
$ stretch(integral) x $
