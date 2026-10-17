from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections
root=Path.cwd();out=root/'00_nucleo/diagnosticos/field-reassign-2026-10-15';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=json.load(gzip.open(out/'before.json.gz'));v=json.loads((out/'validation-details.json').read_text());r=json.loads((out/'validation-result.json').read_text());t=json.loads((out/'workspace/workspace-result.json').read_text());s=json.loads((out/'scope-guard.json').read_text())
assert len(v['commands'])==9 and all(c['exit']==0 for c in v['commands']) and r['exit']==t['exit']==0
assert all(sha(p)==h for p,h in v['source_hashes'].items()) and v['sources_unchanged_during_validation'] and t['sources_unchanged']
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head']
assert v['clippy_total']==119 and v['clippy_delta']=={'clippy::field_reassign_with_default':-25}
a=json.loads((out/'clippy-inventory.json').read_text());counts=collections.Counter(x['code'] for x in a);assert counts['clippy::field_reassign_with_default']==counts['clippy::type_complexity']==0 and counts['clippy::too_many_arguments']==57
assert r['memory_after']['memory.max'].strip()=='12884901888' and r['memory_after']['memory.swap.max'].strip()=='0' and r['cargo_jobs']=='1'
events={k:int(n) for k,n in (l.split() for l in r['memory_after']['memory.events'].splitlines())};assert events['oom']==events['oom_kill']==0
html={}
for crate in ['typst_core','typst_shell','typst_infra','typst']:
 p=root/'target/doc'/crate/'index.html'
 with p.open() as f:assert '<html' in f.read(300).lower()
 html[str(p.relative_to(root))]=sha(p)
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],working_tree='não commitado; fontes e L0 idênticos à validação anterior',source_hashes=v['source_hashes'],documentation_indices=html,status=subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),temporary_workers_missing_after_interruption=True)
(out/'final-state.json').write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n');(out/'remaining-clippy-rules.json').write_text(json.dumps(dict(sorted(counts.items())),indent=2)+'\n')
link=lambda title,name:f'[{title}]({out/name})'
report=f'''# field_reassign_with_default — etapa concluída

Os 25 casos foram corrigidos em oito consumers, todos em fixtures/helpers de teste. Oito L0 foram atualizados antes do código e resselados de forma restrita. Nenhuma supressão nova, API ou comportamento padrão foi introduzido. A configuração de cada fixture agora fica reunida em um literal com `..Tipo::default()`.

Foram movidos 41 valores explícitos para os inicializadores, preservando os defaults dos demais campos. Foram retirados 23 `mut` dispensáveis. Stroke permanece mutável para `specified.thickness/cap`; o gerador de recursos conserva a atribuição condicional de variações em `if axes`. Os clones e as mutações posteriores de peso/tracking foram preservados. O probe de timing F020 continua ignorado por padrão e foi compilado por all-targets; nenhum resultado de benchmark é alegado.

{link('Lista dos 25 casos','lista.md')} · {link('CSV','cases.csv')} · {link('JSON','cases.json')} · {link('Diff da etapa','stage.diff')}. A lista identifica campos, função/helper, fonte atual e L0 proprietário. O {link('controle de escopo','scope-guard.json')} registra gates cfg(test), nomes/atributos de testes, trechos de assertions e partes produtivas preservadas. A revisão do diff confirmou as mutações posteriores intactas. Esta manutenção de sintaxe não mede ganho para IA nem paridade nova.

## Validação

O gate RED reproduziu os 25 erros antes da edição; o GREEN passou com `-D clippy::field_reassign_with_default -D unfulfilled_lint_expectations`. O inventário completo caiu de **144 para 119 avisos**, com delta exclusivamente de −25 nessa regra. `type_complexity` segue em zero. Os 57 casos de `too_many_arguments` permanecem na auditoria anterior.

A suíte workspace terminou com **{t['passed']} testes aprovados, {t['failed']} falhas e {t['ignored']} ignorados**. Inputs, nomes, esperados e tolerâncias não foram relaxados; não foram criados testes que espelhem a sintaxe do inicializador. Build, Clippy all-targets, cargo fmt, Rustdoc com `RUSTDOCFLAGS="-D warnings"`, git diff --check, linhagem estrita V1/V5/V15/V26 e linhagem reversa passaram. Os quatro índices HTML foram conferidos após a suíte. Fontes e L0 permaneceram imutáveis durante a execução e continuam idênticos na conferência de fechamento.

O Tekt completo registrou {v['architecture_levels']}; avisos por regra: {v['architecture_warnings']}. Nenhum erro ou aviso arquitetural novo. Os avisos restantes de Clippy/Tekt continuam pendentes; o CI global ainda não está livre de avisos.

{link('Comandos, horários e estado da validação','validation-details.json')} · {link('Resultado da suíte','workspace/workspace-result.json')} · {link('Gate RED','red-clippy-result.json')} · {link('Gate GREEN','green-clippy-result.json')}.

## RAM e proveniência

Os serviços usaram **MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1**. O pico do cgroup da validação foi **{int(r['memory_after']['memory.peak'])/1024**3:.3f} GiB**, incluindo cache de arquivos. O limite foi atingido: memory.events max={events['max']}; OOM={events['oom']} e OOM-kill={events['oom_kill']}. Esses eventos de max registram pressão/recuperação no limite, não OOM. O limite cobre os processos dos serviços, não os demais aplicativos do host. {link('Recibo de memória','validation-result.json')}.

HEAD `{b['head']}`, working tree não commitado. Origem em `{b['utc']}`; validação entre `{v['start']}` e `{v['end']}` (UTC). {link('Snapshot inicial','before.json.gz')} e recibos conservam fontes/L0, hashes, status, arquivos não rastreados e a lista exata de alterações (`git diff HEAD --stat`). {link('Conferência final','final-state.json')} registra os hashes atuais e sua equivalência com o estado medido.

A suíte usou o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, e o linter CI pinado em `49c6bb0e584104a0c89270431d8402f5c6c61150`. Os caminhos temporários de execução constam nos recibos; os scripts temporários desapareceram após a interrupção, e não se alega preservação desses workers. O finalizador deste fechamento está salvo em workers/finalize.py. {link('Manifesto SHA-256','manifest.sha256')}.

Sem commit, push ou release. Alterações anteriores preservadas.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text('\n'.join(f'{sha(p)}  {p.relative_to(out)}' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256')+'\n')
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=v['clippy_total'],tests=t['passed'],peak_gib=int(r['memory_after']['memory.peak'])/1024**3)))
