from pathlib import Path
import gzip,json,hashlib,base64,datetime,subprocess,shutil
from PIL import Image,ImageChops
out=Path('00_nucleo/diagnosticos/tiling-preparacao-2026-10-14')
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
with gzip.open(out/'candidate-final.json.gz','rt') as f:candidate=json.load(f)
assert all(sha(p)==v['sha256'] for p,v in candidate['files'].items())
suite=json.loads((out/'workspace-result.json').read_text());assert suite['exit']==0 and suite['sources_unchanged']
checks=json.loads((out/'followup-validation-results.json').read_text());assert all(r['exit']==0 for r in checks)
summary=json.loads((out/'followup-summary.json').read_text())
rows={}
for phase in ['before-renders','after-renders']:
 with gzip.open(out/phase/'results.json.gz','rt') as f:r=json.load(f)
 rows[phase]={(x['case'],x['binary'],x['format']):x for x in r['rows']}
def raster(phase,case,binary,fmt):
 p=out/phase/(f'{case}-{binary}.'+fmt+('.png' if fmt!='png' else ''))
 im=Image.open(p).convert('RGBA');bg=Image.new('RGBA',im.size,'white');bg.alpha_composite(im);return bg.convert('RGB')
def compare(a,b):
 assert a.size==b.size,(a.size,b.size)
 diff=list(ImageChops.difference(a,b).getdata())
 return dict(size=a.size,pixels_gt16=sum(max(v)>16 for v in diff),maximum=max(map(max,diff)),exact=not any(max(v) for v in diff))
comparisons=[]
for key,after in rows['after-renders'].items():
 case,binary,fmt=key
 if binary!='tekt':continue
 before=rows['before-renders'][key]
 row=dict(case=case,format=fmt,before_exit=before['exit'],after_exit=after['exit'],before_stderr=before['stderr'],after_stderr=after['stderr'])
 if not before['exit'] and not after['exit']:
  row['preservation']=compare(raster('before-renders',*key),raster('after-renders',*key))
 for phase in rows:
  tekt=rows[phase][key];vanilla=rows[phase][case,'vanilla',fmt]
  if not tekt['exit'] and not vanilla['exit']:
   row[phase]=compare(raster(phase,*key),raster(phase,case,'vanilla',fmt))
 comparisons.append(row)
(out/'comparison.json').write_text(json.dumps(comparisons,indent=2))
for p in Path('/tmp').glob('tekt-prepare-cell-*.py'):shutil.copyfile(p,out/p.name)
for name in ['src/main.rs','Cargo.toml','Cargo.lock']:
 p=Path('/tmp/tekt-shape-syntax')/name;dest=out/'syntax-tool'/name;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dest)
with gzip.open(out/'final-patch.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),patch=subprocess.check_output(['git','diff','--','01_core/src/compiler/layout/tiling.rs','00_nucleo/prompts/compiler/layout/tiling.md'],text=True)),f)
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name not in ['evidence.json.gz','evidence.sha256']:
  data=p.read_bytes();entries.append(dict(path=str(p.relative_to(out)),sha256=hashlib.sha256(data).hexdigest(),base64=base64.b64encode(data).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:again=json.load(f)['entries']
assert len(again)==len(entries)
for e in again:assert hashlib.sha256(base64.b64decode(e['base64'])).hexdigest()==e['sha256']
evidence_sha=sha(out/'evidence.json.gz');(out/'evidence.sha256').write_text(evidence_sha+'  evidence.json.gz\n')
assert sha('00_nucleo/materialization/tiling-transporte-2026-10-14/manifest.json')=='d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d'
lines=[]
for x in comparisons:
 if 'preservation' in x and not x['preservation']['exact']:
  lines.append(f"- {x['case']} / {x['format']}: {x['before-renders']['pixels_gt16']} → {x['after-renders']['pixels_gt16']} pixels >16 contra o pin; máximo final {x['after-renders']['maximum']}.")
report=f'''# Tiling: preparação única da célula — 2026-10-14

O layout agora prepara um TilingFrame com uma única célula,
sem depender da área do alvo. Resolve size, spacing, offset, angle e relative,
conserva Color/Gradient integralmente e passa Image pelo layout uma vez.
A célula usa origem geométrica e referencial local explícito; erros causais
chegam ao documento e os mapas de labels/posições e transações do pai são
restaurados. O contrato público aprovado e seu manifesto permanecem intactos.

**Estado intermediário de T1c:** o adaptador materialize_fill consome a célula
preparada, mas ainda cria a grade legada de Groups. Shape.fill/stroke ainda
usam Paint/Stroke, o limite10000 permanece e stroke ainda não usa a célula.
A preparação não encerra T1c/T2/T3 e não certifica produção nem CI/release.

## Proveniência e testes

HEAD `{candidate['head'].strip()}`, working tree não commitado. before,
candidate-final, workspace-pre e pre-validation guardam UTC, fontes, hashes,
status e diffstat. Antes de reconstruir, o binário Tekt da sonda foi comprovado
idêntico ao raster-mascaras/after-final: SHA256
8f0f4f1a6dd04ba797db7d04e0d963448151274bf59401f87930bb5b7cb947c8.
O pin vanilla é 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489; SHA256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Sondas conservam fontes, argv, códigos de saída, stderr e hashes.

RED inicial: os casos de diagnóstico, ponte Color/Gradient e referencial local
falharam com a implementação anterior. fixture-compile.log registra um erro
anterior do próprio teste (Rect usa w/h), excluído do RED comportamental.
A revisão encontrou que o checkpoint de fluxo apaga seu próprio envelope;
isolation-red.log reproduz a perda da transação externa. O reparo suspende e
restaura pending/active/rejected/checkpoint/replay/depth do pai. Testes adicionais
cobrem auto, empty, placement, Image uma vez, labels locais, erros anteriores
preservados e rejeição antes de executar body. Dez testes foram acrescentados.

Suíte final: **{suite['passed']} passaram, {suite['failed']} falharam, {suite['ignored']} ignorados**,
UTC {suite['start']} → {suite['end']}; fontes verificadas inalteradas durante execução.
Build, rustdoc com -D warnings, check all-targets, fmt, diff check, linhagem
V1/V5/V15/V26 estrita e reverse lineage passaram. Clippy é inventário, não gate
-D warnings: {summary['clippy_before']} → {summary['clippy_after']}, delta por código
{summary['clippy_delta']}. Arquitetura completa: {summary['architecture']}.
Não há alegação de CI remoto verde ou de publicação.

Após a suíte e as sondas, Clippy apontou type_complexity na declaração da
matriz de casos de teste. A única alteração posterior é um alias desse tipo,
mais hashes de linhagem: test-alias-audit.json verifica por igualdade exata
do restante do arquivo e do L0. before-test-alias/candidate-final conserva
o código da suíte/sondas; candidate-final conserva a árvore final. Os dez
testes focados foram executados novamente e build/doc/check/fmt/Clippy/
linhagem foram revalidados. Não se atribui uma nova execução da suíte inteira
ao alias; a implementação produtiva é a mesma que passou a suíte completa.

## Sondas e limites observados

Canvas inteiro 480×280, PNG explícito --ppi144; SVG librsvg72dpi/zoom2;
PDF Poppler144dpi. Sem recorte, realinhamento ou aumento de tolerância.
O limiar >16 é métrica descritiva do delta, não critério suficiente de aceitação.
A primeira chamada pdftoppm usou argumento inválido -scale-to -1; os arquivos
parciais permanecem em before-renders-interrupted e não entram na comparação.
Todos os PDFs emitidos com sucesso têm o retorno de qpdf --check registrado.

{chr(10).join(lines) or 'Os renders válidos do Tekt permaneceram idênticos na amostra.'}

Auto vazio passa a falhar com `tile size must be non-zero` e hint explícito,
em vez de emitir preenchimento preto. Pitch zero passa a produzir
`tile pitch must be positive and finite`. O vanilla pinado sofre panic nesse
caso em PDF/PNG; essa falha não é requisito do Tekt. Body com label ausente
continua erro causal nos dois compiladores. comparison.json conserva cada
resultado, inclusive falhas, sem excluir controles desfavoráveis.

As sentinelas dense-open e stroke-open continuam divergentes: exigem a troca
efetiva de Shape.fill/stroke e patterns nativos. Gradientes raster, composição
afínica de Parent, recursos recursivos, anotações decorativas, clipping da célula
nativa e integrações de Place/Rotate continuam gates abertos. O checkpoint
não prova isolamento universal de toda feature; a evidência cobre os casos
nomeados. Não se mediu desempenho/memória nem se alega produção pelo número
de objetos de teste.

## Continuação da migração autorizada

shape-field-migration-inventory.json.gz conserva spans/expressões de cada
construção Shape e hashes dos arquivos. A migração deve manter reconstruções
pass-through e converter os produtores reais: shape, Box/Block, contornos de
grid/table, cell_tail_commit e formas Color-only. Grid ainda compara Stroke
antes de fundir segmentos; preparar no emit conserva essa decisão, sem dar
PartialEq a FrameStroke. PageFill é Paint declarativo separado: não substituir
imports globalmente nos exporters. PDF/SVG/PNG, coletores de imagens/fontes/
spots/gradientes e shaper precisam percorrer a célula; FrameVisitor genérico e
plain_text da página não devem fazê-lo. Não adicionar From<Paint> total ou
fallback sólido para obter compilação. T1c/T2/T3 e o objetivo geral permanecem
ativos, sem nova necessidade de aprovação do contrato já ratificado.

Arquivo reaberto e verificado: {len(entries)} entradas,
SHA256 `{evidence_sha}`. Sem commit, push, PR ou release.
'''
Path(str(out)+'.md').write_text(report)
print(json.dumps(dict(entries=len(entries),sha256=evidence_sha,passed=suite['passed'],changed_renders=lines,summary=summary),indent=2))
