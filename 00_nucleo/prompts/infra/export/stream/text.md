# L0 — stream/text
Hash do Código: b0d3c9c9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/pdf-shaped-advances.toml sha256:d6da63fc3b0282aaeb2573e92e850f807a2691c04d3578c54e61c1f4e01a51f3
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:3b0c535436048536ecd3133fcf8299e88c128a62c222661f36b7bcc6d98a598c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/text.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/text.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Text and glyph operators

Own font lookup by family/variant/variations and Text/TextShaped/Glyph emitters in
Compact/Verbose. Type1 retains WinAnsi fallback. CIDFont/Multifont use supplied
old-to-new GID mappings; direct mathematical glyphs use their per-font reverse map.
Paint bitmap glyphs through prepared XObjects and placement metadata, not outline
fonts. Do not perform shaping, font instancing or resource allocation.

TJ uses (nominal - x_advance) / units_per_em * 1000 and compensates x_offset with
the existing paired adjustments, rounding and units. Preserve the current y_offset
scope without introducing new operators. Verbose uses balanced q/cm/color/BT/Tr/Tf/
Tc/Tm/TJ-or-Tj/ET/Q envelopes, explicit rendering mode, faux bold, tracking and spot
colors. Compact retains its current operator form. These helpers serve both top
and local dispatchers, whose coordinate composition is owned by draw.

Coalesced-run compatibility, boundary selection and rounded inter-item TJ
compensation belong to `00_nucleo/prompts/infra/export/stream/text/runs.md`.
Declare the descendant runs module and preserve the internal reexports used
by the stream/page dispatcher. This owner retains single-item text/glyph
operators and their shared envelope helpers; do not duplicate run decisions.

## Verification

Exercise kerning, offsets, remapping, same-GID multifont identity, bitmap placement,
mode envelopes. Run boundaries and mathematical/UA1 noncoalescing are
verified through the runs owner and existing integration regressions.

## Substituição e restauração gráfica — refinamento medido

### Medição anterior à decisão

O primeiro build SHA-256 8b223826b2a6ec44bf95a82235e5caaae57777eba27be27200e084e3ce389241
(`after/`, `after-extra/`) conserva SHY no compacto, mas no verboso a extração
coloca os substitutes em x=0/y=400. O experimento controlado de 18 streams em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/envelope-probe/`
refuta a abrangência inicial: fechar EMC antes de Q conserva caixas; fechar após
Q perde a posição no Poppler. Abrir antes/depois de cm/Tm não muda essa conclusão.
Não fechar por contagem de codepoints, nem reescrever streams procurando Q.

### Decisão interna e responsabilidade

Formatter textual conserva suas assinaturas existentes e fornece variantes
privadas com tail de restauração gráfica diferível: emitir todo o texto/ET,
retornar se há Q pendente em vez de restaurar nesse momento. Type1 considera
somente o q de faux-bold; fontes embutidas no verboso têm q próprio; bitmap
não cria tail textual. O dispatcher pede essa forma apenas ao último filho
visual de TextReplacement. Ele fecha as marcações de folha, depois ActualText,
e só então emite Q pendente. Filhos anteriores e itens normais mantêm emissão
existente. Compact/Verbose preservam seus operadores e transformações, sem
troca de modo ou heurística sobre o texto dos operadores.

É refinamento privado de paridade, sem contrato público/fase/flag nova. O owner
text possui formatação/tail, draw possui despacho e marcações de folha, e
text_replacement possui exclusivamente a codificação/BDC/EMC. Aceitação requer
extração textual na posição original em top/local e ambos os modos, coexistência
literal, scopes balanceados e raster preservado.

## Montagem Large: posição de ActualText no estado Glyph

A sonda bilateral `math-large-identidade-2026-09-30` usa o vanilla pinado e
NewCMMath com DisplayOperatorMinHeight=3000du. Envolver as três peças de `∫`
em ActualText fora dos estados gráficos devolve o caractere na extração simples,
mas Poppler mede caixa de altura zero no Verbose e coloca a ocorrência antes de
`∑` em `-layout`; o Compact também inverte a ordem. Experimento de PDF temporário
com as mesmas peças e posições, sem mudar GIDs, refuta o escopo exterior: abrir
BDC após BT da primeira peça e fechar EMC antes de ET da última recupera caixa
positiva e a ordem `∑`, `∫` em ambos os modos. Não inferir texto de GIDs.

Oferecer variantes internas dos emissores Glyph que aceitam abertura/fecho
causais fornecidos pelo dispatcher, reaproveitando integralmente seleção de
fonte, remapping de GID, cor e posições dos emissores normais. Só o ponto de
inserção de BDC/EMC muda para peças de um TextReplacement de glifos. Não
alterar o formato dos glifos comuns, o modo default, o caminho Type1 nem
acrescentar texto invisível. O helper de marcação continua pertencendo a
`text_replacement.md`; este owner só posiciona as chamadas no bloco Glyph.

## Tracking shaped — soma única e estado PDF

### Medição anterior à decisão

`00_nucleo/diagnosticos/shy-autoral-2026-09-28/tracking-controls.json`
mede plain-verbose com desvio máximo 2.652pt contra vanilla pinado,
enquanto plain-compact tem 0.348pt. Estado, binários e comandos estão no
recibo daquela frente. O novo estado e a matriz estão em
`00_nucleo/diagnosticos/tracking-pdf-2026-09-28/initial.json` e `before/`.
`03_infra/src/shaper/shaping.rs:319-326` soma tracking aos avanços;
`03_infra/src/export/stream/text.rs:639,668` e
`03_infra/src/export/stream/text/runs.rs:107` emitem Tc novamente.
O cursor de runs considera os avanços e ajustes TJ, sem essa soma Tc.
O vanilla pinado aplica tracking nos glyphs em
`lab/typst-original/crates/typst-layout/src/inline/shaping.rs:1269-1292`.
É comportamento de espaçamento medido, não requisito de operador ou bytes.
A hipótese é refutável por avanços sem tracking ou ausência do Tc adicional.

### Decisão de paridade interna

TextShaped CIDFont/Multifont carrega os avanços autoritativos já ajustados:
tracking deve vir desses avanços somente. Tanto emissão single quanto
coalescida estabelece explicitamente Tc=0 dentro do bloco BT antes de TJ,
inclusive tracking ausente/zero, para eliminar estado herdado. O zero é
o estado normativo de espaçamento adicional, não uma exceção de fonte, GID,
fixture ou largura. Não subtrair tracking dos avanços nem inferir causa
pelo texto. Text e fallback Type1, que emitem Tj sem avanços shaped,
continuam usando seu tracking existente. Bitmap continua desenho de imagem.
Sem contrato público, flag, modo padrão ou fase nova: fluxo ADR-0127.
Arredondamento dos avanços, medição de layout e política de tracking em
ligaduras são frentes independentes; esta correção não os certifica.

### Responsabilidade exclusiva e aceitação

Own helper privado push_shaped_spacing, que emite Tc=0. Chamá-lo no BT
de TextShaped CIDFont/Multifont compacto/verboso, inclusive variante de
restauração diferida. Preservar delta TJ, offsets, remapping, scopes,
transformações e o caminho bitmap. O child runs usa o mesmo helper.
Os testes deste consumer verificam a posição de cada glyph usando um
interpretador mínimo independente de Tc/TJ, com estado Tc anterior não
zero, tracking positivo/negativo/zero/ausente, CIDFont e Multifont. Verificar
também fronteiras absolutas do run fundido e Text/Type1 ainda com tracking.
Sondas PDF exigem equivalência visual compacto/verboso e preservação
lexical SHY/U+002D; progresso contra vanilla deve ser medido, sem fechar
arredondamento já divergente por normalização ou tolerância alargada.

## Correção de tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/tracking-residual-2026-09-29/initial.json`, `results.json`, `summary.json` e `causal-checks.json` registram 36 entradas/108 PDFs contra vanilla pinado, fontes, UTC, HEAD e working tree completa. A palavra isolada acumula +0.052pt com tracking 0.2pt, mas fronteiras perdem um intervalo na adjacência, dois com um espaço e quatro com três espaços literais. ±1.2pt elimina a quantização sem eliminar as fronteiras. O código shaper/shaping.rs:225-232,319-326 arredonda em i32; text.rs:49-83 divide palavras; space.rs:15-33 aplica somente avanço nominal. O estado anterior à materialização está em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29/initial.json`.

### Decisão interna e responsabilidade

Correção interna de paridade em fluxo contínuo ADR-0127, sem contrato público, flag ou fase nova. As cláusulas desta rodada supersedem somente a preservação dos limites de tracking descritos abaixo; demais políticas de render permanecem.

Nos emissores shaped single/bitmap, usar resíduo fracionário derivado do owner tracking. TJ preserva o delta inteiro/rounding anterior de kerning e acrescenta somente o resíduo de tracking; serializar cinco casas quando resíduo existe. X-offset e zero-tracking conservam formato anterior. Private helper TJ pode receber TextStyle; nunca inferir tracking de GID/texto. Bitmap soma o mesmo resíduo ao cursor, sem novo operador textual. Tc=0 segue obrigatório em shaped.

Aceitação: testes RED→GREEN de fronteiras/seleção e interpretação independente Tc/TJ, controles bilaterais de tracking ±/zero, repetidos, adjacência e wrap; build e lint sem erros, resselo restrito.
