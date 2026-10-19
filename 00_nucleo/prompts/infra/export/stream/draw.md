# L0 — stream/draw
Hash do Código: 15b337b4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/pdf-conic-placement.toml sha256:9010358e51cfa3267960432244b972b179d447f572c902101eba650f6d6a67cb
- 00_nucleo/prompts/_nuclei/pdf-group-local-space.toml sha256:492b4c76d690fad97626f180997765e9ae130f2eb0df0c003fd0332839010274
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:8b7fcfaf39e60540d16868e662d48fb466fab58860385fcdae499f8c2edfe799
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee



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

## Clip efetivo de imagem — correção de paridade 2026-10-08

Medição bilateral do `image.typ` (JPEG 1×1 em target 80×80 pt): os bytes
extraídos do XObject são idênticos, mas a rasterização a 144 dpi difere em
321 pixels nas bordas direita/inferior. O stream vanilla pinado emite a
matriz `80 0 0 80 8 40 cm` sem clip; o Tekt emite a mesma matriz precedida
por `8 40 80 80 re W n` (`draw.rs:284-293`). O owner L1 conserva
`clip_rect` para fit cover mesmo quando a imagem transformada ocupa
exatamente o target. Classificação: render observável; a hipótese de clip
redundante é refutável por raster inalterado ou por regressão de crop real.

No desenho PDF de Image, omitir o clip apenas quando o retângulo da imagem
transformada já está integralmente contido no `clip_rect`. Emitir o clip
quando qualquer aresta do desenho o ultrapassa; manter matriz EXIF,
posição, recursos, alt, tags e layout. Comparar geometria real em pontos,
sem epsilon que apague crop pequeno. A mesma decisão de clip efetivo deve
valer nos caminhos top/local equivalentes; não alterar carrier público,
default fit, fase ou formato de bytes do JPEG. Exigir raster bilateral do
controle sem crop, teste positivo de cover com crop e regressões de EXIF.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/stream/draw.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/core/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Delegar TextSeparator ao owner 00_nucleo/prompts/infra/export/stream/separator.md em coordenadas top/local, com reflexão local existente; nenhuma soma de advance.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

Em UA1, TextSeparator recebe o mesmo tratamento de folha textual Span e MCID coordenado, preservando enclausuramento e semântica do parágrafo.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/core/entities/layout_types.md`.

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
`lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/envelope-probe/`
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

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.


## Face efetiva de TextShaped — materialização ratificada 2026-10-05

Medição anterior à decisão: `lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distingue GIDs da regular de seleção Oblique. O campo público `TextShaped.font: Option<Font>` do owner layout_types foi confirmado pelo humano (SHA f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409). O snapshot e inventário desta rodada estão em `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

Este owner migra apenas suas referências: constructors legados usam None; transformações preservam a fonte; comparação/junção shaped discrimina face e eixos. Emissão do shaper fornece clone da fonte efetiva também após cache hit; não alterar semântica/estilo para induzir seleção. Collectors/subset/CMap e PDF usam binding da mesma face original com coordenadas do estilo. PNG/SVG preparam bindings locais e instanciam uma vez por face/pedido antes de render, mantendo índice de recursos original distinto dos bytes instanciados. None segue resolução legada. API pública e FontKey existentes permanecem. Não criar fontes artificiais, unsafe, globals ou fase nova. Funções privadas podem receber fonte/bindings explicitamente.

A associação é por bytes da face original e pedido de eixos; nomes e slots não bastam. A tabela local dura a exportação e conserva ordem dos recursos fornecidos, anexando faces faltantes. Para multifont, distribuir itens pelos bindings uma vez antes de coletores por fonte; não reutilizar GIDs/CMap globais em faces diferentes. Código deste owner continua 1:1 com seu L0; referências a outros owners não compartilham propriedade. Aceitação RED→GREEN e controles bilaterais incluindo miss/hit/VF, sem promessa de ganho de tempo.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.

## Espaço literal terminal em embedding LTR (2026-10-06)

### Medição anterior à decisão

`lab/results/construction/diagnostics/espaco-terminal-2026-10-06/before.json` conserva
working tree, HEAD, código/L0 e binários. O caso literal-terminal-space no
lote bidi-identidade-ocorrencia tem glifos posicionados corretamente, mas
caixa extraída de 123 termina em 121.49581pt versus 125.000002pt no oracle.
O PDF emite o carrier de espaço em 121.50342pt; separator.rs:10-14 apenas
emite U+0020, que Poppler descarta no fim do run. O vanilla pinado 9f2b6e871
usa ActualText 0020202C nesse espaço, encerrando o embedding de exportação.
`pdf-boundary-probe.json` mede que apenas ActualText 0020 não resolve, mas
0020202C conserva a caixa até 125.00142pt sem mudar glifos/avanços. A sonda
alternativa word-envelope-refutation.json refuta envolver o texto anterior
com o espaço cruzando restaurações gráficas: a extração muda de linha.
São identidade textual/geometria observáveis; não obrigação de copiar bytes
ou reproduzir todos os operadores. Hipótese refutável por perda/duplicação de
espaços, alteração da tinta ou regressão de markup/base LTR após a correção.

### Classificação e limites

Correção interna ADR-0127, sem campo/API pública/fase/flag/dependência/unsafe.
U+202C é formatação derivada **somente do PDF ActualText**, para fechar o
embedding de exportação comprovado pelo span; não conteúdo autoral, não
carrier, glifo, input bidi nem ToUnicode global. Não fabricar controle virtual
na representação autoral. L0 de paragraph-bidi-origin permanece integral.

### Responsabilidade exclusiva

Nos dois despachos de TextSeparator, repassar origin e bidi.as_ref() para o
owner separator, preservando x/PDF-baseline e reflexão local. Não certificar
segmentos aqui nem inserir marcações/texto virtual no documento. Corpo segue
desenhado uma vez; streams/scopes e tagging existentes permanecem.

## Transformações e recortes em grupos aninhados — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/stream/draw.rs:530-565` aplica posição, matriz e
clip ao Group de página; `:947-952` apenas visita filhos no caminho
local, descartando esses três campos. O contrato Recursive painting
acima já exige aplicá-los uma vez. O teste histórico
`03_infra/src/export/tests/mod.rs:8306` verifica só presença de texto,
sem medir sua posição. Código e L0 anteriores, HEAD, working tree não
commitado, status, diffstat, UTC e binário estão em
`lab/results/construction/diagnostics/grupos-aninhados-2026-10-13/before.json.gz`.
A investigação nasceu do controle independente de cor única
`lab/results/construction/diagnostics/conic-cobertura-2026-10-13/probe.json.gz`;
o defeito de posicionamento de Pattern é distinto e não se fecha aqui.

### Decisão interna e aceitação

No Group local, salvar o estado gráfico, concatenar a matriz do grupo
com a translação pos + (tx,ty), aplicar clip_mask nas dimensões internas
via emit_shape_path_local, visitar filhos e restaurar o estado. Não
refletir Y outra vez: o grupo de página já estabeleceu o espaço Y-down.
Não substituir a matriz afim por bbox nem perder rotação, reflexão,
escala ou cisalhamento. Clip fica dentro do mesmo envelope q/Q e não
afeta irmãos. Preservar ordem, bbox contextual e envelopes semânticos
e de texto já existentes. Não alterar o dispatcher de página.

Correção interna de uma omissão, ADR-0127 em fluxo contínuo: nenhuma
assinatura/campo público, modo, flag, fase ou novo default. Antes do
código, acrescentar testes RED que interpretem os operadores emitidos
e confiram pontos transformados contra resultados aritméticos
independentes, recorte antes da pintura e restauração para irmãos.
Cobrir Compact/Verbose, composição não comutativa em dois níveis,
escala, rotação e posição; exigir GREEN sem relaxar os esperados.
Revalidar suíte infra, build, formato e linhagem. Não chamar presença
de cm/texto ou contagem de testes de prova de geometria/paridade.

As invariantes compartilhadas de coordenadas e escopo gráfico pertencem ao
Núcleo pdf-group-local-space pinado acima; este owner materializa somente
a responsabilidade descrita para seu consumer exclusivo.

### Resultado nomeado no teste de geometria

O primeiro inventário registrou type_complexity em draw.rs:1246 para
a tupla de retorno do interpretador de testes (first-validation.json.gz).
Usar uma struct privada de teste com campos points e rectangles, sem
mudar operações, fixtures, valores esperados ou cobertura. Não suprimir
o aviso nem alterar tipos produtivos.


## Auxiliares de teste e bindings produtivos — 2026-10-14

### Medição anterior à decisão

`03_infra/src/export/stream/draw.rs` nas linhas [324, 325, 983, 984] apresenta os avisos selecionados.
`lab/results/construction/diagnostics/ci-helpers-testes-2026-10-14/before.json.gz`
conserva fonte/owner, HEAD, working tree não commitado, UTC e diffstat.
references-before.json.gz registra chamadas, imports, módulos e referências;
o estado da suíte/inventário anterior foi verificado por hashes. A lista de
testes foi capturada antes da edição. Não se infere inutilidade funcional
apenas de dead_code: as chamadas privadas e sua compilação foram auditadas.

### Responsabilidade e aceitação

Nos patterns Image top/local, não criar bindings intrinsic_width/intrinsic_height sem uso; esses campos continuam no carrier e são cobertos pelo .. já existente. Desenhar com width/height resolvidos, posição, orientação e clip vigentes, sem recomputar tamanho a partir de pixels ou modificar recursos, matrizes e branches.

Manutenção interna em fluxo contínuo ADR-0127: nenhuma API pública, campo,
default de produto, fase, dependência ou supressão de lint muda. O gate de
diagnósticos selecionados deve falhar antes e passar depois; ele não é um
novo teste funcional nem prova de paridade. Preservar a lista e as expectativas
dos testes existentes, executar a suíte completa, compilar lib/all-targets,
verificar build/rustdoc/fmt/linhagem e registrar o inventário total. Não criar
testes que apenas espelhem cfg ou remoção de bindings. Resselo restrito ao owner.

## Posicionamento cônico — 2026-10-14

### Medição anterior à decisão

`03_infra/src/export/gradients/mod.rs:224-264` perde a posição própria de
Conic RGB/CMYK e deduplica por Arc/bbox; `builder/resources.rs:564-574`
emite matriz identidade nesses ramos. `stream/draw.rs:522-555,939-965`
compõe grupos, mas Pattern é relativo ao espaço inicial do stream.
`stream/page.rs:45-49` acrescenta bleed antes dos itens. O corpus constante
red versus conic(red,red), em before/probe.json.gz, reproduz a perda de
cobertura na árvore atual. Fonte, L0, HEAD, UTC, status e diffstat estão em
`lab/results/construction/diagnostics/conic-posicionamento-2026-10-14/placement-before.json.gz`.
Os dois lados do teste são Tekt: o defeito independe de imitar o vanilla.

### Responsabilidade e aceitação

Passar a referência do FrameItem Shape atual aos helpers internos de pintura em top/local, incluindo fill e stroke. Manter coordenadas, ordem, transformações de grupos e recortes; a matriz do Pattern é preparada fora deste owner.

Correção interna ADR-0127, sem entidade/assinatura pública externa, flag,
default ou fase nova. L0 precede testes RED e implementação; resselo restrito.
Testar posição e dimensão, mesmo Arc em ocorrências/páginas distintas,
fill/stroke, grupos afins aninhados, bleed, parent explícito/local/página,
recursos e pintura nos modos Compact/Verbose. Repetir o corpus raster sem
recortar ou realinhar o resultado.

Esta etapa permanece incompleta quanto à funcionalidade cônica integral:
a etapa seguinte, conic-geometria-amostragem, deve completar centro autoral,
cobertura dos cantos e amostragem CMYK/offsets. Não atribuir cobertura total
à correção de posicionamento nem legitimar o círculo inscrito como resultado
final. O owner conic conserva a malha nesta etapa para isolar a causa.

Refinamento da composição privada: os helpers de paint recebem o PageContext
já existente em vez de dois parâmetros separados para mapa/referências.
Isso transporta os mesmos borrows sem aumentar a lista de argumentos; nenhum
campo ou construtor de PageContext muda. O dispatcher passa seu contexto.

## Caminho de forma sem pintura — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/block-regiao-2026-10-14/open-pdf/results.json.gz`
mede um backing azul extra em Block(outset:4pt) sem fill/stroke, somente no
PDF Tekt. A inspeção visual da amostra confirma a pintura indevida. O corpus
`lab/results/construction/diagnostics/pdf-caminho-vazio-2026-10-14/before-full/results.json.gz`
conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree sujo,
status, diffstat, UTC, fontes, comandos e hashes de binários. Rect/Ellipse/
RoundedRect invisíveis seguidos de marcador azul põem tinta desde (20,20) no
PDF Tekt, normal e compacto; SVG Tekt e vanilla pinado
9f2b6e8715237cb086899a42873660fe744622e8 só pintam o marcador (70,60).
Os dois ramos `draw.rs:513-517` e `:922-926` constroem o caminho mas nada
emitem para (fill=None,stroke=None), antes de Q. Hipótese causal: o caminho
pendente é consumido pela pintura/recorte seguinte. Refutadores: isolamento
top/local/grupo, pintura fill/stroke/ambos e máscara seguinte inalterados
apesar de encerrar o caminho. Isso é geometria visível de linguagem, não
obrigação de igualdade de bytes nem inferência de intenção upstream.
O caso polygon(stroke:none) é rejeitado antes do exportador no Tekt e fica
registrado como defeito distinto, sem crédito de paridade deste lote.

### Responsabilidade exclusiva e correção interna

Nos dispatchers top/local, terminar sem pintar o caminho construído de uma
Shape cujo fill e stroke sejam ambos ausentes. Preservar envelopes gráficos
e semânticos, ordem de pintura, resource/tag bookkeeping, transformações,
primitivas e os caminhos de fill/stroke efetivos. O ramo sem pintura não pode
contaminar nem a tinta nem a máscara da ocorrência seguinte, inclusive ao
sair de Group. Não descartar documentos, converter ausência em preto nem
alterar o layout produtor para esconder a falha do exportador.

### Aceitação

Correção interna ADR-0127, L0 primeiro e RED→GREEN sem gate público: nenhum
campo, assinatura pública, default, flag, dependência, fase ou unsafe novo.
Os testes no mesmo consumer observam quais retângulos participam da pintura
e do clip em um interpretador independente do estado de caminho PDF, com
valores esperados próprios; não exigem uma string exata ou operador específico
se outra implementação cumprir a geometria. Cobrir top/local, escape de Group,
Compact/Verbose, fill/stroke/ambos e clip posterior. Repetir PDF/SVG bilateral,
controles positivos/sem pintura, qpdf e regressões existentes, suíte completa,
build/doc/fmt/check, inventários e linhagem. Resselo somente deste owner.
Outset/overflow de Block, espessura de sublinhado e aceitação de polygon None
continuam com seus owners e suas evidências; não são fechados por esta correção.

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
consumer as construções/desestruturações antes da edição: [523, 934, 1299, 1314, 1353, 1361, 1445, 1497].
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

### Integração PDF ainda refutada nesta etapa

first-render-delta.json e after-first/results.json.gz em paint-root-2026-10-14
registram uma regressão no controle block-auto-gradient-parent/PDF: 3840
pixels mudam mais de 16 níveis contra o baseline e a distância ao pin cresce.
Os demais 35 renders Tekt do corpus mantêm pixels. O collector legado de
gradientes troca parent_bbox_override a cada Group; o novo envelope de raiz
expõe que esse override não é um referencial afim. Os campos de Shape ainda
são declarativos. Essa diferença é falha aberta, não nova referência aceita,
e não autoriza mudar tolerância ou o esperado. T1c/T2 devem fechar o contexto
resolvido de pintura e os recursos para corrigir a causa; T1b isoladamente
não certifica render nem produção. Preservar a amostra refutadora e também
os controles que coincidem, inclusive a divergência vanilla PDF de quarter.


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

Migrar fixtures de FrameItem::Shape para FramePaint/FrameStroke, preservando valores e asserts semânticos existentes. Tiling deve construir célula preparada explícita; fixtures de avaliação continuam declarativas. Não adicionar conversão produtiva total nem enfraquecer asserções.

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

## Composição de período e alpha sólido — 2026-10-14

### Medição anterior à decisão

`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserva fontes, owners, HEAD, UTC e working tree. `tiling-nativo-2026-10-14/final-renders` e `remaining-analysis.md` registram cell8×8/pitch4×4 alpha50%: PDF vermelho opaco; SVG só uma contribuição; PNG quatro contribuições (255,15,15). `pdf_tiling.rs:184` emite BBox=size, `svg/paints.rs:148` usa overflow visible; `stream/shape.rs:155` e `stream/text.rs:275` descartam alpha. São defeitos observáveis de composição; o vanilla também corta overlap e não serve de requisito para esse defeito. Hipótese refutável: compor explicitamente um período preserva controles sem overlap e quatro contribuições coincidem com fonte-over analítica.

### Responsabilidade desta unidade

Abrir q/ExtGState somente para folhas translúcidas, fechar após a folha sem contaminar a próxima. Top/local preservam ActualText: o número privado de Q pendentes inclui o envelope adicional e só é descarregado depois de EMC, sem busca textual de operadores. Aplicar também a glifos marcados que passam por caminho direto. Não alterar transformações nem marcações opacas.

### Aceitação

Correção interna ADR-0127 dentro do T1–T3 já aprovado, sem nova API/fase/flag/default. L0 primeiro, RED→GREEN, composição analítica (incluindo ordem com cores distintas), controles simples/nested/rotated/text/image e extração/seleção de páginas. Build, suíte workspace, formato, Rustdoc e linhagem. Alpha variável de gradientes e qualidade PNG seguem incompletos; não certificar pacote de produção com esses itens abertos.


## Simplificações locais de Clippy — 2026-10-15

### Medição anterior à decisão

`03_infra/src/export/stream/draw.rs:647,1205` empresta novamente opening, já ligado por referência no ramo Some.
`lab/results/construction/diagnostics/avisos-clippy-tekt-2026-10-15/before.json.gz`
conserva fontes, owners, HEAD, working tree não commitado, UTC e diffstat;
selected-red.json identifica os diagnósticos do compilador antes da edição.
São expressões Rust equivalentes, sem nova medição de paridade upstream.

### Responsabilidade e aceitação

Repassar opening diretamente a push_str nos dois despachos Semantic. Conservar exatamente os mesmos envelopes, texto, ordem e balanceamento, incluindo UA1 e modos top/local.

Manutenção interna em fluxo contínuo ADR-0127: conservar contratos públicos,
defaults e fases. Não adicionar supressões nem testes que espelhem sintaxe.
O gate dos avisos selecionados deve passar a GREEN; validar a suíte existente,
build, Clippy all-targets, formato, documentação e linhagem com resselo restrito.


## FrameItem — exaustividade de decisões existentes, V16, 2026-10-16

### Medição anterior à decisão

F057: 03_infra/src/export/stream/draw.rs:114 (ua1_leaf_opening). Os spans AST, patterns/guards/corpos e complements foram conferidos antes do código. `lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat. `classification.json` registra os 70 casos e seus complements em 26 owners, usando a árvore do parser pinado do CI. Cláusulas pertinentes do L0 foram lidas junto da fonte; não há proposta de nova arquitetura ou interpretação de intenção upstream. Padrões e exaustividade Rust são mecânica; semântica/morfologia, geometria, efeitos e diagnósticos do produto devem permanecer.

### Prescrição exclusiva desta unidade

- **F057 — `ua1_leaf_opening`**: Trocar somente o padrão '_' pelo grupo nominal `TextReplacement`, `Semantic`, `Line`, `Shape`, `Group`, `Link`. Usar o mesmo prefixo Rust em escopo (`FrameItem`). Conservar integralmente o corpo atual do fallback, os braços anteriores, guards, transformações, chamadas, returns e ordem.

Manter callbacks, efeitos, metadados, clipping, fontes, positions, tinta e largura conforme os owners vigentes. Esta rodada não amplia travessias, não transforma wrappers em folhas nem usa text/ActualText como substituto da geometria. Helpers de teste mantêm fixtures, esperados, tolerâncias, contagens e falha Unknown; não aproveitar o saneamento para aceitar ausência de observação. Grupos nominais compartilham o mesmo corpo anterior: notas V19 de alternativas continuam visíveis, sem confundir cobertura de um braço com todas as alternativas.

### Gate e aceitação

Manutenção interna ADR-0127 em fluxo contínuo: nenhum enum/campo novo, método de trait/assinatura pública, default, dependência, compatibilidade ou fase nova. Não materializar propostas históricas pendentes por este adendo. L0 primeiro e resselo restrito em ambas as direções. Não adicionar supressões, alterar limiar ou reescrever match como if-let para esconder V16.

Antes do código, executar o controle novo de FrameVisitor e a suíte workspace nos mesmos esperados, e confirmar RED dos 70 avisos da família por leitura AST. Após código, zero V16 dessa família nos locais abrangidos e nenhum aviso novo; manter V16 externos como pendências. Nos matches do core produtivo alcançados por cargo check -p typst-core, usar cópia descartável com uma variante sentinela de FrameItem para medir ausência de E0004 nos locais antes e presença depois. A sonda cobre somente os locais efetivamente compilados: não extrapolar aos helpers cfg(test) ou L3 que a dependência impede de alcançar. Para todo o lote, conferir padrões/guards/corpos por replay, enum e assinaturas inalterados, build e testes reais. Não substituir corpos por stubs.

Revalidar suíte workspace completa, cargo build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF e diff --check. Fontes/L0/configuração ficam congelados durante comandos. Execuções pesadas sequenciais com MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não apresentar saneamento estrutural como nova paridade global, ganho de memória ou benefício medido para IA.
