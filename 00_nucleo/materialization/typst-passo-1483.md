# Passo 1483 — realizar cor spot no PDF e nos demais backends

**Estado:** EXECUTADO em 2026-09-23 — working tree não commitada.
Completa o escopo PDF de `color.spot.tint` iniciado em P1482;
o carrier e o gate público P1482 foram confirmados pelo humano.

**Baseline:** auditoria do commit limpo
`90aad760ba472cffb70063baa684c461a240e447` em 2026-09-22 contra o
vanilla pinado `586e1bd43`; hashes e sonda de linguagem em P1482 §1.

## 1. Medição anterior à decisão

P1473 registrou que `.tint()` no vanilla produz `Color::Spot`, mas deixou
fora a emissão PDF `/Separation` porque o exportador cristalino não a
possui (`00_nucleo/materialization/typst-passo-1473.md` §7.5;
`00_nucleo/prompts/compiler/stdlib/color.md` §P1473). A ausência de
`Color::Spot` no cristalino impedia observar a segunda metade do defeito:
resolver a chamada não prova que uma tinta spot apareça no PDF.

Antes de decidir a implementação, gerar bilateralmente PDFs com o mesmo
colorant a 0%, 50% e 100% em preenchimento e traço; dois colorants com
nomes diferentes; `color.spot(none, fallback)` e
`color.spot("all", fallback)`; uso repetido; uma cor spot em
gradiente, se o vanilla o aceitar. Inspecionar espaços de cor, função de
transferência e aparência por ferramenta independente. Medir também
SVG/PNG/HTML para saber se o fallback visual é semântico ou há outro
comportamento. A estrutura interna do `krilla` não legitima copiar seus
passos ou IDs de objetos.

Classificação: cor impressa como separação, identidade de colorant e
aparência fallback são observáveis públicos. Bytes, nomes de recursos
internos e ordem de objetos não são. Inferência: PDF `/Separation` mais
fallback de processo nos demais backends é suficiente; qualquer sonda
vanilla que use `/DeviceN` ou comportamento diferente refuta a inferência.

## 2. Contrato e owners

Preservar o colorant e tint do valor P1482 na emissão PDF de fill/stroke;
produzir uma separação válida com alternativa de processo e transformação
que reflita a intensidade medida. Não substituir silenciosamente por RGB
em PDF quando o vanilla preserva spot. Evitar recursos duplicados para o
mesmo colorant se isso afetar o significado; deduplicação por bytes não é
critério. Nos backends sem tinta spot, usar somente o fallback observado,
sem alterar `repr`/tipo da linguagem.

Auditar L0s `infra/export/builder.md`, `infra/export/stream.md`,
`infra/export/svg.md`, `infra/export/render.md`, `infra/export/html.md` e
os L0s de pintura/gradientes em L1 efetivamente atingidos. Atualizar
somente owners alterados, antes de código. Se um novo campo público,
assinatura ou mudança de fase for indispensável, parar no ADR-0127; não
assumir que a confirmação de P1482 cobre nova API não descrita.

## 3. RED→GREEN

Testes de linguagem P1482 continuam verdes. RED de PDF deve distinguir
“compila mas converte a processo” de “emite separação”; depois do GREEN,
verificar semanticamente os colorants e percentuais no PDF e comparar
render de fallback com o vanilla dentro de tolerância justificada. Controles:
cor de processo pura, spot em texto/shape/gradiente quando suportado,
PDF comum e standards relevantes, SVG, PNG e HTML com os gates corretos.

Fechamento exige execução bilateral, PDF estrutural validado, testes de
export e `cargo build && crystalline-lint .` sem violação. Registrar
commit/tree ou diff exato, hora, hashes dos binários, ferramentas e
observáveis; não usar igualdade byte a byte nem simplesmente contar a rota
`color.spot.tint` como MATCH antes de testar seu uso real.
## 4. Execução e resultado — 2026-09-23

O L0 dos owners de builder, stream, gradientes, funções, Coons,
testes e fixtures foi editado antes do código e ressellado.
O teste PDF começou RED (separação ausente); gates adicionais
foram RED para xref de IDs reservados, gradientes de background
e coordenadas do próprio shape. Os sete testes P1483 terminaram
GREEN. Nenhum contrato público novo foi introduzido.

O PDF agora usa recursos `/Separation` por colorante, tint
quantizado para fill/stroke/texto e função Type 2 de branco
(ou CMYK zero) até ao fallback. Nomes especiais `/None`/`/All`,
escape de nome e fallbacks homónimos distintos foram cobertos.
Gradientes spot linear/radial/cónico mantêm um canal de tinta,
e seu shading/Pattern segue o bbox do shape. A correção
necessária do xref marcou IDs reservados sem objeto como livres.

A sonda bilateral contra o vanilla pinado `586e1bd43`
mostrou 0%, 50%, 100%, `None`/`All`, CMYK e reuso.
`qpdf 11.9.0 --check` aprovou os PDFs cristalinos solid,
special, gradient, stroke direto, PDF/A-2b e PDF/UA-1
(este último com título obrigatório). Isso prova sintaxe
e referências, não certificação formal A/UA. `mutool 1.23.10`
e `pdftoppm 26.05.0` renderizaram os PDFs: o gradiente
axial não aparecia com eixo no centro da página grande e
passou a pintar depois de receber o bbox do shape; radial e
cónico também se tornaram visíveis. SVG contém o fallback
`#ffffff`, `#ffa09a`, `#ff4136`; PNG renderiza
o mesmo fallback. HTML não foi compilado nos dois binários
sem `--features html`.

Controle e regressão: `cargo test -p typst-infra --lib p1483`
7/7; suíte completa de infra 1052/1052; `cargo build &&
crystalline-lint .` exit 0, zero V5/V15/V26; `rustfmt`
dos arquivos P1483 e `git diff --check` passam.
Os fixtures binários P307b 05/06/07 foram regenerados
pelo harness Compact/tags Disabled porque antes fill de
gradiente era sólido. Duas gerações deram os mesmos SHA-256,
e os três novos PDFs passaram qpdf. Os seis demais PDFs
não mudaram; metadados de tamanho/md5 previamente desatualizados
no MANIFEST foram corrigidos por leitura, sem regeneração.

Scope-out observado: o parsing de `stroke: (paint:, thickness:)`
já falha também com cor RGB de processo e continua fora de P1483;
o traço spot direto emite `SCN` corretamente. Igualdade de
bytes/ID ou aparência pixel a pixel com o vanilla não foi usada
como critério de paridade.

### Proveniência dos números

Medição final em 2026-09-23T19:55:10-03:00, HEAD
`3d2aea6618f81bcde66aaf8a4dce8aea1bc0f3be`,
working tree **não commitada** com alterações P1482 preexistentes
e P1483. SHA-256 de `git diff HEAD` rastreado:
`90ed71369517690194b50f94a728dc401789521c66bf412ed1c9bc13f5c0f668`.
Binário vanilla `/usr/local/bin/typst`:
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
binário cristalino `target/debug/typst`:
`56b0426d0fe41effc396f100475d48fb02c1395c34aa8cd58c2ad4894de6d1c6`.
Os artefatos de sonda estão em `/tmp/p1483-pdfs-gnOMZv`
e não integram o commit. Lista exata de arquivos rastreados
alterados na medição (`git diff HEAD --name-only`):

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/call_dispatch.md
00_nucleo/prompts/compiler/eval/repr.md
00_nucleo/prompts/compiler/stdlib/color.md
00_nucleo/prompts/compiler/stdlib/foundations/color.md
00_nucleo/prompts/compiler/stdlib/gradients.md
00_nucleo/prompts/entities/color.md
00_nucleo/prompts/entities/gradient.md
00_nucleo/prompts/entities/value.md
00_nucleo/prompts/infra/export-fixtures.md
00_nucleo/prompts/infra/export/builder.md
00_nucleo/prompts/infra/export/gradients/conic.md
00_nucleo/prompts/infra/export/gradients/function_dict.md
00_nucleo/prompts/infra/export/gradients/mod.md
00_nucleo/prompts/infra/export/stream.md
00_nucleo/prompts/infra/export/tests/mod.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/bindings/value_methods.rs
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/introspect.rs
01_core/src/compiler/layout/block.rs
01_core/src/compiler/layout/boxed.rs
01_core/src/compiler/layout/cursor.rs
01_core/src/compiler/layout/decorations.rs
01_core/src/compiler/layout/grid.rs
01_core/src/compiler/layout/tests.rs
01_core/src/compiler/layout/text.rs
01_core/src/compiler/math/layout/callbacks.rs
01_core/src/compiler/math/layout/cancel.rs
01_core/src/compiler/math/layout/matrix.rs
01_core/src/compiler/math/layout/mod.rs
01_core/src/compiler/math/layout/p1431_tests.rs
01_core/src/compiler/math/layout/underline.rs
01_core/src/compiler/stdlib/color.rs
01_core/src/compiler/stdlib/foundations/color.rs
01_core/src/compiler/stdlib/gradients.rs
01_core/src/compiler/stdlib/html.rs
01_core/src/compiler/stdlib/layout.rs
01_core/src/compiler/stdlib/mod.rs
01_core/src/compiler/stdlib/shapes.rs
01_core/src/compiler/stdlib/structural/table_grid.rs
01_core/src/compiler/stdlib/text/constructor.rs
01_core/src/compiler/stdlib/visualize.rs
01_core/src/entities/color.rs
01_core/src/entities/content.rs
01_core/src/entities/elements/boxed.rs
01_core/src/entities/elements/grid.rs
01_core/src/entities/elements/grid_cell.rs
01_core/src/entities/elements/overline.rs
01_core/src/entities/elements/strike.rs
01_core/src/entities/elements/table.rs
01_core/src/entities/elements/table_cell.rs
01_core/src/entities/elements/underline.rs
01_core/src/entities/gradient.rs
01_core/src/entities/paint.rs
01_core/src/entities/style.rs
01_core/src/entities/style_chain.rs
01_core/src/entities/tiling.rs
01_core/src/entities/value.rs
02_shell/src/cli.rs
03_infra/fixtures/p307b/MANIFEST.md
03_infra/fixtures/p307b/reference/05-gradient-linear.pdf
03_infra/fixtures/p307b/reference/06-gradient-conic.pdf
03_infra/fixtures/p307b/reference/07-multi-feature.pdf
03_infra/src/export/builder.rs
03_infra/src/export/gradients/adaptive.rs
03_infra/src/export/gradients/conic.rs
03_infra/src/export/gradients/function_dict.rs
03_infra/src/export/gradients/mod.rs
03_infra/src/export/mod.rs
03_infra/src/export/render.rs
03_infra/src/export/stream.rs
03_infra/src/export/svg.rs
03_infra/src/export/tests/mod.rs
03_infra/src/p307b_snapshot_tests.rs
03_infra/tests/p1250_svg_integrated.rs
```

Arquivos não rastreados na mesma medição
(`git ls-files --others --exclude-standard`):

```text
00_nucleo/materialization/typst-passo-1482.md
00_nucleo/materialization/typst-passo-1483.md
00_nucleo/materialization/typst-passo-1484.md
00_nucleo/materialization/typst-passo-1485.md
00_nucleo/materialization/typst-passo-1486.md
00_nucleo/materialization/typst-passo-1487.md
00_nucleo/materialization/typst-passo-1488.md
00_nucleo/prompts/infra/export/tests/p1483_pdf_spot_tests.md
03_infra/src/export/tests/p1483_pdf_spot_tests.rs
```
