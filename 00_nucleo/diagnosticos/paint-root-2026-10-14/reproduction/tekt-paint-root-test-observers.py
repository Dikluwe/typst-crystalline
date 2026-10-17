from pathlib import Path
import subprocess,json,re,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/paint-root-2026-10-14');owners=json.loads((out/'owners.json').read_text());extra=['01_core/src/compiler/layout/columns.rs','01_core/src/compiler/layout/placement.rs'];paths=[]
for src in extra:
 owner=re.search(r'^//! @prompt (.+)$',Path(src).read_text(),re.M).group(1);owners[src]=owner;paths += [src,owner]
with gzip.open(out/'before-extra-test-owners.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in paths}),f)
(out/'owners.json').write_text(json.dumps(owners,indent=2))
for src in extra:
 p=Path(owners[src]);s=p.read_text();pin='- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee';a=s.index('Núcleos Tekt:\n')+len('Núcleos Tekt:\n');b=a
 while s[b]=='-':b=s.index('\n',b)+1
 s=s[:a]+'\n'.join(sorted(s[a:b].splitlines()+[pin]))+'\n'+s[b:]
 s+='''\n## Observação dos grupos de pintura — T1b — 2026-10-14\n\nMedição anterior à decisão: layout-first.log em paint-root-2026-10-14\nregistra testes que procuram Text somente em page.items e não encontram\nas folhas dentro da nova raiz de Block. before-extra-test-owners conserva\nfonte/L0, HEAD/UTC/diffstat antes deste ajuste. O contrato de Group.paint_root\nfoi aprovado no manifesto d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d.\n\nNeste consumer alterar somente o observador de teste afetado para percorrer\na árvore e projetar as coordenadas por Group.pos/matrix, usando o helper\nde teste de layout. Conservar conteúdo, valores esperados e tolerâncias.\nNenhum algoritmo produtivo deste owner muda. O owner do helper é\n00_nucleo/prompts/compiler/layout/tests.md; sua visibilidade interna para\ntestes irmãos não é API do produto. Revalidar os testes e os renders do lote.\n''';p.write_text(s)
for src in ['01_core/src/compiler/layout/tests.rs','01_core/src/compiler/layout/block.rs']:
 p=Path(owners[src]);s=p.read_text()+'''\n### Observadores das regressões históricas\n\nlayout-first.log registra folhas ausentes em buscas planas após o novo\nenvelope, embora o body permaneça no frame. Os observadores dos testes\nafetados devem usar a vista recursiva com projeção afim, preservando todos\nos valores de geometria/texto/tolerâncias. Disponibilizar o helper somente\na árvore de módulos de teste. Não alterar esperados para tornar uma\nregressão visual verde. A fixture de tiling reconhece células como raízes\nlocais; máscaras técnicas externas continuam soft.\n''';p.write_text(s)
log=(out/'layout-first.log').read_text();failed=set(re.findall(r'^---- ([\w:]+) stdout ----$',log,re.M));names={n.split('::')[-1] for n in failed};names|={'p1379_text_y','text_y'}
files=['01_core/src/compiler/layout/tests.rs','01_core/src/compiler/layout/block.rs']+extra
rows=[json.loads(s) for s in subprocess.check_output(['/tmp/tekt-group-syntax/target/debug/tekt-group-syntax']+files,text=True).splitlines()]
changes=[]
for file in files:
 s=Path(file).read_text();lines=s.splitlines(True);edits=[]
 helper='frame_items_recursive' if file.endswith('/tests.rs') else 'super::super::tests::frame_items_recursive'
 for r in rows:
  if r['kind']!='fn' or r['path']!=file or r['name'] not in names:continue
  a,b=r['line']-1,r['end_line'];old=''.join(lines[a:b]);new=old
  expr=r'(?:\b(?:doc|document)\s*\.\s*pages\s*\[\s*0\s*\]|\b(?:page|p))\s*\.\s*items'
  new=re.sub(r'('+expr+r')\s*\.\s*iter\(\)',lambda m:helper+'(&'+m.group(1)+').into_iter()',new)
  if r['name']=='p1254_content_tiling_vira_grupos_repetidos_e_recortados':new=new.replace('paint_root: false','paint_root: true')
  if new!=old:edits.append((a,b,new));changes.append((file,r['name']))
 for a,b,new in sorted(edits,reverse=True):lines[a:b]=[new]
 s=''.join(lines)
 if file.endswith('/tests.rs'):s=s.replace('fn frame_items_recursive(items:', 'pub(super) fn frame_items_recursive(items:',1)
 Path(file).write_text(s)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true']+files,check=True)
(out/'test-observer-changes.json').write_text(json.dumps(changes,indent=2));print('Updated affected observers, with expected values retained:',len(changes))
