# L0 — infra/pipeline/paged
Hash do Código: 85777032

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compilation-metric-cache.toml sha256:b4372cbdd36e0e25377d1258638a6859bcc88c9e584b91b377a2a12d0d66135f
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:bdbe23b72d7083b7672d71e851dc3d35baa8ed7ccb66f009f61edcf770d0aeff
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:16ae092d990d3d26f03232afcabf63f3b5c08ffb5b6297989368f0346d14f2ab




**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/paged.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
```

## Current evidence

- `03_infra/src/pipeline/paged.rs:188` — `compile_to_paged_document_full_error_and_features_with_introspector`.
- `03_infra/src/pipeline/paged.rs:686` — `bind_logical_page_numbers`.
- `03_infra/src/pipeline/paged.rs:712` — `realize_page_numberings`.
- `03_infra/src/pipeline/paged.rs:318`, `:390`, `:521` — the ordinary page-numbering cycle performs two grid layouts with a fresh `FallbackFontMetrics` each time. The local GDB probe in `00_nucleo/diagnosticos/grid-cache-real-2026-10-03.md` measured 962 shaping-cache misses for distinct text versus 5 for repeated text; `C000` misses used the same size/font key in separate cache instances. This is evidence of repeated internal work, not a new vanilla language-parity requirement. The proposed cache lifetime is refuted if a fresh cache is necessary for changed World or if text/geometry/diagnostics change.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/paged.rs`.

## Owned contract

- Own paged compilation, runtime introspection, outline realization, callback coordination, PageStore construction and final logical-number binding. Only complete validated candidates may reach shaping and export.
- Forward explicit features unchanged to evaluation; compatibility wrappers use the empty set. Features, target, PDF tagging and stream modes remain independent axes.
- Delegate selective contextual preparation to its child. A selected result already supplies final content, introspector, document and warnings: do not run a second contextual/page-numbering loop. Without selection, preserve the ordinary path and its errors.
- Realize outlines through L1 using each occurrence's lexical snapshot. After realization, discover newly produced context blocks with causal Locations and expand before layout. Remove only markers of already executed causal producers from the discovery view, not sibling results; do not replay old producers or identify them solely by numeric id.
- Coordinate independent math and `par.line` stores outside L1 layout. Pending requests are realized with their captured style/span and Engine, then replayed under the existing finite eight-attempt budget. Math angle requests receive the default Angle and must return Angle; line numbering receives exactly one integer and uses canonical `value_to_content`.
- New transcripts or partially consumed stores invalidate the candidate. Errors, invalid casts or exhausted callback budgets prevent export. Rebuild stores for every external relayout; never cache them across compilations or merge their budget with introspection fixpoints.
- Within one ordinary paged compilation and the same World, reuse the `FallbackFontMetrics` cache allocation across page-numbering relayout candidates by cloning its cache-sharing handle. Keep each candidate, callback transcript/store, and introspector fresh as before. Do not retain metric caches across compilations or change the selected context-stabilization path. Cache reuse must preserve language-visible text, geometry and diagnostics; it is an internal optimization, not a semantic equality rule for `Content`.
- The final line-position reconciliation of that ordinary compilation may borrow the same metrics handle after the numbering loop. The GDB cache probe in `00_nucleo/diagnosticos/grid-cache-real-2026-10-03.md` observed a separate third cache instance for the same `C000` width key in reconciliation (`03_infra/src/shaper/reconciliation.rs:234`). This predicts redundant internal shaping; the hypothesis is refuted by changed output or no decrease in misses/time. Retain the existing fresh-metrics reconciliation for a selected contextual result and for other callers. No cache survives the compilation.
- The pre-shaping bidi pass currently receives fresh metrics at `03_infra/src/pipeline/paged.rs:477`, while `03_infra/src/layout_bidi.rs:128-173` asks `FontMetrics::text_width` on RTL words. In the same ordinary compilation and World, it may borrow the layout metrics handle as reconciliation does, avoiding widths already measured in layout. A selected contextual result still receives fresh metrics. This is an internal cache-lifetime experiment: preserve bidi order, text, geometry and diagnostics, and retain the change only if a real RTL A/B shows a repeatable benefit.
- Realize visible page numbering with `[current,total]`; realize the unary reference view `[current]` only for actual resolved page-reference targets. Preserve raw numbering objects separately from realized content and propagate callback diagnostics.
- The page-numbering cycle is bounded to five passes and checks page counts, logical snapshots, raw objects and realized content, not page count alone. Reconstruct from original input; never accumulate markers, callbacks or warnings.
- Preserve zero-size causal context markers for layout position sealing. Convert layout-warning strings to detached `SourceDiagnostic::warning` in L3 and retain them through errors.
- Associate L1-collected format options with each complete candidate. Return its realized PageStore, not a pre-layout clone; inject final positions after shaping. Bind every page's logical number from the aligned final store, failing on mismatch.
- Keep semantic wrappers and all page layers intact. Resource collection and shaping must include background, items and foreground. HTML does not enter this global paged path.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Reaproveitamento do índice realizado — 2026-10-05

A inspeção em `03_infra/src/pipeline/paged.rs:242` revelou reconstrução do índice inicial já produzido pela fachada de realização. Consultar Module::realization_introspector, definido pelo owner `00_nucleo/prompts/entities/module.md`, e clonar seu snapshot quando disponível; caso ausente, executar o caminho vigente de introspecção de intr_content. Manter conversão bibliográfica do conteúdo visual e injeção de bibliography_styles na cópia inicial. Os loops contextuais, pós-processadores runtime, posições e páginas posteriores continuam próprios deste consumer; não substituir seus índices por esse snapshot. Validar contadores, labels, originais/gerados, colunas, contexto e geometria.


## Reuso sem transformação — 2026-10-05

A inspeção em `03_infra/src/pipeline/paged.rs:90–109` revela introspecção de prefixos mesmo quando realize_outlines não transforma o Content. O contrato atualizado do owner `00_nucleo/prompts/compiler/eval/rules/outline.md` conserva o carrier quando não há aplicação. Nesse caso, o coordenador pode reutilizar o introspector recebido e encerrar a mesma fase, sem reindexar prefixos. Comprovar continuidade por Arc::ptr_eq entre variantes compatíveis de root imutável (Sequence, Styled, Located, Realized); para roots sem prova, conservar caminho anterior. Não usar Hash/PartialEq de Content como prova, não criar identidade de linguagem por endereço e não copiar regras de outline em L3. Novos prefixos, erros, callbacks e contextos de outlines efetivamente realizados continuam nos fluxos vigentes. Correção interna de trabalho redundante, sem nova API, modo/default de produto ou transferência de fase (ADR-0127).

O helper de outlines pode ter visibilidade pub(super) para a suíte descendente do pipeline; não integra a API pública L3.


## Reuso do candidato inicial independente de páginas — 2026-10-05

Evidência anterior à decisão: `03_infra/src/pipeline/paged.rs:364–453` constrói PageStore e repete runtime/expansão/realização/layout mesmo sem dependência de páginas. O diagnóstico `00_nucleo/diagnosticos/grid-pagination-cost-2026-10-05.md` mediu duas chamadas para tabelas simples e cerca de 15 ms de CPU na segunda chamada das 320 células no snapshot histórico indicado, sem alegar ganho causal. O baseline atual é preservado em `00_nucleo/diagnosticos/layout-inicial-2026-10-05/before-state.json.gz`; fonte e binário estão identificados.

O coordenador pode reutilizar o primeiro candidato completo dentro da mesma fase quando uma prova estrutural positiva cobre tanto o conteúdo contextual original como o conteúdo final realizado e todas as páginas finais têm numbering ausente. Nunca decidir por igualdade visual, uma página só ou leitura do texto-fonte. Preservar a construção de PageStore, warnings e injeção de posições/páginas antes de encerrar o ciclo; binding final, format options, bidi, shaping e reconciliação seguem vigentes. Não interferir com prepared.settled ou bundles.

Prova deliberadamente estreita, por travessia emprestada sem transformação: aceitar folhas Empty/Text/Space/SpaceFromNewline/Parbreak/Raw/HSpace/VSpace/Pagebreak/Colbreak/Linebreak, Sequence/Par/Strong/Emph, Styled com custom apenas text.font/text.size/par.linebreaks, page.width/page.height/page.margin-left/page.margin-right/page.margin-top/page.margin-bottom e eval.flow.scope. Geometria usa escalares, eval.flow.scope usa Bool, text.font pode usar Array de Str além de Str, Located e Realized cobrindo original e visual. Table/Grid e suas células cobrem todos os corpos, caption, header/footer; demais campos desses carriers são dados já materializados. SetPage aceita geometria e estilos físicos, mas rejeita numbering presente, supplement explícito, marginal de conteúdo e camadas visuais. Campos explicitamente none/auto que não contêm corpo não introduzem dependência. Qualquer variante não listada invalida a prova; isso inclui ContextBlock, context markers, Ref, Outline/Entry, heading, matemática/callbacks, counters/state, bibliografia, Dynamic, PageRun e ShowRecipe residual. Não ampliar esse conjunto por inércia.

Manter o ciclo completo para qualquer candidato sem prova. Este é reuso interno de um resultado já produzido; nenhuma lógica migra de fase, nenhuma assinatura pública ou semântica/default de produto muda. Fluxo contínuo ADR-0127. Refutar a hipótese por alteração de geometria, diagnósticos, posições, numeração ou ausência de benefício medido. Testes devem provar uma chamada para tabela simples multipágina e duas chamadas no caminho ordinário com numbering; contextos e outlines conservam testes existentes. Sonda cfg(test) pode contar chamadas do wrapper, sem instrumentação em produção.

Refinamento medido: a sentinela multipágina revelou em Styled os customs page.margin-*/width/height e eval.flow.scope, registrados em eval/rules/set/page.rs e eval/markup_stream.rs:218. `eval/rules/set/text.rs:269` normaliza text.font para Array. Esses carriers materializam geometria física e escopo de fluxo sem consulta ao PageStore; admitir apenas as chaves explicitadas e Array de famílias literais Str. Fontes em dicionário/regex e demais chaves conservam fallback. A leitura dessa representação precede a ampliação da prova.


## Snapshot realizado de Page — ratificado em 2026-10-05

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree e binários exatos em `00_nucleo/diagnosticos/contexto-page-total-2026-10-05/measurement.json.gz`. single imprime B1/0 F0 contra B1/1 F1; duas páginas total/final 0 contra 2; update inicial 7 conserva total 7 contra 8; update intermediário altera página anterior (7/7 e 8/7 contra 1/7 e 7/7); step final imprime final 1 contra 2. Bundle imprime 1/7 no primeiro PDF contra 1/1, e final 7 no segundo contra 8. HTML misto devolve 1 contra 7. `03_infra/src/pipeline/paged.rs:360–365,840–913` usa o último update como início de todas as páginas; `context_stabilization.rs:969–1010` faz o mesmo por body. `compiler/stdlib/counter.rs:490–513` e `counter/display.rs:96–110` leem final global causal. Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, `lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254,730–773,934–963`, combina escopo documental, updates e avanços físicos. A classificação é paridade de linguagem; não exige copiar a mecânica de memoização upstream.

Decisão proposta: aplicar o Núcleo page-counter-snapshot. Esta unidade substitui o scope-out histórico de final apenas após ratificação. Contrato público e realização page-aware de updates exigem gate ADR-0127; não materializar ou ressellar antes da confirmação. Preservar as assinaturas existentes, ciclos e budget; não adivinhar página/total antes do snapshot conhecido.

Fornecer ao fold puro as ocorrências por body e páginas/posições da contribuição candidata. Preparar snapshot Page depois de conhecer posição física; reconstruir e validar sob o mesmo orçamento. Não derivar início de todas as páginas do último update. Não reexecutar fonte inteira, não carregar caches de outro documento e não deslocar I/O para L1. Numbering/PageStore e margens consomem a numeração realizada pertinente, sem contaminar o snapshot global com projeção local de exportação. Erros e warnings respeitam retenção dos produtores. Implementação de outros owners exige L0 próprio atualizado antes de código; este owner não legitima render ou handlers de counter.


## Materialização do conjunto ratificado — 2026-10-05

No caminho legado paginado, realizar snapshot Page depois de posições candidatas; repetir contexto até estabilizar ambas as vistas e posições pertinentes, sob as cinco tentativas existentes. Reintrospecção não pode apagar o snapshot antes de nova geração. Numbering recebe sequência lógica realizada e final do próprio documento, não logical_start + índice baseado no último update.

### Convergência sem leituras contextuais

A igualdade das novas vistas Page é necessária à convergência quando existem blocos contextuais. Um documento sem blocos contextuais mantém o critério anterior de contagem de páginas; a publicação da vista não exige uma passagem adicional sem consumidor.

Ratificação: o humano confirmou o conjunto do manifesto `00_nucleo/diagnosticos/contexto-page-total-2026-10-05/l0-manifest.json` (SHA-256 1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e). O gate ADR-0127 deste conjunto está satisfeito. Clarificações internas subsequentes seguem fluxo contínuo, L0 antes do código e RED→GREEN.

## Métricas compartilhadas na sessão contextual — 2026-10-05

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado preservado em `00_nucleo/diagnosticos/contexto-cache-fontes-2026-10-05/before-state.json.gz`, binário baseline 5bc45f4ec0972c6c46450c0cab3479e2ae2ef41c286c424201c1b7fc07823eb1. Sonda GDB `count-baseline.json.gz`: selected_distinct fez 2243 consultas, 965 cálculos de shaping após miss e duas entradas no wrapper; selected_repeated teve 8 misses; controle ordinário 321 misses. `context_stabilization.rs:548` chama o wrapper sem métricas; `paged.rs:656–661` cria FallbackFontMetrics quando o handle é ausente. `font_metrics.rs:71–96` cria caches Arc por instância e Clone compartilha-os, conforme seu L0 proprietário. Há assimetria com o caminho ordinário já compartilhado. O relatório `grid-cache-share-ab-2026-10-03.md:17–23` delimitou explicitamente o compartilhamento ao caminho comum, mantendo os wrappers contextuais e de bundle com métricas novas. Esta etapa amplia aquele escopo depois da medição; não classificar esse limite documentado como decisão por inércia. A origem mais antiga do wrapper fresco permanece não auditada.

Decisão interna, fluxo contínuo ADR-0127: a sessão contextual paginada selecionada pode fornecer o seu handle metrics já existente ao wrapper de layout. Cada tentativa conserva conteúdo, introspector, regiões, callbacks, transcripts, sinks e budget próprios. Somente resultados de métricas por texto/tamanho/estilo/fontes no mesmo World são compartilhados. O handle morre com a sessão; nenhuma retenção entre compilação/edições, nenhum cache de Content, frames ou execução contextual. Bundles/HTML, reconciliação final selecionada e demais callers preservam seus recursos atuais. Não copiar algoritmo/chaves de fonte em pipeline. Não alterar contrato público, modo ou fase. Manter a mudança somente com redução demonstrada de trabalho, medições A/B e controles de linguagem/geometria/diagnósticos sem regressão.

O helper existente layout_with_realized_math_callbacks_with_metrics pode ter visibilidade pub(super), restrita ao módulo pipeline e descendentes, para a sessão contextual selecionada. Os outros wrappers preservam assinatura e default de métricas frescas. A cláusula histórica de cache exclusivo do caminho ordinário fica ampliada apenas para o handle por sessão descrito acima. Uma sonda cfg(test) conta alocações de métricas frescas no ramo shared_metrics ausente; não instrumentar produção.

## Remoção de wrapper privado sem consumidor — 2026-10-05

Depois de a sessão usar o helper com métricas explícitas, `rg -n "layout_with_realized_math_callbacks\(" 03_infra -g "*.rs"` encontra apenas a definição em paged.rs:592. O import privado em pipeline.rs:90 fica sem uso. Remover essa fachada privada sem consumer e seu import; preservar os helpers com métricas explícitas e com seed usado por bundles. Não alterar reexports públicos, assinaturas públicas ou defaults dos callers remanescentes. Correção interna e reversível no mesmo domínio, fluxo contínuo ADR-0127.

## Transporte interno de métricas até a reconciliação contextual — 2026-10-05

Medição anterior à decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado e binário baseline 17201a15e36b06bb3a9ac8adba0a98c61b3dc925c4ce1f118a39d26bce26cf05, preservados em `00_nucleo/diagnosticos/contexto-cache-reconciliacao-2026-10-05/before-state.json.gz`. A sonda count-baseline.json.gz isolou a entrada em fix_line_positions_with_metrics: selected_distinct fez 321 consultas/321 cálculos nessa etapa (643 cálculos totais), enquanto ordinary_control fez 320 consultas/zero cálculos. `context_stabilization.rs:14–17,660–663` entrega Content, introspector e documento, descartando o handle da Session; `paged.rs:337–345,563–572` mantém None no caminho selecionado e chama reconciliação com métricas frescas. O owner `infra/shaper/reconciliation.md` já permite helper interno emprestando métricas do mesmo World/compilação; equações e ABI pública não mudam.

Decisão interna, fluxo contínuo ADR-0127: transferir um clone do handle existente juntamente com o candidato completo aceito da sessão paginada. Prepared e prepare são internos ao módulo pipeline; seu lifetime explicita os empréstimos existentes de World/Source. Nenhuma API pública ou campo de entidade L1 muda. Preservar o contrato de aceitação, erros, contribuição, sinks e budget (inclusive o resultado final aceito sob não convergência provada); somente o braço que já produz settled transporta métricas. Resultados sem settled, HTML e bundles não transportam handle.

O coordenador paginado usa o handle selecionado somente em fix_line_positions_with_metrics. Bidi selecionado continua com métricas frescas nesta etapa, isolando a contribuição causal da reconciliação; o caminho ordinário mantém seu handle nos dois consumidores. Nenhuma execução/layout adicional, nenhum cache de frames ou snapshots, nenhuma retenção entre compilações. Manter invariantes do Núcleo compilation-metric-cache, com lifetime encerrado pelo coordenador paginado. Esta seção amplia o limite anterior de descarte no fim da Session apenas para esse transporte. Refutar a mudança por output/posição/diagnóstico diferente ou ausência de redução comprovada de trabalho/tempo.

## Métricas retidas no bidi contextual selecionado — 2026-10-05

Medição anterior à decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree e binário c35dc34f0f24466b0b7339fcb856d5fc59c6f8701308f54963a4a1704f7746b8 preservados em `00_nucleo/diagnosticos/contexto-cache-bidi-2026-10-05/before-state.json.gz`. A sonda count-initial.json.gz delimita bidi entre reorder_bidi_document e shape_document: selected_rtl_distinct realizou 2057 consultas e 299 cálculos; ordinary_rtl, 2057 consultas e 137 cálculos. Em `paged.rs:565–568`, o caminho selecionado cria métricas frescas embora seu handle esteja disponível; `layout_bidi.rs:23–29,128–173` já recebe FontMetrics e mede palavras. A inferência é que parte dessas larguras existe no cache de layout; ausência de redução de cálculos ou alteração de output refuta a hipótese. Cache é mecânica, não requisito da língua (ADR-0107).

Decisão interna, fluxo contínuo ADR-0127: estender o empréstimo do handle selecionado à chamada existente de bidi, antes do shaping. Esta seção substitui a restrição temporária da etapa anterior que isolou reconciliação. Não modificar algoritmo de bidi, assinaturas, fases, regiões, callbacks, conteúdo ou introspecção. Respeitar compilation-metric-cache: mesmo World, mesma compilação; sem retenção entre edições. Sentinela cfg(test) deve falhar no braço fresco selecionado antes da mudança; A/B deve comprovar trabalho evitado e avaliar tempo sem atribuir todas as diferenças da CLI ao bidi. Preservar texto, posições, raster e diagnósticos; não exigir igualdade de bytes PDF.

## Contexto integral bidi ratificado — 2026-10-05

Medição anterior à decisão: o lote bilateral e a refutação Unicode estão em `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/{initial,probe}.json.gz`, com fontes/estado/binários. `layout_bidi.rs:73–82,227–240` redetecta base e inventa separadores; `shaper/bidi.rs:34` redetecta o fragmento; isto perde a entrada integral ratificada pelo humano. Correção interna de paridade, sem nova fase/assinatura pública externa.

Para spans certificados, usar registry proprietário local de strings auxiliares e mapas por input Arc, depois views BidiInfo emprestadas normais; não usar unsafe/self-reference. Embeddings virtuais LRE/RLE/PDF só quando dir do segmento difere da base; espaços reais e Linebreak não justificado LF (justify=true fora do certificado). Não retirar controles autorais. Resolver uma vez por input em cada registry; o pipeline paginado compartilha registry/views entre reorder e shape. Wrappers públicos existentes podem construir registry local.

Reorder consulta intervalo real da linha, intersects exatas por segment/range e níveis com regras de fim de linha; não midpoint nem espaços inventados. Text misto divide por fronteiras originais, preserva estilo e subspan. Itens não textuais e fontes desconhecidas não ganham certificado. Reorder conserva âncora física aceita por L1, largura/avanço reais; não aplicar fallback de suffix/coalescing ao contexto certificado.

Shape consulta os runs do input completo para span original e não auto-detecta direção de fragmento; manter offsets de subruns por travessia sequencial, emitir subspan exato para cada TextShaped. Chave de cache existente já discrimina direção; não pôr contexto em TextStyle. Reconciliation usa base do input quando existe em vez de qualquer run RTL. Fallback None anterior permanece. Testes RED→GREEN da refutação de 456 789, overrides/neutral, extração sem controles auxiliares e medição bilateral.

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

Coletas/substituições/reconciliação existentes veem somente contribuição visual preparada; conservar plano e preservar fonte causal sem registrar origem ocultada. Não criar Engine Table no layout nem passagem extra de fonte; callback/math/page orçamentos existentes ficam.

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

workspace-seventh/independent_table refuta Source como unknown impuro: prova positiva de independência paginada delega somente ao body; source span não é geometria. Body desconhecido continua fail-closed, sem relaxar comparação observacional ou tornar efeitos independentes.

### Receita local de Cell sem dependência de páginas — 2026-10-09

Medição antes da decisão: `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/layout-call-counts.json.gz` conserva release SHA-256 `4d63c08cb3a01aed112ac4612a6e6413894284ff579626c4c277b818e1ba18be`, HEAD/árvore não commitada/status/diffstat/UTC/fontes/comandos e script GDB. Em 1.600 células, `local-set` executa duas resoluções completas de colunas/linhas e 3.200 subframes; `no-fill` executa uma e 1.600. A mesma relação vale em 100 células. `grid-invocation-traces-deep.json.gz` mostra ambas as resoluções locais sob o coordenador paginado comum. `paged.rs:30-44` rejeita toda chave custom de Cell e `eval.table.local.recipe`; `paged.rs:434-468` então reexecuta expansão, realização e layout após construir PageStore. A inferência de que esta recusa explica o excesso local será refutada se a classificação ampliada não reduzir o número de layouts ou o tempo pareado. A receita captura corpo, regras, cadeia de estilos e guards em `entities/content.rs:202-211`; não inferir independência apenas da cor visível.

Decisão interna de mecânica, fluxo contínuo ADR-0127: admitir na prova `Styled` apenas `cell.fill` com valor escalar físico (`Color` ou `None`), `eval.table.local` igual a Bool(true) e `eval.table.local.recipe` contendo ShowRecipe de corpo que também passa na prova, sem regras locais nem guards, com estilos capturados cujo delta colapsado contém somente propriedades já certificadas pela prova escalar e `cell.fill` físico. O wrapper ainda precisa ter body independente. A receita permanece dado lexical e não passa a ser layoutável diretamente. Qualquer outra propriedade `cell.*`, outra receita, regra local, guarda, função, contexto, marker, contador, numbering ou chave desconhecida conserva o ciclo completo. A prova não usa igualdade visual/Rust para decidir. PageStore, warnings, posições e binding finais permanecem calculados como antes; apenas reutilizar o candidato completo já produzido quando a prova positiva passa em conteúdo contextual e realizado, sem mover fases, API ou default.

Aceitação: teste RED→GREEN de uma chamada do wrapper paginado para tabela local multipágina sem numbering; controle com numbering e receita com show/contexto ainda usa a segunda chamada. GDB confirma 3.200→1.600 subframes em 1.600 células, comparações bilaterais de texto/páginas/caixas/raster das 23 fixtures e benchmark pareado contra vanilla. Reverter se o excesso local não cair. Suites, build, lint e resselo apenas deste owner após validação.

Manter owner exclusivo e despacho estático; sem I/O, assinatura/trait/flag nova ou fase eval↔layout. Testes e coletas C2/C/macro, builds e lineage necessários antes de certificar.

## Descoberta emprestada de blocos após outline — 2026-10-09

Medição anterior à decisão: `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/stacks-deep.json.gz`
guarda binário, HEAD/working tree, fontes, interpositor, comandos e UTC; uma
em cada 64 alocações é amostrada. Sob `expand_realized_outline_contexts`,
`TableElem::map_content` aparece em 902 amostras de `local-set-1600` e 300
de `block-no-set-1600`. A fonte `03_infra/src/pipeline/paged.rs:185-204`
reconstrói duas vezes a árvore somente para comparar identidade dos
`ContextBlock`; se não houver produtor novo, descarta ambas as cópias.
`outline-context-red-gate.json.gz` registra 957.331 pedidos no binário
anterior, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree
não commitado, comando e UTC. Inferência refutável: uma leitura por referência
deve poupar cópias sem mudar o conjunto ordenado de produtores ou a condição
de reexpansão.

Substituir apenas as duas descobertas por visitante privado e read-only.
Preservar a projeção de `map_content`: Source transparente, Located suprime
o callback no body raiz mas visita seus descendentes, Realized visita apenas
visual, ShowRecipe/Styled visitam body, Sequence na ordem, Table resolvida
visita cells e caption ou, quando crua, children/header/footer/caption;
TableCell visita visual standalone/resolved ou body. ContextBlock terminal
é emitido somente quando seu próprio callback seria visitado. Variantes
restantes mantêm fallback via `map_content`, sem interpretar subárvore
desconhecida como vazia. A comparação conserva identidade `Arc::ptr_eq`,
ordem e regra de substituir apenas produtores antigos quando há novos.
Nenhuma API pública, modo, fase ou identidade causal muda. Aceitação:
comparar sequência de IDs e identidades com o visitante anterior em árvore
aninhada, saída PDF/raster/linguagem igual, gate de alocações e medição
pareada de tempo. Fluxo contínuo ADR-0127.

## Referências de página sem destinos — otimização interna 2026-10-09

Medição anterior à decisão: `00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/page-ref-empty-red-gate.json.gz`, coletado em working tree não commitado no HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, conserva UTC, estado/diffstat, comando, fontes, fixtures, interpositor e release SHA-256 `14d023d36fb765eb041f24f440eaad3bc5d67183325b611e4fd98502bd26815e`. O gate predefinido de até 539.000 pedidos glibc para `local-set-1600` falhou com 541.535; `no-fill-1600` fez 410.264. O perfil `cpu-style-current.json.gz` contém 62 e 36 amostras, respectivamente, sob `page_reference_pages`; `03_infra/src/pipeline/paged.rs:987-1004` percorre e reconstrói Content para consultar destinos, mas `extracted_label_pages` vazio torna toda consulta negativa. Atribuição de custo exclusivo ainda é inferência, refutável por gate GREEN sem redução e por tempo pareado sem ganho.

Quando `doc.extracted_label_pages` estiver vazio, `page_reference_pages` devolve o conjunto vazio antes da travessia. Com destinos presentes, conservar a travessia, os forms Page, resolução por Label e o conjunto de páginas sem alteração. A função continua privada de L3; nenhum contrato público, fase, default ou semântica de referência muda. Exigir gate de alocações RED→GREEN, teste de referência Page com/sem destino e forma Normal, equivalência de saídas e tempo pareado; retirar a otimização se piorar o tempo de forma repetida.


## Auxiliares de teste e bindings produtivos — 2026-10-14

### Medição anterior à decisão

`03_infra/src/pipeline/paged.rs` nas linhas [340] apresenta os avisos selecionados.
`00_nucleo/diagnosticos/ci-helpers-testes-2026-10-14/before.json.gz`
conserva fonte/owner, HEAD, working tree não commitado, UTC e diffstat.
references-before.json.gz registra chamadas, imports, módulos e referências;
o estado da suíte/inventário anterior foi verificado por hashes. A lista de
testes foi capturada antes da edição. Não se infere inutilidade funcional
apenas de dead_code: as chamadas privadas e sua compilação foram auditadas.

### Responsabilidade e aceitação

compile_to_paged_document_full_error é fachada pub(super) exclusivamente importada sob cfg(test) pelo ancestral e usada em pipeline/tests. Aplicar cfg(test) também à definição, preservando a assinatura para a suíte, Features::default e delegação. Os helpers and_features/with_introspector continuam produtivos e recebem as features/contextos vigentes. Corrigir a descrição do adaptador para não afirmar uso pelos backends que chamam o caminho parametrizado.

Manutenção interna em fluxo contínuo ADR-0127: nenhuma API pública, campo,
default de produto, fase, dependência ou supressão de lint muda. O gate de
diagnósticos selecionados deve falhar antes e passar depois; ele não é um
novo teste funcional nem prova de paridade. Preservar a lista e as expectativas
dos testes existentes, executar a suíte completa, compilar lib/all-targets,
verificar build/rustdoc/fmt/linhagem e registrar o inventário total. Não criar
testes que apenas espelhem cfg ou remoção de bindings. Resselo restrito ao owner.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

03_infra/src/pipeline/paged.rs:373 (C097).
O snapshot before.json.gz em 00_nucleo/diagnosticos/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

O sucesso interno de compile_to_paged_document_full_error_and_features_with_introspector usa PagedCompilation privado à família pipeline, com document e introspector nomeados. O helper continua retornando SourceResult separado de warnings, com os mesmos resultados/erros/timings. A fachada pública que retorna só documento projeta .document; os demais chamadores projetam .introspector. Não alterar públicos, fases ou aquisição de recursos.

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


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/pipeline/paged.rs:807 (clippy::map_clone). O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Trocar shared_metrics.map(Clone::clone) por shared_metrics.cloned(), mantendo unwrap_or_else preguiçoso e o contador de métricas sem cache no mesmo ramo. Mesma instância compartilhada, ciclo, World, callback budget e comportamento de ausência.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## RG1 — refinamento compartilhado de fase — candidato 2026-10-17

### Medição anterior à decisão

Medição anterior: 00_nucleo/diagnosticos/table-row-groups-2026-10-17/before.json.gz, measurement.json.gz, supplement.json.gz, lines.json.gz, styles-phase.json.gz e svg.json.gz conservam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC/status/diffstat/fontes/binários/argv/cwd. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. table_grid.rs:126–150 funde grupos; row_groups.rs:22–35/70/355–366 conta entradas, table_cells.rs:1204 aplica receita antes do planner e :1259–1282 deriva prefixos. model/table.rs:307–316 sintetiza; typst-realize/src/lib.rs:367–385/558–568 sintetiza antes de receita; layout/grid/resolve.rs:1380–1457/1597–1650 preserva intervalos e cursores; typst-html/src/rules.rs:591–665 projeta papéis. TableHeader level/presença somem no Tekt, coordenadas de linha não são lidas em table_lines.rs:173–260, e supressão None em layout/grid/rules.rs:40–46 deixa traços. svg-analysis.json confere todos os strokes retos e mudanças pareadas (mesmo binário): top vermelho vanilla 82→41, Tekt 82→82; azul após None vanilla 41→0, Tekt 40→40. Estes são efeitos/células/roles/erros/geometria da língua; não obrigação de copiar vtables, estruturas Rust, bytes SVG/PDF ou inferir intenção upstream. O primeiro extrator SVG não reconhecia line/h/v; foi descartado como prova, conservado e substituído por extração completa registrada.

### Responsabilidade exclusiva

Atualizar somente o pin do núcleo compartilhado. A distinção entre validação estrutural da Table e receita Cell/body substitui a leitura histórica de “Table antes de tudo”; este consumer conserva sua responsabilidade e código, sem novas operações locais, contratos próprios ou alargamento de escopo. Visual completo continua sendo a única vista publicada.

### Gate

Contrato candidato RG1: não vigente antes da confirmação humana do manifesto em 00_nucleo/materialization/table-row-groups-2026-10-17/review.md. A aprovação precisa abranger contratos públicos, compatibilidade e fase (ADR-0127 §2.1/§2.3/§2.4). Após aprovação, esta seção supersede apenas cláusulas históricas conflitantes de grupos Table/linhas e da fronteira estrutural/receitas. Outras propostas não são ratificadas. Não implementar campo/API novo antes do gate. Instalar L0/kernel/pins primeiro, testes funcionais RED antes do código, sem tratar erro de compilação/setup como RED; GREEN mais R1–R20 e corpus bilateral, build, workspace completo, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, formato, V1/V5/V15/V26 e reversa, diferencial SARIF/replay/diff-check. V16 externos continuam abertos; não alterar severity, criar allow/expect ou esconder um complemento em if-let só para zerar lint. V19 tem justificativa local e corpo compartilhado quando a política é a mesma. Núcleos são compartilhamento normativo; cada L0 conserva exatamente um consumer. Sob MemoryMax=12 GiB, swap zero, Cargo um job, comandos pesados sequenciais; sondas/mutações usam target Cargo isolado e nenhuma variante mutante vai para target principal. Sem commit/push/merge/release ou pin novo.
