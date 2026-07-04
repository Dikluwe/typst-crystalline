# Paridade de Produção — Passo 560

**Data:** 2026-07-04  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino: `./target/release/typst`
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `pdftoppm`, `mutool draw`, `mutool extract`, `fontTools`

---

## 1. Objetivo

P559 encontrou, de passagem, que PDFs com texto árabe/hebraico gerados pelo
cristalino não renderizavam em poppler (`pdftoppm`) nem mupdf (`mutool draw`)
— ambos falhavam a carregar a fonte embutida `AAAAAA+CrystallineFont`. Este
passo investiga a causa e corrige.

---

## 2. Sonda

### 2.1 Caso mínimo

Documento RTL mínimo:

```typst
#set text(lang: "ar")
مرحبا
```

Antes da correção:

- `mutool draw`: `ignored error when loading embedded font; attempting to load system font`
- `pdftoppm`: `Couldn't create a font for 'AAAAAA+CrystallineFont'`

Caso latino equivalente (`Hello`) renderizava sem erros.

### 2.2 Especificidade do problema

| Caso | Antes da correção | Nota |
|---|---|---|
| Latino | OK | Fonte TrueType (`Liberation Serif`) |
| Árabe mínimo | Falha | Fallback CFF (`FreeMono`) |
| Hebraico | Falha | Fallback CFF (`FreeMono`) |
| Tashkeel | Falha | Fallback CFF (`FreeMono`) |

O problema é específico do caminho CFF usado para RTL, não geral à exportação.

### 2.3 Estrutura da fonte extraída

`mutool extract` do PDF RTL antes da correção devolveu `font-0007.ttf`
(183 KB), um contêiner OpenType/SFNT com tabela `CFF`. O PDF declarava:

```pdf
/Subtype /CIDFontType2
/FontFile2
```

Isto é inválido para fontes CFF — leitores de PDF esperam `CIDFontType0` +
`FontFile3` + stream `CIDFontType0C` (programa CFF puro).

Comparação com o vanilla para o mesmo texto árabe:

```pdf
/Subtype /CIDFontType0
/FontFile3
<< /Length 979 /Filter /FlateDecode /Subtype /CIDFontType0C >>
```

E `mutool extract` do vanilla devolveu `font-0010.cid` (CFF puro, 1,1 KB).

---

## 3. Causa

`03_infra/src/export/builder.rs` (caminhos `build_cidfont` e
`build_multifont`) hardcoded sempre:

- `/Subtype /CIDFontType2` no dicionário descendente `/Font`
- `/FontFile2` no `/FontDescriptor`
- `/Subtype /CIDFontType2` no stream de fonte

Isto só é válido para fontes TrueType (`glyf`). Quando o fallback árabe/hebraico
escolhe uma fonte CFF/OpenType (`FreeMono`), o PDF fica estruturalmente
inválido e leitores rejeitam a fonte embutida.

---

## 4. Implementação

Ficheiros alterados:

| Ficheiro | Alteração |
|---|---|
| `00_nucleo/prompts/infra/export/builder.md` | §P560: descritor PDF conforme tipo de fonte (TrueType vs CFF) |
| `00_nucleo/prompts/infra/export/font_subset.md` | CFF não retorna `None`; descritor PDF tratado no builder |
| `03_infra/src/export/builder.rs` | `font_embedding_data` detecta CFF, extrai tabela `CFF`, emite `CIDFontType0` + `FontFile3` + `CIDFontType0C` |
| `03_infra/src/export/subset.rs` | `@prompt-hash` actualizado |
| `03_infra/src/export/tests.rs` | Testes `cidfont_presente_quando_ha_fonte` e `p560_fonte_cff_usa_cidfont_type0` |

Lógica em `builder.rs`:

```rust
fn cff_table_data(font_data: &[u8]) -> Option<&[u8]> {
    let face = Face::parse(font_data, 0).ok()?;
    if face.tables().cff.is_some() {
        face.table_data(ttf_parser::Tag::from_bytes(b"CFF "))
    } else {
        None
    }
}

fn font_embedding_data(font_data: &[u8]) -> (&str, &str, &str, &[u8]) {
    if let Some(cff) = cff_table_data(font_data) {
        return ("/CIDFontType0", "/FontFile3", "CIDFontType0C", cff);
    }
    ("/CIDFontType2", "/FontFile2", "CIDFontType2", font_data)
}
```

- TrueType: emite a fonte SFNT completa como antes.
- CFF/OpenType: extrai apenas a tabela `CFF` e emite como `CIDFontType0C`.

---

## 5. Resultados

### 5.1 Renderização após correção

| Caso | `pdftoppm` | `mutool draw` | Imagem |
|---|---|---|---|
| Árabe mínimo | OK | OK | `مرحبا` visível |
| Hebraico | OK | OK | texto hebraico visível |
| Tashkeel (`مُحَمَّد`) | OK | OK | diacríticos visíveis |
| Latino | OK | OK | sem regressão |

### 5.2 Estrutura PDF após correção (RTL)

```pdf
/Subtype /CIDFontType0
/FontFile3 8 0 R
<< /Length 183068 /Subtype /CIDFontType0C >>
stream
<CFF bytes>
endstream
```

`mutool extract` agora devolve `font-0007.otf` parseável, e os leitores
poppler/mupdf carregam a fonte sem erro.

### 5.3 Testes

```bash
cargo test --workspace
# todos passam

cargo test -p typst-infra p560
# test export::tests::p560_fonte_cff_usa_cidfont_type0 ... ok

cargo test -p typst-infra cidfont_presente_quando_ha_fonte
# test export::tests::cidfont_presente_quando_ha_fonte ... ok

crystalline-lint .
# ✅ 0 violations
```

---

## 6. Decisão

- Fontes CFF/OpenType usam `/CIDFontType0` + `/FontFile3 /Subtype /CIDFontType0C`
  com o programa CFF puro extraído da tabela `CFF`.
- Fontes TrueType mantêm o comportamento existente `/CIDFontType2` + `/FontFile2`.
- A detecção é feita em runtime via `ttf_parser::Face::tables().cff`, sem
  depender de extensão de ficheiro.

---

## 7. Critério de fecho do passo

- [x] Sonda completa: problema confirmado específico de RTL/CFF.
- [x] Causa localizada em `03_infra/src/export/builder.rs`.
- [x] Corrigido e testado em `pdftoppm` e `mutool draw`.
- [x] Corpus RTL completo (`arabic_basic.typ`, `hebrew_basic.typ`, tashkeel)
      testado com imagem.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p560.md`.
- [x] L0s `builder.md` e `font_subset.md` actualizados e hashes sincronizados.

---

## 8. Nota sobre escopo

A direção do texto RTL (`dir: rtl`) continua não suportada em `set text`
(emitido como warning). A ordem visual dos caracteres árabes no PDF gerado
aparece da esquerda para a direita porque o bidi layout ainda não está
implementado. Este passo resolve apenas o problema de **renderização da fonte
embutida**, não o de layout bidireccional.
