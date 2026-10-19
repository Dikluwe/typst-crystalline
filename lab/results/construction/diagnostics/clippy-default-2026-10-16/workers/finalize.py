from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,difflib,collections,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-default-2026-10-16'
load=lambda name:json.loads((out/name).read_text())
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));validation=load('validation-details.json');receipt=load('validation-result.json');suite=load('workspace/workspace-result.json');inventory=load('clippy-inventory.json')
assert receipt['exit']==0 and len(validation['commands'])==9 and all(x['exit']==0 for x in validation['commands'])
assert validation['sources_unchanged_during_validation'] and suite['sources_unchanged'] and suite['exit']==0 and suite['failed']==0
assert all(sha(p)==h for p,h in validation['source_hashes'].items())
assert len(inventory)==91
assert not any(x['code'] in ['clippy::new_without_default','clippy::field_reassign_with_default','clippy::type_complexity'] for x in inventory)
assert {p for p,h in before['all_source_hashes'].items() if sha(p)!=h}==set(before['files'])
assert all(c['exit']==101 for c in load('red-details.json')['commands'])
assert all(c['exit']==0 for c in load('green-details.json')['commands'])
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert (root/'target/doc'/crate/'index.html').is_file()
mem=receipt['memory_after'];assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0
events=dict(line.split() for line in mem['memory.events'].splitlines());assert events['oom']=='0' and events['oom_kill']=='0'
peak=int(mem['memory.peak'])/1024**3
# New productive bodies only delegate to the existing constructors. Existing code retained.
api=[]
for item in json.loads((root/'00_nucleo/materialization/clippy-default-2026-10-16/proposal.json').read_text())['public_default_implementations']:
 p=item['source'];name=item['type'];old=before['files'][p]['text'];new=Path(p).read_text()
 impl=f'impl std::default::Default for {name} {{\n    fn default() -> Self {{\n        Self::new()\n    }}\n}}\n\n'
 assert new.count(impl)==1,p
 old_fns=re.findall(r'pub(?:\([^)]*\))? fn [^{;]+',old);new_fns=re.findall(r'pub(?:\([^)]*\))? fn [^{;]+',new)
 assert old_fns==new_fns,p
 normalize=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,flags=re.M)
 if name=='EvalContext':
  assert normalize(new.replace(impl,''))==normalize(old)
 else:
  marker='\n#[cfg(all(test, unix))]\nmod tests' if name=='FontCatalogSession' else '\n#[cfg(test)]\nmod tests'
  assert old.count(marker)==1 and new.count(marker)==1
  assert normalize(new.replace(impl,'').split(marker)[0])==normalize(old.split(marker)[0])
 api.append(dict(type=name,source=p,default_delegates_to_new=True,existing_public_signatures_unchanged=True,existing_productive_source_unchanged_beyond_impl_and_hash=True))
(out/'contract-guard.json').write_text(json.dumps(api,indent=2)+'\n')
diff=''.join(''.join(difflib.unified_diff(v['text'].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p,v in before['files'].items())
(out/'stage.diff').write_text(diff)
rules=collections.Counter(x['code'] for x in inventory);(out/'remaining-rules.json').write_text(json.dumps(dict(rules.most_common()),indent=2)+'\n')
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:sha(p) for p in before['files']},clippy=91,clippy_rules=dict(rules),tests=suite,validation=validation['commands'],memory_peak_gib=peak,approved_manifest=before['approved_manifest'])
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n')
order=['clippy::derivable_impls','clippy::map_entry','clippy::items_after_test_module','clippy::unnecessary_literal_unwrap','dead_code','clippy::single_range_in_vec_init','clippy::mutable_key_type','clippy::too_many_arguments','clippy::should_implement_trait','clippy::inherent_to_string','clippy::large_enum_variant','clippy::vec_box']
assert set(order)==set(rules)
lines=['# Pendências Clippy após Default\n\n',f'Inventário medido na validação {validation["end"]} UTC, HEAD `{before["head"]}`, working tree não commitado. Fontes, hashes, diffstat e comandos: [validation-details.json]({out}/validation-details.json). Total: **91**. Ordem de triagem mantém Clippy local → decisões Tekt → estrutura/contratos; não autoriza alterações de API.\n\n','| Regra | Casos |\n|---|---:|\n']
for rule in order:lines.append(f'| `{rule}` | {rules[rule]} |\n')
lines.append('\nOs 57 too_many_arguments conservam a classificação anterior (11 correções propostas e 46 exceções justificadas); nenhuma API dessa frente foi alterada aqui. Os demais grupos exigem auditoria de seus owners antes de decidir correção ou exceção. Não transformar Range em seus elementos, nem remover retenção causal apenas porque um campo não tem leitura explícita.\n')
for rule in order:
 lines.append(f'\n## {rule}\n\n')
 for item in inventory:
  if item['code']!=rule:continue
  span=item['spans'][0];f=span['file_name'];line=span['line_start'];lines.append(f'- [{f}:{line}]({root/f}:{line}) — {item["message"]}\n')
(out/'lista-pendencias.md').write_text(''.join(lines))
link=lambda name:f'[{name}]({out/name})'
report=f'''# Default público — etapa concluída

Implementados os três acréscimos públicos aprovados: `Default` para `EvalContext`, `FontCatalogSession` e `SourceSession`. Todos delegam exclusivamente a `Self::new()`. Os construtores existentes, campos, valores, configuração do produto, aquisição lazy, limites e fases permanecem iguais. Não há nova supressão de lint.

O dono aprovou o manifesto SHA-256 `{before['approved_manifest']}`; recibo em [approval-receipt.json]({root}/00_nucleo/materialization/clippy-default-2026-10-16/approval-receipt.json). Os quatro L0 ratificados precederam testes e implementação; linhagem e resselo restritos aos quatro owners/consumers. {link('l0-before-code.json')} · {link('contract-guard.json')} · {link('stage.diff')}.

## D1–D5 e resultado

Quatro controles novos exercitam: uso pelo trait Default; configurações, orçamento e isolamento de EvalContext; sessão de fontes fria, aquisição sob demanda e reuso sem contaminar outra sessão; Sources em Worlds novos com validação de texto, preservação de fontes antigas, isolamento e limites de retenção (8 entradas, 1 MiB total, 512 KiB por entrada). Os testes existentes continuam intactos. São controles da construção/retensão Rust, sem alegação nova de paridade upstream.

RED: os testes falharam por ausência dos três impls (`E0277`), antes da implementação. GREEN: os quatro controles passaram, além de Clippy com new_without_default, field_reassign_with_default, type_complexity e unfulfilled_lint_expectations como erros. {link('red-details.json')} · {link('green-details.json')}.

O inventário Clippy passou de **94 para 91 avisos**, exclusivamente pela eliminação dos três new_without_default; nenhum aviso novo. new_without_default, field_reassign_with_default e type_complexity ficaram zerados. Suíte workspace: **{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados**. Build, formato, Rustdoc com `-D warnings`, git diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. Os quatro índices HTML foram conferidos. Tekt: **{sum(validation['architecture_warnings'].values())} avisos preexistentes**, {validation['architecture_warnings']}, sem erros ou avisos novos. {link('validation-details.json')} · {link('workspace/workspace-result.json')}.

## RAM e proveniência

Execução limitada por cgroup a **12 GiB, swap zero e um job Cargo**. Pico da validação **{peak:.3f} GiB** (inclui cache), eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. O teto cobre os processos dos serviços. {link('validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação {validation['start']}–{validation['end']} UTC. Fontes/L0 imutáveis durante a validação. {link('before.json.gz')} e {link('final-state.json')} guardam hashes, lista exata de alterações (`git diff HEAD --stat`), estado não rastreado e horários. Workers duráveis em workers/. CI linter no pin `49c6bb0e584104a0c89270431d8402f5c6c61150`, proveniência na etapa [expressões]({root}/00_nucleo/diagnosticos/clippy-expressions-2026-10-16/linter-provenance.json). A suíte verifica o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. {link('manifest.sha256')}.

A etapa está concluída. Restam **91 avisos Clippy** e a dívida Tekt; os 57 too_many_arguments permanecem na auditoria anterior. A meta geral continua aberta. {link('lista-pendencias.md')} · {link('remaining-rules.json')}. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
manifest=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name!='manifest.sha256':manifest.append(f'{sha(p)}  {p.relative_to(out)}\n')
(out/'manifest.sha256').write_text(''.join(manifest))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=91,tests=suite['passed'],peak_gib=peak)))
