# Passo 1486 — rótulos de páginas PDF após seleção

**Estado:** MATERIALIZADO em 2026-09-24 após confirmação ADR-0127; a prova
bilateral, os testes e os limites de paridade estão em
`00_nucleo/diagnosticos/typst-p1486-page-labels.md`. Independente de
P1480–P1485.

**Baseline:** HEAD limpo `90aad760ba472cffb70063baa684c461a240e447`
(2026-09-22), vanilla `586e1bd43`; hashes dos binários em P1480 §1.

## 1. Medição anterior à decisão

`00_nucleo/prompts/infra/export/builder.md:918-923` registra como
scope-out que o vanilla preserva a numeração física no rótulo PDF quando
`--pages` omite páginas e o cristalino não emite rótulos. A fonte vanilla
`lab/typst-original/crates/typst-pdf/src/convert.rs:130-149` escolhe
`PageLabel::generate(numbering, page.number)` ou, com páginas puladas,
`PageLabel::arabic((i + 1) as u64)`;
`typst-pdf/src/page.rs:17-60` distingue padrão/sufixo e sistemas que o PDF
representa nativamente. No cristalino já existem
`Page.numbering` (`01_core/src/entities/layout_types.rs:849-855`) e
`PageIndexConverter` (`03_infra/src/export/builder.rs:163-170`).

Antes da decisão final, produzir PDFs bilaterais com 4 páginas e exportar
1–4, 2–4, 1+3, só 4; repetir com numeração arábica, romana e sem padrão.
Inspecionar o rótulo visível nos leitores e o catálogo/number tree com
`qpdf`, registrando a **chave real** (`/PageLabels` no catálogo, se
confirmada), índices e valores. Não tomar a grafia singular `/PageLabel`
do L0 antigo como nome de chave PDF sem essa checagem.

Classificação: rótulo visível da página exportada é observável público;
forma interna da number tree, IDs de objetos e bytes não são. Inferência:
os dados existentes dispensam novo campo público; refutação seria um caso
bilateral cujo número/padrão não pode ser reconstruído de `Page` e do
conversor sem novo transporte.

## 2. Contrato

Para cada página retida, o PDF comunica o rótulo que o vanilla comunica
para a página física original, inclusive quando o índice no PDF resultante
é outro. Preservar numeração definida pelo autor e a regra de fallback
quando páginas foram puladas. Não alterar a numeração *desenhada* no
conteúdo, a seleção, links, outlines ou os índices de `ParentTree`.
Nenhuma opção nova de CLI e nenhum default novo de numeração.

## 3. L0, RED→GREEN e fechamento

Retificar primeiro o scope-out P1468 no owner
`infra/export/builder.md` ↔ `03_infra/src/export/builder.rs`; auditar
`entities/layout_types.md` apenas se o transporte de `Page` realmente
mudar. Campo/assinatura pública ou alteração da fase de cálculo exige
paragem ADR-0127; emissão interna de paridade segue fluxo contínuo.

RED compara rótulos bilaterais, não strings de PDF. GREEN cobre a matriz
acima, PDFs sem seleção e seleção vazia, quando aceita pelo contrato, além
de regressões de `--pages`, tagged/PDF/A/PDF/UA. Validar estrutura do PDF,
`cargo build`, testes e `crystalline-lint .`/V5/V15/V26. Reportar estado
Git/hora/hashes, fixture e comando de extração do rótulo; não fechar com
mera presença de uma chave no catálogo se o número estiver incorreto.
