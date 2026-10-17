from pathlib import Path
import json,gzip,hashlib,re,datetime,subprocess,collections,difflib
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
before=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));baseline=json.load(open(out/'baseline-result.json'));validation=json.load(open(out/'validation-result.json'));workspace=json.load(open(out/'workspace/workspace-result.json'));delta=json.load(open(out/'architecture-delta.json'))
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==before['head']
assert subprocess.check_output(['git','branch','--show-current'],text=True).strip()=='Tekt'
assert validation['validation_complete'] and validation['sources_unchanged_during_validation'];assert all(x['exit']==(1 if x['label']=='architecture' else 0) for x in validation['commands']);assert all(sha(p)==h for p,h in validation['source_hashes'].items())
previous=json.load(open(out.parent/'tekt-v16-math-value-2026-10-16/workspace/workspace-result.json'));assert workspace['exit']==0 and workspace['sources_unchanged'] and workspace['failed']==0;assert workspace['passed']==previous['passed']+11 and workspace['ignored']==previous['ignored']
assert '6 passed; 5 failed;' in (out/'baseline-source-red.log').read_text();assert '11 passed; 0 failed;' in (out/'validation-controls.log').read_text()
probes={}
for enum,selected,total_before,total_after in [('Value',1,28,29),('Content',5,17,22)]:
 red=json.load(open(out/('probe-baseline-'+enum+'-result.json')));green=json.load(open(out/('probe-green-'+enum+'-result.json')));assert red['selected_e0004']==0 and green['selected_e0004']==selected;assert red['total_e0004']==total_before and green['total_e0004']==total_after;assert not red['other_errors'] and not green['other_errors'];probes[enum]={'before':red,'after':green}
assert delta['new_counts']=={'V16:error':239,'V19:note':900,'V20:note':871};assert delta['other_diagnostics_preserved_including_mapped_locations']
changed=sorted(p for p,h in before['hashes'].items() if sha(p)!=h);assert changed==sorted(scope['sources']+scope['owners'])
public=lambda s:re.findall(r'^\s*pub(?:\([^)]*\))?\s+[^\n]+',s,re.M)
for source in scope['sources']:assert public(before['files'][source])==public(Path(source).read_text()),source
patch=[]
for path in scope['sources']+scope['owners']:patch.extend(difflib.unified_diff(before['files'][path].splitlines(keepends=True),Path(path).read_text().splitlines(keepends=True),fromfile='a/'+path,tofile='b/'+path))
(out/'stage.patch').write_text(''.join(patch))
latent={'utc':now(),'head':before['head'],'closed_ids':{'T027':{'target':'I1','barrier':'Value sentinela produz E0004 no match do resultado do documento'},'T158':{'target':'I3','barrier':'Content sentinela produz E0004 no callback prepared_locations'},'T160':{'target':'I3','barrier':'mesmo ponto nominal após a união anterior; não é um segundo match'}},'other_latent_decisions_closed':['I2','I4','I5'],'remaining_emitted':239,'pending':[{'id':'OutlineEntry-alignment','source':'01_core/src/compiler/introspect/extract_payload.rs','status':'is_locatable true / extract_payload None; caminho sintetizado e certificado de preparação requerem investigação própria, não fechados por esta etapa'}]}
(out/'latent-status.json').write_text(json.dumps(latent,indent=2)+'\n')
resources={p.name:json.load(open(p)) for p in out.glob('*-resources.json')};assert resources['validation-resources.json']['exit']==0;peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());assert peak<=12*1024**3;events={}
for name,r in resources.items():
 assert r['memory_before']['memory.max']=='12884901888' and r['memory_before']['memory.swap.max']=='0' and r['cargo_jobs']=='1'
 e=dict(line.split() for line in r['memory_after']['memory.events'].splitlines());assert e['oom']=='0' and e['oom_kill']=='0';events[name]=e
max_events=max(int(e['max']) for e in events.values())
lines=['# Correções e decisões V19','','Medição: HEAD `'+before['head']+'`, working tree não commitado; hashes, UTC e diffstat em before.json.gz.','','| Ponto | Função | Decisão | Grupos V19 novos |','|---|---|---|---:|']
for item in delta['new_v19_equivalent_groups']:
 line=item['notes'][0]['locations'][0]['physicalLocation']['region']['startLine'];reason='normalizar Source e fechar ausência de payload por nomes' if item['id']=='I6' else 'fechar o domínio nominal preservando as consequências atuais';lines.append('| '+item['id']+' | ['+item['function']+']('+str(root/item['source'])+':'+str(line)+') | '+reason+' | '+str(item['alternatives_written'])+' |')
lines+=['','Os oito grupos partilham uma consequência ou efeito atual. V19 permanece informativo e prescrito em seus owners, sem supressão. Isso não comprova execução de todas as alternativas nos testes.','', 'I1 fecha T027; I3 fecha o ponto associado a T158/T160. I2, I4 e I5 fecham fallbacks implícitos. I6 corrige Source por normalização existente, preserva as delegações de elementos e fecha o único V16 emitido do lote. is_locatable recebe apenas qualificação documental; seu código executável não mudou.']
(out/'lista-correcoes.md').write_text('\n'.join(lines)+'\n')
pending=json.load(open(out/'remaining-v16.json'));assert len(pending)==239
lines=['# V16 e alinhamentos pendentes','','Snapshot: HEAD `'+before['head']+'`, working tree não commitado, pin `'+before['linter']['commit']+'`. 239 emissões abertas; esta lista não as classifica automaticamente como defeitos funcionais.','','| Consumer e linha | Diagnóstico |','|---|---|']
for item in pending:
 physical=item['locations'][0]['physicalLocation'];path=physical['artifactLocation']['uri'].removeprefix('./');line=physical['region']['startLine'];message=' '.join(item['message']['text'].split()).replace('|','\\|');lines.append('| ['+path+':'+str(line)+']('+str(root/path)+':'+str(line)+') | '+message+' |')
lines+=['','## Alinhamento ainda não fechado','','OutlineEntry continua true no classifier e None no extractor; há caminho sintetizado e pendência expressa nos dois L0. Investigar a semântica do construtor, a travessia e a prova de preparação antes de corrigir. Não fabricar payload ou remover a classificação para satisfazer uma igualdade Rust. Outros catchalls internos também exigem auditoria.','', 'T027 e T158/T160 foram fechados nesta etapa, com barreiras locais verificadas em código real; [estado dos latentes](latent-status.json).']
(out/'lista-pendencias.md').write_text('\n'.join(lines)+'\n')
docs={}
for name in ['target/doc/typst_core/index.html','target/doc/typst_infra/index.html','target/doc/typst_shell/index.html','target/doc/typst/index.html']:
 p=Path(name);assert p.is_file();docs[name]={'sha256':sha(p),'mtime_ns':p.stat().st_mtime_ns}
report=f'''# V16 — origens, extração e Source

Etapa concluída; objetivo geral continua aberto.

## Resultado

**V16: 240 → 239.** Além da emissão corrigida em extract_payload, foram fechados os casos latentes T027 e T158/T160 e três predicados com fallback implícito. São seis decisões protegidas contra variantes futuras: uma sobre Value e cinco sobre Content. Os IDs T158/T160 pertencem ao mesmo ponto após a união anterior e não foram contados como dois matches novos.

A auditoria também confirmou e corrigiu falha real: Source(Heading/Strong/Emph/Metadata) perdia o payload do body, e Located(Source(Heading)) não qualificava como origem preparada. Normalizar pelo source_body existente restaura o núcleo de origem transparente. needs_fresh_index também normaliza Source, conservando o fallback das classes especiais. Nenhuma Location, payload próprio de Source, callback adicional, API, modo padrão ou fase nova foi criado. A coleta conserva as Locations já preparadas sem duplicação, inclusive no original de Realized.

V19 **892 → 900 notas**, somente os oito grupos uniformes previstos nos L0. V20 permanece **871 notas**. Não houve supressões, exceções globais ou mudança de configuração. [Lista de correções e grupos](lista-correcoes.md) · [patch completo da etapa](stage.patch) · [pendências](lista-pendencias.md).

## Verificação

- RED funcional: **cinco falhas e seis controles passando** antes da implementação, em [baseline-source-red.log](baseline-source-red.log). GREEN: **11 passaram**, em [validation-controls.log](validation-controls.log).
- Os testes existentes single_evaluation_ passaram antes/depois, preservando execução única e conteúdo/introspecção de origem/produto.
- Sentinela Value no workspace real: barreira selecionada **0 → 1 E0004**, total **28 → 29**; sentinela Content separada: **0 → 5 E0004**, total **17 → 22**. Sem outros erros, sem mutação dos fontes produtivos; entradas, logs e resultados de cada sonda foram preservados.
- Suíte completa: **{workspace['passed']} passaram, zero falhas, {workspace['ignored']} ignorados preexistentes**, 11 testes a mais que o último baseline completo. [Resultado workspace](workspace/workspace-result.json).
- Build, fmt, Clippy all-targets com warnings/expectativas não cumpridas como erro, Rustdoc workspace com -D warnings, V1/V5/V15/V26 estritos, linhagem reversa e diff-check passaram. [validation-result.json](validation-result.json).
- [architecture-delta.json](architecture-delta.json) compara todos os resultados SARIF, preservando mensagens, níveis, multiplicidade e posições após mapear deslocamentos de linha. Remove somente o V16 selecionado e adiciona somente oito V19. O gate global continua exit 1 pelos 239 V16 pendentes.

## Refinamentos e integridade

O guard de resselo impediu aplicar um pin bruto de bytes do TOML em vez do digest canônico; a tentativa ficou em cópia descartável, com o log preservado. O novo consumer pin de source-origin usa o digest canônico **4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67**. O arquivo do núcleo não mudou; seu SHA dos bytes é distinto e consta no snapshot. V26 valida o grafo final.

A primeira captura other @ (grupo nominal) acrescentava profundidade V20 sem necessidade. [flattening-before.json.gz](flattening-before.json.gz) e preview-architecture.log preservam a medição; o L0 foi refinado antes da troca por grupo plano, reutilizando node no corpo por alpha-renomeação. A exaustividade e as mesmas chamadas/curto-circuito foram conservadas, sem a nota V20 adicional. [flat-check.json](flat-check.json).

Somente quatro consumers e seus quatro owners mudaram no inventário produtivo: eval/mod.rs, layout/realized.rs, introspect/extract_payload.rs e introspect/locatable.rs. Este último recebeu somente documentação/metadados; o guard confirma código executável intacto. Assinaturas públicas, módulos e campos permanecem. As delegações existentes foram preservadas, acrescentando somente a normalização Source descrita acima. [code-guard.json](code-guard.json), materialization.json e l0-refinement.json preservam as versões prescritas e o ajuste plano. A revisão vigente substitui a ressalva histórica de fallback aberto no extractor; OutlineEntry continua pendência explícita.

## RAM e proveniência

Execução por cgroup com teto **12 GiB**, swap **0**, Cargo **um job** e comandos pesados sequenciais. Pico: **{peak} bytes ({peak/1024**3:.3f} GiB)**; sem OOM ou OOM kill. Maior contador memory.events/max: {max_events}; os registros preservam a pressão no limitador. Não se infere ganho de desempenho desta correção. [Recursos](validation-resources.json).

HEAD `{before['head']}`, branch Tekt, **working tree não commitado**. [before.json.gz](before.json.gz) conserva UTC, lista exata git diff HEAD --stat, status e hashes completos; [final-state.json.gz](final-state.json.gz) conserva o estado após validação. As fontes permaneceram congeladas durante os gates. Oracle da suíte: pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, verificado pelo executor. Linter: `{before['linter']['commit']}`, binário SHA-256 `{before['linter']['sha256']}`. Não houve commit, push, release, merge ou alteração de pin do CI nesta etapa.

## Continuidade

Restam **239 emissões V16** e a investigação do alinhamento/certificado de preparação de OutlineEntry. A divergência true/None e o caminho sintetizado são anteriores e foram preservados de modo explícito; não foram declarados resolvidos pela exaustividade. O próximo lote deve medir esse caso e seus produtores/travessia antes de decidir. O fechamento desta etapa não encerra o objetivo geral.
'''
(out/'relatorio.md').write_text(report)
record={'utc':now(),'head':before['head'],'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'source_hashes':validation['source_hashes'],'scope':scope,'linter':before['linter'],'changed':changed,'public_signatures_preserved':True,'tests':workspace,'source_red_failures':5,'controls_green':11,'architecture':delta,'probes':probes,'resources':resources,'events':events,'peak_bytes':peak,'peak_gib':peak/1024**3,'docs':docs,'latent_status':latent,'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'stage_complete':True,'goal_scope_incomplete':True,'remaining_v16':239,'no_commit_push_release':True}
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(record,f)
manifest={str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest-sha256.json'};(out/'manifest-sha256.json').write_text(json.dumps(manifest,indent=2)+'\n');assert all(sha(out/p)==h for p,h in manifest.items())
print(json.dumps({'stage_complete':True,'V16':239,'V19':900,'V20':871,'passed':workspace['passed'],'failed':0,'ignored':workspace['ignored'],'peak_gib':peak/1024**3,'manifest_files':len(manifest),'goal_scope_incomplete':True}),flush=True)
