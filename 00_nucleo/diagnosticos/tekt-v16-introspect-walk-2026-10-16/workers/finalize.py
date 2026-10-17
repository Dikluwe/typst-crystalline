from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,re,difflib
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for part in iter(lambda:f.read(4*1024*1024),b''):h.update(part)
 return h.hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));v=json.load(open(out/'validation-result.json'));w=json.load(open(out/'workspace/workspace-result.json'));delta=json.load(open(out/'architecture-delta.json'));parity=json.load(open(out/'parity-result.json'))
assert v['validation_complete'] and v['sources_unchanged_during_validation'];assert all(sha(p)==h for p,h in v['source_hashes'].items());assert all(c['exit']==(1 if c['label']=='architecture' else 0) for c in v['commands'])
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==before['head'];assert subprocess.check_output(['git','branch','--show-current'],text=True).strip()=='Tekt'
changed=sorted(p for p,h in before['hashes'].items() if sha(p)!=h);assert changed==sorted(scope['sources']+scope['owners']);assert len(changed)==20
public=lambda s:re.findall(r'^\s*pub(?!\()\s+[^\n]+',s,re.M)
for source in scope['sources']:assert public(before['files'][source])==public(Path(source).read_text()),source
new_tests=sum(Path(p).read_text().count('#[test]')-before['files'][p].count('#[test]') for p in scope['sources']);assert new_tests==18,new_tests
previous=json.load(open(out.parent/'tekt-v16-introspect-origins-2026-10-16/workspace/workspace-result.json'));assert w['exit']==0 and w['sources_unchanged'] and w['failed']==0 and w['passed']==previous['passed']+new_tests and w['ignored']==previous['ignored']
for log,result in [('baseline-red.log','1 passed; 7 failed;'),('source-refinement-red.log','7 passed; 2 failed;'),('methods-refinement-red.log','10 passed; 2 failed;'),('order-refinement-functional-red.log','10 passed; 4 failed;'),('shell-refinement-red.log','1 passed; 2 failed;'),('validation-controls.log','16 passed; 0 failed;'),('validation-shell-controls.log','3 passed; 0 failed;')]:assert result in (out/log).read_text(),log
assert re.findall(r'error\[([^]]+)\]',(out/'order-refinement-contract-red.log').read_text())==['E0599']*6
red=json.load(open(out/'probe-baseline-result.json'));green=json.load(open(out/'probe-green-result.json'));assert red['selected']==0 and green['selected']==6 and green['auxiliary_receiver_e0004'];assert red['total_e0004']==22 and green['total_e0004']==29 and not red['other_errors'] and not green['other_errors']
assert delta['new_counts']=={'V16:error':233,'V19:note':907,'V20:note':871} and delta['other_diagnostics_preserved_including_mapped_locations'];assert len(parity['checks'])==9
measurement=json.load(open(out/'measurement-after.json'));assert all(sha(p)==h for p,h in measurement['binaries'].items());assert sha(before['linter']['binary'])==before['linter']['sha256']
resources={p.name:json.load(open(p)) for p in out.glob('*-resources.json')};assert resources['validation-resources.json']['exit']==0;peak=max(int(r['memory_after']['memory.peak']) for r in resources.values());assert peak<=12*1024**3;events={}
for name,r in resources.items():
 assert r['memory_before']['memory.max']=='12884901888' and r['memory_before']['memory.swap.max']=='0' and r['cargo_jobs']=='1'
 e=dict(line.split() for line in r['memory_after']['memory.events'].splitlines());assert e['oom']=='0' and e['oom_kill']=='0';events[name]=e
max_events=max(int(e['max']) for e in events.values())
patch=[]
for p in scope['sources']+scope['owners']:patch.extend(difflib.unified_diff(before['files'][p].splitlines(keepends=True),Path(p).read_text().splitlines(keepends=True),fromfile='a/'+p,tofile='b/'+p))
(out/'stage.patch').write_text(''.join(patch))
cleanup=[]
for binary,receipt in [('trace-eval','trace-result.json'),('trace-runtime','trace-runtime-result.json')]:
 p=out/binary
 if p.exists():
  r=json.load(open(out/receipt));assert sha(p)==r['binary_sha256'];cleanup.append({'path':binary,'sha256':r['binary_sha256'],'bytes':p.stat().st_size,'reason':'derived diagnostic executable; source, command, linked library hashes and outputs retained'});p.unlink()
(out/'generated-artifact-cleanup.json').write_text(json.dumps({'utc':now(),'removed':cleanup},indent=2)+'\n')
docs={}
for p in ['target/doc/typst_core/index.html','target/doc/typst_infra/index.html','target/doc/typst_shell/index.html','target/doc/typst/index.html']:
 assert Path(p).is_file();docs[p]={'sha256':sha(p),'mtime_ns':Path(p).stat().st_mtime_ns}
rows=['# Correções e grupos V19','','HEAD `'+before['head']+'`, working tree não commitado; estado completo, UTC e diffstat em before.json.gz.','','| Ponto | Função | Alternativas com mesma consequência |','|---|---|---:|']
for target,note in zip(scope['targets'],delta['new_v19_groups'][:6]):
 loc=note['locations'][0]['physicalLocation'];line=loc['region']['startLine'];rows.append('| '+target['id']+' | ['+target['function']+']('+str(root/target['source'])+':'+str(line)+') | '+str(target['group_size'])+' |')
note=delta['new_v19_groups'][6];line=note['locations'][0]['physicalLocation']['region']['startLine'];rows.append('| Receptor | [outline_entry_receiver]('+str(root/scope['additional_group']['source'])+':'+str(line)+') | 118 |')
rows+=['','W6 é o oráculo de teste old_reserve; cinco dos seis V16 removidos estavam em produção. Os sete grupos têm consequência uniforme e foram prescritos nos L0, sem supressão. Não são prova de execução de todas as alternativas.','', 'O complemento de extract_payload perde OutlineEntry (99→98); needs_fresh_index move OutlineEntry entre true/false (6→7 e 114→113). Os demais diagnósticos ficam iguais após mapear linhas.','', 'Correções funcionais: delegação do payload e alinhamento de classificação; preservação de Location/modo raso em Source; body e métodos da entry através de Source/Located; índice por tipo; ordem de entries sintetizadas na posição causal do outline, com prefixos de state/counter; projeção legada de level/element/fill em L2, mantendo snapshots Some autoritativos.']
(out/'lista-correcoes.md').write_text('\n'.join(rows)+'\n')
remaining=json.load(open(out/'remaining-v16.json'));assert len(remaining)==233
rows=['# Pendências V16','','HEAD `'+before['head']+'`, working tree não commitado. Pin do linter `'+before['linter']['commit']+'`; captura final em final-state.json.gz. **233 emissões pendentes**; não são automaticamente 233 falhas funcionais.','','| Consumer e linha | Diagnóstico |','|---|---|']
for item in remaining:
 loc=item['locations'][0]['physicalLocation'];p=loc['artifactLocation']['uri'].removeprefix('./');line=loc['region']['startLine'];msg=' '.join(item['message']['text'].split()).replace('|','\\|');rows.append('| ['+p+':'+str(line)+']('+str(root/p)+':'+str(line)+') | '+msg+' |')
rows+=['','## Próxima investigação','','W3 fechou o fallback de table_root por nomes, conservando a classificação anterior. O caso Source(Table) nesse predicado ainda deve ser medido em sua linguagem e travessia antes de decidir; o controle raso desta etapa prova o transporte de contexto quando o modo já foi selecionado. Esta é hipótese de auditoria por fonte, não falha de linguagem reproduzida nem item encerrado.','', 'O alinhamento OutlineEntry aqui foi validado nos cenários diretos/sintetizados/mistos, bare/where, aliases, body/fields, state/custom counter e projeção CLI descritos no relatório. Isso não certifica todas as famílias/propriedades da linguagem ou os demais fallbacks.']
(out/'lista-pendencias.md').write_text('\n'.join(rows)+'\n')
report=f'''# V16 — walk, OutlineEntry e ordem causal

Etapa concluída; objetivo geral de correções permanece aberto.

## Resultado

**V16: 239 → 233.** Foram fechados os seis pontos W1–W6 de walk.rs: cinco produtivos e um no oráculo do teste de reserva. As alternativas nominalmente agrupadas conservam as consequências anteriores. V19 **900 → 907 notas**: seis grupos de consequência uniforme e um no novo helper privado de receiver. V20 fica em **871 notas**, sem aumento. Três mensagens V19 existentes mudam somente pelas decisões prescritas de OutlineEntry. Nenhuma regra foi silenciada ou rebaixada.

A auditoria corrigiu falhas funcionais do mesmo caminho:

- OutlineEntry já era locatável e fornecia NativeElement, mas extract_payload o descartava. A delegação nativa foi restaurada; o walk indexa a própria entry e conserva element/fill como dados semânticos, sem headings fantasmas.
- Source perdia forced_location e shallow_table ao chamar walk. Agora delega pelo mesmo walk_at e conserva os dois valores. O alias Located(Source(entry)) mantém um único registro na Location recebida; Source(Table) conserva o modo raso.
- body() e a identificação do receptor reconhecem Source/Located, preservando o body morfológico, campos e identidades. Strong/Realized não são novos unwraps por esses helpers; outros tipos continuam com o mesmo erro/fallthrough.
- O seletor bare outline.entry usa o índice por tipo. A presença no índice NativeElement não bastava; agora a entry direta também está no kind_index próprio.
- A síntese continua pós-walk, com os mesmos campos e IDs. Seus ordinais são registrados após a âncora real do outline, preservando a ordem relativa dos demais registros. Os dois helpers pub(crate) mantêm dados de ordem em CounterRegistry/StateRegistry, sem executar callbacks nem alterar históricos/ações. Isso corrige bare/where, múltiplos outlines e leituras do prefixo temporal.
- A última diferença de query --field era apresentação: o resultado direto existia, mas o fallback L2 não projetava level/element/fill e o filtro eliminava o campo ausente. O novo ramo projeta os dados existentes, sem fabricar snapshot L1; Some, inclusive vazio, segue autoritativo.

[Lista de correções e grupos](lista-correcoes.md) · [patch completo](stage.patch) · [lista de pendências](lista-pendencias.md).

## Evidência de linguagem e testes

As nove sondas bilaterais passaram no pin vanilla **9f2b6e8715237cb086899a42873660fe744622e8**: entry direta, sintetizada e mista usando body() ou acesso a campos; bare/where em ordem; state e contador customizado na Location da entry sintetizada. O valor observado antes era 2 onde o prefixo correto era 1; depois ambos retornam 1, mantendo o final 2 nos testes. Query --field level passa a devolver [1] no direto e [1,1] no misto. [parity-result.json](parity-result.json), [measurement-after.json](measurement-after.json) e fixtures conservam comandos, fontes e binários. Comparação de dados semânticos impressos/JSON, não identidade de bytes PDF ou certificação de render/paridade global. A projeção recursiva de outras famílias mantém o alcance legado.

Foram adicionados **{new_tests} testes**, e o teste antigo que documentava a pendência de OutlineEntry foi migrado explicitamente para exigir a delegação nativa. Os **16 controles Core e três Shell passaram**. A suíte completa teve **{w['passed']} aprovados, zero falhas e {w['ignored']} ignorados preexistentes**, {new_tests} testes a mais que o baseline completo anterior. [workspace/workspace-result.json](workspace/workspace-result.json).

O RED foi preservado por causa, antes de cada correção: inicialmente sete falhas/um controle; perda de contexto Source com duas falhas/sete controles; receivers/body com duas falhas/dez controles; índice/ordem/prefixo com quatro falhas/dez controles; serialização com duas falhas/um controle. Os dois helpers internos inexistentes produziram seis E0599 em três chamadas por helper antes da implementação. O scaffold cfg(test) do reconhecimento bruto serviu somente para o RED unitário do novo helper; o erro real de método foi medido na CLI e o scaffold foi removido. Logs de erro de tipo da fixture e de contagem incorreta do guard foram preservados e não contaram como RED funcional.

A sonda de nova variante Content foi feita em cópia do workspace real, com lib test compilada: os seis pontos selecionados passaram de **zero para seis E0004**, mais um E0004 no helper novo de receiver. Total **22 → 29**, sem outros erros e sem alterar as fontes produtivas. [probe-green-result.json](probe-green-result.json).

## Gates e integridade

Build workspace, fmt, Clippy all-targets com warnings e expectativas não cumpridas como erro, Rustdoc workspace -D warnings, linhagem V1/V5/V15/V26 estrita, reversa e diff-check passaram. [validation-result.json](validation-result.json). O diferencial SARIF conserva todos os demais resultados, mensagens, níveis, multiplicidades e posições mapeadas: [architecture-delta.json](architecture-delta.json). O gate global continua **exit 1 pelos 233 V16 restantes**.

Mudaram dez consumers e seus dez owners L0 no inventário produtivo. Os L0 precederam cada implementação/refinamento; o resselo foi restrito a esses pares. is_locatable recebeu somente documentação/metadados. API externa, campos públicos, traits, flags/defaults, fases, pin do CI e dependências permanecem. Foram adicionadas duas primitivas crate-private de ordenação e um helper privado de receiver. [code-guard.json](code-guard.json), refinement.json e recibos guardam o escopo. Não houve commit, push, merge ou release nesta etapa.

Os rastros externos [trace-runtime.log](trace-runtime.log) e recibos distinguem índice geral, runtime e L3 do índice por tipo e da apresentação de --field. As sondas não instrumentaram código produtivo. Os executáveis derivados de diagnóstico foram removidos após conferência de SHA; fontes, receitas, hashes das bibliotecas e saídas permanecem em generated-artifact-cleanup.json. A revisão automática de permissão expirou antes de criar a primeira sonda; a única retentativa permitida foi concluída, sem bloqueio de aprovação pendente.

## RAM e proveniência

Comandos pesados sequenciais em cgroups com teto **12 GiB**, swap **0** e **Cargo um job**. Pico registrado: **{peak} bytes ({peak/1024**3:.3f} GiB)**; sem OOM/OOM kill. Maior contador memory.events/max: **{max_events}**; os arquivos *-resources.json preservam pressão e limites inclusive das tentativas. Não se infere ganho de desempenho.

HEAD `{before['head']}`, branch Tekt, working tree não commitado. [before.json.gz](before.json.gz) e suplementos preservam UTC, fontes/hashes e lista exata git diff HEAD --stat; [final-state.json.gz](final-state.json.gz) preserva o estado final. Fontes congeladas durante a validação. Linter `{before['linter']['commit']}`, SHA-256 `{before['linter']['sha256']}`. Oráculo SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, pin acima e proveniência ratificada em CLAUDE.md. Os hashes da medição final coincidem com os binários após os gates.

## Continuidade

Restam **233 V16**. A hipótese Source(Table) em table_root está registrada para medição própria; o fechamento nominal desse match não certifica sua regra de linguagem. O objetivo geral não foi marcado completo.
'''
(out/'relatorio.md').write_text(report)
record={'utc':now(),'head':before['head'],'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'source_hashes':v['source_hashes'],'changed':changed,'scope':scope,'external_public_declarations_preserved':True,'new_tests':new_tests,'tests':w,'architecture':delta,'parity':parity,'probes':{'before':red,'after':green},'resources':resources,'events':events,'peak_bytes':peak,'peak_gib':peak/1024**3,'docs':docs,'linter':before['linter'],'measurement_binaries':measurement['binaries'],'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'stage_complete':True,'goal_scope_incomplete':True,'remaining_v16':233,'no_commit_push_release':True}
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(record,f)
manifest={str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest-sha256.json'};(out/'manifest-sha256.json').write_text(json.dumps(manifest,indent=2)+'\n');assert all(sha(out/p)==h for p,h in manifest.items())
print(json.dumps({'stage_complete':True,'V16':233,'V19':907,'V20':871,'passed':w['passed'],'failed':0,'ignored':w['ignored'],'peak_gib':peak/1024**3,'manifest_files':len(manifest),'goal_scope_incomplete':True}),flush=True)
