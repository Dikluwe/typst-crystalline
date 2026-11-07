# Candidatos de investigação de desempenho no código atual

**Resultado:** varredura estática de **1011 arquivos Rust** nas quatro camadas, seguida de leitura dirigida de tipos, callers, limites de cache, cfgs, L0 e resultados anteriores. Catálogo de **28 testes possíveis**, com âncora de fonte, hipótese, critério, esforço relativo e gate. Isso não equivale à revisão semântica manual de cada função nem a 28 gargalos ou defeitos confirmados. Nenhum ganho novo foi medido; hotspots e percentuais permanecem **Unknown** onde falta perfil.

Nenhum código produtivo, teste, L0, dependência ou experimento anterior foi alterado. Não houve build ou execução de testes nesta auditoria: trata-se de localizar e delimitar os próximos experimentos. Este diagnóstico não é L0 nem autoriza materialização.

## Cobertura e método

|Camada|Arquivos .rs lidos pelo scan|Linhas, incluindo testes/comentários|
|---|---:|---:|
|01_core|821|393155|
|02_shell|9|5431|
|03_infra|156|96639|
|04_wiring|25|12985|
|Total|1011|508210|

Inventário inclui `src`, testes, benches/build.rs quando presentes; a contagem de linhas não é LOC produtivo executado. Foram percorridos todos os .rs sob 01_core/02_shell/03_infra/04_wiring, buscando clones/coleções, alocação/reserva/reúso, Arc/dyn/locks, cache/memo, arenas/índices, acesso ao filesystem e serialização. Busca lexical é triagem: comentários, cfgs e testes podem aparecer. Os candidatos abaixo foram confrontados com o código e owners relevantes; ausência de match não prova ausência de custo, nem o número de matches mede tempo. Não foram analisados assembly, contadores de hardware ou bytes vivos do alocador.

Cobertura funcional: parser/CST e checkpoints; Value/Content/Scope/StyleChain; eval/imports/coleções/plugins; realização e pipeline paginado; convergência/checkpoints/parágrafos/tabelas; métricas/fontes/shaping/bidi; PDF/font binding/subset/imagens/raster; HTML/SVG; CLI/query/diagnósticos; composição/watch/SourceSession. Código original em lab e dependências não entram no denominador; relatórios históricos foram consultados como evidência qualificada, não como desempenho atual. Macros/dependências/configurações não executadas permanecem limites da auditoria.

## Ordem recomendada

P0 significa testar correção ou atribuir um custo já observado antes de otimizar; P1 é candidato pequeno e próximo de rotas reais para medir; P2 depende de perfil mais específico/contrato mais amplo; P3 é condicional, de alto alcance ou sem caller alvo demonstrado. Prioridade não prevê magnitude de ganho. Esforço é relativo ao desenho/execução do experimento e não prazo de implementação.

|Candidato|Prioridade|Foco|Esforço relativo|
|---|---|---|---|
|C01|P0|Vida útil e identidade do cache de imagens|baixo para o teste isolado; médio no watch|
|C25|P0|Namespace dos caches de plugins entre hosts|baixo para controle; médio para integração|
|C02|P0|Atribuição das consultas de frescor|médio|
|C03|P1|Construção da chave de shaping|baixo/médio|
|C04|P1|Alocação em hits de métricas|baixo/médio|
|C07|P1|Buffer de saída do walker de shaping|baixo/médio|
|C08|P1|Cópias entre tentativas de convergência|médio|
|C13|P1|Construção da biblioteca em imports|médio|
|C19|P1|Serialização e capacidade de saída PDF|baixo/médio|
|C20|P1|Escaping e formatação HTML/SVG|baixo|
|C10|P1|Scratch e bounds checks do seletor produtivo|médio|
|C16|P1|Tamanho e frequência dos carriers|baixo para medição; variável para mudança|

**Primeira rodada recomendada:** dois controles de identidade/lifetime (C01 imagens e C25 plugins) e uma bancada curta de atribuição das fontes (C02). São escopos distintos: uma falha em cache não explica automaticamente a latência das fontes. Em seguida, escolher **uma** frente de RAM com alocações medidas, começando por chave de shaping/métricas (C03/C04) ou saída do walker (C07). Não iniciar 28 implementações ou uma reescrita de Content.

Para outline, priorizar a medição de cópias por tentativa (C08); para muitos imports, construção da biblioteca (C13) antes de cachear todo Module (C14); para exportação longa, formatação/serialização (C19/C20). Um documento pequeno dominado por fontes não é bancada discriminante de arena de layout ou bounds checks.

## Correspondência com as oito ideias propostas

|Ideia|Candidatos|Condição para testar|
|---|---|---|
|Índices u32|C17; medir carriers C16|Começar por armazenamento local congelado, com owner e geração; não migrar AST/Content globalmente.|
|Arena por fase|C11, C10, C05|Demonstrar pressão do alocador e lifetimes comuns; primeiro reaproveitar buffers. Verificar Drop e resultados retidos.|
|Variantes grandes indiretas|C16, C15, C06|Medir tamanho×quantidade e clones, incluindo indireção, detach e alocação. Parte já existe.|
|Despacho estático|C23|Content nativo já é fechado; provar custo do World antes de propagar generics.|
|Reutilização/capacidade|C03–C11, C18–C22, C27/C28|Testar um buffer/coleção de cada vez, retenção limitada e sem estado residual.|
|Bounds checks|C10, C05|Assembly de laço material e alternativas seguras; não introduzir unchecked por hipótese.|
|Threads com estado próprio|C23|Começar após layout estabilizado; incluir perda de cache, custo de cópia e merge determinístico.|
|Estruturas persistidas|C24|Definir frescor e versão antes do formato de bytes. O owner atual proíbe disk cache nessa rota.|

A ADR-0029 permite gerenciamento local de RAM; não precisa ser revista só porque se usam índices ou arenas. Substituir Sequence compartilhada conflita com ADR-0026-R1 e o L0 atual. Trocar payload público ou World genérico exige classificar a alteração concreta pela ADR-0127. Arenas em L1 com crate nova exigem whitelist. Todos os testes de candidatos implementáveis exigem L0 atual antes do produto; contratos compartilhados seguem owners 1:1 e nuclei V15/V26. Nenhuma revisão/ratificação foi solicitada agora, pois não há implementação proposta para aprovação.

## Candidatos delimitados

### C01 — Vida útil e identidade do cache de imagens (P0)

**Evidência:** [03_infra/src/export/images/scan.rs](../../03_infra/src/export/images/scan.rs#L24). PAYLOAD_CACHE é thread-local, indexado por usize derivado do Arc; get().cloned() e insert(p.clone()) retêm payloads. As limpezas encontradas estão nos testes. O conjunto validated e ptr_to_idx, por outro lado, é criado por invocação.

**Teste proposto:** No mesmo thread, exportar dois documentos com PNGs distintos. Primeiro manter um Arc externo, liberar o documento e usar Arc::get_mut para trocar os bytes no mesmo Arc, quando a propriedade única permitir; depois construir o segundo documento. Separadamente testar reciclagem de endereços e exportações com muitas imagens únicas. Registrar pixels, payload selecionado e bytes retidos.

**Critério e refutação:** O segundo resultado deve refletir exclusivamente a imagem atual. Medir também crescimento da retenção após documentos descartados. Risco inferido da leitura, ainda não reproduzido; não declarar defeito confirmado.

**Esforço relativo:** baixo para o teste isolado; médio no watch. **L0/gate:** [00_nucleo/prompts/infra/export/images/scan.md](../../00_nucleo/prompts/infra/export/images/scan.md). L0 manda preservar o comportamento atual de lifetime/key do TLS. Qualquer correção exige evidência do caso, revisão desse owner e classificação ADR-0127; não limpar silenciosamente o cache como otimização.

### C02 — Atribuição das consultas de frescor (P0)

**Evidência:** [03_infra/src/fontdb/generation.rs](../../03_infra/src/fontdb/generation.rs#L175). fast_key/fast_key_unchanged combinam configuração e provas de raízes. A bancada conjunta anterior reduziu enumeração e acrescentou statx/read, sem ganho final consistente.

**Teste proposto:** Plain no startup, original/controle/candidato: separar observação lexical, destino canônico, configuração, G1/G2/G3 e prova final. Agregar por operação/path/motivo/barreira; medir overhead off/on. Usar strace somente fora dos relógios.

**Critério e refutação:** Explicar o delta de consultas e o custo por fronteira antes de outro candidato. Manter oito campos, follow/nofollow, ordem e barreiras. Refutar a hipótese de custo RAM dominante se as próprias observações explicarem o tempo.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/infra/fontdb/generation.md](../../00_nucleo/prompts/infra/fontdb/generation.md). Owners generation/font_session e experimento joint-font-discovery; não repetir o percurso conjunto como uma nova otimização já validada. Sem retirada de reparse ou prova.

### C03 — Construção da chave de shaping (P1)

**Evidência:** [03_infra/src/shaper/cache.rs](../../03_infra/src/shaper/cache.rs#L36). Cada chave produz String; eixos usam format!, collect Vec e join antes do format final. O lookup em shaping.rs:425 é precedido dessa construção.

**Teste proposto:** Contar chamadas, comprimento/bytes alocados, hits/misses e tempo de chave versus shape. Em lab, comparar codificação atual e uma chave estruturada que preserve exatamente cada dimensão. Repetições e textos/eixos únicos.

**Critério e refutação:** Resultado e hit/miss equivalentes em AB/BA: bits f32, ordem dos eixos, tracking i32, slot, direção, ssty, idioma ausente/explícito e superscript efetivo. Ganho deve sobreviver ao custo de hash/lookup e ao pipeline.

**Esforço relativo:** baixo/médio. **L0/gate:** [00_nucleo/prompts/infra/shaper/cache.md](../../00_nucleo/prompts/infra/shaper/cache.md). O L0 especifica chaves textuais exatas; atualizar antes de alterar representação. Não reduzir a chave a hash sem igualdade completa nem arredondar eixos.

### C04 — Alocação em hits de métricas (P1)

**Evidência:** [03_infra/src/font_metrics/caches.rs](../../03_infra/src/font_metrics/caches.rs#L160). shaped_width_key/advance_width_key criam text.to_string() antes do lookup e recalculam hashes de fontes/eixos. Locks são curtos para lookup/insert; compute fica fora.

**Teste proposto:** Medir alocações e custo da chave em hits reais. Comparar lookup com entrada emprestada ou identidade estável local, sem mudar campos nem escopo; separar custo de lock de custo de compute.

**Critério e refutação:** Mesmo valor para cache compartilhado e instância fresca, em ambas as ordens; preservar tamanho explícito, math/TextItem/ssty, idioma, eixos, tracking bypass, retry de falha e lock order.

**Esforço relativo:** baixo/médio. **L0/gate:** [00_nucleo/prompts/infra/font_metrics/caches.md](../../00_nucleo/prompts/infra/font_metrics/caches.md). L0 fixa campos e escopos; não trocar Mutex por RefCell presumindo uso exclusivo, nem estender lifetime entre Worlds.

### C05 — Reúso dos buffers de rustybuzz (P1)

**Evidência:** [03_infra/src/shaper/shaping.rs](../../03_infra/src/shaper/shaping.rs#L456). Há UnicodeBuffer::new em caminhos de medição e emissão e um Vec de clusters materializado a partir de glyph_infos.

**Teste proposto:** Medir capacidade, alocações e retenção por sub-run. Em bancada, comparar buffer reutilizado com reset completo e leitura direta dos clusters em zip/windows seguros, um mecanismo por vez.

**Critério e refutação:** Fresh/reused iguais em LTR/RTL, scripts mistos, idioma, ignoráveis, ligaduras, tracking, ssty/sups e falhas. Sem estado residual, sem mudança de clusters/caracteres extraídos e com limite de capacidade retida.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/infra/shaper/shaping.md](../../00_nucleo/prompts/infra/shaper/shaping.md). Revisar owner shaping e política da dependência fixada antes de implementar reset; nenhuma API nova ou unsafe para eliminar bounds checks.

### C06 — Cópias dos glifos nos hits (P2)

**Evidência:** [03_infra/src/shaper/shaping.rs](../../03_infra/src/shaper/shaping.rs#L444). CachedRun deriva Clone com Vec<ShapedGlyph>; hit chama cloned(), miss clona run_glyphs para armazenar e move outro Vec para TextShaped.

**Teste proposto:** Contar glifos/bytes copiados e frequência hit/miss. Primeiro avaliar empréstimo/adiamento de cópia dentro do carrier existente; só depois modelar payload compartilhado isolado.

**Critério e refutação:** Manter glifos, clusters, fonte real, estilos e independência das transformações posteriores. Contabilizar Atomic/Arc, memória retida e efeitos em todos os exportadores.

**Esforço relativo:** médio/alto. **L0/gate:** [00_nucleo/prompts/infra/shaper/shaping.md](../../00_nucleo/prompts/infra/shaper/shaping.md). TextShaped.glyphs é contrato público; uma troca por Arc não é correção local autorizada. Exige L0s dos carriers/consumidores e gate ADR-0127 se mudar tipo público.

### C07 — Buffer de saída do walker de shaping (P1)

**Evidência:** [03_infra/src/shaper/traversal.rs](../../03_infra/src/shaper/traversal.rs#L52). Cada camada cria Vec::with_capacity(items.len()), drena o anterior e o substitui. Sub-runs podem expandir a quantidade de itens; grupos/links/semântica também percorrem filhos.

**Teste proposto:** Contar alocações e capacidades por camada e profundidade. Comparar scratch local alternado, preservando buffer de origem e saída distintos enquanto necessário.

**Critério e refutação:** Ordem background→items→foreground, wrappers/metadados, expansão, fallback sem fonte e itens não textuais iguais. Nenhuma retenção de documento anterior nem re-shaping de TextShaped.

**Esforço relativo:** baixo/médio. **L0/gate:** [00_nucleo/prompts/infra/shaper/traversal.md](../../00_nucleo/prompts/infra/shaper/traversal.md). L0 traversal: caches por documento e ordem explícita. Candidato interno, com owner atualizado antes do código.

### C08 — Cópias entre tentativas de convergência (P1)

**Evidência:** [01_core/src/compiler/layout/convergence.rs](../../01_core/src/compiler/layout/convergence.rs#L132). Mapas de páginas, figure/table page numbers e estado de diagnóstico são clonados para o Layouter; ao rejeitar tentativa, mapas extraídos são clonados novamente. metrics.clone é compartilhamento existente, não prova de cópia profunda.

**Teste proposto:** Registrar tipo, cardinalidade e bytes copiados por coordenador/tentativa. Comparar transferência de propriedade somente de produtos de candidatos definitivamente descartados; medir separadamente sharing dos dados somente leitura.

**Critério e refutação:** Mesmos candidatos necessários, Complete/Pending, mapas, callbacks, diagnósticos e rollback. Injetar falha tardia e transcritos pendentes. Não tratar tentativas distintas como resultados intercambiáveis.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/core/compiler/layout/convergence.md](../../00_nucleo/prompts/core/compiler/layout/convergence.md). Owner convergence e contratos de page-running; tipos públicos alterados exigem gate. Não remover tentativas por contagem nem memoizar Content apenas por hash morfológico.

### C09 — Checkpoint: custo residual de snapshots (P2)

**Evidência:** [01_core/src/compiler/layout/checkpoint.rs](../../01_core/src/compiler/layout/checkpoint.rs#L209). Prefixos de FrameItem já são preservados por comprimentos/truncate. Ainda há clones de runtime, mapas, footnotes, planos e caches declarados no checkpoint.

**Teste proposto:** Medir bytes/campos e taxa de checkpoints/restores por tabelas, notas e subframes. Modelar journal/deltas apenas para um campo com custo material e reversão completa comprovável.

**Critério e refutação:** Restaurar também mutações em entradas existentes; prefixo aceito intacto, sem publicar eventos nem consumir resoluções físicas especulativas. Casos de falha/nesting obrigatórios.

**Esforço relativo:** alto. **L0/gate:** [00_nucleo/prompts/core/compiler/layout/checkpoint.md](../../00_nucleo/prompts/core/compiler/layout/checkpoint.md). L0 explicitamente exige estado causal completo. Reescrever tudo como arena ou somente truncar comprimentos violaria o contrato atual.

### C10 — Scratch e bounds checks do seletor produtivo (P1)

**Evidência:** [01_core/src/compiler/layout/paragraph_select.rs](../../01_core/src/compiler/layout/paragraph_select.rs#L155). O seletor produtivo aloca prefix, before, totals e predecessors. O laço indexa fatias com limites relacionados. Bounded/forward, apesar de aparecerem no scan, são módulos cfg(test).

**Teste proposto:** Medir alocações por parágrafo e cada tamanho. Comparar scratch local limitado; inspecionar assembly de laços medidos, depois avaliar zip/slices seguros. Bench deve incluir preparação e reconstrução, não só DP interno.

**Critério e refutação:** Mesmas linhas, custos, desempates, progresso em palavra longa, tracking, hifenização e formas primeira/continuação. Reusar os históricos como hipóteses, não como certificação do estado atual.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/core/compiler/layout/paragraph_select.md](../../00_nucleo/prompts/core/compiler/layout/paragraph_select.md). Não promover protótipos test-only nem podas aproximadas como se já estivessem no pipeline. Owner paragraph_select; separar buffers de mudança de algoritmo.

### C11 — Preparação de parágrafos e arena limitada (P2)

**Evidência:** [01_core/src/compiler/layout/paragraph_prepare.rs](../../01_core/src/compiler/layout/paragraph_prepare.rs#L137). Prepared possui Vec de palavras, fragments e gaps; há style.clone por fragmento e mem::take para transferir trailing. Recusas e restauração de escopo fazem parte da rota.

**Teste proposto:** Medir distribuição de pequenos Vecs, clones de estilo e caminhos recusados. Primeiro capacidade/scratch; arena somente se temporários nascem e morrem juntos e a economia do alocador for material.

**Critério e refutação:** Preparação recusada emite zero e restaura estado; preservar proveniência de espaços/SHY, estilos e elegibilidade. Verificar Drop dos campos e nenhum empréstimo escapando para frames/comemo.

**Esforço relativo:** médio/alto. **L0/gate:** [00_nucleo/prompts/core/compiler/layout/paragraph_prepare.md](../../00_nucleo/prompts/core/compiler/layout/paragraph_prepare.md). Arena global proibida. Nova crate L1 exige whitelist e L0; não é preciso alterar ADR-0029 apenas para alocação local em RAM.

### C12 — Caminhos de resolução de estilos (P2)

**Evidência:** [01_core/src/entities/style_chain.rs](../../01_core/src/entities/style_chain.rs#L359). local_delta_since clona deltas e constrói cadeia intermediária; collapse percorre pais e usa busca linear nos custom já retidos. custom faz busca lexical e devolve empréstimo.

**Teste proposto:** Contar profundidade, número de propriedades e consultas reais. Comparar acumulação local ou resultado efetivo por escopo imutável, incluindo custo de preparar/invalidate esse resultado.

**Critério e refutação:** Top-wins, ausência versus limpeza, composição relativa de tamanhos, ordem e sombras preservadas. Sem achatar origem declarativa necessária à morfologia ou capturas show-set.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/core/entities/style_chain.md](../../00_nucleo/prompts/core/entities/style_chain.md). Owner style_chain e consumidores; cache de estilo não pode usar chave que ignore cadeia/contexto. Não presumir que todos os clones de Arc custem como Vec.

### C13 — Construção da biblioteca em imports (P1)

**Evidência:** [01_core/src/compiler/eval/modules.rs](../../01_core/src/compiler/eval/modules.rs#L59). eval_imported_file chama make_stdlib_with_features com inputs, clona stdlib para std e espalha bindings num global local a cada avaliação importada.

**Teste proposto:** Medir construção da biblioteca separada do corpo importado. Testar base somente leitura reutilizável no escopo da compilação com mesma identidade de inputs/features, sem memoizar o Module resultante neste primeiro teste.

**Critério e refutação:** Sombras de std/builtins, inputs/features, módulos isolados, closures/contexto, set/show e diagnósticos iguais. Comparar muitos imports pequenos e poucos caros.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/core/compiler/eval/modules.md](../../00_nucleo/prompts/core/compiler/eval/modules.md). L0 modules exige isolamento/base; não compartilhar escopo mutável nem mudar nome global. Se representação pública da biblioteca mudar, classificar gate específico.

### C14 — Memoização mínima de imports reais (P2)

**Evidência:** [01_core/src/compiler/eval/modules.rs](../../01_core/src/compiler/eval/modules.rs#L44). Há CST retido, mas isso não guarda a avaliação do import. O experimento 4 só demonstrou hit de projeção isolada com World congelado.

**Teste proposto:** Editar só o main e demonstrar execução realmente evitada do import no caller real. Testar dependência transitiva, missing→present, inputs/target/features, ciclo/erro, diagnósticos e efeitos. Medir chave+validação+replay.

**Critério e refutação:** Module/bindings/closures corretos entre Worlds e hit útil após edição normal, sem emprestar fonte ou identidade antiga. Ganho final no watch; startup/CLI independentes.

**Esforço relativo:** alto. **L0/gate:** [00_nucleo/prompts/core/compiler/eval/modules.md](../../00_nucleo/prompts/core/compiler/eval/modules.md). Conciliar autorização comemo/ADR-0030 e V13 no escopo concreto. Engine.world não vira tracked só por existir #[track] em outro objeto. L0 antes de código.

### C15 — Clones profundos de Array/Dict e coleções (P2)

**Evidência:** [01_core/src/entities/value.rs](../../01_core/src/entities/value.rs#L98). Array é Vec<Value>, Dict é IndexMap; Clone derivado propaga custo profundo. Callbacks de sorted/filter/map também clonam valores, mas ordem/quantidade de callbacks é contrato.

**Teste proposto:** Perfil por tipo de Value em bindings/capturas e coleções grandes compartilhadas versus mutadas. Modelar COW local em lab e comparar read-mostly/unique/mutate-heavy. Separar ordenação com callback da sem callback.

**Critério e refutação:** Independência de mutação, captura léxica, ordem de dict, estabilidade, curto-circuito e erros preservados. Custo de detach e retenção incluídos; não trocar insertion sort(by:) por outro algoritmo ignorando callbacks.

**Esforço relativo:** alto para representação; baixo para perfil. **L0/gate:** [00_nucleo/prompts/core/entities/value.md](../../00_nucleo/prompts/core/entities/value.md). L0 Value mantém Array/Dict atuais e público enum. COW requer pacote de owners e ADR-0127; autorização RAM não autoriza mudança pública automaticamente.

### C16 — Tamanho e frequência dos carriers (P1)

**Evidência:** [01_core/src/entities/value.rs](../../01_core/src/entities/value.rs#L1818). Já existe guarda sizeof(Value/Binding); Stroke já usa Arc. Content possui muitas variantes Arc, Sequence usa Arc slice; alguns payloads e FrameItem continuam inline.

**Teste proposto:** Medir size_of/align_of no build real, frequência/pico vivo por variante e multiplicação por contêiner. Escolher a maior contribuição tamanho×quantidade, não a maior variante isolada.

**Critério e refutação:** Comparar RSS/heap vivo, alocações, clones e tempo. Boxing deve justificar indireção/alocação; preservar linguagem. Não reapresentar a compactação já executada como ganho novo.

**Esforço relativo:** baixo para medição; variável para mudança. **L0/gate:** [00_nucleo/prompts/core/entities/value.md](../../00_nucleo/prompts/core/entities/value.md). Owners Value/Content/layout_types/Binding. Payload público e ABI exigem gate. Limite por plataforma/toolchain, não promessa portátil de tamanho exato.

### C17 — Índices u32 numa árvore local (P3)

**Evidência:** [01_core/src/entities/syntax_node/representation.rs](../../01_core/src/entities/syntax_node/representation.rs#L24). SyntaxNode tem Leaf inline e Inner/Error Arc, com edição COW, spans, caches estruturais, numbering e navegação. Parser já possui MemoArena de Vec<SyntaxNode> com ranges.

**Teste proposto:** Construir modelo isolado de árvore congelada por arquivo; medir clone/drop, travessia e retenção. Preservar proprietário do armazenamento, checks de overflow, identidade de geração e coexistência de CSTs antigos/novos.

**Critério e refutação:** Mesma árvore/texto/spans/erros e isolamento de edição. Medir custo de manter arena viva para um nó retido; não inferir ganho de dividir tamanho de ponteiro por dois.

**Esforço relativo:** alto. **L0/gate:** [00_nucleo/prompts/core/entities/syntax_node/representation.md](../../00_nucleo/prompts/core/entities/syntax_node/representation.md). Não substituir AST/Content globalmente. L0 representation fixa COW/Arc; ADR-0026-R1/L0 Content precisam decisão se Sequence mudar. ADR-0029 não proíbe índices nem exige sua revisão só por usá-los.

### C18 — Concatenação de texto da CST (P2)

**Evidência:** [01_core/src/entities/syntax_node/representation.rs](../../01_core/src/entities/syntax_node/representation.rs#L120). into_text em Inner cria String e concatena textos de filhos recursivamente; em árvores profundas pode haver cópias intermediárias. texto() de folha é O(1) e não deve ser confundido com esta rota.

**Teste proposto:** Localizar callers produtivos de into_text e medir bytes copiados por profundidade; comparar um acumulador com reserva pelo len conhecido.

**Critério e refutação:** Mesmo texto byte a byte e erros/spans intactos, sem nova cópia de toda Source nos caminhos que já usam slices. Recusar prioridade se callers forem só testes ou pouco frequentes.

**Esforço relativo:** baixo/médio. **L0/gate:** [00_nucleo/prompts/core/entities/syntax_node/representation.md](../../00_nucleo/prompts/core/entities/syntax_node/representation.md). Owner representation; SyntaxText e COW preservados. Não precisa começar pela migração para índices.

### C19 — Serialização e capacidade de saída PDF (P1)

**Evidência:** [03_infra/src/export/builder/serialization.rs](../../03_infra/src/export/builder/serialization.rs#L108). serialize cresce Vec final e formata cabeçalhos/xref em temporários; utf16be_hex_string cria bytes e format! por byte. offsets usa HashMap de ids.

**Teste proposto:** Medir alocações/cópias separadas de subset, compressão e desenho. Comparar reserva segura pelo tamanho dos objetos e escrita em buffer, mantendo layout de saída requerido. Avaliar índice denso só se ids/consumo justificarem.

**Critério e refutação:** PDF válido, xref/offsets corretos, ordem de objetos, escaping, Unicode, IDs/metadata e modos Pretty/compact/oracle preservados. Overflow e objetos ausentes testados.

**Esforço relativo:** baixo/médio. **L0/gate:** [00_nucleo/prompts/infra/export/builder/serialization.md](../../00_nucleo/prompts/infra/export/builder/serialization.md). Owner serialization/streams; default verbose é decisão ADR-0126. Não ganhar tempo mudando default para compact ou omitindo tags/metadata.

### C20 — Escaping e formatação HTML/SVG (P1)

**Evidência:** [03_infra/src/export/html/encoding.rs](../../03_infra/src/export/html/encoding.rs#L224). HTML faz cadeia de replace com Strings intermediárias. SVG fmt_pair chama fmt_num antes de format final. São pontos de alocação reais, não prova de custo dominante.

**Teste proposto:** Contar bytes/copias e comparar escrita numa saída única em textos longos e atributos com alta/baixa taxa de escape; SVG com muitos paths/números.

**Critério e refutação:** Mesmos escapes por modo/contexto, whitespace/DOM, número/precisão/-0, raw/title e diagnósticos. Não mudar distinções de HTML semântico nem regras do serializer.

**Esforço relativo:** baixo. **L0/gate:** [00_nucleo/prompts/infra/export/html/encoding.md](../../00_nucleo/prompts/infra/export/html/encoding.md). Owners encoding/svg serialization e callers. Modificar helpers internos pode seguir fluxo contínuo após L0; API pública nova não é necessária para medir.

### C21 — PNG composto sem encode/decode intermediário (P3)

**Evidência:** [03_infra/src/export/render.rs](../../03_infra/src/export/render.rs#L172). render_document_to_png codifica cada página, decodifica os PNGs, premultiplica, clona raw e compõe a imagem final. A busca encontrou export público/reexport e teste; não caller interno CLI deste helper composto.

**Teste proposto:** Primeiro demonstrar uso no cenário alvo; depois medir pixmap direto com composição e encode final único. Separar de renderização PNG por página que já tem outra entrada.

**Critério e refutação:** Mesmos pixels/alpha, bleed, fill, gap, páginas vazias e clipping. Medir pico de memória ao acumular pixmaps; não anunciar ganho CLI/PDF de helper público sem caller.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/infra/export/render.md](../../00_nucleo/prompts/infra/export/render.md). Owner render; contratos públicos de saída preservados. Prioridade baixa enquanto alcance no produto não demonstrado.

### C22 — Recursos de exportação e associação de fontes (P2)

**Evidência:** [03_infra/src/pipeline/resources.rs](../../03_infra/src/pipeline/resources.rs#L82). Combinações são retidas em Vec ordenado e lookup linear; recursos.rs resolve fonte em Vec<u8>. FontBinding já possui LookupCache local e Cow/transferência para evitar cópias desnecessárias.

**Teste proposto:** Medir cardinalidade e bytes de fontes copiados por export; comparar tabela de consulta com ordem estável somente em corpus com muitas combinações/eixos. Verificar miss/hit dos caches existentes antes de criar outro.

**Critério e refutação:** Mesma fonte real+eixos, ordem do primeiro encontro, instancing e GID/ToUnicode. Nomes ou slot isolado não identificam a face.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/infra/pipeline/resources.md](../../00_nucleo/prompts/infra/pipeline/resources.md). Owners resources/font_binding; trabalho anterior prioridade7 já reduziu clones. Não descartar lookup local nem alegar repetição que já foi removida.

### C23 — Despacho World e paralelismo após layout (P3)

**Evidência:** [01_core/src/entities/engine.rs](../../01_core/src/entities/engine.rs#L45). Engine usa &dyn World; layout já é genérico em FontMetrics/ImageSizer. Shaping cria caches por documento e percorre páginas sequencialmente; o layout tem dependências de fluxo/convergência.

**Teste proposto:** Primeiro medir custo inclusivo das chamadas World versus trabalho L3. Comparar devirtualização em adaptador isolado apenas se material. Para threads, começar por render/export de páginas já estabilizadas, com estado local e merge determinístico.

**Critério e refutação:** Mesma seleção, efeitos, recursos, diagnósticos e ordem; comparar tempo de compilação/tamanho do binário, CPU/wall/RSS, cópias e perda de hits dos caches por worker.

**Esforço relativo:** alto. **L0/gate:** [00_nucleo/prompts/core/entities/engine.md](../../00_nucleo/prompts/core/entities/engine.md). Generificar World propaga mudança pública por vários owners; gate ADR-0127. Não paralelizar layout por página presumindo independência. Sem remoção genérica de dyn ou Mutex.

### C24 — Persistência do catálogo em disco (P3)

**Evidência:** [03_infra/src/fontdb/generation.rs](../../03_infra/src/fontdb/generation.rs#L38). A validação de conteúdo serializado não prova que fontes/configuração atuais correspondem ao catálogo; o L0 de generation explicitamente exclui disk cache nessa rota.

**Teste proposto:** Somente após definir chave/frescor e medir parcela de reconstrução evitável: bancada isolada de leitura+validação+versão contra preparação fresca, incluindo escrita/corrupção/limites.

**Critério e refutação:** Mesmo catálogo, ordem/índices/cobertura e recusas diante de alterações; custo total menor com a mesma prova. Não reapresentar cache Fontconfig reprovado como substituto equivalente.

**Esforço relativo:** alto. **L0/gate:** [00_nucleo/prompts/infra/fontdb/generation.md](../../00_nucleo/prompts/infra/fontdb/generation.md). Nova rota persistente exige revisão L0 e decisão concreta de default/compatibilidade. Nenhuma nova dependência rkyv/zerocopy é necessária para a investigação atual.

### C25 — Namespace dos caches de plugins entre hosts (P0)

**Evidência:** [01_core/src/entities/plugin_func.rs](../../01_core/src/entities/plugin_func.rs#L49). Os métodos memoizados usam Eq/Hash sem host; o carregador atribui ids no host. O diagnóstico de memoização já registra a possibilidade de colisão entre hosts.

**Teste proposto:** Em um processo, dois hosts com mesmo module/name/args e respostas ou transições distintas; verificar call e transition em AB/BA e após novo World. Usar casos reais de importação de plugin além de mocks.

**Critério e refutação:** Cada função deve observar o host correto; falha concreta interrompe qualquer ampliação de cache. Não tratar isso como descoberta inédita nem afirmar reprodução nesta auditoria.

**Esforço relativo:** baixo para controle; médio para integração. **L0/gate:** [00_nucleo/prompts/core/entities/plugin_func.md](../../00_nucleo/prompts/core/entities/plugin_func.md). Owner plugin_func reconhece a limitação. Chave/identidade pública e host ownership exigem classificação/L0 antes de correção.

### C26 — Retenção e limites do SourceSession (P3)

**Evidência:** [03_infra/src/world/source_session.rs](../../03_infra/src/world/source_session.rs#L16). O LRU usa Vec e limite explícito de oito entradas; lookup/remove e soma de bytes são lineares, mas limitados. CST só é reutilizada após leitura/validação dos bytes e identidade atual.

**Teste proposto:** Corpus de muitos imports acima do limite, hit/miss/eviction e bytes lidos/CST retida; separar efeito de capacidade do custo da estrutura. Só modelar outro índice se esse custo aparecer.

**Critério e refutação:** Sem CST antiga após edição, FileId/path errados, erro de leitura/UTF-8 ou arquivo recriado. Memória limitada e sem confundir CST com memo de eval.

**Esforço relativo:** baixo para perfil. **L0/gate:** [00_nucleo/prompts/infra/world/source_session.md](../../00_nucleo/prompts/infra/world/source_session.md). Owner SourceSession fixa limites; ampliá-los pode alterar consumo/contrato e exige classificação. HashMap não tem vantagem demonstrada para oito entradas.

### C27 — Cópias de Source no formatter de diagnósticos (P2)

**Evidência:** [02_shell/src/diagnostic.rs](../../02_shell/src/diagnostic.rs#L31). O adaptador Files retorna String e source() faz text().to_owned(); line_index chama esse source(), potencialmente copiando o arquivo inteiro para consultas de linha. Busca de FileId é linear na lista fornecida.

**Teste proposto:** Contar invocações e bytes copiados num arquivo grande com muitos erros e em traces de imports. Comparar acesso emprestado/índice local de linhas compatível com o trait fixado, preservando o formato.

**Critério e refutação:** Mesmas mensagens, ordem, spans, Unicode/tab/linhas vazias, cores e comportamento em source ausente. Ganho é de apresentação de erros, não da compilação bem-sucedida.

**Esforço relativo:** baixo/médio. **L0/gate:** [00_nucleo/prompts/shell/diagnostic.md](../../00_nucleo/prompts/shell/diagnostic.md). Owner shell/diagnostic; fonte permanece materializada por L4. Não fazer filesystem em L2 nem omitir diagnósticos para reduzir custo.

### C28 — Árvore intermediária de query/JSON e YAML (P2)

**Evidência:** [02_shell/src/cli.rs](../../02_shell/src/cli.rs#L2077). semantic_to_json reconstrói arrays/objetos e clona chaves antes da serialização. YAML já escreve num buffer, mas cria strings de indentação por chamada.

**Teste proposto:** Query com resultados grandes e estruturas aninhadas: medir árvores vivas, cópias, alocações e saída. Comparar serialização direta da projeção sem árvore serde_json adicional; capacidade/indentação numa bancada separada.

**Critério e refutação:** Mesmos modos/formato, ordem contratada, precisão, escaping, newline, valores especiais e erros. Não atribuir ganho PDF/CLI compile a um caminho exclusivo de query.

**Esforço relativo:** médio. **L0/gate:** [00_nucleo/prompts/shell/cli.md](../../00_nucleo/prompts/shell/cli.md). Owner shell/cli e caller query. Alterar modelo SemanticValue público requer classificação própria; uma comparação isolada não autoriza mudar seu contrato.

## O que a auditoria evita repetir

- `Content::Sequence` já compartilha Arc slices; muitas variantes pesadas já são indiretas. Não existe migração universal de Box→u32 autorizada ou ganho presumido de 50% do documento.
- `Value::Stroke` já foi compactado e existe guarda de footprint de Value/Binding. O [resultado de outubro](../../lab/results/construction/diagnostics/value-compacto-memoria5-execucao-2026-10-08.md) pertence àquele estado/pin; não é medida nova. Não fixar ABI portátil com sizeof exato.
- Parser já usa MemoArena de nós/ranges. SyntaxNode tem edição COW e SourceSession retém CST validada. Isso não memoiza eval e não justifica descartar proprietário/geração de um índice.
- `map_children_if_changed` em content/traversal.rs:376 já aloca apenas após o primeiro filho alterado. Checkpoint já evita cópia de prefixos de FrameItem por comprimentos. Uma proposta genérica de acrescentar esses mecanismos repetiria trabalho existente.
- Métricas/FaceCache/shaping/font binding e medição de células já têm caches locais. Raw mantém SyntaxSet/Theme lazy; Regex clonada compartilha compilado. A pergunta é custo/hit/miss/escopo do cache existente, não simplesmente acrescentar cache.
- `paragraph_select_bounded` e `paragraph_select_forward` estão sob cfg(test) no hub layout/mod.rs:170–173. Seus microbenches e podas históricas não comprovam melhoria no seletor produtivo ou no estado/pin atual.
- O helper PNG composto com encode/decode intermediário não teve caller interno CLI identificado. Otimizá-lo não demonstra ganho na exportação PDF nem no PNG por página.
- SourceSession tem um conjunto explicitamente pequeno e limitado. Encontrar Vec::position/remove não prova que um HashMap seja melhor. Mutex compartilhado entre clones não prova contenção; a computação de métricas já ocorre fora de vários locks.
- `dyn World` não torna o percurso inteiro dinâmico, e generics não eliminam o custo do trabalho de I/O chamado. Layout por página não é independente apenas por mover dados para owned.
- Os experimentos [TTC](bottleneck-experiment2-2026-11-04.md), [snapshots](bottleneck-experiment5-2026-11-04.md), [eventos/Fontconfig](bottleneck-experiment6-2026-11-04.md) e [descoberta conjunta](joint-font-discovery-2026-11-05.md) limitam candidatos já tentados. Não reciclar variantes reprovadas como solução segura.

## Medição mínima para escolher, em vez de ampliar a busca

1. **Reproduzir alcance:** fixture que passa pelo caller real, cold/startup/hot separados; contar execuções, alocações, bytes copiados/retidos e hits/misses com significado observável. Para C01/C25, começar pelo contraexemplo determinístico de identidade, não por benchmark.
2. **Atribuir:** tempo por responsabilidade, inclusive preparação/chave/validação/replay/drop; custos aninhados não são somados. Profiling do alocador fica fora do lote de tempos; instrumentação precisa off/on e overhead conhecido. Relógios somente fora de L1 puro.
3. **Limitar:** estimar teto plausível da economia a partir da parcela medida. Se a parcela for imaterial ou o caller não executar no cenário, rebaixar o candidato. Não transformar quantidade de clones/chamadas em porcentagem prevista.
4. **Comparar:** original, mesmo binário experimental desligado e candidato na mesma fixture, alternados. CLI nova, startup, watch quente/edições e erros/recusas pertinentes; mais corpus grande/pequeno e favorável/adversário. CPU, latência de publicação, RSS/retido e bytes de cópia devem acompanhar o resultado.
5. **Aceitar:** semântica/sintaxe/morfologia e seleção/diagnósticos/render preservados; ganho final repetível no alvo, sem deslocamento oculto de custo. Resultado inconclusivo permanece Unknown. Falha de correção bloqueia o candidato, sem enfraquecer o teste.

Construções futuras continuam na RAM exclusiva `/dev/shm`, com um job, teto de 12GiB e zero swap; sem competir com medições. O skill de materialização segregada aplica-se quando houver implementação/testes independentes, não é requisito para esta leitura estática. Não foram iniciados agentes ou alterações de produto nesta auditoria.

## Proveniência

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, **uncommitted working tree**. Captura inicial UTC `2026-11-05T16:56:39.638757+00:00`. As contagens são do estado presente, incluindo fontes recebidas não commitadas; não representam apenas HEAD. Lista exata de `git diff HEAD --stat=240 --stat-name-width=200` no apêndice. A lista inclui 182 arquivos rastreados, 7.719 inserções e 443 remoções já recebidos; não foram produzidas por esta auditoria. Arquivos não rastreados recebidos também foram incluídos na conferência de preservação; o catálogo experimental joint-font-discovery em lab foi preservado e não entra nas contagens produtivas.

Conferência final UTC `2026-11-05T17:08:12.146286+00:00`: os 52910 arquivos recebidos permaneceram idênticos. A única adição desta tarefa é este diagnóstico. Fontes/L0/configuração/lock e relatórios recebidos foram conferidos byte a byte ao encerrar; nenhuma alteração concorrente foi incorporada silenciosamente. Inventário/scan e snapshot de conferência foram temporários, sem novos manifestos versionados. Os links de fonte/L0 localizam as evidências; os diagnósticos antigos conservam sua própria proveniência e limitações. Não há nova comparação com vanilla, certificação de ganho, suite executada ou selo de linhagem nesta auditoria.

## Apêndice — diffstat recebido

```text
 00_nucleo/prompts/core/compiler/eval/bindings/field_access.md                 |  19 ++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md |  51 +++++++++++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/fields.md          |  30 +++++++-
 00_nucleo/prompts/core/compiler/eval/observation/elements.md                  |  63 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/eval/repr/content.md                          |  30 +++++++-
 00_nucleo/prompts/core/compiler/eval/rules/set.md                             |  41 ++++++++++-
 00_nucleo/prompts/core/compiler/eval/rules/set/text.md                        |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/eval/tests.md                                 |  29 +++++++-
 00_nucleo/prompts/core/compiler/introspect.md                                 |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/convergence.md                     |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/figure_demands.md                  |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/materialize_time.md                |  32 ++++++++-
 00_nucleo/prompts/core/compiler/introspect/page_running.md                    |  23 +++++-
 00_nucleo/prompts/core/compiler/introspect/walk.md                            |  16 ++++-
 00_nucleo/prompts/core/compiler/layout/bidi_origin.md                         |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/block.md                               |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/checkpoint.md                          |  61 +++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/columns.md                             |  60 ++++++++++++++--
 00_nucleo/prompts/core/compiler/layout/columns/flow.md                        |  18 ++++-
 00_nucleo/prompts/core/compiler/layout/columns/segmented.md                   |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/columns/transitions.md                 |  16 ++++-
 00_nucleo/prompts/core/compiler/layout/cursor.md                              |  40 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/equation.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/figure.md                              |  32 ++++++++-
 00_nucleo/prompts/core/compiler/layout/flush.md                               |  19 ++++-
 00_nucleo/prompts/core/compiler/layout/footnote.md                            |  39 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_flush.md                      |  24 ++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_fragments.md                  |  31 +++++++-
 00_nucleo/prompts/core/compiler/layout/grid/cell_emission.md                  |  95 ++++++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/grid/tracks.md                         |  81 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/helpers.md                             |  10 ++-
 00_nucleo/prompts/core/compiler/layout/image.md                               |  58 ++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/measurement.md                         |  19 ++++-
 00_nucleo/prompts/core/compiler/layout/mod.md                                 |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/page_run.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/page_running_requests.md               |  53 +++++++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph.md                           |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph_indent.md                    |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/place.md                               |  32 ++++++++-
 00_nucleo/prompts/core/compiler/layout/placement.md                           |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/references.md                          |  48 ++++++++++++-
 00_nucleo/prompts/core/compiler/layout/region_progress.md                     |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/repeat.md                              |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/set_page.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/shape.md                               |  37 +++++++++-
 00_nucleo/prompts/core/compiler/layout/slicing.md                             |  10 ++-
 00_nucleo/prompts/core/compiler/layout/stack.md                               |  69 +++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/sub_frame.md                           |  17 ++++-
 00_nucleo/prompts/core/compiler/layout/text/style.md                          |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/tiling.md                              |  37 +++++++++-
 00_nucleo/prompts/core/compiler/math/layout/delimited.md                      |  80 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/external_bridge.md                |  34 ++++++++-
 00_nucleo/prompts/core/compiler/math/layout/mod.md                            |  81 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/op.md                             |  52 +++++++++++++-
 00_nucleo/prompts/core/compiler/realize/figure.md                             |  32 ++++++++-
 00_nucleo/prompts/core/compiler/realize/figure_kind.md                        |  19 ++++-
 00_nucleo/prompts/core/compiler/stdlib/figure_image.md                        |  87 ++++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/tests/figure_image.md                  |  65 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/text/constructor.md                    |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/visualize.md                           |  37 +++++++++-
 00_nucleo/prompts/core/entities/content/construction.md                       |  70 ++++++++++++++++++-
 00_nucleo/prompts/core/entities/content/fields.md                             |  30 +++++++-
 00_nucleo/prompts/core/entities/content/tests/traversal.md                    |  19 ++++-
 00_nucleo/prompts/core/entities/content/traversal.md                          |  20 +++++-
 00_nucleo/prompts/core/entities/elements/figure.md                            |  20 +++++-
 00_nucleo/prompts/core/entities/elements/image.md                             |  64 ++++++++++++++++-
 00_nucleo/prompts/core/entities/image_format.md                               |  26 ++++++-
 00_nucleo/prompts/core/entities/image_sizer.md                                |  26 ++++++-
 00_nucleo/prompts/core/entities/layout_types.md                               |  31 +++++++-
 00_nucleo/prompts/core/entities/mod.md                                        |  24 ++++++-
 00_nucleo/prompts/core/entities/tiling.md                                     |  37 +++++++++-
 00_nucleo/prompts/infra/export/builder.md                                     |  15 ++++
 00_nucleo/prompts/infra/export/builder/resources.md                           |  15 +++-
 00_nucleo/prompts/infra/export/images.md                                      |  31 +++++++-
 00_nucleo/prompts/infra/export/images/png.md                                  |  20 +++++-
 00_nucleo/prompts/infra/export/images/scan.md                                 |  31 +++++++-
 00_nucleo/prompts/infra/export/render.md                                      |  36 +++++++++-
 00_nucleo/prompts/infra/export/stream/draw.md                                 |  10 ++-
 00_nucleo/prompts/infra/export/svg/images.md                                  |  19 ++++-
 00_nucleo/prompts/infra/export/svg/tests.md                                   |  10 ++-
 00_nucleo/prompts/infra/export/tests/mod.md                                   |  27 ++++++-
 00_nucleo/prompts/infra/fontdb/selective.md                                   |  10 ++-
 00_nucleo/prompts/infra/lib.md                                                |  17 ++++-
 00_nucleo/prompts/infra/pipeline/context_stabilization.md                     |  19 ++++-
 00_nucleo/prompts/infra/pipeline/paged.md                                     |  17 ++++-
 00_nucleo/prompts/infra/pipeline/tests.md                                     | 125 ++++++++++++++++++++++++++++++++-
 00_nucleo/prompts/infra/shaper/bidi.md                                        |  37 +++++++++-
 00_nucleo/prompts/infra/shaper/shaping.md                                     |  35 +++++++++-
 00_nucleo/prompts/infra/tests/svg_composite_graph.md                          |  10 ++-
 00_nucleo/prompts/shell/cli.md                                                |  19 ++++-
 01_core/src/compiler/eval/bindings/field_access.rs                            |   4 +-
 01_core/src/compiler/eval/bindings/field_access/content_methods.rs            |  21 ++++--
 01_core/src/compiler/eval/bindings/field_access/fields.rs                     |   6 +-
 01_core/src/compiler/eval/observation/elements.rs                             |  34 ++++++---
 01_core/src/compiler/eval/repr/content.rs                                     |  12 +++-
 01_core/src/compiler/eval/rules/set.rs                                        |  54 +++++++++++++-
 01_core/src/compiler/eval/rules/set/text.rs                                   |  42 ++++++++++-
 01_core/src/compiler/eval/tests.rs                                            |   4 +-
 01_core/src/compiler/introspect.rs                                            |   4 +-
 01_core/src/compiler/introspect/convergence.rs                                |   4 +-
 01_core/src/compiler/introspect/figure_demands.rs                             |   4 +-
 01_core/src/compiler/introspect/materialize_time.rs                           |  34 ++++++++-
 01_core/src/compiler/introspect/page_running.rs                               |  68 +++++++++++++++---
 01_core/src/compiler/introspect/walk.rs                                       |  69 +++++++++++++++++-
 01_core/src/compiler/layout/bidi_origin.rs                                    |   7 +-
 01_core/src/compiler/layout/block.rs                                          |   9 +--
 01_core/src/compiler/layout/checkpoint.rs                                     |  17 ++++-
 01_core/src/compiler/layout/columns.rs                                        |  32 +++++----
 01_core/src/compiler/layout/columns/flow.rs                                   |  58 ++++++++++++++-
 01_core/src/compiler/layout/columns/segmented.rs                              |  30 +++-----
 01_core/src/compiler/layout/columns/transitions.rs                            |  72 ++++++++++++++++++-
 01_core/src/compiler/layout/cursor.rs                                         |  75 +++++++++++++++++++-
 01_core/src/compiler/layout/equation.rs                                       |   9 +--
 01_core/src/compiler/layout/figure.rs                                         | 240 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/flush.rs                                          |   3 +-
 01_core/src/compiler/layout/footnote.rs                                       | 171 +++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/footnote_flush.rs                                 |  71 ++++++++++++++++++-
 01_core/src/compiler/layout/footnote_fragments.rs                             |  99 ++++++++++++++++++++++++--
 01_core/src/compiler/layout/grid/cell_emission.rs                             | 167 +++++++++++++++++++++++++++++++++++++++++---
 01_core/src/compiler/layout/grid/tracks.rs                                    | 114 ++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/helpers.rs                                        |   5 +-
 01_core/src/compiler/layout/image.rs                                          |  80 ++++++++++++++++++---
 01_core/src/compiler/layout/measurement.rs                                    |   4 +-
 01_core/src/compiler/layout/mod.rs                                            |   6 +-
 01_core/src/compiler/layout/page_run.rs                                       |  16 +----
 01_core/src/compiler/layout/page_running_requests.rs                          |  79 ++++++++++++++++++++-
 01_core/src/compiler/layout/paragraph.rs                                      |  78 ++++++++++++++++++---
 01_core/src/compiler/layout/paragraph_indent.rs                               |   9 +--
 01_core/src/compiler/layout/place.rs                                          |  67 ++++++++++++++++--
 01_core/src/compiler/layout/placement.rs                                      |   9 +--
 01_core/src/compiler/layout/references.rs                                     | 141 ++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/layout/region_progress.rs                                |  86 +++++++++++++++++++++--
 01_core/src/compiler/layout/repeat.rs                                         |   7 +-
 01_core/src/compiler/layout/set_page.rs                                       |  16 +----
 01_core/src/compiler/layout/shape.rs                                          |   7 +-
 01_core/src/compiler/layout/slicing.rs                                        |   4 +-
 01_core/src/compiler/layout/stack.rs                                          |  17 +++--
 01_core/src/compiler/layout/sub_frame.rs                                      |  57 ++++++++++++++-
 01_core/src/compiler/layout/text/style.rs                                     |  11 ++-
 01_core/src/compiler/layout/tiling.rs                                         |  12 +++-
 01_core/src/compiler/math/layout/delimited.rs                                 | 211 ++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/math/layout/external_bridge.rs                           |  10 ++-
 01_core/src/compiler/math/layout/mod.rs                                       |  10 ++-
 01_core/src/compiler/math/layout/op.rs                                        |  91 +++++++++++++++++++++++-
 01_core/src/compiler/realize/figure.rs                                        |  76 ++++++++++++++++++--
 01_core/src/compiler/realize/figure_kind.rs                                   |   6 +-
 01_core/src/compiler/stdlib/figure_image.rs                                   | 114 ++++++++++++++++++++++++------
 01_core/src/compiler/stdlib/tests/figure_image.rs                             | 282 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/stdlib/text/constructor.rs                               |  20 +++++-
 01_core/src/compiler/stdlib/visualize.rs                                      |   7 +-
 01_core/src/entities/content/construction.rs                                  |   9 ++-
 01_core/src/entities/content/fields.rs                                        |   4 +-
 01_core/src/entities/content/tests/traversal.rs                               |   4 +-
 01_core/src/entities/content/traversal.rs                                     |   4 +-
 01_core/src/entities/elements/figure.rs                                       |  79 ++++++++++++++++++++-
 01_core/src/entities/elements/image.rs                                        |  21 ++++--
 01_core/src/entities/image_format.rs                                          |   9 ++-
 01_core/src/entities/image_sizer.rs                                           |  13 +++-
 01_core/src/entities/layout_types.rs                                          |   4 +-
 01_core/src/entities/mod.rs                                                   |   4 +-
 01_core/src/entities/tiling.rs                                                |   7 +-
 02_shell/src/cli.rs                                                           |   4 +-
 03_infra/Cargo.toml                                                           |   7 ++
 03_infra/src/export/builder.rs                                                |   2 +-
 03_infra/src/export/builder/resources.rs                                      |   7 +-
 03_infra/src/export/images.rs                                                 |   7 +-
 03_infra/src/export/images/png.rs                                             |  93 +++++++++++++++++++++++-
 03_infra/src/export/images/scan.rs                                            |  27 ++++++-
 03_infra/src/export/render.rs                                                 |  58 ++++++++++++++-
 03_infra/src/export/stream/draw.rs                                            |   3 +-
 03_infra/src/export/svg/images.rs                                             |   8 ++-
 03_infra/src/export/svg/tests.rs                                              |   4 +-
 03_infra/src/export/tests/mod.rs                                              |  15 +++-
 03_infra/src/fontdb/selective.rs                                              | 534 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 03_infra/src/lib.rs                                                           |   4 +-
 03_infra/src/pipeline/context_stabilization.rs                                |   4 +-
 03_infra/src/pipeline/paged.rs                                                |   4 +-
 03_infra/src/pipeline/tests.rs                                                | 593 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/src/shaper/bidi.rs                                                   |  78 ++++++++++++++++++++-
 03_infra/src/shaper/shaping.rs                                                |  14 ++--
 03_infra/tests/svg_composite_graph.rs                                         |   3 +-
 Cargo.lock                                                                    | 275 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++---
 182 files changed, 7719 insertions(+), 443 deletions(-)
```
