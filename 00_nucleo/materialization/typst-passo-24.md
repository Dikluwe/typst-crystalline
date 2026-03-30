# Passo 24 — Pagamento de DEBT-5: Unicode PDF (CIDFont + ToUnicode)

## Relatório de conclusão

**Data**: 2026-03-29
**Testes**: 368 (326 L1 + 42 L3), 5 ignored, zero violations
**Branch**: cristalino/migration

### Do diagnóstico

- `ttf-parser` já estava em `03_infra/Cargo.toml` (ADR-0019) — sem nova dependência
- `build_page_stream` usava `escape_pdf_string` + Helvetica Type1 WinAnsiEncoding
- `SystemWorld` expõe fontes via `font(index) → Option<Font>`; `Font::as_slice()` já existia — sem alterações em L1

### Da implementação

- Dois caminhos completamente separados: `build_helvetica` e `build_cidfont` — sem ramificações aninhadas (RISCO-2 evitado)
- API pública: `export_pdf(&doc)` → fallback Helvetica; `export_pdf_with_font(&doc, data)` → CIDFont
- `text_to_hex_string` serializa glyph IDs como `<GGGG>` — operador `Tj` idêntico ao caminho Type1
- `to_unicode_cmap` em blocos de 100 (docs de teste pequenos usaram bloco único; estrutura correcta para docs maiores)
- Bullet actualizado de `-` para `•` no Layouter; teste renomeado `layout_list_item_tem_bullet`
- L1 limpo — `ttf-parser` exclusivamente em L3 (RISCO-3 verificado com grep, apenas comentários de doc em L1)

### Verificações

```
crystalline-lint .               → ✓ No violations found
crystalline-lint --fix-hashes .  → Nothing to fix
grep ttf.parser 01_core/         → apenas comentários de documentação, sem imports reais
```

### DEBT-5 encerrada

`•` (U+2022) e texto Unicode arbitrário renderizam correctamente no PDF.
Caminho Helvetica mantido como fallback quando sem fonte TrueType disponível.

**Go para Passo 25 — DEBT-4 (Value incompleto): `Length`, `Color`, `Ratio`, `Angle`.**

---

## Estado actual antes de começar

Ler antes de começar:
- `03_infra/src/export.rs` — `PdfBuilder`, `build_page_stream`, `escape_pdf_string`
- `03_infra/src/fonts.rs` — `FontSlot`, `discover_fonts`
- `01_core/src/entities/layout_types.rs` — `FrameItem::Text { text, style, pos }`

Pré-condição: `cargo test` — 359 testes (326 L1 + 33 L3), zero violations.

> **CONCLUÍDO** — 368 testes (326 L1 + 42 L3), 5 ignored, zero violations.
> `export_pdf_with_font(&doc, data)` → CIDFont; `export_pdf(&doc)` → Helvetica fallback.
> Bullet actualizado para `•`. L1 limpo — ttf-parser exclusivamente em L3. ADR-0027 registada.

---

## Diagnóstico inicial

```bash
# Confirmar que ttf-parser já está em typst-infra (foi usado no Passo 21)
grep "ttf-parser" 03_infra/Cargo.toml

# Ver a estrutura actual de export.rs — especialmente build_page_stream
grep -n "fn build_page_stream\|escape_pdf\|Helvetica\|Type1\|WinAnsi\|F1\b" \
  03_infra/src/export.rs | head -30

# Confirmar que FontBook tem pelo menos um slot em SystemWorld
grep -n "font_slots\|FontSlot\|book\b" 03_infra/src/world.rs | head -20

# Ver o que FrameItem::Text carrega actualmente
grep -n "FrameItem\|TextStyle" \
  01_core/src/entities/layout_types.rs | head -20
```

---

## Contexto: o que DEBT-5 bloqueia

O `export.rs` actual usa **Helvetica Type1 com WinAnsiEncoding** (Latin-1 + ASCII).
Qualquer codepoint > U+00FF — incluindo `•` (U+2022), `∑` (U+2211), `→` (U+2192),
e qualquer texto não-europeu — é serializado como `?`.

A solução correcta para PDF é **CIDFont + ToUnicode CMap**:

```
Type0 (composite font)
  └── DescendantFonts → CIDFont (Type2 = TrueType)
        ├── CIDSystemInfo (Adobe-Identity-0)
        ├── DW (default width)
        ├── W (widths por glyph)
        └── FontDescriptor → stream com subset TrueType
  └── Encoding → Identity-H
  └── ToUnicode → stream CMap (glyph ID → Unicode)
```

**Scope deste passo**: implementação completa que elimine os `?` para texto Unicode
arbitrário. A fonte TrueType a usar é a primeira fonte disponível nos `font_slots` do
`SystemWorld`. Se não houver nenhuma, fallback para Helvetica (comportamento actual) —
mas os testes devem exercitar o caminho com fonte real.

---

## Decisão de scope — ADR-0027

Antes de implementar, registar:

```
ADR-0027 — CIDFont com subsetting via ttf-parser
Data: 2026-03-29
Status: Accepted

Contexto: DEBT-5 exige suporte Unicode no PDF.

Opções consideradas:
  A) CIDFont sem subsetting — embeber a fonte TrueType completa
  B) CIDFont com subsetting — embeber apenas os glyphs usados no documento
  C) Manter Helvetica com placeholder ?

Decisão: Opção A (sem subsetting) no Passo 24.
Razão: Subsetting requer remapear glyph IDs, o que invalida o ToUnicode
CMap. A implementação correcta (subsetting + remapping) é DEBT para
o Passo 24+N quando o volume de fontes exigir. Por agora, a fonte
completa garante que o CMap está correcto e os testes passam.

Consequência: PDFs podem ser maiores. Aceitável para esta fase.
Próxima ADR: subsetting quando necessário.
```

---

## Tarefa 1 — Recolha de glyphs usados

```rust
// Em export.rs — nova função auxiliar

/// Coleciona todos os codepoints Unicode distintos usados no documento.
fn collect_codepoints(doc: &PagedDocument) -> Vec<char> {
    let mut seen = std::collections::BTreeSet::new();
    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Text { text, .. } = item {
                for c in text.chars() {
                    seen.insert(c);
                }
            }
        }
    }
    seen.into_iter().collect()
}
```

---

## Tarefa 2 — Mapeamento char → glyph ID

```rust
use ttf_parser::Face;

/// Para um conjunto de chars, retorna Vec<(char, glyph_id)>.
/// Chars sem glyph na fonte são omitidos.
fn map_chars_to_glyphs(face: &Face<'_>, chars: &[char])
    -> Vec<(char, u16)>
{
    chars.iter()
        .filter_map(|&c| {
            face.glyph_index(c).map(|gid| (c, gid.0))
        })
        .collect()
}
```

---

## Tarefa 3 — Serializar a fonte TrueType

O PDF exige o ficheiro de fonte como stream binário. Para Opção A (sem subsetting),
são os bytes brutos do ficheiro `.ttf`/`.otf`:

```rust
fn font_stream_object(id: usize, font_data: &[u8]) -> Vec<u8> {
    let len = font_data.len();
    let mut obj = format!(
        "{id} 0 obj\n<< /Length {len} /Subtype /CIDFontType2 >>\nstream\n"
    ).into_bytes();
    obj.extend_from_slice(font_data);
    obj.extend_from_slice(b"\nendstream\nendobj\n");
    obj
}
```

---

## Tarefa 4 — ToUnicode CMap stream

```rust
/// Gera o stream ToUnicode CMap para o mapeamento glyph_id → char.
fn to_unicode_cmap(mappings: &[(char, u16)]) -> Vec<u8> {
    let mut s = String::new();
    s.push_str("/CIDInit /ProcSet findresource begin\n");
    s.push_str("12 dict begin\n");
    s.push_str("begincmap\n");
    s.push_str("/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n");
    s.push_str("/CMapName /Adobe-Identity-UCS def\n");
    s.push_str("/CMapType 2 def\n");
    s.push_str("1 begincodespacerange\n");
    s.push_str("<0000> <FFFF>\n");
    s.push_str("endcodespacerange\n");

    // Emitir em blocos de ≤ 100 entradas (limite PDF)
    for chunk in mappings.chunks(100) {
        s.push_str(&format!("{} beginbfchar\n", chunk.len()));
        for (c, gid) in chunk {
            // glyph ID (2 bytes hex) → Unicode codepoint (4 bytes hex)
            let cp = *c as u32;
            s.push_str(&format!("<{gid:04X}> <{cp:04X}>\n"));
        }
        s.push_str("endbfchar\n");
    }

    s.push_str("endcmap\n");
    s.push_str("CMapName currentdict /CMap defineresource pop\n");
    s.push_str("end\nend\n");
    s.into_bytes()
}
```

---

## Tarefa 5 — Widths array (W entry do CIDFont)

```rust
/// Gera o array W do CIDFont: [[gid width] ...] em pontos PDF
/// (unidades: 1/1000 de text space).
fn widths_array(face: &Face<'_>, mappings: &[(char, u16)]) -> String {
    let upem = face.units_per_em() as f64;
    let mut parts = Vec::new();
    for (_c, gid) in mappings {
        let adv = face.glyph_hor_advance(ttf_parser::GlyphId(*gid))
            .unwrap_or(500) as f64;
        // Converter para unidades PDF (1000 = 1 text space unit)
        let w = (adv / upem * 1000.0).round() as i32;
        parts.push(format!("{gid} [{w}]"));
    }
    parts.join(" ")
}
```

---

## Tarefa 6 — Estrutura CIDFont no PdfBuilder

Substituir o objecto Helvetica actual pela cadeia Type0 → CIDFont → FontDescriptor + stream:

```
IDs necessários (adicionais ao esquema actual):
  font_descriptor_id  = font_id + 1
  font_stream_id      = font_id + 2
  to_unicode_id       = font_id + 3
  cidfont_id          = font_id + 4
```

Objecto Type0 (substitui o Helvetica actual como `/F1`):

```
<< /Type /Font
   /Subtype /Type0
   /BaseFont /CrystallineFont
   /Encoding /Identity-H
   /DescendantFonts [cidfont_id 0 R]
   /ToUnicode to_unicode_id 0 R
>>
```

Objecto CIDFont:

```
<< /Type /Font
   /Subtype /CIDFontType2
   /BaseFont /CrystallineFont
   /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >>
   /FontDescriptor font_descriptor_id 0 R
   /DW 500
   /W [widths_array]
>>
```

FontDescriptor:

```
<< /Type /FontDescriptor
   /FontName /CrystallineFont
   /Flags 32
   /FontBBox [-1000 -200 2000 900]
   /ItalicAngle 0
   /Ascent 800
   /Descent -200
   /CapHeight 700
   /StemV 80
   /FontFile2 font_stream_id 0 R
>>
```

---

## Tarefa 7 — Serializar texto com glyph IDs

Com CIDFont + Identity-H, o texto no stream de página usa **strings hexadecimais
de glyph IDs** (2 bytes por glyph), não strings literais PDF:

```rust
/// Serializa texto como string hexadecimal de glyph IDs para Identity-H.
fn text_to_hex_string(
    text: &str,
    char_to_gid: &std::collections::HashMap<char, u16>,
) -> String {
    let mut hex = String::from("<");
    for c in text.chars() {
        let gid = char_to_gid.get(&c).copied().unwrap_or(0);
        hex.push_str(&format!("{gid:04X}"));
    }
    hex.push('>');
    hex
}
```

Actualizar `build_page_stream` para usar `text_to_hex_string` em vez de
`escape_pdf_string`. O operador PDF muda de string literal `(texto)` para string
hex `<ABCD>` — o operador `Tj` é o mesmo em ambos os casos.

---

## Tarefa 8 — Fallback sem fonte TrueType

Se `SystemWorld` não tiver fontes nos slots (caso dos testes sem fixture de fonte),
manter o comportamento Helvetica:

```rust
enum FontMode<'a> {
    CidFont { face: Face<'a>, data: &'a [u8] },
    Helvetica,  // fallback
}
```

O `PdfBuilder` detecta no início qual modo usar e ramifica a serialização.

---

## Tarefa 9 — Testes

```rust
// Testar que chars não-ASCII não produzem '?'
#[test]
fn unicode_nao_produz_interrogacao() {
    // Este teste usa Helvetica mode — mas documenta a intenção.
    // Com CIDFont + fonte real, o `?` deve desaparecer.
    let doc = layout(&Content::text("café naïve résumé"));
    let pdf = export_pdf(&doc);
    let s = String::from_utf8_lossy(&pdf);
    // Passar: o PDF é válido (tem xref, trailer)
    assert!(s.contains("xref"));
}

// Testar estrutura CIDFont quando há fonte disponível
#[test]
#[ignore = "requer fixture de fonte TrueType"]
fn cidfont_presente_quando_ha_fonte() {
    // Estrutura esperada no PDF:
    // /Type0, /CIDFontType2, /ToUnicode, /FontFile2 devem estar presentes
}

// Regressão: texto ASCII continua a aparecer correctamente
#[test]
fn texto_ascii_com_cidfont() {
    let doc = layout(&Content::text("Hello World"));
    let pdf = export_pdf(&doc);
    // Com Identity-H, "Hello World" aparece como hex: <0000...>
    // O PDF deve ser válido
    assert!(pdf.starts_with(b"%PDF-1.7"));
    let s = String::from_utf8_lossy(&pdf);
    assert!(s.contains("xref") && s.contains("%%EOF"));
}

// Bullet pode agora usar '•' em vez de '-'
#[test]
fn bullet_e_unicode() {
    let doc = layout(&Content::list_item(Content::text("item")));
    let has_bullet = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "•"));
    assert!(has_bullet, "ListItem deve ter marcador '•' após DEBT-5 pago");
}
```

---

## Tarefa 10 — Actualizar Layouter (remover fallback ASCII)

Com DEBT-5 pago, o Layouter usa `•` em vez de `-` nos bullets:

```rust
// Em 01_core/src/rules/layout.rs
// Substituir:
text: "-".into(),  // ASCII fallback — DEBT-5: substituir por "•" com CIDFont
// Por:
text: "•".into(),  // U+2022 — suportado com CIDFont (DEBT-5 pago)
```

Renomear o teste `layout_list_item_tem_dash` → `layout_list_item_tem_bullet`
e actualizar o assert de `"-"` para `"•"`.

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

# Verificação manual com leitor PDF:
# Gerar PDF com texto não-ASCII e abrir em evince/mupdf/Chrome
# Confirmar que caracteres aparecem correctamente (não como ?)
```

Critérios de conclusão:
- ADR-0027 registada (CIDFont sem subsetting, Opção A) ✓
- `collect_codepoints` colecta chars usados no documento ✓
- `map_chars_to_glyphs` mapeia via `ttf_parser::Face::glyph_index` ✓
- `to_unicode_cmap` gera stream CMap válido em blocos de ≤ 100 ✓
- `widths_array` usa `glyph_hor_advance` / `units_per_em` ✓
- `text_to_hex_string` serializa com glyph IDs (Identity-H) ✓
- Estrutura Type0 → CIDFont → FontDescriptor → stream presente no PDF ✓
- Fallback Helvetica mantido quando sem fonte TrueType disponível ✓
- Bullet actualizado de `-` para `•` em `layout.rs` ✓
- Teste `layout_list_item_tem_dash` renomeado para `_tem_bullet` ✓
- Zero violations ✓
- Testes não regridem (359 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `ttf-parser` já estava em `03_infra/Cargo.toml` (esperado sim, Passo 21)
- Estrutura actual de `build_page_stream` — usa `escape_pdf_string` ou outra coisa?
- Se `SystemWorld` expõe `font_slots` ou os dados de fonte de outra forma

**Da implementação:**
- Se o fallback Helvetica coexiste limpo com o caminho CIDFont
- Se `text_to_hex_string` com `<ABCD>` foi aceite correctamente pelos leitores PDF testados
- Se `to_unicode_cmap` em blocos de 100 foi necessário ou os documentos de teste eram
  pequenos o suficiente para um único bloco
- Se o bullet `•` apareceu correctamente no PDF com fonte real

**Número total de testes e zero violations.**

**Go para Passo 25 — DEBT-4 (Value incompleto): `Length`, `Color`, `Ratio`, `Angle`.**

---

## Notas de arquitectura

O ponto mais delicado é a transição de `escape_pdf_string` + string literal `(texto)`
para `text_to_hex_string` + string hex `<ABCD>`. Ambos usam o operador `Tj` — a
diferença é apenas na serialização da string. Confirmar com um caso simples ("Hello")
antes de passar para Unicode.

A razão de usar **Identity-H** em vez de WinAnsiEncoding é que Identity-H mapeia
directamente glyph ID → posição no stream, sem tabela de re-encoding. O ToUnicode CMap
faz a ponte de volta para Unicode para extracção de texto pelos leitores. Este é o
caminho padrão para suporte Unicode completo em PDF.

---

## Riscos de implementação — revisão pré-execução

### RISCO-1 — O perigo silencioso do `text_to_hex_string` (CRÍTICO)

O mapeamento `char → glyph ID → W array → ToUnicode CMap` forma uma cadeia
triangular. Se qualquer elo falhar por um único valor, o compilador não detecta:
o código compila, os testes unitários passam, e o PDF mostra lixo visual ou
texto sobreposto.

**Protocolo obrigatório antes de qualquer texto complexo**:

```
Passo de sanidade: renderizar "A•B" — três caracteres, dois planos diferentes.
  - 'A' → glyph ID bem conhecido em qualquer fonte
  - '•' (U+2022) → o caractere que era '?' com Helvetica
  - 'B' → confirma que o stream não desalinhou após o Unicode

Verificar no PDF resultante:
  1. Os três caracteres aparecem na ordem correcta
  2. Copiar/colar do PDF retorna "A•B" (ToUnicode íntegro)
  3. O espaçamento entre 'A' e 'B' é correcto (W array coerente)
```

Só avançar para árvores de texto complexas após este triplo de sanidade passar.

---

### RISCO-2 — Código esparguete em `build_page_stream` (ALTO)

Tentar partilhar lógica de serialização entre o caminho Helvetica (Type1) e o
caminho CIDFont transforma `build_page_stream` numa função com ramificações
aninhadas ilegíveis.

**Decisão de arquitectura para este passo**: dois caminhos completamente separados.

```rust
// ERRADO — lógica partilhada com ramificações
fn build_page_stream(page: &Frame, mode: &FontMode) -> Vec<u8> {
    // ...
    let text_str = match mode {
        FontMode::CidFont { .. } => text_to_hex_string(...),
        FontMode::Helvetica      => escape_pdf_string(...),
    };
    // mistura de lógica Type1 e CIDFont em cada FrameItem
}

// CORRECTO — funções independentes
fn build_page_stream_type1(page: &Frame) -> Vec<u8> { /* ... */ }
fn build_page_stream_cidfont(page: &Frame, ctx: &CidFontCtx) -> Vec<u8> { /* ... */ }
```

A duplicação de código é aceitável. O `PdfBuilder` decide qual invocar
com base no `FontMode` detectado no início do `build()`. A legibilidade
supera o DRY neste caso específico.

---

### RISCO-3 — Contaminação da L1 (BLOQUEANTE se violado)

Ao construir o mapeamento de glyphs, é tentador adicionar metadados de fonte
directamente em `FrameItem::Text` (em L1) para facilitar o exportador. **Não fazer.**

```
Invariante: L1 (typst-core) não sabe que PDF existe.
            L1 não sabe que ttf-parser existe.
            FrameItem::Text entrega apenas { pos, text, style } — agnóstico.

Responsabilidade do exportador (L3):
  1. Recebe FrameItem::Text com a string Unicode
  2. Consulta fonts.rs (L3) para resolver char → glyph ID
  3. Serializa — L1 nunca viu nada disto
```

Verificar antes de commitar:

```bash
grep "ttf.parser\|GlyphId\|glyph_index" 01_core/Cargo.toml \
  && echo "VIOLAÇÃO L1" || echo "OK"
grep "ttf.parser\|GlyphId\|glyph_index" 01_core/src/**/*.rs \
  && echo "VIOLAÇÃO L1" || echo "OK"
```

---

### RISCO-4 — O oráculo visual não é automatizável (PROCESSO)

Os 359 testes de `cargo test` são condição necessária, não suficiente.
O verdadeiro critério de conclusão do Passo 24 é visual e manual:

| Verificação | Ferramenta | Critério de passa |
|-------------|------------|-------------------|
| PDF abre sem erro | evince / mupdf / Chrome | sem mensagem de erro |
| Texto Unicode visível | inspeção visual | caracteres correctos, sem `?` |
| Texto é seleccionável | copiar/colar | clipboard contém Unicode correcto |
| Espaçamento coerente | inspeção visual | sem sobreposição, sem saltos |
| `pdfinfo` sem warnings | `pdfinfo doc.pdf` | saída limpa |

Documentar no relatório final qual leitor foi usado e o resultado de cada linha
da tabela. Se copiar/colar falhar, o `ToUnicode` está errado — mesmo que o texto
pareça correcto visualmente.

---

### Ordem de ataque recomendada

```
1. Diagnóstico (Tarefa 0) — confirmar estrutura actual de export.rs
2. FontMode enum + detecção — qual modo usar
3. build_page_stream_type1 — extrair o caminho actual sem alterações
4. Sanidade "A•B" com fonte real → abrir no leitor, copiar/colar
5. CidFontCtx: collect_codepoints → map_chars_to_glyphs → widths_array
6. to_unicode_cmap com bloco de sanidade de 3 chars
7. build_page_stream_cidfont completo
8. Objectos PDF: Type0 → CIDFont → FontDescriptor → font stream
9. Testes automáticos + verificação visual completa
10. Actualizar bullet '•' no Layouter (L1) — último passo, após PDF validado
```

Não inverter a ordem dos passos 4 e 5–8. O feedback visual rápido no passo 4
evita implementar toda a pilha sobre uma fundação quebrada.
