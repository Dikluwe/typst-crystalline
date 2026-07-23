# Prompt L0 — entities/font-book
Hash do Código: f3f3080f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/font_book.rs`
**Criado em**: 2026-03-26
**Atualizado em**: 2026-07-23 (P875 — cobertura Unicode para filtro de fallback)
**ADRs relevantes**: ADR-0022 (FontBook real em L1), ADR-0019 (ttf-parser em L3)

## Contexto

`FontBook` é o catálogo de metadados de fontes disponíveis no sistema.
Contém `Vec<FontInfo>` com família, variante (estilo/peso/largura), flags e
cobertura Unicode. Populado em L3 via `font_info_from_bytes` (ttf_parser); consultado em L1.

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

/// Bitmap de cobertura Unicode por blocos de 256 codepoints.
/// P875 — usado pelo shaper para filtrar o fallback global: um candidato só
/// é considerado para um caractere se o bloco desse caractere estiver no bitmap.
pub struct Coverage {
    pub blocks: [u64; 64], // 64×64 = 4096 bits → blocos 0..4095 (U+0000..U+3FFFF)
}

impl Coverage {
    pub fn new() -> Self;
    pub fn insert(&mut self, codepoint: u32);
    pub fn contains(&self, codepoint: u32) -> bool;
    pub fn is_empty(&self) -> bool;
}

pub struct FontInfo {
    pub family: String,
    pub variant: FontVariant,
    pub flags: FontFlags,
    pub coverage: Coverage, // P875
}

pub struct FontBook { ... }
impl FontBook {
    pub fn new() -> Self
    pub fn push(&mut self, info: FontInfo)
    pub fn infos(&self) -> &[FontInfo]
    pub fn len(&self) -> usize
    pub fn is_empty(&self) -> bool
    pub fn select(&self, family: &str, variant: &FontVariant) -> Option<usize>
    pub fn select_family<'a>(&'a self, family: &'a str) -> impl Iterator<Item = usize> + 'a
    pub fn select_fallback(
        &self,
        like: Option<&FontInfo>,
        variant: &FontVariant,
        candidates: impl IntoIterator<Item = usize>,
    ) -> Option<usize>

    /// P875 — devolve os índices de slots que podem cobrir `c`, i.e., cujo
    /// `coverage` contém o bloco de 256 codepoints a que `c` pertence.
    /// O chamador (L3) ainda deve confirmar com `face_covers_char`/`glyph_index`
    /// (o bitmap é aproximado por bloco), mas isto evita carregar faces cujo
    /// bitmap já exclui o caractere.
    pub fn candidates_for_char(&self, c: char) -> impl Iterator<Item = usize> + '_
}
```

## `select_fallback` — scoring de similaridade (P838)

Paridade com o vanilla (`typst-library/src/text/font/book.rs:139-185`
`find_best_variant` + `similarity` + `distance`). A cobertura do caractere
**não** é verificada aqui (L1 não tem acesso a faces); o chamador (L3) filtra
previamente os candidatos que cobrem o caractere e entrega os índices.

Score por candidato (maior vence; comparação estritamente maior preserva o
primeiro candidato em empate total — mesma regra do vanilla, ordem do book):

1. `similarity(candidato, like)` (só se `like` for `Some`), tuplo:
   - `monospace` igual nos dois (bool);
   - `serif` igual nos dois (bool);
   - `shared_prefix_words(família_candidato, família_like)` — nº de palavras
     partilhadas no prefixo, via `unicode_words()` (unicode-segmentation,
     ADR-0013) — a mesma função do vanilla;
   - `Reverse(família_candidato.len())` — em empate, a família mais curta
     (menos especializada) vence.
2. `Reverse(distance(candidato, variant))` com
   `distance = (style_distance, stretch_distance, weight_distance)`
   (distâncias absolutas já existentes em `FontStyle`/`FontWeight`;
   `FontStretch::distance` = `abs_diff` — adicionada neste passo).

**Divergência declarada (mecânica, ADR-0107):** o vanilla tem um 3.º elemento
no score (preferência por fontes variáveis, flag `VARIABLE`) e considera eixos
de variação na `distance`. O `FontInfo` cristalino não tem eixos nem flag
`VARIABLE` (VF é tratada por `axis_variations`, P525/P836); esses dois
elementos são omitidos.

## `Coverage` — bitmap por blocos de 256 codepoints (P875)

- Cada bit representa um bloco contíguo de 256 codepoints.
- `insert(c)` activa o bloco `c / 256`.
- `contains(c)` verifica o bit do bloco `c / 256`.
- A extracção em L3 percorre a tabela `cmap` da fonte e insere o bloco de cada
codepoint presente. Não é necessário mapear todos os codepoints — o bitmap é
aproximado por bloco.

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

Dado FontInfo com coverage contendo bloco 0x370 (grego)
Quando candidates_for_char('α') for chamado
Então o índice desse FontInfo aparece na lista

Dado FontInfo com coverage vazio
Quando candidates_for_char('α') for chamado
Então o índice desse FontInfo NÃO aparece na lista
```

## Relação com Outros Módulos

| Módulo | Como consome `FontBook` |
|--------|------------------------|
| `fonts.rs` (L3) | Popula `FontBook` com `FontInfo` extraído de bytes, incluindo `coverage` |
| `shaper.rs` (L3) | Usa `candidates_for_char` para filtrar o fallback global antes de carregar faces |
| `Layouter` (L1) | Recebe `&dyn FontMetrics` — nunca toca em `FontBook` directamente |

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-03-26 | Criação — FontBook em L1 | `font-book.md`, `font_book.rs` |
| 2026-07-22 | P838 — scoring de fallback por similaridade | `font-book.md`, `font_book.rs` |
| 2026-07-23 | P875 — `Coverage` em `FontInfo` + `candidates_for_char` para filtro de fallback | `font-book.md`, `font_book.rs`, `fonts.rs`, `shaper.rs` |
