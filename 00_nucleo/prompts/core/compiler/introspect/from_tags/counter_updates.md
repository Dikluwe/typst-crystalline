# L0 — compiler/introspect/from_tags/counter_updates
Hash do Código: f22a6878

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:bdbe23b72d7083b7672d71e851dc3d35baa8ed7ccb66f009f61edcf770d0aeff
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6


## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/from_tags/counter_updates.rs`. A lógica original estava em `01_core/src/compiler/introspect/from_tags.rs:34-103,157-287` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Own filtered counter replay, ordinary counter functions and bundle Page replay. Preserve causal order, key equality, source position, callback inputs/results, diagnostic priority and event history. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.




## Medição da ordem antes da decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at e StateRegistry::value_at comparam Location.as_u128; queries before/after e eventos de contador fazem o mesmo. A realização prepara identidades antes de transformar, portanto o número deixa de ordenar origens e produtos. A sonda CLI e a sentinela single_evaluation_counter_follows_causal_order conservam a evidência no diagnóstico de implementação. Aplicar o núcleo realization-order; manter fallback histórico somente sem ordem registrada.


## Realização com avaliação única

Replay de contador filtrado respeita ordinal causal registrado, mantendo ordem dos callbacks e prefixos demandados.


## Snapshot realizado de Page — ratificado em 2026-10-05

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree e binários exatos em `lab/results/construction/diagnostics/contexto-page-total-2026-10-05/measurement.json.gz`. single imprime B1/0 F0 contra B1/1 F1; duas páginas total/final 0 contra 2; update inicial 7 conserva total 7 contra 8; update intermediário altera página anterior (7/7 e 8/7 contra 1/7 e 7/7); step final imprime final 1 contra 2. Bundle imprime 1/7 no primeiro PDF contra 1/1, e final 7 no segundo contra 8. HTML misto devolve 1 contra 7. `03_infra/src/pipeline/paged.rs:360–365,840–913` usa o último update como início de todas as páginas; `context_stabilization.rs:969–1010` faz o mesmo por body. `compiler/stdlib/counter.rs:490–513` e `counter/display.rs:96–110` leem final global causal. Vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, `lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254,730–773,934–963`, combina escopo documental, updates e avanços físicos. A classificação é paridade de linguagem; não exige copiar a mecânica de memoização upstream.

Decisão proposta: aplicar o Núcleo page-counter-snapshot. Esta unidade substitui o scope-out histórico de final apenas após ratificação. Contrato público e realização page-aware de updates exigem gate ADR-0127; não materializar ou ressellar antes da confirmação. Preservar as assinaturas existentes, ciclos e budget; não adivinhar página/total antes do snapshot conhecido.

Fachada pública proposta neste owner, acessível através do módulo já existente; qualquer delegação apenas de caminho permanece mecânica:
```rust
pub fn realize_page_counter_snapshot(
    tags: &[Tag],
    documents: &[(Vec<Location>, Option<std::num::NonZeroUsize>)],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    observer: &EvalContext,
) -> SourceResult<()>;
```
Cada tuple contém as ocorrências de um body e o total físico realizado, None para HTML. Não ordenar por Location numérica para inferir documento. Validar partição sem sobreposição e identidade; posições vêm do intr corrente. Dobrar Page com estado inicial da linguagem, Set/Step/Func e avanços físicos; respeitar ocorrências anteriores e total após últimos updates/páginas. Callbacks Page recebem o estado page-aware e seus sinks seguem a retenção da contribuição; não executar novamente callbacks de outras chaves. Antes de posição/partição suficientes, não publicar snapshot incompleto. Publicar as duas vistas juntas somente na mesma geração válida; preservar diagnósticos originais e traces. Caracterizar callbacks, empty/multilevel states, updates tardios e HTML com oráculos upstream antes de fechar o contrato comportamental. Não interpretar ausência de fonte instalada como teste positivo.


## Materialização do conjunto ratificado — 2026-10-05

O log causal canônico intr.counters.actions, já extraído das tags, preserva ações Func originais; tags fornecidas não duplicam eventos. Descritores de bundle usam o pertencimento de ocorrências registrado pelo walk, nunca intervalos entre boundaries ou Location bruta. Descriptor único com ocorrências explícitas cobre standalone. Recriar logical_numbers do PageStore existente com o fold realizado, conservando slots de metadata até a realização de numbering pelo owner; final inclui updates após a última leitura.

### Partição explícita da realização

Quando existem vários documentos, o fold usa o pertencimento registrado pelo walk de bundle no CounterRegistry. Os descritores fornecidos não substituem esse pertencimento por intervalos de checkpoints. Se não houver escopos registrados, as listas fornecidas são escopos explícitos, disjuntos. Escopos vazios são válidos; uma ação Page fora dos escopos impede publicação.

### Normalização de leituras Page

Medição CLI em contexto-page-snapshot-2026-10-05/measurement.json.gz: update(()) devolve get (0,) no vanilla contra () no cristalino; update((7, 2)) devolve get (7,) contra (7, 2). Vanilla pinado, counter.rs:621–629,798–807,869–880: a leitura e final projetam o primeiro nível e preenchem zero quando vazio mesmo com delta físico zero. A sequência de ações só aplica avanço quando delta positivo (counter.rs:939–948), preservando argumentos originais de callbacks entre ações na mesma página. Aplicar essa projeção somente às vistas de leitura/final; não mutar o estado causal por observar.

Ratificação: o humano confirmou o conjunto do manifesto `lab/results/construction/diagnostics/contexto-page-total-2026-10-05/l0-manifest.json` (SHA-256 1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e). O gate ADR-0127 deste conjunto está satisfeito. Clarificações internas subsequentes seguem fluxo contínuo, L0 antes do código e RED→GREEN.
