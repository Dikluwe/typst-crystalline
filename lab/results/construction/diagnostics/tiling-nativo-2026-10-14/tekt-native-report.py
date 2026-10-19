from pathlib import Path
import gzip,json,hashlib,base64,datetime,subprocess,shutil
from PIL import Image,ImageChops
out=Path('00_nucleo/diagnosticos/tiling-nativo-2026-10-14')
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
with gzip.open(out/'candidate-final.json.gz','rt') as f:candidate=json.load(f)
assert all(sha(p)==v['sha256'] for p,v in candidate['files'].items())
suite=json.loads((out/'workspace-result.json').read_text());assert suite['exit']==0 and suite['sources_unchanged']
checks=json.loads((out/'followup-validation-results.json').read_text());assert all(x['exit']==0 for x in checks)
summary=json.loads((out/'followup-summary.json').read_text());assert summary['clippy_after']<=181
assert summary['architecture'].get('error',0)==0
rows={}
for phase in ['before-cleanup','final-renders']:
 with gzip.open(out/phase/'results.json.gz','rt') as f:record=json.load(f)
 rows[phase]={(x['case'],x['binary'],x['format']):x for x in record['rows']}
def raster(phase,case,binary,fmt):
 p=out/phase/(f'{case}-{binary}.'+fmt+('.png' if fmt!='png' else ''))
 im=Image.open(p).convert('RGBA');bg=Image.new('RGBA',im.size,'white');bg.alpha_composite(im);return bg.convert('RGB')
def compare(a,b):
 assert a.size==b.size
 diff=list(ImageChops.difference(a,b).getdata())
 return dict(size=a.size,pixels_gt16=sum(max(v)>16 for v in diff),maximum=max(map(max,diff)),exact=not any(max(v) for v in diff))
comparisons=[]
for key,after in rows['final-renders'].items():
 case,binary,fmt=key
 if binary!='tekt':continue
 before=rows['before-cleanup'][key];vanilla=rows['final-renders'][case,'vanilla',fmt]
 row=dict(case=case,format=fmt,before_exit=before['exit'],after_exit=after['exit'],vanilla_exit=vanilla['exit'],render_errors=[after.get('render_error'),vanilla.get('render_error')])
 if not before['exit'] and not after['exit'] and not before.get('render_error') and not after.get('render_error'):
  row['preservation']=compare(raster('before-cleanup',*key),raster('final-renders',*key));assert row['preservation']['exact'],row
 if not after['exit'] and not vanilla['exit'] and not any(row['render_errors']):row['against_pin']=compare(raster('final-renders',*key),raster('final-renders',case,'vanilla',fmt))
 comparisons.append(row)
(out/'final-comparison.json').write_text(json.dumps(comparisons,indent=2))
for p in Path('/tmp').glob('tekt-native-*.py'):shutil.copyfile(p,out/p.name)
shutil.copyfile('/tmp/tekt-seal-owned.py',out/'tekt-seal-owned.py')
shutil.copyfile('/tmp/tekt-shape-syntax/src/main.rs',out/'shape-syntax-main.rs.txt')
with gzip.open(out/'final-patch.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=candidate['head'],patch=subprocess.check_output(['git','diff','--',*candidate['files']],text=True)),f)
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name not in ['evidence.json.gz','evidence.sha256','report-result.log']:
  data=p.read_bytes();entries.append(dict(path=str(p.relative_to(out)),sha256=hashlib.sha256(data).hexdigest(),base64=base64.b64encode(data).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:again=json.load(f)['entries']
assert len(again)==len(entries)
for e in again:assert hashlib.sha256(base64.b64decode(e['base64'])).hexdigest()==e['sha256']
hash_=sha(out/'evidence.json.gz');(out/'evidence.sha256').write_text(hash_+'  evidence.json.gz\n')
selected=json.loads((out/'selected-page/result.json').read_text());assert selected['passed']
exact_cases = [case for case in sorted({r['case'] for r in comparisons}) if len([r for r in comparisons if r['case']==case and r.get('against_pin',{}).get('exact')]) == 3]
report=f'''# Tiling: transporte preparado e primeiros exporters nativos — 2026-10-14

Shape.fill/stroke agora transportam FramePaint/FrameStroke. Os produtores do
core preparam a célula e emitem uma Shape, eliminando a grade de clones e seu
limite10000. PDF registra PatternType1, SVG emite pattern userSpaceOnUse e
PNG usa textura periódica. Recursos/shaping percorrem as células; o visitor
semântico público e plain_text continuam sem promovê-las a conteúdo de página.
Box/Block e grupos técnicos conservam o contrato paint_root já aprovado.

**T1c integrado; T2/T3 ainda incompletos.** Transparência PDF, overlap vetorial,
gradientes raster, rotação e guardas de render ainda impedem certificação de
produção. Este relatório não fecha a meta geral nem autoriza release.

## Proveniência

HEAD `{candidate['head'].strip()}`, working tree não commitado. candidate-final,
workspace-pre e pre-validation conservam UTC, fontes, hashes, status e diffstat.
A suíte final correu de `{suite['start']}` a `{suite['end']}` com fontes congelados.
O pin vanilla é 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489, binário SHA256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Os manifests aprovados não foram reescritos. O recibo de aprovação do tiling
continua em materialization/tiling-transporte-2026-10-14/approval-receipt.json.

before-cleanup usa o estado preservado em verified-v3/candidate-final.json.gz;
final-renders usa candidate-final.json.gz. Cada execução registra hash dos
binários/comandos/diagnósticos; PNG usa144PPI, SVG librsvg72DPI×2 e PDF
pdftoppm144DPI, canvas480×280. Não há alinhamento ou máscara escolhida pelo erro.
A sonda after-final com --root falhou no CLI Tekt (main file not found, basename
resolvido na raiz), foi preservada e não serve para afirmar render correto.
As sondas válidas seguintes usam imagem local da fixture. Esse erro de --root
fica aberto para seu owner. native-v1 é investigação, não prova de fechamento.

## Validação

- Suíte completa: **{suite['passed']} aprovados, {suite['failed']} falhas,
  {suite['ignored']} ignorados**, com os fontes inalterados.
- Build, Rustdoc com -D warnings, all-targets, formato, diff-check,
  linhagem estrita v1/v5/v15/v26 e linhagem reversa passaram.
- Clippy: {summary['clippy_after']} avisos existentes; inventário com cap-lints warn,
  não aprovação do gate -D warnings. Os sete avisos introduzidos na migração foram
  corrigidos. Arquitetura: {summary['architecture'].get('error',0)} erros,
  {summary['architecture'].get('warning',0)} avisos e {summary['architecture'].get('note',0)} notas.
- RED→GREEN: preparação de fill+stroke (uma célula por ocorrência), e reuso de
  célula aninhada entre páginas. O cache PDF passou a ser por página; selecionar
  apenas a segunda página não depende de recursos da primeira.
- A primeira suíte falhou na fixture que exigia fallback sólido. A fixture válida
  agora exige padrão nativo com size e ausência de erros; o cenário sem size
  permanece como teste separado do diagnóstico causal. Execução anterior íntegra
  em suite-v1. A contagem inclui migração de testes da grade obsoleta e não é
  evidência isolada de aumento de cobertura.
- A seleção real --pages2 gerou PDF verificado por qpdf e texto PAGE TWO,
  sem PAGE ONE. O recurso aninhado continuou válido.
- A limpeza estática preservou exatamente todos os renders Tekt válidos do corpus;
  comparações em final-comparison.json. PDFs gerados têm check qpdf por caso.

## Resultado e limites

Casos com igualdade de pixels nos três targets, nesta medição: {", ".join(exact_cases)}.
O caso dense-open contém área200×100pt e período1×1pt, acima do limite legado10000.
Igualdade neste corpus não promove todos os parâmetros da linguagem.
Texto de célula coincide em PDF/PNG; SVG vanilla contém XML inválido e seu erro
foi preservado, não corrigido artificialmente.

O contrato exige overlap para spacing negativo. PNG Tekt compõe quatro
contribuições de alpha50% (alpha93.75%); o vanilla medido corta ao pitch e mantém
alpha50%. Não copiar essa diferença. SVG ainda precisa composição explícita de
overlap; PDF ainda perde alpha. PNG de gradiente conserva a limitação anterior
(cor representativa). Rotação/offset e guardas de singularidade/alocação continuam
abertos. remaining-analysis.md distingue medições, inferências e refutadores.

Não há base estatística para dizer que a maioria das falhas de todo o projeto
está em código ou prompts. Nesta frente, o limite de grade e o fallback eram
falhas do transporte/consumidores; o contrato público de transporte precisou ser
ampliado em L0 e foi implementado após aprovação. Divergência com vanilla não é,
por si só, erro do Tekt.

Sem commit, push, PR ou publicação. CI/release de produção continua pendente das
fronteiras abertas e dos demais débitos registrados anteriormente.

## Evidência recuperável

Arquivo evidence.json.gz contém {len(entries)} entradas {{path,sha256,base64}},
reabertas e verificadas. SHA256: `{hash_}`.
'''
Path('00_nucleo/diagnosticos/tiling-nativo-2026-10-14.md').write_text(report)
print(json.dumps(dict(entries=len(entries),sha256=hash_,report='00_nucleo/diagnosticos/tiling-nativo-2026-10-14.md',tests=suite['passed'],clippy=summary['clippy_after'],architecture=summary['architecture']),indent=2))
