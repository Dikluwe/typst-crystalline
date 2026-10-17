# L0 — stream/page
Hash do Código: 32f519a3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:8b7fcfaf39e60540d16868e662d48fb466fab58860385fcdae499f8c2edfe799

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/page.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/page.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Page assembly

Own build_page_stream and emit_page_items. Emit canvas fill and background, body,
foreground in that order, using the physical canvas and bleed translation with a
balanced q/cm/Q envelope. Borrow PageContext and return page-operator bytes.
Delegate prosaic run coalescing to text and individual item dispatch to draw;
UA1 leaves keep their marked-content boundaries: this owner bypasses prose run
coalescing for UA1. Auto and None canvas fill emit no explicit fill operators;
Paint uses the existing solid/spot or to_color projection, not a canvas Pattern.
When UA1 is enabled, explicit canvas fill is an Artifact Layout. Do not allocate resources, objects,
ParentTree entries or fonts, and do not compress the returned operators.

## Verification

Check blank pages, fill, bleed, background/foreground order and unchanged top-level
coordinates with both graphic modes and tagging settings.

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

No canvas Solid translúcido, aplicar opacidade com q/Q envolvendo somente o fill da página. Não alterar projeção antiga de PageFill Gradient/Tiling nem fingir que esta etapa a corrige. Os itens seguintes começam opacos.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.
