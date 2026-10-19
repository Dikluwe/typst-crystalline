from pathlib import Path
import gzip,json,statistics,hashlib,datetime
D=Path(__file__).resolve().parent
read=lambda n:json.load(gzip.open(D/n,'rt'))
old=read('measurements-exploratory.json.gz');ctx=read('measurements.json.gz');heap=read('retention.json.gz');counts=read('counts.json.gz');phases=read('phases.json.gz');qual=read('qualification.json.gz')
selected={'plain':old,'import':old,'context300':ctx}
comparisons=[c for c in old['comparison']if c['fixture']!='context300']+ctx['comparison']
assert len(comparisons)==108 and all(c['same_text'] and c['same_words'] and c['bbox_max_delta_pt']<=.000021 for c in comparisons)
assert all(c['comparison']['same_raster']and c['comparison']['same_words']and c['comparison']['same_text']and c['comparison']['same_page_size']for c in qual['cases'].values())
analysis={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'inputs':{n:hashlib.sha256((D/n).read_bytes()).hexdigest()for n in ['measurements-exploratory.json.gz','measurements.json.gz','retention.json.gz','counts.json.gz','phases.json.gz','qualification.json.gz']},'event_comparisons':108,'max_bbox_delta_pt':max(c['bbox_max_delta_pt']for c in comparisons),'summaries':{n:x['summary'][n]for n,x in selected.items()},'phase_medians':{c['name']:{k:statistics.median(s['timings'][k]for s in c['samples'])for k in c['samples'][0]['timings']}for c in phases['cases']}}
(D/'analysis.json').write_text(json.dumps(analysis,indent=2))
t='''# Item 6 — watch e contexto no release atual — 2026-10-10

## Conclusão

A investigação foi concluída, sem mudança produtiva. O watch reutiliza CST e catálogo; avaliação da raiz/import continua sendo chamada a cada evento aceito. O contexto com leitura de página reexecuta 300 contribuições três vezes por ciclo (900 entradas contadas); a confirmação de paginação é o próximo candidato de custo a investigar. O RSS maior não corresponde a crescimento do heap vivo neste recorte. Não há ganho novo de desempenho certificado nesta etapa.

## Equivalência antes de decidir

A primeira fixture contextual emitia `context [#i:#counter(page).get().first() ]`. O Tekt separou `130:` e `1` ao quebrar linha; vanilla conservou `130:1`. Os 36 ciclos contextuais desse ensaio não são equivalentes em fluxo e seus tempos não fundamentam comparação de desempenho equivalente. Ficaram preservados em measurements-exploratory.json.gz e measure-exploratory.py. A divergência do produto continua aberta.

A fixture qualificada emite uma string única: `context { str(i) + ":" + str(counter(page).get().first()) + " " }`. Ela conserva o resultado observável pretendido e a leitura de página, mas é outro caso de teste, não uma correção do compilador. Repetimos apenas esse caso, com 300/320 produtores. Seus 36 ciclos, somados aos 72 ciclos válidos plain/import, têm texto e palavras iguais; delta de caixa máximo 0,000020 pt, registrado sem arredondar a zero. Qualificação adicional das 12 combinações de documento/palavra/estrutura confirmou páginas/tamanhos, texto, palavras e raster 36 dpi iguais nos motores. Contexto mantém aquele pequeno delta de extração de caixa. Não fechar paridade geral de render com esse corpus.

## CPU e latência de publicação

Processos watch reais, um motor por vez; três rodadas, quatro condições, três amostras medidas por condição/motor/rodada (nove por condição). Primeiros quatro eventos aquecem cada processo e são excluídos. Ordem dos motores alterna por rodada. Mesma CPU, Rayon/jobs=1, fontes, epoch e stack 64 MiB. Não houve build/testes/debugger/interpositor simultâneo à medição nativa.

CPU soma schedstat das threads do watcher; latência é edição→PDF completo e inclui espera/polling/publicação. Não usar a latência como custo exclusivo de recompilar.

| Documento / edição | CPU Tekt / vanilla (ms) | Latência Tekt / vanilla (ms) | RSS Tekt / vanilla (KiB) |
|---|---:|---:|---:|
'''
for n,x in selected.items():
 for k in ['same_bytes','small','structural']:
  a=x['summary'][n][k]['tekt'];b=x['summary'][n][k]['vanilla']
  t+=f"| {n} / {k} | {a['cpu_median_ms']:.2f} / {b['cpu_median_ms']:.2f} | {a['latency_median_ms']:.2f} / {b['latency_median_ms']:.2f} | {a['rss_median_kib']:.0f} / {b['rss_median_kib']:.0f} |\n"
t+='''
Todos os eventos de escrita desses três tipos publicaram em ambos os motores. Sem evento, nenhum watcher publicou durante a janela de 0,8 s: CPU Tekt mediana 0,59/0,89/0,45 ms nos casos plain/import/context; vanilla marcou zero na resolução observada. Isso é atividade de espera, não recompilação. Metadados isolados são um eixo distinto: Tekt publicou 9/9 por caso; vanilla 0/9. O Tekt gasta CPU semelhante à recompilação completa nesse toque. Filtrar esse evento alteraria a política vigente de watch; não atribuir o delta a memoização nem mudar o default implicitamente.

Startup é registrado separadamente com três amostras, sem conclusão robusta de ganho. Os dados de memória nativos mostram estabilização após poucos ciclos, mas não são prova de ausência de crescimento em outros documentos ou em execuções longas.

## Reutilização contada separadamente

GDB executou somente seus próprios processos, fora dos tempos e do interpositor. Sandbox negou ptrace; o rerun com permissão específica produziu counts.json.gz. Entradas acumuladas initial→mtime→mesmos bytes→palavra→estrutura:

| Caso / entrada | Acumulados |
|---|---|
| import / Source::new | 2 / 2 / 2 / 3 / 4 |
| import / root eval | 1 / 2 / 3 / 4 / 5 |
| import / import eval | 1 / 2 / 3 / 4 / 5 |
| context / Source::new | 1 / 1 / 1 / 2 / 3 |
| context / root eval | 1 / 2 / 3 / 4 / 5 |
| context / Session::execute | 900 / 1800 / 2700 / 3600 / 4560 |
| ambos / font generation prepare | 1 / 1 / 1 / 1 / 1 |

A última expansão contextual tem 320 produtores e acrescenta 960 entradas. São contagens de entrada, não tempo exclusivo ou igualdade de algoritmo upstream. Os símbolos standalone introspect/content também foram sondados; inlining limita a interpretação como totais, e esses contadores não fundamentam custo de travessia. A sonda inicial de stabilize marcou zero; fonte/rota abaixo explica por que este caso de Page segue o caminho ordinário, não o orçamento da sessão selecionada. Vanilla é stripped; não inventar contagem equivalente.

## Custo por fases e rota de Page

Sonda adicional Tekt com --timings-json, seis edições de palavra após aquecimento por documento. Não juntar seus tempos ao benchmark nativo ou subtrair medianas para atribuição causal.

| Caso | eval_ms | expand_context_ms | layout_ms | shape_ms | total_ms |
|---|---:|---:|---:|---:|---:|
'''
for n,v in analysis['phase_medians'].items():t+=f"| {n} | {v['eval_ms']:.2f} | {v['expand_context_ms']:.2f} | {v['layout_ms']:.2f} | {v['shape_ms']:.2f} | {v['total_ms']:.2f} |\n"
t+='''
`pipeline/paged.rs:365–386` mede a preparação contextual; `:438–549` inclui no campo layout a realização de snapshots Page, reexpansão de contextos e relayout. Portanto cerca de 40 ms de layout_ms não são 40 ms exclusivos do layouter. `context_stabilization.rs:1202–1238` expande o caminho ordinário com Session nova/discover; `:304–318` executa os produtores. O loop paginado `paged.rs:450–549` confirma convergência depois de reexpansão/relayout. Isso sustenta investigar custo dessa confirmação, não autorização para apagar uma passagem.

O L0 vigente conserva identidade, capture, Location, estilos, target, features, callbacks e diagnóstico; não permite trocar implicitamente a rota selecionada. Page snapshot igual sozinho não prova igualdade de leituras de posição, queries, margens ou numeração. O próximo experimento deve manter a rota e medir a repetição/reconstrução dentro dela; qualquer proposta que reutilize contribuição exige validação causal completa, refutadores de here/page-numbering/counter updates/numeração/multipágina e L0 antes de código. Não usar igualdade Rust de Content, hash de closure ou vtable para certificar reuso. Memoização de imports/layout entre Worlds é um contrato separado de dependências/efeitos, ainda fora do lote syntax-only.

## Heap vivo e retenção

Sonda glibc diagnóstica em processos separados, 36 edições por motor, 37 checkpoints após publicar e estabilizar. O registro do interpositor ocupa 6 MiB de mmap, medido também por pagemap; não contamina os tempos nativos. Signal handler só lê contadores atômicos e escreve um buffer próprio; checkpoints com mutação concorrente são rejeitados por epoch. Não invoca malloc/printf/mallinfo/locks no handler. Selftest comprovou +200.000 bytes e retorno ao baseline após free. Zero registros perdidos e zero frees não rastreados em ambos os processos.

Tekt voltou a **3.177.620 bytes solicitados vivos (3,03 MiB)** em todos os 37 checkpoints; pico de pedidos simultaneamente vivos foi **21.436.917 bytes (20,44 MiB)**. Vanilla começou com 5.923.257 e terminou com 13.256.698 bytes vivos (12,64 MiB), estabilizando próximo desse patamar após aquecimento; pico 17.280.969 bytes (16,48 MiB). São somas de pedidos glibc vivos interceptados, não tamanho de todos os mapeamentos nem RSS. O estudo não identifica cada objeto/cache retido do vanilla.

RSS nativo contextual ficou aproximadamente 50.020 KiB Tekt / 41.884 KiB vanilla no controle de mesmos bytes. A diferença de RSS não pode ser atribuída integralmente a fontes, introspectors ou histórico: há páginas de executável/bibliotecas/mapeamentos, arena e fragmentação, além de objetos vivos. Os checkpoints Tekt descartam crescimento contínuo do heap interceptado neste recorte; não provam ausência de leaks em outros casos, não justificam malloc_trim como política e não fecham toda memória contextual.

## Proveniência e reprodução

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before/final.json.gz guardam status, diff HEAD --stat, UTC, hashes de todos os 966 Rust produtivos e de todos os L0/núcleos. Todos permanecem iguais. Release Tekt fd758cf69301c297f6123f042ab19cc2de41e38a7f4af7fc351987728fa18655; oracle pin 9f2b6e8715237cb086899a42873660fe744622e8 e binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea (árvore/receita P1489). Não inferir proveniência de versão.

Receipts conservam argv, fixtures/texto/hashes, ambiente, CPU, UTC, fontes/scripts/binários e logs. font-provenance registra 1.276 paths de fonte/configuração; 1.100 conferidos contra hashes anteriores, sem timestamps posteriores ao início. A coleta exploratory contém o controle contextual rejeitado e os dados válidos plain/import; measurements contém a repetição contextual equivalente. analysis.json guarda seleção e hashes dos inputs. count-initial.py reconstrói exatamente o script hashado da primeira sonda; count.py é a sonda complementar. Não houve mudança em código ou prompts, commit, reset ou resselo global.

Reproduzir congelando release em /tmp/tekt-watch-context/typst: measure.py (todas as fixtures atuais) ou --context-only, retention.py, count.py (ptrace de processos próprios), phases.py, qualify.py, font_provenance.py. Compilar heap_probe.c.txt com cc -shared -fPIC -O2 -std=c11 -Wall -Wextra -x c -o heap_probe.so antes de retention.py. Preservar os receipts antes de repetir, pois os scripts sobrescrevem saídas. Build dev e crystalline-lint passaram, com avisos herdados; não houve alteração produtiva que exigisse nova suíte funcional.
'''
t+=f"\nUTC benchmark inicial: {old['utc_start']} a {old['utc_end']}; repetição contextual: {ctx['utc_start']} a {ctx['utc_end']}.\n"
D.with_suffix('.md').write_text(t)
print('report written; valid comparisons',len(comparisons))
