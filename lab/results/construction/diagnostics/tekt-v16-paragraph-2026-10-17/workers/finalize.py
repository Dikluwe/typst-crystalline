from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections,re
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();b=json.load(gzip.open(out/'before.json.gz'));v=json.load(open(out/'validation-result.json'));scope=json.load(open(out/'scope.json'));assert v.get('complete') and v['sources_unchanged'];assert all(sha(p)==h for p,h in v['source_hashes'].items());assert sha(b['linter']['path'])==b['linter']['sha256']
changed=[p for p,h in b['hashes'].items() if sha(p)!=h];assert set(changed)==set(scope['sources'])|set(scope['owners'].values()),changed
resources={str(p.relative_to(out)):json.load(open(p)) for p in out.rglob('*resources.json')};events=lambda s:dict((n,int(v)) for n,v in (x.split() for x in s.splitlines()));assert all(x['memory_after']['memory.max']=='12884901888' and x['memory_after']['memory.swap.max']=='0' and events(x['memory_after']['memory.events'])['oom']==0 and events(x['memory_after']['memory.events'])['oom_kill']==0 for x in resources.values());units={(x['start'],x['label']):x for x in resources.values()};peak=max(int(x['memory_after']['memory.peak']) for x in units.values());max_events=sum(events(x['memory_after']['memory.events'])['max'] for x in units.values());high_events=sum(events(x['memory_after']['memory.events'])['high'] for x in units.values());peak_gate=[name for name,x in resources.items() if int(x['memory_after']['memory.peak'])==peak]
assert all(x['memory_after'].get('memory.high','10737418240')=='10737418240' for x in resources.values());delta=json.load(open(out/'architecture-delta.json'));assert delta['after']=={'V16:error':125,'V19:note':1065,'V20:note':867};suite=json.load(open(out/'workspace/workspace-result.json'));assert suite['exit']==0 and suite['sources_unchanged'] and (suite['passed'],suite['failed'],suite['ignored'])==(8775,0,7);observations=json.load(open(out/'observables-result.json'));assert (observations['pairs'],observations['constructible_pairs'],observations['repaired'],observations['preserved'])==(70,68,42,26);assert len(observations['retained_open'])==2
for name in ['rustdoc-result.json','strict-details.json']:
 r=json.load(open(out/name));assert r['exit']==0
rg1=Path('00_nucleo/materialization/table-row-groups-2026-10-17');assert sha(rg1/'SHA256SUMS')=='a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9'
for line in (rg1/'SHA256SUMS').read_text().splitlines():h,p=line.split('  ',1);assert sha(rg1/p)==h,p
rgs=json.load(open(rg1/'scope.json'));protected=rgs['semantic_sources']+rgs['metadata_only_sources']+list(rgs['owners']);assert not set(changed)&set(protected);assert all(sha(p)==b['hashes'][p] for p in protected)
remaining=[r for r in json.load(open(out/'validation-architecture.log'))['runs'][0]['results'] if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');groups=collections.defaultdict(list)
for r in remaining:
 l=r['locations'][0]['physicalLocation'];groups[l['artifactLocation']['uri']].append(l['region']['startLine'])
(out/'inventario-v16-restante.md').write_text('# V16 após parágrafos\n\n125 erros em '+str(len(groups))+' consumers. Proveniência: final-state.json.gz/validation-result.json. O gate global permanece aberto.\n\n| Consumer | Casos | Linhas |\n|---|---:|---|\n'+''.join(f'| `{p}` | {len(lines)} | {", ".join(map(str,lines))} |\n' for p,lines in sorted(groups.items())))
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],working_tree='uncommitted',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],changed_since_checkpoint=changed,tests={k:suite[k] for k in ['passed','failed','ignored']},peak_bytes=peak,peak_gate=peak_gate,memory_max_events=max_events,memory_high_events=high_events,resources=resources,observables=observations,RG1_manifest_preserved=True,RG1_files_unchanged={p:sha(p) for p in protected},remaining_v16=125,remaining_consumers=len(groups),public_api_changes=0)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
report=f'''# Relatório — V16 e parágrafos vazios

Etapa concluída. **V16 128 → 125**, **V19 1060 → 1065 notas**, V20 **867 notas** preservadas. O objetivo geral continua aberto: o gate global ainda falha pelos 125 V16. Não houve commit/push, merge ou release.

## Alterações

Os L0s proprietários foram atualizados antes do código, em fluxo contínuo ADR-0127. Apenas dois consumers e seus dois owners mudaram desde o checkpoint: paragraph.rs e stdlib/structural/par.rs. Nenhum contrato público, fase, flag, dependência ou núcleo mudou. A árvore anterior, incluindo o lote de potência e a proposta RG1, foi preservada.

- Source é transparente no reconhecimento do parágrafo e do corpo vazio. Uma projeção privada emprestada enumera as 120 variantes Content em uma fronteira única, com decisões exaustivas nos consumidores. O grupo de tipos opacos mantém sua recusa; variantes futuras exigem revisão do compilador.
- Par vazio passa a ter fim causal próprio, sem tinta. As margens acima e abaixo colapsam separadamente com as unidades vizinhas; a margem terminal permanece pendente. Isso preserva dois vazios consecutivos, inclusive sem texto, e o controle com margens fortes de Block.
- Text vazio conserva a cadeia lexical da última ocorrência que fornece o strut, incluindo tamanho e arestas locais. O frame virtual usa contribuições não negativas de top e descida, sem consultar fontes quando não há Text vazio nem emitir tinta.
- O constructor par valida spacing como Length, inclusive em/zero/negativo, e o transporta lexicalmente para o layout. Tipos inválidos recebem mensagem/hint existentes e span da última ocorrência nomeada, com fallback sintético.

Os grupos V19 estão prescritos no L0 por política comum de admissão/recusa. O diferencial remove três V16 e substitui dois grupos antigos por sete grupos prescritos (saldo de cinco notas), sem supressões ou outros diagnósticos alterados. Executar um braço não prova todas as alternativas: os grupos permanecem visíveis.

## Evidência e testes

**25 testes novos** passaram. O primeiro RED funcional teve quatro falhas/cinco controles; o RED de margens e transporte teve nove falhas/doze controles; os quatro testes finais de métricas locais também falharam antes da implementação. A hipótese prospectiva de máximo entre Text vazios foi refutada pela fonte/coleta antes do código e não conta como prova funcional; o teste foi corrigido para o fallback observado.

A repetição mediu **70 pares por corpus**, com inputs repetidos entre famílias: não são 70 defeitos independentes. **68 pares construíveis** passaram na geometria de fluxo: 42 observações antes divergentes corrigidas e 26 controles preservados. Páginas e palavras exatas; altura de página, xMin/yMin/yMax com tolerância fixa de0.002pt. Os dois casos restantes são rejeições anteriores de Block com margens negativas, mantidas explicitamente abertas. Dez valores de spacing foram medidos nos dois binários: sete rejeições indevidas corrigidas; erros/hints agora concordam, e os três Length válidos permanecem aceitos.

O xMax extraído pelo pdftotext apresenta resíduo anterior de até {observations['max_xmax_residue']:.6f}pt; ele permaneceu inalterado dentro de Tekt, sem alargar a tolerância para chamá-lo de equivalente. Sua natureza (glyph/metadado/medição) permanece por classificar. A correção certifica fluxo vertical e preservação horizontal, não paridade PDF universal ou raw fields de Par.

Sonda rustc isolada: controle com todas as formas Content compila; Content futura é rejeitada na projeção; projeção futura é rejeitada nos três consumidores. Corpos produtivos extraídos, payloads externos mínimos; prova estrutural, não execução completa do workspace.

Suíte completa: **8775 passaram, zero falhas, sete ignorados**. Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, V1/V5/V15/V26 estrito, linhagem reversa, SARIF diferencial completo e diff check passaram.

## Recursos e proveniência

Comandos pesados sequenciais, cada gate em um novo cgroup: MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero e Cargo jobs=1. Pico do grupo **{peak} bytes ({peak/1024**3:.3f}GiB)** em {', '.join(peak_gate)}. Eventos memory.max **{max_events}**, memory.high **{high_events}**, sem OOM ou OOM-kill. Não é uma medição de RSS isolado nem comparação de desempenho com o lote anterior.

HEAD `{b['head']}`, working tree não commitado. before.json.gz, extended-before.json.gz, coletas por fase, validation-result.json e final-state.json.gz guardam UTC, status/diffstat exatos, fontes/L0/configurações, comandos, fontes tipográficas e SHA-256 de binários/artefatos. O oráculo é o pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. O linter é 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA{b['linter']['sha256']}.

O primeiro candidato e suas regressões foram preservados. Erros de escopo/string nos harnesses e interpretação de guard no verificador diferencial ficam separados; não contam como RED funcional. As fontes/L0 ficaram congeladas durante os gates finais; retomada corrigiu somente o verificador diagnóstico e não repetiu gates aprovados.

## Pendências preservadas

[Inventário V16 restante](inventario-v16-restante.md):125 casos em {len(groups)} consumers. [Pendências observadas](pendencias.json): text(tamanho) posicional, margens negativas no constructor Block e classificação do xMax extraído. Indentação/limites de justificação e raw fields genéricos de Par não foram certificados.

[Auditorias seguintes](next-audit.md) registram text relativo/posicional, a interação específica Par vazio→Box inline e a classificação de xMax; pontos ainda não medidos não são apresentados como bugs confirmados.

A proposta RG1 continua aguardando aprovação ADR-0127. Seus consumers/owners vigentes e o manifesto a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9 foram conferidos e permanecem intactos. Ao retomá-la, preservar os pares independentes power e paragraph/par que passaram depois de sua medição inicial.
'''
(out/'relatorio.md').write_text(report)
for p in list((out/'workers').glob('__pycache__/*.pyc')):p.unlink()
cache=out/'workers/__pycache__'
if cache.exists() and not any(cache.iterdir()):cache.rmdir()
manifest=''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='SHA256SUMS');(out/'SHA256SUMS').write_text(manifest)
print(json.dumps(dict(tests=suite['passed'],remaining_v16=125,consumers=len(groups),peak_bytes=peak,max_events=max_events,manifest_sha256=sha(out/'SHA256SUMS'))))
