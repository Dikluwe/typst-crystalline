# Passo 45 — Extracção de Texto para Glifos Matemáticos (Resolução do DEBT-9)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — `FrameItem::Glyph` (não deve ser alterado)
- `01_core/src/rules/layout.rs` — `trait FontMetrics` com o método `glyph_to_char(glyph_id: u16) -> Option<char>`
- `03_infra/src/font_metrics.rs` — `FontBookMetrics::from_bytes` e a implementação actual de `glyph_to_char` (que retorna `None`)
- `03_infra/src/export.rs` — geração do PDF, especificamente a secção que constrói o stream `/ToUnicode` e o `cmap`

Pré-condição: `cargo test` — 518 L1 + 75 L3 + 50 parity, zero violations.

---

## Contexto

A emissão de delimitadores matemáticos extensíveis via `FrameItem::Glyph` (Passo 43) injecta directamente o `glyph_id` no PDF (`<XXXX> Tj`). Como estes glifos alternativos não possuem mapeamento na tabela `cmap` da fonte, o PDF resultante não mapeia estes IDs de volta para caracteres Unicode. O texto torna-se impossível de copiar (DEBT-9).

**Arquitectura da Solução:**
O L1 permanece intacto (pureza geométrica mantida). A resolução ocorre integralmente no L3 através de um **Dicionário Reverso Preemptivo**.
1. Durante o parse da fonte (`FontBookMetrics::from_bytes`), o sistema iterará sobre os caracteres matemáticos base conhecidos.
2. Utilizando a tabela MATH (já acessível via `ttf-parser`), o L3 extrairá os IDs de todas as variantes de tamanho e peças de montagem (assembly).
3. Estes IDs serão guardados num `HashMap<u16, char>` interno ao `FontBookMetrics`.
4. O método `glyph_to_char(glyph_id)` passará a consultar este mapa.
5. O `export.rs` usará este método para povoar o bloco `bfchar` na geração do `ToUnicode` do PDF.

---

## Diagnósticos obrigatórios antes de qualquer código

```bash
# 1. Confirmar assinatura de glyph_to_char no trait e na implementação L3
grep -n "fn glyph_to_char" 01_core/src/rules/layout.rs 03_infra/src/font_metrics.rs

# 2. Localizar onde FontBookMetrics armazena o seu estado interno
grep -A 10 "pub struct FontBookMetrics" 03_infra/src/font_metrics.rs

# 3. Localizar onde export.rs constrói o CMap / ToUnicode
grep -rn "ToUnicode\|beginbfchar\|cmap" 03_infra/src/export.rs | head -20

# 4. Confirmar que vertical_glyph_variants e vertical_glyph_assembly existem e são chamáveis no from_bytes
grep -n "fn vertical_glyph_variants\|fn vertical_glyph_assembly" 03_infra/src/font_metrics.rs
```

**Reportar o output antes de continuar.**

---

## Tarefa 1 — O Mapa Reverso em FontBookMetrics (L3)

Adicionar o dicionário de mapeamento ao estado do `FontBookMetrics` em `03_infra/src/font_metrics.rs`.

`from_bytes` constrói a instância antes de `self` existir — não é possível chamar
métodos do trait dentro dele. A extracção de variantes e assembly deve ser feita
por **funções livres privadas** que recebem `&ttf_parser::Face` directamente.

```rust
// ── Funções auxiliares privadas (fora de qualquer impl) ──────────────────

/// Retorna os glyph_ids de todas as variantes de tamanho vertical para `c`.
fn extract_variants(face: &ttf_parser::Face, c: char) -> Vec<u16> {
    let glyph_id = match face.glyph_index(c) {
        Some(id) => id,
        None => return Vec::new(),
    };
    let variants = match face.tables().math
        .and_then(|m| m.variants)
    {
        Some(v) => v,
        None => return Vec::new(),
    };
    let construction = match variants.vertical_constructions.get(glyph_id) {
        Some(c) => c,
        None => return Vec::new(),
    };
    construction.variants
        .into_iter()
        .map(|v| v.variant_glyph.0)
        .collect()
}

/// Retorna as peças de assembly vertical para `c` como `(glyph_id, is_extender)`.
fn extract_assembly_parts(face: &ttf_parser::Face, c: char) -> Vec<(u16, bool)> {
    let glyph_id = match face.glyph_index(c) {
        Some(id) => id,
        None => return Vec::new(),
    };
    let variants = match face.tables().math
        .and_then(|m| m.variants)
    {
        Some(v) => v,
        None => return Vec::new(),
    };
    let construction = match variants.vertical_constructions.get(glyph_id) {
        Some(c) => c,
        None => return Vec::new(),
    };
    let assembly = match construction.assembly {
        Some(a) => a,
        None => return Vec::new(),
    };
    assembly.parts
        .into_iter()
        .map(|p| (p.glyph_id.0, p.part_flags.is_extender()))
        .collect()
}

// ── struct e from_bytes ───────────────────────────────────────────────────

// 1. Adicionar o campo à struct
pub struct FontBookMetrics<'a> {
    // ... campos existentes ...
    /// Mapa reverso preemptivo: glyph_id → char original
    glyph_to_unicode: std::collections::HashMap<u16, char>,
}

// 2. Modificar from_bytes para construir o mapa usando as funções livres
pub fn from_bytes(data: &'a [u8]) -> Result<Self, FontError> {
    // ... inicialização existente do face ...

    let mut glyph_to_unicode = std::collections::HashMap::new();

    let stretchy_bases = ['(', ')', '[', ']', '{', '}', '|', '√'];

    for &base_char in &stretchy_bases {
        for glyph_id in extract_variants(&face, base_char) {
            glyph_to_unicode.entry(glyph_id).or_insert(base_char);
        }
        for (glyph_id, is_extender) in extract_assembly_parts(&face, base_char) {
            let mapped_char = if is_extender { '|' } else { base_char };
            glyph_to_unicode.entry(glyph_id).or_insert(mapped_char);
        }
    }

    // Self { ..., glyph_to_unicode }
}

// 3. Implementar o trait
fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
    self.glyph_to_unicode.get(&glyph_id).copied()
}
```

**Nota**: os nomes dos campos de `GlyphPart` no `ttf-parser` (`part_flags`,
`is_extender()`, `glyph_id`) devem ser confirmados no diagnóstico — são os
mesmos usados no Passo 43, pelo que não deve haver surpresas.

*Não altere o L1. A estrutura de `FrameItem::Glyph` mantém-se apenas com a geometria.*

---

## Tarefa 2 — Actualizar ToUnicode no export.rs (L3)

Em `03_infra/src/export.rs`, durante a geração do PDF, o exportador precisa incluir os glifos usados na tabela `ToUnicode`.

Actualmente, o exportador provavelmente extrai caracteres de `FrameItem::Text`.
Tem de garantir que, ao percorrer as páginas para registar as fontes e construir os subconjuntos (ou dicionários CID), processe também os `FrameItem::Glyph`:

```rust
// Ao processar os items da página para recolher metadados de fonte:
match item {
    FrameItem::Text { text, .. } => {
        // Lógica existente para texto
    }
    FrameItem::Glyph { glyph_id, .. } => {
        // Consultar o caractere original
        if let Some(c) = font_metrics.glyph_to_char(*glyph_id) {
            // Registar este glyph_id associado ao char 'c' para a geração do bfchar.
            // A estrutura de dados depende de como o export.rs gere o subsetting do CIDFont.
            register_glyph_for_cmap(*glyph_id, c);
        }
    }
    // ...
}
```

Na função que escreve o stream `/ToUnicode` (o CMap do PDF):
Garantir que a secção `bfchar` inclui as entradas registadas dos glifos:

```text
% Exemplo do formato PDF esperado no stream:
1 beginbfchar
<00A2> <0028> % glyph_id (Hex) -> Unicode (Hex)
endbfchar
```

*(Certifique-se de que a formatação hexadecimal do `glyph_id` e do `char as u32` tem a formatação padronizada correcta do seu exportador).*

---

## Tarefa 3 — Testes em L3

Adicionar testes em `03_infra/src/integration_tests.rs` e `03_infra/src/font_metrics.rs`.

```rust
#[cfg(test)]
mod tests_tounicode {
    use super::*;

    #[test]
    #[ignore = "requer fonte com tabela MATH"]
    fn dicionario_reverso_preenchido_no_parse() {
        let data = std::fs::read("tests/fixtures/stix-two-math.otf").unwrap();
        let m = FontBookMetrics::from_bytes(&data).unwrap();
        
        // Testar se obtivemos o glyph_id original de um caractere base
        let base_glyph = m.face.glyph_index('(').unwrap().0;
        
        // A fonte STIX Two Math tem de gerar variantes.
        // Assumindo que a variante maior tem um ID diferente:
        let variants = m.vertical_glyph_variants('(');
        assert!(!variants.is_empty());
        
        let largest_variant_id = variants.variants.last().unwrap().glyph_id;
        
        // O dicionário reverso tem de mapear esse glifo variante de volta para '('
        assert_eq!(m.glyph_to_char(largest_variant_id), Some('('));
    }

    #[test]
    #[ignore = "requer compilação com fonte MATH explícita — Helvetica não emite FrameItem::Glyph"]
    fn pdf_tounicode_contem_mapeamento_de_delimitador() {
        // compile_to_pdf usa Helvetica como fallback — sem tabela MATH, o MathLayouter
        // emite FrameItem::Text em vez de FrameItem::Glyph, e o bloco ToUnicode
        // não contém mapeamentos de variantes. O teste deve usar a fixture explícita.
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/stix-two-math.otf")
        ).expect("fixture necessária");
        let pdf = compile_pdf_with_font("$(frac(a, b))$", &data);
        let pdf_str = String::from_utf8_lossy(&pdf);
        // U+0028 é '(', U+0029 é ')'
        assert!(pdf_str.contains("<0028>"), "CMap não mapeou parêntese de abertura");
        assert!(pdf_str.contains("<0029>"), "CMap não mapeou parêntese de fecho");
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
# ✓ No violations found
```

Critérios de conclusão:
- [ ] `FrameItem::Glyph` **não** foi modificado no L1.
- [ ] `FontBookMetrics` possui o `HashMap` populado durante o `from_bytes`.
- [ ] `glyph_to_char` retorna o mapeamento correcto em vez de `None`.
- [ ] Extensores partilhados mapeiam para `|` sem sobrescrever caracteres base (`or_insert`).
- [ ] `export.rs` integra os mapeamentos de glifos no bloco `bfchar` do stream `/ToUnicode`.
- [ ] Os testes asseguram que o PDF contém o código hex `<0028>` para parênteses grandes.
- [ ] Zero violations no linter.
