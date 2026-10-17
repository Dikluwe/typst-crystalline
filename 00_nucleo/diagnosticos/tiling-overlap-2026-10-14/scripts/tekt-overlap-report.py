from pathlib import Path
import gzip,json,hashlib,base64,datetime,subprocess,shutil
from PIL import Image,ImageChops
out=Path('00_nucleo/diagnosticos/tiling-overlap-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
with gzip.open(out/'candidate-final.json.gz','rt') as f:candidate=json.load(f)
assert all(sha(p)==v['sha256'] for p,v in candidate['files'].items())
suite=json.loads((out/'workspace-result.json').read_text());assert suite['exit']==0 and suite['sources_unchanged']
checks=json.loads((out/'followup-validation-results.json').read_text());assert all(x['exit']==0 for x in checks)
summary=json.loads((out/'followup-summary.json').read_text());assert summary['clippy_after']==181 and summary['architecture'].get('error',0)==0
semantic=json.loads((out/'semantic-v2/result.json').read_text());assert all(x['passed'] for x in semantic['rows'])
package=Path('00_nucleo/materialization/tiling-transporte-2026-10-14');assert sha(package/'manifest.json')=='d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d'
manifest=json.loads((package/'manifest.json').read_text());assert all(sha(package/p)==h for p,h in manifest['candidates'].items())
rows={}
for phase in ['initial-renders','final-renders']:
 with gzip.open(out/phase/'results.json.gz','rt') as f:r=json.load(f)
 rows[phase]={(x['case'],x['binary'],x['format']):x for x in r['rows']}
def raster(phase,case,binary,fmt):
 p=out/phase/(case+'-'+binary+'.'+fmt+('.png' if fmt!='png' else ''));im=Image.open(p).convert('RGBA');bg=Image.new('RGBA',im.size,'white');bg.alpha_composite(im);return bg.convert('RGB')
def compare(a,b):
 assert a.size==b.size
 d=list(ImageChops.difference(a,b).getdata());return dict(maximum=max(map(max,d)),pixels_gt16=sum(max(x)>16 for x in d),exact=not any(max(x) for x in d))
comparisons=[]
for key,after in rows['final-renders'].items():
 case,binary,fmt=key
 if binary!='tekt':continue
 before=rows['initial-renders'][key];vanilla=rows['final-renders'][case,'vanilla',fmt]
 row=dict(case=case,format=fmt,exit=after['exit'],vanilla_exit=vanilla['exit'],render_errors=[after.get('render_error'),vanilla.get('render_error')])
 assert before['exit']==after['exit']
 if not after['exit'] and not after.get('render_error'):
  row['preservation']=compare(raster('initial-renders',*key),raster('final-renders',*key));assert row['preservation']['exact']
  assert after.get('qpdf_exit',0)==0
  if not vanilla['exit'] and not vanilla.get('render_error'):row['against_pin']=compare(raster('final-renders',*key),raster('final-renders',case,'vanilla',fmt))
 comparisons.append(row)
(out/'final-comparison.json').write_text(json.dumps(comparisons,indent=2))
assert sha('target/debug/typst')==r['binaries']['tekt']['sha256']==semantic['binary_sha256']
full=[]
for name,expected in [('overlap-alpha',(255,15.9375,15.9375)),('overlap-order',(159.375,63.75,79.6875))]:
 for fmt in ['pdf','svg','png']:
  im=raster('final-renders',name,'tekt',fmt);errors=sorted(max(abs(v-e) for v,e in zip(im.getpixel((x,y)),expected)) for y in range(40,100) for x in range(40,120))
  full.append(dict(case=name,format=fmt,expected=expected,mask='x in [40,120), y in [40,100), from source geometry',max_error=errors[-1],p95=errors[int(.95*(len(errors)-1))],over16=sum(e>16 for e in errors)))
(out/'final-analytic.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_provenance='final-renders/results.json.gz',source_snapshot='candidate-final.json.gz',rows=full),indent=2))
for p in Path('/tmp').glob('tekt-overlap-*.py'):shutil.copyfile(p,out/'scripts'/p.name)
shutil.copyfile('/tmp/tekt-seal-owned.py',out/'scripts/tekt-seal-owned.py')
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=candidate['head'],status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(sha256=sha(p),text=Path(p).read_text()) for p in candidate['files']},approved_manifest_verified=True,candidates_unchanged=True),f)
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name not in ['evidence.json.gz','evidence.sha256','report-result.log']:
  data=p.read_bytes();entries.append(dict(path=str(p.relative_to(out)),sha256=hashlib.sha256(data).hexdigest(),base64=base64.b64encode(data).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:again=json.load(f)['entries']
assert len(again)==len(entries)
for e in again:
 data=base64.b64decode(e['base64']);assert hashlib.sha256(data).hexdigest()==e['sha256'];assert data==(out/e['path']).read_bytes()
digest=sha(out/'evidence.json.gz');(out/'evidence.sha256').write_text(digest+'  evidence.json.gz\n')
report=f'''# Tiling: sobreposição vetorial e alpha sólido PDF — 2026-10-14

PDF e SVG agora compõem explicitamente um período de tiling, referenciando uma
célula vetorial única e recortada. O PDF conserva alpha sólido de formas, linhas,
texto/glifos, canvas e células aninhadas, com recursos ExtGState e restauração
de estado gráfico. O transporte público aprovado permanece intacto.

**Progresso integrado e verificado; T2/T3 ainda incompletos.** O controle PDF de
quatro cores revelou marcas nas junções periódicas; gradientes PNG, alpha
variável de gradientes e guardas extremos continuam abertos. Não é certificação
de produção, fechamento da meta geral ou autorização de release.

## Proveniência

HEAD `{candidate['head'].strip()}`, branch Tekt, working tree não commitado.
`before.json.gz`, `candidate-probe.json.gz`, `candidate-final.json.gz`,
`workspace-pre.json.gz`, `pre-validation.json.gz` e `final-state.json.gz`
registram fontes/owners, SHA-256, UTC, status e diffstat. O manifesto aprovado
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` e seus candidatos
foram revalidados sem alteração. O recibo humano continua no pacote aprovado.

Pin vanilla: `9f2b6e8715237cb086899a42873660fe744622e8`; árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Binário Tekt final: `{r['binaries']['tekt']['sha256']}`.
Os registros de renders contêm comandos, fontes e hashes dos binários. PNG:
144 PPI; SVG: librsvg 72 DPI ×2; PDF: pdftoppm 144 DPI. Nenhuma saída foi
realinhada por pixels. A máscara analítica é a caixa autoral, não uma seleção
pelo erro.

## Causa e correção

O código PDF descartava alpha. O L0 de tiling também tinha uma hipótese
insuficiente: confiar no overlap do Pattern PDF e em overflow visible do SVG.
A medição anterior mostrou PDF opaco e SVG com apenas uma contribuição.
Portanto esta frente tinha defeitos no código e na especificação do mecanismo;
isso não permite estimar a maioria dos defeitos de todo o projeto.

L0 atualizado primeiro, em fluxo interno ADR-0127, com novo Núcleo de alpha
compartilhado e ownership 1:1. O helper de tiling enumera as células que cruzam
um período, na ordem y/x crescente. PDF guarda o corpo num Form XObject com
BBox=size; o Pattern tem BBox=pitch e referencia o Form. SVG guarda um g
recortado e usa referências traduzidas dentro do pattern. Não há clones L1
nem grade proporcional ao viewport.

Recursos PDF coletam pares fill/stroke de alpha f32 sem quantização u8, inclusive
em células. Os dispatchers isolam folhas translúcidas; o número de restaurações
pendentes mantém ActualText antes de Q. O prefixo verboso compartilhado cobre
runs fundidos. Os controles opacos conservam seus operadores.

## Verificação

- Suíte completa de `{suite['start']}` a `{suite['end']}`: **{suite['passed']}
  aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados**. Hashes das
  fontes verificados antes/depois; execução com TMPDIR exclusivo em /dev/shm.
- Build, Rustdoc com -D warnings, all-targets, formato, diff-check, linhagem
  estrita v1/v5/v15/v26 e linhagem reversa passaram.
- Clippy mantém **{summary['clippy_after']} avisos**, sem aumento. Foi inventário
  com cap-lints warn; o gate de produção -D warnings não está liberado.
  Arquitetura: **0 erros, {summary['architecture']['warning']} avisos,
  {summary['architecture']['note']} notas**.
- RED→GREEN: padrão com quatro contribuições/corpo único; alpha e restauração
  antes de folha opaca seguinte, com ActualText em top/local e compacto/verboso.
  Interseções parciais do período também têm teste analítico.
- A primeira montagem de testes tinha tipos de mapas/cap incorretos; red.log
  conserva esse erro de fixture. red-2.log e red-alpha.log são as falhas
  funcionais anteriores à implementação. A expectativa SVG foi ajustada de
  atributo fill-opacity para o CSS rgba que o exporter já usa, sem mudar alpha.
- Todos os renders Tekt válidos de initial-renders e final-renders são idênticos;
  a preparação final alterou apenas formato/linhagem e fortaleceu os testes.
  Os PDFs do corpus passaram qpdf.
- Oito controles reais em semantic-v2 cobrem compacto/verboso, tagging padrão,
  desligado, UA-1 e seleção da página2. qpdf e pdftotext passaram: CELL não vira
  texto de página; seleção preserva PAGE TWO sem PAGE ONE. Isso não certifica
  conformidade integral PDF/UA. A fixture inicial usava title como content e
  foi rejeitada antes de exportar; semantic/fixture-error.json conserva o erro.

## Resultados e refutador visual

Na célula8×8 com pitch4×4 e vermelho alpha50%, quatro contribuições dão
alpha93,75%. Nos **4.800 pixels** da caixa autoral, PDF/SVG/PNG produzem
(255,15,15), erro máximo0,9375 contra composição analítica em canais de8 bits.
O vanilla pinado mantém apenas uma contribuição; essa divergência do Tekt é
intencional e correta segundo o contrato aprovado, não um defeito a copiar.

O controle de ordem usa quadrantes amarelo→verde→azul→vermelho a50%. O valor
analítico final é (159,375;63,75;79,6875). SVG e PNG são uniformes (159,63,79),
erro máximo0,75. PDF dá (158,62,78) nos pontos internos, mas a inspeção da
**caixa inteira** encontra junções: erro máximo7,75 e p95=4,75; nenhum pixel
supera16. O número16 é somente a métrica descritiva já usada no corpus,
**não um novo limite de aceitação**. O controle não está fechado como uniforme.
A origem exata das junções ainda precisa de isolamento; não se atribui intenção
nem se relaxa o contrato para aprovar a imagem.

O PDF do controle rotate-offset agora coincide em todos os pixels com o pin;
antes tinha1.070 pixels com diferença>16. Os demais controles antigos sem
overlap preservam os renders anteriores. Os novos controles solid-alpha,
text-alpha e text-alpha-cell coincidem com o pin em PDF. Comparações completas
estão em final-comparison.json e final-analytic.json.

## Fronteiras ainda abertas

- Junções PDF do período multicolorido descritas acima.
- Gradientes PNG continuam em cor representativa; no controle anterior há
  4.500 pixels com diferença>16. A rotação PNG conserva451 pixels>16. Esses
  números vêm de tiling-nativo/final-comparison e do corpus desta etapa,
  não de certificação de qualidade.
- Alpha variável de gradientes PDF exige trabalho próprio; não usar alpha de
  uma parada como substituto. Os perfis de conformidade continuam com o escopo
  limitado já declarado pelo owner PDF.
- Ranges extremos, underflow de size/pitch, singularidade e alocação ainda
  precisam diagnóstico integrado. As chamadas expect do PDF e do enumerador
  não constituem uma política de erro de produção. Não há truncamento do padrão
  nem fallback sólido para fingir sucesso.
- Continuam os demais itens de T3: transformações/contornos complexos,
  fractional Box/Block fragmentado, qualidade subpixel e performance física.
  O erro CLI --root já medido, os avisos de CI e o restante da meta geral
  permanecem abertos.
- O SVG vanilla de texto de célula contém XML inválido; o PNG vanilla subpixel
  entra em panic. Erros externos preservados, sem alterar o oráculo.

Nenhum commit, push, PR ou publicação. A aprovação cobre implementação/testes;
release continua excluída.

## Evidência recuperável

`tiling-overlap-2026-10-14/evidence.json.gz` contém **{len(entries)} entradas**
{{path,sha256,base64}}, reabertas e verificadas também contra os arquivos atuais.
SHA-256: `{digest}`.
'''
Path('00_nucleo/diagnosticos/tiling-overlap-2026-10-14.md').write_text(report)
print(json.dumps(dict(entries=len(entries),sha256=digest,tests=suite['passed'],clippy=summary['clippy_after'],report='00_nucleo/diagnosticos/tiling-overlap-2026-10-14.md'),indent=2))
