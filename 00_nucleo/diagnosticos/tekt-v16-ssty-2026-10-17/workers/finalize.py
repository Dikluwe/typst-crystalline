from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();v=json.load(open(out/'validation-result.json'));assert v['complete'];hashes={p:sha(p) for p in set(b['hashes'])|set(scope['nuclei'])};assert hashes==v['source_hashes'];assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];before=json.load(open(out/'before-probe-result.json'));after=json.load(open(out/'after-probe-result.json'));assert before['structural_guarded_sites']==0 and after['structural_guarded_sites']==2;assert before['results'][0]['runtime']==after['results'][0]['runtime'];w=json.load(open(out/'workspace/workspace-result.json'));assert (w['passed'],w['failed'],w['ignored'],w['exit'],w['sources_unchanged'])==(8921,0,7,0,True);arch=json.load(open(out/'architecture-delta.json'));assert arch['after']=={'V16:error':115,'V19:note':1088,'V20:note':873};assert json.load(open(out/'integrity-result.json'))['exact_replay'];resources=[];events=collections.Counter()
for p in sorted(out.glob('*-resources.json')):
 r=json.load(open(p));assert r['memory_before']['memory.max']==str(12*1024**3) and r['memory_before']['memory.high']==str(10*1024**3) and r['memory_before']['memory.swap.max']=='0';a=dict(x.split() for x in r['memory_before']['memory.events'].splitlines());z=dict(x.split() for x in r['memory_after']['memory.events'].splitlines());events.update({k:int(z[k])-int(a[k]) for k in z});resources.append(dict(path=str(p),record=r))
peak=max(int(r['record']['memory_after']['memory.peak']) for r in resources);assert events['max']==events['oom']==events['oom_kill']==0
binaries={name:dict(path=p,sha256=sha(p)) for name,p in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]};assert binaries['vanilla']['sha256']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
state=dict(utc=now(),head=b['head'],source_hashes=hashes,binaries=binaries,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),scope=scope,architecture=arch,workspace=w,memory=dict(peak_bytes=peak,peak_gib=peak/1024**3,events=dict(events),executions=resources),objective='active')
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'changes.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'].get(p,'').splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in sorted(set(scope['sources'])|set(scope['owners'].values())|set(scope['nuclei']))))
rows=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];remaining=[r for r in rows if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');(out/'inventario-v16-restante.md').write_text('# V16 restantes\n\nMedição: '+state['utc']+', HEAD '+b['head']+', working tree não commitado; source freeze/status/diffstat em final-state.json.gz.\n\n'+''.join('- `'+r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')+':'+str(r['locations'][0]['physicalLocation']['region']['startLine'])+'` — '+r['message']['text'].replace('\n',' ')+'\n' for r in remaining))
(out/'follow-ups.md').write_text('''# Continuação

- Restam 115 V16, listados em inventario-v16-restante.md e remaining-v16.json, com proveniência no fechamento. Retomar lotes por domínio; preservar os V19 uniformes com prescrição/cobertura, sem wildcard genérico ou supressão.
- Os 12 cenários comparativos em três classes (string incompleta, hexadecimal e morfologia de include), o contrato document.format, três âncoras matemáticas, RG1/FG1/RM1/SY1 e igualdade de std entre arquivos continuam abertos conforme os relatórios anteriores.
- RG1 permanece pendente de aprovação e seus candidatos precisam de rebase sobre o estado final atual; não copiá-los integralmente.
- Sem commit/push/merge/release; objetivo global ativo.
''')
(out/'relatorio.md').write_text(f'''# Relatório — V16 de MathSize/ssty

Etapa concluída. V16 **117 → 115**, V19 **1086 → 1088 notas**, V20 **873 → 873 notas**. Os dois fallbacks de ssty foram tornados nominais. O gate global continua bloqueado pelos 115 V16 restantes; esta etapa não declara o objetivo global concluído.

## Correção e decisão

Em 03_infra/src/font_metrics/math.rs::ssty_level_of e 03_infra/src/shaper/shaping.rs::ssty_level_for_subrun, Display e Text agora compartilham explicitamente None. Script conserva Some(1), ScriptScript conserva Some(2). Os gates anteriores de math, math_text_item e elegibilidade permanecem byte a byte. Shaping continua usando o predicado canônico; sua seleção de fontes, cache, features, glifos e emissão não mudou.

Dois pares source/L0 e o novo núcleo fonts/ssty-math-size.toml pinado compartilham a obrigação. L0 foi escrito antes da materialização. É manutenção interna em fluxo contínuo ADR-0127, sem alteração de MathSize, TextStyle, contrato público, default ou fase. O replay permite somente os complementos nominais, o import privado/prefixos de MathSize e o hash de linhagem; todos os outros arquivos protegidos foram conferidos. Não se alega melhoria medida de leitura por IA nem nova paridade com o vanilla.

As duas notas V19 são intencionais: ambos os tamanhos não solicitam ssty e são exercitados separadamente. Permanecem visíveis. Uma futura variante não recebe None silenciosamente: o compilador exige revisão. Não houve supressão nem alteração de severidade/configuração.

## Evidências

- RED estrutural anterior: before-guard-result.json falha porque o mutante MathSize::TektSstyFuture compila nos dois matches, com zero E0004 selecionados. Os controles atuais já passam; não houve falha funcional nova inventada.
- GREEN: a mesma sonda rejeita a variante futura com exatamente dois E0004, um em cada função auditada. Os 224 resultados dos quatro tamanhos e dos gates de math/TextItem/elegibilidade permanecem idênticos. Métricas cobre 32 entradas; shaping cobre 192 com 12 textos (vazio, ASCII, Unicode, números/decimal, combinações e não elegíveis).
- A sonda copia as funções e a declaração reais; modela somente os três campos de TextStyle lidos. Isso prova a proteção estrutural local e seus valores de controle, sem substituir shaping integral. O enum produtivo não foi alterado nem a árvore real mutada pela sonda.
- Testes reais focalizados ssty passaram. A suíte completa registra {w['passed']} aprovados, {w['failed']} falhas e {w['ignored']} ignorados, sem novo teste produtivo que apenas espelhe o match.
- Build, fmt, Clippy workspace/all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito e quatro índices, V1/V5/V15/V26, reverse lineage, replay e diff --check passaram. O SARIF diferencial confirma somente os dois V16 retirados e os dois V19 prescritos; demais diagnósticos preservados por fonte/regra/severidade/mensagem, sem V20 novo.
- Recursos sequenciais: MemoryMax12GiB, MemoryHigh10GiB, swap zero, Cargo jobs1. Pico {peak/1024**3:.6f} GiB ({peak} bytes); eventos {dict(events)}. Sem alcance de MemoryMax, OOM ou OOM kill.

## Proveniência e continuidade

HEAD `{b['head']}`, working tree não commitado. Baseline `{b['utc']}`, fechamento `{state['utc']}`. before.json.gz conserva textos/fontes/L0/status/diff HEAD --stat e os {len(b['hashes'])} hashes verificados da etapa import-syntax. final-state.json.gz congela {len(hashes)} arquivos, status/diffstat e binários. As sondas guardam rustc, argv, programas, saídas e hashes; validation-result.json registra comandos, UTC e source freeze. O linter é o pin 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 {b['linter']['sha256']}. O alvo vanilla é o commit 9f2b6e8715237cb086899a42873660fe744622e8; o binário tem SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. É proveniência do pin, sem alegar medição nova de paridade deste reparo mecânico.

inventario-v16-restante.md organiza os 115 locais restantes. follow-ups.md preserva as frentes de linguagem e propostas pendentes. Os pacotes TS1 aprovado e RG1 pendente são conferidos pela entrega. Sem commit, push, merge ou release.
''');print('Closed',arch['after'],'tests',w['passed'],'peak',peak)
