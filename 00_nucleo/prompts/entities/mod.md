# entities — módulo raiz
Hash do Código: 1f3c3465

Módulo de re-exportação dos tipos de domínio de L1.

Contém: `FileId`, `SyntaxKind`, `Span`, `Spanned`.

## P836 — módulo `font_variations`

Registado `pub mod font_variations;` (P836, achado #21 de P831):
tipo `FontVariations` de eixos OpenType explícitos — ver
`entities/font_variations.md`.

## P1462 — módulo `document_format_options` (PROPOSTO; gate ADR-0127)

Após confirmação humana de P1462, declarar
`pub mod document_format_options;` para expor os carriers puros definidos no
L0 proprietário `entities/document_format_options.md`. Esta fachada não
duplica defaults, parsing, extração documental ou precedência de exportação.
