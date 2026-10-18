from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,difflib,collections,re
p=Path(__file__).resolve().parent
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
load=lambda f:json.loads(gzip.decompress((p/f).read_bytes()))
base=load('before-sources.json.gz');summary=json.loads((p/'complete-summary.json').read_text());freeze=load('complete-freeze.json.gz');lint=load('lint-diff.json.gz')
assert all(sha(f)==h for f,h in freeze['sources'].items()),'source changed after validation'
assert all(x['exit']==0 for x in summary['results'] if x['label']!='architecture')
patch=[]
for f,v in base['files'].items():patch.extend(difflib.unified_diff(v['text'].splitlines(True),Path(f).read_text().splitlines(True),fromfile='before/'+f,tofile='after/'+f))
(p/'change.patch.gz').write_bytes(gzip.compress(''.join(patch).encode()))
resources={f.name:json.loads(f.read_text()) for f in p.glob('*-resources.json')}
peak=max(int(v['memory_after']['memory.peak']) for v in resources.values());events=collections.Counter()
for v in resources.values():events.update({a:int(b) for a,b in (line.split() for line in v['memory_after']['memory.events'].splitlines())})
assert events['oom']==events['oom_kill']==0
(p/'resources.json.gz').write_bytes(gzip.compress(json.dumps(resources,indent=2).encode()))
(p/'evidence.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'freeze':freeze,'summary':summary,'logs':{f.name:f.read_text() for f in p.glob('*.log')}},indent=2).encode()))
state={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'sources':freeze['sources'],'binary_sha256':summary['binary_sha256']}
(p/'final-state.json.gz').write_bytes(gzip.compress(json.dumps(state,indent=2).encode()))
doc=(p/'complete-doctest.log').read_text();docs=sum(map(int,re.findall(r'test result: ok\. (\d+) passed;',doc)))
text=f'''# R1 — reflexão de Sequence e conteúdo vazio

Etapa concluída na working tree não commitada do HEAD `{state['head']}`. Freeze de validação: **{freeze['utc']}**. Vanilla pinado em `bd5ebec285505889e8ff1d4f68804ab147a699f3`; SHA do oráculo `{freeze['oracle_sha256']}`. Binário debug Tekt validado: `{summary['binary_sha256']}`. O diffstat e a lista exata de arquivos alterados estão no freeze/final-state; o patch desta etapa está separado do trabalho A1 e da correção de pilha.

## Correções

- `children` passou a ser campo conhecido de Empty e Sequence, inclusive vazio. O array conserva os filhos imediatos, sua ordem e seus wrappers.
- `fields()` inclui somente `children` para essas duas variantes. `has()` e `at()` usam a mesma projeção do getter.
- A identidade de linguagem do conteúdo vazio passou de `empty` para `sequence`, corrigindo o nome retornado por `func()` e as mensagens de campo inexistente. A assinatura pública e o Debug Rust foram preservados.
- Text vazio continua texto, Styled comum permanece opaco, aliases conservam os conteúdos anteriores e sequências com um filho mantêm sua identidade.

Os quatro L0s proprietários foram atualizados antes do código. A primeira tentativa de teste falhou por escopo de MockWorld, sem crédito de RED; o harness foi corrigido. O RED funcional registrou quatro falhas e um controle verde antes da implementação. Os mesmos cinco testes passaram depois. As 10 sondas bilaterais de reparo e os dois controles de Text/Styled foram aprovados na auditoria final. Sondas e fontes exatas estão em before/after e before-sources. Hashes foram ressellados após ownership/núcleos válidos. Nenhum contrato Rust público novo ou fase foi introduzido.

## Validação e RAM

**{summary['passed']} testes passaram, {summary['failed']} falharam, {summary['ignored']} ignorados; {docs} doctests passaram.** Build workspace, Clippy de todos os targets com warnings proibidos, Rustdoc com warnings proibidos, fmt, V1/V5/V15/V26, linhagem reversa e diff-check passaram. Doctests foram executados em invocação separada conforme a receita validada anteriormente.

Todas as execuções pesadas foram sequenciais, com um job Cargo, **MemoryMax 12 GiB, MemoryHigh 8 GiB e swap zero**. Pico da etapa: **{peak/1024**3:.3f} GiB** ({peak} bytes). Eventos acumulados: `{dict(events)}`. Esta medição é de cgroup; não demonstra ganho de RSS do produto. Receita/horários/ambiente/comandos e resultados estão em resources/evidence.

Linter global: `{lint['after']}`. O diferencial por regra/consumer/mensagem não adicionou erros ou warnings; os V16 anteriores continuam bloqueando o gate global. As notas V19/V20 não foram silenciadas.

## Pendências explícitas

R1 não encerra a reflexão completa de Sequence. As sondas ainda registram diferenças na chamada do construtor retornado por func(), em selector/counter e na mensagem de método inválido. R2 está nomeada nos L0s para completar construção e admissão em selector/where/show/counter, após medição dos argumentos e identidades; nenhuma função será admitida apenas pelo nome. As sondas JSON foram preservadas, mas comparação do texto diagramado não foi usada como prova de igualdade do JSON.

As pendências A1 e de pilha documentadas nas etapas anteriores, os V16 restantes e os demais lotes de paridade continuam abertos. Este relatório encerra apenas R1. Sem commit, push, publicação ou release nesta etapa.

Evidências: before.json.gz, after.json.gz, before-sources.json.gz, complete-freeze.json.gz, change.patch.gz, evidence.json.gz, resources.json.gz, lint-diff.json.gz, final-state.json.gz e manifest.sha256, neste diretório.
'''
(p/'relatorio.md').write_text(text)
files=sorted(f for f in p.iterdir() if f.is_file() and (f.suffix in ['.py','.md'] or f.name.endswith('.json.gz') or f.name.endswith('.patch.gz') or f.name=='.gitignore'))
(p/'manifest.sha256').write_text('\n'.join(f'{sha(f)}  {f.name}' for f in files)+'\n')
print(json.dumps({'report':str(p/'relatorio.md'),'tests':summary['passed'],'doctests':docs,'peak_gib':peak/1024**3,'manifest_sha256':sha(p/'manifest.sha256')}))
