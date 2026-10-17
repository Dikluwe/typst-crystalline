# L0 — exibição de counter
Hash do Código: e3ceea2b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:bdbe23b72d7083b7672d71e851dc3d35baa8ed7ccb66f009f61edcf770d0aeff
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/counter/display.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência antes da decisão

`01_core/src/compiler/stdlib/counter.rs:666-845` reúne a política de `.display()` para localização, valores, `both`, numbering e callback. O wrapper nativo em `:304-315` preserva a entrada pública. Medição estrutural da árvore em `320c2149b`; não é comparação de desempenho ou inferência sobre intenção upstream.

## Contrato

Receber o mesmo `Counter`, argumentos, scopes, contexto, engine e span da fachada. Validar localização contextual e named args na ordem vigente; obter histórico filtrado pelo owner irmão; observar CounterAt/CounterTotal nos mesmos pontos; aplicar default numbering, pattern explícito ou callback na mesma ordem e produzir o mesmo Content ou diagnóstico. O wrapper e `counter_display` público permanecem na fachada sem mudança de assinatura. Não alterar defaults, casts ou fase de eval.

## Aceitação

Texto, conteúdo, spans, erros, avisos e ordem de chamadas iguais nos casos de padrão implícito/explicito, callback, `both` e counters filtrados. Build, testes focais e lint válidos.




## Medição da ordem antes da decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at e StateRegistry::value_at comparam Location.as_u128; queries before/after e eventos de contador fazem o mesmo. A realização prepara identidades antes de transformar, portanto o número deixa de ordenar origens e produtos. A sonda CLI e a sentinela single_evaluation_counter_follows_causal_order conservam a evidência no diagnóstico de implementação. Aplicar o núcleo realization-order; manter fallback histórico somente sem ordem registrada.


## Realização com avaliação única

Prefixo de contador filtrado consultado em Location usa ordinal causal do introspector, sem ordenar por identidade bruta.


## Correção da leitura de página lógica — 2026-10-05

Medição anterior à decisão: no HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree registrado em `00_nucleo/diagnosticos/contexto-page-display-2026-10-05/before-state.json.gz`, a sonda `#context counter(page).display()` imprime 0; vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 imprime 1. `01_core/src/compiler/stdlib/counter/display.rs:61–77` consulta somente CounterRegistry; `01_core/src/compiler/stdlib/counter.rs:378–390,463–482` já resolve get/at pela Location física e PageStore lógico. No vanilla, `lab/typst-original/crates/typst-library/src/introspection/counter.rs:271–291,456–472` display_at e at compartilham o estado resolvido. Os recibos do diagnóstico preservam a prova CLI.

Decisão de paridade interna (ADR-0107/0127): a leitura atual em display aplica o helper privado page_counter_value da fachada quando CounterKey::Page possui Location e página lógica realizadas. Sem essa vista, conservar histórico/fallback causal; não inventar página 1. Demais chaves mantêm histórico e ordem causal vigentes. O valor observado CounterAt deve ser o mesmo usado no padrão/callback, coerente com replay get/at. Preservar callbacks, padrões, validação, erros, assinatura pública e fase. A fórmula de CounterTotal/final permanece fora desta correção, conforme scope-out P1485.2 do owner da fachada; não usar o último PDF de um bundle para fabricar totais de outro.

Aceitação: display padrão, padrão explícito e callback usam a página lógica da ocorrência; pagebreak incrementa e update antes do documento é respeitado; get/at continuam coerentes. Counters de strings e headings não recebem página física. Exercitar testes integrados RED→GREEN, testes de counter e contexto existentes, CLI no pin ratificado e build/lint. Não reduzir ciclos ou orçamento de estabilização nesta correção.


## Snapshot realizado de Page — ratificado em 2026-10-05

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree e binários exatos em `00_nucleo/diagnosticos/contexto-page-total-2026-10-05/measurement.json.gz`. single imprime B1/0 F0 contra B1/1 F1; duas páginas total/final 0 contra 2; update inicial 7 conserva total 7 contra 8; update intermediário altera página anterior (7/7 e 8/7 contra 1/7 e 7/7); step final imprime final 1 contra 2. Bundle imprime 1/7 no primeiro PDF contra 1/1, e final 7 no segundo contra 8. HTML misto devolve 1 contra 7. `03_infra/src/pipeline/paged.rs:360–365,840–913` usa o último update como início de todas as páginas; `context_stabilization.rs:969–1010` faz o mesmo por body. `compiler/stdlib/counter.rs:490–513` e `counter/display.rs:96–110` leem final global causal. Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, `lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254,730–773,934–963`, combina escopo documental, updates e avanços físicos. A classificação é paridade de linguagem; não exige copiar a mecânica de memoização upstream.

Decisão proposta: aplicar o Núcleo page-counter-snapshot. Esta unidade substitui o scope-out histórico de final apenas após ratificação. Contrato público e realização page-aware de updates exigem gate ADR-0127; não materializar ou ressellar antes da confirmação. Preservar as assinaturas existentes, ciclos e budget; não adivinhar página/total antes do snapshot conhecido.

Leitores Page usam snapshot realizado por Location quando presente. Current get/at/display consulta page_counter_values, final/both consulta page_counter_final_values da Location contextual pertinente; antes disso, conservar fallback causal existente, sem misturar entrada parcial e total de outra geração. Replay de CounterAt/CounterFinal/CounterTotal segue a mesma resolução, incluindo erros e estado vazio. Outras chaves não são modificadas. A vista lógica antiga do PageStore não corrige snapshots inexistentes por inferência do último PDF.

Ratificação: o humano confirmou o conjunto do manifesto `00_nucleo/diagnosticos/contexto-page-total-2026-10-05/l0-manifest.json` (SHA-256 1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e). O gate ADR-0127 deste conjunto está satisfeito. Clarificações internas subsequentes seguem fluxo contínuo, L0 antes do código e RED→GREEN.
