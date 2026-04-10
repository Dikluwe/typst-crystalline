# Passo 43 — FrameItem::Glyph e GlyphAssembly

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — `FrameItem::Text`, `FrameItem::Line`
- `01_core/src/entities/glyph_variants.rs` — `GlyphVariants`, `GlyphVariant`
- `01_core/src/rules/layout.rs` — trait `FontMetrics`, `vertical_glyph_variants()`, `glyph_to_char()`
- `01_core/src/rules/math/layout.rs` — `layout_stretchy_delimiter`, `layout_delimited`, `layout_root`
- `03_infra/src/font_metrics.rs` — `FontBookMetrics`, leitura de `math_table.variants`
- `03_infra/src/export.rs` — tratamento actual de `FrameItem::Text` e `FrameItem::Line` no PDF

Pré-condição: `cargo test` — 492 L1 + 68 L3 + 50 parity, zero violations.

---

## Contexto

No Passo 42, `layout_stretchy_delimiter` selecciona a variante de glifo
correcta por altura via `GlyphVariants::select()`. No entanto, como
`glyph_to_char` retorna `None` (mapeamento reverso de cmap não implementado),
o renderizador recai sempre no caractere base — a variante seleccionada é
descartada. O PDF resultante usa sempre o glifo de tamanho base,
independentemente do conteúdo que o delimitador envolve.

Este passo resolve isso em dois sub-âmbitos:

**Sub-âmbito A — FrameItem::Glyph**: nova variante de `FrameItem` que
transporta um `glyph_id` numérico directamente, sem depender de
mapeamento Unicode. O export PDF escreve-o como matriz hexadecimal
(`<XXXX> Tj`). Isso desbloqueia a renderização das variantes seleccionadas.

**Sub-âmbito B — GlyphAssembly**: quando a altura exigida excede todas as
variantes disponíveis, a tabela MATH define como construir o glifo por partes
(peças fixas + extensores repetíveis). Este passo lê essa estrutura em L3,
passa-a para L1 via struct de domínio, e o `MathLayouter` monta os
delimitadores empilhando as peças verticalmente.

Não há ADR nova — ADR-0019 já autoriza `ttf-parser` em L3; ADR-0029 autoriza
`Arc` em L1; o padrão de isolamento L1/L3 via structs simples está estabelecido.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Confirmar variantes actuais de FrameItem em L1
grep -n "FrameItem\|enum Frame" 01_core/src/entities/layout_types.rs | head -20

# 2. Como export.rs trata FrameItem::Text actualmente
grep -n "FrameItem\|Text\|Line" 03_infra/src/export.rs | head -30

# 3. API de GlyphAssembly no ttf-parser 0.25.1
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "GlyphAssembly\|assembly\|GlyphPart\|GlyphConstruction\|extender" {} | head -30

# 4. Campos de GlyphConstruction — tem campo assembly?
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "struct GlyphConstruction\|pub assembly\|pub variants" {} | head -20

# 5. Campos de GlyphPart / peças da assembly
find ~/.cargo/registry/src -path "*/ttf-parser-*/src" -type d 2>/dev/null \
  | head -1 | xargs -I{} grep -rn "start_connector\|end_connector\|full_advance\|part_flags\|is_extender" {} | head -20

# 6. Como layout_stretchy_delimiter usa glyph_to_char actualmente
grep -n "glyph_to_char\|glyph_id\|select" 01_core/src/rules/math/layout.rs | head -20

# 7. Confirmar que CIDFont em export.rs usa glyph IDs ou codepoints Unicode
grep -n "CIDFont\|ToUnicode\|Tj\|show\|glyph" 03_infra/src/export.rs | head -30

# 8. Confirmar upem disponível em FontBookMetrics para conversão du→pt
grep -n "upem\|units_per_em" 03_infra/src/font_metrics.rs | head -10
```

**Reportar o output antes de continuar.**

Se `GlyphAssembly` não estiver acessível na versão instalada de `ttf-parser`,
o sub-âmbito B fica fora do passo; implementar apenas `FrameItem::Glyph`.
Se a estrutura de `GlyphConstruction.assembly` existir mas com campos
diferentes dos esperados, adaptar os nomes no diagnóstico antes de codificar.

---

## Tarefa 1 — FrameItem::Glyph em L1

Adicionar variante ao enum `FrameItem` em `01_core/src/entities/layout_types.rs`.

```rust
// Adicionar ao enum FrameItem:

/// Glifo renderizado directamente por ID, sem mapeamento Unicode.
///
/// Usado para variantes de tamanho matemático onde `glyph_to_char`
/// retorna `None`. O export PDF escreve o ID como matriz hexadecimal.
///
/// `pos`: posição final do glifo (coordenadas de página), já com
///        qualquer deslocamento vertical calculado pelo `MathLayouter`.
///        O `MathLayouter` é responsável por calcular `pos.y` final
///        antes de emitir este item — não existe `y_offset` separado.
/// `glyph_id`: índice do glifo na fonte (u16, índice CIDFont).
/// `x_advance`: largura horizontal do glifo em pt.
/// `size`: corpo tipográfico em pt (usado para seleccionar o glifo correcto).
Glyph {
    pos: Point,
    glyph_id: u16,
    x_advance: Pt,
    size: Pt,
},
```

Nenhuma outra entidade de L1 muda nesta tarefa.

---

## Tarefa 2 — GlyphAssemblyParts em L1

Nova struct de domínio puro para representar a montagem por partes.
Criar em `01_core/src/entities/glyph_variants.rs`, junto a `GlyphVariants`.

```rust
/// Uma peça individual de um delimitador montado por partes.
///
/// `glyph_id`: índice do glifo da peça.
/// `start_connector`: sobreposição mínima com a peça anterior (design units).
/// `end_connector`: sobreposição mínima com a peça seguinte (design units).
/// `full_advance`: avanço total da peça sem sobreposição (design units).
/// `is_extender`: se true, esta peça pode ser repetida para preencher altura.
#[derive(Debug, Clone)]
pub struct GlyphPart {
    pub glyph_id: u16,
    pub start_connector: u16,
    pub end_connector: u16,
    pub full_advance: u16,
    pub is_extender: bool,
}

/// Montagem por partes para um delimitador extensível.
///
/// Usado quando a altura exigida excede todas as variantes em `GlyphVariants`.
/// As peças são empilhadas verticalmente (bottom → top).
#[derive(Debug, Clone, Default)]
pub struct GlyphAssembly {
    pub parts: Vec<GlyphPart>,
}

impl GlyphAssembly {
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Calcula a altura total mínima desta assembly (sem repetição de extensores).
    ///
    /// Soma `full_advance` de todas as peças, em design units.
    /// Esta é a altura base antes de repetir extensores.
    pub fn min_advance(&self) -> f64 {
        self.parts.iter().map(|p| p.full_advance as f64).sum()
    }
}
```

---

## Tarefa 3 — FontMetrics::vertical_glyph_assembly() em L1

Adicionar método ao trait `FontMetrics` em `01_core/src/rules/layout.rs`.
Default retorna `GlyphAssembly` vazia.

```rust
use crate::entities::glyph_variants::GlyphAssembly;

/// Montagem por partes para um glifo extensível.
///
/// Retorna as peças ordenadas bottom→top para montagem vertical.
/// Default: sem assembly (fallback para variante máxima disponível).
///
/// `c` é o caractere base (ex: '(', ')', '{', '√').
fn vertical_glyph_assembly(&self, c: char) -> GlyphAssembly {
    let _ = c;
    GlyphAssembly::default()
}
```

`FixedMetrics` herda o default — sem alteração.

---

## Tarefa 4 — FontBookMetrics implementa glyph_assembly() em L3

```rust
// Em 03_infra/src/font_metrics.rs — impl FontMetrics for FontBookMetrics:

fn vertical_glyph_assembly(&self, c: char) -> GlyphAssembly {
    let glyph_id = match self.face.glyph_index(c) {
        Some(id) => id,
        None => return GlyphAssembly::default(),
    };

    let math = match self.face.tables().math {
        Some(m) => m,
        None => return GlyphAssembly::default(),
    };

    let variants_table = match math.variants {
        Some(v) => v,
        None => return GlyphAssembly::default(),
    };

    let construction = match variants_table.vertical_constructions.get(glyph_id) {
        Some(c) => c,
        None => return GlyphAssembly::default(),
    };

    // `construction.assembly` pode ser Option<GlyphAssembly> no ttf-parser.
    // Confirmar campo exacto no diagnóstico (Tarefa 0).
    let assembly = match construction.assembly {
        Some(a) => a,
        None => return GlyphAssembly::default(),
    };

    let parts: Vec<GlyphPart> = assembly.parts
        .into_iter()
        .map(|p| GlyphPart {
            glyph_id: p.glyph_id.0,
            start_connector: p.start_connector_length,
            end_connector: p.end_connector_length,
            full_advance: p.full_advance,
            is_extender: p.part_flags.is_extender(),
        })
        .collect();

    GlyphAssembly { parts }
}
```

**Nota**: os nomes dos campos (`start_connector_length`, `end_connector_length`,
`part_flags`, `is_extender()`) devem ser confirmados no diagnóstico. Se
diferirem, adaptar antes de codificar.

---

## Tarefa 5 — layout_stretchy_delimiter usa FrameItem::Glyph

Em `01_core/src/rules/math/layout.rs`, modificar `layout_stretchy_delimiter`
para emitir `FrameItem::Glyph` quando uma variante é seleccionada, em vez
de `FrameItem::Text` com o char base.

```rust
// layout_stretchy_delimiter — lógica actual (simplificada):
//   1. Obter GlyphVariants via self.metrics.vertical_glyph_variants(c)
//   2. Seleccionar variante com select(min_advance_du)
//   3. Se variante encontrada: actualmente emite FrameItem::Text com char base
//   4. Se não encontrada: emite FrameItem::Text com char base

// Nova lógica:
//   3a. Se variante encontrada E glyph_to_char retorna Some(ch):
//         emitir FrameItem::Text { text: ch, ... }  (comportamento actual)
//   3b. Se variante encontrada E glyph_to_char retorna None:
//         emitir FrameItem::Glyph { glyph_id, x_advance, y_offset, size }
//         x_advance: variant.advance * (size / upem)  — converter du→pt
//   4.  Se nenhuma variante: tentar GlyphAssembly (Tarefa 6)
//   5.  Se nem variantes nem assembly: fallback para FrameItem::Text com char base
```

A conversão de `advance` de design units para pt segue o padrão do Passo 41:
`advance_pt = size * (advance_du / upem)`.

O `upem` está disponível via `self.constants.upem` (já existente).

---

## Tarefa 6 — layout_assembly: montar delimitador por partes

Novo método privado no `MathLayouter`. Chamado por `layout_stretchy_delimiter`
quando nenhuma variante é suficiente mas existe `GlyphAssembly`.

```rust
/// Monta um delimitador vertical a partir de peças extensíveis.
///
/// `target_advance`: altura mínima necessária em design units.
/// `assembly`: peças da tabela MATH, ordenadas bottom→top.
///
/// Estratégia de montagem:
/// 1. Calcular a soma dos `full_advance` das peças não-extensor (fixas).
/// 2. Calcular altura restante = target_advance - soma_fixas.
/// 3. Se altura_restante <= 0: usar cada peça uma vez, sem repetição.
/// 4. Se extenders existem: calcular quantas repetições de cada extensor
///    são necessárias para cobrir a altura_restante.
///    Cada repetição adiciona `full_advance - min(start_connector, end_connector)`
///    à altura total (sobreposição mínima nas junções).
/// 5. Empilhar todos os glifos (fixos + extensores repetidos) de baixo para cima,
///    emitindo `FrameItem::Glyph` para cada peça.
///    Regra de posicionamento Y entre peças consecutivas:
///      `pos.y[i] = pos.y[i-1] + (full_advance[i-1] - overlap) * (size / upem)`
///    onde `overlap = min(end_connector[i-1], start_connector[i])`,
///    e a conversão `* (size / upem)` transforma design units em pt.
///    A primeira peça (bottom) parte de `pos.y = 0` no referencial do MathBox.
///    O `MathLayouter` é responsável por calcular `pos.y` final de cada peça
///    antes de emitir — não existe campo de deslocamento separado no FrameItem.
/// 6. Retornar um `MathBox` com a altura total montada.
///
/// Se assembly estiver vazia: retornar MathBox com FrameItem::Text do char base.
fn layout_assembly(
    &self,
    c: char,
    assembly: GlyphAssembly,
    target_advance: f64,
) -> MathBox {
    // implementação
}
```

**Simplificação permitida neste passo**: para o cálculo de repetições de
extensores, usar uma única iteração de cada extensor se a altura total
mínima for suficiente. A optimização de repetições múltiplas fica para
Passo 44+.

---

## Tarefa 7 — export.rs trata FrameItem::Glyph

Em `03_infra/src/export.rs`, adicionar caso para `FrameItem::Glyph` no
loop de renderização de frames.

```rust
// No match sobre FrameItem — adicionar:
FrameItem::Glyph { pos, glyph_id, x_advance: _, size } => {
    // 1. Posicionar: y invertido (origem PDF = canto inferior esquerdo)
    //    pos.y já é a posição final, calculada pelo MathLayouter.
    let pdf_x = pos.x.val();
    let pdf_y = page_height - pos.y.val();

    // 2. Definir tamanho da fonte
    // "Tf" já foi emitido para o font resource — reutilizar ou emitir novo
    // Emitir: "<size> 0 0 <size> <x> <y> Tm"

    // 3. Escrever glifo como matriz hexadecimal de 2 bytes (big-endian):
    //    format!("<{:04X}> Tj", glyph_id)
    //
    // Isto requer que o CIDFont já esteja registado no stream da página.
    // Como o font resource é partilhado (CIDFont Type2 com todas as
    // glyphs da fonte), qualquer glyph_id válido pode ser escrito.

    // Sequência PDF:
    // BT
    // /F1 <size> Tf
    // <x> <y> Td
    // <<glyph_id_hex>> Tj
    // ET
}
```

**Restrição**: `FrameItem::Glyph` usa o mesmo font resource (`/F1`) que os
`FrameItem::Text` existentes — não é necessário um novo recurso de fonte.
O `glyph_id` corresponde directamente ao índice CID no CIDFont Type2 (que
usa glyph indices, não codepoints Unicode). Verificar no diagnóstico se o
CIDFont em `export.rs` já usa este esquema.

**Se o CIDFont actual usa codepoints Unicode** (não glyph indices), é
necessário uma abordagem diferente. Reportar no diagnóstico e discutir
antes de implementar.

**Limitação conhecida — extracção de texto quebrada (DEBT-9)**:
Glifos emitidos via `<XXXX> Tj` sem entrada correspondente na tabela
`ToUnicode` do PDF tornam-se opacos para leitores de PDF: o utilizador
não consegue copiar e colar o delimitador — o caractere é ignorado ou
produz conteúdo inválido na área de transferência. Esta limitação é
aceite neste passo. A solução de longo prazo exige actualizar o mapa
`ToUnicode` dinamicamente com o `char` original do delimitador
(ex: `'('` → `U+0028`) para cada `glyph_id` de variante emitido.
Registar como DEBT-9 no estado de migração e resolver em Passo 45+.

---

## Tarefa 8 — Testes

### Testes unitários em L1

```rust
#[cfg(test)]
mod tests_glyph_assembly {
    use super::*;
    use crate::entities::glyph_variants::{GlyphAssembly, GlyphPart};

    fn make_part(full_advance: u16, is_extender: bool) -> GlyphPart {
        GlyphPart {
            glyph_id: 0,
            start_connector: 50,
            end_connector: 50,
            full_advance,
            is_extender,
        }
    }

    #[test]
    fn assembly_min_advance_soma_full_advances() {
        let a = GlyphAssembly {
            parts: vec![
                make_part(400, false),
                make_part(200, true),
                make_part(400, false),
            ],
        };
        assert_eq!(a.min_advance(), 1000.0);
    }

    #[test]
    fn assembly_vazia_min_advance_zero() {
        assert_eq!(GlyphAssembly::default().min_advance(), 0.0);
    }

    #[test]
    fn assembly_vazia_is_empty() {
        assert!(GlyphAssembly::default().is_empty());
    }

    // ── Layout com FixedMetrics ──────────────────────────────────────────

    #[test]
    fn layout_stretchy_sem_variantes_sem_assembly_usa_char_base() {
        // FixedMetrics não tem variantes nem assembly — deve usar char base
        let doc = layout_test("$(frac(a, b))$");
        let text = doc.plain_text();
        assert!(text.contains('('), "delimitador base: {}", text);
    }

    #[test]
    fn layout_frame_contem_glyph_ou_text_para_delimitadores() {
        // Independentemente do caminho, o frame deve ter items para os delimitadores
        let doc = layout_test("$(x)$");
        // Deve existir pelo menos um FrameItem (Text ou Glyph) por página
        assert!(!doc.pages.is_empty());
        let page = &doc.pages[0];
        assert!(!page.items.is_empty());
    }

    // ── Regressão ────────────────────────────────────────────────────────

    #[test]
    fn frac_dentro_de_delimitadores_nao_regride() {
        let doc = layout_test("$(frac(a, b))$");
        let text = doc.plain_text();
        assert!(text.contains('a'));
        assert!(text.contains('b'));
    }

    #[test]
    fn sqrt_nao_regride() {
        let doc = layout_test("$sqrt(x)$");
        let text = doc.plain_text();
        assert!(text.contains('√') || text.contains('x'));
    }

    #[test]
    fn attach_nao_regride() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }
}
```

### Testes em L3

```rust
#[cfg(test)]
mod tests_glyph_export {

    #[test]
    fn pdf_com_delimitadores_nao_vazio() {
        let pdf = compile_to_pdf("$(frac(a, b))$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_com_sqrt_frac_nao_vazio() {
        let pdf = compile_to_pdf("$sqrt(frac(a, b))$");
        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_com_delimitadores_contem_bt_et() {
        // BT/ET são marcadores obrigatórios de bloco de texto em PDF
        let pdf = compile_to_pdf("$(x + 1)$");
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("BT"), "PDF deve ter BT");
        assert!(s.contains("ET"), "PDF deve ter ET");
    }

    #[test]
    #[ignore = "requer fonte com tabela MATH em tests/fixtures/"]
    fn font_math_assembly_para_parentese() {
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/stix-two-math.otf")
        ).expect("fixture necessária");
        let m = FontBookMetrics::from_bytes(&data).unwrap();
        let a = m.vertical_glyph_assembly('(');
        // STIX Two Math deve ter GlyphAssembly para '('
        // (pode ser vazia se apenas variantes estiverem definidas — não falhar)
        let _ = a; // apenas confirmar que não pânica
    }

    #[test]
    fn fixed_metrics_assembly_vazia() {
        let m = FixedMetrics;
        let a = m.vertical_glyph_assembly('(');
        assert!(a.is_empty());
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

- [ ] `FrameItem::Glyph { pos, glyph_id, x_advance, size }` existe em L1 (sem `y_offset` — posição final calculada pelo `MathLayouter` em `pos.y`)
- [ ] `GlyphPart` e `GlyphAssembly` existem em `entities/glyph_variants.rs`
- [ ] `FontMetrics::vertical_glyph_assembly()` tem default vazio
- [ ] `FontBookMetrics` lê `GlyphAssembly` da tabela MATH quando disponível
- [ ] `FontBookMetrics` retorna assembly vazia quando fonte não tem tabela MATH ou não tem assembly para o glifo
- [ ] `layout_stretchy_delimiter` emite `FrameItem::Glyph` quando variante encontrada e `glyph_to_char` retorna `None`
- [ ] `layout_stretchy_delimiter` chama `layout_assembly` quando nenhuma variante é suficiente
- [ ] `layout_assembly` monta `MathBox` com peças empilhadas verticalmente
- [ ] `export.rs` trata `FrameItem::Glyph` produzindo `<XXXX> Tj` no stream PDF
- [ ] Limitação de extracção de texto (`ToUnicode`) documentada como DEBT-9
- [ ] Com `FixedMetrics`, nenhum caminho novo é activado — comportamento idêntico ao Passo 42
- [ ] Todos os testes de regressão de frac/attach/sqrt/delimited passam
- [ ] `ttf-parser` não aparece em `01_core/Cargo.toml`
- [ ] Zero violations no linter

---

## Ao terminar, reportar

**Do diagnóstico:**
- API exacta de `GlyphConstruction.assembly` no `ttf-parser` (nome do campo, tipo)
- Nomes dos campos de `GlyphPart` no `ttf-parser` (`part_flags`, `is_extender`, etc.)
- Se o CIDFont em `export.rs` usa glyph indices ou codepoints Unicode (impacto em `FrameItem::Glyph`)
- Se `GlyphAssembly` estava acessível (sub-âmbito B executado ou não)

**Da implementação:**
- Se `FrameItem::Glyph` foi suficiente para renderizar variantes no PDF
- Se `layout_assembly` foi implementado ou ficou como stub (`GlyphAssembly::default()`)
- Se foi necessário mudar a estrutura do CIDFont em `export.rs`

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 44:**
- **GO — Kern matemático (MathKernInfo)**: se variantes e assembly funcionam, Passo 44 implementa `MathKernInfo` para ajuste de espaçamento entre símbolos adjacentes via tabela MATH
- **GO — Baselines matemáticas**: eixo de equação (`AxisHeight`) para alinhar fracções e delimitadores ao centro da linha de texto
- **NO-GO — CIDFont usa codepoints**: se o CIDFont actual não suporta glyph indices directos, é necessário reestruturar o export antes de continuar com variantes
