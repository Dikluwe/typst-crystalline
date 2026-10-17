from common import *
c=json.loads((D/'classification.json').read_text());n=json.load(gzip.open(D/'native-benchmark.json.gz','rt'));q=json.load(gzip.open(D/'qualification.json.gz','rt'));over=json.load(gzip.open(D/'overhead.json.gz','rt'));donor=json.load(gzip.open(D/'symbol-donor.json.gz','rt'));assets=json.load(gzip.open(D/'font-provenance.json.gz','rt'))
s='''# Perfil de CPU refeito após cmap — 2026-10-10

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
'''
for f in F:
 for e in ['tekt','orig']:
  z=c['summary'][e+'/'+f.stem];p=z['partition'];total=z['samples'];catalog=sum(v for k,v in p.items()if k.startswith('catalog.'));lay=p.get('layout',0)+p.get('shape',0);pdf=p.get('export',0)+p.get('pdf_outer_residual',0)
  s+=f"| {f.stem} | {e} | {total} | {catalog/total*100:.1f}% | {p.get('eval_residual',0)/total*100:.1f}% | {lay/total*100:.1f}% | {pdf/total*100:.1f}% | {p.get('other',0)/total*100:.1f}% |\n"
s+='''
## Tempo/RSS sem sonda

Valores observados, não alegação geral de performance/paridade. orig = vanilla ratificado. famílias/variantes ficam fora de uma comparação de trabalho equivalente.

| Documento | Tekt ms | Vanilla ms | Tekt pico KiB | Vanilla pico KiB |
|---|---:|---:|---:|---:|
'''
for f in F:
 z=n['summary'][f.stem];a,b=z['tekt'],z['orig'];s+=f"| {f.stem} | {a['wall_median_ms']:.1f} | {b['wall_median_ms']:.1f} | {a['rss_median_kib']:g} | {b['rss_median_kib']:g} |\n"
s+='\nCusto da instrumentação (mediana dos ratios pareados, nove pares por caso):\n\n'
for k,v in over['summary'].items():s+=f"- {k}: {v['wall_delta_paired_median']*100:+.1f}% de wall.\n"
s+='''
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
'''
s+=f"\nRecibos: {sum(z['samples']for z in c['summary'].values())} amostras classificadas; {len(assets['fonts_and_configs'])} caminhos de fontes/configs, {assets['prior_assets_compared']} hashes comparados com a captura anterior; auxiliar de símbolos SHA-256`{donor['donor_sha256']}`.\n"
s+='\nVerificação final: 966 consumers produtivos e ambas as árvores de fonte examinadas permaneceram iguais aos snapshots; binários ratificados preservaram seus SHA-256. crystalline-lint . terminou com exit0 e warnings históricos. Nenhum teste de lógica nova foi necessário, pois este item só produziu diagnósticos. final.json.gz guarda os hashes e logs de validação.\n'
(D.with_suffix('.md')).write_text(s)
print('report saved',D.with_suffix('.md'))
