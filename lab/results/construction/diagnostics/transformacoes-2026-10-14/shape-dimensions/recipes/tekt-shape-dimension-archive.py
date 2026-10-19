from pathlib import Path
import json,gzip,hashlib,base64,datetime,re,shutil
base=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14');out=base/'shape-dimensions';v=out/'validation';sha=lambda b:hashlib.sha256(b).hexdigest()
pre=json.load(gzip.open(v/'pre-validation.json.gz'));assert all(sha(Path(p).read_bytes())==x['sha256'] for p,x in pre['files'].items())
checks=json.loads((v/'followup-validation-results.json').read_text());assert len(checks)==9 and all(x['exit']==0 for x in checks)
suite=json.loads((out/'workspace/workspace-result.json').read_text());assert suite['exit']==0 and suite['sources_unchanged']
summary=json.loads((v/'followup-summary.json').read_text());assert summary['sources_unchanged']
folders=['shape-dim-before-complete','shape-dim-after','shape-dim-pdf-complete','shape-dim-controls','after-shape-dim','refuters-shape-dim','rtl-mixed-shape-dim']
results={name:json.load(gzip.open(base/name/'results.json.gz')) for name in folders};assert all(x['source_guard'] for x in results.values())
report=f'''# Dimensões relativas de formas — correção interna

Estado: correção de Length/Ratio/Relative materializada e validada. None/Auto,
identidade circle/square, casts, altura automática de página e RTL continuam
pendentes; este resultado não certifica todas as formas nem produção/release.

## Proveniência e decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Cada resultado citado inclui UTC, status/diffstat exatos e hashes das fontes e
binários. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, receita/árvore
P1489, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Os bytes anteriores dos owners estão em before-sources.json.gz; focused.patch
isola esta etapa do restante da árvore suja.

shape-dim-before-complete/results.json.gz registra 20 documentos bilaterais,
UTC {results['shape-dim-before-complete']['start']} → {results['shape-dim-before-complete']['end']}.
helpers.rs:427–438 descartava Ratio/Relative e a parte em de Length. O L0
herdado não especificava sua resolução completa. Shape usa agora um resolvedor
privado no próprio owner, mantendo unidades até layout. A parcela percentual
usa o eixo regional; a parcela absoluta usa a fonte efetiva. Produto relativo
não finito contribui zero, como medido, sem perder a parcela absoluta.

Block instala/restaura relative_region_size com largura interna e altura de
referência após inset. Subframe herda a região real em vez do marcador infinito
de altura Auto. Transform nativo também usa a largura interna de Block para
medir/compor, evitando que a margem física da raiz inclua parte do inset.
Nenhuma API pública, campo público ou fase nova; L0 atualizado primeiro,
resselo restrito nos cinco consumers e seus owners.

## Testes e refutadores

Quatro regressões falharam antes da implementação (implementation/shape-dim-red).
A primeira tentativa passou duas, mas Block media 37.5 em vez de 35; a seguinte
passou três, deixando o mesmo erro em Rotate dentro de Block. A largura interna
foi aplicada nesses dois consumidores. implementation/shape-dim-green-third
passou as quatro regressões, incluindo estilo do uso, Box explícito/auto/aninhado,
Block explícito/auto com inset, restauração do pai e região ilimitada.

shape-dim-after e shape-dim-pdf-complete recompilam os 20 controles. Os retângulos
com dimensões explícitas e as regiões aninhadas coincidem nos três formatos.
A imagem de Block→Rotate→Rect foi inspecionada sem realinhamento.
A primeira tentativa before abortou ao rasterizar SVG inválido de unbounded;
a primeira rodada PDF after abortou por tamanho de imagem diferente. Ambas
são tentativas incompletas; as rodadas complete registram esses resultados
como falhas/diferenças, sem interromper nem tratá-los como aprovação.

O controle unbounded passou de SVG inválido para forma 10×10, mas a página
continua maior que a referência: o fluxo inclui margem vertical final indevida.
shape-dim-controls compara forma absoluta 10pt com relativa 50%+10pt: cada binário
produz os mesmos pixels dentro do próprio par, embora Tekt e pin difiram na
página. Os controles de elipse 102×51 também são idênticos dentro de cada binário,
mas ainda divergem entre eles; a resolução dimensional não é a causa dessa
diferença. equivalent-controls.json conserva hashes e comparação exata dos
pares. Não presumir que toda diferença de contorno seja defeito do Tekt.
Move percentual em Block Auto com inset coincide em SVG/PNG após a correção.

Defaults de rect/ellipse/circle e dimensões Auto continuam incorretos e foram
preservados como refutadores abertos. O passo de continuidade está em
00_nucleo/materialization/shape-dimensoes-automaticas.md; não é L0. Dar o mesmo
default a circle e ellipse perderia a identidade, que o carrier atual não separa.
O helper legado de estimativa continua com seus limites; não foi ampliado por
inferência nem usado para certificar os resultados do layout real.

A rodada after-shape-dim conserva 74 consultas e diagnósticos, 24 renders SVG
coincidentes e o erro esperado de dois eixos auto. refuters-shape-dim conserva
as mesmas métricas do corpus anterior. rtl-mixed-shape-dim passa os controles
percentuais LTR; os defeitos de alinhamento RTL permanecem, agora sem a
compensação acidental de uma forma com largura errada.

## Validação final e limites

Suíte completa: {suite['passed']} testes aprovados, {suite['failed']} falhas,
{suite['ignored']} ignorados; UTC {suite['start']} → {suite['end']}.
workspace/workspace-pre.json.gz, workspace-result.json e workspace.log
preservam comandos, ambiente isolado e guard das fontes inalteradas.
Build, rustdoc com -D warnings, check de todos os alvos, fmt, linhagem estrita
V1/V5/V15/V26, linhagem reversa e diff check passaram; evidência em validation/.
Clippy registra {summary['clippy_after']} avisos no inventário com cap-lints warn,
não um CI estrito verde. Tekt registra {summary['architecture'].get('warning',0)} warnings,
{summary['architecture'].get('note',0)} notes e {summary['architecture'].get('error',0)} errors.
A meta geral continua aberta; não houve commit, push, publicação ou release.
'''
(out/'review.md').write_text(report)
main=base.with_suffix('.md');main.write_text(main.read_text()+f'''\n\n## Dimensões relativas de formas — correção posterior\n\n`shape-dimensions/review.md` registra a correção de porcentagens, em e\ncomprimentos mistos em Shape, a região real de Block/Subframe e a largura\ninterna usada pelas transformações. Quatro regressões RED→GREEN passaram.\nA suíte completa terminou com {suite['passed']} testes passados, {suite['failed']} falhas\ne {suite['ignored']} ignorados; UTC {suite['start']} → {suite['end']}.\nBuild/doc/check/fmt/linhagem passaram. Dimensões automáticas, identidade de\ncírculo/quadrado, altura Auto de página, contorno de elipse e RTL continuam\npendentes. Não confundir esta correção com conclusão de X1–X5 ou CI/release.\n''')
paths={p for p in out.rglob('*') if p.is_file()}
for folder in folders:paths.update(p for p in (base/folder).rglob('*') if p.is_file())
paths.update(p for p in (base/'implementation').glob('shape-dim*') if p.is_file());paths.add(base/'implementation/changed-sources.json')
paths.add(Path('00_nucleo/materialization/shape-dimensoes-automaticas.md'))
for p in [Path('01_core/src/compiler/layout')/(stem+'.rs') for stem in ['shape','block','sub_frame','transform','tests']]:
 paths.add(p);owner=Path(re.search(r'^//! @prompt (.+)$',p.read_text(),re.M)[1]);paths.add(owner)
 for n in re.findall(r'^- (00_nucleo/prompts/_nuclei/\S+) sha256:',owner.read_text(),re.M):paths.add(Path(n))
recipes=out/'recipes';recipes.mkdir(exist_ok=True)
for p in list(Path('/tmp').glob('tekt-shape-dimension-*.py'))+[Path('/tmp/tekt-transform-run.py'),Path('/tmp/tekt-seal-owned.py'),Path('/tmp/tekt-transform-rtl-mixed.py'),Path('/tmp/tekt-transform-reprobe.py'),Path('/tmp/tekt-transform-refuters.py')]:
 dest=recipes/p.name;shutil.copyfile(p,dest);paths.add(dest)
for p in list(Path('/tmp').glob('tekt-shape-dim-*.log'))+list(Path('/tmp').glob('tekt-transform-*shape-dim.log')):
 dest=out/p.name;shutil.copyfile(p,dest);paths.add(dest)
entries=[]
for p in sorted(paths):
 b=p.read_bytes();entries.append(dict(path=str(p),sha256=sha(b),base64=base64.b64encode(b).decode()))
archive=out/'evidence.json.gz';assert not archive.exists()
with gzip.open(archive,'wt') as f:json.dump(entries,f)
for e in json.load(gzip.open(archive)):
 b=base64.b64decode(e['base64']);assert sha(b)==e['sha256'] and b==Path(e['path']).read_bytes()
receipt=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),archive=str(archive),sha256=sha(archive.read_bytes()),entries=len(entries),reopened_and_compared=True)
(out/'receipt.json').write_text(json.dumps(receipt,indent=2));print(json.dumps(receipt),flush=True)
main.write_text(main.read_text()+f"\nEvidência recuperável: `shape-dimensions/evidence.json.gz`, SHA-256 `{receipt['sha256']}`, {receipt['entries']} entradas reabertas e comparadas.\n")
