# L0 — stream/draw
Hash do Código: dd9612f3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/draw.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/draw.rs:26` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Recursive painting

Own exhaustive static draw_item_top/draw_item_local dispatch and recursive handling
of Text/TextShaped/Glyph, Line/Image/Shape/Group/Link/Semantic. Preserve paint order.
Top-level uses page_height - y; top-level Link children use that same convention.
Group applies position/matrix and clip once, then uses local drawing. Local text
uses the existing reflection envelope q / 1 0 0 -1 0 0 cm / Q with (x,-y), restoring
the text helpers' coordinate precondition; line/shape/image keep their own conventions.
Images compose resolved size/position with EXIF 1-8 once at the existing precision.

With tags disabled, Semantic is visually transparent. With tags enabled, draw
Formula/Lbl/TH/TD from supplied MCIDs and balance BDC/EMC; alt is never painted.
Artifact Other uses BMC. Header emits `/Artifact << /Attached [/Top] /Subtype /Header
/Type /Pagination >> BDC`; Background emits its existing Type Background property
list `/Artifact << /Type /Background >> BDC`. Other artifact variants retain BMC. These branches do not select properties
by PDF version and consume no MCID. Do not manufacture a StructElem here. In UA1, meaningful leaves
use Span/Figure while geometric decorations use Artifact Layout. Boundary carriers
paint no empty content. Recurse through each child exactly once.
Delegate text and geometry to their children; resource IDs and ParentTree belong
outside this owner. Unsupported artifact-version/AT behavior remains unproven.

## Verification

Check top/local coordinate composition, nested reflection, clips, asymmetric EXIF
images, balanced semantic/artifact envelopes and UA1 boundary/leaf behavior.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/stream/draw.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Delegar TextSeparator ao owner 00_nucleo/prompts/infra/export/stream/separator.md em coordenadas top/local, com reflexão local existente; nenhuma soma de advance.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

Em UA1, TextSeparator recebe o mesmo tratamento de folha textual Span e MCID coordenado, preservando enclausuramento e semântica do parágrafo.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Delegar abertura/fecho ActualText ao owner 00_nucleo/prompts/infra/export/stream/text_replacement.md e desenhar filhos com o mesmo dispatcher top/local. Não incluir envelope como nova folha MCID, nem alterar seus recursos.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Montagem Large: marcação por ocorrência em glifos compostos

Quando TextReplacement possui somente filhos Glyph já posicionados, o
texto causal do envelope abre ActualText depois do BT da primeira peça e
fecha antes do ET da última. O dispatcher mantém um único escopo marcado,
desenha cada filho uma vez com os seus GIDs/posições/estilos e conserva as
marcações de folha e MCIDs aplicáveis. Outros TextReplacement, inclusive
hífen discricionário com Text/TextShaped, mantêm o caminho genérico atual.
A sonda PDF com o mesmo `∫` em três partes mede extração simples, `-layout`,
caixas positivas e raster em Compact e Verbose; não basta encontrar o token
ActualText no stream. Sem heurística de GID ou varredura posterior do stream.

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
