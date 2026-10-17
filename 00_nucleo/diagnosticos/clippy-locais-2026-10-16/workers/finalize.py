from pathlib import Path
import gzip,json,hashlib,subprocess,collections,difflib,datetime,csv,io
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-locais-2026-10-16';load=lambda n:json.loads((out/n).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));validation=load('validation-details.json');receipt=load('validation-result.json');suite=load('workspace/workspace-result.json');inventory=load('clippy-inventory.json')
assert receipt['exit']==0 and len(validation['commands'])==10 and all(c['exit']==0 for c in validation['commands'])
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged'] and validation['sources_unchanged_during_validation']
assert all(sha(p)==h for p,h in validation['source_hashes'].items())
assert len(inventory)==79
removed=collections.Counter(c['rule'] for c in before['cases']);assert validation['clippy_delta']=={k:-v for k,v in removed.items()}
assert load('red-details.json')['errors']=={'clippy::derivable_impls':3,'clippy::map_entry':1}
assert load('red-tests-details.json')['errors']=={'clippy::items_after_test_module':2,'clippy::unnecessary_literal_unwrap':1}
assert all(c['exit']==0 for c in load('green-result-details.json')['commands'])
assert not any(c['code'] in ['clippy::derivable_impls','clippy::map_entry','clippy::items_after_test_module','clippy::unnecessary_literal_unwrap','clippy::new_without_default','clippy::type_complexity','clippy::field_reassign_with_default'] for c in inventory)
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
mem=receipt['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0'
peak=int(mem['memory.peak'])/1024**3
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(v['text'].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p,v in before['files'].items()))
rules=collections.Counter(x['code'] for x in inventory)
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:sha(p) for p in before['files']},clippy=len(inventory),rules=dict(rules),tests=suite,memory_peak_gib=peak)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n');(out/'remaining-rules.json').write_text(json.dumps(dict(rules.most_common()),indent=2)+'\n')
(out/'cases.json').write_text(json.dumps(before['cases'],ensure_ascii=False,indent=2)+'\n')
stream=io.StringIO();writer=csv.DictWriter(stream,fieldnames=['rule','source','line','message']);writer.writeheader();writer.writerows(before['cases']);(out/'cases.csv').write_text(stream.getvalue())
link=lambda n:f'[{n}]({out/n})'
lines=['# Avisos corrigidos neste lote\n\n',f'HEAD `{before["head"]}`, working tree não commitado, inventário original e fontes em [before.json.gz]({out}/before.json.gz), UTC {before["utc"]}. São 12 ocorrências; os números das linhas são os do snapshot anterior.\n\n','| Regra | Fonte e linha anterior |\n|---|---|\n']
for case in before['cases']:lines.append(f'| `{case["rule"]}` | `{case["source"]}:{case["line"]}` |\n')
(out/'lista.md').write_text(''.join(lines))
lines=['# Pendências Clippy\n\n',f'Inventário validado em {validation["end"]} UTC, HEAD `{before["head"]}`, working tree não commitado. Fontes/hashes, status e diffstat em [validation-details.json]({out}/validation-details.json). Total: **79**. Ordem de triagem: casos locais → decisões Tekt → estrutura e contratos.\n\n','| Regra | Casos |\n|---|---:|\n']
order=['dead_code','clippy::single_range_in_vec_init','clippy::mutable_key_type','clippy::too_many_arguments','clippy::should_implement_trait','clippy::inherent_to_string','clippy::large_enum_variant','clippy::vec_box']
assert set(order)==set(rules)
for rule in order:lines.append(f'| `{rule}` | {rules[rule]} |\n')
lines.append('\nOs 57 too_many_arguments mantêm a auditoria anterior: 11 correções prescritas e 46 exceções justificadas, ainda não materializadas nesta etapa. Dead code remanescente exige distinguir retenção causal de armazenamento sem uso. Não trocar contratos públicos para satisfazer lint sem o gate ADR-0127.\n')
for rule in order:
 lines.append(f'\n## {rule}\n\n')
 for item in inventory:
  if item['code']!=rule:continue
  p=item['spans'][0];lines.append(f'- [{p["file_name"]}:{p["line_start"]}]({root/p["file_name"]}:{p["line_start"]}) — {item["message"]}\n')
(out/'lista-pendencias.md').write_text(''.join(lines))
report=f'''# Clippy local e observação real — etapa concluída

Corrigidos **12 avisos em 11 consumers**: sete do Clippy e cinco dead_code do compilador Rust, todos observados no inventário cargo clippy. Os 11 L0 foram atualizados antes do código, com resselo restrito. Clippy passou de **91 para 79 avisos**, exclusivamente pelo delta esperado: {dict(removed)}. Nenhuma supressão ou aviso novo; os gates type_complexity, field_reassign_with_default e new_without_default permanecem zerados. {link('lista.md')} · {link('cases.csv')} · {link('stage.diff')}.

## O que mudou e por quê

SvgOptions conserva Default false/false por derive; StreamMode e PdfTags conservam os contratos Default já publicados, com variantes Verbose e Enabled explicitamente marcadas. Não há trait público novo, alteração de default do produto ou assinatura. FaceCache usa Entry com closure lazy, preservando consultas somente em miss, cache negativo None e ownership dos bytes da face. Os módulos de teste HTML/SVG foram movidos intactos ao fim dos próprios owners.

O campo privado duplicado BundleHttpServer.live foi retirado, preservando o parâmetro público e o live capturado pelo worker/serve_connection. A constante privada I64 sem uso foi retirada do encoder de fixtures sem alterar bytes de testes WASM. Nas três suítes CLI, stdout foi mantido e passa a aparecer junto a stderr nos diagnósticos das assertions de sucesso; fixtures, status, esperados canônicos de stderr e demais verificações ficaram intactos. {link('contract-test-guard.json')} comprova equivalência fora das mudanças prescritas.

O smoke test p273_6_layouter_parent_bbox_consumed_by_block_arm verificava apenas Some(Rect) fabricado localmente. Mantivemos seu nome e substituímos o corpo por um controle real: Shape com gradiente Parent dentro de Content::block 200x100 → layout → bbox emitido de 200x100 → PDF com extensão horizontal do gradiente de 200pt, diferente do mesmo Shape fora do Block. Não foi inventado bbox manual como entrada nem alterado código de layout/export para acomodar um esperado. O novo controle passou, assim como os controles adjacentes. Não se alega nova paridade upstream ou conformance geral PDF.

## Verificação

RED: três derivable_impls, map_entry, dois items_after_test_module e unnecessary_literal_unwrap observados como erros antes do código. O harness inicialmente esperou os sete na mesma invocação, mas Cargo bloqueou testes após os quatro erros produtivos; o RED dos três avisos de teste foi medido separadamente sem repetir a medição produtiva. O recibo original foi preservado com a correção do harness documentada em {link('red-harness-review.json')}. GREEN: teste real de bbox e gate Clippy selecionado, incluindo unfulfilled_lint_expectations como erro, passaram. {link('red-details.json')} · {link('red-tests-details.json')} · {link('green-result-details.json')}.

Suíte workspace: **{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados**. Nomes dos testes preservados; o total histórico não é o critério de aceitação. Build, fmt, Rustdoc com RUSTDOCFLAGS=-D warnings, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. Tekt mantém **{sum(validation['architecture_warnings'].values())} avisos anteriores**, {validation['architecture_warnings']}, sem erros ou avisos novos. Fontes/L0 imutáveis durante os comandos. Quatro índices HTML conferidos após a suíte. {link('validation-details.json')} · {link('workspace/workspace-result.json')}.

A primeira validação confirmou os 79 avisos Clippy, mas bloqueou o fechamento por um V16 novo no helper bbox (`_ => None`). Após ler o enum vigente e atualizar o L0 antes do código, o observador passou a enumerar todos os carriers: contêineres recursam, Shape observa o bbox e as seis folhas sem bbox/filhos devolvem None. Variantes futuras exigirão decisão do compilador. O baseline Tekt não foi elevado. {link('exhaustiveness-repair.json')} · {link('first-validation/validation-details.json')}.

## Recursos, proveniência e continuidade

Teto por cgroup **12 GiB**, swap zero e CARGO_BUILD_JOBS=1. Pico da validação **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. O limite cobre os processos dos serviços. {link('validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação {validation['start']}–{validation['end']} UTC. {link('before.json.gz')} e {link('final-state.json')} preservam fontes, hashes, estado rastreado/não rastreado e git diff HEAD --stat. Workers duráveis em workers/. Linter CI pinado em 49c6bb0e584104a0c89270431d8402f5c6c61150, proveniência em [linter-provenance.json]({root}/00_nucleo/diagnosticos/clippy-expressions-2026-10-16/linter-provenance.json). O harness verifica o vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 do binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. {link('manifest.sha256')}.

A primeira chamada do RED não iniciou porque a revisão automática de aprovação expirou; a única repetição permitida iniciou o serviço limitado com sucesso. Não houve rejeição por risco declarado nem execução sem o limite.

Restam **79 avisos Clippy** e a dívida Tekt. A etapa está concluída; a meta geral de todas as correções continua aberta. {link('lista-pendencias.md')}. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=79,tests=suite['passed'],memory_peak_gib=peak)))
