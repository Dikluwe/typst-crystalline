from pathlib import Path
import json,gzip,hashlib,base64,datetime,shutil,re,collections
base=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14');out=base/'png-afim';v=out/'validation-final'
summary=json.loads((v/'followup-summary.json').read_text());assert summary['sources_unchanged']
assert all(r['exit']==0 for r in json.loads((v/'followup-validation-results.json').read_text()))
pre=json.load(gzip.open(v/'pre-validation.json.gz','rt'))
sha=lambda b:hashlib.sha256(b).hexdigest()
assert all(sha(Path(p).read_bytes())==r['sha256'] for p,r in pre['files'].items())
integration=json.loads((out/'integration/workspace-result.json').read_text());assert integration['exit']==0 and integration['sources_unchanged'] and integration['passed']==7
before=json.loads(Path('00_nucleo/diagnosticos/png-gradientes-2026-10-14/followup-clippy-inventory.json').read_text());after=json.loads((v/'followup-clippy-inventory.json').read_text())
key=lambda x:(x['code'],x['message'],tuple(s['file_name'] for s in x['spans']))
same=collections.Counter(map(key,before))==collections.Counter(map(key,after));assert same and len(after)==181
p=base.with_suffix('.md');s=p.read_text();s=s.replace('''- Raster PNG de texto afim: render.rs:402–405 usa apenas sx/sy e ignora kx/ky;
  o pin typst-render/src/text.rs:53–91 usa contorno sob transformação completa.
  Reflow-wrap-rotate coincide em SVG e quase integralmente em PDF, mas PNG não.''','''- O defeito de rasterização afim de TextShaped foi corrigido na etapa abaixo.
  Casos com Box ainda divergem também em SVG/PDF; não confundir sua baseline ou
  largura lógica com falha remanescente de rotação dos glifos. Glyph direto,
  fontes coloridas/bitmap e suporte integral de matemática continuam não certificados.''')
s=s.replace('''Ainda faltam concluir estes refutadores, a integração completa X1–X5, inventário
final de avisos e checks amplos após a última correção.''','''Ainda faltam concluir estes refutadores e a integração completa X1–X5.
Os avisos registrados abaixo continuam impedindo considerar o CI estrito limpo.''')
s+=f'''

## Rasterização afim de texto PNG — etapa verificada

Em `03_infra/src/export/render.rs`, TextShaped agora usa contorno da face preparada
sob a transformação completa para rotação, shear, escala não uniforme e reflexão;
o caminho pixglyph uniforme permanece. Origem e escala de unidades da fonte são
compostas uma vez, com inversão do eixo da fonte, máscara do dispositivo e alpha.
Nenhuma API pública ou fase mudou. L0 foi atualizado antes dos testes e código.

Dois testes novos falharam funcionalmente antes da correção: a rotação de 90 graus
perdia a tinta, e o clip não encontrava o glifo transformado. Após a correção,
21 testes do renderizador passaram (`implementation/png-affine-{{red,green}}-result.json.gz`,
logs correspondentes e `png-afim/render-red.rs.txt`). A integração PNG passou com
7 testes, UTC {integration['start']} → {integration['end']}, em
`png-afim/integration/workspace-result.json`, com guard das fontes e proveniência.
A suíte completa de 8557 testes descrita acima antecede esta mudança PNG; não
foi apresentada como uma nova suíte completa do estado posterior.

`refuters-fourth/results.json.gz` mostra reflow-wrap-rotate e matemática rotacionada
coincidentes em PNG. `margin-png-after` conserva a correção com margem grande e
passa a coincidir também no controle rotacionado. `glyph-affine-final/results.json.gz`
mede cinco controles adicionais coincidentes em SVG e PNG: quarter-turn, shear,
anisotropic, flip-x e flip-y. O sexto, glyph-clip, ainda diverge nos dois formatos:
a geometria do frame de Box precisa ser corrigida; o teste unitário de máscara
confirma somente o recorte do contorno já posicionado. Não realinhar imagens para
ocultar posição nem usar soma de pixels como prova universal de paridade.

A primeira validação expôs dois avisos novos de sin/cos nos testes; foram trocados
por libm pinado. Os auxiliares privados novos enumeram os casos sem baseline e
não nativos, exigindo revisão de futuras variantes em vez de defaults silenciosos.
As regressões de reflow foram reexecutadas após essa mudança, passando
(`implementation/hygiene-core-result.json.gz`).

Validação final ({summary['end']}): build, rustdoc com -D warnings, check de todos
os alvos, fmt, linhagem estrita V1/V5/V15/V26, linhagem reversa e diff check passaram.
Comandos, saídas completas, UTC, HEAD/diffstat e bytes/hashes atuais estão em
`png-afim/validation-final/`. O inventário Clippy permanece com {len(after)} avisos,
com mesmos códigos/mensagens/arquivos do inventário anterior; não é execução
verde de -D warnings. O linter Tekt registra {summary['architecture'].get('warning',0)} warnings,
{summary['architecture'].get('note',0)} notes e nenhum error. As duas categorias de
avisos permanecem trabalho aberto, distinto da linhagem estrita já validada.

Arquivo da etapa anterior: `implementation-evidence-1.json.gz`, 1440 entradas,
SHA-256 `46edca1da3dfc5724418a8cbe68b78b3360da997503b8f8d665611c1f51050c8`.
Ele foi reaberto e comparado antes desta etapa PNG. Nenhum commit, push ou release.
'''
p.write_text(s)
shutil.copyfile(p,out/'report-snapshot.md')
paths=set(p for p in out.rglob('*') if p.is_file() and 'evidence' not in p.name)
for name in ['refuters-fourth','margin-png-after','glyph-affine-final']:
 paths.update(p for p in (base/name).rglob('*') if p.is_file())
paths.update(p for p in (base/'implementation').glob('png-affine*') if p.is_file())
paths.update(p for p in (base/'implementation').glob('hygiene-core*') if p.is_file())
for source in json.loads((base/'implementation/changed-sources.json').read_text()):
 p=Path(source);paths.add(p);owner=Path(re.search(r'^//! @prompt (.+)$',p.read_text(),re.M)[1]);paths.add(owner)
 for n in re.findall(r'^- (00_nucleo/prompts/_nuclei/\S+) sha256:',owner.read_text(),re.M):paths.add(Path(n))
paths.add(Path('03_infra/fixtures/fonts/NotoSans-Regular.ttf'))
recipes=out/'recipes';recipes.mkdir(exist_ok=True)
for pattern in ['tekt-transform*.py','tekt-seal-owned.py']:
 for p in Path('/tmp').glob(pattern):
  dest=recipes/p.name;shutil.copyfile(p,dest);paths.add(dest)
for name in ['tekt-transform-glyph-final.log','tekt-transform-refuters-fourth.log','tekt-transform-margin-png-after.log','tekt-transform-png-seal.log','tekt-transform-hygiene-seal.log']:
 dest=out/name;shutil.copyfile(Path('/tmp')/name,dest);paths.add(dest)
entries=[]
for p in sorted(paths):
 b=p.read_bytes();entries.append(dict(path=str(p),sha256=sha(b),base64=base64.b64encode(b).decode()))
archive=out/'evidence.json.gz';assert not archive.exists()
with gzip.open(archive,'wt') as f:json.dump(entries,f)
check=json.load(gzip.open(archive,'rt'))
for e in check:
 b=base64.b64decode(e['base64']);assert sha(b)==e['sha256'] and b==Path(e['path']).read_bytes()
receipt=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),archive=str(archive),sha256=sha(archive.read_bytes()),entries=len(check),bytes=archive.stat().st_size,reopened_and_compared=True,clippy_inventory_same=True)
(out/'receipt.json').write_text(json.dumps(receipt,indent=2));print(json.dumps(receipt),flush=True)
p=base.with_suffix('.md');p.write_text(p.read_text()+f"\nArquivo recuperável desta etapa: `{archive.relative_to(base)}`, {receipt['entries']} entradas\ncom path/SHA-256/base64, reabertas e comparadas. SHA-256 `{receipt['sha256']}`.\n")
