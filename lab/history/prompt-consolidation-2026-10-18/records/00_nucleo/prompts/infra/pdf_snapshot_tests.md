# L0 — Controlled PDF snapshot corpus
Hash do Código: 2be850e7

**Layer:** L3 (tests and fixtures)
**Exclusive consumer:** `03_infra/src/pdf_snapshot_tests.rs`

## Current evidence

- `03_infra/src/pdf_snapshot_tests.rs` — `compile_fixture` and `assert_bytes_eq`.
- `03_infra/fixtures/p307b/sources/`, `reference/` and `MANIFEST.md` — current input/reference corpus.
- `03_infra/fixtures/fonts/NotoSans-Regular.ttf` and `LICENSE-NotoSans.md` — the CIDFont input and license.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own only the binary snapshot harness and its controlled corpus, not PDF production algorithms. This is crystalline-to-crystalline regression evidence, distinct from language parity against upstream. Byte identity is the selected snapshot observable; it does not prove visual correctness, upstream PDF byte equality or performance.

| Input stem | Primary observable |
| --- | --- |
| 01-markup-plain | Basic text and string escaping |
| 02-markup-heading | Heading/text emission |
| 03-text-styling | Styled bold/italic text |
| 04-shapes | Rect/circle and solid paint |
| 05-gradient-linear | Linear gradient fill |
| 06-gradient-conic | Conic gradient fill |
| 07-multi-feature | Mixed structure/gradient and multiple pages |
| 08-image-jpeg | JPEG resource, with `sources/tiny.jpg` |
| 09-cidfont | Noto Sans subset, CIDFont and text mapping |

Resolve inputs/reference PDFs through `CARGO_MANIFEST_DIR`, not a developer's absolute checkout path. The harness creates a SystemWorld for each source and supplies the repository font directory only for the CIDFont case. Preserve Compact streams, Disabled tags and `CRYSTALLINE_PDF_FIXED_EPOCH=0` as explicit snapshot controls; these do not change the production defaults. Keep source/reference pairing and the licensed font identity recorded in MANIFEST.

Add a fixture for a newly exercised export path or an uncovered regression; remove it only when the behavior is removed or equivalent coverage replaces it. Regenerate references only for an intentional observable change with recorded justification. The current harness reads reference files at runtime and has an explicit `UPDATE_P307B_SNAPSHOTS` maintenance path. Use that harness rather than the removed include_bytes template or obsolete CLI invocation.

Regeneration must preserve the fixed epoch/stream/tag controls, select affected cases, repeat generation to verify deterministic bytes in the same environment, check PDF validity and refresh the operational MANIFEST. Do not bless unrelated reference changes. Do not promise identical output across toolchains, uncontrolled fonts or production identifier defaults. Exporter clocks, identifiers and deterministic serialization belong to their excluded owners, not this fixture prompt.

## Verification boundary

The existing p307b tests compare actual/reference bytes. Maintenance runs may use `cargo test -p typst-infra --lib p307b_ -- --test-threads=1` with the explicit update environment only when regeneration is authorized. No fixture/reference regeneration is part of documentary sanitation.

## Referência CIDFont — manutenção de tracking PDF em 2026-09-29

Medição anterior: tests-before-reference-update.log em
`lab/results/construction/diagnostics/tracking-pdf-2026-09-28/` registra somente
p307b_09_cidfont falhando após o estado Tc=0 explícito no caminho shaped
compacto; 6768 bytes atuais versus 6759 anteriores. A fonte e entrada
continuam iguais, sem tracking configurado; a mudança de operadores é
intencional para impedir herança de espaçamento, não uma prova de regressão
visual ou linguagem. Regerar somente 09-cidfont pelo harness, duas vezes,
com controles existentes. Validar PDF e conservar pixels/texto anteriores;
registrar bytes/hash, diferenças de stream e atualizar MANIFEST. Não
regerar outros casos nem desativar comparação byte a byte. O owner conserva
seu único harness produtivo e corpus, sem algoritmo de exportação novo.

## Referência JPEG — clip efetivo de imagem em 2026-10-08

Medição antes da manutenção: o teste `p307b_08_image_jpeg` passou de
3557 para 3541 bytes após omitir o clip redundante de uma imagem cujo
retângulo coincide com o target. O stream preserva a matriz e os bytes do
JPEG; a rasterização bilateral com o vanilla passou a coincidir nas quatro
resoluções medidas (36, 72, 144 e 288 dpi). Regenerar somente 08-image-jpeg
pelo harness Compact/Disabled/epoch fixo, repetir para verificar bytes
determinísticos, validar o PDF e atualizar o MANIFEST com os hashes. Os
outros oito snapshots devem permanecer byte a byte idênticos.

## Árvore de destinos nomeados — 2026-10-14

Medição anterior: destinos-pdf-2026-10-14/workspace.log registra apenas
02-markup-heading e 07-multi-feature falhando no corpus após corrigir a
árvore de destinos e os operandos GoTo. Os tamanhos passaram de 2709 para
2754 bytes e de 15032 para 15112 bytes. Leitura independente por MuPDF
confirma que os destinos antigos não são resolvidos e os novos são.
O estado completo e a suíte estão no diagnóstico; referências/fontes/owner
antes da manutenção em destinos-pdf-2026-10-14/snapshots/before.json.gz.

Regenerar somente 02 e 07 pelo harness existente, preservando Compact,
Disabled e epoch fixo. Repetir geração, verificar determinismo e validar
PDF com qpdf. Comparar objetos/streams decodificados, texto e renderização
com as referências anteriores; a mudança intencional é a árvore de nomes,
não a pintura ou o conteúdo. Conservar os outros sete PDFs byte a byte e
revalidar os nove testes sem UPDATE. Registrar diferenças e atualizar o
MANIFEST; não reduzir a comparação binária do harness.

## Referência cônica — posicionamento físico 2026-10-14

Medição anterior: first-validation/workspace.log no diagnóstico
conic-posicionamento-2026-10-14 registra somente p307b_06_gradient_conic falhando
no corpus: 5327 bytes anteriores contra 5350 atuais. O controle independente
red versus conic(red,red) e cinco testes RED→GREEN comprovam a ausência anterior
de escala/posição física, incluindo páginas e grupos. O fixture é um círculo;
a mudança de render é intencional, e não mera atualização para esconder falha.
A suíte foi interrompida com SIGTERM após o resultado infra e não é validação
completa; a causa não foi inferida. snapshot-before.json.gz conserva corpus,
fonte/L0, HEAD, UTC, diffstat e hashes antes de qualquer regeneração.

Regenerar somente 06-gradient-conic pelo harness existente, Compact/Disabled/
epoch fixo. Repetir geração, exigir determinismo, qpdf e rasterização; comparar
objetos para provar que somente a matriz do padrão e offsets derivados mudaram,
com a mesma malha e o mesmo stream de pintura. Conservar os outros oito PDFs,
fontes e asserts; atualizar MANIFEST e repetir corpus sem UPDATE e suíte completa.
Não certificar com isso centro/cobertura geral/amostragem CMYK ainda pendentes.

## Referência cônica — geometria e amostragem 2026-10-14

Medição anterior: conic-geometria-amostragem-2026-10-14/conic-tests.log
registra apenas snapshot 06 divergente no conjunto cônico. geometry-final-tests.log
confirma centro, raio físico, orientação, offsets CMYK, descontinuidades e
representação do centro. As sondas final-coverage e boundary-check preservam
HEAD, árvore e binário; cobertura RGB de formas sem rotação foi recuperada.
No controle rotacionado, MuPDF produz sólido/cônico idênticos; Poppler conserva
diferenças de borda. Controles de stroke via dicionário não exercitaram o gradiente
porque o produto ignorou o paint: não são evidência de cobertura do stroke.
CMYK sólido/gradiente diverge em cor; isso permanece pendente, sem mascaramento.

Regenerar somente 06-gradient-conic pelo harness determinístico existente.
A nova malha deve representar centro, geometria física e cores do contrato,
com coordenadas de 16 bits e Decode correspondente. Espera-se alteração do
Shading e dos comprimentos/offsets derivados, conservando Matrix, stream de
pintura e demais objetos. Exigir duas gerações idênticas, qpdf, inspeção raster,
outros oito PDFs e fontes intactos, MANIFEST atualizado, corpus e suíte sem UPDATE.
Não exigir identidade binária com vanilla nem tratar este fixture isolado como
certificação de todos os espaços de cor, transparência, stroke ou domínios extremos.
