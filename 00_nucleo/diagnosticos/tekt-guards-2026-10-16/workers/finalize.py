from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,difflib,collections,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-guards-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');service=load('validation-result.json');suite=load('workspace/workspace-result.json');scope=load('scope.json');strict=load('strict-details.json');baseline=load('baseline-details.json');base_suite=load('baseline-workspace/workspace-result.json');delta=load('architecture-delta.json')
assert service['exit']==0 and len(v['commands'])==12 and all(c['exit']==0 for c in v['commands'])
assert not load('clippy-inventory.json') and not strict['diagnostics'] and strict['exit']==0 and v['clippy_delta']=={}
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged'] and suite['passed']==8617 and suite['ignored']==7
assert baseline['sources_unchanged'] and [c['exit'] for c in baseline['commands']]==[0,1] and base_suite['exit']==0 and base_suite['passed']==suite['passed']
assert v['architecture_warnings']=={'V16':323,'V12':19} and delta['v16_stage_not_closed']
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
first_service=load('attempt-1/validation-result.json');assert first_service['exit']==1 and load('attempt-1/validation-details.json')['sources_unchanged_during_validation'];assert load('resume-proof.json')['only_reverse_metadata_changed']
for rec in [first_service,load('baseline-result.json')]:
 ev=dict(s.split() for s in rec['memory_after']['memory.events'].splitlines());assert ev['oom']=='0' and ev['oom_kill']=='0' and int(rec['memory_after']['memory.max'])==12*1024**3 and int(rec['memory_after']['memory.swap.max'])==0
mem=service['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());peak=max(int(mem['memory.peak']),int(first_service['memory_after']['memory.peak']))/1024**3;assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0'
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],clippy=0,tekt=v['architecture_warnings'],suite=suite,memory_peak_gib=peak,v16_tracking='architecture-delta.json and latent-v16.json retain cases no longer emitted after arm consolidation')
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n')
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(before['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p in scope['sources']+scope['owners']))
results=json.loads((out/'validation-architecture.log').read_text())['runs'][0]['results'];remaining=[]
for r in results:
 if r['level']!='warning':continue
 loc=r['locations'][0]['physicalLocation'];p=loc['artifactLocation']['uri'].removeprefix('./');owner=re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M)[1];remaining.append(dict(rule=r['ruleId'],source=p,line=loc['region']['startLine'],owner=owner,message=r['message']['text']))
(out/'remaining.json').write_text(json.dumps(remaining,ensure_ascii=False,indent=2)+'\n')
latent=[]
for d in delta['v16_dispositions'][:2]:
 p=d['source'];t=Path(p).read_text();needle='_ => None,' if d['ids']==['T027'] else 'other => {';start=t.index('let content = match value') if d['ids']==['T027'] else t.index('fn prepared_locations');line=t[:t.index(needle,start)].count('\n')+1
 latent.append(dict(**d,line=line,owner=re.search(r'^//! @prompt (.+)$',t,re.M)[1]))
(out/'latent-v16.json').write_text(json.dumps(latent,ensure_ascii=False,indent=2)+'\n')
text=f'''# Pendências Tekt — após V17

**342 avisos emitidos**, Clippy zero; HEAD `{before['head']}`, working tree não commitado. Medição {v['end']} UTC; validation-details.json e final-state.json registram hashes, status e git diff HEAD --stat. Notas informativas ficam no SARIF e não entram como avisos.

| Ordem | Frente | Situação |
|---|---|---|
| 1 | V16 — wildcards de domínio | 323 avisos emitidos; conferir também os dois locais sem emissão abaixo |
| 2 | V12 — tipos/impl em L4 | 19 avisos: um enum produtivo e 18 declarações de fixtures |

Os 28 V17 foram corrigidos. A queda adicional de quatro V16 resulta de união de braços e heurísticas da regra: não é prova de quatro correções por exaustividade. T027 deixou de ser emitido quando dois braços Value::Content foram unidos e o match passou a ter um único braço com prefixo qualificado. T158/T160 viraram um braço com binding reincorporado no classificador; essa forma é isenta na regra, sem nova enumeração nominal. T311/T313 continuam como um único aviso no fallback de metadata. Não perder estes IDs na próxima auditoria. architecture-delta.json preserva as mensagens antes/depois, sem novo local de warning.

## Locais preservados para decisão, mesmo sem emissão atual

| IDs originais | Local | Decisão pendente |
|---|---|---|
'''
for d in latent:text+=f"| {', '.join(d['ids'])} | [{d['source']}:{d['line']}]({root/d['source']}:{d['line']}) | {d['status']} — {d['reason']} |\n"
text+='''
Diagnóstico, não L0 ou autorização de API. Ler cada owner e seu enum/classificador antes da prescrição. Não usar citação para isentar V16, rebaixar regras globais ou mover fixtures de camada apenas para esconder avisos. V12 exige separar composição produtiva de suporte de teste e conferir o contrato aplicável.

## Avisos emitidos

| Regra | Local | Owner | Evidência |
|---|---|---|---|
'''
for r in remaining:text+=f'| {r["rule"]} | [{r["source"]}:{r["line"]}]({root/r["source"]}:{r["line"]}) | [{r["owner"]}]({root/r["owner"]}) | '+r['message'].replace('|','\\|').replace('\n',' ')+' |\n'
(out/'lista-pendencias.md').write_text(text)
link=lambda p:f'[{p}]({out/p})'
report=f'''# V17 — guards compostos: etapa concluída

**28 V17 → zero**, sem supressões, mudança de limite ou API. **Clippy zero**, inclusive workspace/all-targets com -D warnings e expectativas não cumpridas como erro. Atualizados **21 consumers e 21 L0** com prescrição por caso antes da implementação; resselo restrito. {link('classification.json')} · {link('stage.diff')} · {link('code-guard.json')}.

## O que foi corrigido

- Presença de campos e snapshots: braços unificados com escolha explícita entre accessor declarado e getter; snapshot Some continua autoritativo mesmo sem a chave. Sem snapshot, conteúdo comum conserva getter. O texto do adendo foi precisado após a baseline e antes do código, sem alterar os testes ou seu comportamento; recibo l0-clarification.json.
- Avaliação e spans: preservados os oito alvos por identidade nativa, With recursivo, curto-circuito e realização única; caminho sem recipe/preparo continua sem realizar.
- Grid/Table, origens e tracking: elegibilidade/fallback no corpo dos braços, sem ampliar cache, remover células com atributos ou repetir callbacks; RTL continua com tracking assinado e barreiras.
- Matrizes: modos array/delimitado explícitos em padrões de tupla; mistura continua inválida; getters/repr de augment conservam inteiro/dict/presença e stroke auto.
- Caption: padrões estruturais Top/Bottom sem horizontal; mesmos casts, diagnósticos e defaults.
- Cores e níveis de tabela: condições no corpo, mesmos intervalos, casts, mensagens, Debug e interpolação spot/processual. Sem mudança de fórmulas ou tolerâncias.
- PDF/HTML: Link e folhas MCID conservam Artifact, contador, ordem e recursão; Raw inelegível mantém erro sem observar texto; átomos typc conservam escape/code/span.
- Decoder de teste HTML: prefixo #x/#X reconhecido por operação ASCII segura para UTF-8; mesma validação Unicode/decimal e decodificação única.

A regra CI V17 recomenda desdobrar a condição ou movê-la para o corpo. Não foi criado helper para esconder conjunções. Este lote melhora a explicitação local de decisões; não mede benefício de leitura por IA, ganho de RAM ou paridade universal. As assinaturas públicas e o restante das fontes/configuração estão preservados por replay exato de 30 substituições, formatação restrita e hash global.

## Testes antes e depois

Dois controles novos foram escritos antes do código: componentes Relative em 0/interior/1, NaN/infinito/out-of-range e parte absoluta não zero nos dois casters; entidades HTML com #X, Unicode escalar, surrogate, valor acima do máximo, truncamento, prefixos UTF-8 inválidos e decodificação única. Os controles protegem comportamento existente e passaram antes; não foi inventada uma falha de produto. Os esperados históricos ficaram intactos.

Baseline: **{base_suite['passed']} aprovados, zero falhas, {base_suite['ignored']} ignorados**. O gate --checks v17 --fail-on warning falhou com exatamente os 28 avisos anteriores. Após a implementação o mesmo gate passou sem V17. {link('baseline-details.json')} · {link('baseline-red.log')} · {link('validation-v17.log')}.

Final: **{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados**. Build, Clippy normal/estrito, fmt, linhagem V1/V5/V15/V26 estrita e reversa, diff --check e Rustdoc -D warnings passaram; os quatro índices HTML existem. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

O primeiro runner de baseline teve erro de sintaxe antes de iniciar Cargo; foi corrigido e reexecutado, com logs do erro preservados. Nenhuma medição desse runner é usada como validação.

A primeira rodada final parou na linhagem reversa: o --fix-hashes reparou apenas pares com drift de prompt, deixando 20 hashes reversos de código antigos após o seal pré-baseline. O worker restrito foi corrigido para solicitar resselo de todos os pares selecionados numa cópia temporária e copiar somente header/metadado autorizados. A correção mudou apenas 20 linhas Hash do Código em L0; fontes Rust, configurações e texto normativo ficaram idênticos. resume-proof.json recompõe e confere os hashes anteriores. Build, Clippy normal/estrito, fmt e replay aprovados foram reutilizados; linhagem direta/reversa foi repetida e os checks restantes foram executados. attempt-1/ preserva a falha e recursos; reverse-reseal.log registra o reparo. Não houve relaxamento do gate.

## Contagem Tekt e continuidade

O SARIF passou de 374 a **342 avisos emitidos**: V16=323 e V12=19, sem erros. A redução é composta por 28 V17 corrigidos e quatro V16 afetados pela união/heurística dos braços, **sem certificação de exaustividade**. T027 e T158/T160 permanecem rastreados para auditoria mesmo sem emissão; T311/T313 viraram um único aviso ainda pendente. O único texto novo de V16 é esse fallback consolidado de metadata, não um novo local. {link('architecture-delta.json')} · {link('latent-v16.json')}.

Próxima ordem: V16, incluindo os locais preservados sem aviso, depois V12. Lista nominal com arquivos, linhas e L0: {link('lista-pendencias.md')}. A meta geral continua aberta; esta etapa conclui apenas V17.

## Recursos e proveniência

Jobs pesados sequenciais em cgroup **MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1**. Pico final **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. A baseline atingiu {int(load('baseline-result.json')['memory_after']['memory.peak'])/1024**3:.3f} GiB, também sem OOM. {link('validation-result.json')} · {link('memory-samples.jsonl')}.

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação final {v['start']}–{v['end']} UTC. Fontes/L0/configurações permaneceram congelados durante os comandos de cada rodada; entre as duas rodadas finais houve somente a correção dos metadados reversos comprovada em resume-proof.json; {link('before.json.gz')}, {link('baseline-details.json')} e {link('final-state.json')} guardam conteúdo, hashes, estado, status e git diff HEAD --stat.

Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150, SHA-256 26fe38b564a0aa92e7cc49f3b09a4324d1127e3586862afa5af4480e6c394dde reconfirmado. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/ e {link('manifest.sha256')}. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
files=sorted(p for p in out.rglob('*') if p.is_file() and p.name!='manifest.sha256' and '__pycache__' not in p.parts)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in files))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=0,tekt=len(remaining),tests=suite['passed'],peak_gib=peak,manifest_files=len(files))))
