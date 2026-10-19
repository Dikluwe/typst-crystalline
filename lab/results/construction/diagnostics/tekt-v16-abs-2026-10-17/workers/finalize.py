from pathlib import Path
import datetime,gzip,hashlib,json,collections,difflib,subprocess
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));assert v.get('complete') and v['sources_unchanged'];assert all(sha(p)==h for p,h in v['source_hashes'].items());assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head']
changed=[p for p,h in b['hashes'].items() if sha(p)!=h];assert set(changed)==set(scope['sources'])|set(scope['owners'].values())
packages={'table-row-groups-2026-10-17':'a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9','text-size-length-2026-10-17':'771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba'}
for name,h in packages.items():
 p=Path('00_nucleo/materialization')/name;assert sha(p/'SHA256SUMS')==h
 for line in (p/'SHA256SUMS').read_text().splitlines():want,file=line.split('  ',1);assert sha(p/file)==want,file
 ps=json.load(open(p/'scope.json'))
 if name.startswith('text-size'):assert all(sha(p)==h for p,h in ps['source_hashes'].items())
 else:
  protected=ps['semantic_sources']+ps['metadata_only_sources']+list(ps['owners']);assert all(sha(p)==b['hashes'][p] for p in protected)
red=json.load(open(out/'red-probe-result.json'));green=json.load(open(out/'green-probe-result.json'));assert [r['exit'] for r in red['results']]==[0,0];assert [r['exit'] for r in green['results']]==[0,1] and green['results'][1]['codes']==['E0004'];assert green['source_sha256']==sha(scope['sources'][0])
assert '4 passed; 0 failed' in (out/'controls-before.log').read_text() and '4 passed; 0 failed' in (out/'green.log').read_text();obs=json.load(open(out/'observables-result.json'));assert obs['pairs']==39 and obs['certified_abs_pairs']==37 and len(obs['retained_literal_debt'])==2
suite=json.load(open(out/'workspace/workspace-result.json'));assert (suite['exit'],suite['passed'],suite['failed'],suite['ignored'],suite['sources_unchanged'])==(0,8796,0,7,True)
delta=json.load(open(out/'architecture-delta.json'));assert delta['all_other_diagnostics_preserved'] and delta['after']=={'V16:error':124,'V19:note':1066,'V20:note':867};assert json.load(open(out/'strict-details.json'))['exit']==json.load(open(out/'rustdoc-result.json'))['exit']==0
resources={str(p.relative_to(out)):json.load(open(p)) for p in out.rglob('*resources.json')};events=lambda x:dict((n,int(v)) for n,v in (s.split() for s in x.splitlines()))
for r in resources.values():
 m=r['memory_after'];assert m['memory.max']=='12884901888' and m['memory.high']=='10737418240' and m['memory.swap.max']=='0';assert events(m['memory.events'])['oom']==events(m['memory.events'])['oom_kill']==0
peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());cap=sum(events(r['memory_after']['memory.events'])['max'] for r in resources.values());high=sum(events(r['memory_after']['memory.events'])['high'] for r in resources.values())
rows=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];remaining=[r for r in rows if r['ruleId']=='V16'];groups=collections.defaultdict(list)
for r in remaining:
 loc=r['locations'][0]['physicalLocation'];groups[loc['artifactLocation']['uri']].append(loc['region']['startLine'])
(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');(out/'inventario-v16-restante.md').write_text('# V16 após calc.abs\n\n124 erros em '+str(len(groups))+' consumers. Fonte: final-state.json.gz/validation-result.json/SARIF integral. Gate global permanece aberto.\n\n| Consumer | Casos | Linhas |\n|---|---:|---|\n'+''.join(f'| `{p}` | {len(ns)} | {", ".join(map(str,ns))} |\n' for p,ns in sorted(groups.items())))
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],working_tree='uncommitted',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],changed_since_checkpoint=changed,tests={k:suite[k] for k in ['passed','failed','ignored']},observables=obs,architecture=delta,peak_bytes=peak,memory_max_events=cap,memory_high_events=high,resources=resources,pending_proposal_manifests_preserved=packages,public_api_changes=0)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
patch=''
for p in scope['sources']+list(scope['owners'].values()):patch+=''.join(difflib.unified_diff(b['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile='before/'+p,tofile='after/'+p))
(out/'patch-since-checkpoint.diff').write_text(patch)
(out/'pendencias.json').write_text(json.dumps(dict(global_v16=124,TS1='Pending explicit approval; package and its 19 consumers/owners preserved',RG1='Pending explicit approval; package and protected sources preserved',literal_construction=dict(witnesses=['overflow','overflow-extra'],observed='Tekt yields Float for minimum manual integer literal, pin rejects before abs',oracle_source='typst-eval/src/ops.rs:116-133',required='Audit lexical classification, Int literal overflow and unary diagnostics before deciding; use int.min as valid abs witness'),other_debts='See text-argumentos and Par/Box reports: relative text size, Paint, Styled equality, show text, fractional Box, SVG multi-page; all remain part of the active goal'),indent=2)+'\n')
(out/'relatorio.md').write_text(f'''# Relatório — V16 em calc.abs

Etapa concluída. **V16 125 → 124**; **V19 1065 → 1066 notas**; **V20 867 notas** preservadas. O gate global continua aberto pelos124 V16. Nenhum contrato público, default, fase, dependência ou núcleo mudou; não houve commit/push, merge ou release.

## Correção e justificativa

Apenas `01_core/src/compiler/stdlib/calc/abs.rs` e seu L0 proprietário mudaram. L0 atualizado antes do código; resselo restrito ao par. O wildcard que delegava o vocabulário a type_name foi substituído pela enumeração explícita de Value: Str→string, Bool→boolean e outras37 alternativas com a política comum de nome canônico. Nenhuma admissão numérica foi ampliada; tipos já tratados antes permanecem com suas condições.

O grupo V19 está prescrito no L0 pela política comum de diagnóstico, sem supressão. Uma variante nova agora exige classificação pelo compilador nesse consumer. Executar o braço não comprova cobertura individual de37 alternativas. Não foi alegado um erro no linter nem modificada sua política global.

## Testes e evidência

A alteração preserva runtime. O **RED é estrutural**, explicitamente: o match produtivo antigo compila tanto com as39 variantes atuais quanto com uma variante futura, violando a exigência nova de revisão. O GREEN compila o controle39 e rejeita a variante futura com E0004. Match extraído da produção e aridades obtidas do enum real, payloads mínimos; rustc isolado, sem target Cargo compartilhado. Isso comprova a trava de compilação, não paridade de runtime.

**Quatro testes funcionais novos passaram antes e depois**: aliases/mensagem/span detached, nomes de famílias genéricas, erro/overflow antes de extras e preservação de tipos/zero assinado/domínio de Length. Não foram apresentados como RED funcional, pois não houve falha funcional corrigida nesse formatter.

Medições bilaterais: **39 pares / 78 invocações em cada fase**, sendo37 testemunhas válidas de abs e dois casos de construção de literal. As37 testemunhas coincidem com o pin nos códigos, resultados e mensagens/hints medidos. Antes/depois, expressão/exit/stdout/stderr completos permaneceram exatos em ambos os binários, inclusive nos dois casos mantidos como dívida.

Os dois negativos com -9223372036854775808 não chegam ao mesmo valor nos dois compiladores: vanilla rejeita a construção manual na validação de negação em typst-eval/src/ops.rs:116–133; Tekt observa Float e prossegue. Não contam como prova de overflow de abs. Os controles int.min, int.min com extra e bound with confirmam o overflow real e sua precedência. A promoção foi localizada no lexer cristalino, compiler/lexer/code.rs:167–178; o defeito permanece aberto para auditoria de limites, bases, sufixos e fases antes de materialização.

Suíte completa: **8796 testes aprovados, zero falhas, sete ignorados**. Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings e quatro índices, V1/V5/V15/V26 estrito, reverse lineage e diff check passaram. Diferencial SARIF integral remove exatamente um V16 e adiciona somente o V19 prescrito; todos os demais diagnósticos permanecem após mapear linhas. SARIF global ainda retorna1 pelos124 V16 conhecidos.

## RAM e proveniência

Comandos pesados sequenciais, por gate em cgroup: **MemoryMax12GiB, MemoryHigh10GiB, swap zero, Cargo jobs1**. Pico **{peak} bytes ({peak/1024**3:.3f} GiB)**; eventos memory.max **{cap}**, memory.high **{high}**, sem OOM/OOM-kill. Memória do grupo inclui cache; não é RSS isolado nem prova de otimização de desempenho.

HEAD **{b['head']}**, working tree não commitado. before.json.gz, snapshots dos controles, resultados do probe, coletas por fase, validation-result.json e final-state.json.gz guardam UTC/status/diffstat, fontes/L0/configuração, argv e hashes de binários/artefatos. Oráculo9f2b6e8715237cb086899a42873660fe744622e8, SHA-2567fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 {b['linter']['sha256']}.

[Inventário restante](inventario-v16-restante.md):124 casos em {len(groups)} consumers. [Pendências](pendencias.json) e [auditoria seguinte do literal](literal-next-audit.md). A [triagem do próximo lote](next-batch-triage.json) reúne oito V16 em quatro consumers privados, ainda sem implementação. TS1/RG1 continuam aguardando as confirmações já solicitadas; seus manifestos completos e fontes protegidas foram conferidos e preservados. Ao retomar qualquer proposta, preservar esta correção independente do par abs. O objetivo geral permanece ativo.
''')
p=out/'relatorio.md';s=p.read_text()
for a,z in [('pelos124','pelos 124'),('outras37','outras 37'),('de37','de 37'),('as39','as 39'),('controle39','controle de 39'),('sendo37','sendo 37'),('As37','As 37'),('retorna1','retorna 1'),('MemoryMax12GiB','MemoryMax 12 GiB'),('MemoryHigh10GiB','MemoryHigh 10 GiB'),('Cargo jobs1','Cargo jobs=1'),('{peak}bytes', '{peak} bytes'),('Oráculo9f2','Oráculo 9f2'),('SHA-2567fa','SHA-256 7fa'),('Linter127','Linter 127'),('md):124','md): 124')]:s=s.replace(a,z)
p.write_text(s)
for p in (out/'workers').rglob('*.pyc'):p.unlink()
manifest=''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='SHA256SUMS');(out/'SHA256SUMS').write_text(manifest)
print(json.dumps(dict(tests=suite['passed'],v16=124,consumers=len(groups),peak_bytes=peak,manifest_sha256=sha(out/'SHA256SUMS'))))
