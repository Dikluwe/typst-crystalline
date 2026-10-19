# L0 — compiler/eval/bindings/field_access/content_methods
Hash do Código: 13cd3f0f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:79283c9047419768139e0227b9ed7e34146a85706ed47731655736545c95306b
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:f29e26ff07053f940cfe4ab5358fc9a6ac8d4d689528e169b5f8567492f52f46
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:dc549d79f9e996433894effc8e4efd1f8f3da3d9c30933110eeabbab582ae9a8
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:afbe9da9643b22adaaaedbff55495488d3ceac311999a5cdec8a3d005f50ef7c
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:71d4b43f260cff61fffa8ba7831733e7a62f2517109911c8af346458d64cbd5b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:16ae092d990d3d26f03232afcabf63f3b5c08ffb5b6297989368f0346d14f2ab
- 00_nucleo/prompts/_nuclei/table/row-groups.toml sha256:472e9e168c439ce5e1d17557fd8dbb55047e2f3c2f259eada2ff8b763ae27ade
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:cbd8b56af347a35e867b2c4695581721d5983ec24ee7bb022e9f9987500d40f3


## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`. No HEAD `ba5a4c7d305ed1444660d9eb80862f51b417efb6` e árvore limpa, `01_core/src/compiler/eval/bindings/field_access.rs:1165-1589` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-field-access-2026-09-30-before.json.gz`.

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
00_nucleo/diagnosticos/matrix-morfologia-2026-10-06/before.json.gz guarda
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
00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz
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

00_nucleo/diagnosticos/table-cell-realizacao-2026-10-07/before.json.gz e
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
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
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

Medição anterior: 00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz registra HEAD/working tree/UTC/fontes/argv e binários no pin 9f2b6e8715237cb086899a42873660fe744622e8. A fonte desta unidade tem a operação transparente correspondente nos ramos existentes de ShowRecipe/Located/Realized; Source não pode vazar como elemento observável. Aprovação humana `confirmo`, 2026-10-07, contrato em materialization/table-origens-d08b3c2-2026-10-07/contract.md.

Fields/func/métodos devem observar body de Source sem perder origem nem converter Styled em Cell.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Presença lexical Raw — ratificada em 2026-10-14

### Medição anterior à decisão

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz registra HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree, UTC, fontes/binários e pin 9f2b6e8715237cb086899a42873660fe744622e8. RawElem raw.rs:21–25 perde omissão; native_raw markup.rs:78–136 colapsa ausência/false/none. Campos, igualdade e encoders distinguem esses valores no pin. Trata-se de semântica/morfologia, não mecânica Rust.

### Responsabilidade e aceitação

content_field reconhece text/block/lang, Set versus Unset; demais Undeclared. content_set_fields enumera text,block,lang conhecidos nessa ordem.

Manifesto aprovado b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45, recibo em materialization/raw-presenca-2026-10-14/approval-receipt.json. Esta seção completa apenas a presença anteriormente incompleta de Raw e supersede as cláusulas de ausência de máscara. Preservar contratos alheios; sem fase/default/render novo. Validar testes RED→GREEN, suíte, build e linhagem.

## Transformações tipadas — contrato aprovado

### Medição anterior à decisão

O corpus transformacoes-2026-10-14/results.json.gz e controles suplementares
registra identidade/presença perdida no carrier matrix/body. Fonte e owner
pré-migração estão em materialization/transformacoes-tipadas-2026-10-14/base-files.json.gz
com hashes e linhas em consumers.json. A aprovação do manifesto
960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d autoriza
a nova representação e resolução no layout. Semântica/morfologia são linguagem;
formato Rust e mecanismo de reconstrução são mecânica.

### Responsabilidade exclusiva

Projetar os campos ordenados do contrato tipado; distinguir declarado omitido de inexistente. func deve ligar à função nativa correspondente à operação. Matrix continua não chamável pelo fallback genérico.

Testar RED→GREEN e revalidar os caminhos dependentes; o build não fecha X1–X5.


## RG1 — grupos Table ordenados e síntese estrutural — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Adicionar schemas estáticos Table.children, Header repeat/level/children, Footer repeat/children, HLine y/start/end/stroke/position e VLine x/start/end/stroke/position. Distinguir Set/Unset/Undeclared usando getters e máscaras; has/at/fields/repr e func mantêm ordem e identidade nativa (não homônimo por nome). Campos omitidos não viram defaults por leitura; nomes declarados omitidos usam erro conhecido/não assente. Nomes públicos de diagnóstico dos quatro elementos seguem header/footer/hline/vline; qualificação da função continua table.*. Table.children é lexical, inclusive durante Table show. Outros campos de Table permanecem RG2; não preencher faltantes com zeros/None.

### Gate, aceitação e incompletude

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
