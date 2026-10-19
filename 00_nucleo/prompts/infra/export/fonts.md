# L0 — infra/export/fonts
Hash do Código: ad60f016

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:3c6209895df993843d15259c7ee95195e7f0dc732ee56cf3cc553257e30298b0
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/fonts.rs:26` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF font and Unicode helpers

Own escape_pdf_string, document collectors, char/GID mapping, widths, ToUnicode
CMap, UTF-16BE hexadecimal encoding and shaped cluster reconstruction.
Keep existing sibling visibility and no filesystem or network lookup.

The Helvetica literal helper escapes parentheses and backslash, retains printable
ASCII (including DEL under the current predicate), and replaces non-ASCII or
characters below ASCII space with "?". This is the actual restricted helper,
not a claim of full Latin-1 support. The Identity-H helper emits four-hex-digit
GIDs and maps an absent character to 0000.

Collect codepoints from plain Text and shaped char_code; collect_text_codepoints
reads only plain Text; collect_glyph_ids includes direct Glyph and shaped GIDs.
Traverse background/body/foreground and Group/Link/Semantic descendants through
FrameVisitor. Nonvisual alt never contributes glyphs or text. Ordered sets keep
unique output deterministic. Shaped mapping uses the first char_code per GID and
ignores zero-x_advance marks; shaped cluster-text collection preserves run order.

cluster_text derives boundaries from sorted unique UTF-8 byte offsets plus text
length, independently of visual LTR/RTL glyph order. Invalid/non-boundary ranges
yield an empty substring. Only the first occurrence of a cluster receives its
complete UTF-16BE text; later glyphs in the cluster receive empty hex. Preserve
ligatures and non-BMP surrogate pairs, not truncated u16 codepoints.

widths_array accepts current (gid, hex) pairs and converts the effective face's
advance to rounded 1/1000 text units, retaining missing-advance fallback 500.
to_unicode_cmap accepts GID-to-UTF-16BE strings, Identity-UCS codespace 0000–FFFF,
and batches bfchar entries at no more than 100. Empty input produces a valid
empty map without an empty bfchar batch.
These helpers do not select effective font axes, embed streams or decide remapping;
those belong to `00_nucleo/prompts/infra/export/builder/fonts.md`.

## Verification

Check escaping, ordered recursive collection, marks, ligatures, RTL clusters,
surrogate pairs, notdef and CMap validity. Extracted Unicode and positioned glyph
morphology are observables; subset sizes and exact GID assignment are not parity.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/fonts.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Recolher U+0020 de cada TextSeparator tanto no conjunto global como no fallback textual usado no subset e no ToUnicode.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## SHY autoral não selecionado — correção de identidade

### Medição anterior à decisão

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/initial.json` e
`before/resultados.json` registram estado, binários, comandos e trinta sondas
bilaterais. Em largura 300pt, repre<U+00AD>sentation extrai dois SHY no Tekt
e um no vanilla pinado. O controle spaces extrai seis versus um.
`03_infra/src/shaper/shaping.rs:313-325` transporta o cluster SHY produzido
pelo rustybuzz; `03_infra/src/export/fonts.rs:318-328` associa sua substring
ao GID de espaço. `export/builder/fonts.rs:523-535` conserva uma identidade
por GID; a stream do controle anterior associa CID 0001 a 00AD e o reutiliza
no espaço posterior. São observáveis textuais, não igualdade de bytes.
A hipótese seria refutada por ausência dessa associação ou por duplicação
já existente na string que entra no shaper. Não inferir intenção histórica.

### Decisão e limite

Correção de paridade interna em fluxo contínuo ADR-0127: reutiliza contrato
TextReplacement aprovado, sem campo, assinatura pública, flag ou mudança
de fase. Não modifica seleção de quebras ou desenho. A causa explícita é
o U+00AD existente no texto autoral, não aparência, GID ou posição.
Casos estreitos já têm divergências de seleção e não fecham a política SHY.

cluster_text não põe U+00AD autoral no mapeamento global por GID:
remover somente esse codepoint das substrings de cluster usadas para ToUnicode.
Omitir a entrada se sua substring não vazia consistia apenas em SHY; não
reservar o GID compartilhado com espaço com hex vazio. Manter fronteiras
UTF-8, marcas repetidas, ligaduras, RTL e pares substitutos existentes.
O texto lógico completo permanece no envelope por ocorrência do shaper.
A identidade não é inferida de avanço zero nem de um número específico de GID.
SHY em cluster compartilhado com letra também deve ser retirado do mapa
global, conservando a letra. Não apagar outros controles Unicode além do conjunto bidi especificado abaixo.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.

## Controles bidi autorais — identidade de espaço (2026-10-06)

### Medição anterior à decisão

`lab/results/construction/diagnostics/controles-bidi-pdf-2026-10-06/before.json` registra
HEAD, working tree, fontes de código/L0 e binário anterior. As sondas
`bidi-contexto-rtl_literal_embedding-cmap-before.json` e
`bidi-contexto-rtl_forced_mark-cmap-before.json` mostram CID 0001 associado
respectivamente a U+202C e U+200F: espaços posteriores extraem controles e
fundem palavras. A extração bilateral com fontes e comandos está no lote
`lab/results/construction/diagnostics/block-align-2026-10-06/verification.json.gz`.
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:992-997`, pin
9f2b6e8715237cb086899a42873660fe744622e8, remove placeholders Default_Ignorable
após shaping. `03_infra/src/export/fonts.rs:307-328` mantém controles na
substring global por GID; `03_infra/src/export/builder/fonts.rs:523-535`
retém o primeiro mapeamento. São observáveis de texto e geometria, não
paridade de bytes ou obrigação de copiar o algoritmo upstream.
Hipótese refutável por persistência da colisão após remover placeholders e
controles apenas do mapa visual. Não inferir intenção histórica.

### Decisão interna e limites

Correção interna de paridade ADR-0127, sem campo/assinatura pública, fase,
flag, default de produto ou unsafe novo. As invariantes compartilhadas pertencem ao núcleo bidi-visual-identity pinado acima. SHY conserva envelope ActualText existente por ocorrência.
Controles bidi não ganham envelope ActualText: o vanilla não os copia como
identidade dos espaços nestas sondas. Aceitação exige espaços/letras separados,
clusters Unicode preservados e controle visual; extratores podem inserir seus
próprios marcadores de direção, logo texto bruto não prova identidade autoral.

Este owner remove apenas SHY e o conjunto bidi das substrings visuais de
cluster para ToUnicode. Omitir clusters exclusivamente desses controles sem
reservar GID compartilhado. Preservar espaços, letras, UTF-16 e marcas.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`03_infra/src/export/fonts.rs` nas linhas [511] contém as expressões apontadas pelo inventário
Clippy. `lab/results/construction/diagnostics/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

No fixture de coleta Unicode, construir EcoString uma vez e usar esse valor diretamente no campo text. Retirar somente o Into identidade; manter texto, glyphs, fonte, geometria e asserções.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.

## Referencial de pintura — transporte aprovado T1b — 2026-10-14

### Medição anterior à decisão

O manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
foi aprovado pelo humano para T1–T3; recibo em
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A evidência bilateral de Parent e controles Solid está no contrato aprovado;
ela não é substituída por inspeção estrutural. A rodada
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` conserva
fontes, estado sujo, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, UTC,
diffstat e hashes dos binários; cobre contêineres, Place e Rotate com Solid
e Gradient Parent. O inventário before-source/syntax-nodes registra neste
consumer as construções/desestruturações antes da edição: [550].
layout_types.rs:531–539 não distingue raízes de grupos técnicos; helpers.rs:253–269
reconstrói o Group e precisará preservar o campo. Box agrega fundo/corpo;
Block mantém body plano exceto no clip. Preservação do referencial é requisito
de linguagem; formato dos envelopes é mecânica. A aprovação não fecha as
divergências gerais de geometria previamente medidas em Rotate/Box.

### Responsabilidade exclusiva

Migrar construções e padrões de Group para o campo paint_root aprovado. Grupos técnicos novos e fixtures legadas são false; reconstruções transportam o valor recebido, sem inferir raiz a partir de dimensão, clipping ou transformação. Preservar os demais campos e o comportamento de desenho/recursos. Exportadores ainda não usam este metadado para patterns; essa integração é T2 e não está certificada aqui.

### Aceitação e incompletude

L0 primeiro, testes RED→GREEN, resselo restrito; conservar geometria e texto
nos controles fonte→PDF/SVG e revalidar suíte/build/doc/check/fmt/linhagem.
A migração deste metadado não substitui Shape.fill/stroke por FramePaint nem
implementa patterns PDF/SVG/raster; T1c/T2/T3 continuam explicitamente abertos
e já autorizados. Não alegar paridade de tiling ou produção a partir do build.


## Transporte preparado em Shape — T1c/T2 aprovado — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` registra
HEAD, working tree, diffstat/UTC e fontes/owners antes desta migração.
layout_types.rs:510–511 ainda transporta Paint/Stroke; shape.rs:151–191
materializa só fill em grupos; tiling.rs:85 limita/clona a grade. O preparador
prepare_cell já retorna TilingFrame uma vez por ocorrência. As sondas
`tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
comprovam que dense-open e stroke-open ainda divergem nos três targets.
O contrato foi aprovado no manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
recibo em `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
A preparação/contagem de objetos é mecânica de transporte; render, contornos,
texto, geometria e diagnósticos são obrigações observáveis de linguagem.

### Responsabilidade exclusiva desta unidade

Coletar codepoints, glyph IDs, clusters e mapeamentos também das células preparadas para subsetting correto. A coleta de recursos não insere conteúdo semântico na página.

### Aceitação e continuidade

Aplicar o núcleo pinado. A ratificação supersede cláusulas antigas que
aguardavam aprovação ou exigiam Paint/Stroke declarativo dentro de Shape,
somente no âmbito do contrato aprovado. Preservar os demais comportamentos
vigentes desta unidade. L0 primeiro e testes RED→GREEN; migração explícita de
fixtures e consumidores, sem conversão total From<Paint>, sem PartialEq nos
carriers e sem fallback sólido para obter build verde. Não elevar o limite
antigo nem transferir layout para L3. Validar transporte fill/stroke, recursos,
clipping, Parent afim, alvos nativos, controles Solid/Gradient, semântica de
página e toda a suíte/build/doc/fmt/linhagem. T1c, T2 e T3 só estão concluídos
quando integrados e verificados; estado intermediário quebrado ou parcial
não é certificação de produção. T4 e publicação continuam fora deste pacote.
