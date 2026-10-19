# L0 — stdlib/collections
Hash do Código: 9cc7e161

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/collections.rs:37` (pubcrate) struct CollectionCallSpans {); `01_core/src/compiler/stdlib/collections.rs:66` (pubcrate) fn try_dispatch_collection_method); `01_core/src/compiler/stdlib/collections.rs:180` (fn try_dispatch_bytes_method) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own CollectionCallSpans, try_dispatch_collection_method, collection_type_field, static_collection_call and the static operation catalogue. The algorithmic owners in `collections/*.md` own argument validation and collection behaviors for Array, Dictionary, String, Bytes and Arguments. Field access/call-dispatch owners discover and forward operations; l-value writeback belongs to the binding owner. Preserve the same native operation identity and self-first calling convention. Do not duplicate algorithms in eval.

### Public operations and defaults

Array: all/any/filter/find/position(test), map(mapper), fold(init,folder), reduce(reducer), at(index,default:?), first/last(default:?), len, contains(value), chunks(size,exact:false), windows(size), dedup(key:?), enumerate(start:0), flatten, insert(index,value), intersperse(separator), join(separator:none,last:?,default:none), pop/push(value), product/sum(default:?), remove(index,default:?), rev, slice(start,end:none,count:?), sorted(key:?,by:?), split(at), to-dict and zip(exact:false,..others). Preserve global/static range(start:0,end,inclusive:false,step:1) linkage; range implementation belongs to foundations/cast.

Dictionary: at(key,default:?), filter(test), insert(key,value), keys/values/pairs, len, map(mapper), remove(key,default:?). Preserve the separate crystalline update extension. String: at(index,default:?), first/last(default:?), len, clusters, codepoints, contains/ends-with/starts-with/find/position/match/matches(pattern), normalize(form:"nfc"), replace(pattern,replacement,count:?), rev, slice(start,end:none,count:?), split(pattern:none), trim(pattern:none,at:?,repeat:true). to-unicode/from-unicode stay delegated to foundations/str. Preserve crystalline char-len/char-at/char-slice, repeat and to-upper/to-lower extensions without substituting them for vanilla names. Bytes exposes only len, at(index,default:?) and slice(start,end:none,count:?), not first/last. Arguments exposes at(key,default:?), filter(test), len, map(mapper), named and pos.

? denotes an optional named value absent when omitted. Preserve argument order, named validation, duplicate causal carriers and first-error precedence; no silent acceptance of extras or coercion by truthiness where Bool is required.

### Semantics

all/any short-circuit; all/any/find/position/filter and sorted(by:) require Bool results. map/fold/reduce preserve order. flatten is recursive, superseding the old one-level prose. dedup keeps the first global occurrence and supports key. sorted is stable and supports key and by together. chunks exact discards the incomplete remainder; windows produces only complete windows; zero sizes fail. zip truncates to the shortest by default, exact rejects unequal lengths, and no additional arrays yields singleton tuples.

Negative indexes count from the end. Defaults apply only to absence/out-of-bounds, not invalid casts; insert accepts the terminal boundary. Preserve bound l-value mutation and result/error semantics for push/pop/insert/remove; unbound projections do not mutate remote bindings. sum/product use language operations and require a default on empty inputs; reduce on empty returns none. join uses language join with the distinct final separator and empty default. Dictionary iteration preserves insertion order; to-dict requires (str,any) pairs and last duplicate key wins.

String indexing/positions/match offsets are UTF-8 bytes. Reject invalid character boundaries rather than clamping at/slice; first/last are scalars, codepoints are scalars and clusters are extended graphemes. find returns the matched substring or none, position its byte index or none. match/matches return start/end/text/captures, nonoverlapping and ordered; preserve literal/regex and replacement/trim/split distinctions. normalize supports nfc/nfd/nfkc/nfkd, default nfc. Bytes counts octets, at returns 0..255 Int and slice returns Bytes.

### Causal Arguments

Static forwarding removes only the receiver through the coherent Args API. Arguments.filter/map iterate occurrence_sequence in joint positional/named order, including legal duplicate names. Filter invokes once per occurrence, requires Bool and retains the original occurrence when true. Map invokes once, keeps name/argument span, replaces value and detaches value_span even if the new Value compares equal. Return Args::from_occurrences with detached aggregate; do not discard a user's carrier to use legacy separated views.

Arguments.len counts occurrences when Some, otherwise items+named; this does not redefine Rust Args::len. pos/named return coherent views; at uses positional index or last named lookup. Filter's return-type cast uses the Func diagnostic_span and long type names; callback-produced errors keep their own origins. Synthetic/detached functions stay detached. Preserve first failure, With/alias origins and one callback invocation per occurrence. Map does not inherit old lexical origins by equality.

CollectionCallSpans remains private call metadata for the specifically supported byte-method corrections, not a universal source-origin policy. Full messages, hints, traces and spans remain observable; canonical repr belongs to its eval owner.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## Atomização comportamental P1497

**Medição:** a working tree não commitada identificada em `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contém 3542 linhas neste consumer; a suíte do L1 passou com 6401 testes e 2 ignorados. Esta medição não é benchmark nem nova comparação com o vanilla.

**Decisão:** manter neste owner o despacho exaustivo, a tabela estática de membros, os adaptadores de chamada estática e os metadados sintácticos de chamada. Os algoritmos são delegados aos owners exclusivos `collections/{validation,bytes,array_access,array_aggregate,array_predicate,array_sequence,dictionary,str_access,str_search,str_transform,arguments}.md`. As cláusulas de comportamento acima são o mapa de superfície; cada owner filho especifica sua família. A separação é de código e L0, sem alteração intencional da linguagem.


## Caminhos residuais internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/stdlib/collections.rs` nas linhas 519, 553 contém os itens sem consumo/visibilidade sinalizados pelo compilador. A pesquisa de usos e os dispatchers ativos estão registrados antes da edição em `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`, com fontes, L0, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status e diffstat. Isto não é nova medição bilateral de paridade.

### Decisão e aceitação

Eliminar os wrappers privados não registrados native_array_all_static e native_str_clusters_static. Preservar o dispatcher e os wrappers collection_static ativos, array_all_with_pred e str_clusters; array.all e str.clusters continuam expostos pelo mesmo caminho com a mesma avaliação/validação. A primeira compilação após remover os wrappers mostrou um import órfão de array_all_with_pred no hub; remover só esse import. A função continua chamada por array_all dentro de array_predicate.rs.

Correção interna em fluxo contínuo ADR-0127, sem nova assinatura pública, default de produto ou fase. Preservar os resultados de linguagem e as expectativas dos testes. Revalidar todos os targets, suíte core, build, Clippy e linhagem; não suprimir warnings para substituir a correção.

### Medição anterior à decisão — catálogo V16, 2026-10-17

lab/results/construction/diagnostics/tekt-v16-catalogos-2026-10-17/before.json.gz e
before-measurement.json.gz fixam HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, UTC/status/diffstat, fontes/L0/config e binários.
O inventário contém118 campos,115 exercitados pelo CLI e três Location por
teste no módulo. Há159 pares/318 invocações:141 coincidem em código/stdout/
erro principal+hints,18 divergem no diagnóstico já existente. As funções/
constantes inventariadas exercitadas são expostas nos dois backends. Isso
não prova que todo campo omitido deva estar ausente upstream. O pin é
9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Type tem38 variantes e Value39 no value.rs deste snapshot. Os wildcards
sinalizados misturam família fechada e nome aberto de campo/método. O estado
e as localizações anteriores estão no inventário e no before.json.gz.

### Decisão deste owner

try_dispatch_collection_method deve classificar Value exaustivamente antes
do lookup aberto de method:str. Array/Dict/Str/Args conservam os mesmos
algoritmos/RHS, argumentos, chamadas de callback e resultados Some(Ok/Err);
Bytes continua delegado integralmente ao owner filho, inclusive metadados
de span. Nome de método não reconhecido em uma família retorna None para
o caller continuar; erro de método reconhecido continua Some(Err), nunca None.
O grupo V19 das outras34 variantes Value é coeso: nenhuma é receptor de
coleção deste dispatcher. Não apagar informação por curinga de domínio.

collection_type_field deve classificar Type exaustivamente: Array/Dictionary/
Str/Bytes/Arguments mantêm seu catálogo, identidade de callback e ABI. O
grupo V19 das outras33 variantes Type é coeso por ausência de catálogo
de coleção neste owner; não representa ausência universal de membros.
Dentro de cada família, str continua aberto e desconhecido retorna None.

Não mover algoritmos, alterar static_collection_call/self-first/writeback
ou validators. Preservar as extensões cristalinas declaradas. Não adicionar
campos ausentes do pin ou esconder dívida de diagnóstico como equivalência.
Uma variante nova de Value/Type exige classificação pelo compilador.

### Aceitação e classe

Correção interna estrutural ADR-0127 em fluxo contínuo, sem nova API, default,
compatibilidade ou fase; intenção de runtime preservado, não nova certificação
universal de paridade. O RED é estrutural: padrões produtivos antigos aceitam
variante futura; após a mudança, o controle atual compila e a variante futura
é rejeitada E0004. RHS apagados nos probes apenas para isolar cobertura de
padrões; payloads são stubs, não testes de equivalência de execução. Não
mutar workspace/target Cargo para testar variantes futuras. Os controles
funcionais no módulo passam antes/depois; não alegá-los RED funcional.

Quatro V16 dão lugar a três seleções de família fechada (o primeiro owner
combina seus dois matches). Verificar159 pares antes/depois com stdout/
stderr completos preservados dentro de cada backend, inclusive18 negativos
já divergentes. Retê-los no inventário para correção própria. Nenhuma supressão
ou relaxamento global do linter. Testar identidades de funções de mesmo nome,
constante versus função, misses cruzados e Location. L0 primeiro, selos só
dos dois owners após V15/V26 válido; build/suíte workspace/Clippy -D warnings
/-D unfulfilled_lint_expectations, Rustdoc/quatro índices, formato/diff,
linhagem estrita/reversa e SARIF diferencial sob12GiB/high10GiB/swap0/jobs1.
Sem commit/push, merge ou release.


## Avaliação única em chamadas — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/call-once-2026-10-17/before.json.gz registra UTC,
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

Manter try_dispatch_collection_method com assinatura vigente como adapter para args já avaliados. Adicionar CollectionDispatch { Handled(SourceResult<Value>), Unhandled(Value) } restrito ao crate e try_dispatch_collection_method_with com o mesmo catálogo estático, recebendo FnOnce(&mut Scopes, &mut EvalContext, &mut Engine) -> SourceResult<Args> em vez de Args. Uma macro local bind_args(args, expr) resolve args somente no braço reconhecido e retorna Handled; falha de args propaga SourceResult. Nome desconhecido devolve receiver original (reconstruído por move na família) em Unhandled; sem clones/probes de execução/segunda tabela. O complemento nominal Value mantém enum exaustivo. Integrar somente os três nomes bytes len/at/slice na tabela principal, delegando aos mesmos primitives; não mover algoritmos. Adapter eager converte Handled a Some e Unhandled a None. Catálogo Type/static wrappers e extensões Tekt preservados.

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


Matrizes de aceitação do complemento V19: call_once_unhandled_keeps_every_value_and_does_not_evaluate_arguments e call_once_callable_classification_covers_all_value_variants_before_arguments constroem 39 discriminantes distintos de Value (36 via linguagem e Path/Location/LocatedContent pelos construtores puros RootedPath/Locator/IntrospectedContent existentes). A primeira testa retorno Unhandled para nome desconhecido, igualdade do payload transferido, ponteiro do Vec original e callback de args que panic se executado; a segunda testa 37 rejeições antes de args e os dois caminhos Func/Type com panic real do argumento. São testes do transporte/classificação interna; não certificam que toda categoria tem a mesma capacidade de cast upstream, nem paridade geral de todos os schemas.
