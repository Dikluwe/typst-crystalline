from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,difflib,collections
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');service=load('validation-result.json');suite=load('workspace/workspace-result.json');scope=load('proposal-scope.json');approval=load('approval-receipt.json');inv=load('clippy-inventory.json');strict=load('strict-details.json')
assert service['exit']==0 and len(v['commands'])==12 and all(c['exit']==0 for c in v['commands'])
assert not inv and v['clippy_delta']=={'clippy::large_enum_variant':-2} and v['clippy_total']==0
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged'] and strict['exit']==0 and not strict['diagnostics']
assert load('actual-sizes.json')['sizes']==dict(tag=48,element_info=400,transformation=120,show_rule=168,styles=328)
assert load('red-result.json')['exit']==101 and load('green-result.json')['exit']==0
assert all(x['exit']==101 for x in load('budget-red.json')['samples']) and all(x['exit']==0 for x in load('budget-green.json')['samples'])
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
pkg=root/approval['package'];assert sha(pkg/'manifest.sha256')==approval['manifest_sha256']
for line in (pkg/'manifest.sha256').read_text().splitlines():
 h,p=line.split('  ',1);assert sha(pkg/p)==h
mem=service['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());peak=int(mem['memory.peak'])/1024**3
assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0'
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],clippy=0,tekt=v['architecture_warnings'],suite=suite,memory_peak_gib=peak,approval=approval)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n')
paths=[r[k] for r in scope for k in ['source','owner']]
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(before['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p in paths))
link=lambda p:f'[{p}]({out/p})'
report=f'''# Payloads públicos compactos — etapa concluída

Clippy **2 → 0 avisos**, com workspace/all-targets e **-D warnings**. Implementados os dois contratos ratificados, nos oito consumers e seus oito L0, com resselo restrito. {link('stage.diff')} · {link('code-guard.json')}.

## Alterações e compatibilidade

Tag::Start passa a (Location, Box<ElementInfo>); End e os campos de ElementInfo/ElementPayload permanecem. Atualizados 35 construtores: cinco produtivos e trinta de fixtures. Não mudar padrões de leitura, pareamento/ordem Start/End, Location, hash de End, índice, snapshot ou callbacks. Box mantém propriedade exclusiva e clone profundo. No ABI local, o carrier passa de 416 para 48 bytes; o payload de 400 continua alocado separadamente, uma alocação adicional por Start.

Transformation::Style passa a Arc<Styles>; as outras variantes e ShowRule/Selector/NodeKind/RuleId permanecem. O produtor envolve a mesma captura em Arc::new. Cópias de regras compartilham o payload imutável; os consumidores já usam empréstimos e deref, sem nova lógica. Transformation passa de 328 para 120 bytes e ShowRule de 376 para 168 no ABI medido. Clientes Rust externos devem envolver os payloads em Box::new/Arc::new e usar Arc::make_mut para editar uma cópia dos estilos. Estas mudanças de compatibilidade foram expressamente ratificadas.

Nenhuma função pública, dependência, default, flag, variante adicional ou fase de pipeline foi introduzida. Não foram adicionadas exceções aos dois avisos. Os parsers Option, traits anteriores e exceções nominais já justificadas continuam validados.

Aprovação do manifesto **{approval['manifest_sha256']}**, [review.md]({pkg}/review.md). Os oito L0 foram aplicados antes dos testes/código; pacote original verificado e preservado. {link('approval-receipt.json')} · {link('l0-before-code.json')}.

## Medições e limites

A sonda anterior usa os carriers inline reais e candidatos espelhados Box/Arc. Para 4.096 pares de tags, bytes solicitados caem de 3.407.872 para 2.031.616, com alocações de 1 para 4.097. O clone de Tag boxed também aloca por Start; as travessias produtivas auditadas usam o vetor temporário por empréstimo. Não tomar a frequência de clone sintética como carga produtiva.

Nas 128 inserções de regras 100% show-set simples, o algoritmo de to_vec/push/Arc solicita 12.276.960 bytes inline e 5.530.624 com Arc; Box exige 8.640 alocações contra 512 Arc. Ao reter snapshots, o pico solicitado cai de 3.203.856 para 1.477.808 bytes. A variante rica de custom confirma que o compartilhamento evita a cópia do backing em snapshots de regras.

**Contraexemplos mantidos:** a construção única de 4.096 regras 100% show-set simples cresce de 1.540.096 para 2.097.152 bytes; Arc preserva capacidade sobrante, tem refcount atômico e indireção. O final vivo do controle rico de 128 registros sem snapshots antigos cresce de 78.064 para 156.432 bytes. A aplicação de estilos ainda faz as cópias de delta necessárias. A evidência favorece o ciclo auditado de cópias/registro, sem provar menor RSS/tempo em todo documento. Nenhum benchmark de compilação/layout real ou ganho de leitura por IA foi alegado. {link('measurements.json')} · {link('probe-details.json')} · {link('callers.txt')}.

## RED, correção da migração e validação

Os testes de contratos foram rejeitados no estado inline por tipos incompatíveis, antes da implementação. Orçamentos separados no rlib anterior falharam: Tag 416 > 48 e Transformation 328 > 128. Os novos testes passam e demonstram clone independente de Tag, igualdade/hash por valor e compartilhamento/COW de estilos com isolamento de campos e custom. {link('red.log')} · {link('budget-red.json')} · {link('budget-green.json')} · {link('actual-sizes.json')}.

A primeira migração automatizada atingiu três padrões de leitura; o compilador rejeitou as chamadas Box::new em patterns. A tentativa está preservada em first-migration/. Restauradas apenas as oito fontes deste lote ao snapshot pós-testes, corrigido o filtro e conferidos exatamente 35 construtores. A migração final preserva todos os padrões de leitura e os corpos fora dos spans explicitados; nenhum teste esperado foi relaxado.

Suíte workspace: **{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados**. Core GREEN pré-validação, build, Clippy all-targets normal e estrito, budgets, fmt, linhagem estrita V1/V5/V15/V26 e reversa, Rustdoc -D warnings e quatro índices HTML e diff --check passaram. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

Tekt mantém **{sum(v['architecture_warnings'].values())} avisos**, {v['architecture_warnings']}, sem erros ou aumento por regra. Clippy está encerrado no estado medido; a meta geral de zero violations Tekt continua aberta.

## Recursos e proveniência

Serviços pesados sequenciais: **12 GiB**, swap zero, CARGO_BUILD_JOBS=1. Pico do cgroup **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. {link('validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação {v['start']}–{v['end']} UTC, com fontes/L0 imutáveis durante os comandos. {link('before.json.gz')} · {link('final-state.json')} conservam fontes/hashes/status/git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150; vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/ e {link('manifest.sha256')}. [Lista de avisos Tekt por owner](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-inventario-2026-10-16/lista.md) para a próxima etapa. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=0,tests=suite['passed'],peak_gib=peak)))
