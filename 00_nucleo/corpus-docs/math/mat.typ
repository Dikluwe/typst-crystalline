// Corpus docs-derived (Passo 998) — página: https://typst.app/docs/reference/math/mat/
// Cada caso cita a frase exacta da documentação que o motiva.

#set page(width: auto, height: auto, margin: 1cm)
#set text(font: "New Computer Modern", size: 11pt)

// Fonte: https://typst.app/docs/reference/math/mat/
// Citação: "The elements of a row should be separated by commas, while the rows themselves should be separated by semicolons."
$ mat(
  1, 2, ..., 10;
  2, 2, ..., 10;
  dots.v, dots.v, dots.down, dots.v;
  10, 10, ..., 10;
) $

// Fonte: https://typst.app/docs/reference/math/mat/#parameters-delim
// Citação: "Can be a single character specifying the left delimiter, in which case the right delimiter is inferred."
#set math.mat(delim: "[")
$ mat(1, 2; 3, 4) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Otherwise, can be an array containing a left and a right delimiter."
#set math.mat(delim: ("[", "]"))
$ mat(1, 2; 3, 4) $

// Fonte: https://typst.app/docs/reference/math/mat/#parameters-align
// Citação: "The horizontal alignment that each cell should have."
#set math.mat(align: right)
$ mat(-1, 1, 1; 1, -1, 1; 1, 1, -1) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "Content in cells can be aligned with the `align` parameter, or content in cells that are in the same row can be aligned with the `&` symbol."
$ mat(1 & +2; -1 & -2) $

// Fonte: https://typst.app/docs/reference/math/mat/#parameters-augment
// Citação: "A single number: A vertical augmentation line is drawn after the specified column number. Negative numbers start from the end."
$ mat(1, 0, 1; 0, 1, 2; augment: #2) $
// Equivalent to:
$ mat(1, 0, 1; 0, 1, 2; augment: #(-1)) $

// Fonte: https://typst.app/docs/reference/math/mat/#parameters-augment
// Citação: "With a dictionary, multiple augmentation lines can be drawn both horizontally and vertically. Additionally, the style of the lines can be set."
$ mat(0, 0, 0; 1, 1, 1; augment: #(hline: 1, stroke: 2pt + green)) $

// Caso derivado da prosa, sem exemplo na doc:
// Citação: "`vline`: The offsets at which vertical lines should be drawn. [...] Accepts either an integer for a single line, or an array of integers for multiple lines."
$ mat(1, 2, 3; 4, 5, 6; 7, 8, 9; augment: #(vline: (1, 2), hline: -1)) $

// Fonte: https://typst.app/docs/reference/math/mat/#parameters-gap
// Citação: "This is a shorthand to set `row-gap` and `column-gap` to the same value."
#set math.mat(gap: 1em)
$ mat(1, 2; 3, 4) $

// Fonte: https://typst.app/docs/reference/math/mat/#parameters-row-gap
// Citação: "The gap between rows."
#set math.mat(row-gap: 1em)
$ mat(1, 2; 3, 4) $

// Fonte: https://typst.app/docs/reference/math/mat/#parameters-column-gap
// Citação: "The gap between columns."
#set math.mat(column-gap: 1em)
$ mat(1, 2; 3, 4) $

// Fonte: https://typst.app/docs/reference/math/mat/#parameters-rows
// Citação: "An array of arrays with the rows of the matrix."
#let data = ((1, 2, 3), (4, 5, 6))
#let matrix = math.mat(..data)
$ v := matrix $
