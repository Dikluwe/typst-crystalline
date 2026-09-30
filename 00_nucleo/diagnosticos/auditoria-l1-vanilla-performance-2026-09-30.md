# Auditoria do L1 frente ao vanilla pinado e diferenças com impacto possível no desempenho

## Resultado principal

Na árvore medida em 30 de setembro de 2026, a maior diferença de tempo observada em documentos pequenos com texto coincide com a primeira construção do catálogo de fontes. Esse trabalho começa sob demanda, durante `eval` ou `layout`, mas sua implementação está sobretudo no L3. Nos documentos sem texto da sonda, os tempos totais dos dois binários ficaram próximos. Dentro do L1, o custo mais concreto encontrado é a segunda passagem de layout de células em grid; a dupla avaliação do documento e a ausência de memoização ampla são candidatos adicionais para documentos grandes ou recompilações, ainda sem atribuição causal de tempo.

Esta é uma auditoria transversal dos domínios do L1 e de seus caminhos de produção correspondentes no vanilla. A inspeção de código cobre a arquitetura de `contracts`, `entities`, lexer, parser, eval, stdlib, introspecção, layout, matemática e linguagem; as sondas executáveis cobrem cinco documentos qualificados. Não há perfil amostral que atribua tempo a cada função privada nem ensaio exaustivo de todos os 564 arquivos Rust do L1. Logo, “maior diferença” abaixo significa prioridade sustentada pelas evidências disponíveis, não uma percentagem global de custo do L1.

## Proveniência e método

- HEAD Tekt `17f13b1df79b87ea065ee62e78164b18e477da3d`; **working tree não commitado**. A lista exata de entradas alteradas e não rastreadas e `git diff HEAD --stat` no instante da sonda estão em [samples.json.gz](l1-vanilla-auditoria-2026-09-30/samples.json.gz). A árvore tinha outras alterações extensas antes desta auditoria; este trabalho não as reverteu.
- Janela sem tracing: **2026-09-30 14:22:25.033679–14:22:31.292145 UTC**. Fingerprint dos arquivos `.rs` de L1–L4 e `Cargo.toml`/`Cargo.lock`: `a3c531e607cdf41ef23596f052e6be6d22d1027a64444efeb9e7bb2a90cab8aa`. `git status --porcelain=v1 --untracked-files=all` e SHA dos executáveis coincidiram antes/depois. A lista não captura mudança e restauração entre leituras nem todas as fontes não Rust.
- `cargo build --release --locked --offline -p typst-wiring` concluiu antes da sonda. SHA-256 do Tekt release: `295faa5cf9a18e13274ce718848912585e5b4ac96175b6e006bde543795dd3c0`. Vanilla: commit pinado `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. A proveniência da fonte vanilla foi demonstrada em [P1489](typst-p1489-resync-vanilla.md).
- Seis pares alternados por fixture, novo processo em cada invocação, `SOURCE_DATE_EPOCH=0`, `RAYON_NUM_THREADS=1`, mesmo `--root`, `--font-path`, `--pdf-tagged=false`, `--pretty=false`; vanilla com `--jobs 1`. A sonda de fases tem três invocações por binário e fixture; recibos em [phase-samples.json.gz](l1-vanilla-auditoria-2026-09-30/phase-samples.json.gz). `pdfinfo` e `pdftotext` confirmaram, nos cinco pares inspecionados, mesmo número de páginas, dimensões e texto extraído. Não verificaram geometria de cada objeto, imagem ou paridade integral de linguagem.
- Coleta curta em desktop ativo, sem quiet gate de 30 segundos, afinidade fixa, perfil de alocações nem as três rodadas longas da [baseline](performance-baseline.md). Os números atuais são **sondas indicativas**. A baseline de 27 de setembro mediu outra árvore e não substitui estes dados.

## Comparação atual da CLI

Medianas em milissegundos; diferença e razão são medianas dos **pares**, não quociente ou subtração das medianas das colunas.

| Fixture | Tekt | Vanilla | Diferença pareada | Razão pareada | Leitura |
| --- | ---: | ---: | ---: | ---: | --- |
| `shapes` | 10,19 | 8,71 | +1,52 | 1,17× | Sem primeira seleção de fonte no trace vanilla. |
| `pipeline-single-font` | 184,66 | 127,17 | +55,98 | 1,44× | Primeiro catálogo durante layout. |
| `text-dense` | 217,68 | 130,68 | +88,10 | 1,67× | Catálogo em eval; trabalho adicional de texto/layout depois dele. |
| `image` | 7,15 | 7,89 | −0,73 | 0,91× | Sem texto; delta pequeno, não prova vantagem estável. |
| `context` | 187,82 | 130,88 | +57,17 | 1,44× | Catálogo durante eval. |

Na sonda com tracing, o timer Tekt da fase que contém o primeiro catálogo deu mediana de **174,07 ms** em layout para `pipeline-single-font` e **176,05/176,74 ms** em eval para `text-dense`/`context`. O trace vanilla registrou `discover fonts` aninhado nessas fases, entre aproximadamente **116 e 124 ms** nas três invocações de cada fixture. Os spans têm fronteiras diferentes: esta correspondência localiza o trabalho e sugere uma parcela dominante do delta, mas não mede o tempo exclusivo de `fontdb` no Tekt. Para `text-dense`, o Tekt também registrou medianas de **18,26 ms** em layout e **9,64 ms** em shape; a iteração paginada vanilla ficou perto de **3 ms** depois da descoberta. Essas fases não são estritamente equivalentes, de modo que o residual de texto precisa de perfil pareado.

## Mapa transversal do L1

| Domínio do L1 | Correspondente vanilla pinado | Diferença relevante | Prioridade de desempenho |
| --- | --- | --- | --- |
| `contracts` e `entities/world_types` | `typst-library::World`, `Engine` | Ambos expõem World e valores de domínio; vanilla exige cache interno no World e usa `LazyHash`/tracking, Tekt usa contratos próprios e `SystemWorld` sob demanda. | Alta para primeira fonte e compilação repetida; o scan é L3. |
| `entities/content`, estilos, valores e AST | `typst-library::foundations`, `typst-syntax` | Tekt usa enum fechado com `Arc<[Content]>` para sequências; vanilla usa modelo de `Content`/elementos e memoização em vários consumers. Igualdade Rust e forma da árvore não são critério de paridade de linguagem. | Média, dependente de contagem de alocações/travessias reais. |
| Lexer e parser | `typst-syntax` | Organização diferente com scanner/parse próprios. Sem medição bilateral isolada do parser nesta árvore. | Baixa nos cinco casos atuais; em documentos grandes continua aberta. |
| Eval, closures e stdlib | `typst-eval`, `typst-library` | Tekt executa duas passagens do eval e monta `Scope` da stdlib dentro de cada passagem; vanilla tem `#[comemo::memoize]` no eval e `Library` construído separadamente. Closures Tekt partilham captura via `Arc`. | Média para documentos extensos, alta para recompilação incremental se houver repetição. |
| Introspecção e contexto | `typst-library::introspection`, `typst-realize` | Tekt constrói stores explícitos e faz estabilização própria; vanilla rastreia dependências e memoiza partes do pipeline. | Média em documentos com contexto/TOC; baixa no caso `context` curto após descontar a aquisição de fontes como hipótese. |
| Layout geral, texto e grid | `typst-layout` | Tekt mantém layouter e subframes próprios; grid remede células na emissão. Vanilla memoiza funções de layout/inline/flow. | Alta para tabelas/grids e texto denso; faltam fixtures pareadas que isolem esses casos. |
| Matemática e linguagem | `typst-layout::math`, `typst-library::math`, linguagem | Métodos e representação próprios, com callbacks para matemática no Tekt. Nenhuma razão end-to-end válida de matemática: assets NewCM e seleção de fonte diferiam na qualificação anterior. | Indeterminada, exige fonte idêntica e saída comparável. |

## Diferenças priorizadas e prova disponível

### 1. Aquisição de fontes e construção de cobertura, sobretudo L3

`SystemWorld::with_fonts_and_system` agenda o catálogo em um `OnceLock` ([world/fonts.rs:62](../../03_infra/src/world/fonts.rs)); o primeiro `book()` o materializa ([world/fonts.rs:104](../../03_infra/src/world/fonts.rs)). O Tekt usa `fontdb::Database::load_system_fonts` e constrói metadata e cobertura para cada face ([fontdb.rs:42](../../03_infra/src/fontdb.rs), [fontdb.rs:16](../../03_infra/src/fontdb.rs)). Seu extrator acumula codepoints de subtables Unicode e chama `Coverage::from_codepoints` ([fonts.rs:404](../../03_infra/src/fonts.rs), [font_book.rs:172](../../01_core/src/entities/font_book.rs)). O vanilla também percorre a cmap, mas constrói cobertura por subtable e faz união ([info.rs:119](../../lab/typst-original/crates/typst-library/src/text/font/info.rs)); portanto não é correto afirmar que só Tekt calcula cobertura. A [comparação de microbenchmarks de fontes](font-microbenchmarks-vanilla.md) encontrou o caminho público metadata+cobertura vanilla mais rápido nos inputs examinados, mas a dispersão impede converter isso em parcela exata da CLI.

**Inferência:** as diferenças de construção de metadata/cobertura e de composição do catálogo são candidatas fortes para parte do excedente de aproximadamente 56–57 ms nos casos de texto curto. Refutação: perfis ou um controle A/B de universo de fontes que deixem a diferença no L1 depois de igualar seleção e cobertura. A medição atual não separa I/O, parse, cmap, alocação e deduplicação. O `OnceLock` atual corrige a atribuição antiga do custo ao residual anterior ao pipeline; não remove o primeiro custo.

### 2. Medição repetida de células em grid, L1

O comentário e o fluxo em [grid.rs:382](../../01_core/src/compiler/layout/grid.rs) registram uma primeira medição com `layout_sub_frame`; a emissão chama `layout_sub_frame` novamente em [grid.rs:776](../../01_core/src/compiler/layout/grid.rs). A própria implementação estima custo próximo de duas passagens para células abrangidas. Isto é prova de trabalho repetido, **não** de que o documento inteiro fica 2× mais lento. A fixture `shapes` não exercita grid. Próximo controle: grid com células de texto e geometria idêntica, contagem de chamadas e perfil de CPU; só depois avaliar cache por índice de input com L0 atualizado primeiro.

### 3. Dupla avaliação e reconstrução da stdlib, L1

Em [eval/mod.rs:1219](../../01_core/src/compiler/eval/mod.rs), Tekt executa passagem pré-show e passagem renderizada; [eval/mod.rs:1259](../../01_core/src/compiler/eval/mod.rs) cria a stdlib dentro da closure de cada passagem, e [registry/mod.rs:25](../../01_core/src/compiler/stdlib/registry/mod.rs) registra seus módulos. O vanilla memoiza `eval` ([typst-eval/lib.rs:38](../../lab/typst-original/crates/typst-eval/src/lib.rs)) e usa `Library` já construída. **Inferência:** há custo fixo e de construção de Scope potencialmente evitável, sobretudo em recompilações. Os aproximadamente 5 ms de eval Tekt em `shapes` contra cerca de 1,3–1,5 ms de span eval vanilla incluem muito mais que stdlib; não isolam sua parcela. Testar contagem de montagens e tempo exclusivo da chamada atual antes de mudar contrato ou cache.

### 4. Memoização de eval/layout e estabilização, L1 e L3

O vanilla tem `#[comemo::memoize]` em eval, layout de páginas, fluxo, inline, shaping e outras unidades ([typst-eval/lib.rs:38](../../lab/typst-original/crates/typst-eval/src/lib.rs), [pages/mod.rs:53](../../lab/typst-original/crates/typst-layout/src/pages/mod.rs), [inline/mod.rs:70](../../lab/typst-original/crates/typst-layout/src/inline/mod.rs)). Tekt usa `comemo::track` em algumas interfaces e caches locais, mas o entrypoint paginado cria um `Layouter` por iteração e clona stores para convergência ([layout/mod.rs:2600](../../01_core/src/compiler/layout/mod.rs)); contexto adicional estabiliza em [context_stabilization.rs](../../03_infra/src/pipeline/context_stabilization.rs). **Inferência:** recompilações quentes e documentos com contexto/TOC podem perder reutilização. Contar passes e medir o mesmo `World` retido são pré-requisitos; a presença de anotações `memoize` no vanilla, isoladamente, não estabelece speedup em compilação fria.

### 5. Texto denso após primeiro catálogo

`text-dense` mantém 2 páginas e texto extraído idêntico nos PDFs inspecionados. A sonda atual tem delta pareado mediano de **88,10 ms**, maior que o dos dois casos de texto curto, enquanto layout/shape/export Tekt também crescem. Isso qualifica texto como segunda frente após o catálogo, mas não separa L1 de métricas/shaping em L3. O benchmark existente de pipeline aquece World e exclui setup e parsing ([benches/README.md](../../benches/README.md)); não deve ser subtraído da CLI. Um perfil de pilha pareado no mesmo input e fonte é a próxima prova útil.

## Ordem de investigação

1. Perfilar o primeiro `book()` e a carga de fontes nos dois binários, com a mesma seleção e cobertura. Executar controle A/B de `--font-path` e de universo `FONTCONFIG_FILE`, preservando páginas, texto, glifos e geometria. Isso decide quanto da diferença observada é L3.
2. Repetir `text-dense` com perfis de CPU e contadores de layout, métricas e shaping. Focar no custo adicional após o catálogo, sem somar fases com fronteiras diferentes.
3. Criar fixture pareada de grid/tabela e contar `layout_sub_frame` por célula; controlar geometria antes de modificar o algoritmo.
4. Em processo quente com `World` retido, medir passagens de eval, montagem da stdlib, estabilização e cache hits. Separar primeira compilação de recompilação.
5. Só depois examinar Content, parser e matemática com inputs representativos. As microbenchmarks existentes medem operações próprias do Tekt e não estabelecem razão bilateral ou causalidade end-to-end.

Qualquer otimização produtiva deve partir do Prompt L0 owner vigente, teste RED→GREEN quando pertinente e validação de linguagem contra o vanilla pinado. Mudanças de contrato público, comportamento padrão, fase ou compatibilidade exigem o gate humano da ADR-0127; as correções internas podem seguir o fluxo contínuo previsto ali.
