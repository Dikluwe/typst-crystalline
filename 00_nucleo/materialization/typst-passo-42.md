# Passo 42 — Glifos extensíveis e delimitadores stretchy

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/math_constants.rs` — `MathConstants` com `to_pt()`
- `01_core/src/rules/math/layout.rs` — `MathLayouter`, `layout_root`, `layout_frac`, `offset_item`
- `01_core/src/entities/content.rs` — `Content::MathDelimited`, `Content::MathRoot`
- `01_core/src/entities/layout_types.rs` — `FrameItem::Text`, `FrameItem::Line`
- `03_infra/src/font_metrics.rs` — `FontBookMetrics`, `face.tables().math`
- `03_infra/src/export.rs` — PDF com CIDFont/ToUnicode
- ADR-0019 — `ttf-parser` em L3

Pré-condição: `cargo test` — 480 L1 + 64 L3 + 50 parity, zero violations.

---

## Contexto

Actualmente, delimitadores (`(`, `)`, `[`, `]`, `{`, `}`) e o símbolo
radical (`√`) são renderizados com tamanho fixo — o caractere base da
fonte, independentemente da altura do conteúdo que envolvem.

Fontes OpenType com tabela MATH definem duas formas de crescer um glifo
verticalmente (ou horizontalmente):

1. **Variantes de tamanho** (`MathGlyphVariant`): glifos alternativos
   progressivamente maiores. Ex: `(` tem variantes para alturas de 1x,
   1.5x, 2x, 2.5x do tamanho base.

2. **Montagem por partes** (`GlyphAssembly`): quando nenhuma variante é
   grande o suficiente, o glifo é construído por partes repetíveis
   (extensores) e partes fixas (caps). Ex: `{` é montado com
   top-cap + extensor + meio + extensor + bottom-cap.

A tabela relevante é `MathVariants`, acessível via
`face.tables().math?.variants`.

Este passo implementa variantes de tamanho (forma 1). A montagem por
partes (forma 2) fica para um passo futuro — é mais complexa e requer
`FrameItem` adicional ou composição de múltiplos glifos.

---

## Decisão arquitectural

O mesmo padrão dos passos anteriores: L3 extrai dados da fonte, L1
usa-os via interface.

Criar `GlyphVariants` em L1 como struct de domínio puro. L3 preenche
a partir de `ttf-parser`. O `MathLayouter` consulta as variantes para
seleccionar o glifo com a altura mínima necessária.

**Não é necessária ADR nova** — ADR-0019 já autoriza `ttf-parser` em L3.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. API de MathVariants no ttf-parser
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "MathVariants\|variants\|glyph_variant\|GlyphAssembly\|GlyphConstruction\|min_connector" {} | head -30

# 2. Estrutura de MathGlyphConstruction
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "struct.*Variant\|struct.*Assembly\|struct.*Part\|vert_glyph_variants\|horiz_glyph_variants" {} | head -20

# 3. Como obter variantes verticais para um glyph_id
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "fn.*variant\|fn.*construction\|fn.*assembly" {} | head -20

# 4. glyph_index para delimitadores na fonte
# (confirmar que '(' tem glyph_id via ttf-parser)
grep -n "glyph_index" 03_infra/src/font_metrics.rs | head -5

# 5. Como MathDelimited é tratado actualmente no eval
grep -n "MathDelimited\|Delimited" 01_core/src/rules/eval.rs | head -10

# 6. Como MathDelimited é tratado no layout
grep -n "MathDelimited\|Delimited\|delimit" 01_core/src/rules/math/layout.rs | head -10

# 7. Tamanho actual dos delimitadores no layout
# (verificar se são simplesmente MathText com tamanho base)
grep -n "open\|close\|delim" 01_core/src/rules/math/layout.rs | head -15

# 8. Content::MathRoot — como o símbolo √ é emitido actualmente
grep -n "√\|radical" 01_core/src/rules/math/layout.rs | head -10
```

**Reportar o output antes de continuar.**

Se `ttf-parser` não expor `MathVariants` na versão instalada, verificar
versão mais recente. Se nenhuma versão suportar, o passo muda de âmbito
para fallback sem variantes (apenas escalamento linear do glifo base).

---

## Tarefa 1 — GlyphVariants em L1

Struct de domínio puro que representa as variantes de tamanho de um
glifo matemático.

```rust
// 01_core/src/entities/glyph_variants.rs

//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/glyph_variants.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-04-10

/// Uma variante de glifo com tamanho diferente.
///
/// `advance` é a medida na direcção de crescimento (altura para
/// variantes verticais, largura para horizontais), em design units.
#[derive(Debug, Clone)]
pub struct GlyphVariant {
    /// Identificador do glifo alternativo (glyph ID na fonte).
    pub glyph_id: u16,
    /// Medida de avanço na direcção de crescimento, em design units.
    pub advance: f64,
}

/// Variantes de tamanho para um glifo extensível.
///
/// Ordenadas por tamanho crescente. O `MathLayouter` selecciona a
/// primeira variante cuja `advance` (convertida para pt) seja >= à
/// altura mínima necessária.
#[derive(Debug, Clone, Default)]
pub struct GlyphVariants {
    pub variants: Vec<GlyphVariant>,
}

impl GlyphVariants {
    /// Selecciona a variante mais pequena com advance >= min_advance.
    ///
    /// `min_advance` em design units. Retorna o glyph_id da variante
    /// seleccionada, ou None se nenhuma variante for grande o suficiente.
    pub fn select(&self, min_advance: f64) -> Option<u16> {
        self.variants.iter()
            .find(|v| v.advance >= min_advance)
            .map(|v| v.glyph_id)
    }

    pub fn is_empty(&self) -> bool {
        self.variants.is_empty()
    }
}
```

Adicionar a `entities/mod.rs`:
```rust
pub mod glyph_variants;
```

---

## Tarefa 2 — Expandir FontMetrics com glyph_variants()

Adicionar método ao trait `FontMetrics` em L1. Default retorna
`GlyphVariants` vazio (sem variantes — fallback para glifo base).

```rust
// Em 01_core/src/rules/layout.rs — adicionar ao trait FontMetrics:

use crate::entities::glyph_variants::GlyphVariants;

/// Variantes de tamanho vertical para um glifo extensível.
///
/// Retorna as variantes ordenadas por tamanho crescente.
/// Default: sem variantes (fallback para glifo base).
///
/// `c` é o caractere base (ex: '(', ')', '√').
fn vertical_glyph_variants(&self, c: char) -> GlyphVariants {
    let _ = c;
    GlyphVariants::default()
}

/// Texto Unicode para um glyph_id específico.
///
/// Necessário para emitir o glifo variante como texto no FrameItem.
/// Retorna None se o glyph_id não tiver mapeamento Unicode reverso.
/// Default: None.
fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
    let _ = glyph_id;
    None
}
```

`FixedMetrics` herda os defaults — sem alteração.

---

## Tarefa 3 — FontBookMetrics implementa glyph_variants() em L3

```rust
// Em 03_infra/src/font_metrics.rs — impl FontMetrics for FontBookMetrics:

fn vertical_glyph_variants(&self, c: char) -> GlyphVariants {
    let glyph_id = match self.face.glyph_index(c) {
        Some(id) => id,
        None => return GlyphVariants::default(),
    };

    let math = match self.face.tables().math {
        Some(m) => m,
        None => return GlyphVariants::default(),
    };

    let variants_table = match math.variants {
        Some(v) => v,
        None => return GlyphVariants::default(),
    };

    // API exacta depende da versão de ttf-parser — confirmar no diagnóstico.
    // Esquema provável:
    //   variants_table.vert_glyph_construction(glyph_id)
    //     → Option<GlyphConstruction>
    //   construction.variants()
    //     → impl Iterator<Item = MathGlyphVariantRecord>
    //   record.variant_glyph → GlyphId
    //   record.advance_measurement → u16

    let construction = match variants_table
        .vert_glyph_construction(glyph_id)
    {
        Some(c) => c,
        None => return GlyphVariants::default(),
    };

    let variants: Vec<GlyphVariant> = construction
        .variants()
        .map(|record| GlyphVariant {
            glyph_id: record.variant_glyph.0,
            advance: record.advance_measurement as f64,
        })
        .collect();

    GlyphVariants { variants }
}

fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
    // Mapeamento reverso: glyph_id → codepoint.
    // ttf-parser não tem API directa para isto.
    // Solução: iterar sobre cmap subtables.
    // Para a maioria dos glifos math, o cmap contém o mapeamento.
    //
    // Alternativa pragmática: construir tabela reversa no from_bytes()
    // e armazená-la em FontBookMetrics.
    //
    // Para este passo, usar fallback: retornar None e o MathLayouter
    // usa o caractere base escalado quando não encontra mapeamento.
    //
    // TODO: implementar mapeamento reverso cmap no passo seguinte.
    let _ = glyph_id;
    None
}
```

**Nota sobre glyph_to_char**: o mapeamento reverso é necessário para
emitir o glifo variante como `FrameItem::Text`. Sem ele, o sistema
precisa de `FrameItem::Glyph { glyph_id, ... }` — uma variante nova
que o export PDF teria de tratar. Duas opções:

**Opção A — FrameItem::Glyph (preferida a longo prazo)**:
Variante nova em `FrameItem` que emite glifos por ID em vez de texto.
O export PDF usa `<glyph_id> Tj` em vez de texto Unicode.

**Opção B — Mapeamento reverso cmap**:
Construir tabela `glyph_id → char` em `from_bytes()` e usar em
`glyph_to_char()`. Funciona apenas para glifos com mapeamento Unicode.

**Para este passo, usar Opção B parcial**: se `glyph_to_char` retorna
`None`, o MathLayouter usa o caractere base com o tamanho original
(sem variante). O glifo extensível é uma melhoria progressiva.

---

## Tarefa 4 — MathLayouter usa variantes para delimitadores

Modificar o layout de `MathDelimited` para seleccionar delimitadores
com a altura correcta.

```rust
// Em MathLayouter — layout de MathDelimited (ou onde delimitadores são emitidos)

/// Selecciona e emite um delimitador com a altura mínima necessária.
///
/// Se a fonte tem variantes para o caractere, selecciona a variante
/// mais pequena que cobre `min_height`. Se não há variantes (FixedMetrics
/// ou fonte sem tabela MATH), emite o caractere base.
fn layout_stretchy_delimiter(
    &self,
    c: char,
    min_height: f64,  // em design units
) -> MathBox {
    let variants = self.metrics.vertical_glyph_variants(c);

    let (text, style) = if let Some(glyph_id) = variants.select(min_height) {
        // Variante encontrada — tentar converter para char
        if let Some(variant_char) = self.metrics.glyph_to_char(glyph_id) {
            (variant_char.to_string(), TextStyle::regular(self.size))
        } else {
            // Sem mapeamento reverso — usar caractere base
            (c.to_string(), TextStyle::regular(self.size))
        }
    } else {
        // Sem variantes — usar caractere base
        (c.to_string(), TextStyle::regular(self.size))
    };

    let width = self.metrics.advance(&text, self.size);
    let (ascender, _) = self.metrics.vertical_metrics(self.size);

    MathBox {
        width,
        ascent: ascender,
        descent: Pt::ZERO,
        items: vec![FrameItem::Text {
            pos: Point::ZERO,
            text: text.into(),
            style,
        }],
    }
}
```

Integrar em `layout_delimited` (onde `MathDelimited` é processado):

```rust
// Onde actualmente os delimitadores são emitidos como MathText simples,
// substituir por layout_stretchy_delimiter:

fn layout_delimited(
    &mut self,
    open: &str,
    body: &Content,
    close: &str,
) -> MathBox {
    // 1. Layout do corpo
    let body_box = self.layout_node(body);

    // 2. Altura mínima do delimitador = altura total do corpo
    let body_height = body_box.ascent + body_box.descent;
    let min_height_du = body_height.val() * self.constants.upem / self.size.val();

    // 3. Delimitadores extensíveis
    let open_char = open.chars().next().unwrap_or('(');
    let close_char = close.chars().next().unwrap_or(')');
    let open_box = self.layout_stretchy_delimiter(open_char, min_height_du);
    let close_box = self.layout_stretchy_delimiter(close_char, min_height_du);

    // 4. Concatenar: open + body + close
    hconcat(vec![open_box, body_box, close_box])
}
```

---

## Tarefa 5 — MathLayouter usa variantes para radical

Em `layout_root`, o símbolo `√` actualmente é emitido com tamanho fixo.
Com variantes, seleccionar a variante que cobre a altura do radicando.

```rust
// Em layout_root — substituir emissão fixa de √:

// ANTES:
// let radical_item = FrameItem::Text {
//     pos: Point::ZERO,
//     text: "√".into(),
//     style: TextStyle::regular(base_size),
// };

// DEPOIS:
let rad_height = rad_box.ascent + rad_box.descent + gap + line_thickness;
let min_height_du = rad_height.val() * self.constants.upem / self.size.val();
let radical_box = self.layout_stretchy_delimiter('√', min_height_du);
```

---

## Tarefa 6 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_glyph_variants {
    use super::*;
    use crate::entities::glyph_variants::{GlyphVariant, GlyphVariants};

    #[test]
    fn select_variante_minima() {
        let v = GlyphVariants {
            variants: vec![
                GlyphVariant { glyph_id: 100, advance: 500.0 },
                GlyphVariant { glyph_id: 101, advance: 800.0 },
                GlyphVariant { glyph_id: 102, advance: 1200.0 },
            ],
        };
        // Pedir 600 → primeira variante >= 600 é 101 (800)
        assert_eq!(v.select(600.0), Some(101));
    }

    #[test]
    fn select_variante_exacta() {
        let v = GlyphVariants {
            variants: vec![
                GlyphVariant { glyph_id: 100, advance: 500.0 },
                GlyphVariant { glyph_id: 101, advance: 800.0 },
            ],
        };
        assert_eq!(v.select(500.0), Some(100));
    }

    #[test]
    fn select_nenhuma_suficiente() {
        let v = GlyphVariants {
            variants: vec![
                GlyphVariant { glyph_id: 100, advance: 500.0 },
            ],
        };
        assert_eq!(v.select(1000.0), None);
    }

    #[test]
    fn select_vazio() {
        let v = GlyphVariants::default();
        assert_eq!(v.select(100.0), None);
    }

    #[test]
    fn fixed_metrics_sem_variantes() {
        let m = FixedMetrics;
        let v = m.vertical_glyph_variants('(');
        assert!(v.is_empty());
    }

    #[test]
    fn fixed_metrics_glyph_to_char_none() {
        let m = FixedMetrics;
        assert_eq!(m.glyph_to_char(42), None);
    }

    // ── Layout — regressão ───────────────────────────────

    #[test]
    fn layout_delimited_funciona_sem_variantes() {
        // Com FixedMetrics (sem variantes), delimitadores continuam a funcionar
        let doc = layout_test("$[a + b]$");
        let text = doc.plain_text();
        assert!(text.contains('a'), "corpo: {}", text);
    }

    #[test]
    fn layout_sqrt_funciona_sem_variantes() {
        let doc = layout_test("$sqrt(x)$");
        let text = doc.plain_text();
        assert!(text.contains('√'), "radical: {}", text);
    }

    #[test]
    fn layout_frac_em_delimitadores() {
        // Fracção dentro de parênteses — altura grande
        let doc = layout_test("$(frac(a, b))$");
        let text = doc.plain_text();
        assert!(text.contains('a'), "numerador: {}", text);
        assert!(text.contains('b'), "denominador: {}", text);
    }
}
```

### Testes em L3 (com fonte MATH)

```rust
#[cfg(test)]
mod tests_stretchy_font {
    use super::*;

    #[test]
    #[ignore = "requer fonte com tabela MATH em tests/fixtures/"]
    fn font_math_tem_variantes_para_parenteses() {
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/stix-two-math.otf")
        ).expect("fixture necessária");

        let m = FontBookMetrics::from_bytes(&data).unwrap();
        let v = m.vertical_glyph_variants('(');
        assert!(!v.is_empty(),
            "STIX Two Math deve ter variantes verticais para '('");
        // Variantes devem estar ordenadas por tamanho crescente
        for w in v.variants.windows(2) {
            assert!(w[0].advance <= w[1].advance,
                "variantes devem ser ordenadas: {} <= {}",
                w[0].advance, w[1].advance);
        }
    }

    #[test]
    #[ignore = "requer fonte com tabela MATH"]
    fn font_math_tem_variantes_para_radical() {
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/stix-two-math.otf")
        ).unwrap();
        let m = FontBookMetrics::from_bytes(&data).unwrap();
        let v = m.vertical_glyph_variants('√');
        assert!(!v.is_empty(),
            "STIX Two Math deve ter variantes verticais para '√'");
    }

    #[test]
    #[ignore = "requer fonte com tabela MATH"]
    fn font_sem_math_retorna_vazio() {
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/liberation-sans-regular.ttf")
        ).unwrap();
        let m = FontBookMetrics::from_bytes(&data).unwrap();
        let v = m.vertical_glyph_variants('(');
        assert!(v.is_empty(),
            "Liberation Sans não tem tabela MATH — sem variantes");
    }

    #[test]
    fn pdf_delimited_com_frac() {
        let pdf = compile_to_pdf("$(frac(a, b))$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_sqrt_expressao_alta() {
        let pdf = compile_to_pdf("$sqrt(frac(a, b))$");
        assert!(!pdf.is_empty());
    }
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:
- [ ] `GlyphVariants` e `GlyphVariant` existem em L1
- [ ] `FontMetrics::vertical_glyph_variants()` tem default vazio
- [ ] `FontMetrics::glyph_to_char()` tem default None
- [ ] `FontBookMetrics` lê variantes da tabela MathVariants quando disponível
- [ ] `FontBookMetrics` retorna vazio quando fonte não tem tabela MATH
- [ ] `MathLayouter::layout_stretchy_delimiter()` selecciona variante por altura
- [ ] Delimitadores em `MathDelimited` usam `layout_stretchy_delimiter`
- [ ] Radical em `layout_root` usa `layout_stretchy_delimiter` para `√`
- [ ] Com `FixedMetrics`, tudo funciona como antes (fallback para glifo base)
- [ ] Testes existentes de frac/attach/sqrt/delimited continuam a passar
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- API exacta de `MathVariants` no `ttf-parser` (nomes dos métodos)
- Se `vert_glyph_construction` existe ou o acesso é diferente
- Quantas variantes `(` tem na fonte de teste (se fixture disponível)

**Da implementação:**
- Se `glyph_to_char` foi implementado (mapeamento reverso) ou ficou como None
- Se foi necessário `FrameItem::Glyph` ou se `FrameItem::Text` foi suficiente
- Se os delimitadores mudam visualmente no PDF com fonte MATH

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 43:**
- **GO — GlyphAssembly (montagem por partes)**: variantes funcionam;
  Passo 43 implementa montagem com extensores para delimitadores maiores
  que qualquer variante disponível
- **GO — Kern matemático**: variantes funcionam; Passo 43 implementa
  `MathKernInfo` para ajuste fino de espaçamento entre símbolos adjacentes
- **GO — FrameItem::Glyph**: se `glyph_to_char` retornou None para
  variantes, Passo 43 adiciona `FrameItem::Glyph` ao export PDF
- **NO-GO — ttf-parser sem MathVariants**: tabela não acessível;
  avaliar crate alternativa ou implementar escalamento linear
