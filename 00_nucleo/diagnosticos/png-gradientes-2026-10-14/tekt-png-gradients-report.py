from pathlib import Path
import gzip,json,hashlib,base64,datetime,shutil,collections,difflib,subprocess
from PIL import Image,ImageChops
root=Path.cwd();out=root/'00_nucleo/diagnosticos/png-gradientes-2026-10-14'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
load=lambda p:json.load(gzip.open(p,'rt'))
candidate=load(out/'candidate-final.json.gz');before=load(out/'before-final/results.json.gz')
assert all(sha(p)==v['sha256'] for p,v in candidate['files'].items())
changed=[p for p,h in before['files'].items() if sha(p)!=h]
assert set(changed)=={'03_infra/src/export/render.rs','00_nucleo/prompts/infra/export/render.md'},changed
suite=json.loads((out/'workspace-result.json').read_text());checks=json.loads((out/'followup-validation-results.json').read_text());summary=json.loads((out/'followup-summary.json').read_text())
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged']
assert all(x['exit']==0 for x in checks)
assert summary['clippy_after']==181 and summary['architecture']['warning']==382
def warnings(path):
 return collections.Counter((r['code'],r['message'],tuple(sorted(s['file_name'] for s in r['spans']))) for r in json.loads(Path(path).read_text()))
assert warnings(out/'followup-clippy-inventory.json')==warnings(root/'00_nucleo/diagnosticos/tiling-overlap-2026-10-14/followup-clippy-inventory.json')
metrics=[]
for phase in ['before-final','final-v2','isolation-v2','svg-controls']:
 for p in sorted((out/phase).glob('*-tekt.png')):
  name=p.name[:-9];a=Image.open(p).convert('RGBA');b=Image.open(p.with_name(name+'-vanilla.png')).convert('RGBA');assert a.size==b.size
  ds=[max(v) for v in ImageChops.difference(a,b).getdata()]
  metrics.append(dict(phase=phase,case=name,size=a.size,max_channel=max(ds),changed=sum(v>0 for v in ds),above16=sum(v>16 for v in ds)))
analytic=[];im=Image.open(out/'final-v2/cell-alpha-tekt.png').convert('RGB')
for x in range(8):
 color=[255.0]*3
 for t in [(x+8)/16,x/16,(x+8)/16,x/16]:
  alpha=.2+.6*t;src=[255*(1-t),0,255*t];color=[s*alpha+d*(1-alpha) for s,d in zip(src,color)]
 pixel=im.getpixel((40+x,50));analytic.append(dict(x=x,expected=color,actual=pixel,max_error=max(abs(a-b) for a,b in zip(color,pixel))))
assert max(x['max_error'] for x in analytic)<1.0
final_probe=load(out/'final-v2/results.json.gz');isolation=load(out/'isolation-v2/results.json.gz')
assert not any(x['exit'] for x in final_probe['rows']+isolation['rows'])
assert all(sha(x['path'])==x['sha256'] for x in final_probe['binaries'].values())
verification=dict(utc=now(),head=candidate['head'],candidate_sha256=sha(out/'candidate-final.json.gz'),productive_changes=changed,source_guard=True,clippy_inventory_same=True,metrics=metrics,overlap_analytic=analytic)
(out/'verification.json').write_text(json.dumps(verification,indent=2))
old=before['sources']['03_infra/src/export/render.rs'].splitlines(True);new=Path('03_infra/src/export/render.rs').read_text().splitlines(True)
(out/'render-change.patch').write_text(''.join(difflib.unified_diff(old,new,fromfile='before/render.rs',tofile='final/render.rs')))
for name in ['probe','isolation','svg-controls','implement','checkpoint','suite','validate','report']:
 p=Path('/tmp/tekt-png-gradients-'+name+'.py')
 if p.exists():shutil.copyfile(p,out/p.name)
paths=[p for p in out.rglob('*') if p.is_file() and p.name not in ['evidence.json.gz','evidence.sha256','report-result.log']]
records=[dict(path=str(p.relative_to(root)),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()) for p in sorted(paths)]
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(records,f)
digest=sha(out/'evidence.json.gz');(out/'evidence.sha256').write_text(digest+'  evidence.json.gz\n')
for r in load(out/'evidence.json.gz'):
 data=base64.b64decode(r['base64']);assert hashlib.sha256(data).hexdigest()==r['sha256'];assert (root/r['path']).read_bytes()==data
rows=[r for r in metrics if r['phase']=='final-v2'];exact=sum(r['max_channel']==0 for r in rows)
table='\n'.join(f"| {r['case']} | {r['max_channel']} | {r['above16']} |" for r in rows)
report=f'''# Gradientes PNG e raio externo dos contornos

O rasterizador agora amostra Linear, Radial e Conic em vez de usar a primeira
cor. Preserva alpha, stops, espaço cromático, foco radial, Self/Parent, raiz de
grupo/célula, transforms recebidos e paint do fundo. O raio externo de RoundedRect
com stroke também foi corrigido: a linha central desconta meia espessura.
Não é certificação de toda a paridade PNG nem conclusão da meta geral.

## Proveniência

HEAD `{candidate['head'].strip()}`, working tree não commitado. before-final/results.json.gz
preserva UTC, status/diffstat exatos, hashes de todas as fontes/owners/núcleos
inspecionados e binários. candidate-final.json.gz e workspace-pre.json.gz registram
o estado final; verification.json prova que somente render.rs e seu L0 mudaram
entre as fontes produtivas do inventário. Diff próprio em render-change.patch.
Pin vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário
SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
CLI PNG com --ppi144; SVG de controle via rsvg-convert --zoom2 e DPI72.
Sem realinhamento, recorte por erro ou equivalência de PNG comprimido.

## Implementação e gates

L0 editado antes de código, correção interna de paridade ADR-0127. Sem API pública,
flag, default ou fase nova. Estado privado transporta dimensões da raiz e densidade.
Gradiente usa Pad/Nearest; tiling conserva Repeat e composição do período.
Alpha é premultiplicado antes da quantização. Retângulos negativos espelham Self.
Os raios de clips sem stroke permanecem intactos; fill fecha e rounded stroke
retorna ao início sem fechamento implícito, preservando cap/join.

red-functional-2.log registra cinco regressões funcionais; red-source.rs.txt
preserva o código. Os logs RED anteriores são tentativas de compilação das fixtures,
não evidência funcional. anisotropic-red.log isola densidade diferente por eixo;
round-red-2.log é o RED do canto externo com pixel verificado na sonda. As primeiras
fixtures before/before-valid continham stops/sharp inválidos e foram corrigidas em
before-final, com erros preservados. round-red.log usava ponto de controle inadequado;
o ponto correto está em round-red-2.log e round-red-source.rs.txt, antes da correção.
Oito testes novos estão incluídos na suíte final. Os resultados intermediários de
8.553 testes foram preservados em first-pass; não são a prova do estado final.

Suíte final UTC {suite['start']} → {suite['end']}:
**{suite['passed']} passaram, {suite['failed']} falharam, {suite['ignored']} ignorados**, com source guard.
Build, rustdoc com -D warnings, check all-targets, fmt, diff e linhagem estrita
v1/v5/v15/v26 mais linhagem reversa passaram. Clippy permanece em 181 avisos,
inventário normalizado idêntico ao anterior; arquitetura em 382 avisos,
{summary['architecture'].get('note',0)} notas e zero erros. Não confundir estes resultados com CI remoto
verde: cargo clippy -D warnings ainda bloqueia. Não houve commit/push/release.

## Resultado visual e limites

{exact} dos {len(rows)} controles principais coincidem pixel a pixel. Stops explícitos
divergem no máximo 1 nível de canal. A elipse tem diferença máxima 15 nas bordas;
o controle sólido também apresenta diferença de cobertura (máximo 13). Isso não
é aprovado por uma tolerância geral: os valores completos estão abaixo.
Contorno arredondado sólido e gradiente passaram de 396 pixels acima de 16 para
coincidência exata após o inset. Gradiente linear RGB, Oklab, LinearRGB, HSV e
Oklch coincidem nos controles registrados; HSL difere no máximo 1. CMYK permanece
divergente (máximo 114,4800 pixels acima de 16) e exige investigação de conversão.

| Controle principal | Máximo por canal | Pixels acima de 16 |
|---|---:|---:|
{table}

O limiar 16 é inventário descritivo, não gate de aceitação nem tolerância ampliada.
Quatro casos rotate/scale ainda divergem. A mesma divergência de posição aparece nos
controles sólidos PNG e SVG; transform.rs conserva um pivô próprio. Não corrigir
essa posição distorcendo a amostragem de gradientes. O teste direto de Group afim,
Parent e siblings passa; o caminho fonte→layout de transformações continua aberto.

Overlap transparente é um caso em que não copiar o pin: typst-render/src/paint.rs:
296–308 rasteriza diretamente num canvas do período e recorta parte da célula.
No controle constante a 50%, vanilla PNG fica(255,127,127), enquanto as quatro
contribuições exigem alpha 93,75%. No gradiente variável, a composição explícita
Tekt fica a no máximo {max(x['max_error'] for x in analytic):.6f} nível de canal do cálculo analítico.
O controle sem overlap coincide. Preservar composição autorizada no L0/núcleo de
tiling; a diferença do pin nesse caso não deve virar regressão deliberada.

## Continuidade

A meta geral permanece aberta: posição/origem de transformações, conversões CMYK,
avisos de CI e demais refutadores dos relatórios anteriores ainda precisam de
correção. Esta etapa não certifica Glyph direto, fontes afins, EXIF, alocação,
performance/caching, PageFill Tiling ou toda a produção. As junções de PDF ficam
na investigação separada tiling-juncoes-2026-10-14.md; este trabalho não mudou PDF.

## Evidência recuperável

evidence.json.gz contém {len(records)} entradas path/sha256/base64, reabertas e
comparadas com os arquivos atuais. SHA-256 `{digest}`. Inclui fontes RED/final,
scripts, comandos, imagens, resultados e logs completos. Os scripts de mutação
são registros históricos, não devem ser reexecutados sobre o estado final.
'''
report_path=root/'00_nucleo/diagnosticos/png-gradientes-2026-10-14.md';report_path.write_text(report)
print(json.dumps(dict(report=str(report_path),entries=len(records),sha256=digest,tests=suite['passed'],productive_changes=changed),indent=2))
