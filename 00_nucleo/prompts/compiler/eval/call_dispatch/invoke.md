# L0 — compiler/eval/call_dispatch/invoke
Hash do Código: 7cf318ff


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:825182e1dfe3591249a5efbc4c0ddbd937ebffde953c6595e0d1ed8176106435
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/invoke.rs`. No HEAD `1492976302212e007282875aef066f84a3cf13aa` e árvore limpa, `01_core/src/compiler/eval/call_dispatch/call.rs:638-827` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-call-core-2026-09-30-before.json.gz`.

## Contrato

Resolver o callee preservado, avaliar argumentos e executar o match fechado de Func/Type/Plugin/Content com spans e traces atuais. Preservar `00_nucleo/prompts/compiler/eval/call_dispatch.md` e o owner `call.md`, sem mudar assinatura pública, defaults, fase do pipeline ou compatibilidade. A extração fica em L1, com despacho estático.

## Aceitação

Preservar a suíte L1 e os testes de call_dispatch; `crystalline-lint .` valida ownership/hashes. Não inferir ganho de desempenho.

## Origem de falhas na realização de conteúdo chamado — 2026-10-06

Medição: controls-first.json mostra maximum show rule depth exceeded sem fonte causal; invoke.rs:145 aplica a intercepção depois do trace de apply_func e perde o span da construção. Ao propagar erros de intercept_content, conservar spans conhecidos e ancorar somente os detached no call.span real. Não reconstruir origem pelo texto do diagnóstico nem sobrescrever origens internas. Mantém assinatura/fase e o trace de apply_func.

## D06 — origem causal de receita diferida matemática

Medição after.json.gz show-clone-body tem erro/hints corretos mas sem span da
inserção; invoke.rs:145–155 só ancora erros eager. Ao capturar ShowRecipe de
conteúdo que contém inserção Mathy externa, guardar call.span no Styles privado
eval.math.origin usando bits de Span::into_raw (Value::Int), sem alterar o
constructor/valor cru quando não há receita. Contexto existe somente na receita.
A realização reconstrói com Span::from_raw e ancora somente detached; mantém
spans conhecidos, Source real e ordem de hints/traces. O escopo de capture é
restaurado antes de retornar, inclusive em erro. Não derivar span da mensagem
ou recriar Source pelo texto. Nenhum contrato público ou fase além do gate D06.

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

Result Content::Table/Content::TableCell de constructor ou closure não é inserção por si só: conservar valor cru e não assar recipe global sobre valor guardado em let. Demais resultados conservam intercepção existente. Spans específicos e traces de argumento permanecem; a inserção de Table/Cell pertence a markup/join/produto efetivamente emitido, não ao nome da função.

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

### Refinamento interno medido — 2026-10-07

Medição anterior à decisão: 00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second}.json.gz, pin/binários/HEAD/estado/fontes/UTC/argv/resultados completos.
after-second/header-fields mostra table.header/footer como Cell e receitas duplicadas. invoke intercepta Header/Footer e native_table só extrai variantes diretas em table_grid.rs:85–100. Grupos estruturais TableHeader/TableFooter também permanecem crus no constructor, para serem extraídos pelo owner Table e realizados no plano. Não alterar outros nativos ou a política de Grid.
A aceitação é bilateral, efeitos uma vez e geometria; não normalizar whitespace nem alterar contratos ratificados.


## Avaliação única em chamadas — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/call-once-2026-10-17/before.json.gz registra UTC,
status, diff HEAD --stat, fontes/L0/config e hashes. Os 2124 hashes da etapa
anterior foram verificados exatamente antes do baseline. before-probe.json.gz
e before-math-probe.json.gz conservam binários/argv/fixtures/saídas bilaterais.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
calc.abs/array.len/str.from-unicode medem receiver=3/args=2 contra 1/1;
module.measure/layout/within/before/after/func/has/body chegam a receiver=4;
fallback mutante mede args=3 contra 1. Métodos ausentes executam panic de
argumento antes do erro causal. math.mat soma duas vezes; array prévio (1,)
fica (1,1,1) contra (1,1). Array vazio mascara o defeito com truncate.
call.rs:70–118 perde receiver; collection_methods.rs:43 avalia args antes de
reconhecer operação; contextual/content/selector reavaliam AST no fallthrough;
method_dispatch.rs:247 descarta args; invoke.rs:66–120 compensa por nome.
O L0 call_dispatch.md já exige receiver/args uma vez. pinned-source.json.gz
fixa call.rs/methods.rs upstream: interesse é ordem/efeito/diagnóstico da
linguagem, não copiar Vm, storage ou igualdade Rust.

A primeira fixture declarava palavra reservada or como função: é inválida e
está arquivada em invalid-fixture-attempt, sem crédito para efeitos de chamada.
Ela revelou panic 101 em import inválido; manter aberto para correção causal
própria no owner de parsing/import. O baseline válido substitui só a fixture.

### Decisão e responsabilidade exclusiva

Acrescentar apenas parâmetro interno resolved_args: Option<Args>. Usar os Args transportados pelo fallback mutante ou avaliar args normalmente uma vez. Remover is_mat_call por nome, isolate_mat_args, cópias isoladas de Scopes e reparos por array_lengths/prefix/truncate: efeitos legítimos do usuário não são deduplicáveis por conteúdo. Preservar todos os metadados/spans de chamadas, match fechado, callbacks, With, traces, Content/ShowRecipe e contratos matemáticos de inserção já existentes.

Consumir o núcleo eval/call-once.toml pinado. Mudança mecânica interna e
correção de paridade ADR-0127 em fluxo contínuo. Assinaturas pub fn/pub use e
structs públicos não mudam; os parâmetros novos/retornos/helpers acima têm
visibilidade crate/eval, sem novo trait público, campo público, default ou
fase. Esta cláusula supersede somente restrições históricas conflitantes de
assinatura interna e compensação por nome; manter extensões Tekt existentes.

### Aceitação e limites

Testes funcionais co-localizados antes do código: efeitos de receiver/args,
fallback mutante, erro antes de args, where/with/selector/coleções preservados
e matriz sem truncar efeito legítimo. Confirmar RED funcional e GREEN.
Comparar casos bilaterais válidos e controles; hits e misses de todos os
métodos de coleção, erros, spans/occurrences e complementos de Value. Não
mascarar dívida residual de diagnóstico como igualdade. Testar dispatch
Unhandled sem perder receiver ou executar callback de args; contar avaliações
pelo efeito observado, não por bytes de PDF, Rust Eq ou microbenchmark.

Ownership/V26 antes de resselo só dos owners do lote. Build, suíte workspace,
Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
fmt, V1/V5/V15/V26 e reverse, SARIF diferencial sem erro novo. V19 uniforme
continua informativo, com matriz de alternativas e relação de comportamento;
sem supressão nem mudança de configuração. Comandos pesados sequenciais com
12 GiB/high10GiB/swap0/jobs1. Não publicar/commit/merge/release nem declarar o
objetivo global fechado por esta etapa.


### Refinamento causal: callee não chamável antes dos argumentos

A sonda control/30 de before-probe.json.gz mede m.data(panic(...)), onde data
é inteiro: Tekt executa panic e o pin retorna expected function, found integer
antes dos argumentos. Separar preparação interna de Args/spans em helper privado
prepare_call_args, invocado somente nos braços Func e Type do match de callee.
Transportar resolved_args ou avaliar AST uma vez; devolver Args e spans HTML
privados já calculados. Não alterar definição/campos de Func ou criar novo tipo
chamável. O complemento nominal das outras 37 variantes de Value retorna
expected function, found <vanilla_type_name> no span do callee, sem preparar args.
O match final torna-se exaustivo e seu curinga antigo é removido; a eventual nota V19 do
complemento é uniforme por rejeição antes de efeitos, sem inferir cobertura
por execução de uma só alternativa. Acrescentar controles de categorias de
Value e preservar misses/builtins anteriores. Eventuais hints de shadowing de
stdlib continuam frente de diagnóstico própria se não medidos nesta etapa.


Matrizes de aceitação do complemento V19: call_once_unhandled_keeps_every_value_and_does_not_evaluate_arguments e call_once_callable_classification_covers_all_value_variants_before_arguments constroem 39 discriminantes distintos de Value (36 via linguagem e Path/Location/LocatedContent pelos construtores puros RootedPath/Locator/IntrospectedContent existentes). A primeira testa retorno Unhandled para nome desconhecido, igualdade do payload transferido, ponteiro do Vec original e callback de args que panic se executado; a segunda testa 37 rejeições antes de args e os dois caminhos Func/Type com panic real do argumento. São testes do transporte/classificação interna; não certificam que toda categoria tem a mesma capacidade de cast upstream, nem paridade geral de todos os schemas.


### Forma plana do complemento — medição V20

candidate-4/validation-architecture.log e differential-review.json medem nota
V20 nova por captura target/other @ (grupo de variantes) com profundidade 3.
A captura é dispensável: guardar o local mutável em variável target antes do
match e referenciá-lo no complemento, ou referenciar o callee original cujo
payload foi ignorado. Manter os grupos explícitos planos, os mesmos membros e
retornos, sem supressão de V20/V19 e sem duplicar classificação. As matrizes de
39 alternativas, os efeitos e o ponteiro original devem permanecer iguais.


## R2-S — realização nominal de Sequence, ratificada em 2026-10-17

### Medição anterior à decisão

As coletas before/expanded/controls.json.gz e candidate.log em
00_nucleo/diagnosticos/sequence-constructor-2026-10-17 fixam HEAD
b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado,
UTC/diffstat/fontes/binários e pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
Oito REDs iniciais; cinco passaram no candidato, show e recursão ainda não.
markup_stream.rs:240–260 aplana antes de selecionar a sequência; realize/mod.rs:
170–210 visita filhos antes de receita; apply.rs:140–170 delega não-localizáveis
ao ponto-fixo. O vanilla aceita alias e rejeita nova Sequence com mesmos
filhos. São efeitos, escopo, valores e diagnósticos de linguagem.

O dono aprovou a revisão
00_nucleo/materialization/sequence-realization-2026-10-17/review.md, manifesto
46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327,
nesta sessão. O gate ADR-0127 §2.3 cobre a ordem de realização desta nova
fronteira antes dos filhos/Table/math; não cobre novo contrato público.

### Responsabilidade exclusiva

Com regra Sequence ativa, resultados crus Sequence/Empty de chamada mantêm
a identidade/Source do call.span e seguem para a fronteira de inserção, sem
a antecipação de recipe genérico no retorno guardado em let. Demais resultados
e caminhos sem regra Sequence preservam a intercepção atual. Não executar
recipe por leitura de fields/func ou pelo nome da função.

### Aceitação

S1–S20 ratificados, RED antes de código, GREEN bilateral e regressões; não
reduzir expectativas por limite mecânico. Todos os owners 1:1 e núcleos pinados
válidos antes do resselo. Build/workspace/doctests separados, Clippy/Rustdoc
estritos, fmt/linhagem/reversa e SARIF sem novo erro/warning. 12GiB/High8GiB/
swap0/jobs1 e comandos pesados sequenciais. Sem publicação/release.
