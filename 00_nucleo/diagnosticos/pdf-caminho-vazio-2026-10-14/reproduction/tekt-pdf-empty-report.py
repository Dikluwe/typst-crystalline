from pathlib import Path
import gzip,json,hashlib,datetime,base64,difflib
out=Path('00_nucleo/diagnosticos/pdf-caminho-vazio-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(name):
 with gzip.open(out/name,'rt') as f:return json.load(f)
def compare(folder):
 r=read(folder+'/results.json.gz');pairs={}
 for x in r['rows']:pairs.setdefault(x['case'],{})[x['binary']]=x
 eligible={n:v for n,v in pairs.items() if all(x['exit']==0 for x in v.values())}
 return dict(total_cases=len(pairs),eligible_cases=len(eligible),pdf_pixels_match=sum(v['tekt']['pixel_sha256']==v['vanilla']['pixel_sha256'] for v in eligible.values()),compact_pixels_match=sum(v['compact']['pixel_sha256']==v['vanilla']['pixel_sha256'] for v in eligible.values()),pdf_svg_blue_bbox_match=sum(v['tekt']['blue_bbox']==v['tekt-svg']['blue_bbox'] for v in eligible.values()),errors={n:{k:x['commands'][-1]['stderr'] for k,x in v.items() if x['exit']} for n,v in pairs.items() if any(x['exit'] for x in v.values())})
pre=read('before-source.json.gz');final=read('candidate-final.json.gz');checks=json.loads((out/'followup-summary.json').read_text());suite=json.loads((out/'workspace-result.json').read_text());assert suite['exit']==0 and suite['sources_unchanged'];assert all(sha(n)==v['sha256'] for n,v in final['files'].items())
cmp={n:compare(n) for n in ['before-full','after']}
before=read('before-full/results.json.gz');after=read('after/results.json.gz')
previous={(x['case'],x['binary']):x for x in before['rows']};svg_unchanged=[x['case'] for x in after['rows'] if x['binary']=='tekt-svg' and x['exit']==0 and x['pixel_sha256']==previous[x['case'],x['binary']]['pixel_sha256']];cmp['svg_unchanged']=svg_unchanged
qpdf=[c for r in after['rows'] for c in r['commands'] if c['command'][0]=='qpdf'];original=read('original-block/results.json.gz');qpdf += [c for r in original['rows'] for c in r['commands'] if c['command'][0]=='qpdf'];assert all(c['exit']==0 for c in qpdf);cmp['final_qpdf_checks']=len(qpdf)
(out/'comparison.json').write_text(json.dumps(cmp,indent=2))
(out/'current.patch').write_text(''.join(''.join(difflib.unified_diff(pre['files'][n]['text'].splitlines(True),v['text'].splitlines(True),fromfile=n,tofile=n)) for n,v in final['files'].items()))
repro=out/'reproduction';repro.mkdir(exist_ok=True)
for name in ['probe','setup','seal','validate','suite','original','report']:
 p=Path('/tmp/tekt-pdf-empty-'+name+'.py');(repro/p.name).write_bytes(p.read_bytes())
p=Path('/tmp/tekt-pdf-empty-tests.txt');(repro/p.name).write_bytes(p.read_bytes())
(repro/'README.txt').write_text('Scripts históricos. Não reaplicar setup/seal ao estado corrigido. current.patch contém os testes e os dois ramos produtivos alterados. before/ é tentativa parcial interrompida pelo erro de polygon(stroke:none); before-full/ conserva o corpus integral, com erros explícitos e proveniência. after/ e original-block/ usam o candidato selado. Nenhuma comparação conta erro de compilação como coincidência visual.\n')
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name!='evidence.json.gz':entries.append(dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:r=json.load(f)
assert all(hashlib.sha256(base64.b64decode(x['base64'])).hexdigest()==x['sha256'] for x in r['entries'])
report=f'''# PDF: isolamento de caminhos sem pintura — 2026-10-14

Corrigido o vazamento do caminho de Shape sem fill/stroke para a pintura ou
recorte seguinte. O PDF já não pinta o backing invisível do controle de Block
nem amplia o clip de um irmão. O reparo é exclusivamente do exportador L3;
outset/overflow do layout e rejeição de polygon(stroke:none) continuam abertos.

## Proveniência

HEAD `{pre['head'].strip()}`, working tree não commitado. before-source,
red-state, candidate-final e workspace-pre preservam fontes, hashes, UTC,
status e git diff HEAD --stat. Suíte completa: {suite['start']} →
{suite['end']}. Fontes mantiveram hashes durante testes e validação.
Nenhum commit, push, PR, publicação ou release foi realizado.

Referência vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Os probes conservam argv, fontes, stdout/stderr, UTC, hash de cada binário,
artefato e imagem. A rasterização usa 2px/pt, sem recortar ou realinhar a saída.
A versão textual não substitui proveniência.

## Medição antes da decisão

O lote anterior block-regiao-2026-10-14/open-pdf flagrou um retângulo azul extra
no PDF com Block(outset) sem fill/stroke, ausente no SVG. A sonda isolada desta
rodada separou exportação de layout: Place mantém as formas fora do fluxo,
e Rect/Ellipse/RoundedRect invisíveis precedem um marcador azul. O PDF Tekt
pinta desde (20,20), em vez de somente o marcador (70,60). A falha ocorre nos
modos normal e compacto, atravessa Group e amplia a máscara posterior.

Na fonte anterior draw.rs:513–517 e :922–926, ambos os dispatchers construíam
o caminho, executavam um ramo vazio quando fill/stroke eram None e fechavam Q.
A geometria pendente entrava no operador de pintura ou clip seguinte. Os cinco
REDs observaram precisamente isso: retângulo invisível participava do conjunto
pintado ou do conjunto recortado. Não se infere intenção histórica do código.

O reparo termina o caminho sem pintar nos dois ramos, preservando envelopes,
ordem, tags, recursos, transformações e todos os ramos com pintura efetiva.
Não modifica o produtor L1 para esconder a falha L3. O owner
00_nucleo/prompts/infra/export/stream/draw.md foi atualizado antes do código;
resselo restrito ao par 1:1 com draw.rs, sem núcleo novo ou contrato público.
Correção interna ADR-0127 em fluxo contínuo.

## Resultados e limites

O corpus tem dez fontes, em PDF normal/compacto, PDF vanilla e SVG dos dois
binários. Nove fontes chegam à exportação em todas as variantes. Uma fonte,
polygon(stroke:none), é rejeitada pelo constructor Tekt antes do exportador;
essa falha permanece explícita, sem ser contada como coincidência visual.
A primeira tentativa do probe parou nesse erro; o corpus completo foi refeito
com registro das falhas. Ambos os registros estão preservados.

Entre os nove casos exportáveis, pixels PDF idênticos ao pin passaram de
**{cmp['before-full']['pdf_pixels_match']}/9 para {cmp['after']['pdf_pixels_match']}/9**
no normal, e de **{cmp['before-full']['compact_pixels_match']}/9 para
{cmp['after']['compact_pixels_match']}/9** no compacto. Isso inclui fill/stroke,
clip, grupo aninhado, saída de Group, formas invisíveis e controles positivos.
Os {len(svg_unchanged)} SVGs Tekt exportáveis permaneceram pixel a pixel iguais
antes/depois. Não é uma exigência de igualdade de bytes PDF nem uma prova de
paridade universal.

Os {len(qpdf)} PDFs finais dos dois conjuntos passaram qpdf. No controle
original de Block, o retângulo extra desapareceu: a tinta agora fica só no
marcador em (17,43), igual ao SVG Tekt. O pin usa (17,35), portanto a diferença
de fluxo de outset ainda está aberta. O clip de Box/outset agora também deixa
de ganhar a região invisível no PDF; a origem e expansão próprias de outset
continuam incorretas no layout. Overflow na raiz conserva a página vazia extra.

Cinco testes de regressão foram observados em RED e passaram em GREEN. O
interpretador de teste acompanha quais retângulos entram em pintura/clip;
não exige a string n e aceitaria outra implementação geometricamente correta.
As variantes cobrem top/local, saída de Group, clip posterior, três operações
de pintura e Compact/Verbose. As imagens finais de clip foram inspecionadas.

Suíte completa: **{suite['passed']} passaram, {suite['failed']} falharam,
{suite['ignored']} ignorados**. Build offline/locked, rustdoc -D warnings,
all-target check, fmt, diff check, V1/V5/V15/V26 estritos e reverse lineage
passaram. Inventário Clippy: {checks['clippy_after']} avisos, sem aumento.
Linter completo: zero erros, {checks['architecture']['warning']} warnings e
{checks['architecture']['note']} notes. As duas notes V19 novas estão no
interpretador de teste: alternativas f/S/B e operadores de estado sem pintura.
As três operações de pintura são exercitadas explicitamente. As notas ficam
registradas; não se afirma zero violations nem aprovação de Clippy -D warnings.

Esta falha confirmada era de código/integração do exportador. O L0 ganhou a
invariante explícita e seus refutadores antes do reparo; isso não determina a
proporção de falhas de código versus prompts no projeto inteiro. CI remoto,
release e demais lacunas de produção continuam abertos; objetivo permanece ativo.

## Evidências preservadas

[evidence.json.gz](pdf-caminho-vazio-2026-10-14/evidence.json.gz) contém
{len(entries)} entradas com caminho, SHA-256 e bytes base64, relidas e verificadas.
SHA-256 do arquivo: `{sha(out/'evidence.json.gz')}`.
Inclui estados, patch, RED/GREEN, gates/SARIF, suíte completa, SVG/PDF/PNG,
resultados e scripts históricos. Não reaplicar os scripts de mutação.
'''
out.with_suffix('.md').write_text(report)
print(json.dumps(dict(report=str(out.with_suffix('.md')),entries=len(entries),archive_sha256=sha(out/'evidence.json.gz'),suite={k:suite[k] for k in ['passed','failed','ignored','sources_unchanged']},comparison=cmp),indent=2))
