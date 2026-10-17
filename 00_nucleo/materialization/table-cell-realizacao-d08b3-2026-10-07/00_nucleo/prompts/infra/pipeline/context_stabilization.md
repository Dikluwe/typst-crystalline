# L0 — infra/pipeline/context_stabilization
Hash do Código: 81e10588

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
