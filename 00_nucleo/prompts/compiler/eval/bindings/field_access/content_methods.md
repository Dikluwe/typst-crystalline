# L0 — compiler/eval/bindings/field_access/content_methods
Hash do Código: 7369fe79



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:923f1c9e4c2fd5934316ae2eef3dbb461c3bef3009de1d8931d951e793dda342
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:825182e1dfe3591249a5efbc4c0ddbd937ebffde953c6595e0d1ed8176106435
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:79283c9047419768139e0227b9ed7e34146a85706ed47731655736545c95306b
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:71d4b43f260cff61fffa8ba7831733e7a62f2517109911c8af346458d64cbd5b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`. No HEAD `ba5a4c7d305ed1444660d9eb80862f51b417efb6` e árvore limpa, `01_core/src/compiler/eval/bindings/field_access.rs:1165-1589` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-field-access-2026-09-30-before.json.gz`.

## Contrato

Consultar campos assentes e métodos de Content/LocatedContent, incluindo os wrappers estáticos do tipo. Preservar as assinaturas e semântica de `00_nucleo/prompts/compiler/eval/bindings/field_access.md`; despacho estático na mesma camada L1, sem I/O nem estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de acesso a campos; `crystalline-lint .` passa com ownership e hashes corretos. Não inferir ganho de desempenho.


## Realização com avaliação única

Presença de campos projeta carriers internos; campos não conhecidos/não assentes continuam distintos, sem confundir carrier com elemento nativo.


## Realização com avaliação única

Métodos func/has/at/fields/location projetam o corpo/visual dos carriers. location da ocorrência preparada é observável pela receita; não forçar none quando o carrier fornece Location. A função nativa concreta e os campos originais são preservados.


## Aspa semântica e resolução tardia — ratificada em 2026-10-06

O contrato desta seção é vigente e supersede as cláusulas históricas conflitantes
das seções anteriores apenas para SmartQuote. A ratificação humana autoriza a
materialização em fluxo RED→GREEN, com ownership exclusivo e pins abaixo.

### Medição anterior à decisão

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
registra pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 dos binários,
HEAD/working tree, diff HEAD --stat, UTC, fontes, argv/cwd/ambiente e stdout/stderr.
PDF usa texto extraído, HTML usa DOM/texto e diagnósticos, não igualdade de bytes PDF.
A avaliação converte aspas de markup para Text; campos double/enabled são perdidos.
A variável contendo aspas mantém glyph inglês sob uso em alemão/francês ou
smartquote(enabled:false). Primes 6'/2" viram aspas no Tekt. Show da função
nativa e nome público foram corrigidos separadamente e passam antes deste gate;
markup ainda não é SmartQuote. Constructor vazio tem fields vazio no vanilla;
double:false é campo conhecido, mas double omitido é Unset. enabled:false
permanece SmartQuote no vanilla. set double:false e show filtrado têm positivos
novos. HTML literal funciona porque recebe Text; SmartQuote programática hoje
é rejeitada. Título literal renderiza aspas; script recebe string, rejeitando
Content antes da exportação nos dois binários.

Fontes: markup_stream.rs:79–128 decide glyph em eval; smartquote.rs de layout
alterna dois flags sem texto precedente; stdlib/text/smartquote.rs:112–117 apaga
elemento disabled; observation/elements.rs:242–250 compara os campos atuais.
Vanilla typst-eval/markup.rs:141–146 cria elemento com double explícito;
typst-layout/inline/collect.rs:196–211 resolve estilo/contexto na coleta;
typst-library/text/smartquote.rs:119–158 distingue prime/apóstrofo/nesting.

### Decisão ratificada e limites

Preservar identidade/campos de SmartQuote em eval e resolver glyph só na
composição de render de cada target. Paged usa sua unidade; HTML permanece
semântico e não chama layout paginado para aspas (ADR-0128). A lógica de
seleção contextual é helper puro L1 único, com estado por fluxo/compilação,
sem I/O, globais, vtable, registry de propriedades ou cópia do algoritmo de
memoização upstream. Stack/estado é mecânica escolhida do Tekt; linguagem,
campo conhecido/omitido, glyph e estilo são aceitação. Escaping HTML permanece
com seus owners. Sem novos modos/flags, unsafe, API de watch ou math phase.

Este gate muda double público bool→Option<bool>, adiciona enabled:Option<bool>,
expõe helper/contexto puro e move escolha de glyph eval→render. Confirmação humana recebida em 2026-10-06. O contrato ratificado abaixo
supersede cláusulas históricas conflitantes apenas para aspas; ADRs/núcleos
existentes e propostas alheias permanecem intactos.

Campos declarados de SmartQuote: double,enabled,alternative,quotes. Some é Set;
None é Unset, nome fora do schema Undeclared. fields/has/at expõem apenas Set;
.double omitido fora de contexto retorna o diagnóstico Unset medido, não bool
fabricado. Campos conhecidos explicitamente retornam valores corretos. Preservar
Source/span e captured Some; não mudar campos dos demais elementos. Usar
representação canônica de quotes existente para public Value.

## D07 — correção de morfologia de matriz, fluxo ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
lab/results/construction/diagnostics/matrix-morfologia-2026-10-06/before.json.gz guarda
UTC/diff/fontes/binários e casos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Mat raw/query sai como {func:math.matrix} sem rows; getter/métodos perdem campos
e func-equality retorna false. Fonte upstream math/matrix.rs:91–221 declara
propriedades e rows. Tekt MathMatrixElem conserva rows/explicit em math_matrix.rs:45–52,
mas Element.get_field é default; content_methods.rs:151–191 não enumera mat;
cli.rs:1154–1348 cai no fallback body. É morfologia/semântica da linguagem,
não exigência de estrutura Rust ou bytes PDF; nenhuma intenção foi inferida.

### Responsabilidade exclusiva

Classificar rows como Set; propriedades declaradas delim/align/augment/row-gap/column-gap como Set via getter quando explicit ou Unset; demais Undeclared. Enumerar fields na ordem delim,align,augment,row-gap,column-gap,rows, reutilizando valores do getter. func usa native_math_mat real pelo bridge privado, não stub e não identidade por nome. Não adicionar selector/show/where para mat nesta frente.

### Aceitação

RED→GREEN e coleta bilateral de fields/has/at/func, JSON/query, padding e
presença; build/lint e regressões D05/D06/query. Gate contínuo: contratos
públicos de tipos/traits/assinaturas e fase ficam iguais. Não certificar
mat selectors, render, Styled geral ou outros elementos não medidos.

## D08B1 — projeção existente, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz
registra UTC, diff HEAD --stat, hashes de fontes/binários, argv/cwd e resultados
contra vanilla 9f2b6e8715237cb086899a42873660fe744622e8.
Vanilla model/table.rs:734–778 declara body e propriedades; Tekt
entities/elements/table_cell.rs:62–109 não implementa getter; content/identity.rs:107
usa table.cell; content_methods.rs:226–254 não projeta a função real;
rules/show.rs:300–320 omite a nativa. Body/fields/func-eq e show explícito
refutam paridade. É semântica/morfologia, não igualdade Rust ou bytes PDF.
Não se infere intenção da estrutura upstream.

### Responsabilidade exclusiva

Classificar os dez campos declarados em Set via getter ou Unset, enumerando body,x,y,colspan,rowspan,inset,align,fill,stroke,breakable. func projeta native_table_cell real.

### Limites e aceitação

RED→GREEN e coleta bilateral; cargo build e crystalline-lint sem erros.
Esta correção não muda contrato público de tipos/assinaturas nem fase.
D08B permanece incompleto: D08B2 completa presença omitido/auto/none por
contrato próprio; D08B3 completa realização de posições/estilos/padding/show.
Option None não prova omissão nem none explícito: não certificar esses casos.

## D08B2 — presença lexical ratificada em 2026-10-07

### Medição anterior à decisão

lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz e
presence-refuters.json conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree, diffstat, UTC, binários e argumentos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Upstream model/table.rs:734–778 possui campos Smart/Option; o Tekt
entities/elements/table_cell.rs:40–51 conserva apenas Option de valor.
X:auto e x omitido colapsam; fill:none e stroke:none perdem presença.
Os refutadores medem auto permitido em x/y/fill/align/inset/breakable,
rejeitado em colspan/rowspan/stroke; none permitido só em fill/stroke.
São observáveis da linguagem, sem obrigação de copiar a estrutura upstream.

### Contrato ratificado e responsabilidade exclusiva

Set/Unset segue getter estrito da entidade; campos declarados e ordem permanecem os dez de D08B1. Func permanece nativa real. Nenhum default herdado inventa Set.

### Aceitação e limite

RED→GREEN para body/presença/métodos/JSON, clone/map/observação e rejeições com span. Build/suítes/lint, coletas D08A/D05/D07 e novos refutadores; proveniência exata obrigatória. D08B3 completa realização por posição/estilo/padding/show em proposta separada. Contrato público por campo/tipos novos confirmado pelo humano em 2026-10-07; recibo table-cell-presenca-aprovacao-2026-10-07.json. Não altera fase/defaults de render.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Medição order/table-replacement-before-cells e show-table-body: tabela precisa manter identidade nativa para seleção. content_methods.rs:240–255 tem Cell mas não Table. Adicionar Content::Table para native_table em content_elem_func sem executar constructor; fields/has/at continuam pela projeção do owner, sem novos defaults genéricos de Table. Cell resolvida projeta props conhecidas, raw omission continua Unset, snapshots e ordem permanecem.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.


## D08B3-C2 — responsabilidade ratificada

Medição anterior: lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Fields/func/métodos devem observar body de Source sem perder origem nem converter Styled em Cell.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Presença lexical Raw — ratificada em 2026-10-14

### Medição anterior à decisão

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes/binários e pin 9f2b6e8715237cb086899a42873660fe744622e8. RawElem raw.rs:21–25 perde omissão; native_raw markup.rs:78–136 colapsa ausência/false/none. Campos, igualdade e encoders distinguem esses valores no pin. Trata-se de semântica/morfologia, não mecânica Rust.

### Responsabilidade e aceitação

content_field reconhece text/block/lang, Set versus Unset; demais Undeclared. content_set_fields enumera text,block,lang conhecidos nessa ordem.

Manifesto aprovado b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45, recibo em lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json. Esta seção completa apenas a presença anteriormente incompleta de Raw e supersede as cláusulas de ausência de máscara. Preservar contratos alheios; sem fase/default/render novo. Validar testes RED→GREEN, suíte, build e linhagem.

## Transformações tipadas — contrato aprovado

### Medição anterior à decisão

O corpus transformacoes-2026-10-14/results.json.gz e controles suplementares
registra identidade/presença perdida no carrier matrix/body. Fonte e owner
pré-migração estão em lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/base-files.json.gz
com hashes e linhas em consumers.json. A aprovação do manifesto
960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d autoriza
a nova representação e resolução no layout. Semântica/morfologia são linguagem;
formato Rust e mecanismo de reconstrução são mecânica.

### Responsabilidade exclusiva

Projetar os campos ordenados do contrato tipado; distinguir declarado omitido de inexistente. func deve ligar à função nativa correspondente à operação. Matrix continua não chamável pelo fallback genérico.

Testar RED→GREEN e revalidar os caminhos dependentes; o build não fecha X1–X5.

## Completar projeção da caption existente — 2026-10-17

Medição anterior à decisão: caption-shape-before.json e caption-after.json em
lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17 fixam fontes, UTC,
binários, HEAD/working tree de validation-result.json e pin
9f2b6e8715237cb086899a42873660fe744622e8. Projeção de caption já produz o
carrier, mas sua CLI imprime só func:figure.caption; body/getters/fields estão
ausentes. json.encode tem o mesmo defeito. Metadata de kind rejeita a função
de caption válida. O pin em model/figure.rs:510 declara caption Locatable e
em :511-594 declara position,separator,body e kind/supplement/numbering/counter
sintetizados públicos, com figure_location interno. Não sintetizar esses dados
nesta correção; a função de elemento já existe desde a aprovação P1473.

Responsabilidade exclusiva e aceitação:

Listar campos candidatos de FigureCaption na ordem position,separator,body;
expor somente getters conhecidos. content_elem_func retorna o callback nativo
native_figure_caption para FigureCaption, conservando nome interno existente;
não usar o callback genérico não chamável nem igualdade por nome como prova.
Não alterar política de Unset/síntese dos campos ausentes nesta fatia.

Correção interna/paridade em fluxo contínuo ADR-0127. Não há novo campo/trait/
assinatura Rust pública, fase, modo ou default. L0 antes de testes/código, RED
real de getters/metadados/encoder/CLI, GREEN e corpus externo sem relaxar
expectativas de corpo/nome. Revalidar todos os gates da etapa sob 12 GiB, swap
zero, jobs1. Resselo só dos owners após V15/V26. FG1/SY1 permanecem incompletos
conforme follow-ups.md; nenhuma API pendente está autorizada por esta seção.

## A1 — reparo de paridade medido em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c; estado/UTC/fontes em
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
Vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. before-pdf-probe.json.gz
no mesmo diagnóstico congela controles bilaterais de campo, admissão, callback
e init. A auditoria anterior paridade-novo-vanilla-2026-10-17/inventario.md
E02–E04 registra os limites de carrier e as fontes discriminantes; não se usa
dump/estrutura Rust como critério. Estes observáveis são linguagem.

### Responsabilidade exclusiva e aceitação

Presença de Text continua Set(text). H/V amount é Set; weak só Set quando
weak_explicit, senão Unset. Box e Block: body é Set; width/height são Set quando
transportados em Some (incluindo width_fraction de Box), senão Unset. fields
enumera os campos em ordem width,height,body nos containers e amount,weak nos
espaços; só inclui Set. Styled não-equação não projeta o schema do filho: preservar seu comportamento
próprio, o guard de equação e snapshots autoritativos. Outros cosméticos desses containers
sem presença fiel permanecem fora da fatia; não inferir explicit:false/none a
partir do default nem afirmar paridade total desses construtores.

L0 primeiro, RED funcional e GREEN bilateral. Não alterar contrato público,
default de produto, fase ou compatibilidade; gate A1-S1 é independente. Preservar
1:1 e núcleos pinados; resselo após grafo válido. Build/workspace/Clippy estrito,
Rustdoc/fmt/linhagem estrita e reversa, jobs sequenciais sob MemoryMax12GiB,
High8GiB/swap0/jobs1. Debug0/incremental0 somente no ambiente de validação para
reduzir picos; comparar consumo medido sem inferir ganho do produto.

### Limite de presença de body opcional de Box

box_elem.rs:29-45 aceita body omitido e grava Content::Empty, indistinguível de
body explícito vazio. Portanto esta fatia NÃO promove body de Box em has/at/fields
ou na rota pública estrita: manter a ausência anterior até transporte de presença
com contrato próprio. Box expõe somente width/height já em Option. Block exige
body e pode expô-lo. Não criar regressão em box().fields() nem inferir presença
pelo valor Empty. Esta cláusula supera o body Set genérico da seção A1 acima.


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Usar a projeção nominal constructor_heading para func()==native_heading e para
presença/at/has/fields, incluindo supplement e numbering explícitos. Ordem raw:
level,depth,numbering,supplement,outlined,bookmarked,body; somente campos Set são
emitidos, body por último. Fora das propriedades transportadas delegar ao Heading
base. Não aceitar Styled comum de texto ou wrapper externo como Heading.
Snapshots continuam autoritativos; não reinferir seus campos pelo estilo atual.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.

## R1 — reflexão de sequência e vazio, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado.
lab/results/construction/diagnostics/sequence-reflection-2026-10-17/before.json.gz e
before-sources.json.gz guardam UTC, diffstat, fontes e SHA dos binários. Pin
vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. No upstream
lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:724–728,
Sequence tem children requerido; :538–585 projeta has/at/fields. As sondas
empty-fields/empty-func/empty-children e sequence-fields/sequence-children/
sequence-single/sequence-for refutam o Tekt: campos ausentes e nome empty.
fields.rs:317–322 deixa Empty/Sequence no complemento sem campos;
identity.rs:24 usa empty; content_methods.rs:242 enumera somente body no
fallback. São morfologia e semântica da linguagem, não exigência de copiar
representação Rust. Text vazio continua texto; Styled comum permanece opaco.

### Responsabilidade exclusiva

Enumerar children como único campo candidato para Empty e Sequence.
content_field usa get_field do owner; children é Set inclusive quando vazio,
body/missing são Undeclared. Preservar Source e a opacidade de Styled.
func continua pelo bridge existente, com nome sequence fornecido por elem_name.
Não classificar funções pelo nome nem ampliar o catálogo de seletores aqui.

### Aceitação e divisão explícita

R1 é incompleto para a função Sequence: R2 completará construção e admissão
em selector/where/show/counter, medindo argumentos, identidade e limites antes
de alterar seus owners. before.json.gz registra esses refutadores abertos.
Não anunciar paridade total de func() ou de sequência por este reparo.
A mensagem genérica de método inválido também permanece fora de R1.

Sem novos campos, tipos, traits ou assinaturas Rust públicas, fase ou modo
de produto. L0 antes de código, RED→GREEN, testes de regressão, workspace,
build, Clippy/Rustdoc estritos, ownership/núcleos/linhagem e reversa válidos;
V16 não aumenta. Resselo somente após V15/V26 válido. Validação sequencial
com MemoryMax12GiB/High8GiB/swap0/jobs1; registrar pico, não inferir ganho.

## R2 — construção e seleção de Sequence, fluxo contínuo ADR-0127

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado.
lab/results/construction/diagnostics/sequence-constructor-2026-10-17/{before,expanded}.json.gz
e before-sources.json.gz conservam UTC, diffstat/status, binários/fontes e casos
bilaterais no pin bd5ebec285505889e8ff1d4f68804ab147a699f3. Vanilla
foundations/content/mod.rs:724–728 declara children requerido;
foundations/value.rs:644–650 define cast Content/string/symbol/none e
typst-macros/src/elem.rs:573–604 gera o constructor. Tekt
01_core/src/compiler/eval/bindings/field_access/content_methods.rs:358–410
ainda projeta fallback não chamável; o catálogo element_where não tem Sequence
e selector_matching.rs:272–337 não tem sua identidade. rules/set.rs:55–76
só recusa With de transforms e não reconhece set da Sequence. São semântica,
morfologia e diagnósticos de língua, não exigência de copiar storage upstream.
A mensagem de children() marcada pendente em R1 já coincide no baseline atual: o
getter recém-corrigido permite que o owner existente produza o erro correto.
A medição nova retifica essa pendência; não editar código de diagnóstico.

### Responsabilidade exclusiva

Completar a projeção de func para Empty e Sequence com callback exclusivo
crate-interno native_sequence, ABI nativa atual de quatro argumentos. Receber
children requerido posicional, validar array, converter cada filho Content/
LocatedContent/string/symbol/none pelo cast de conteúdo atual. Preservar filhos
imediatos, ordem e seq de zero/um filho sem Content::sequence normalizador.
Borrow dos argumentos: não clonar o array de entrada para analisá-lo. Clonar
somente os conteúdos que entram no resultado. Não executar callbacks dos filhos.
Erros: children nomeado sem posicional gera the argument `children` is positional
com hint try removing `children:`; ausente gera missing argument: children;
array inválido gera expected array, found T; filho inválido gera expected content,
found T. Cast do argumento e dos filhos precede detecção de extras. O primeiro
extra pela sequência causal gera unexpected argument[: nome], com span da
ocorrência; tipos usam value_span da primeira posição e missing usa args.span.
Função com With funciona na chamada pelas rotas existentes; func retorna sempre
o callback real, sem usar o fallback genérico e sem publicar binding global.

### Aceitação e limites

Esta seção completa a responsabilidade R2 nomeada no L0 de R1. Corrigir
continuamente tabelas/composição/callback interno, sem novos campos/traits/
assinaturas Rust externas, modo padrão, fase ou quebra de compatibilidade.
L0 antes de teste e código, RED funcional, GREEN e corpus bilateral. Não
certificar guardas globais de show, todos os casts/elementos, nem paridade
universal de render. Qualquer refutador residual deve permanecer no relatório
e ganhar owner/L0 antes de intervenção, sem mudar testes para esconder o caso.

Validar build/workspace/doctests em invocação separada, Clippy/Rustdoc estritos,
fmt, V1/V5/V15/V26 e reversa, SARIF diferencial sem erro/warning novo. V19 pode
registrar agrupamento nominal prescrito sem supressão, testando alternativas.
Heavy jobs sequenciais, MemoryMax12GiB/High8GiB/swap0/jobs1, debug0/incremental0
só no ambiente de validação. Resselo após V15/V26 válido e proveniência exata.


## R2-S — realização nominal de Sequence, ratificada em 2026-10-17

### Medição anterior à decisão

As coletas before/expanded/controls.json.gz e candidate.log em
lab/results/construction/diagnostics/sequence-constructor-2026-10-17 fixam HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/diffstat/fontes/binários e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Oito REDs iniciais; cinco passaram no candidato, show e recursão ainda não.
markup_stream.rs:240–260 aplana antes de selecionar a sequência; realize/mod.rs:
170–210 visita filhos antes de receita; apply.rs:140–170 delega não-localizáveis
ao ponto-fixo. O vanilla aceita alias e rejeita nova Sequence com mesmos
filhos. São efeitos, escopo, valores e diagnósticos de linguagem.

O dono aprovou a revisão
lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md, manifesto
46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327,
nesta sessão. O gate ADR-0127 §2.3 cobre a ordem de realização desta nova
fronteira antes dos filhos/Table/math; não cobre novo contrato público.

### Responsabilidade exclusiva

No cast dos filhos, nomear as 34 variantes Value não convertíveis no grupo
que devolve expected content, found T, preservando o domínio fechado para
variantes futuras. O agrupamento é prescrito pela mesma rejeição, sem allow/
expect. Text vazio permanece Text; None é Empty. A aceitação de Symbol nesta
frente é somente glyph herdado do cast atual; sua identidade Symbol distinta
requer contrato próprio, não inferência por repr ou igualdade de Text.

### Aceitação

S1–S20 ratificados, RED antes de código, GREEN bilateral e regressões; não
reduzir expectativas por limite mecânico. Todos os owners 1:1 e núcleos pinados
válidos antes do resselo. Build/workspace/doctests separados, Clippy/Rustdoc
estritos, fmt/linhagem/reversa e SARIF sem novo erro/warning. 12GiB/High8GiB/
swap0/jobs1 e comandos pesados sequenciais. Sem publicação/release.

## Divisão explícita — pendências após R2/R2-S

Medição e proveniência: r2s-scope.json.gz/text-before.json.gz no diagnóstico
lab/results/construction/diagnostics/sequence-constructor-2026-10-17. Este pacote é
incompleto no universo de morfologia e regras textuais. R3-Symbol deve
completar identidade/igualdade do cast de Symbol (text/true no Tekt contra
symbol/false no vanilla); exige proposta/gate próprio se criar variante
pública, que R2-S não autoriza. R3-Markup deve preservar a Sequence
interpolada como filho imediato no valor de ContentBlock sem regra de
Sequence, hoje aplanada. R3-Textual deve medir/corrigir a precedência dos
literais em escopos distintos (C vs B), no owner matching. R3-Diagnostics
deve completar rastros genéricos de show de filhos emitidos. Estes nomes
identificam os pacotes de conclusão; não são prompts L0 nem autorização
para código adicional. Não classificar tais diferenças como mecânica
aceita ou paridade já demonstrada.
