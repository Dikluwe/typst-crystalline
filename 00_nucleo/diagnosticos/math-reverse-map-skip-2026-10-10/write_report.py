from pathlib import Path
import gzip,json,hashlib,datetime
D=Path(__file__).resolve().parent
read=lambda n:json.load(gzip.open(D/n,'rt'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
x=read('measurements-risk-repeat.json.gz');q=read('qualification-tables.json.gz');a=read('allocation-equal-red.json.gz');g=read('allocation-equal-green.json.gz')
assert g['passed']
assert all(c['same_text']and c['same_words']and c['same_page_size']and c['same_raster']and c['bbox_max_delta_pt']==0 for c in q['across'].values())
assert all(c['before']['stderr']==c['after']['stderr']for c in q['cases'].values())
oracle={}
for n,c in q['cases'].items():
 u,v=c['after'],c['vanilla'];wu=[w for p in u['pages']for w in p['words']];wv=[w for p in v['pages']for w in p['words']];same=[w['text']for w in wu]==[w['text']for w in wv]
 oracle[n]={'same_words':same,'same_layout_text':u['layout_text_sha256']==v['layout_text_sha256'],'same_page_sizes':[p['size']for p in u['pages']]==[p['size']for p in v['pages']],'bbox_max_delta_pt':max((abs(i['box'][k]-j['box'][k])for i,j in zip(wu,wv)for k in ['xMin','xMax','yMin','yMax']),default=0)if same else None,'same_raster':u['raster_sha256']==v['raster_sha256']}
(D/'oracle-comparison.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'input_sha256':sha(D/'qualification-tables.json.gz'),'cases':oracle},indent=2))
t='''# Variantes: evitar reverse cmap sem MATH variants — 2026-10-10

## Correção mantida

`build_math_glyph_reverse_map` agora retorna mapa vazio antes de percorrer a cmap quando MATH ou seu campo variants está ausente. Os quatro extractors já retornavam vazio nesses casos; o mapa intermediário era descartado. Com variants presente, o algoritmo permanece integral: codepoints reais, first-wins para glifos compartilhados, fallback de variantes/assemblies e operadores grandes.

L0 atualizado primeiro, testes semânticos escritos antes de implementação e resselo restrito. Nenhuma mudança em FontKey, seleção, pesos, axes, identidade, binding, fallback ou despacho enum. Testes semânticos negativos já passavam no baseline; o RED→GREEN é de recurso, com limite pré-código de 1.116.479 alocações em variants-1600, frente ao baseline 1.117.479. O baseline realmente falhou nesse limite e o candidato passou.

## Alocações

Variants-1600 caiu de **1.117.479 para 1.108.679 pedidos glibc**: menos 8.800. Bytes solicitados caíram 1.535.035.332→1.475.940.932, redução de **59.094.400 bytes (56,36 MiB)**. Families/no-fill de 1.600 caíram 11 pedidos e 73.868 bytes cada. Três repetições, interpositor malloc/calloc/realloc/memalign. São pedidos e bytes acumulados, não heap vivo nem RSS; tempo com sonda não fundamenta desempenho.

## Benchmark nativo

Três rodadas de oito pares por fixture, warmup excluído, antes/candidato/oracle na mesma janela, ordem alternada, CPU fixa, Rayon/jobs=1 e stack 64 MiB. Razão de tempo é mediana dos pares; diferenças entre medianas não substituem a razão pareada.

| Caso | Antes (ms) | Depois (ms) | Razão pareada | RSS antes/depois (KiB) |
|---|---:|---:|---:|---:|
'''
for n,c in x['summary'].items():
 t+=f"| {n} | {c['before']['wall_median_ms']:.2f} | {c['after']['wall_median_ms']:.2f} | {c['after_before']['wall_ratio_median']*100:+.2f}% | {c['before']['rss_median_kib']:.0f}/{c['after']['rss_median_kib']:.0f} |\n"
t+='''
O ganho de variantes foi cerca de 30% nas três rodadas (−30,28%, −30,31%, −29,80%). Families e local têm sinais mistos, compatíveis com pouco efeito nessa janela. RSS de variantes permanece próximo de 286 MiB: não houve redução relevante de memória residente. Manter a saída antecipada pelo ganho forte de tempo e redução do tráfego de alocação, com observáveis preservados.

## Paridade e trabalho restante

Os 11 controles mantiveram texto, palavras, páginas, tamanhos, caixas, raster a 36 dpi e warnings exatamente iguais entre Tekt antes/depois, incluindo MATH large/stretch/mixed. Os testes existentes de MATH verificam Unicode real de assembly, fallback de extensores/variantes e operadores grandes; fontes sem MATH e MATH sem variants têm testes novos. Toda a suíte infra passou: 1.285 testes, dois ignorados. Cargo build debug/release e crystalline-lint passaram; avisos históricos permanecem.

O corpus variants-1600 **continua sem equivalência de layout com vanilla**: texto em palavras e páginas coincidem, mas layout-text e raster diferem, com delta de caixa de 4,290024 pt. Families tem delta 531,75 pt. Tempos do oracle nesses dois casos são diagnóstico de cargas distintas, não razão de desempenho equivalente. Nos três controles matemáticos, raster coincide com oracle; pequenos deltas de caixa extraída de 0,000010–0,000030 pt foram preservados, sem arredondar para zero.

Font-resources confirma que Tekt antes/depois mantém 800 recursos no corpus de pesos (401 Regular, 149 Semibold, 250 Bold), PDF com 2.006.019 bytes; vanilla tem três recursos e PDF 24.554 bytes. Estes números de estrutura não são requisitos de linguagem. A correção não resolve a retenção dos buffers de fonte por pedido. A próxima investigação nesse eixo é compartilhar bytes da face física, preservando a identidade completa de cada pedido e ToUnicode/binding; não trocar o contrato público apenas para reduzir a contagem de recursos.

## Proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before/final.json.gz registram status e diff HEAD --stat completos, UTC, hashes dos 966 arquivos produtivos e conteúdo integral dos owners. Somente 03_infra/src/font_metrics/math.rs mudou; a correção anterior de Cell e todas as outras fontes foram preservadas.

- Baseline Tekt: 71e968c20d5ba32c652b2b66c4fd444c672158ba9bb27b0272f59cfaa5530c78.
- Candidato: fd758cf69301c297f6123f042ab19cc2de41e38a7f4af7fc351987728fa18655.
- Oracle pin 9f2b6e8715237cb086899a42873660fe744622e8, binary SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Árvore/receita P1489; versão pública não prova proveniência.

Receipts allocation-equal-red/green, qualification-tables, font-resources, measurements-risk-repeat incluem argv, ambiente, fixtures, CPU, stack e hashes de scripts/binários. font-provenance guarda 1.276 paths de fontes/configuração, 1.100 confrontados com hashes anteriores, sem timestamps posteriores ao início da medição. oracle-comparison deriva comparações do receipt de qualificação com seu hash. Patchs dos owners, fontes e logs de testes/build/lint estão preservados; nenhuma modificação em lab/oracle, commit, reset ou reparo global.

## Reprodução

Congelar target/release/typst antes/depois em /tmp/tekt-math-map/old/new/typst. Executar allocation-equal-argv.py red (falha esperada)/green, qualify.py, font_provenance.py, font_resources.py e measure-risk-repeat.py. Scripts na pasta deste diagnóstico; novas execuções sobrescrevem recibos, por isso preservar a medição anterior antes de repetir. Testes: cargo test -p typst-infra --lib -- --test-threads=1; cargo build; cargo build --release; crystalline-lint .
'''
t+=f"\nBenchmark UTC: {x['utc_start']} a {x['utc_end']}.\n"
D.with_suffix('.md').write_text(t)
