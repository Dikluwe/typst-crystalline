from pathlib import Path
import gzip,json,hashlib,base64,datetime,difflib,shutil,re
base=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14');out=base/'shape-flow';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
pre=json.load(gzip.open(out/'validation/pre-validation.json.gz'));assert {p for p,x in pre['files'].items() if sha(p)!=x['sha256']} == {'03_infra/fixtures/p307b/MANIFEST.md'}
suite=json.loads((out/'workspace/workspace-result.json').read_text());assert suite['exit']==101 and suite['sources_unchanged'] and suite['failed']==2
checks=json.loads((out/'validation/followup-validation-results.json').read_text());assert len(checks)==9 and all(x['exit']==0 for x in checks)
summary=json.loads((out/'validation/followup-summary.json').read_text());assert summary['sources_unchanged']
gen=json.loads((out/'snapshots/generations.json').read_text());assert len(gen)==2 and gen[0]['hashes']==gen[1]['hashes'] and all(x['exit']==0 for x in gen)
normal=(out/'snapshots/normal.log').read_text();assert '9 passed; 0 failed' in normal
comparison=json.loads((out/'snapshots/comparison.json').read_text());assert all(x['text_words_identical'] for x in comparison['rows'])
folders=['shape-terminal-complete','shape-flow-before','shape-flow-after','shape-flow-pdf','shape-dim-flow-after','after-shape-flow','shape-block-terminal-before']
probes={n:json.load(gzip.open(base/n/'results.json.gz')) for n in folders};assert all(x['source_guard'] for x in probes.values())
old=json.load(gzip.open(out/'before-sources.json.gz'));diff=''
for path,x in old['files'].items():
 a=base64.b64decode(x['base64']).decode();b=Path(path).read_text();diff+=''.join(difflib.unified_diff(a.splitlines(True),b.splitlines(True),fromfile=path+' (before)',tofile=path))
(out/'focused.patch').write_text(diff)
(out/'review.md').write_text(f'''# Fronteira vertical de Shape e espaço forte

Correção implementada no fluxo principal de Shape e na manutenção do fim
confirmado após VSpace forte. A integração geral de Block e espaços fracos
continua incompleta e refutada; não é uma certificação de produção.

## Proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Os JSONs preservam UTC, status/diffstat exatos e hashes de fontes/binários.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, receita P1489,
árvore 6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, binário SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
`before-sources.json.gz` preserva bytes dos três consumers e owners antes
 desta etapa; `focused.patch` separa o lote do restante da árvore suja.

## Medição, decisão e reparo

`shape-terminal-complete` mede o fundo indevido da forma isolada, a segunda
forma deslocada pela cap-height e leitores usando prev_line_baseline antiga.
`shape-flow-before` amplia a 32 documentos, incluindo espaço forte/fraco,
texto, transformações e página explícita. A tentativa `shape-terminal-before`
abortou por uma suposição errada do harness sobre o template de saída da CLI;
não foi usada como corpus completo. A CLI Tekt conserva literalmente {{p}}
na saída SVG desse caso; o harness corrigido registra o caminho efetivo.

L0 atualizado antes de código. As quatro regressões em
`implementation/shape-flow-red.log` falharam; `shape-flow-green.log` passou
as quatro. Shape passou a publicar fim físico em prev_line_baseline e
last_block_descent_y; o topo seguinte usa esse fim e o gap, sem descontar
cap-height. A transição de página recupera a origem física. Subframes,
dimensões, pintura e estilos mantêm seus caminhos. VSpace forte mantém o
fundo confirmado e desloca a baseline da cadeia pelo mesmo amount; não
soma o below pendente como conteúdo. Nenhum contrato público ou fase mudou.

## Evidência posterior e limites

`shape-flow-after` mede os mesmos 32 documentos em SVG/PNG; `shape-flow-pdf`
cobre PDF. Forma isolada, texto adjacente, duas ordens Shape/Rotate, espaço
forte terminal/intermediário, parbreak e quebra explícita recuperaram a
geometria esperada. `shape-dim-flow-after` conserva os 20 controles de
dimensões e corrige também a página auto de 150×150pt do caso unbounded.
As 74 consultas/diagnósticos e renders anteriores estão em `after-shape-flow`;
seus erros esperados não são falhas novas.

Não exigir pixels idênticos como substituto de geometria: formas consecutivas
coincidem em SVG/PDF, mas o PNG vanilla encaixa as bordas em pixels inteiros
onde Tekt aplica suavização. O registro `raster-edge-observation.json`
preserva os pixels lidos; essa divergência não motivou alteração no rasterizador.
Elipses conservam diferenças anteriores. Alguns PDFs têm diferença de um
nível por canal; não se afirma igualdade binária com vanilla.

Pendências confirmadas: weak VSpace entre elementos ainda tem prioridade de
colapso errada; VSpace negativo é rejeitado; dimensões None/Auto e identidade
circle/square continuam incompletas; RTL permanece aberto. Block deixa o
fundo do filho escapar em vez de publicar o fim de sua própria alocação:
`shape-block-terminal-before` mede 20 controles de Shape/Text/Empty/Rotate,
com inset e altura explícita. Shape em Block height40 termina página em30pt,
quando a alocação exige60pt; inset5 produz35pt em vez40pt. Isso também ocorre
com Rotate e deve ser reparado no owner Block. Não encerrar B2b nem usar
suíte verde como prova de correção desses refutadores.

## Testes, snapshots e CI

A execução completa de {suite['start']} a {suite['end']} terminou com
{suite['passed']} testes aprovados, duas falhas de snapshots e
{suite['ignored']} ignorados. As fontes ficaram estáveis. Os nove comandos
em validation/ passaram: build, rustdoc estrito, check all-targets, inventário
Clippy, fmt, linhagem estrita, reversa, relatório arquitetural e diffcheck.
Clippy permanece com {summary['clippy_after']} avisos; Tekt com
{summary['architecture'].get('warning',0)} warnings,
{summary['architecture'].get('note',0)} notes e zero errors. Não é CI -D warnings verde.
Os 16 testes Python de tools/ci também passaram.

Somente snapshots 04/07 foram regenerados segundo o contrato já vigente de
export-fixtures, após justificação registrada e backup. Duas gerações foram
idênticas; qpdf passou, palavras extraídas foram preservadas e objetos/streams
foram comparados. Os outros sete PDFs/fontes/harness/L0 ficaram intactos.
Os nove snapshots passaram sem UPDATE. Essa reexecução dirigida resolve as
duas falhas de referência; não se apresenta como uma nova suíte completa
com todos os testes executados após a manutenção. MANIFEST registra os hashes.

A meta geral permanece aberta. Não houve commit, push ou publicação/release.
''')
paths={p for p in out.rglob('*') if p.is_file()}
for n in folders:paths.update(p for p in (base/n).rglob('*') if p.is_file())
paths.update(p for p in (base/'implementation').glob('shape-flow*') if p.is_file())
for p in old['files']:paths.add(Path(p))
paths.add(Path('03_infra/fixtures/p307b/MANIFEST.md'))
for p in Path('03_infra/fixtures/p307b/reference').glob('*.pdf'):paths.add(p)
recipes=out/'recipes';recipes.mkdir(exist_ok=True)
for p in list(Path('/tmp').glob('tekt-shape-flow-*.py'))+[Path('/tmp/tekt-shape-terminal-probes.py'),Path('/tmp/tekt-shape-block-terminal.py'),Path('/tmp/tekt-transform-run.py'),Path('/tmp/tekt-seal-owned.py')]:
 d=recipes/p.name;shutil.copyfile(p,d);paths.add(d)
for p in Path('/tmp').glob('tekt-shape-flow-*.log'):
 d=out/p.name;shutil.copyfile(p,d);paths.add(d)
entries=[dict(path=str(p),sha256=sha(p),base64=base64.b64encode(p.read_bytes()).decode()) for p in sorted(paths)]
a=out/'evidence.json.gz';assert not a.exists()
with gzip.open(a,'wt') as f:json.dump(entries,f)
for e in json.load(gzip.open(a)):
 b=base64.b64decode(e['base64']);assert hashlib.sha256(b).hexdigest()==e['sha256'] and b==Path(e['path']).read_bytes()
receipt=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),path=str(a),sha256=sha(a),entries=len(entries),reopened_and_compared=True)
(out/'receipt.json').write_text(json.dumps(receipt,indent=2));print(json.dumps(receipt),flush=True)
main=base.with_suffix('.md');main.write_text(main.read_text()+f'''\n\n## Fronteira vertical de Shape — avanço com pendências explícitas\n\n`shape-flow/review.md` registra quatro regressões RED→GREEN, posição/fim\ncausal das formas e manutenção de espaço forte. Suíte: {suite['passed']}\naprovados, duas falhas de referências deliberadamente alteradas, {suite['ignored']}\nignorados. Somente 04/07 foram regeneradas após comparação independente;\nos nove snapshots passaram depois sem UPDATE. Verificações de linhagem,\nbuild/doc/check/fmt passaram. Block/inset/altura e espaços fracos continuam\nrefutados; a meta geral e CI/release não estão concluídos.\n\nArquivo recuperável: `shape-flow/evidence.json.gz`, SHA-256\n`{receipt['sha256']}`, {receipt['entries']} entradas reabertas e comparadas.\n''')
