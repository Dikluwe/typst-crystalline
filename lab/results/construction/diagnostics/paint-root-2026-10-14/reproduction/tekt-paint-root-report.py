from pathlib import Path
import gzip,json,hashlib,datetime,base64,difflib
out=Path('00_nucleo/diagnosticos/paint-root-2026-10-14');pkg=Path('00_nucleo/materialization/tiling-transporte-2026-10-14')
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(name):
 with gzip.open(out/name,'rt') as f:return json.load(f)
pre=read('before-source.json.gz');extra=read('before-extra-test-owners.json.gz');final=read('candidate-final.json.gz')
checks=json.loads((out/'followup-summary.json').read_text());suite=json.loads((out/'workspace-result.json').read_text())
assert suite['exit']==0 and suite['sources_unchanged'];assert all(sha(n)==v['sha256'] for n,v in final['files'].items())
assert sha(pkg/'manifest.json')=='d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d'
before=dict(pre['files']);before.update(extra['files'])
(out/'current.patch').write_text(''.join(''.join(difflib.unified_diff((before.get(n) or {'text':''})['text'].splitlines(True),v['text'].splitlines(True),fromfile=n if n in before else '/dev/null',tofile=n)) for n,v in final['files'].items()))
repro=out/'reproduction';repro.mkdir(exist_ok=True)
for p in Path('/tmp').glob('tekt-paint-root-*'):
 if p.suffix in ['.py','.txt']:(repro/p.name).write_bytes(p.read_bytes())
for name in ['Cargo.toml','Cargo.lock','src/main.rs']:
 p=Path('/tmp/tekt-group-syntax')/name
 q=repro/'group-syntax'/name;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(p.read_bytes())
(repro/'README.txt').write_text('Scripts históricos; inventory/l0/migrate/implement/test-observers não são idempotentes: não reaplicar. current.patch combina o snapshot inicial com o snapshot posterior dos owners columns/placement antes de sua primeira edição; os UTCs são distintos e preservados. O parser syn expandiu matches! como expressão, produzindo inicialmente paint_root:false em duas expectativas de tiling; retificadas para true. Testes planos passaram a projetar a geometria recursiva sem mudar tolerâncias. T1b permanece incompleto; a regressão visual de Block-auto PDF está aberta.\n')
entries=[]
for p in sorted(out.rglob('*'))+sorted(pkg.rglob('*')):
 if p.is_file() and p.name!='evidence.json.gz':entries.append(dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:r=json.load(f)
assert all(hashlib.sha256(base64.b64decode(x['base64'])).hexdigest()==x['sha256'] for x in r['entries'])
report=f'''# Raízes de pintura — integração T1b incompleta — 2026-10-14

O campo aprovado `FrameItem::Group.paint_root` foi materializado e transportado
pelos construtores, reconstruções e consumidores internos. Box não fracionário,
Block em uma região e células do tiling legado recebem marcadores de raiz;
grupos técnicos herdam a raiz. O fundo do próprio container fica fora da nova
raiz dos descendentes. Esta etapa não conclui T1 nem certifica produção:
**há uma regressão PDF medida em gradiente parent de Block automático**.
Shape ainda usa Paint/Stroke; a migração desses campos é T1c, seguida de
recursos nativos em T2 e geometria em T3. Todos já aprovados; sem nova pergunta.

## Proveniência e autorização

HEAD `{pre['head'].strip()}`, working tree não commitado. before-source,
before-extra-test-owners, candidate-final, pre-validation e workspace-pre
conservam UTC, fontes/hashes e git diff HEAD --stat. Columns/placement foram
capturados mais tarde, antes de sua primeira edição; o patch reúne esses
snapshots sem lhes atribuir o mesmo instante. Aprovação humana do manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`;
pacote original e recibo preservados. Não houve commit, push ou publicação.

Oracle: pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário
lab/typst-original/target/release/typst SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
As sondas before/after-first conservam fontes, comandos, UTC, estado e hashes
dos binários. Imagens comparam o canvas inteiro de 120×96 pt em 240×192 pixels,
sem recorte ou alinhamento. Rsvg usa 72 dpi e zoom 2; PDFs passam por qpdf.

## Resultados e limites

Quatro testes novos cobrem body/backing de Box e Block com/sem clipping,
transformação que herda a raiz e transporte por translação/slicing.
O RED inicial inclui API ausente e imports de fixture; o RED de comportamento
posterior distingue ausência de raiz da página extra já existente em Block.
Os observadores históricos de testes foram adaptados à descida pelos grupos,
com composição de matrizes e coordenadas; expectativas/tolerâncias de geometria
não foram relaxadas. O lote layout terminou com 1174 passados e 3 ignorados.

A investigação da falha Block encontrou subtração dupla da margem inferior:
o limite local de altura explícita omitia a margem que page_bottom_limit
subtrai. A fórmula agora inclui essa margem antes de restringir ao limite
físico externo. Nos dois controles clip:false/true, o Tekt passou de duas
páginas (primeira vazia) para uma, como o vanilla. Registros em
block-pages-before e block-pages-after-first distinguem o binário anterior
da árvore que já tinha edições ainda não compiladas.

Das 36 renderizações Tekt dos 18 controles SVG/PDF, 35 preservaram os pixels
da entrada. A exceção **block-auto-gradient-parent PDF piorou**: pixels com
diferença >16 contra o pin passaram de 2304 para 3792; máximo de 79 para 106.
Entre os próprios renders Tekt, 3840 pixels mudaram acima desse limiar,
máximo 30. first-render-delta.json registra essa falha, não uma aprovação.
SVG inalterado não comprova paridade de Parent.

A leitura local do SVG pinado mede root automático 54×38 para body40×24 com
inset7, enquanto o Block atual usa a largura disponível. A fonte pinada
flow/block.rs:145–166,281–290,339–363 desativa expansão horizontal para Auto;
a geometria final precisa resultar do fluxo. O exporter legado também
substitui parent bbox em cada grupo, sem respeitar paint_root, e suas
coordenadas PDF de gradientes não podem ser rebaseadas indiscriminadamente.
A correção exige dimensões finais e contexto afim de raiz coerentes, com
controles técnicos/nested/clip e preservação das divergências Tekt justificadas.
Não tratar o controle regressivo como aceito por passar nos testes de layout.

## Validação

Suíte completa: **{suite['passed']} passaram, {suite['failed']} falharam,
{suite['ignored']} ignorados**, de {suite['start']} a {suite['end']}.
Hashes de fontes ficaram estáveis durante essa execução. O ajuste posterior
exclusivamente no observador cfg(test) tem estado e execução própria em
observer-cleanup; seus quatro testes passaram. suite-state-delta confirma que
somente esse owner cfg(test) e seu L0 mudaram após a suíte; a execução completa
não é atribuída retroativamente a essa edição.
Build, rustdoc -D warnings, all-target check, fmt, diff check,
V1/V5/V15/V26 estritos e reverse lineage passaram. Resselo limitado ao
manifesto de owners; benchmark fixture não produtiva atualizada separadamente.
Clippy mantém {checks['clippy_after']} avisos (inventário --cap-lints warn,
não aprovação do gate CI -D warnings). Linter completo: zero erros,
{checks['architecture'].get('warning',0)} warnings e {checks['architecture'].get('note',0)} notes.

## Continuidade

Prioridade: resolver a dimensão Auto e referência afim de pintura; completar
Shape.fill/stroke com FramePaint/FrameStroke e preparar células uma vez;
recursos PDF/SVG/PNG nativos, fill e stroke, clipping, recursos recursivos e
metadados decorativos. Box fracionário e Block fragmentado ainda não têm
raízes finais completas. Outset/overflow e outros defeitos geométricos
continuam abertos; não foram encerrados pela correção de margem.
CI remoto/release e o objetivo global de correções permanecem abertos.

Nesta etapa há defeitos concretos de implementação e integração de contrato.
Não há levantamento estatístico suficiente para dizer que a maioria das
falhas de todo o projeto está no código ou nos prompts.

## Evidência preservada

[evidence.json.gz](paint-root-2026-10-14/evidence.json.gz): {len(entries)} entradas
com caminho, SHA-256 e base64, relidas e verificadas; inclui fontes antes/depois,
patch, testes, sondas, PDFs/SVGs/PNGs, logs e scripts históricos.
SHA-256: `{sha(out/'evidence.json.gz')}`.
'''
Path('00_nucleo/diagnosticos/paint-root-2026-10-14.md').write_text(report)
print(json.dumps(dict(entries=len(entries),sha256=sha(out/'evidence.json.gz'),suite=suite,checks=checks),indent=2))
