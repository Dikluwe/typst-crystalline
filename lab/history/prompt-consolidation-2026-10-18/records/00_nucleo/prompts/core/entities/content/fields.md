# L0 — entities/content/fields
Hash do Código: 5adf16c5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:923f1c9e4c2fd5934316ae2eef3dbb461c3bef3009de1d8931d951e793dda342
- 00_nucleo/prompts/_nuclei/content/projection-exhaustiveness.toml sha256:71273fd4ed96ebc9bb2910e90f03cde1ecdd3543d7cf02a934c945f2c159a880
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:79283c9047419768139e0227b9ed7e34146a85706ed47731655736545c95306b
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:71d4b43f260cff61fffa8ba7831733e7a62f2517109911c8af346458d64cbd5b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/fields.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.


## Contract

Own `Content::get_field` and its transparent styled-equation lookup. Preserve
field presence, omission, style precedence, and delegation to element owners.
Unsupported fields return `None`.

## Acceptance

Existing field-access and show-rule tests pass. The implementation remains a
closed static match and does not introduce a property registry.


## Realização com avaliação única

ShowRecipe e Located delegam leitura de campos ao body; Realized delega ao visual. Nenhum carrier aparece como elemento público novo.


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

Classificar SmartQuote em match estático. Getter para matching semântico conserva
os defaults de double/enabled true, alternative false e quotes auto quando
necessários ao filtro, sem mutar campos assentados. Presence estrita pertence
a content_methods; não fabricar presença a partir de valor default.
Config quotes vira Value canônico por tipos existentes (Auto/dict single/double,
pares array<string>), sem PropMap/registry. Validar show filtrado de constructor
vazio contra oracle. Se estilo/filtro tiver novo contraexemplo, medir e atualizar
cláusula antes de código; não inferir Unknown como false ou mutar entidade.

### Ordem observável da configuração quotes

Medição antes da decisão: upstream text/smartquote.rs:394 converte configuração
para dict double, single; quotes-key-order-red.json mede keys() nessa ordem no
vanilla e single, double no Tekt. keys() é linguagem; bytes JSON não são o gate.
Decisão: Value canônico de quotes custom insere double antes de single, mantendo
Auto por membro omitido e pares como array. Não altera schema público.

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

Delegar Content::MathMatrix.get_field ao owner MathMatrixElem, sem algoritmo de matriz neste hub. Wrappers causais/snapshots mantêm semânticas atuais; não fabricar presença.

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

Delegar TableCell ao getter do owner; sem lógica de célula neste hub.

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

Delegar getter TableCell; não interpretar presença em outro algoritmo neste hub.

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

Delegar Cell getter; valores conhecidos do resolved carrier não são recalculados aqui. Wrappers/snapshot continuam autoritativos.

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

Source é transparente nesta projeção, delegando ao body; não expõe span como identidade de língua nem executa receitas. Segmentação conserva origem por segmento quando aplicável.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Presença lexical Raw — ratificada em 2026-10-14

### Medição anterior à decisão

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes/binários e pin 9f2b6e8715237cb086899a42873660fe744622e8. RawElem raw.rs:21–25 perde omissão; native_raw markup.rs:78–136 colapsa ausência/false/none. Campos, igualdade e encoders distinguem esses valores no pin. Trata-se de semântica/morfologia, não mecânica Rust.

### Responsabilidade e aceitação

Content::get_field delega Raw ao elemento; sem inferir defaults.

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

Delegar Transform ao get_field do Element, preservando somente os valores explícitos e body; Matrix conserva a projeção genérica anterior.

Testar RED→GREEN e revalidar os caminhos dependentes; o build não fecha X1–X5.

## Reflexão da legenda presente — 2026-10-17

Medição antes da decisão: caption-before.json e more-cast-after.json em
lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17 fixam UTC, fontes,
binários e o pin 9f2b6e8715237cb086899a42873660fe744622e8. A construção
conserva a caption, mas content/fields.rs:73 projeta só body de Figure;
FigureElem mantém o getter default None. O pin retorna FigureCaption para
caption presente (inclusive conteúdo vazio), null para none explícito e erro
not known at this point para omissão. O carrier público Option<Content> não
distingue none explícito de omissão; essa presença continua incompleta no FG1,
exige proposta pública, e não deve ser simulada com Empty/strings.

Responsabilidade exclusiva e aceitação:

Trocar a projeção especial de Figure.body pela delegação estática de todos
os campos Figure ao getter do elemento proprietário. Não colocar montagem da
caption aqui nem alterar outros braços/transparência/pins. O getter existente
mantém a mesma assinatura pública. Sem registro dinâmico ou fallback de corpo.

L0 antes do teste/código. RED real da projeção de caption, GREEN e replay das
seis sondas mais os casos simbólicos. Body, Source e campos ausentes continuam
preservados. Revalidar a etapa integral sob 12 GiB/swap zero/jobs1, resselo
só dos owners após V15/V26 válido; sem API/fase/default novo nem release.

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

Delegar também FigureCaption ao get_field do owner, como Figure. Sem lógica
de montagem/validação no dispatcher. Conservar todos os outros braços/pins.

Correção interna/paridade em fluxo contínuo ADR-0127. Não há novo campo/trait/
assinatura Rust pública, fase, modo ou default. L0 antes de testes/código, RED
real de getters/metadados/encoder/CLI, GREEN e corpus externo sem relaxar
expectativas de corpo/nome. Revalidar todos os gates da etapa sob 12 GiB, swap
zero, jobs1. Resselo só dos owners após V15/V26. FG1/SY1 permanecem incompletos
conforme follow-ups.md; nenhuma API pendente está autorizada por esta seção.


## V16 — projeções explícitas de Content (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/tekt-v16-content-views-2026-10-17/before.json.gz registra
UTC/status/diff HEAD --stat, fontes/L0 e 2129 hashes conferidos. contexts-before.json
mapeia os seis V16 por AST/linhas; projection-plan.json guarda o domínio real
Content (120 variantes) e os complementos por função. before-probe-result.json
conserva 17280 controles simbólicos de padrões e guards: o mutante Content
futuro compila nos seis locais (zero E0004 selecionados). É mecânica de proteção
de domínio, sem nova prova de paridade ou de leitura por IA. Payloads são
opacos no modelo; ele não substitui controles reais dos algoritmos.

before-field-probe.json usa a rlib atual e o programa field-control.rs com hash,
argv, UTC e source freeze: 48 de 84 leituras diferem quando Source, Located,
ShowRecipe ou Realized estão dentro de Styled/Styled aninhado. Fora de Styled,
os quatro já preservam campos. fields.rs:10–13 reconhece só Equation/Styled;
seu guard recusa os carriers antes da delegação. O núcleo source-origin já
exige transparência; o owner fields já exige body/visual dos outros três.
É invariância de campos da língua, não exigência de estrutura Rust vanilla:
esses carriers são internos e não têm syntax própria no pin. Não inferir
intenção do defeito; refutam a correção perda de precedência, ausência/none,
campos indevidos em não-equações, mudança do visual para original ou execução
de receitas. O pin de linguagem permanece 9f2b6e8715237cb086899a42873660fe744622e8;
esta medição metamórfica não é declarada comparação bilateral nova.

### Responsabilidade exclusiva

Em styled_equation, reconhecer Source/ShowRecipe/Located pelo body e Realized pelo visual, recursivamente, sem executar receitas nem mudar Equation/Styled existentes. As outras 114 variantes retornam None nominalmente. Em get_field, manter todos os braços/guards/valores e trocar apenas o wildcard final pelo complemento nominal de 94 variantes com campo desconhecido. Preservar os quatro overrides de equação e a precedência outer-first já vigente; não generalizar campos de equação para Styled de outro tipo. Acrescentar somente dois testes funcionais locais de carriers/precedência/negativos antes do código.

A lista nominal por função, em ordem da declaração vigente, é derivável das
variantes não cobertas incondicionalmente pelos braços anteriores; guardar
Styled no complemento se o guard puder recusar. Nos campos, variantes com
nome literal ou guard conservam também o caso de campo desconhecido; variantes
já delegadas para qualquer f ficam fora do complemento. O domínio string
continua aberto. O núcleo pinado projection-exhaustiveness partilha esta regra;
o owner continua exclusivo e source-origin conserva sua obrigação vigente.

### Aceitação e limites

L0 antes de testes/código; fluxo contínuo ADR-0127, sem campo/assinatura/trait
público, compatibilidade, default ou fase nova. RED funcional de fields antes
de código, sem usar falhas de montagem de sonda como RED. Após o reparo, os 84
controles reais de carriers devem coincidir; testar precedência de estilos e
não-equações (Realized consulta visual, não original). A sonda estrutural deve
passar de zero para seis E0004 selecionados ao adicionar uma variante somente
no modelo descartável. Seus 17280 casos simbólicos conservam a classificação,
exceto os 96 casos do helper que passam a reconhecer os quatro carriers;
cada variante de cada complemento V19 deve ser exercitada separadamente.
Essa prova não é equivalência de payloads: exige replay dos corpos anteriores,
regressão real e invariância dos algoritmos não alterados. Não adicionar
matriz produtiva que apenas espelhe o match.

Esperado: remover exatamente seis V16, acrescentar seis V19 nominais uniformes
prescritos, preservar os demais diagnósticos por fonte/regra/severidade/mensagem
e não acrescentar V20 ou supressão. Grafo V15/V26 antes de resselo dos cinco
pares. Build, testes focais e workspace, Clippy all-targets -D warnings/-D
unfulfilled_lint_expectations, Rustdoc estrito, fmt, linhagem estrita/reversa,
replay, source freeze e diff --check. Pesados sequenciais MemoryMax12GiB,
High10GiB/swap0/jobs1. Sem commit/push/merge/release. As frentes de linguagem,
contratos pendentes e o objetivo global não são encerrados por este lote.

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

Getters semânticos de Text.text; HSpace.amount/weak; VSpace.amount/weak;
Boxed.body/width/height; Block.body/width/height projetam os carriers existentes.
Spacing::Absolute vira Length e Fractional vira Fraction. Box.width_fraction tem
precedência sobre width. Block.width conserva Rel completo e Block.height conserva
Relative/Fractional. Styled não-equação conserva sua identidade e seus campos próprios; não é
carrier transparente. Preservar os overrides de equação outer-first e as vistas
de Source/Located/ShowRecipe/Realized.
Não inventar medidas relativas truncadas, campos perdidos nem máscaras públicas.
A presença estrita continua no owner content_methods; None não vira Auto assente.

L0 primeiro, RED funcional e GREEN bilateral. Não alterar contrato público,
default de produto, fase ou compatibilidade; gate A1-S1 é independente. Preservar
1:1 e núcleos pinados; resselo após grafo válido. Build/workspace/Clippy estrito,
Rustdoc/fmt/linhagem estrita e reversa, jobs sequenciais sob MemoryMax12GiB,
High8GiB/swap0/jobs1. Debug0/incremental0 somente no ambiente de validação para
reduzir picos; comparar consumo medido sem inferir ganho do produto.

Contraprova de escopo: upstream foundations/content/mod.rs:761-768 declara
Styled(child,styles), não o schema do filho. Não generalizar transparência de
Source para Styled; `[A].text` não autoriza `text(fill:red)[A].text`.

### Tipos observáveis das dimensões

value-controls.json.gz no diagnóstico A1 mede que h/v absolutos expõem length,
mas box.width e block.width absolutos expõem relative no pin novo. Sizing em
upstream layout/container.rs:518-527 conserva Rel como Value::Relative. Portanto
Box width/height Length armazenado vira Relative(rel:0,abs:length), fração de
width permanece Fraction; Block já conserva Relative/Fractional. Não normalizar
Box em Length só por o percentual ser zero. Testar type, não somente repr.


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

Adicionar projeção crate-private constructor_heading que reconhece somente
Styled marcado heading.constructor=true cujo filho direto (desconsiderando
Source) é Heading. Retorna Heading e seus Styles. Styled comum fica opaco.
Getters dessa forma projetam supplement explicitamente armazenado, numbering
string/none conforme gate/pattern e os campos já existentes do Heading base.
Não remover wrappers em geral nem importar compiler a partir de entities.

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

Projetar children de Empty como Value::Array vazio e de Sequence como array
de Value::Content, preservando ordem, filhos imediatos, wrappers e identidade
nominal. Não aplanar filhos, converter Text("") em Empty, retirar Styled ou
executar callbacks. Wrappers causais seguem a delegação já contratada; nomes
de campo desconhecidos continuam None no complemento exaustivo.

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
