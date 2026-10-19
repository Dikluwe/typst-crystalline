from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,difflib,collections,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-frame-items-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');service=load('validation-result.json');suite=load('workspace/workspace-result.json');scope=load('scope.json');strict=load('strict-details.json');baseline=load('baseline-details.json');base_suite=load('baseline-workspace/workspace-result.json');pb=load('probe-baseline-details.json');pg=load('probe-green-details.json');gate=load('family-green-result.json')
assert service['exit']==0 and len(v['commands'])==13 and all(c['exit']==0 for c in v['commands'])
assert not load('clippy-inventory.json') and not strict['diagnostics'] and strict['exit']==0 and v['clippy_delta']=={}
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
for s in [suite,base_suite]:assert s['exit']==0 and s['failed']==0 and s['sources_unchanged'] and s['passed']==8622 and s['ignored']==7
assert baseline['sources_unchanged'] and [c['exit'] for c in baseline['commands']]==[0,1]
assert pb['selected_e0004']==0 and pg['selected_e0004']==12 and len(pg['targets'])==12
assert all(t['e0004'] for t in pg['targets']) and not any(t['e0004'] for t in pb['targets'])
assert pb['sources_unchanged'] and pg['sources_unchanged'] and not pb['other_errors'] and not pg['other_errors']
assert v['architecture_warnings']=={'V16':242,'V12':19}
assert gate['exit']==0 and gate['selected_frameitem_warnings']==0 and gate['outside_warnings']==242 and gate['global_exit']!=0
assert load('family-baseline-result.json')['selected_frameitem_warnings']==70
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
resources=[]
for name in ['validation','baseline','probe-baseline','audit']:
 rec=load(name+'-result.json');ev=dict(s.split() for s in rec['memory_after']['memory.events'].splitlines());assert ev['oom']=='0' and ev['oom_kill']=='0' and int(rec['memory_after']['memory.max'])==12*1024**3 and int(rec['memory_after']['memory.swap.max'])==0
 resources.append(dict(label=name,peak_gib=int(rec['memory_after']['memory.peak'])/1024**3,events=ev,start=rec['start'],end=rec['end']))
(out/'resources.json').write_text(json.dumps(resources,indent=2)+'\n')
results=json.loads((out/'validation-architecture.log').read_text())['runs'][0]['results'];old=json.loads((root/'00_nucleo/diagnosticos/tekt-enums-2026-10-16/validation-architecture.log').read_text())['runs'][0]['results']
def key(r):return (r['ruleId'],r['level'],r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./'),r['message']['text'])
a=collections.Counter(key(r) for r in old);b=collections.Counter(key(r) for r in results);added=b-a;removed=a-b
added_warnings=[dict(rule=k[0],level=k[1],source=k[2],message=k[3],count=n) for k,n in added.items() if k[1] in ['warning','error']];assert not added_warnings
new_notes=[dict(rule=k[0],level=k[1],source=k[2],message=k[3],count=n) for k,n in added.items() if k[1]=='note']
(out/'architecture-delta.json').write_text(json.dumps(dict(added_warnings=added_warnings,added_notes=new_notes,removed=[dict(rule=k[0],level=k[1],source=k[2],message=k[3],count=n) for k,n in removed.items()],comparison='multiset rule/level/source/message, excluding shifted line numbers'),ensure_ascii=False,indent=2)+'\n')
(out/'new-notes.json').write_text(json.dumps(new_notes,ensure_ascii=False,indent=2)+'\n')
remaining=[]
for r in results:
 if r['level']!='warning':continue
 loc=r['locations'][0]['physicalLocation'];p=loc['artifactLocation']['uri'].removeprefix('./');owner=re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M)[1];remaining.append(dict(rule=r['ruleId'],source=p,line=loc['region']['startLine'],owner=owner,message=r['message']['text']))
(out/'remaining.json').write_text(json.dumps(remaining,ensure_ascii=False,indent=2)+'\n')
latent=json.loads((root/'00_nucleo/diagnosticos/tekt-enums-2026-10-16/latent-v16.json').read_text());assert all(sha(d['source'])==before['hashes'][d['source']] for d in latent);(out/'latent-v16.json').write_text(json.dumps(latent,ensure_ascii=False,indent=2)+'\n')
peak=max(r['peak_gib'] for r in resources)
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],clippy=0,tekt=v['architecture_warnings'],suite=suite,memory_peak_gib=peak,latent_v16='latent-v16.json',notes=new_notes)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n');(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(before['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p in scope['sources']+scope['owners']))
link=lambda p:f'[{p}]({out/p})'
text=f'''# Pendências Tekt após FrameItem

**261 avisos emitidos: 242 V16 e 19 V12. Clippy zero.** HEAD `{before['head']}`, working tree não commitado; UTC {v['end']}. Fontes, hashes, status e git diff HEAD --stat: {link('final-state.json')} e {link('validation-details.json')}.

Ordem: V16 por família/owner, depois V12 distinguindo composição produtiva de fixtures. Cada correção ou supressão precisa de prescrição no L0. Não mover suporte de teste só para ocultar diagnóstico.

T027 e T158/T160 continuam sem emissão e pendentes em {link('latent-v16.json')}; mudança de heurística não certifica correção. O catchall de metadata que consolidava T311/T313 foi nominalizado neste lote (F054), sem fechar o V16 separado de SemanticKind nesse owner.

| Regra | Local | Owner | Evidência |
|---|---|---|---|
'''
for r in remaining:text+=f'| {r["rule"]} | [{r["source"]}:{r["line"]}]({root/r["source"]}:{r["line"]}) | [{r["owner"]}]({root/r["owner"]}) | '+r['message'].replace('|','\\|').replace('\n',' ')+' |\n'
(out/'lista-pendencias.md').write_text(text)
notes_count=sum(r['count'] for r in new_notes)
report=f'''# V16 — FrameItem: etapa concluída

**70 avisos corrigidos em 26 consumers e 26 L0. Tekt: 331 → 261 avisos emitidos; Clippy continua zero.** Sem mudança de API, campos, enum, defaults ou fase de pipeline. Os L0 foram prescritos antes da implementação, com medição por caso e resselo restrito direto/reverso. {link('classification.json')} · {link('stage.diff')} · {link('code-guard.json')}.

## Correção e alcance

Em 69 matches, o catchall foi substituído pelas variantes atuais que alcançam seu corpo. Guards e campos restritos não contam como cobertura integral: a variante permanece no complemento quando a condição anterior falha. Os corpos, precedência, chamadas, fórmulas, fixtures e esperados foram preservados por replay das edições e formatação restrita.

FrameVisitor já cobria as onze variantes. Foram retirados o fallback inalcançável e seu allow(unreachable_patterns); a documentação agora descreve a decisão exigida em compilação. Callbacks e ordem de travessia permanecem. O novo teste funcional cobre as onze variantes, callbacks e filhos de TextReplacement/Semantic/Group/Link, exigindo a sequência de 14 callbacks. Não decodifica imagem nem mede render.

Os percursos de baseline de Box mantêm sua diferença vigente: first_baseline recorre por Link, enquanto frame_items_have_text_baseline não o faz. Este lote não certifica nem corrige uma nova questão de geometria. Exportação PDF/SVG, acessibilidade, clipping, reconciliação RTL e observadores de testes conservam os corpos anteriores, inclusive falha UNKNOWN_EXPORT_OBSERVATION. Não há alegação nova de paridade global ou ganho de leitura por IA.

## Evidência RED→GREEN

Antes da implementação, a suíte workspace com o teste novo passou com **{base_suite['passed']} testes, zero falhas e {base_suite['ignored']} ignorados**; o gate AST registrou **70 V16 FrameItem**. Depois, o mesmo gate registra **zero nessa família** e 242 V16 externos. A saída global continua em falha por essas pendências, explicitamente registrada. {link('baseline-details.json')} · {link('family-baseline-result.json')} · {link('family-green-result.json')}.

Na cópia descartável do workspace real, uma variante sentinela foi adicionada apenas ao FrameItem copiado. Nos **12 matches produtivos do core efetivamente alcançados por cargo check -p typst-core**, E0004 passou de **{pb['selected_e0004']}/12 antes a {pg['selected_e0004']}/12 depois**. Totais da cópia: {pb['total_e0004']} → {pg['total_e0004']} E0004, sem outro tipo de erro de compilação. Nenhum corpo foi substituído por stub. A sonda não certifica os helpers cfg(test) nem L3 barrado pela dependência; o replay nominal e a validação positiva abrangem os 70 locais. Definição produtiva do enum e contratos públicos permaneceram idênticos. {link('probe-baseline-details.json')} · {link('probe-green-details.json')} · {link('probe-baseline-input.json.gz')} · {link('probe-green-input.json.gz')}.

## Validação final

**{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados** na suíte completa. Passaram build, Clippy workspace/all-targets normal e estrito (-D warnings e -D unfulfilled_lint_expectations), formato, linhagem V1/V5/V15/V26 estrita e reversa, Rustdoc -D warnings com quatro índices HTML e git diff --check. Fontes, L0 e configuração ficaram congelados durante os comandos. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

A comparação SARIF por regra/nível/arquivo/mensagem não encontrou aviso novo. Há **{notes_count} notas novas**, mantidas em {link('new-notes.json')}: alternativas nominais agrupadas compartilham corpo e não autorizam afirmar cobertura de cada alternativa pelo número de braços. Não houve supressão ou mudança de configuração do linter. {link('architecture-delta.json')}.

Restam **242 V16 e 19 V12**. T027 e T158/T160 continuam rastreados apesar da ausência de emissão; F054 corrige o catchall FrameItem de metadata que consolidava T311/T313, sem fechar o caso SemanticKind distinto. A meta geral permanece aberta. {link('lista-pendencias.md')} · {link('latent-v16.json')}.

## RAM e proveniência

Execuções pesadas sequenciais em cgroup: **MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1**. Maior pico observado entre os serviços: **{peak:.3f} GiB**, incluindo cache; zero eventos OOM e OOM-kill. Detalhes de cada serviço e eventos max em {link('resources.json')} e {link('memory-samples.jsonl')}.

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação final {v['start']}–{v['end']} UTC. Lista exata de arquivos, hashes, status e git diff HEAD --stat: {link('before.json.gz')}, {link('baseline-details.json')}, {link('final-state.json')}. {link('manifest.sha256')} conserva a evidência da etapa.

Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150, SHA-256 26fe38b564a0aa92e7cc49f3b09a4324d1127e3586862afa5af4480e6c394dde. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido na suíte. Nenhum commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
files=sorted(p for p in out.rglob('*') if p.is_file() and p.name!='manifest.sha256' and '__pycache__' not in p.parts)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in files))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=0,tekt=len(remaining),tests=suite['passed'],matches=70,peak_gib=peak,manifest_files=len(files))))
