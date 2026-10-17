from pathlib import Path
import json,gzip,hashlib,re,datetime,subprocess,collections,difflib
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-math-value-2026-10-16'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
branch=subprocess.check_output(['git','branch','--show-current'],text=True).strip()
before=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));baseline=json.load(open(out/'baseline-result.json'));validation=json.load(open(out/'validation-result.json'));workspace=json.load(open(out/'workspace/workspace-result.json'));delta=json.load(open(out/'architecture-delta.json'));red=json.load(open(out/'probe-baseline-result.json'));green=json.load(open(out/'probe-green-result.json'));materialization=json.load(open(out/'materialization.json'))
assert validation['validation_complete'] and validation['sources_unchanged_during_validation']
assert all(x['exit']==(1 if x['label']=='architecture' else 0) for x in validation['commands'])
assert all(sha(p)==h for p,h in validation['source_hashes'].items())
assert workspace['exit']==0 and workspace['failed']==0 and workspace['sources_unchanged']
previous=json.load(open(out.parent/'tekt-v16-value-projections-2026-10-16/workspace/workspace-result.json'))
assert workspace['passed']==previous['passed']+7 and workspace['ignored']==previous['ignored']
assert red['selected_e0004']==0 and green['selected_e0004']==4
assert red['total_e0004']==24 and green['total_e0004']==28 and not red['other_errors'] and not green['other_errors']
assert delta['other_diagnostics_preserved_including_mapped_locations']
assert delta['new_counts']=={'V16:error':240,'V19:note':892,'V20:note':871}
for source in scope['sources']:
 public=lambda text:re.findall(r'^\s*pub(?:\([^)]*\))?\s+[^\n]+',text,re.M)
 assert public(before['files'][source])==public(Path(source).read_text()),source
patch=[]
for path in scope['sources']+scope['owners']:
 patch.extend(difflib.unified_diff(before['files'][path].splitlines(keepends=True),Path(path).read_text().splitlines(keepends=True),fromfile='a/'+path,tofile='b/'+path))
(out/'stage.patch').write_text(''.join(patch))
changed=sorted(p for p,h in before['hashes'].items() if sha(p)!=h)
assert changed==sorted(scope['sources']+scope['owners'])
resources={p.name:json.load(open(p)) for p in out.glob('*-resources.json')};assert resources['validation-resources.json']['exit']==0
peak=max(int(x['memory_after']['memory.peak']) for x in resources.values());assert peak<=12*1024**3
memory_events={}
for name,r in resources.items():
 assert r['memory_before']['memory.max']=='12884901888' and r['memory_before']['memory.swap.max']=='0' and r['cargo_jobs']=='1'
 events=dict(line.split() for line in r['memory_after']['memory.events'].splitlines());assert events['oom']=='0' and events['oom_kill']=='0'
 memory_events[name]=events
old_classification=(out/'classification.json').read_bytes();(out/'classification-planning.json').write_bytes(old_classification)
classification={'utc':now(),'before':before['utc'],'head':before['head'],'working_tree':'não commitado','sources_changed':scope['sources'],'owners_changed':scope['owners'],'corrections':[]}
for target,note in zip(scope['targets'],delta['new_v19_equivalent_groups']):
 assert target['id']==note['id']
 classification['corrections'].append({**target,'status':'V16 corrigido por exaustividade nominal','V19':'manter informativo; grupo de consequência e efeitos uniformes prescrito no L0','new_note':note['group']})
(out/'classification.json').write_text(json.dumps(classification,indent=2)+'\n')
lines=['# Correções e grupos V19 preservados','','Medição: working tree não commitado, HEAD `'+before['head']+'`; before.json.gz conserva hashes, UTC, status e diffstat.','','| Caso | Consumer / função | Linha anterior | Grupo atual | Decisão |','|---|---|---:|---:|---|']
for target,note in zip(scope['targets'],delta['new_v19_equivalent_groups']):
 new_line=note['group']['locations'][0]['physicalLocation']['region']['startLine'];lines.append('| '+target['id']+' | ['+target['function']+']('+str(root/target['source'])+':'+str(new_line)+') | '+str(target['line'])+' | '+str(len(target['complement']))+' variantes | Corrigir wildcard; manter V19 informativo |')
lines+=['','M1–M3 preservam None para os tipos atuais sem representação de delimitador. M4 preserva os bits do Stroke já validado; atalhos Length/Color continuam marcando somente seu campo. As alternativas partilham o mesmo corpo; não constituem decisões distintas. Os sete controles exercitam os casos descritos nos L0, sem alegar cobertura de execução de cada variante dos grupos.','', 'As fronteiras existentes que devolvem erro continuam com o mesmo diagnóstico; não foram convertidas em ausência silenciosa. Nenhuma supressão/configuração foi adicionada.']
(out/'lista-correcoes.md').write_text('\n'.join(lines)+'\n')
pending=json.load(open(out/'remaining-v16.json'));assert len(pending)==240
lines=['# V16 ainda pendentes','','Snapshot fechado desta etapa: HEAD `'+before['head']+'`, working tree não commitado e pin `'+before['linter']['commit']+'`. São 240 emissões abertas. validation-result.json conserva hashes e UTC.','','| Consumer e linha | Diagnóstico |','|---|---|']
for item in pending:
 physical=item['locations'][0]['physicalLocation'];path=physical['artifactLocation']['uri'].removeprefix('./');line=physical['region']['startLine'];message=' '.join(item['message']['text'].split()).replace('|','\\|');lines.append('| ['+path+':'+str(line)+']('+str(root/path)+':'+str(line)+') | '+message+' |')
lines+=['','## Casos latentes mantidos','','T027 em eval/mod.rs e T158/T160 em layout/realized.rs seguem abertos, com fontes iguais às do [inventário anterior](../tekt-v16-value-projections-2026-10-16/latent-v16.json). A ausência de emissão não fecha esses casos. O próximo lote deve auditar os respectivos L0 e os classificadores de payload/necessidade de índice. Outros Some(_) e catchalls internos requerem análise própria.','', 'Os 892 V19 são notas informativas; não são automaticamente defeitos nem prova de cobertura das alternativas.']
(out/'lista-pendencias.md').write_text('\n'.join(lines)+'\n')
docs={}
for path in ['target/doc/typst_core/index.html','target/doc/typst_shell/index.html','target/doc/typst_infra/index.html','target/doc/typst/index.html']:
 p=Path(path);assert p.is_file(),path;docs[path]={'sha256':sha(p),'bytes':p.stat().st_size,'mtime_ns':p.stat().st_mtime_ns}
existing=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (out/'baseline-existing-math.log').read_text(),re.M)
existing_passed=sum(int(x[1]) for x in existing)
event_max=max(int(x['max']) for x in memory_events.values())
report=f'''# V16 — projeções matemáticas de Value

Etapa concluída; objetivo geral de correções permanece aberto.

## Resultado

**V16: 244 → 240.** Quatro fallbacks internos foram substituídos pelo complemento nominal de Value em três consumers e seus três L0. Assinaturas, corpos das consequências, defaults, fase, mensagens, spans e carriers foram preservados. V19: **888 → 892 notas**, com quatro grupos equivalentes mantidos e prescritos individualmente nos L0; V20 permanece **871 notas**. Não houve isenção nem mudança de configuração para reduzir o gate.

[Patch desta etapa](stage.patch) · [Lista dos quatro casos e justificativas V19](lista-correcoes.md) · [classificação](classification.json) · [240 pendências e casos latentes](lista-pendencias.md).

M1 parse_delim_val e M2 parse_delim_char, em eval/math/delimiters.rs, preservam aliases, Unicode, None e os defaults de arrays. M3 vec_delim_char preserva a mesma distinção de ausência/primeiro caractere e a conversão de arrays do vetor. M4, em math_matrix_augment, preserva a presença de stroke já validado: Length marca thickness, Color marca paint e Stroke mantém seu payload/bits. Entradas inválidas continuam rejeitadas pelo extractor antes dessa decisão.

A comparação [SARIF anterior](baseline-architecture.log) / [novo](validation-architecture.log) removeu somente os quatro V16 e adicionou somente os quatro V19 previstos. [architecture-delta.json](architecture-delta.json) compara todos os demais resultados, níveis, mensagens, multiplicidade e localizações, mapeando exatamente os deslocamentos de linha nos três arquivos editados.

## Testes e proteção de evolução

Os sete testes foram escritos após os L0 e antes da implementação: passam antes e depois, preservando o comportamento existente. Os controles P1431 também passaram antes ({existing_passed} testes no filtro). O RED estrutural era V16 nos quatro pontos e ausência de proteção local contra variante nova.

Em cópia descartável do workspace real, adicionar TektMathValueProbe a Value produz **0 → 4 E0004 nos pontos selecionados** e **24 → 28 E0004 no total**, sem outros erros. Já havia barreiras em outros owners; esta etapa acrescenta a barreira em cada uma das quatro decisões. Os fontes produtivos permaneceram intactos durante a sonda. [Prova anterior](probe-baseline-result.json) · [nova](probe-green-result.json); entradas completas e logs estão nos mesmos diretórios.

Suíte completa: **{workspace['passed']} passaram, zero falhas e {workspace['ignored']} ignorados preexistentes**, mais sete testes que o último baseline completo. Oracle: pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, verificado pelo executor. [Resultado do workspace](workspace/workspace-result.json).

Build workspace, fmt, Clippy all-targets com warnings/expectativas não cumpridas como erro, Rustdoc workspace com -D warnings, V1/V5/V15/V26 estritos, linhagem reversa e diff-check passaram. [validation-result.json](validation-result.json) registra comandos/UTC e prova que os hashes dos fontes não mudaram durante a validação. O gate arquitetural global continua exit 1 pelos 240 V16 pendentes.

## Recursos e proveniência

Limite por cgroup **12 GiB (12884901888 bytes)**, swap **0**, Cargo **1 job**, comandos pesados sequenciais. Pico medido: **{peak} bytes ({peak/1024**3:.3f} GiB)**. Zero OOM e OOM kill; maior contador de pressão memory.events/max: {event_max}. Não se infere ganho de desempenho/memória desta manutenção. [Recursos finais](validation-resources.json).

HEAD `{before['head']}`, branch {branch}, **working tree não commitado**. [before.json.gz](before.json.gz) conserva lista exata git diff HEAD --stat, status, fontes e hashes antes de L0/código; [planning-before-pin.json.gz](planning-before-pin.json.gz) conserva o planejamento anterior à aplicação do pin autorizado no lote precedente. O snapshot efetivo desta etapa já usa o pin publicado `{before['linter']['commit']}`, binário `{before['linter']['binary']}`, SHA-256 `{before['linter']['sha256']}`. Datas do diretório seguem America/Sao_Paulo (16/10); os registros UTC são 17/10.

Somente os três consumers e seus três owners mudaram no inventário produtivo. [code-guard.json](code-guard.json) e [materialization.json](materialization.json) conservam o diff prescrito, preservação dos testes e corpos, escopo e hashes bidirecionais. Não houve commit, push, merge, release ou nova alteração de pin nesta etapa. Relatórios e passos permanecem fora de prompts/.

## Continuidade

Permanecem 240 emissões V16, os casos latentes T027 e T158/T160, e a auditoria dos demais catchalls internos. A correção atual é estrutural de evolução Rust; não declara novo ganho de paridade da linguagem nem cobertura de execução das 39 variantes. Próximo lote: auditar os owners de eval/mod.rs e layout/realized.rs e os classificadores usados na prova de completude antes de prescrever a correção dos latentes.
'''
(out/'relatorio.md').write_text(report)
record={'utc':now(),'head':before['head'],'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'source_hashes':validation['source_hashes'],'linter':before['linter'],'scope':scope,'changed':changed,'public_signatures_preserved':True,'tests':workspace,'architecture':delta,'evolution_red':red,'evolution_green':green,'resources':resources,'events':memory_events,'peak_bytes':peak,'peak_gib':peak/1024**3,'rustdoc_artifacts':docs,'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cargo':subprocess.check_output(['cargo','-V'],text=True),'stage_complete':True,'goal_scope_incomplete':True,'remaining_v16':240,'latent_ids_still_open':['T027','T158','T160'],'no_commit_push_release':True}
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(record,f)
manifest={str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest-sha256.json'}
(out/'manifest-sha256.json').write_text(json.dumps(manifest,indent=2)+'\n');assert all(sha(out/p)==h for p,h in manifest.items())
print(json.dumps({'stage_complete':True,'V16':240,'V19':892,'passed':workspace['passed'],'failed':0,'ignored':workspace['ignored'],'peak_gib':peak/1024**3,'files_in_manifest':len(manifest),'goal_scope_incomplete':True}),flush=True)
