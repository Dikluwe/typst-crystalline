# Perfil de CPU refeito após cmap — 2026-10-10

## Escopo e proveniência

Item 3: atualizar perfil de CPU antes de escolher outra correção. Nenhum consumer ou L0 produtivo alterado. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; before.json.gz guarda UTC, status/diffstat completos e hashes dos 966 consumers. Tekt atual SHA-256 `584d5c24c243d52c3f80999fe96241c01c98cf69c33eb4f3cb00144d5dc5a1ba` inclui deduplicação cmap aceita; tentativas de lifetime/parsing foram retiradas. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário ratificado `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, árvore/receita P1489. Ambos foram congelados em /tmp/tekt-cpu-rebaseline/{tekt,orig}/typst. Version string não serve como identidade.

Os recibos guardam argv, ambiente relevante, fixtures completas/hashes, binários, scripts, módulos ELF, fontes/hashes e todas as amostras. font-provenance.json.gz complementa as fontes explícitas: fontes instaladas/fontconfig, metadados e comparação com a captura do item anterior. Todos os mtimes/ctimes e identidades de links precedem o início da medição; hashes das entradas comuns ao recibo anterior coincidem. Não há prova contra alterações privilegiadas que forjem metadados, nem se pretende isso.

## Método e limites

perf stat task-clock foi recusado pelo host (perf_event_paranoid=4); nenhuma configuração do sistema foi mudada. Usada sonda diagnóstica LD_PRELOAD/ITIMER_PROF: intervalo1ms, PC interrompido de ucontext, backtrace de até64frames, limite8192 por processo. 24 processos por documento/engine, oito documentos, total384 processos. CPU fixa, RAYON1, vanilla --jobs1, stack64MiB. Autoteste selftest.json.gz obteve99/99 PCs na função busy esperada.

backtrace em signal handler não é async-signal-safe; a coleta é diagnóstica, tem timeout e checagem de entregas/capacidade. Os módulos são normalizados por base ELF e simbolizados com eu-addr2line; nomes Rust legacy com sufixo LLVM foram normalizados por c++filt, preservando nomes brutos e a receita. Nenhum endereço desconhecido do executável vanilla foi ocultado ou atribuído por proximidade sem prova.

O vanilla ratificado foi stripped. cargo rustc --release --locked --offline, com o SHA do pin explícito e apenas -Cstrip=none/--emit para arquivo auxiliar, produziu símbolos sem substituir o oráculo. symbol-build-recipe.json.gz guarda comando/compilador/manifests/fontes; symbol-donor.json.gz valida endereço/tamanho/flags/bytes de TODAS as seções alocáveis contra o ELF medido. A única diferença é .note.gnu.build-id: .text, .rodata, relocations, unwind e demais dados/code runtime são idênticos. O auxiliar é usado somente para nomes dos PCs do original; não foi usado no benchmark.

A classificação é presença em stack, por prioridade explícita: catálogo (coverage/discovery/metadata-parser/seletivo restante/composição/restante), export, shape, layout, eval residual, resíduo do wrapper PDF, outros. Não é duração exclusiva de funções. PCs em read/open/mmap/lseek/stat mostram o endereço de usuário na entrega; não certificam tempo exclusivo de kernel ou espera I/O. Funções inlined podem desaparecer; anchors inclusivos se sobrepõem e não se somam. Bucket metadata vanilla inclui coverage de FontInfo::from_ttf; não compará-lo como metadata pura com o helper lazy do Tekt. A nova instrumentação/classificação difere do perfil anterior: não usar diferenças percentuais antigas/novas como aceleração.

Há418 amostras truncadas em default-scopes Tekt; PC físico continua disponível, mas ancestralidade/resíduo fica limitada. Nos outros casos não houve truncamento. Dados e quality flags ficam em classification.json. Native benchmark separado, sem LD_PRELOAD: três rodadas × seis pares por caso; warmups excluídos. CPU de /usr/bin/time tem resolução de centésimos; não atribuir pequenas diferenças de fases a esse contador.

## Equivalência antes da comparação

qualification.json.gz: plain tem texto, páginas, caixas e raster36dpi iguais. default-scopes conserva fluxo/raster, com diferença numérica máxima0,000020pt de caixa. As quatro tabelas local/no-fill/inline conservam texto, palavras, páginas e caixas até0,000024pt, mas hashes de raster diferem: controle de fluxo textual, sem fechar paridade de render. families conserva palavras/páginas, mas muda disposição; variants conserva palavras/páginas, mas muda disposição/pesos. Estes dois casos diagnosticam o custo interno do Tekt e NÃO sustentam uma razão de desempenho para trabalho equivalente ao vanilla. Igualdade de bytes/Rust/PDF não é o critério de linguagem (ADR-0107); os controles mecânicos têm limites explícitos.

## CPU amostrada

Percentuais abaixo são categorias de stack, não tempos exclusivos. PDF agrega export e resíduo do wrapper PDF (neste último a cópia pode estar na resolução de recursos antes do exportador).

| Documento | Engine | Amostras | Catálogo | Eval residual | Layout/shape | PDF | Outros |
|---|---|---:|---:|---:|---:|---:|---:|
| plain | tekt | 2340 | 96.5% | 1.6% | 0.7% | 0.9% | 0.3% |
| plain | orig | 3016 | 96.9% | 0.8% | 0.8% | 0.8% | 0.6% |
| default-scopes | tekt | 3533 | 60.7% | 10.3% | 11.2% | 5.8% | 12.0% |
| default-scopes | orig | 3429 | 83.7% | 2.3% | 11.3% | 2.0% | 0.8% |
| local-set-400 | tekt | 2553 | 87.9% | 3.5% | 5.0% | 3.2% | 0.4% |
| local-set-400 | orig | 3233 | 88.1% | 1.4% | 8.7% | 1.1% | 0.7% |
| local-set-1600 | tekt | 3498 | 64.0% | 11.0% | 14.9% | 9.8% | 0.3% |
| local-set-1600 | orig | 4091 | 70.1% | 1.9% | 25.4% | 2.1% | 0.5% |
| no-fill-1600 | tekt | 2983 | 74.3% | 6.5% | 11.4% | 7.5% | 0.3% |
| no-fill-1600 | orig | 3974 | 71.7% | 1.9% | 23.7% | 2.2% | 0.5% |
| inline-set-1600 | tekt | 3999 | 56.0% | 21.0% | 13.4% | 9.4% | 0.3% |
| inline-set-1600 | orig | 4215 | 67.8% | 3.9% | 25.6% | 1.9% | 0.7% |
| families-1600 | tekt | 3485 | 64.3% | 14.0% | 14.1% | 7.2% | 0.4% |
| families-1600 | orig | 4192 | 68.1% | 4.3% | 24.5% | 2.3% | 0.7% |
| variants-1600 | tekt | 19051 | 11.7% | 2.4% | 13.2% | 72.3% | 0.4% |
| variants-1600 | orig | 5527 | 53.3% | 3.0% | 41.3% | 1.8% | 0.5% |

## Tempo/RSS sem sonda

Valores observados, não alegação geral de performance/paridade. orig = vanilla ratificado. famílias/variantes ficam fora de uma comparação de trabalho equivalente.

| Documento | Tekt ms | Vanilla ms | Tekt pico KiB | Vanilla pico KiB |
|---|---:|---:|---:|---:|
| plain | 100.5 | 129.4 | 27278 | 30038 |
| default-scopes | 149.4 | 146.8 | 53564 | 34244 |
| local-set-400 | 112.1 | 144.0 | 31150 | 34720 |
| local-set-1600 | 146.2 | 174.1 | 42292 | 46304 |
| no-fill-1600 | 133.3 | 172.1 | 38192 | 43452 |
| inline-set-1600 | 169.9 | 181.0 | 47766 | 48634 |
| families-1600 | 145.8 | 178.1 | 38760 | 46610 |
| variants-1600 | 789.7 | 228.8 | 293094 | 57214 |

Custo da instrumentação (mediana dos ratios pareados, nove pares por caso):

- plain-tekt: +5.7% de wall.
- plain-orig: +2.7% de wall.
- inline-set-1600-tekt: +5.8% de wall.
- inline-set-1600-orig: +3.9% de wall.
- variants-1600-tekt: +3.8% de wall.
- variants-1600-orig: +2.7% de wall.

Não usar tempos instrumentados como benchmark. Por exemplo, percentuais de stack de eval/layout diferentes entre engines não são prova de fase mais lenta: catálogo pode ser chamado por eval ou layout, e foi removido antes de classificar esses resíduos.

## Medição que orienta as próximas investidas

1. **Catálogo frio permanece o maior custo fixo.** Plain Tekt:2257/2340 amostras (96,5%) na aquisição; discovery908, seletivo restante1071, coverage160, metadata/parser57, composição26/resto35. O PC top aponta read/open/stat/lseek/mmap/munmap. O bucket seletivo também pode conter parsing inlined: não transformar57 amostras em custo total exato de Face::parse. Isso explica por que escolher outra microcorreção de parsing, sem medir o caminho projetado/I/O, é fraco. Fonte: fontdb.rs:26-43; fontdb/selective.rs:15-31,79-95; world/fonts.rs:183-229. Não extrapolar frio para watch com catálogo reutilizado.

2. **Variantes põem a preparação/export PDF no centro.** Tekt variants:10507/19051 (55,2%) no exportador,3259/19051 (17,1%) no resíduo do wrapper PDF.6070 amostras inclusivas contêm build_math_glyph_reverse_map (31,9%);1448 PCs físicos estão nessa função. O hash_one dominante é chamado pelo HashMap desse mapa reverso, não inferido como hashing do buffer da fonte. Anchors hash/reverse map se sobrepõem.2092 PCs memcmp aparecem no grupo export;2867 PCs memmove aparecem no resíduo do wrapper. Fonte: pipeline/pdf.rs:436-437, pipeline/resources.rs:195-209,269-281 (to_vec de fonte por combinação); export/builder/fonts.rs:901-911 (mapa reverso por recurso); font_metrics/math.rs:246-254 (cmap, glyph_index e HashMap::entry).

   font-resources.json.gz mediu800 recursos de fonte no PDF Tekt (401 nomes Regular,149 Semibold,250 Bold), três no vanilla. São três nomes PostScript, não prova automática de três buffers/identidades/instâncias equivalentes. PDF Tekt2.006.019bytes contra24.554bytes neste cenário. A multiplicidade e os hotspots apontam custo real interno; não copiar a contagem do vanilla como requisito. Os L0 de resources/font_binding/builder-fonts e o Núcleo shaped-face-identity exigem pedido completo e face correta. Não colapsar variantes/eixos/bindings para simular ganho.

3. **Inline mantém custo residual de árvores/estilos.**839/3999 (21,0%) amostras em eval residual. map_content aparece454 vezes inclusivas; StyleChain::collapse152, clone170, local_delta_since11. Não somar nem tratar esses anchors como duração exclusiva. with_defaults, collapse, linked syntax lookup e alloc/copy aparecem nos PCs/frames de eval residual. Isso orienta medir tamanho/quantidade de clones em células e travessias do enum; não justifica trocar por vtable nem repetir o cache de prefixo rejeitado.

### Sonda adicional de seleção, antes de escolher uma correção

font-selection-probe.json.gz usa SystemWorld real e bibliotecas release registradas, mesma família e800 pesos100..899. Selecionou slots0/4/1 (Regular/Semibold/Bold), com401/149/250 pedidos, coincidentes com os nomes/contagens dos recursos PDF. As três faces são não variáveis e não possuem MATH nem variantes MATH. O helper atual varre cmap em font_metrics/math.rs:239-254 ANTES dos loops de extract_variants/assembly: esses helpers retornam vazio quando falta MATH; o mapa final é vazio.

Essa é uma hipótese interna mais direta do que outra representação ou cache: deixar de construir o mapa intermediário em fontes sem MATH. O perfil localiza o custo; ainda não foi medido um patch nem autorizado ganho por porcentagem inclusiva.

A mesma sonda somou252.121.292bytes de payload de fonte nos800 pedidos (aproximadamente240,44MiB se cada pedido fizer uma cópia completa). Trata-se de soma de comprimentos + a fórmula do to_vec observado em resources.rs:281, não medição de heap/RSS ou prova de todo pico. O alias público FontKey carrega Vec, e a multiplicidade confirma a relevância de medir cópias antes de alterar seu contrato.

## Próximos experimentos, sem nova arquitetura decidida

- Primeiro: retorno antecipado do mapa reverso quando falta tabela MATH. A fonte constrói cmap_reverse antes de consultar variantes/assemblies, cujos helpers devolvem vazio sem MATH. Confirmar RED de trabalho/alocações e GREEN mantendo vazio sem MATH, operadores/assembly/ToUnicode reais com MATH. Depois medir payloads cmap repetidos/hashing nas faces que realmente possuem MATH, antes de escolher hasher, representação ou cache. Preservar primeiro codepoint real, operadores/assembly e ToUnicode dos L0 de math/font_metrics e builder/fonts. O hashing dominante de map_reverse usa chaves inteiras de glifo: medir esse mapa antes de atribuir o custo a hashing dos bytes de fonte. O mapa percorre todas as subtabelas hoje; não aplicar cegamente o filtro Unicode do adapter de coverage.
- Depois: atribuir volume copiado por resolve_fonts e igualdade de binding por fonte/requisição. Qualquer representação proposta precisa ler os L0 e tratar o alias público FontKey (render.rs:35) e o gate ADR-0127; esse relatório não autoriza alteração de contrato.
- Frente dos documentos de estilo: contagem de clones/alocações em map_content_pre e table_cells::with_defaults com documento grande, controles de língua e snapshot. Manter enum fechado e despacho estático. Corrigir equivalência de families/variants antes de usar suas razões contra vanilla para concluir paridade de desempenho.

Nenhuma dessas hipóteses foi materializada neste item. O resultado é um perfil atual e prioridades justificadas, não promessa de ganho por soma de percentuais.

Recibos: 73119 amostras classificadas; 1276 caminhos de fontes/configs, 1100 hashes comparados com a captura anterior; auxiliar de símbolos SHA-256`b355bdc07703a5265aa092a63bcdf0d24787b33c9aca0f747663ba8a07ca7b14`.

Verificação final: 966 consumers produtivos e ambas as árvores de fonte examinadas permaneceram iguais aos snapshots; binários ratificados preservaram seus SHA-256. crystalline-lint . terminou com exit0 e warnings históricos. Nenhum teste de lógica nova foi necessário, pois este item só produziu diagnósticos. final.json.gz guarda os hashes e logs de validação.
