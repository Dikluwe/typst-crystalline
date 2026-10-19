from pathlib import Path
import json,gzip,hashlib,base64,datetime,subprocess,shutil
out=Path('00_nucleo/diagnosticos/tiling-juncoes-2026-10-14');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
with gzip.open(out/'probe.json.gz','rt') as f:probe=json.load(f)
assert all(sha(p)==x['sha256'] for p,x in probe['files'].items())
versions=[]
for cmd in [['pdftoppm','-v'],['pdftocairo','-v'],['mutool','-v'],['qpdf','--version']]:
 p=subprocess.run(cmd,capture_output=True,text=True);versions.append(dict(command=cmd,exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
checks=[]
for pdf in sorted(out.glob('*.pdf')):
 if '.qdf.' in pdf.name:continue
 p=subprocess.run(['qpdf','--check',str(pdf)],capture_output=True,text=True);assert p.returncode==0,(pdf,p.stderr);checks.append(dict(path=str(pdf),sha256=sha(pdf),exit=p.returncode,stdout=p.stdout,stderr=p.stderr))
(out/'verification.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),versions=versions,checks=checks,productive_sources_unchanged=True),indent=2))
for p in Path('/tmp').glob('tekt-seams-*.py'):shutil.copyfile(p,out/p.name)
entries=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name not in ['evidence.json.gz','evidence.sha256','report-result.log']:
  data=p.read_bytes();entries.append(dict(path=str(p.relative_to(out)),sha256=hashlib.sha256(data).hexdigest(),base64=base64.b64encode(data).decode()))
with gzip.open(out/'evidence.json.gz','wt') as f:json.dump(dict(entries=entries),f)
with gzip.open(out/'evidence.json.gz','rt') as f:again=json.load(f)['entries']
for e in again:assert hashlib.sha256(base64.b64decode(e['base64'])).hexdigest()==e['sha256']==sha(out/e['path'])
digest=sha(out/'evidence.json.gz');(out/'evidence.sha256').write_text(digest+'  evidence.json.gz\n')
Path('00_nucleo/diagnosticos/tiling-juncoes-2026-10-14.md').write_text(f'''# Junções de tiling PDF: isolamento entre observadores

O PDF multicolorido produzido pela implementação atual é uniforme no MuPDF e
no Cairo, mas apresenta junções no caminho padrão de pdftoppm. Os controles
abaixo isolam o comportamento na rasterização do padrão por esse observador.
Isso refina a hipótese do relatório tiling-overlap: a marca vista em pdftoppm
não prova, isoladamente, uma falha semântica do arquivo ou do exporter.
A compatibilidade visual com esse observador continua aberta; não há alteração
produtiva nem certificação integral do pacote T2/T3 nesta investigação.

## Proveniência e método

HEAD `{probe['head'].strip()}`, working tree não commitado, UTC inicial
`{probe['start']}`. probe.json.gz preserva status, diffstat, fontes e L0 lidos,
além dos comandos/hashes de cada PDF e raster. O PDF de entrada tem SHA-256
`{probe['source_pdf_sha256']}` e vem de
`tiling-overlap-2026-10-14/final-renders/results.json.gz`, cujo snapshot é
candidate-final.json.gz. verification.json registra versões e qpdf de todas
as variantes; os fontes produtivos permaneceram idênticos.

Medição a 144 DPI, caixa autoral x=[40,120), y=[40,100), 4.800 pixels. O valor
analítico das quatro camadas a 50% é (159,375;63,75;79,6875). Não há recorte
por erro, realinhamento, mudança de tolerância ou comparação de bytes como
paridade de linguagem. As mutações QDF são sondas diagnósticas, não um caminho
de produção nem permissão para gerar uma grade de clones no layout.

## Resultados

| Controle | pdftoppm | MuPDF | Cairo |
|---|---|---|---|
| Pattern atual | quatro cores de pixels; máximo 7,75, p95 4,75 | uniforme (159,64,80) | uniforme (159,63,79) |
| Mesmas células desenhadas numa grade finita, sem Pattern | uniforme (158,62,78) | uniforme (158,64,79) | uniforme (159,63,79) |
| Quatro camadas retangulares inteiras, sem célula ou Pattern | uniforme (158,62,78) | uniforme (158,64,79) | uniforme (159,63,79) |

Desabilitar antialias vetorial em pdftoppm não elimina as junções. TilingType
2 ou 3 também não altera o resultado. Colocar o corpo diretamente no Pattern,
sem Form e com o mesmo clip, conserva as junções. Acrescentar grupo de
transparência isolado/não isolado à célula também não resolve e altera o
arredondamento em outros observadores. Essas hipóteses foram refutadas.
Todos os PDFs de controle passam qpdf; isso verifica estrutura, não qualidade
visual nem conformidade integral PDF/A ou PDF/UA.

## Decisão baseada na medição

Não mudar o L0 ou o exporter para inserir grupos, trocar TilingType ou gerar
uma grade: nenhuma dessas hipóteses corrige a causa medida. Conservar a célula
vetorial única e a composição explícita do período. Não chamar a ausência de
marcas em um único observador de prova universal; a observação multilateral
mais os controles geométricos sustentam a classificação como diferença do
caminho de rasterização de padrões de pdftoppm neste ambiente.

A inferência seria refutada por uma diferença geométrica/compositiva entre os
arquivos ou reprodução equivalente em observadores independentes. Uma análise
futura do código do rasterizador ou um caso mínimo externo poderá determinar
a causa interna exata. Não atribuir o comportamento a uma versão upstream
específica só pela saída local, e não publicar issue sem autorização.

A próxima correção produtiva deve atacar falhas já comprovadas do próprio
projeto, incluindo o fallback de gradientes PNG, mantendo esta compatibilidade
registrada. Os avisos de CI (181 Rust/Clippy e 382 Tekt) permanecem inalterados;
sua correção faz parte da meta geral. Nenhuma release foi publicada.

## Evidência recuperável

Arquivo evidence.json.gz com {len(entries)} entradas path/sha256/base64,
reabertas e verificadas contra os arquivos. SHA-256 `{digest}`.
''')
print(json.dumps(dict(entries=len(entries),sha256=digest,productive_changes=False,report='00_nucleo/diagnosticos/tiling-juncoes-2026-10-14.md'),indent=2))
