# Oportunidades para evoluir o Typst cristalino

Esta investigação seleciona áreas candidatas; não materializa código nem constitui Prompt L0. “Estado da arte” aqui significa aproximar uma unidade de técnicas demonstradas em motores atuais e verificar benefício observável. Não foi demonstrada superioridade sobre esses motores.

## Evidência e proveniência antes da decisão

Inspeção da working tree não commitada sobre HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. O arquivo irmão `oportunidades-estado-da-arte-2026-09-28/proveniencia.json` registra UTC, HEAD real, status completo, diff HEAD --stat e SHA-256 dos arquivos consultados. Não foram executados benchmarks, builds ou testes nesta investigação; não há novo número de desempenho.

O alvo de paridade permanece `9f2b6e8715237cb086899a42873660fe744622e8`. A documentação web atual serve como referência técnica; não altera o pin. Os tempos históricos de `infra-performance-followup.md` pertencem ao snapshot e à janela descritos naquele relatório e são exploratórios.

Evidências atuais:

- `03_infra/src/world/fonts.rs:65`: o builder adia a descoberta, mas sua closure compõe embedded, sistema e projeto integralmente. `:104` inicializa esse catálogo na primeira demanda via OnceLock. Adiar o catálogo não reduz o trabalho necessário quando a demanda ocorre.
- `03_infra/src/fontdb.rs:19`: a composição de informação e cobertura chama o extrator de metadados e faz parsing para cobertura. `:42` percorre as faces descobertas. Isso identifica trabalho a medir, não sua fração de tempo atual.
- `04_wiring/src/watch.rs:63`: cada ciclo chama run_compile_observed; `04_wiring/src/compile.rs:246` constrói um novo SystemWorld. A retenção do catálogo dentro de um World não preserva esse catálogo entre esses ciclos.
- `01_core/src/compiler/eval/closures.rs:40`: apply_closure prepara scopes e avalia a chamada sem uma fronteira de memoização nessa função. `:195` registra integração futura. Isso não prova ausência de toda reutilização no compilador.
- `03_infra/src/pipeline/context_stabilization.rs:414`: a estabilização produtiva já possui execução e validação de contribuições. O contrato atual exige validade dos registros e geração coerente. O helper `introspect/fixpoint` é outra unidade e seu L0 proíbe nova memoização; não é o alvo desta proposta.
- `03_infra/src/shaper/cache.rs:34`: resultados usam chave textual construída por formatação, preservando bits dos eixos, tracking efetivo e idioma. `03_infra/src/shaper/shaping.rs:256` configura idioma, já corrigido; `:289` chama rustybuzz::shape. No miss de emissão há criação de face e UnicodeBuffer.
- `lab/typst-original/crates/typst-layout/src/inline/shaping.rs:1026` usa shape_with_plan; `:1218` memoiza create_shape_plan com fonte, direção, script, idioma e features. É evidência mecânica da fonte local de referência, não prova de ganho se transplantada.

Os L0 vigentes de fonts, fontdb, cache, shaping, eval, closures, layout, introspect/fixpoint, context_stabilization, wiring/compile, wiring/watch e eviction foram lidos. Os hashes integrais de seus bytes estão na proveniência. Isso registra a leitura; não substitui a validação de linhagem pelo linter.

## Referências técnicas

- [Fontconfig: configuração, caches e rescan](https://fontconfig.pages.freedesktop.org/fontconfig/fontconfig-user.html): referência para armazenamento de metadados e atualização. Não implica que seus metadados substituam toda a cobertura exata exigida pelo Tekt.
- [Typst: arquitetura e incrementality](https://github.com/typst/typst/blob/main/docs/dev/architecture.md): descreve reutilização por módulos, closures e elementos de layout. Referência atual, distinta do oráculo pinado.
- [comemo](https://github.com/typst/comemo): memoização com acompanhamento de dependências; cache exige validade das leituras, não somente igualdade dos argumentos aparentes.
- [HarfBuzz: planos de shaping](https://harfbuzz.github.io/shaping-and-shape-plans.html): planejamento considera propriedades do segmento e capacidades da fonte.
- [rustybuzz 0.20.1: shape_with_plan](https://docs.rs/rustybuzz/latest/rustybuzz/fn.shape_with_plan.html): suporta plano fornecido e reciclagem de buffer; plano e propriedades do buffer precisam corresponder. Essa é a versão registrada no Cargo.lock local.

## Seleção e experimentos

### Fontes: reduzir aquisição repetida e desnecessária

**Classificação:** forte candidata de desempenho em L3. Linguagem a preservar: seleção, ordem de fallback, cobertura e métricas. Descoberta, parsing e retenção são mecânica livre sob ADR-0107.

**Hipótese:** reutilizar metadados válidos e reduzir parsing redundante pode diminuir o custo de aquisição. Eliminar demanda tipográfica desnecessária em subframes pode evitar a aquisição em alguns documentos. São hipóteses separadas, não uma otimização única.

**Experimento inicial:** dirigir os loaders reais sobre um conjunto fixo de arquivos/índices; contar descobertas, parses e aquisição, separar primeira carga de acesso retido e testar alterações de fontes/configuração. Para Place, variar somente alinhamento/baseline em controles com texto, sem texto, aninhamento e floats, verificando geometria.

**Refutação:** parsing adicional é irrelevante no perfil, metadados existentes não atendem à cobertura exata, ou posicionamento realmente exige aquelas métricas. Precarregar fora do timer não prova melhoria de primeira compilação.

**Limite de L0:** o contrato atual manda compor o catálogo completo na primeira demanda. Catálogo parcial não está autorizado por esse L0. Cache persistente ou sessão compartilhada exige desenho de invalidação e atualização dos owners; I/O continua em L3. Nenhuma política de seleção migra para L4.

### Shaping: reutilizar preparação além de resultados idênticos

**Classificação:** candidata de implementação mais delimitada. O motor rustybuzz já é usado; a oportunidade está na integração.

**Hipótese:** retenção de faces configuradas, planos e buffers beneficia documentos com muitos textos diferentes sob a mesma configuração, em que cache de resultados idênticos tem pouco alcance. Uma chave estruturada também pode reduzir formatação/alocação, mas é experimento separado e secundário.

**Experimento inicial:** comparar muitos runs distintos e repetidos mantendo fonte, script, direção, idioma, features e variações controlados; medir construção de planos/faces, alocações, tempo total e memória retida. Separar medição de largura e emissão, preservando seus contratos distintos.

**Aceitação:** semântica tipográfica e geometria preservadas; controles frescos e cache compartilhado em ordens AB/BA concordam para idioma, eixos e matemática. Glifos são controles auxiliares de render, não prova completa de paridade da linguagem.

**Refutação:** preparação não domina, cache de resultados já cobre a carga, ou memória/lookup anula o benefício. Não assumir que a chave atual cobre script forçado de matemática: o L0 declara essa suficiência não provada.

**Limite de L0:** atualizar cache/shaping e qualquer novo owner antes de implementação. O cache de faces atual armazena ttf-parser, não substitui automaticamente uma face rustybuzz. Não adicionar cache global nem harmonizar width/emission contra o contrato vigente.

### Incrementalidade: conservar trabalho validado entre edições

**Classificação:** maior alcance potencial e maior dificuldade. Começar por retenção de recursos no watch; memoização de closures e layout são frentes posteriores e independentes.

**Hipótese:** recriar World/catálogo em cada ciclo pode consumir trabalho evitável; chamadas repetidas de closures podem admitir reutilização com dependências causais corretamente acompanhadas. Pureza isoladamente não prova que uma chave é suficiente.

**Experimento inicial:** sequência de compilação inicial, repetição sem edição, edição local, mudança de import/asset/input e mudança de fonte. Comparar resultados com execução fresca; registrar trabalho reexecutado, latência por edição e memória retida. Incluir transição erro→correção e diagnóstico causal.

**Refutação:** o perfil mostra pouco trabalho reutilizável, o custo é predominantemente exportação, ou a invalidade não pode ser demonstrada com segurança. Preservar SourceSlot sem invalidar também preservaria fontes de texto e erros antigos: simplesmente manter o World é incorreto.

**Limite de L0:** watch e compile atuais coordenam nova execução; recursos persistentes devem ser propriedade de L3 e compostos por L4. O pipeline já valida contribuições contextuais; preservar esse mecanismo. Não alterar as fases ou substituir o helper fixpoint. Mudanças de contrato público/fase exigem o gate ADR-0127.

## Ordem de investimento

Priorizar fontes pelo gargalo histórico e pelo caminho atual confirmado. Escolher shaping como primeira frente delimitada se a meta for um experimento curto sobre uma unidade. Tratar incrementalidade como projeto subsequente, após medir o custo por edição e definir invalidação.

O objetivo imediato é um experimento que mostre melhoria observável com comportamento preservado. A rastreabilidade Tekt ajuda a localizar a obrigação e limitar a alteração; ela não torna um algoritmo superior por si mesma. Esta seleção não recomenda reescrita geral, despacho dinâmico nem mudança de topologia.
