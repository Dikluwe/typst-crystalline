# Prompt L0 — entities/font-book
Hash do Código: 7916774f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/font_book.rs`
**ADRs relevantes**: ADR-0022 (FontBook real em L1), ADR-0019 (ttf-parser em L3)

## Contexto

`FontBook` é o catálogo de metadados de fontes disponíveis no sistema.
Contém `Vec<FontInfo>` com família, variante (estilo/peso/largura) e flags.
Populado em L3 via `font_info_from_bytes` (ttf_parser); consultado em L1.

`FontBook` em L1 é domínio puro: não sabe de bytes, paths nem ttf_parser.
A extracção de `FontInfo` a partir de bytes fica em `03_infra/src/fonts.rs`.

## Interface pública

```rust
pub enum FontStyle { Normal, Italic, Oblique }
pub struct FontWeight(pub u16);   // 100–900 CSS
impl FontWeight {
    pub const THIN|EXTRALIGHT|LIGHT|REGULAR|MEDIUM|SEMIBOLD|BOLD|EXTRABOLD|BLACK: Self;
    pub fn from_number(weight: u16) -> Self;   // clamp 100-900
    pub fn from_name(name: &str) -> Option<Self>;  // Passo 129 — 9 canónicos
    pub fn to_number(self) -> u16;
    pub fn distance(self, other: Self) -> u16;
}
pub struct FontStretch(pub u16);  // 500–2000 (NORMAL=1000)
pub struct FontVariant { pub style, pub weight, pub stretch }
pub struct FontFlags { pub monospace: bool, pub serif: bool }
pub struct FontInfo { pub family: String, pub variant: FontVariant, pub flags: FontFlags }

pub struct FontBook { ... }
impl FontBook {
    pub fn new() -> Self
    pub fn push(&mut self, info: FontInfo)
    pub fn infos(&self) -> &[FontInfo]
    pub fn len(&self) -> usize
    pub fn is_empty(&self) -> bool
    pub fn select(&self, family: &str, variant: &FontVariant) -> Option<usize>
    pub fn select_family<'a>(&'a self, family: &'a str) -> impl Iterator<Item = usize> + 'a
}
```

## Critérios de Verificação

```
Dado FontBook vazio
Quando select() for chamado
Então None

Dado FontBook com duas fontes da mesma família com pesos 300 e 700
Quando select("família", weight=400) for chamado
Então retorna índice da fonte com peso 300 (distância 100 < 300)

Dado FontBook com família "Liberation Sans"
Quando select("liberation sans", ...) ou select("LIBERATION SANS", ...) for chamado
Então retorna Some (case-insensitive)

Dado FontStretch::from_number(5)
Então FontStretch::NORMAL (1000)

Dado font_info_from_bytes(bytes_invalidos, 0)
Então None
```
