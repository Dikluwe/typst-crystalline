# Diagnóstico de infra PDF para font embedding — Passo 140A

**Data**: 2026-04-24
**ADR alvo**: **ADR-0055** — "Font consumer via pipeline
CIDFont existente".
**Motivação**: Fase C do roadmap DEBT-1 precisa de consumer
real para `text.font`. Este diagnóstico mapeia o que existe e
o que falta para escrever roadmap realista.

**Conclusão antecipada**: **a maior parte da infra já está
materializada** (CIDFont embedding, ttf-parser, font discovery,
CLI --font-path). Gap é **wiring** `TextStyle.font` ao pipeline
existente — trabalho muito menor do que Roadmap 135
anticipou.

---

## 1. L1 — Domínio font runtime

### 1.1 `FontBook` em `01_core/src/entities/font_book.rs`

```rust
pub struct FontBook { infos: Vec<FontInfo> }

impl FontBook {
    pub fn new() -> Self;
    pub fn push(&mut self, info: FontInfo);
    pub fn select(&self, family: &str, variant: &FontVariant) -> Option<usize>;
    pub fn select_family<'a>(&'a self, family: &'a str)
        -> impl Iterator<Item = usize> + 'a;
    pub fn infos(&self) -> &[FontInfo];
    pub fn len(&self) -> usize;
}
```

- `select` devolve **índice** na Vec infos.
- `FontInfo` contém `family: EcoString + variant: FontVariant +
  flags`.
- `FontVariant` agrega `FontWeight + FontStyle + FontStretch`.

### 1.2 Tipo `Font` runtime em `entities/world_types.rs:30`

```rust
pub struct Font(Vec<u8>);
```

Raw TTF/OTF bytes. Produzido por `FontSlot::get()` em L3.

### 1.3 Trait `World::font`

```rust
// contracts/world.rs
fn font(&self, index: usize) -> Option<Font>;
```

Trait method — implementado por `SystemWorld` em L3.

### 1.4 Resolução `StyleDelta.font → bytes` (gap)

```
StyleDelta.font: Option<FontList>
  → FontList (Vec<FontFamily>)
    → each FontFamily.name
      → FontBook::select(name, variant) → Option<usize>  ✓ existe
        → World::font(index) → Option<Font(bytes)>        ✓ existe
          → Font bytes para embedding                     ✓ existe
```

**Todos os passos têm implementação**. Gap é **no pipeline**: o
caminho do layout/export actual **ignora** `TextStyle.font`.

---

## 2. L3 — Descoberta de fontes

### 2.1 `discover_fonts` em `03_infra/src/fonts.rs`

```rust
pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot>
```

- Varre directórios recursivamente.
- Detecta extensões `.ttf`, `.otf`, `.ttc`, `.otc`.
- Suporta TrueType Collections (`.ttc`) — um slot por face.
- Lazy loading: `FontSlot::get() -> Option<Font>`.

### 2.2 CLI `--font-path` (Passos 122-123)

```bash
typst input.typ --font-path /a/b --font-path /c/d
# Ou via env var:
TYPST_FONT_PATHS=/a/b:/c/d typst input.typ
```

- `Args.font_paths: Vec<PathBuf>` em L2.
- `RunIntent.font_paths` propagado para L4.
- L4 `main.rs`: `discover_fonts(&font_paths)` + `SystemWorld::with_fonts`.
- Path inválido → silent skip (L3).

### 2.3 Fontes embutidas

**Zero fontes embutidas**. Cristalino não usa `include_bytes!`
para Computer Modern ou similar. Sem infra de default fonts —
utilizador precisa de `--font-path` explícito OU fallback
Helvetica.

### 2.4 `SystemWorld::with_fonts`

```rust
pub fn with_fonts(mut self, font_slots: Vec<FontSlot>) -> Self {
    self.font_book  = crate::fonts::build_font_book(&font_slots);
    self.font_slots = font_slots;
    self
}
```

Builder que:
1. Popula `FontBook` com metadata (via `font_info_from_bytes`).
2. Armazena slots para lookup posterior.

Resultado: `SystemWorld` com FontBook + `font(index)` funcional.

---

## 3. L3 — PDF embedding

### 3.1 Resource dict actual

`03_infra/src/export.rs:340` define `PdfBuilder` manual (sem
crate PDF). Dois caminhos:

**Path 1: `build_helvetica`** (linha 366, usado por default):
```
/Font << /F1 font_f1 0 R /F2 font_f2 0 R /F3 font_f3 0 R >>
```
F1/F2/F3 Helvetica Type1, **sem embedding** (standard 14 PDF
fonts).

**Path 2: `build_cidfont`** (linha 423, **ativado quando
font_data é provided**):
```
/Font << /F1 {font_id} 0 R >>
```
onde font_id aponta para Type0 CIDFont com **FontFile2 embutido**.

### 3.2 Capacidade de embedding — **MASSIVA DESCOBERTA**

`build_cidfont` **já implementa todo o CIDFont pipeline**:

- Type0 font com Identity-H encoding.
- CIDFontType2 com CIDSystemInfo.
- FontDescriptor com bounding box + weights.
- **FontFile2 stream** — bytes TTF embebidos no PDF.
- **Widths array** (`/W [...]`) — larguras por glyph.
- **ToUnicode CMap** — para texto copiável.
- `collect_codepoints(doc)` + `map_chars_to_glyphs(face, chars)`.
- `build_math_glyph_reverse_map` para glyphs variantes
  (math + DEBT-9 ToUnicode).

**TODO o embedding já está pronto**. Falta apenas **chamar** o
path certo.

### 3.3 PDF writer

**Manual**. `PdfBuilder::new().add(id, content)` + serialização
manual de object table, xref, trailer. Zero crates PDF (nem
`pdf-writer`, `printpdf`, `lopdf`).

Consequência: adicionar embedding de **múltiplas fontes**
requer estender `build_cidfont` para iterar sobre mais que 1
face — não é trivial mas é incremental.

### 3.4 Ponto de entrada público

```rust
// export.rs:23
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8>;

// export.rs:30
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8>;
```

- `export_pdf` → `build_helvetica` (fallback).
- `export_pdf_with_font` → `build_cidfont(font_data)` (embedding).

**Pipeline actual em `infra/pipeline.rs:78`**: usa `export_pdf`
(Helvetica). Wiring `export_pdf_with_font` exige:
1. Extrair font bytes do `TextStyle.font` via FontBook + World.
2. Passar para `export_pdf_with_font`.

---

## 4. Vanilla referência

### 4.1 Pipeline vanilla font → PDF

Vanilla `lab/typst-original/crates/typst-pdf/src/` tem
pipeline muito mais sofisticado:
- Subsetting via `subsetter` crate.
- CFF + TTF support.
- CMap builders.
- Composite glyphs.

Cristalino pipeline **simplificado**: full-font embed (sem
subsetting), TTF only, single-font-per-document.

### 4.2 Crates vanilla

- `ttf-parser` ✓ (cristalino autorizado).
- `rustybuzz` — vanilla USA; cristalino **declara mas não usa**.
- `subsetter` — vanilla usa; cristalino **não**.

### 4.3 Níveis de paridade para cristalino

| Nível | Descrição | Custo |
|-------|-----------|:-----:|
| **Mínima** | Fallback Helvetica se font não match; warning | XS |
| **Básica v1** | Single font per document via `export_pdf_with_font` wired | S |
| **Básica v2** | Multi-font per document (span-level) | M |
| **Completa** | + Subsetting | M-L |
| **Total** | + Shaping (rustybuzz) + CFF | XL |

ADR-0054 perfil "observacional graded" aceita **básica v1 ou v2**
como fecho DEBT-1. Completa + total ficam fora.

---

## 5. ADR-0019 estado empírico

### 5.1 Verificação

```bash
grep "ttf-parser" Cargo.toml 03_infra/Cargo.toml
# → encontrado em ambos
grep "ttf_parser" 03_infra/src/
# → 5+ ficheiros usam (font_metrics, fonts)
grep "rustybuzz" Cargo.toml 03_infra/Cargo.toml
# → declarado em ambos
grep "rustybuzz" 01_core/src/ 03_infra/src/
# → zero uso real
```

### 5.2 Status

**ADR-0019 parcialmente implementada**:
- ✓ `ttf-parser` usado em L3 (`font_metrics.rs`, `fonts.rs`).
- ❌ `rustybuzz` declarado mas **sem uso real** (shaping não
  implementado).

### 5.3 Acção recomendada

**Anotar ADR-0019** com nota factual sobre o estado:
- `ttf-parser` IMPLEMENTADO integralmente.
- `rustybuzz` DECLARADO mas não consumido; shaping real fica
  para futuro (fora DEBT-1 per ADR-0054).

Não revogar ADR-0019 — parcialmente correcto. Candidato para
futura "ADR-0019-R1" quando shaping for atacado.

---

## 6. Crates — já autorizadas

Status actual de `01_core/Cargo.toml` + `03_infra/Cargo.toml`
+ `crystalline.toml [l1_allowed_external]`:

### L1 autorizadas

- `ecow`, `rustc_hash`, `thiserror`, `comemo`, `indexmap`,
  `time`, `unicode_*`, `clap (features env derive)` — todas
  em uso.

### L3 autorizadas (não reguladas por linter L1)

- `ttf-parser` (0.25) — **USADO**.
- `rustybuzz` (0.20) — declarado, NÃO usado.
- `image` (implied pelas integrations tests).

### Crates necessárias para Fase C (avaliação)

**Para paridade básica v1/v2 (recomendada)**: **nenhuma nova**.
`ttf-parser` + pipeline existente suficientes.

**Para paridade completa** (futuro):
- `subsetter` — reduz tamanho PDF.
- `rustybuzz` — efectivar shaping.

Decisão: ADR-0055 autoriza nada de novo. Se Fase E (shaping)
eventualmente chegar, ADR dedicada.

---

## 7. Roadmap proposto de Fase C

### 7.1 Sub-passos

**140B — Wiring TextStyle.font → pipeline CIDFont (S)**:
- Pipeline lê `TextStyle.font` do primeiro FrameItem::Text.
- Invoca FontBook::select + World::font para bytes.
- Passa para `export_pdf_with_font` em vez de `export_pdf`.
- Mantém Helvetica fallback se font não match.
- **Single font per document** (primeira que aparecer usada em
  todo o doc).

**141 — Font array fallback chain (XS)**:
- Itera FontList em ordem; primeiro que FontBook::select
  resolve vence.
- Paridade vanilla básica.

**142 — Multi-font per document (M)**:
- Estender `build_cidfont` para aceitar múltiplas Face + bytes.
- Resource dict com `/F1 /F2 /F3 /F4 ...` dinâmico.
- Export escolhe font ID per-span.
- **Optional** — fica para quando necessário para paridade.

**143 — Lang hyphenation (opcional, M)**:
- Crate `hyphenation` ou `hypher`. Autorização L1.
- Consumer em `flush_line` ou line-break.
- Independent de font — pode vir em qualquer ordem.

**Fecho DEBT-1**: após **140B + 141** com Helvetica fallback.
142 e 143 opcionais.

### 7.2 Estimativas

- **140B**: 2-3h (S).
- **141**: 30-45min (XS).
- **142**: 3-5h (M).
- **143**: 2-3h (M).

**Fase C básica (140B + 141)**: **~3h** para fechar DEBT-1 com
single-font support. **Muito menor** que Roadmap 135 que
estimava 4-5h para Fase C inteira.

### 7.3 Dependências

- 140B ← nada (infra já está).
- 141 ← 140B (FontList iteration vive no mesmo sítio).
- 142 ← 140B (multi-font é extensão).
- 143 ← independent de font — pode vir antes ou depois.

### 7.4 Alternativas de redução

**Escopo mínimo para fechar DEBT-1 imediatamente**:
- 140B single font wired → marca gap 5 resolvido.
- 141 array fallback → marca gap 6 resolvido.
- 143 adiado se lang-hyphenation não urgente.

Fase C efectiva = **2 passos** (140B + 141). ~3h.

---

## 8. DEBTs adjacentes

### 8.1 Relacionamento com DEBT-52

DEBT-52 lista gaps 5-7:
- Gap 5: consumer font string → resolvido por **140B**.
- Gap 6: consumer font array → resolvido por **141**.
- Gap 7: consumer lang hyphenation → resolvido por **143**.
- Gap 8 (opcional): font dict → requer ADR-0055bis autorizar
  `regex`.

Após 140B + 141, DEBT-52 fica com gaps 7 + 8 restantes.

### 8.2 Novos DEBTs propostos

**Nenhum**. O trabalho de shaping (rustybuzz real) é **fora**
DEBT-1 per ADR-0054. Se quisermos DEBT dedicado para shaping,
**DEBT-53** pode ser aberto num passo futuro (não neste).

### 8.3 Candidato futuro

**DEBT-53 "Shaping via rustybuzz"**: aberto quando for
priorizado. Escopo:
- Integrar `rustybuzz::shape()` no pipeline text layout.
- Per-codepoint glyph selection + positioning.
- OpenType features (ligatures, kerning, bidi).

Estimativa: XL, provavelmente série dedicada. **Não bloqueia
fecho DEBT-1**.

---

## 9. Resumo executivo

**Descoberta-chave**: a infra PDF font embedding **já está
materializada** (CIDFont pipeline completo em
`build_cidfont`). Roadmap 135 subestimou o que existia.

**Gap real**: wiring `TextStyle.font` → FontBook::select →
World::font → `export_pdf_with_font`. Trabalho ~3h em 2
passos (140B single-font + 141 array fallback).

**ADR-0019 parcialmente implementada**: ttf-parser OK,
rustybuzz declarado sem uso. Nota factual recomendada, não
revogação.

**Zero crates novas necessárias** para fecho DEBT-1 básico.

**Fase C é mais pequena que Fase B**: 2 passos (vs Fase B 4
passos). Roadmap total DEBT-1 encolhe de "4-8h após 135" para
"~5h após 135" se 143 (hyphenation) for adiado.

**Próximo passo (140B)** pode começar imediatamente após este
diagnóstico ser aprovado. Infra confirmada; pipeline pronto.
