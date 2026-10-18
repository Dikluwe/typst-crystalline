# L0 — infra/pipeline/context_stabilization
Hash do Código: 6c5d98a8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compilation-metric-cache.toml sha256:b4372cbdd36e0e25377d1258638a6859bcc88c9e584b91b377a2a12d0d66135f
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:bdbe23b72d7083b7672d71e851dc3d35baa8ed7ccb66f009f61edcf770d0aeff
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0


**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/context_stabilization.rs`

## Current evidence

- `03_infra/src/pipeline/context_stabilization.rs:42` — `Session`.
- `03_infra/src/pipeline/context_stabilization.rs:651` — `prepare`.
- `03_infra/src/pipeline/context_stabilization.rs:690` — `prepare_html`.
- `03_infra/src/pipeline/context_stabilization.rs:748` — `prepare_bundle`.
- `03_infra/src/pipeline/context_stabilization.rs:897` — `bundle_page_views`.
- `03_infra/src/pipeline/context_stabilization.rs:1043` — `strip_executed_html_markers`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/context_stabilization.rs`.

## Owned contract

- Own only the private per-compilation session: causal discovery, producer contributions, attempts, sinks, retained observations, replay and terminal decisions. Use the existing evaluation/introspection/layout facades; never duplicate selector matching, counter/state folds or Value equality in L3.
- Selection requires an actually reached observation, not AST inspection, variable naming, error substrings or the mere presence of a selector. A failure before demand retains ordinary legacy treatment. Nonselected producers retain their ordinary contribution/error during additional attempts.
- Preserve producer identity, capture, Location, input style chain, target, features and Engine resources per attempt. Replace a producer/closure/chain by invalidating its descendants; numeric context id or Debug/hash of a closure cannot certify reuse. Replay uses captured input styles, not the chain left by execution.
- For selected filtered-counter generations, discard discovery execution/sink and start with empty observational views I0 while retaining origin Locations. Discovery is not an extra stabilization attempt; selection stays latched even if a later branch makes no read.
- Attempt A_k reads I_(k-1), builds current content/document and a complete I_k (runtime introspection, positions and PageStore), then validates retained actual reads against I_k. Reexecute only invalidated selected contributions and descendants; discover real newly produced blocks recursively. Reconstruct from original content/current contributions without accumulating updates or markers. Failed bodies keep only their marker, not stale successful output.
- Reuse only complete valid records with unchanged causal inputs. Equal final values, equal page counts or a lone selection flag do not prove stability. All retained contributions must describe the same candidate generation.
- Share the existing five-attempt budget with paginated invalidations. On validated convergence return the candidate or its pending original error. For proven linguistic nonconvergence without final error, return D5 calculated from I4 and diagnostic history I0 through I5; do not execute A6, substitute replay from I5 or return D4.
- Publish only warnings of retained final executions, once and in causal order; discard validation/replaced/discovery sinks. Preserve original error spans/traces and precedence. Do not reorder by HashMap or infer error classes from text.
- Inability to prove required stability is not linguistic nonconvergence. At the terminal budget, return Error `contextual stability could not be verified`, anchored at `source.root().span()`, without fabricated trace/hints or provisional export. Proven ambiguous identity/topology may terminate at discovery. Original pending errors take precedence; prior global warnings remain.
- HTML preparation preserves causal Html target/features and strips executed context markers only in the terminal serialization view. Residual contexts, unobservably paged dependencies and closure errors fail closed, never become Empty or successful output.
- Standalone HTML position stabilization is selected only by observed LocationPosition, not filtered-counter selection. Recursively discover provisional descendants; discard selected discovery contributions, retain unselected ancestors. Seed with the original HTML introspector and empty HtmlPosition store, not an empty global introspector that would erase query/state.
- Latch the HTML phase after the first observed position read. Every later generation must build and compare its full store, even when producers disappear; nonempty-to-empty requires a further empty-to-empty confirmation. Reintrospect marker-plus-result content, measure only intrinsic islands and validate both full store and observations under the same five-attempt budget.
- Bundle preparation reconstructs/reintrospects all ordered bodies against one global candidate with causal seeds, target/style/features/resources per body. Keep Page boundaries per body in pure and runtime snapshots; local Page reduction only initializes physical pagination and is not a query/validation snapshot.
- Paged bodies contribute positions and PageStore; HTML bodies contribute separate intrinsic HtmlPosition stores without page offsets. Validate the complete set before terminal HTML projection or any export; never promote a body settled under an earlier generation.
- Without selected bundle producers, realize the final paged view once before terminal HTML projection to supply PageStore. With selection, reuse the validated store without a second realization. HTML consumes no physical pages.
- Keep CounterUpdate nodes in candidate snapshots; remove executed HTML updates only in terminal projection. Preserve SpaceFromNewline origin until causal L1 normalization; only an all-whitespace terminal Sequence of Space/SpaceFromNewline collapses to Empty.
- Layout/callback/numbering and intrinsic-position helpers keep their own owners. Do not add an observation ledger, boundary hook or instrumented default.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Realização com avaliação única

Ordenação causal dos produtores reconhece ContextBlock na raiz de Located; a coleta não perde o produtor porque a travessia visual evita callback duplicado da raiz transportada.


## Realização com avaliação única

Fallback legado de produtores também reconhece ContextBlock preparado dentro de Located, conservando uma avaliação por id quando a seleção ainda não exige outra sessão.


## Realização com avaliação única

Após expansão HTML validada, remover marcador contextual e counter-update também quando carregados em Located; sem descartar produtor não executado silenciosamente.


## Reuso sem transformação — 2026-10-05

`Session::content` em `03_infra/src/pipeline/context_stabilization.rs:285–292` chama substitute_context_blocks mesmo quando roots está vazio. A descoberta exaustiva existente fornece essa condição; não inferi-la da sintaxe ou do introspector. Com zero roots, devolver clone do origin conservando os Arcs, sem fabricar mapa/substituição. Com roots, manter a reconstrução e todas as regras de validação, contribuição, erro, warning e normalização vigentes. Não criar outro gate de seleção nem compartilhar contribuições entre gerações. Correção interna em fluxo contínuo ADR-0127; exigir RED→GREEN para conservação do carrier em sessão sem produtores e revalidar produtores reais.




## Snapshot realizado de Page — ratificado em 2026-10-05

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree e binários exatos em `00_nucleo/diagnosticos/contexto-page-total-2026-10-05/measurement.json.gz`. single imprime B1/0 F0 contra B1/1 F1; duas páginas total/final 0 contra 2; update inicial 7 conserva total 7 contra 8; update intermediário altera página anterior (7/7 e 8/7 contra 1/7 e 7/7); step final imprime final 1 contra 2. Bundle imprime 1/7 no primeiro PDF contra 1/1, e final 7 no segundo contra 8. HTML misto devolve 1 contra 7. `03_infra/src/pipeline/paged.rs:360–365,840–913` usa o último update como início de todas as páginas; `context_stabilization.rs:969–1010` faz o mesmo por body. `compiler/stdlib/counter.rs:490–513` e `counter/display.rs:96–110` leem final global causal. Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, `lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254,730–773,934–963`, combina escopo documental, updates e avanços físicos. A classificação é paridade de linguagem; não exige copiar a mecânica de memoização upstream.

Decisão proposta: aplicar o Núcleo page-counter-snapshot. Esta unidade substitui o scope-out histórico de final apenas após ratificação. Contrato público e realização page-aware de updates exigem gate ADR-0127; não materializar ou ressellar antes da confirmação. Preservar as assinaturas existentes, ciclos e budget; não adivinhar página/total antes do snapshot conhecido.

Fornecer ao fold puro as ocorrências por body e páginas/posições da contribuição candidata. Preparar snapshot Page depois de conhecer posição física; reconstruir e validar sob o mesmo orçamento. Não derivar início de todas as páginas do último update. Não reexecutar fonte inteira, não carregar caches de outro documento e não deslocar I/O para L1. Numbering/PageStore e margens consomem a numeração realizada pertinente, sem contaminar o snapshot global com projeção local de exportação. Erros e warnings respeitam retenção dos produtores. Implementação de outros owners exige L0 próprio atualizado antes de código; este owner não legitima render ou handlers de counter.


## Materialização do conjunto ratificado — 2026-10-05

Usar Engine L3 privado para a fachada ratificada. Partição bundle é registrada no walk por pertencimento explícito de ocorrências; os checkpoints servem apenas ao layout. Comparar ambas as vistas Page na estabilidade de gerações. HTML Page pode selecionar-se por replay observado sem alterar has_filtered_counter_reads ou o contrato público de EvalContext.

### Total das margens em bundle

O total lógico de uma margem vem do snapshot final de uma ocorrência com posição extraída pelo próprio documento. Em documento sem ocorrência posicionada, usa o último número lógico daquele documento. Não usa a fronteira artificial do Locator nem uma ocorrência de outro body. A enumeração de posições inclui relações parent_locations.

Ratificação: o humano confirmou o conjunto do manifesto `00_nucleo/diagnosticos/contexto-page-total-2026-10-05/l0-manifest.json` (SHA-256 1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e). O gate ADR-0127 deste conjunto está satisfeito. Clarificações internas subsequentes seguem fluxo contínuo, L0 antes do código e RED→GREEN.

## Métricas compartilhadas na sessão contextual — 2026-10-05

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado preservado em `00_nucleo/diagnosticos/contexto-cache-fontes-2026-10-05/before-state.json.gz`, binário baseline 5bc45f4ec0972c6c46450c0cab3479e2ae2ef41c286c424201c1b7fc07823eb1. Sonda GDB `count-baseline.json.gz`: selected_distinct fez 2243 consultas, 965 cálculos de shaping após miss e duas entradas no wrapper; selected_repeated teve 8 misses; controle ordinário 321 misses. `context_stabilization.rs:548` chama o wrapper sem métricas; `paged.rs:656–661` cria FallbackFontMetrics quando o handle é ausente. `font_metrics.rs:71–96` cria caches Arc por instância e Clone compartilha-os, conforme seu L0 proprietário. Há assimetria com o caminho ordinário já compartilhado. O relatório `grid-cache-share-ab-2026-10-03.md:17–23` delimitou explicitamente o compartilhamento ao caminho comum, mantendo os wrappers contextuais e de bundle com métricas novas. Esta etapa amplia aquele escopo depois da medição; não classificar esse limite documentado como decisão por inércia. A origem mais antiga do wrapper fresco permanece não auditada.

Decisão interna, fluxo contínuo ADR-0127: a sessão contextual paginada selecionada pode fornecer o seu handle metrics já existente ao wrapper de layout. Cada tentativa conserva conteúdo, introspector, regiões, callbacks, transcripts, sinks e budget próprios. Somente resultados de métricas por texto/tamanho/estilo/fontes no mesmo World são compartilhados. O handle morre com a sessão; nenhuma retenção entre compilação/edições, nenhum cache de Content, frames ou execução contextual. Bundles/HTML, reconciliação final selecionada e demais callers preservam seus recursos atuais. Não copiar algoritmo/chaves de fonte em pipeline. Não alterar contrato público, modo ou fase. Manter a mudança somente com redução demonstrada de trabalho, medições A/B e controles de linguagem/geometria/diagnósticos sem regressão.

## Transporte interno de métricas até a reconciliação contextual — 2026-10-05

Medição anterior à decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado e binário baseline 17201a15e36b06bb3a9ac8adba0a98c61b3dc925c4ce1f118a39d26bce26cf05, preservados em `00_nucleo/diagnosticos/contexto-cache-reconciliacao-2026-10-05/before-state.json.gz`. A sonda count-baseline.json.gz isolou a entrada em fix_line_positions_with_metrics: selected_distinct fez 321 consultas/321 cálculos nessa etapa (643 cálculos totais), enquanto ordinary_control fez 320 consultas/zero cálculos. `context_stabilization.rs:14–17,660–663` entrega Content, introspector e documento, descartando o handle da Session; `paged.rs:337–345,563–572` mantém None no caminho selecionado e chama reconciliação com métricas frescas. O owner `infra/shaper/reconciliation.md` já permite helper interno emprestando métricas do mesmo World/compilação; equações e ABI pública não mudam.

Decisão interna, fluxo contínuo ADR-0127: transferir um clone do handle existente juntamente com o candidato completo aceito da sessão paginada. Prepared e prepare são internos ao módulo pipeline; seu lifetime explicita os empréstimos existentes de World/Source. Nenhuma API pública ou campo de entidade L1 muda. Preservar o contrato de aceitação, erros, contribuição, sinks e budget (inclusive o resultado final aceito sob não convergência provada); somente o braço que já produz settled transporta métricas. Resultados sem settled, HTML e bundles não transportam handle.

O coordenador paginado usa o handle selecionado somente em fix_line_positions_with_metrics. Bidi selecionado continua com métricas frescas nesta etapa, isolando a contribuição causal da reconciliação; o caminho ordinário mantém seu handle nos dois consumidores. Nenhuma execução/layout adicional, nenhum cache de frames ou snapshots, nenhuma retenção entre compilações. Manter invariantes do Núcleo compilation-metric-cache, com lifetime encerrado pelo coordenador paginado. Esta seção amplia o limite anterior de descarte no fim da Session apenas para esse transporte. Refutar a mudança por output/posição/diagnóstico diferente ou ausência de redução comprovada de trabalho/tempo.

## Descendentes paginados sem seleção — correção interna 2026-10-06

Medição antes da decisão: query-compilada-2026-10-06/green-cli-focused.log,
query_compiled_context_errors_preserve_origin_trace_and_deprecation retorna
sucesso para #context [#context unknown]. Snapshot initial-source-state fixa o estado não commitado; a tentativa
de coleta debug initial-materialization.log foi interrompida durante rebuild
e não é usada como prova. A falha RED da suíte precede o reparo. discover em
context_stabilization.rs:257–269 chama realize apenas para Html; body substitui
ContextBlocks filhos sem execução quando nenhuma observação seleciona fixpoint.
Vanilla pinado reporta erro <stdin>:1:19. É omissão na descoberta de produtores,
sem nova fase/assinatura; não copiar algoritmo ou reavaliar programa inteiro.

Descoberta em Paged também realiza recursivamente descendentes efetivamente
produzidos, usando realize já existente após execução do root, se identidade
não estiver impossible. Root com invalid=false não é reexecutado. Só corpos
realmente produzidos são visitados; preservar guards, styles, captura, Source,
span/trace, allocator causal e seleção por leituras observadas dos filhos.
Manter tratamento de ids legados e cinco tentativas; não fabricar fixpoint para
um produtor sem leituras. O gate é RED→GREEN do contexto aninhado e revalidação
integrada; nenhuma API, instrumentação ou warning policy nova.

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

Após apply_func do produtor retornar seu Content, e com o Engine causal ainda
vivo, delegar à fachada L1 realize_math_insertions antes de guardar a contribuição,
descobrir filhos e reintrospectar. Passar ctx, styles/rules/guards/Source/sink já
existentes; não fazer reconhecimento de Mathy em L3 nem duplicar normalização,
matching ou fold de contadores. Conservar efeitos uma vez por execução válida,
observações, alocadores, retenção de warnings/erros, cinco tentativas e targets.

Nova contribuição matemática recebe as identidades L1 pertinentes sem reciclar
ID de produtor como Location de Equation e sem usar snapshot final de styles
para substituir o de inserção. Contribuição aceita reutilizada não é realizada
novamente para duplicar efeitos/ocorrências; nova execução substitui o resultado
anterior sob a política vigente. A assinatura pública desta unidade não muda.

### Gate e aceitação

Somente materializar após confirmação humana do pacote math-bare-2026-10-06.md.
RED→GREEN e revalidação bilateral do escopo são obrigatórios. Esta cláusula
supersede restrições de fase anteriores apenas nas fronteiras aqui nomeadas,
sem ratificar paridade universal, D07, math label ou outros débitos.

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

Depois de produzir Content no Engine causal, delegar realize_table_insertions e realizar contribuição matemática pelo owner atual antes de guardar/discover/indexar. Captura Table/Cell preservada; não classificar tipos/props em L3. Contribuições aceitas/repetidas não executam receitas novamente; preservar budget/leituras/sink/observações e alocadores.

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

## Snapshots de entrada compartilhados — medição 2026-10-08

`00_nucleo/diagnosticos/memoria-fases3-2026-10-08/profiles.json.gz` e
profiles-growth.json.gz, HEAD/árvore/horas completos em state.json.gz,
registram ~196 MiB de pico em context300 versus ~46 MiB em plain300;
prepare sobe de ~62,5 para ~138,7 MiB. `context_stabilization.rs:196,244`
no estado before.json.gz da rodada context-snapshots-memoria4 duplica
input para o ctx mutável e novamente para o registro imutável de cada
Execution. TagIntrospector Clone copia coleções próprias, além de Arcs.
Classificação: cópia mecânica privada. Inferência: compartilhar a segunda
vista por geração reduz custo; refutação por teste de independência,
replay alterado, ou ausência de economia pareada. Não inferir leak de RSS.

Conservar ctx.introspector próprio por Execution, inclusive alterações
feitas durante execução. Execution.input pode ser Arc<TagIntrospector>
privado, compartilhado somente entre produtores/descendentes que consumiram
a mesma entrada concreta da geração. Na descoberta materializar um snapshot
próprio; em cada tentativa outro snapshot do input histórico, sem alterar
Vec histórico, budget ou fachadas L1 de diagnósticos. Nunca deduplicar por
igualdade, hash, número de páginas ou id. Registro substituído e retired
conservam o Arc de sua entrada original; a próxima geração não o modifica.
HTML cria vista própria antes de substituir contadores Page para replay.
Não compartilhar ctx mutável, não apagar gerações/retired ou antecipar
reuso de contribuição. Não mudar assinaturas públicas, fase ou defaults.

Aceitação RED→GREEN: identidade física compartilhada nos registros da mesma
entrada, isolamento do ctx mutável e das gerações novas/retired; observar
identidade física somente como testemunha de custo, não gate de linguagem.
Suite contextual/infra inteira, controles PDF/HTML e erros/replay/budget
existentes, builds/lint e bateria de memória/tempo antes/depois.

## Estado de avaliação após a execução — medição 2026-10-08

Medição antes da decisão: depois da equivalência de fluxo, `context300`
mediu cerca de 95 MiB no Tekt e 31 MiB no vanilla. Na sonda de fases,
o RSS subiu de 40,7 para 80,1 MiB durante 300 chamadas de `execute` na
preparação, com acréscimo próximo de 130 KiB por produtor. A tentativa de
descartar `retired` teve mediana de 94 892 KiB, contra 94 826 KiB antes,
e foi revertida. Comandos, binários e estado estão no diagnóstico do item 3.
`context_stabilization.rs:196` clona o introspector para o `EvalContext`
de cada produtor. Depois de `apply_func` e da realização das inserções, os
usos produtivos de `Execution.ctx` são validação/replay e os campos de
target/features/erro; `compiler/eval/context_reads.rs:232-265` cria um novo
contexto com clone do candidato para cada leitura repetida. Nenhum uso
posterior consulta `Execution.ctx.introspector`. Classificação: retenção
mecânica privada; refutar por output/replay alterado ou ausência de ganho.

Preservar `ctx.introspector` próprio e mutável durante toda a avaliação,
incluindo alterações feitas por closures, show rules e realização. Após a
avaliação terminar e antes de guardar `Execution`, libertar apenas o campo
`ctx.introspector`; conservar observações registradas, input `Arc`, resultado,
sink, location, features e parâmetros de replay. `retired` conserva os seus
registros históricos causais, mas os seus contextos já não carregam a vista
introspectiva descartável. Não alterar o contrato público de `EvalContext`,
o histórico I0–I5, cinco tentativas, ordem, fase ou defaults. Aceitar somente
com teste RED→GREEN de liberação após execução, suite contextual, equivalência
de PDFs/erros e redução mensurada de memória.

## Ordem contextual vazia sem reconstrução — ensaio interno 2026-10-09

Medição anterior à decisão: `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/cpu-style-current.json.gz` (HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, release SHA-256 `14d023d36fb765eb041f24f440eaad3bc5d67183325b611e4fd98502bd26815e`, UTC/estado/fixture/comandos no recibo) contém 76 amostras de `ordered_blocks` no caso `local-set-1600` e 25 em `no-fill-1600`, em 12 compilações cada. O gate RED `ordered-blocks-empty-red-gate.json.gz` usa o release já corrigido para referências vazias, SHA-256 `6765fa1113676347dd00007bec1ba1f4e35bf49337d01d0e033d662091c462c4`, e falha no limite predefinido de 523.000 pedidos glibc: 527.107 no caso local e 402.236 no controle. O tempo isolado deste gate sofreu carga externa e não serve como prova de ganho. A fonte `03_infra/src/pipeline/context_stabilization.rs:74-99` primeiro coleta os produtores por id e depois percorre todo o Content por `map_content` para ordená-los e detectar repetição.

Se a coleta exaustiva `by_id` estiver vazia, `ordered_blocks` pode devolver imediatamente `Some(Vec::new())`, sem reconstruir Content. Esta prova só vale porque a coleta do owner pipeline percorre a projeção visual e todos os carriers usados pelo ordenador; se um carrier puder ser visto pelo segundo percurso e não pela coleta, a saída antecipada é refutada. Com qualquer produtor coletado, manter integralmente a ordenação, a detecção de IDs duplicados e `None` que ativa o fallback legado. Verificar caso vazio aninhado em tabela, dois blocos com o mesmo id, bloco distinto e Realized com produtor somente em original. A mudança é privada de L3: não altera fase, contrato público, budget, seleção contextual ou semântica de warnings/erros. Manter somente com gate RED→GREEN, equivalência e ganho pareado de tempo.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/pipeline/context_stabilization.rs:926 (C096).
O snapshot before.json.gz em 00_nucleo/diagnosticos/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

O sucesso interno de prepare_bundle usa PreparedBundle privado à família pipeline, com campos bodies, introspector e document_starts. Conservar os mesmos Vec/TagIntrospector/LocatorSeed por movimento, ordem e geração validada. O retorno continua separando SourceResult e warnings, com erros e todos os caminhos antecipados intactos. Não mudar orçamento, replay, snapshots ou APIs externas.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.


## Retenção explícita e armazenamento dos aposentados — 2026-10-16

### Medição anterior à decisão

O inventário clippy-aridade-final-2026-10-16 registra 17 avisos: parent e final_styles nunca lidos e Vec<Box<Execution>> nesta unidade. before.json.gz em 00_nucleo/diagnosticos/clippy-retencao-2026-10-16 conserva HEAD, working tree não commitado, UTC, diffstat, fontes/L0 e hashes globais. O inventário completo de Execution/retired nesta fonte não contém pin, auto-referência, raw pointer ou teste do endereço do registro. input e os valores Func/StyleChain preservam identidades em Arc independentes do endereço de Execution. É medição da mecânica Rust, sem nova prova de paridade ou promessa de ganho de RSS.

### Responsabilidade exclusiva e aceitação

Retirar somente Producer.parent armazenado, sem remover o argumento parent de Session.add: ele continua calculando target pelo produtor pai. Nomear o campo Execution._retained_styles e conservar a movimentação do mesmo styles ao final da execução, na mesma posição e com a mesma duração/drop. A ausência de leitura é intencional: a cadeia retém valores/closures e não é uma entrada de replay. Não eliminar, zerar ou antecipar a liberação da cadeia.

Usar retired: Vec<Execution>; ao retirar o Box de current, mover *previous/*old/*discovery para retired nas três transições. current continua Option<Box<Execution>>. Preservar todos os registros e sua ordem, inputs Arc, ctx/observações, resultados/sinks/Location e duração até o drop da sessão; só retirar a alocação externa de cada aposentado. Nenhum contrato depende da estabilidade do endereço do container Execution, nem usar esse endereço como identidade futura. Não reexecutar closures para reconstruir o histórico ou apagar gerações.

Teste de retenção usa Session.execute real e um Value::Version com Arc/Weak como testemunha de lifetime: após remover a outra cadeia proprietária, o valor deve sobreviver na execução atual, na aposentada e depois do crescimento real do Vec, e ser liberado ao sair da sessão. Confirmar mesma identidade de input e manutenção dos testes de replay/gerações existentes. Contagem de Arc é testemunha mecânica de retenção, não paridade da linguagem. Controles existentes de target/capturas/HTML/bundle/PDF permanecem; não criar teste que apenas reflita a sintaxe do container.

Fluxo contínuo ADR-0127: somente campos privados e representação interna, sem API pública, default ou fase novos. L0 precede testes/código e resselo restrito. RED de dead_code/vec_box antes da correção, GREEN dos três spans selecionados; nenhum allow/expect novo, nem elevar limiar. O wrapper subset_font é decisão separada do seu owner. Validar build, Clippy all-targets/expectativas, suíte workspace completa, Rustdoc, formato e linhagem estrita/reversa, sem novos avisos Tekt, sob 12 GiB, swap zero e um job. Não inferir ganho de memória/tempo desta refatoração sem medição pareada própria.


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

No ramo de runtime candidato não bundle, fornecer self.target/self.features ao
helper privado de introspecção, em vez de defaults Paged/sem features. Manter
bundle existente, seleção, 5 tentativas, ordem de produtores, snapshots e cache
de métricas. Nenhuma lógica de fold ou callback de Heading passa para L3.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.


## Context — identidade de ocorrências ratificada, 2026-10-18

### Medição anterior à decisão e gate

before.json.gz em 00_nucleo/diagnosticos/context-paridade-2026-10-18 conserva
HEAD b7e7c83223e5daa67bbdf124c36b5b7f93200731, fontes, UTC, diffstat, argv e
SHA dos binários; baseline-build-resources.json contém a receita nova. Vanilla
bd5ebec285505889e8ff1d4f68804ab147a699f3. Duas equações dão EQ EQ Q1/Q2;
prefixo Heading H EQ Q0/Q1; it.location observado por query dá 00/01.
context_stabilization.rs:257 reinicia EvalContext por produtor; eval/mod.rs:438
reinicia Locator; realize/mod.rs:41 aloca sem domínio comum; introspect/walk.rs:
375–390 deduplica pela Location. É semântica/identidade de linguagem.
Upstream typst-realize/src/lib.rs:353–418 prepara identidade antes da receita,
mas não exige copiar representação, vtables ou algoritmo.
Hipótese de colisão será refutada por falha de contagem, alias, callback ou replay.

O humano aprovou a proposta review.md sob
00_nucleo/materialization/context-occurrences-2026-10-18, SHA-256
500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388;
recibo approval-receipt.json no mesmo diretório. Gate ADR-0127 §2.1 satisfeito
para a API aditiva especificada. Não autoriza mudança de fase ou API alheia.

### Responsabilidade exclusiva

Session possui arena da fachada L1 e Producer possui frame exclusivo emitido
por ela. Criar arena do origin real; sem produtores pode usar None e evitar
uma caminhada redundante. Cada novo Producer recebe frame novo; reexecução
retém o frame do mesmo Producer. Descartar descendente não recicla seus slots.
Envolver a operação já existente apply_func → realize_math_insertions →
realize_table_insertions em arena.evaluate, sem mudar ordem ou executar mais
vezes. L3 não escolhe nem manipula Locations numéricas. Session legacy nova
reconstrói deterministicamente arena/frame do mesmo origin e ordem causal;
não inicializar a arena do snapshot de resultados anteriores. Retenção,
seleção, erros, budgets, estilos, targets, efeitos e callbacks ficam vigentes.

### Inicialização na primeira inclusão — correção interna medida

`workspace-rest-occurrences-final.log`, recibo validation-occurrences-final e
freeze occurrences-final-sources.json.gz conservam a falha do teste existente
execution_retains_style_values_through_retirement_and_vector_growth. No consumer,
Session::add em 235–242 pressupõe arena; teste em 1485–1506 cria Session vazia e
inclui o primeiro produtor depois. A arena deve ser inicializada em add na primeira
inclusão usando o origin da sessão, via get_or_insert_with; Session::new pode
sempre iniciar None. Zero produtores permanece sem arena. O teste existente não
muda. Correção privada dentro da API aprovada, sem alterar seleção ou fase;
RED registrado, exigir GREEN e revalidação integral.
