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
| `01-markup-plain.typ` | 2204 | `dce5df2a` | API + Helvetica + escape_pdf_string |
| `02-markup-heading.typ` | 2630 | `1f1b3d48` | Helvetica + emit_text |
| `03-text-styling.typ` | 2328 | `57e163f0` | Bold/italic via Styled → /F2/F3 |
| `04-shapes.typ` | 2263 | `bf4ea743` | Shape kinds (rect, circle) + paint solid |
| `05-gradient-linear.typ` | 2162 | `b53dcaf3` | Linear scan + pattern_resources (P263) |
| `06-gradient-conic.typ` | 2234 | `8c45c042` | Conic Coons (P272) + bezier_control_points |
| `07-multi-feature.typ` | 2992 | `93c2de5f` | Integração (heading + gradient + multi-página) |
| `08-image-jpeg.typ` | 3495 | `6218e60c` | JPEG XObject + dedup + zlib |
| `09-cidfont.typ` | 6476 | `c43ceee6` | CIDFont + Type0 + Identity-H + ToUnicode CMap (subset da fonte versionada) |

**Total**: 26.784 bytes de referência distribuídos por 9 fixtures.

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
