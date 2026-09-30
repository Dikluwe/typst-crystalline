# P307b — corpus canónico de snapshot binário

**Data**: 2026-05-19 (gerado em P307a.2; expandido para 9 fixtures após feedback humano sobre cobertura)
**L0 ancora**: `00_nucleo/prompts/infra/export-fixtures.md`
**Propósito**: invariante observable bit-exact para validar P307b.1
(extracção de submódulos) — ver ADR-0100 §"Critérios de validação"
e ADR-0098 §"Evolução pós-P307".

Este ficheiro é **operacional** (tabela de bytes/md5 actualizada
quando fixtures são regenerados). A documentação de propósito,
critério de escolha e protocolo de regeneração vive no L0
`infra/export-fixtures.md`.

---

## Inventário operacional

| Source | Output (bytes) | md5 (8 prefix) | Cluster exercitado |
|---|---:|---|---|
| `01-markup-plain.typ` | 2277 | `0c88172f` | API + Helvetica + escape_pdf_string |
| `02-markup-heading.typ` | 2709 | `75c05666` | Helvetica + emit_text |
| `03-text-styling.typ` | 2464 | `d6f74d6c` | Bold/italic via Styled → /F2/F3 |
| `04-shapes.typ` | 2325 | `9d5654cd` | Shape kinds (rect, circle) + paint solid |
| `05-gradient-linear.typ` | 14129 | `28eb4d9c` | Linear fill Pattern (P1483) |
| `06-gradient-conic.typ` | 5327 | `0bcb38c3` | Conic fill Pattern + Coons (P1483) |
| `07-multi-feature.typ` | 15032 | `6c1d05b6` | Integração (heading + gradient + multi-página) |
| `08-image-jpeg.typ` | 3557 | `e8c26077` | JPEG XObject + dedup + zlib |
| `09-cidfont.typ` | 6759 | `59d0461d` | CIDFont + Type0 + Identity-H + ToUnicode CMap (subset da fonte versionada) |

**Total**: 54.579 bytes de referência distribuídos por 9 fixtures.

## Cobertura por cluster

| Cluster (per diagnóstico §2) | LOC produção | Fixture | Cobertura |
|---|---:|---|---|
| PageContext + emit (P281) | 668 | 01, 02, 03 | ✓ directa |
| Builder | 630 | todos | ✓ directa |
| Conic Coons (P272) | 445 | 06 | ✓ directa |
| Gradient Linear (P263) | 390 | 05, 07 | ✓ directa |
| Imagens | 311 | **08** | ✓ directa |
| CIDFont helpers | 118 | **09** | ✓ directa |
| Gradient CMYK (P270.2) | 83 | (indirecto via Builder) | ⚠ parcial |
| Gradient relative (P273) | 53 | (indirecto via Builder) | ⚠ parcial |
| Adaptive N (P274) | 64 | (indirecto via Builder) | ⚠ parcial |

**Cobertura directa**: 6/9 clusters (2.562/2.826 LOC = 90,7%).
**Cobertura indirecta**: 3/9 clusters restantes (264 LOC) via testes
inline pré-existentes em `export.rs::tests` que migram para
`export/tests.rs` em P307b.1.

## Determinismo

Cada fixture validado 2× em invocações consecutivas; bytes
idênticos confirmados via `md5sum`. Comando original:

```bash
for f in 01-markup-plain 02-markup-heading 03-text-styling \
         04-shapes 05-gradient-linear 06-gradient-conic \
         07-multi-feature 08-image-jpeg; do
  ./target/debug/typst sources/${f}.typ /tmp/r1.pdf
  ./target/debug/typst sources/${f}.typ /tmp/r2.pdf
  diff -q /tmp/r1.pdf /tmp/r2.pdf
done
# Fixture 09 requer --font-path
./target/debug/typst --font-path 03_infra/fixtures/fonts \
  sources/09-cidfont.typ /tmp/r.pdf
```

**Conclusão P307a.2**: snapshot binário é fiável como invariante
observable em condições controladas. P307b.1 pode usar `assert_eq!`
directo via `include_bytes!`.

## Notas operacionais

### P1483 (gradientes de preenchimento)

Referências 05/06/07 regeneradas pelo teste consumidor com
`UPDATE_P307B_SNAPSHOTS=1`, `StreamMode::Compact`,
`PdfTags::Disabled` e epoch fixo. Antes, fill de gradiente
caía em cor sólida; agora os PDFs contêm Pattern/Shading.
Duas gerações consecutivas produziram SHA-256 idênticos;
`qpdf --check` passou nas três. As seis referências restantes
não mudaram. Os bytes/md5 dessas seis linhas no MANIFEST
já estavam desatualizados antes de P1483; foram corrigidos
por leitura dos arquivos intactos (diferença agregada de 491
bytes em relação ao total anterior), sem regeneração.
Isto é baseline cristalino, não equivalência
binária com o vanilla.

### Fixture 08 (JPEG)

Inclui `sources/tiny.jpg` (634 bytes) — JPEG 1×1 regenerado em **P833**
(o original de P307a.2, 558 bytes, tinha DQT em falta para Cb: o vanilla
rejeita-o com `failed to decode image (...)`, e a validação de P833/#18
passou a fazer o mesmo — o fixture nunca foi compilável pelo vanilla).
Referência regenerada pelo protocolo do L0 (`CRYSTALLINE_PDF_FIXED_EPOCH=0`,
2 invocações byte-idênticas verificadas).

### Fixture 09 (CIDFont)

Requer a fonte externa versionada descoberta via `--font-path
03_infra/fixtures/fonts`. O fixture `.typ` declara
`#set text(font: "Noto Sans")`. A entrada é `NotoSans-Regular.ttf`
(556.216 bytes, SHA-256
`2ec33f84606cbaa0a1a944488e14f97faf2f6a25ecdd8354f5358f06da13c7d9`),
copiada do checkout Cargo de `krilla` `7772dbe`; a redistribuição é OFL-1.1,
preservada em `../fonts/LICENSE-NotoSans.md`. O PDF resultante contém o
subset necessário, em vez da fonte integral.

Se o font path mudar ou a fonte ser actualizada, regenerar fixture.

## Como regenerar

Ver protocolo completo em
`00_nucleo/prompts/infra/export-fixtures.md` §"Protocolo de
regeneração".

## Como consumir (P307b.1)

Ver template em `00_nucleo/prompts/infra/export-fixtures.md`
§"Uso em P307b.1".

### Correção de falhas — 2026-09-28

Referências 01/02/03/07/09 regeneradas pelo harness controlado após a
materialização autorizada dos separadores textuais: espaços reais passaram a
ser emitidos no PDF, incluindo o glifo e mapeamento U+0020 no subset CIDFont.
04/05/06/08 conservaram exatamente os bytes. O texto extraído e a imagem da
primeira página a 144 dpi permaneceram iguais nos cinco casos alterados;
`qpdf --check` aprovou todos. Nova execução sem UPDATE aprovou os nove
snapshots. A prova, os PDFs anteriores, hashes e comparações estão em
`00_nucleo/diagnosticos/correcao-falhas-2026-09-28/snapshot-comparison.json`.
São referências cristalinas controladas, não prova de paridade binária vanilla.

### Tracking shaped — estado PDF em 2026-09-29

Somente 09-cidfont foi regenerado pelo harness Compact/Disabled/fixed epoch
após a correção de soma dupla de tracking. A stream incorpora Tc=0 explícito
nos trechos shaped; a remoção desses operadores restitui exatamente a stream
anterior. Texto e raster a 144dpi são idênticos; qpdf aprovou os PDFs e duas
gerações produziram bytes idênticos. Referência anterior 6759B,
SHA-256 4754801d459118860b642267b636375ae983008d65ea51dba06489861ff2a865; atual 6768B,
SHA-256 29a600be50a6655b49cce33d3d01826dc8e6ccf0aea81928293159498fb59b91. Fonte e entrada não mudaram.
Provas em `00_nucleo/diagnosticos/tracking-pdf-2026-09-28/snapshot-receipt.json`.
