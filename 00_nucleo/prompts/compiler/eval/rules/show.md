# L0 — compiler/eval/rules/show — Avaliação de show rules
Hash do Código: afa1738a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:71d4b43f260cff61fffa8ba7831733e7a62f2517109911c8af346458d64cbd5b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/show.rs`

## Medição antes da decisão

A working tree não commitada capturada em `00_nucleo/diagnosticos/atomizacao-eval-rules-2026-09-30-before.json.gz` contém as rotinas desta família em `01_core/src/compiler/eval/rules.rs`. A suíte L1 passou com 6401 testes e 2 ignorados antes da extração. É medição estrutural, não benchmark nem prova bilateral nova contra o vanilla. As medidas históricas e ressalvas do pin continuam em `00_nucleo/prompts/compiler/eval/rules.md`.

## Contrato de linguagem

Capturar estilos de set e avaliar ShowRule com identidade, seleção, transformação e escopo léxico; preservar guards, erros, callbacks, origem e morfologia.

As assinaturas, identidades, casts, defaults, precedência de erros e fase de avaliação observados permanecem. O parent mantém diagnóstico compartilhado e bindings estáticos; outros owners de selector matching, terminação e textual realization continuam exclusivos. Não duplicar esses algoritmos.

## Aceitação

Suíte L1 e `crystalline-lint .` sem regressão. Paridade em semântica, sintaxe e morfologia (ADR-0107/0108), ownership exclusivo e pins válidos (ADR-0129/0130).

## Reparos independentes do gate de aspas — 2026-10-06

Medição anterior à decisão: smartquote-2026-10-06/{before,fields-before}.json.gz,
com pin/HEAD/working tree/UTC e binários exatos registrados.

fields-before mede compile-show-native: vanilla Q, Tekt rejeita função smartquote.
Adicionar mapping estático por fn_addr_eq(native_smartquote) para o Selector::NativeElement
já existente. Não criar NodeKind, não despachar pelo nome (alias funciona) e
não executar constructor para identificar função. A folha programática já
existe e content_elem_func reconhece sua função. Markup continua fora desta
fatia até o gate de fase; alterar glyph/eval/layout aqui é proibido.

## Inserção matemática automática — proposta sujeita ao gate ADR-0127

### Medição anterior à decisão — D06, 2026-10-06

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado,
UTC/diff/estado/fontes/binários nas coletas canônicas
00_nucleo/diagnosticos/math-bare-2026-10-06/{before,additional,refuters}.json.gz.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8; proveniência no relatório
00_nucleo/diagnosticos/math-bare-medicao-2026-10-06.md.

before: frac, mat, sqrt, lr e op nus têm uma Equation no vanilla e zero no Tekt;
math.bold([x]) e texto comum têm zero nos dois. nested e nested-equation têm
uma nos dois; reuse tem duas no vanilla. block-style/local-block/number-style
resolvem o escopo de inserção. show-generated e context-generated expõem as
mesmas lacunas. additional: Equation explícita preserva o render da fração;
it.body é rejeitado no vanilla por maximum show rule depth exceeded, enquanto
it é válido; o show posterior afeta apenas a segunda inserção. refuters mede
valor frac cru e label original versus Equation explícita.

lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334 transforma
Mathy antes de show fora de math. 01_core/src/compiler/layout/equation.rs:680–689
projeta math nu em texto; compiler/realize/mod.rs:155–199 prepara show/identidade
sem normalização math. entities/content/traversal.rs:14–108 é bottom-up e
recursa no corpo matemático; rules/apply.rs:564–607 trata apenas localizáveis
já existentes. Estes observáveis são semântica/morfologia/render de linguagem,
não requisito de vtable, buffers, igualdade Rust ou bytes PDF.

### Responsabilidade exclusiva

Adicionar ao mapping já existente fn_addr_eq(native_math_equation) para
Selector::NativeElement da função efetiva. Direto, alias e .where seguem o
mesmo owner de matching. Sem NodeKind novo, despacho por string ou constructor
executado para descobrir identidade. Função homônima e partial/closure não
viram elemento por seu nome. Esta entrada é correção de mapping em fluxo
contínuo; neste pacote será promovida com o gate maior, não é mudança de fase.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

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

Adicionar somente mapping estático fn_addr_eq(native_table_cell) para NativeElement já existente. Não executar constructor nem classificar pelo nome.

### Limites e aceitação

RED→GREEN e coleta bilateral; cargo build e crystalline-lint sem erros.
Esta correção não muda contrato público de tipos/assinaturas nem fase.
D08B permanece incompleto: D08B2 completa presença omitido/auto/none por
contrato próprio; D08B3 completa realização de posições/estilos/padding/show.
Option None não prova omissão nem none explícito: não certificar esses casos.

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

Medição order/table-replacement-before-cells: vanilla aceita show table e omite efeitos de células; Tekt rejeita em rules/show.rs:322–331. Adicionar mapping por fn_addr_eq(native_table) para Selector::NativeElement existente, conservando aliases e where explícito. Não criar NodeKind nem identificar pelo nome; regras e matching continuam com seus owners.

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

## Transformações — integração aprovada em 2026-10-14

Medição antes da decisão: corpus transformacoes-2026-10-14 e style-contract/results.json
registram o pin e os comandos/UTC. Fora de context há erro; aliases preservam
identidade; with não admite set/getter/show; dois set origin dobram por eixo.
A fixture de layout/tests.rs:21648 constrói matrix diretamente. Manifesto
aprovado 960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d.

Aceitar move/rotate/scale/skew nativos como Selector::NativeElement pela identidade de fn, conservando aliases. Matching e filtros pertencem ao owner selector_matching. Não aceitar funções with como seletores: o pin as rejeita.

Validar corpus e testes causais; não confundir construção com layout ou release.


## Carriers compactos de introspecção/show — proposta 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em 00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16 registra UTC, fontes/L0, hashes, status e git diff HEAD --stat. Os owners atuais foram lidos antes de propor esta alteração. probe-details.json registra rustc 1.92, argv, rlib/binário/sonda com SHA-256 e saída; probe-result.json registra teto de 12 GiB, swap zero, um job e eventos. A sonda é sintética de carriers e do algoritmo de registro/cópia, não benchmark de compilação/layout/RSS nem distribuição de workloads reais. Os candidatos são espelhos com tipos reais de domínio; o caso inline usa Tag e ShowRule produtivos.

Tag mede 416 bytes; Start boxed e Arc medem 48. Para 4.096 pares Start/End com payload Heading sem heap, inline solicita 3.407.872 bytes numa alocação; Box solicita 2.031.616 em 4.097; Arc solicita 2.097.152 em 4.097. Clone de Tag boxed também aloca por Start; os caminhos produtivos lidos em introspect/fixpoint.rs:86–136 e entrypoints.rs:44–66 usam Vec<Tag> temporário e leituras emprestadas, sem clonar o vetor. Walk :325–356 popula o índice por referência antes de mover info para Start. A decisão não toma o clone sintético como frequência produtiva.

Transformation mede 328 e ShowRule 376 bytes; candidatos com Box/Arc<Styles> medem 120 e 168. rules/show.rs:464–470 copia to_vec antes de cada registro; apply.rs:108–112,349–352 filtra/clona regras. Em 128 registros 100% show-set simples sem reter snapshots antigos, inline solicita 12.276.960 bytes/384 alocações, Box 8.194.560/8.640, Arc 5.530.624/512. Com snapshots retidos, picos solicitados são 3.203.856, 4.141.744 e 1.477.808. No controle com canal custom, as cópias de 4.096 regras solicitam 4.107 alocações inline, 8.203 Box e 11 Arc (crescimento do Vec incluído). As leituras de estilos são emprestadas em apply/intercept/outline/table_cells; a aplicação ainda clona o delta quando necessário.

Contraexemplo obrigatório: construir uma única coleção de 4.096 regras 100% show-set simples pede 1.540.096 bytes inline, 2.031.616 Box e 2.097.152 Arc. Arc pode reter capacidade sobrante do canal custom; no controle rico de 128 registros sem snapshots antigos, o final vivo foi 78.064 bytes inline e 156.432 Arc. Não afirmar ganho universal ou ignorar custo atômico/indireção. É mecânica de armazenamento sob ADR-0030/0107, não paridade de linguagem ou ganho cognitivo por IA medido. A inferência favorável a Arc no ciclo de cópias seria refutada por regressão de memória/tempo em um workload real relevante; manter visíveis estes controles.

### Responsabilidade exclusiva proposta

Após capturar o mesmo Styles com capture_set_styles(set, scopes, ctx, engine)?, envolver o resultado em Arc::new somente para construir Transformation::Style. A captura, swap/restauração da chain, propagação de erro, to_vec/registro e ordenação existentes não mudam. Não duplicar fold, clonar Styles extra, adicionar callback ou avaliar outro set. O carrier público é propriedade de 00_nucleo/prompts/entities/show.md; este owner migra só seu produtor. Os consumidores emprestados não precisam de algoritmo novo.

### Gate e aceitação

Proposta não vigente até ratificação humana ADR-0127 §2.1/§2.4: há mudança de payload público e compatibilidade de construção/acesso. Não editar código ou owners ativos por este pacote antes da aprovação. Após ratificação instalar estes L0, testes primeiro e RED real, implementação e resselo estritamente dos owners alterados. Não silenciar large_enum_variant, alterar limiar ou adicionar dependências/flags/defaults/fases.

T1: contratos de construção Box<ElementInfo>/Arc<Styles> e orçamento relativo dos carriers (Tag até três Location; Transformation até Content+alinhamento) devem falhar no estado antigo e passar no novo, sem fixar ABI universal. T2: Start/End/Location/payload/label, igualdade e hashes estruturais por valor, clone independente da Box e mutation isolation preservados; hash não vira formato persistido. T3: testes existentes de introspecção/convergência, nesting, heading/label, state/counter/query/metadata continuam com os mesmos esperados e erros. T4: clone de Transformation::Style compartilha o Arc; Arc::make_mut em cópia preserva estilos do original, incluindo custom, e as variantes Func/Content/Str permanecem. T5: controles existentes de show-set/func, precedência, escopo, labels, outline, table.cell e textual/regex mantêm semântica/morfologia/erros; nenhuma regra executa a mais. T6: workspace completo, build, Clippy all-targets e expectativas/lints já fechados como erro, rustdoc -D warnings com quatro índices, fmt, diff, linhagem estrita/reversa e Tekt sem aumento; 12 GiB, swap zero, um job. Registrar HEAD/working tree/hashes/diffstat/UTC e distinguir tamanhos/bytes solicitados de RSS. Não publicar release. Alguma mudança de assinaturas, campos ou fase além destes payloads exige outra proposta.

## Ratificação humana — 2026-10-16

Proposta aprovada pelo humano, manifesto SHA-256 `282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`. O contrato acima passa a vigente; inclui materialização/testes T1–T6, sem release. Registro em `00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16/approval-receipt.json`.


## Tabelas: exaustividade, origem e argumentos — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC/status/git diff HEAD --stat, fontes/L0/configuração e hashes em `00_nucleo/diagnosticos/tekt-v16-table-cells-2026-10-17/before.json.gz` (snapshot inicial de dois owners preservado). contexts.json identifica dez V16 e as projeções latentes, antes do código. public-before.json usa APIs públicas reais com rlib/probe/binário/SHA/argv: Source(Cell) perde x/colspan/rowspan, kind e span no header; footer perde Source; níveis distintos são rejeitados somente quando seus membros levam Source; grupo de footer Source muda células a/b para ab/vazio. snapshot-before.json mede context_floor 77→0 sob Source nas duas restaurações. cli-before.json/cli-extra-before.json/cli-controls-before.json registram sondas bilaterais com binários SHA completos no pin 9f2b6e8715237cb086899a42873660fe744622e8. Footer com um argumento [a#strong[b]] gera duas células a/b no Tekt contra ab/vazio no vanilla; show table:none é indevidamente rejeitado. Controles [] ocultam header/footer sem executar panic de Cell; erros de colspan com alias conservam mensagem/caret. Header/footer colspan e grupos de nível distinto também expõem divergências de linhas, que este lote não dará por resolvidas. É semântica/morfologia/origem/erro, não obrigação de igualar estrutura Rust, bytes ou inferir intenção upstream.

### Prescrição exclusiva

Na classificação do valor da transformação, Value::None passa a Transformation::Content(Content::Empty), reutilizando tipos existentes. None é conteúdo vazio nesse cast, não seletor novo; set capture, guard, ordem de avaliação/registro/id e erros legados dos outros tipos ficam iguais. Não executar a receita de Cell nem corpo ocultado só para validar a regra. Fonte medida: show.rs:442–459 rejeita None; o pin aceita none e mantém rejeição de int/auto. Preservar os controles [] e provar equivalência de ocultação; a mensagem localizada dos tipos inválidos permanece como observável legado, sem alegar sua paridade integral.

### Aceitação e pendências

L0 antes de testes/código. Fluxo contínuo ADR-0127: correção interna de Source e casts da linguagem, sem campo/tipo/assinatura/trait público, nova flag/default, fase ou compatibilidade externa. Constructor só assenta Cell crua: não chama receita, with_defaults, placement, fontes ou layout. A ordem Table show→defaults/placement→Cell show→body permanece. Grupos V19 ficam justificados por caso e visíveis; não provar cobertura por executar um braço. Não introduzir allow/expect, captura @ ou if-let que esconda a exaustividade nova. Preservar erros/casts legados não alterados, guards que falham, callbacks e restauração em erro/sucesso.

Regressões RED→GREEN nos owners: propriedades/presence/kind/origem de Cell explícita e Source, casts de footer por argumento e não por fragmento, formas cruas não preparadas, groups Source e duas restaurações de snapshot, negativos de tipos/Auto/None e shadow de chave local inválida, None em transformação show sem abrir aceitação de outros valores. Revalidar efeitos ocultados/late-rule pelo CLI e observáveis estruturais do pin; não reduzir testes ou normalizar whitespace para fechar paridade.

Sondas Content/Value/TableCellKind com variantes futuras reais em workspace descartável, cfg(test) compilado: os 18 matches finais devem exigir E0004, contando apenas uma vez cada match. Inputs e erro-testemunha cfg(test) preservados. Cargo target privado, nunca compartilhar artefatos mutantes/medições com o target principal. Gates: build, suíte workspace completa, fmt, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings e quatro índices, V1/V5/V15/V26 estrito e reversa, replay, diferencial SARIF completo e diff-check. V16 esperado 141→131; V19 só dos grupos prescritos; V20 sem crescimento, remover profundidade das duas projeções opcionais. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais. Sem commit/push/merge/release ou pin CI novo.

Incompletude explícita: header/footer spanning e limites entre níveis ainda exigem o pacote table-row-group-contract, com medição/L0 próprios. Nível do grupo e kind/nível semântico da Cell não são o mesmo conceito; não ratificar a inferência atual como contrato completo. Não corrigir ocupação com comprimento de Vec nem usar Source/Styles como marcador de fronteira. Preservar Grid legada. Demais casts/paint/insets ricos e compromissos históricos não são certificados por fechar V16. O objetivo geral, math-stretch-composite e as duas obrigações de matching permanecem abertos.

## A1 — reparo de paridade medido em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c; estado/UTC/fontes em
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
Vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3. before-pdf-probe.json.gz
no mesmo diagnóstico congela controles bilaterais de campo, admissão, callback
e init. A auditoria anterior paridade-novo-vanilla-2026-10-17/inventario.md
E02–E04 registra os limites de carrier e as fontes discriminantes; não se usa
dump/estrutura Rust como critério. Estes observáveis são linguagem.

### Responsabilidade exclusiva e aceitação

Preservar mappings NodeKind e especiais page/par vigentes. O fallback de
funções usa native_element_fields e NativeElement somente para elementos nativos
já catalogados, incluindo text e bibliography e aliases reais. .with, closures
e homônimos continuam rejeitados. O matching/terminação/escopo é responsabilidade
dos owners atuais; este reparo não o duplica nem promove funções arbitrárias.

L0 primeiro, RED funcional e GREEN bilateral. Não alterar contrato público,
default de produto, fase ou compatibilidade; gate A1-S1 é independente. Preservar
1:1 e núcleos pinados; resselo após grafo válido. Build/workspace/Clippy estrito,
Rustdoc/fmt/linhagem estrita e reversa, jobs sequenciais sob MemoryMax12GiB,
High8GiB/swap0/jobs1. Debug0/incremental0 somente no ambiente de validação para
reduzir picos; comparar consumo medido sem inferir ganho do produto.
