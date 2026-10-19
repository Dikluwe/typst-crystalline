from pathlib import Path
import json,gzip,hashlib,datetime,base64,difflib
out=Path('00_nucleo/diagnosticos/place-fluxo-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(n):
 with gzip.open(out/n,'rt') as f:return json.load(f)
def compare(folder):
 rows=read(folder+'/results.json.gz')['rows'];pairs={}
 for r in rows:pairs.setdefault(r['case'],{})[r['binary']]=r
 return dict(cases=len(pairs),bbox_matches=sum(v['tekt'].get('blue_bbox')==v['vanilla'].get('blue_bbox') for v in pairs.values()),differences={n:{k:v.get('blue_bbox') for k,v in pair.items()} for n,pair in pairs.items() if pair['tekt'].get('blue_bbox')!=pair['vanilla'].get('blue_bbox')})
before=read('before-source.json.gz');final=read('candidate-final.json.gz');checks=json.loads((out/'followup-summary.json').read_text());suite=json.loads((out/'workspace-result.json').read_text());assert suite['exit']==0 and suite['sources_unchanged'];assert all(sha(n)==v['sha256'] for n,v in final['files'].items());assert checks['clippy_after']==181 and checks['architecture']=={'warning':382,'note':1627}
cmp={n:compare(n) for n in ['before','after','pdf-final','block-regions-before']};(out/'comparison.json').write_text(json.dumps(cmp,indent=2))
(out/'current.patch').write_text(''.join(''.join(difflib.unified_diff(before['files'].get(n,{}).get('text','').splitlines(True),v['text'].splitlines(True),fromfile=n,tofile=n)) for n,v in final['files'].items()))
repro=out/'reproduction';repro.mkdir(exist_ok=True)
for name in ['probe','setup','implement','seal','validate','suite','pdf','report']:
 p=Path('/tmp/tekt-place-flow-'+name+'.py');(repro/p.name).write_bytes(p.read_bytes())
p=Path('/tmp/tekt-place-block-region-probe.py');(repro/p.name).write_bytes(p.read_bytes())
(repro/'README.txt').write_text('Scripts históricos, não idempotentes. NÃO reaplicar setup/implement/seal à árvore final. current.patch e candidate-final.json.gz conservam o estado do lote; a adição dos testes RED e o reforço do teste de checkpoint estão no patch. As sondas criam diretórios novos e registram fontes/comandos/binários/UTC. Probes before/after precedem resselo; PDF-final usa o candidato selado. block-regions-before é diagnóstico da próxima etapa B2, sem implementação de região/clipping nesta etapa B1.\n')
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name!='evidence.json.gz':entries.append(dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:r=json.load(f)
assert all(hashlib.sha256(base64.b64decode(x['base64'])).hexdigest()==x['sha256'] for x in r['entries'])
text=f'''# Place: fronteira física após fluxo — etapa B1 — 2026-10-14

Corrigidas as âncoras de Place sem alinhamento vertical explícito depois de linhas,
formas, Block, Align, Grid, Raw e VSpace nos controles medidos. A preparação da
próxima baseline e as margens fracas não entram na posição. Os dois controles de
Box após Linebreak/Shape que haviam piorado na etapa A agora coincidem com o pin.
A conclusão é deste lote; Block/inset/clipping e assentamento tardio de linha
com Box/Transform continuam abertos, assim como as demais correções do objetivo.

## Proveniência

HEAD `{before['head'].strip()}`, working tree não commitado. before-source.json.gz,
red-state.json.gz, candidate-final.json.gz e workspace-pre.json.gz preservam
fontes, hashes, UTC e a cadeia de status/diffstat. Suíte completa:
{suite['start']} → {suite['end']}. Fontes inalteradas durante os testes e gates.
Não houve commit, push, PR, publicação ou release.

Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Os resultados dos probes guardam argv, fontes, stdout/stderr, UTC e hashes dos
binários. Imagens do corpus usam 4 px/pt. A string de versão não é prova do pin.

## Evidência e correção

place.rs reconhecia somente linha pendente. line_commit.rs já preparava a
próxima baseline, Shape agregava below/cap, e Block mantinha sua margem à parte.
Usar o cursor bruto deslocaria o problema: era necessário conservar o fim físico
que o produtor conhece. Fonte pinada flow/distribute.rs:703–715 ancora Place
pelo offset de fluxo quando o eixo vertical não é explícito. As decisões seguem
geometria da linguagem, não a estrutura de buffers do upstream.

O novo estado privado conserva `(fim físico, cursor preparado)`. O consumidor
aceita a evidência somente enquanto o cursor corresponde ao registrado; não
adivinha o fim de um produtor desconhecido. Subframe salva/limpa/restaura em LIFO,
checkpoint conserva os dois valores, e página/coluna/configuração nova os limpa.
Place não consome a fronteira do pai. Parbreak preserva o fim ao preparar spacing;
VSpace soma seu avanço físico. Não se varre o máximo de tinta: um overlay não
pode virar a âncora de outro overlay.

Os produtores registram o fim em seus próprios owners. Linha usa a baseline
estabilizada mais descent real; Shape exclui below; Grid usa o fim da row; Raw
usa end físico; Align usa o fim de seu body. Block usa a altura alocada quando
explícita, ou a fronteira causal do body quando auto, sem reescrever neste lote
o desenho/tamanho legado do container. Isso não fecha as divergências próprias
de dimensionamento de Block. Cursor alterado sem produtor certificado conserva
a rota legada e é pendência, não prova de paridade.

Foram atualizados 14 pares L0/consumer 1:1 antes do código e acrescentado o núcleo
place-flow-boundary, pin
`0e7c32f877924fea773be54f9719b43bb72b4c3bb065f6ff44a842c56683ff78`.
As mudanças são internas/de paridade sob ADR-0127: sem campo/assinatura pública,
modo, flag ou mudança de fase. O pacote público de tiling permanece pendente.

## Resultados

O corpus deliberadamente focado nas falhas contém 24 pares root/Box depois de 12
prefixos. Bboxes coincidentes: {cmp['before']['bbox_matches']}/24 antes e
{cmp['after']['bbox_matches']}/24 depois. As duas diferenças restantes são Rotate
na raiz e em Box; o marcador também sofre sobreposição visual. A integração com
a baseline ainda não assentada da linha permanece para B3. Esses números não
estimam a proporção de falhas de todo o projeto.

Nos 12 pares PDF finais, {cmp['pdf-final']['bbox_matches']} bboxes do marcador
coincidem. Os 24 PDFs passaram qpdf; não se exige igualdade de bytes. As evidências
registram a geometria e os resultados de cada comando, não certificação universal.

Cinco testes novos foram observados em RED e passaram em GREEN: linhas comitadas,
Shape, Block/Grid/VSpace, blocos textuais auto e controles de Top/overlay/página.
O teste de checkpoint existente agora também verifica certificado preservado
entre subframes, rejeição após mover o cursor sem evento e restauração pelo
checkpoint. Suíte de layout: 1166 passaram, 3 ignorados, antes do reforço final
do controle de checkpoint; a suíte completa valida o estado final.

Suíte completa: **{suite['passed']} passaram, {suite['failed']} falharam,
{suite['ignored']} ignorados**. Build offline/locked, rustdoc -D warnings,
all-target check, fmt e diff check passaram. V1/V5/V15/V26 estritos e reverse
lineage passaram, com resselo restrito ao manifesto. Inventário Clippy permanece
em 181 avisos; linter completo tem zero erros, 382 warnings e 1627 notes. Não é
aprovação do gate Clippy -D warnings nem zero violations. CI remoto/release e
as demais correções continuam abertos.

## Próxima etapa: Block, medida antes de propor a correção

block-regions-before contém 24 controles de inset, Bottom/Horizon, lados distintos,
aninhamento, texto, clipping, altura auto e precedente de bloco. Apenas
{cmp['block-regions-before']['bbox_matches']}/24 bboxes coincidem. Exemplos medidos:

- Inset 3 pt: Tekt y15 pt, vanilla18 pt; Bottom+Right: Tekt(71,37), vanilla(62,44).
- Com clip:true, o marcador desaparece. block.rs cria inner_h pelo avanço do
  body, que é zero para Place, apesar da altura explícita do container; também
  envolve filhos em coordenadas absolutas sem o rebase local correspondente.
- Altura auto e Bottom usam a altura da página/Box, enquanto vanilla mede o
  frame local. Não resolver isso somando uma constante de inset.

B2 precisa tratar a região do container, dimensões locais e recorte conjuntamente,
com medição antes de atualizar seus L0. B3 precisa resolver assentamento tardio
com inline Box/Transform e auditar os demais produtores de fluxo, incluindo
Heading/List/Stack/Equation/Columns. O objetivo completo permanece ativo.

## Evidências preservadas

[evidence.json.gz](place-fluxo-2026-10-14/evidence.json.gz) contém {len(entries)}
entradas com caminho, SHA-256 e bytes em base64, relidas e verificadas.
SHA-256 do arquivo: `{sha(out/'evidence.json.gz')}`.
Inclui estados, patch, logs RED/GREEN e gates, SARIF, SVG/PDF/PNG, resultados
bilaterais e scripts históricos. Scripts de mutação não devem ser reaplicados.
'''
out.with_suffix('.md').write_text(text)
print(json.dumps(dict(report=str(out.with_suffix('.md')),archive_entries=len(entries),archive_sha256=sha(out/'evidence.json.gz'),suite={k:suite[k] for k in ['passed','failed','ignored','sources_unchanged']},comparisons={k:{kk:vv for kk,vv in v.items() if kk!='differences'} for k,v in cmp.items()}),indent=2))
