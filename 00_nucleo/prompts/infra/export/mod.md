# Prompt L0 — `infra/export/mod` — fachada pública dos exporters
Hash do Código: 4ee50bcb

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/mod.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0027 (CIDFont), ADR-0033 (paridade observable), ADR-0055 (multifont), ADR-0098 (SSoT), ADR-0100 (coesão L3), ADR-0128 (target HTML), ADR-0129 (ownership 1:1)

---

## Contexto

Sucessor de `00_nucleo/prompts/infra/export.md` (umbrella pré-P307).
P307b decompôs `export.rs` em `export/` subdirectório com 6 submódulos.
Este L0 cobre apenas o `mod.rs` — fachada pública dos exporters +
dispatch PDF para `PdfBuilder`. A semântica de cada exporter permanece no
owner do respetivo submódulo.

## Restrições estruturais

- Camada L3. Não L1: pode usar I/O em sub-helpers (via flate2, ttf-parser).
- API pública mantém 3 entry points: `export_pdf`, `export_pdf_with_font`, `export_pdf_multifont`.
- Dispatch para `PdfBuilder` em `builder.rs`. mod.rs **não** contém lógica de emit.
- Reexporta `PdfImagePayload` e `process_png_for_pdf` (consumidos por outras crates do workspace via `crate::export::*`).

## Interface

```rust
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8>;
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8>;
pub fn export_pdf_multifont(doc: &PagedDocument, fonts: &[(FontList, Vec<u8>)]) -> Vec<u8>;

// P870 — exportação PNG/SVG
pub fn export_png(page: &Page, opts: &RenderOptions) -> Vec<u8>;
pub fn export_png_with_fonts(
    page: &Page,
    opts: &RenderOptions,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
) -> Vec<u8>;
pub fn export_svg(page: &Page, opts: &SvgOptions) -> String;
pub fn export_svg_with_fonts(
    page: &Page,
    opts: &SvgOptions,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
) -> String;
pub use self::images::{PdfImagePayload, process_png_for_pdf};
pub use self::render::{RenderOptions, render_page_to_png, render_document_to_png};
pub use self::svg::{SvgOptions, export_svg};
pub use self::html::{export_html, export_html_with_serialization, HtmlSerializationMode};
```

## Submódulos declarados

```text
mod builder;     // PdfBuilder + build_helvetica/cidfont/multifont
mod fonts;       // CIDFont helpers + escape_pdf_string + collect_text_codepoints (P568)
mod gradients;   // gradient cluster (sub-decomposto P307b.2)
mod images;      // JPEG/PNG/XObject
mod html;        // exporter HTML semântico; permanece privado atrás desta fachada
mod stream;      // PageContext + emit
mod render;      // P870 — rasterização PNG
mod svg;         // P870 — exportação SVG
```

## Não-objectivos

- Não conhece formatadores user-facing (vive em L2).
- Não conhece world/filesystem (vive em pipeline.rs).
- Não decide dispatch baseado em conteúdo do doc — caller decide via fonts.

## Critérios de verificação

`compile_to_pdf_bytes(world, source)` em `pipeline.rs` despacha para:
- `export_pdf(doc)` se `fonts.len() == 0`
- `export_pdf_with_font(doc, bytes)` se `fonts.len() == 1`
- `export_pdf_multifont(doc, &fonts)` se `fonts.len() >= 2`

## P836 — assinaturas multifont com variações

As funções `export_pdf_multifont*` passam a receber
`&[((FontList, FontVariant, FontVariations), Vec<u8>)]` — a chave
inclui as variações explícitas (P836), propagadas da pipeline para que
runs com `variations:` distintas partilhem apenas o que for idêntico.


## P956 — `StreamMode` público + assinaturas com modo explícito

ADR-0126 (emendada P956): o exporter ganha o tipo público

```rust
pub enum StreamMode { Verbose, Compact }
impl Default for StreamMode { /* Verbose */ }
```

- **`Verbose`** = modo vanilla-espelhado, novo padrão de produção
  (`stream.md` §P956). **`Compact`** = formato Passo 20 (actual), preservado
  para a flag `--compact`.
- As três entry points PDF ganham `stream_mode: StreamMode` como **último
  parâmetro**: `export_pdf(doc, stream_mode)`, `export_pdf_with_font(doc,
  font_data, stream_mode)`, `export_pdf_multifont(doc, fonts, stream_mode)`.
  **Quebra de assinatura deliberada** (precedente P113): o compilador força
  cada caller — produção e testes — a declarar o modo; nenhum consumidor fica
  ambíguo sobre o formato que espera (regra da Fase B.3 de P956).
- O modo é propagado ao `PdfBuilder` e aos `PageContext` (`builder.md` §P956).
  PNG/SVG (`export_png*`, `export_svg*`) **inalterados** — não emitem content
  streams PDF.

## P1140.5-A — fronteira semântica, sem tagging prematuro

### Medição antes da decisão

Vanilla default é tagueado; `--no-pdf-tags` desliga tags. O cristalino não
expõe opção de tagging nem estrutura semântica no builder.

### Decisão

As APIs existentes aceitam `FrameItem::Semantic` e preservam-no até o
exportador. P1140.5 não muda assinaturas nem mistura isso com `StreamMode`.
PDF visual permanece válido mas ainda `Tagged: no`; P1140.6 será o dono de
opção/padrão, structure tree e conformidade PDF/UA.

## P1140.6 — configuração pública de tagging PDF

### Medição antes da decisão

O vanilla ratificado gera PDF tagueado por defeito e `--no-pdf-tags` remove a
estrutura. A API cristalina só recebe `StreamMode`; logo não consegue exprimir
tagging sem o acoplar indevidamente à verbosidade.

### Decisão

Adicionar o enum público tipado `PdfTags { Enabled, Disabled }`, cujo
`Default` é `Enabled`. As entry points de conveniência existentes preservam a
assinatura e selecionam `Enabled`; variantes públicas explícitas
`*_with_tags`/`*_and_tags` recebem `tags: PdfTags` como último parâmetro,
depois de `stream_mode`. `StreamMode` e `PdfTags` são eixos ortogonais: as
quatro combinações são suportadas. PNG/SVG não recebem esta opção. `Enabled`
solicita a estrutura completa descrita em `builder.md` e `stream.md`;
`Disabled` omite BDC/EMC, MCID, StructTreeRoot, ParentTree e MarkInfo marcado.
Esta opção não promete nem valida PDF/UA.

## P1293.reopen-C — fachada pública do modo de serialização HTML

### Medição anterior à decisão

No estado não commitado medido em 2026-09-02T14:18:28-03:00, sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`,
`03_infra/src/export/mod.rs:24` declara `mod html;`, portanto o submódulo é
privado, e `:534` reexporta somente `export_html`. O consumer
`03_infra/src/pipeline.rs:168,181,189,213` precisa nomear
`HtmlSerializationMode` e `export_html_with_serialization`; usar
`crate::export::html::*` atravessaria a privacidade do submódulo irmão.

Os owners vigentes já fecham a substância: `infra/export/html.md` define o
enum, as duas entry points, o default cristalino e o escaping de cada modo;
`infra/pipeline.md` define o transporte do modo no pipeline HTML. Faltava
somente a obrigação de exposição na fachada dona de `export/mod.rs`.

Hashes da medição: este L0 antes da decisão
`bdf4ec679daede866f8f1f303cd17389f6e5f3c2cc0d06849132903870eaa6ce`;
`export/mod.rs`
`56d2484579e26715fa870b66cdbfb6aaca8aa88a674d1ab9b9efefda2490632c`;
`export/html.md`
`505895669df4a1d7ce39f4c3236d6ac4e39fc9c61ce14d8cf578b0d394a2d237`;
`pipeline.md`
`ce6da4f623a0270869606bdf42d07dae0283f3f9e5a593258609b219424e63a8`.
A working tree tinha 61 ficheiros alterados no `git diff HEAD --stat`; esta
medição não atribui essas alterações a um único executor.

### Decisão

Manter `html` como submódulo privado e expor pela fachada pública, sem wrapper
nem lógica adicional:

```rust
pub use self::html::{
    export_html,
    export_html_with_serialization,
    HtmlSerializationMode,
};
```

`export/mod.rs` é dono exclusivamente da visibilidade e do caminho público.
`export/html.md` continua dono único da definição do enum, comportamento das
funções, default, escaping e morfologia HTML. `pipeline.rs` e demais callers
devem consumir a fachada `crate::export::{HtmlSerializationMode,
export_html_with_serialization}` e não o submódulo privado.

Preservar byte-conceitualmente todas as APIs e dispatches PDF, PNG e SVG.
Não duplicar o enum, não criar função de forwarding, não mover lógica de
serialização para `mod.rs` e não alterar feature, target, default, fase,
`Content`, `HtmlElem` ou outro exporter. Esta é a closure operacional da API
L3 já confirmada no gate P1293/C, não um novo contrato público.

### Critérios de verificação

- ownership permanece `infra/export/mod.md` ↔ `export/mod.rs` em 1:1;
- V15 e V26 continuam verdes;
- o resselo posterior altera somente o header de `export/mod.rs`;
- callers L3/L4 conseguem nomear o enum e a entry point pela fachada;
- nenhuma lógica HTML, API PDF/PNG/SVG ou entrada protegida muda nesta fase.
