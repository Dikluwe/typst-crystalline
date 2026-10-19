# L0 — compiler/eval/expression_dispatch
Hash do Código: 1c8802cb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/eval/mod.rs:1651-2237`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/expression_dispatch.rs`. Dispatcher exaustivo e estático de Expr e helpers de expressão; preserva ordem, escopos, spans e diagnósticos. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.

## D05 — captura lexical de show para equação literal

Medição antes da decisão: equation-where-2026-10-06/after.json.gz registra
show-block/show-inline aceitos sem transformação. expression_dispatch.rs:270–281
cria Equation e a retorna sem intercept_content; markup_stream.rs:119 só realiza
ocorrência inserida quando já está no modo de realização. Decisão: depois de
avaliar body uma vez e construir Equation, passar pela fronteira existente
rules::intercept_content, como os demais elementos de markup. A fronteira
captura receita lexical quando diferida e realiza quando solicitado, com guards
existentes; não adicionar avaliação global, fase nova ou novo wrapper. Preservar
Source/spans, captura de body para query e comportamento sem show. Fluxo contínuo
ADR-0127, mesma fase eval; teste literal RED→GREEN antes de código.

## D06 — captura das fronteiras lexicais existentes

Medição after.json.gz local-block-query retorna [2,0,2] versus [2,1,1];
after-additional.json.gz context-show-render produz fração versus EQ. Inspeção
de CodeBlock em expression_dispatch.rs:86–154 retorna Mathy sem guardar delta
local; Contextual:475–495 conserva scope da closure, mas não snapshot de show.

No join de cada valor Content de bloco de código, quando o estilo local mudou
e há math externo a inserir, guardar o Styles delta local existente no produto,
sem assar variável/constructor e sem espalhar set a expressões posteriores fora
do bloco. Capturar context externo com carrier Styled privado contendo um
ShowRecipe existente como Value::Content na chave privada eval.context.recipe;
sem campo/enum/assinatura pública nova. O snapshot conserva rules/styles/guards/
file/context_id_floor e não é executado ao capturar. A fachada L1 ratificada
realiza o produto contextual usando esse snapshot causal, sem regra retroativa.

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

No join de cada Content realmente emitido por CodeBlock, capturar ambiente local de inserção Table/Cell antes de restaurar scopes/styles/rules. LetBinding não é inserção de seu valor. Respeitar local-cell-alias/OUTER LOCAL e older-global-alias NEW, sem contador falso nem busca por nomes. Context captura snapshot Table/Cell pelos helpers L1 existentes; resultado contextual só realiza depois de produzir conteúdo. Preservar bloco matemático/D06, flow/diagnósticos/uma avaliação.

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

### Medição e correção suplementar interna — 2026-10-07

Proveniência: diagnosticos/table-cell-realizacao-d08b3-2026-10-07/geometry-fifth.json e local-bool-red.log, binário 59da863f, fonte/HEAD/estado/UTC/argv/glifos/testes.
local-bool-red.log mede FUNCcell em vez de FUNCsequence; geometry-fifth.json mede LOCAL/a quebrados. Em CodeBlock join efetivo, Cell crua precisa realizar regra local antes de compor outro body; empacotar receita diferida preservava Cell como body da Cell externa. Join Content com apply_show_rules=true delega imediatamente à fachada causal de Table/Cell; passagem original com apply_show_rules=false permanece sem preparo. O corpo continua valor lexical no let fora de join. Defaults de Cell isolada seguem D08B3-B; preservar it nominal, alias tardio, escopo e flags/restauração.
Não certificar antes da recoleta RED→GREEN e suites; nenhum campo/método público, flag ou mudança de fase suplementar.


## D08B3-C2 — candidato não vigente antes do gate ADR-0127

### Medição anterior à decisão

`00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/baseline.json.gz` registra oito sondas completas: fontes, comandos, UTC, HEAD, working tree/diffstat, hashes e binários. Pin 9f2b6e8715237cb086899a42873660fe744622e8. Executável Tekt congelado do C validado; `source-verification.json` confirma hashes produtivos/L0 iguais ao snapshot tested-final do C. O target da raiz não estava disponível nesta retomada; não foi fingido um rebuild.

bare/alias/spread-conflict: mensagens e hints iguais, caret/trace divergentes. No alias o vanilla usa a definição [a] (linha 1), não a chamada (linha 3). No spread usa [a], não o spread inteiro. local-body publica sequence com metadata+Cell preparada no Tekt, styled contendo Cell crua no vanilla; cores/efeitos coincidem no fixture. plain-func coincide cell; set-func/show-func/set-show-func medem cell/sequence contra styled. A primeira rodada `before` tinha str(function), foi invalidada para decidir morfologia; somente baseline usa repr(function).

Fonte: vanilla `typst-eval/src/code.rs:148` aplica Value.spanned; `typst-library/src/foundations/value.rs:210–216` só assenta Content/Func; `foundations/content/mod.rs:109–114` preserva span já assentado. `model/table.rs:436–437` transporta value.span ao fabricar Cell. `typst-eval/src/code.rs:26–57` conserva set/show como styled sobre a cauda. Tekt `eval/expression_dispatch.rs:125–136` realiza Cell no join; `stdlib/structural/table_grid.rs:160–175` guarda children sem origem; `realize/table_cells.rs:176–183` fabrica Cell com span detached; `layout/grid/row_groups.rs:109` usa esse span. É língua (diagnóstico/morfologia/efeitos), não igualdade Rust ou bytes PDF. Não inferir intenção upstream.

### Responsabilidade exclusiva candidata

Capturar primeiro span de Content no eval existente, preservando referências/spreads; CodeBlock preserva Styled lexical da cauda local e snapshot causal existente, sem executar receita ao guardar o valor. Respeitar pipeline/guards/escopos e não aplicar a correção cegamente a Flow/Math.

A especificação detalhada e limites estão no contrato candidato do pacote C2. Antes de materializar, instalar este conteúdo no caminho canônico do owner, auditar seus núcleos e selar hashes somente depois de RED→GREEN. A fonte vigente permanece intacta até aprovação.

A cláusula C2 supersede a realização imediata da Cell no join descrita pelo refinamento A: capturar valor lexical e realizar somente na inserção efetiva. Essa mudança de fronteira/ordem também integra o gate ADR-0127 §2.3; preservar a ordem Table→placement→Cell→body do núcleo vigente.
