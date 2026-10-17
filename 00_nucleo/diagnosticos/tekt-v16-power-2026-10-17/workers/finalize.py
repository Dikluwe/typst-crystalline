from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,re,collections
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();b=json.load(gzip.open(out/'before.json.gz'));v=json.load(open(out/'validation-result.json'));assert v.get('complete') and v['sources_unchanged'];assert all(sha(p)==h for p,h in v['hashes'].items());assert sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')=='e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2'
changed=[p for p,h in b['hashes'].items() if sha(p)!=h];assert set(changed)=={b['source'],b['owner']}
resources=[json.load(open(p)) for p in out.rglob('*resources.json')];assert all(x['memory_after']['memory.max']=='12884901888' and x['memory_after']['memory.swap.max']=='0' and 'oom_kill 0' in x['memory_after']['memory.events'] for x in resources);peak=max(int(x['memory_after']['memory.peak']) for x in resources)
delta=json.load(open(out/'architecture-delta.json'));assert delta['after']=={'V16:error':128,'V19:note':1060,'V20:note':867}
text=(out/'validation-workspace.log').read_text();summaries=[dict(status=m[0],passed=int(m[1]),failed=int(m[2]),ignored=int(m[3])) for m in re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',text,re.M)];totals={n:sum(x[n] for x in summaries) for n in ['passed','failed','ignored']};assert totals==dict(passed=8750,failed=0,ignored=7),totals
rg1=Path('00_nucleo/materialization/table-row-groups-2026-10-17');assert sha(rg1/'SHA256SUMS')=='a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9'
for line in (rg1/'SHA256SUMS').read_text().splitlines():h,p=line.split('  ',1);assert sha(rg1/p)==h,p
remaining=[r for r in json.load(open(out/'validation-architecture.log'))['runs'][0]['results'] if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');groups=collections.defaultdict(list)
for r in remaining:
 loc=r['locations'][0]['physicalLocation'];groups[loc['artifactLocation']['uri']].append(loc['region']['startLine'])
(out/'inventario-v16-restante.md').write_text('# V16 restantes após calc/power\n\n128 erros em '+str(len(groups))+' consumers. Proveniência completa: final-state.json.gz e validation-result.json. Gate global permanece aberto.\n\n| Consumer | Casos | Linhas |\n|---|---:|---|\n'+''.join(f'| `{p}` | {len(lines)} | {", ".join(map(str,lines))} |\n' for p,lines in sorted(groups.items())))
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],working_tree='uncommitted',status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['hashes'],changed_since_checkpoint=changed,tests=totals,summaries=summaries,peak_bytes=peak,resources=[str(p.relative_to(out)) for p in out.rglob('*resources.json')],remaining_v16=len(remaining),remaining_consumers=len(groups),RG1_manifest_preserved=True,public_api_changes=0)
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
report=f'''# Relatório — lote V16 calc/power

Etapa concluída. **V16 131 → 128**, V19 **1059 → 1060 notas**, V20 **867 notas** preservadas. O gate global continua aberto pelos 128 V16 restantes; não houve commit/push do Typst, merge ou release.

## Correções

O owner `00_nucleo/prompts/compiler/stdlib/calc/power.md` foi atualizado antes do código. A conversão explícita de Value delimita Int/Float/Decimal na base e Int/Float no expoente; os predicados de zero e o despacho usam enums privados exaustivos. Tipos não numéricos compartilham um único braço de erro, justificando a nota V19. Nenhuma supressão V16 ou alteração pública.

Corrigida a precedência: base inválida é rejeitada antes dos guards do expoente (por exemplo, `calc.pow(none,2147483648)`). Expoente decimal passa a receber o hint de conversão explícita. O erro de base agora inclui Decimal entre os tipos aceitos. Resultados e tipos numéricos, overflow, zero elevado a zero, erro decimal×float, sqrt/root e os demais diagnósticos locais foram preservados.

## Validação

Oito testes novos: RED funcional com três falhas e cinco controles passando; GREEN com oito passando. Suíte completa: **{totals['passed']} passaram, zero falhas e {totals['ignored']} ignorados**. Os 20 casos CLI medidos antes da decisão foram repetidos: resultados/códigos preservados, diagnósticos alterados somente nos cinco casos previstos. Fonte `calc.rs:172-201` conferida byte a byte no pin `9f2b6e8715237cb086899a42873660fe744622e8`; binários e estado registrados em before-measurement.json.gz/after-measurement.json.gz. Isso não certifica paridade geral de mensagens, spans, infinitos ou fórmulas de powf/powi.

Sonda isolada rustc: as 39 variantes atuais compilam; uma variante futura é rejeitada com E0004 na conversão produtiva extraída. Payloads não numéricos são stubs nessa sonda, que prova cobertura estrutural, não execução completa do compilador. Não compartilhou target Cargo.

Linter pinado `12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Build do workspace, fmt, Clippy estrito para todos os targets (incluindo expectativas não cumpridas), Rustdoc com -D warnings, V1/V5/V15/V26, linhagem reversa, diff check e diferencial SARIF completo passaram. O diferencial remove exatamente os três V16 selecionados e acrescenta somente o V19 prescrito; demais diagnósticos são idênticos após mapear linhas.

## RAM, proveniência e limites

Todos os comandos pesados foram sequenciais em cgroup com **12 GiB**, swap zero e Cargo jobs=1. Pico máximo registrado **{peak} bytes ({peak/1024**3:.3f} GiB)**: atingiu o teto rígido. O grupo registrou 1127 eventos memory.max, sem OOM nem OOM-kill. O limite foi mantido, porém houve pressão de memória; estas amostras agregadas não atribuem o pico a um processo ou gate específico. Em lotes futuros, separar gates pesados em cgroups sequenciais permite descartar a contabilidade acumulada entre etapas sem repetir validações já aprovadas. HEAD `{b['head']}`, working tree não commitado; UTC, status completo, diffstat exato e SHA-256 de todos os fontes/L0s em before.json.gz, validation-result.json e final-state.json.gz. Apenas o par power.rs/power.md mudou desde o checkpoint; alterações anteriores foram preservadas.

A primeira medição com argumentos CLI errados está arquivada sem crédito funcional em attempt-cli-arguments. Um erro de retorno da nova conversão, corrigido antes do GREEN, está em attempt-result-type e também não conta como RED. Os logs reais do RED funcional e de todos os gates são preservados.

O [inventário restante](inventario-v16-restante.md) registra 128 casos em {len(groups)} consumers. A proposta pública RG1 de grupos de tabelas permanece aguardando aprovação ADR-0127; seu manifesto `a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9` foi revalidado integralmente e não foi alterado por este lote independente.
'''
(out/'relatorio.md').write_text(report)
manifest=''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='SHA256SUMS');(out/'SHA256SUMS').write_text(manifest)
print(json.dumps(dict(tests=totals,peak_bytes=peak,remaining_v16=128,consumers=len(groups),manifest_sha256=sha(out/'SHA256SUMS'))))
