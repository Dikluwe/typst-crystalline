from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,re,difflib,collections
root=Path.cwd();out=Path(__file__).resolve().parents[1];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for part in iter(lambda:f.read(4*1024*1024),b''):h.update(part)
 return h.hexdigest()
b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));w=json.load(open(out/'workspace/workspace-result.json'));delta=json.load(open(out/'architecture-delta.json'));baseline=json.load(open(out/'probe-baseline-result.json'));green=json.load(open(out/'probe-green-result.json'));assert v['validation_complete'] and v['sources_unchanged_during_validation'];assert all(sha(p)==h for p,h in v['source_hashes'].items());assert all(c['exit']==(1 if c['label']=='architecture' else 0) for c in v['commands'])
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];assert subprocess.check_output(['git','branch','--show-current'],text=True).strip()=='Tekt'
changed=sorted(p for p,h in b['hashes'].items() if sha(p)!=h);assert changed==sorted(scope['sources']+scope['owners']);assert baseline['selected']==0 and green['selected']==11 and green['total_e0004']==baseline['total_e0004']+11;assert not baseline['other_errors'] and not green['other_errors'];assert baseline['sources_unchanged'] and green['sources_unchanged']
assert '1410 passed; 0 failed;' in (out/'baseline-controls.log').read_text();prev=json.load(open(out.parent/'tekt-v16-introspect-walk-2026-10-16/workspace/workspace-result.json'));assert w['exit']==0 and w['sources_unchanged'] and w['failed']==0 and w['passed']==prev['passed'] and w['ignored']==prev['ignored'];assert Path(scope['sources'][0]).read_text().count('#[test]')==b['files'][scope['sources'][0]].count('#[test]')
assert delta['after']=={'V16:error':222,'V19:note':918,'V20:note':871};assert delta['all_other_diagnostics_preserved'];assert sha(b['linter']['binary'])==b['linter']['sha256']
docs={str(root/'target/doc'/crate/'index.html'):sha(root/'target/doc'/crate/'index.html') for crate in ['typst','typst_core','typst_shell','typst_infra']}
resources={p.name:json.load(open(p)) for p in out.glob('*-resources.json')};assert resources['validation-resources.json']['exit']==0;peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());assert peak<=12*1024**3
for r in resources.values():
 assert r['memory_before']['memory.max']=='12884901888' and r['memory_before']['memory.swap.max']=='0' and r['cargo_jobs']=='1';events=dict(line.split() for line in r['memory_after']['memory.events'].splitlines());assert events['oom']=='0' and events['oom_kill']=='0'
measurement=json.load(open(out/'source-table-measurement.json'));assert measurement['sources_unchanged'];assert all(sha(p)==h for p,h in measurement['binaries'].items());pairs={}
for r in measurement['cases']:pairs.setdefault(r['name'],{})['vanilla' if r['binary'].startswith('lab/') else 'tekt']=r
assert len(pairs)==6
observations=[]
for name,pair in pairs.items():
 a,z=pair['vanilla'],pair['tekt'];ok=a['exit']==0 and z['exit']==0;data_a=json.loads(a['stdout']) if a['exit']==0 else None;data_z=json.loads(z['stdout']) if z['exit']==0 else None;observations.append({'fixture':name,'both_queries_succeeded':ok,'same_heading_bodies':ok and data_a==data_z,'vanilla_count':len(data_a) if ok else None,'tekt_count':len(data_z) if ok else None})
(out/'source-table-analysis.json').write_text(json.dumps({'observations':observations,'scope':'Six heading query probes through table/cell show and alias; not a proof of table_root for every carrier or of global table parity.','pending':'Source in table_root returns false; no productive correction in this stage. Further causal measurement required before deciding.'},indent=2)+'\n')
patch=[]
for p in scope['sources']+scope['owners']:patch.extend(difflib.unified_diff(b['files'][p].splitlines(keepends=True),Path(p).read_text().splitlines(keepends=True),fromfile='a/'+p,tofile='b/'+p))
(out/'stage.patch').write_text(''.join(patch))
remaining=json.load(open(out/'remaining-v16.json'));counts=collections.Counter(r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./') for r in remaining);assert sum(counts.values())==222 and counts[scope['sources'][0]]==52
pending='# Pendências após o lote de helpers de eval\n\nEstado e proveniência: final-state.json.gz. O gate global permanece vermelho com 222 V16. A lista integral de locais e mensagens está em remaining-v16.json.\n\n| Consumer | V16 |\n|---|---:|\n'+''.join(f'| `{p}` | {n} |\n' for p,n in counts.most_common())+'\nOs 52 casos restantes em eval/tests.rs exigem auditoria própria; não foram silenciados. A hipótese Source(Table) permanece aberta com seis sondas em source-table-analysis.json; os resultados de sondas não devem ser confundidos com prova de todos os carriers. V19 permanece informativo; não há autorização para converter toda nota em correção ou supressão.\n';(out/'pendencias.md').write_text(pending)
listing='| ID | Linha de entrada | Helper | Complemento |\n|---|---:|---|---:|\n'+''.join(f"| {r['id']} | {r['line']} | `{r['function']}` ({r['enum']}) | {len(r['complement'])} |\n" for r in scope['targets'])
results='\n'.join(f"- `{r['fixture']}`: vanilla {r['vanilla_count']}, Tekt {r['tekt_count']}; campos {'iguais' if r['same_heading_bodies'] else 'divergentes ou execução inválida'}." for r in observations)
report=f'''# V16 — helpers de projeção e estilos de eval

Etapa concluída em {now()}. **V16 233 → 222**, **V19 907 → 918 notas**, V20 **871 notas preservadas**. O gate global continua com exit 1 pelos V16 restantes; o objetivo geral está aberto.

## Correção e equilíbrio V19

Foram fechados onze fallbacks no consumer cfg(test) `01_core/src/compiler/eval/tests.rs`. Os corpos anteriores None/false/no-op/herança, os ramos atendidos, a ordem de busca, short-circuit, estilos, fixtures e asserções permanecem. Cada fallback recebeu o complemento nominal das variantes atuais, forçando nova decisão quando Content ou Value crescer. Nenhum código produtivo de avaliação ou contrato público mudou.

{listing}

As onze notas V19 novas são esperadas e justificadas individualmente no [00_nucleo/prompts/compiler/eval/tests.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval/tests.md), após a medição: variantes excluídas compartilham o mesmo resultado daquele helper. Não se alega cobertura de execução de todas as alternativas. Não foram adicionados allow/expect, limiares, bindings capturados ou reescritas em if-let. As notas continuam visíveis; não se alega ganho medido para leitura de IA. [scope.json](scope.json), [stage.patch](stage.patch).

## Validação

Os **1.410 testes de eval passaram antes da alteração**. A sonda estrutural em cópia descartável do workspace, com cfg(test) realmente compilado e uma variante sentinela em cada enum, passou de **0 para 11 E0004 nos matches selecionados**; total **58 → 69**, sem outros erros. [probe-baseline-result.json](probe-baseline-result.json), [probe-green-result.json](probe-green-result.json). Isso é RED→GREEN de exaustividade, sem alegar correção funcional onde o comportamento foi preservado.

Suíte workspace: **{w['passed']} passaram, {w['failed']} falharam, {w['ignored']} ignorados**, mesmos totais da etapa anterior. Sem adicionar testes que espelhem a implementação. Build workspace, fmt, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices HTML, V1/V5/V15/V26 estrito, linhagem reversa e diff-check passaram. [validation-result.json](validation-result.json), [workspace/workspace-result.json](workspace/workspace-result.json).

O diferencial SARIF confere todos os campos dos demais diagnósticos, inclusive mensagens, níveis, multiplicidades e posições mapeadas: somente onze V16 removidos e onze V19 acrescentados. [architecture-delta.json](architecture-delta.json). O replay confirma que apenas padrões/metadados e os selos do par proprietário mudaram, com enums, assinaturas, asserções e arquivos externos preservados. [guard-result.json](guard-result.json).

A primeira execução do replay falhou porque o rustfmt no diretório temporário não carregou rustfmt.toml do repositório. Repetir a partir da entrada congelada com --config-path explícito reproduziu exatamente o código esperado. O verificador foi corrigido; fontes/L0 não precisaram de reparo. A tentativa e seu consumo estão preservados em guard-repair.json e materialization-resources.json.

## Investigação Source(Table)

O ramo local table_root continua classificando Source como false, enquanto Table/Located/ShowRecipe têm reconhecimento próprio. É uma hipótese de transparência a medir causalmente, não correção certificada por fechar seu wildcard. Foram executadas seis sondas bilaterais de query(heading) através de table/cell show e alias, contra o pin `9f2b6e8715237cb086899a42873660fe744622e8`:

{results}

Fontes, comandos, stdout/stderr, hashes dos binários e UTC estão em [source-table-measurement.json](source-table-measurement.json). A análise conserva divergências e execuções inválidas; os resultados não provam todos os carriers ou paridade global de tabelas. Nenhuma regra de tabela foi alterada nesta etapa. [source-table-analysis.json](source-table-analysis.json).

## RAM, proveniência e continuidade

Comandos pesados sequenciais, **MemoryMax=12 GiB**, **swap zero**, **Cargo um job**. Pico de memória do cgroup **{peak} bytes ({peak/1024**3:.3f} GiB)**; sem OOM ou OOM kill. Todos os recibos *-resources.json, inclusive a tentativa do replay, foram conservados.

HEAD `{b['head']}`, branch Tekt, working tree não commitado. [before.json.gz](before.json.gz) e [final-state.json.gz](final-state.json.gz) preservam UTC, hashes, status e lista exata git diff HEAD --stat. O L0 foi atualizado antes do código e só seu par foi ressellado. Fontes/L0/configuração ficaram congelados durante os gates. Linter `{b['linter']['commit']}`, SHA-256 `{b['linter']['sha256']}`; oráculo pinado SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, proveniência ratificada em CLAUDE.md. Sem alteração do pin CI ou dependências; sem commit, push, merge ou release.

Restam **222 V16**, incluindo **52** neste consumer. [Lista organizada de pendências](pendencias.md), [inventário integral](remaining-v16.json). As etapas seguintes exigem medição/L0 próprios; nenhuma pendência foi declarada corrigida por este relatório.
'''
(out/'relatorio.md').write_text(report)
record={'utc':now(),'head':b['head'],'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'source_hashes':v['source_hashes'],'changed':changed,'scope':scope,'tests':w,'architecture':delta,'probes':{'before':baseline,'after':green},'resources':resources,'peak_bytes':peak,'docs':docs,'linter':b['linter'],'measurement_binaries':measurement['binaries'],'stage_complete':True,'goal_scope_incomplete':True,'remaining_v16':222,'no_commit_push_release':True}
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(record,f)
manifest={str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest-sha256.json'};(out/'manifest-sha256.json').write_text(json.dumps(manifest,indent=2)+'\n');assert all(sha(out/p)==h for p,h in manifest.items());print(json.dumps({'stage_complete':True,'remaining_V16':222,'V19':918,'V20':871,'passed':w['passed'],'failed':0,'ignored':w['ignored'],'peak_gib':peak/1024**3,'manifest_files':len(manifest),'goal_scope_incomplete':True}),flush=True)
