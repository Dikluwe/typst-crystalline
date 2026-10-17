from pathlib import Path
import json,gzip,hashlib,collections,datetime,subprocess,difflib,re,csv,io
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-excecoes-2026-10-16';load=lambda n:json.loads((out/n).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));validation=load('validation-details.json');receipt=load('validation-result.json');inventory=load('clippy-inventory.json');strict=load('strict-details.json')
assert receipt['exit']==0 and len(validation['commands'])==12 and all(c['exit']==0 for c in validation['commands'])
assert validation['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in validation['source_hashes'].items())
assert len(inventory)==28 and strict['exit']==0 and len(strict['remaining_functions'])==11
expected={'clippy::too_many_arguments':-46,'clippy::mutable_key_type':-1,'clippy::single_range_in_vec_init':-4};assert validation['clippy_delta']==expected
assert all(c['exit']==101 for c in load('red-details.json')['commands'])
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL);guard=load('code-guard.json');assert guard['attributes']==49 and len(guard['files'])==22
summaries=[]
for label in ['counter','coverage']:
 text=(out/('validation-'+label+'.log')).read_text();rows=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',text,re.M);assert rows,label
 for status,passed,failed,ignored in rows:
  assert status=='ok' and int(failed)==0;summaries.append(dict(label=label,passed=int(passed),failed=int(failed),ignored=int(ignored)))
summary_text='; '.join(f'{x["label"]}: {x["passed"]} aprovados, {x["failed"]} falhas e {x["ignored"]} ignorados' for x in summaries)
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
initial_receipt=load('first-validation/validation-result.json')
assert all(int(r['memory_after']['memory.max'])==12*1024**3 and int(r['memory_after']['memory.swap.max'])==0 for r in [initial_receipt,receipt])
assert all(dict(line.split() for line in r['memory_after']['memory.events'].splitlines())['oom']=='0' and dict(line.split() for line in r['memory_after']['memory.events'].splitlines())['oom_kill']=='0' for r in [initial_receipt,receipt])
mem=receipt['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0';peak=max(int(r['memory_after']['memory.peak']) for r in [initial_receipt,receipt])/1024**3
rules=collections.Counter(x['code'] for x in inventory);state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:sha(p) for p in before['files']},clippy=28,rules=dict(rules),targeted_tests=summaries,memory_peak_gib=peak,attributes=49,consumers=22,verification_scope=validation['verification_scope'])
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n');(out/'remaining-rules.json').write_text(json.dumps(dict(rules.most_common()),indent=2)+'\n');(out/'cases.json').write_text(json.dumps(before['cases'],ensure_ascii=False,indent=2)+'\n')
stream=io.StringIO();writer=csv.DictWriter(stream,fieldnames=['id','rule','source','line','function','reason']);writer.writeheader();writer.writerows(before['cases']);(out/'cases.csv').write_text(stream.getvalue())
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(v['text'].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p,v in before['files'].items()))
lines=['# Exceções nominais materializadas\n\n',f'Estado de origem: HEAD `{before["head"]}`, working tree não commitado; {before["utc"]} UTC. Fontes/L0, hashes, diffstat, motivos e spans em [before.json.gz]({out}/before.json.gz). São 51 avisos tratados por 49 atributos em 22 consumers.\n\n','| ID | Regra | Função | Linha anterior | Motivo |\n|---|---|---|---|---|\n']
for case in before['cases']:lines.append(f'| {case["id"]} | `{case["rule"]}` | `{case["function"]}` | `{case["source"]}:{case["line"]}` | {case["reason"].replace("|","/")} |\n')
(out/'lista.md').write_text(''.join(lines))
lines=['# Pendências após as exceções nominais\n\n',f'Inventário validado em {validation["end"]} UTC, HEAD `{before["head"]}`, working tree não commitado. [validation-details.json]({out}/validation-details.json) conserva fontes, hashes e diffstat. Total: **28**.\n\n','| Regra | Casos |\n|---|---:|\n']
order=['dead_code','clippy::too_many_arguments','clippy::should_implement_trait','clippy::inherent_to_string','clippy::large_enum_variant','clippy::vec_box'];assert set(order)==set(rules)
for rule in order:lines.append(f'| `{rule}` | {rules[rule]} |\n')
lines.append('\nAs 11 aridades restantes são exatamente C014/C021/C045/C047/C062/C064/C066/C067/C069/C071/C102. Os dois cleanups C047/C066 só podem receber a exceção residual após retirar o argumento morto prescrito; os demais precisam de reorganização interna. Não há anotação nova nestas funções. Contratos públicos novos continuam sujeitos à proposta concreta e ratificação ADR-0127.\n')
for rule in order:
 lines.append(f'\n## {rule}\n\n')
 for item in inventory:
  if item['code']!=rule:continue
  p=item['spans'][0];lines.append(f'- [{p["file_name"]}:{p["line_start"]}]({root/p["file_name"]}:{p["line_start"]}) — {item["message"]}\n')
(out/'lista-pendencias.md').write_text(''.join(lines))
link=lambda n:f'[{n}]({out/n})'
prior=json.loads((root/validation['prior_full_suite']).read_text());assert prior['exit']==0 and prior['failed']==0
report=f'''# Exceções nominais Tekt — etapa concluída

Materializadas **46 exceções diretas de too_many_arguments**, já classificadas e prescritas nos L0, e as exceções de **cinco avisos de testes** (quatro single_range_in_vec_init e um mutable_key_type). São **49 atributos #[expect] em 22 consumers**, pois cada uma das duas funções de cobertura reúne duas ocorrências de intervalo. Inventário cargo clippy: **79 → 28 avisos**, sem avisos novos. Os **11 casos de aridade destinados a correção continuam visíveis**, exatamente nas funções prescritas. Não se afirma que os 51 avisos representavam defeitos funcionais. {link('lista.md')} · {link('cases.csv')} · {link('stage.diff')}.

## Motivos e limites

As aridades preservadas pertencem a fachadas de compatibilidade, operações puras com operandos explícitos, fronteiras de pintura/shaping/travessia ou fixtures de teste. Cada atributo traz seu ID e motivo nominal presente no owner. Não houve supressão por módulo/impl/crate, aumento de limiar ou anotação dos casos a corrigir. As APIs públicas, defaults, fases e despacho estático foram preservados.

Os arrays e Vec de cobertura representam coleções de Range semiabertos (2..6 e 40..42), não os pontos dentro do intervalo. Alterá-los para satisfazer a heurística destruiria o tipo/observação de append e rollback. O teste de Counter usa somente Str e Selector::Kind, preservando HashMap, duas entradas e ambos os lookups. A exceção é exclusiva desse teste: não declara seguros todos os payloads Selector/Value nem valida Eq para NaN. Os dois L0 registram as condições de reabertura; expectativas que deixarem de ser necessárias devem ser removidas.

Todos os 22 L0 foram atualizados antes do código. {link('l0-before-code.json')} e {link('code-guard.json')} comprovam que remover exclusivamente os novos atributos e o hash de linhagem recupera a fonte anterior normalizada por rustfmt: **zero mudanças de assinatura, corpo, cfg ou testes**. Resselo restrito aos owners/consumers; sem API nova, commit, push ou release.

## Validação e escopo da prova

RED anterior: 15 erros de aridade produtiva do core (Cargo interrompe antes dos alvos de teste), um mutable_key_type e quatro single_range_in_vec_init. O inventário all-targets de origem preserva os 57 avisos de aridade. GREEN: Clippy all-targets com unfulfilled_lint_expectations e os lints de teste como erros passou; todas as 49 expectativas foram satisfeitas. O gate conferiu por nome/arquivo os 11 avisos residuais de aridade, sem ocultá-los. {link('red-details.json')} · {link('strict-details.json')}.

Testes atuais: {summary_text}. Build, compilação all-targets, fmt, Rustdoc com RUSTDOCFLAGS=-D warnings, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. Tekt mantém **{sum(validation['architecture_warnings'].values())} avisos anteriores**, {validation['architecture_warnings']}, sem erros ou avisos novos. Os quatro índices HTML foram conferidos. {link('validation-details.json')}.

A suíte funcional completa não foi repetida nesta etapa de metadados. O último resultado completo foi **{prior['passed']} aprovados, {prior['failed']} falhas e {prior['ignored']} ignorados**, em [workspace-result.json]({root}/00_nucleo/diagnosticos/clippy-locais-2026-10-16/workspace/workspace-result.json). A equivalência comprovada dos corpos/assinaturas/cfg/testes, a compilação all-targets e os testes atuais de Counter/cobertura delimitam a reutilização dessa evidência; não se apresenta o total anterior como uma nova execução completa. Não foram criados testes que espelhem atributos nem medidas novas de performance/paridade.

O primeiro comando estrito compilou com exit0, mas o harness falhou porque -A warnings ocultou os 11 resíduos de aridade que ele precisava listar. A opção foi retirada e a validação retomou em strict, reaproveitando guard/build/inventário aprovados sob hashes idênticos de fontes/L0. As duas execuções e seus recibos de memória foram preservados; não houve alteração de código para satisfazer o harness. {link('validation-harness-repair.json')} · {link('first-validation/validation-details.json')}.

## RAM, proveniência e pendências

Serviço limitado por cgroup a **12 GiB**, swap zero e um job Cargo. Maior pico entre as duas execuções de validação: **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. O teto cobre os processos dos serviços. {link('validation-result.json')} · {link('first-validation/validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação {validation['start']}–{validation['end']} UTC, sem alterações de fontes/L0 durante os comandos. {link('before.json.gz')} e {link('final-state.json')} guardam fontes, hashes, status e git diff HEAD --stat. Scripts duráveis em workers/. Linter CI no pin 49c6bb0e584104a0c89270431d8402f5c6c61150, com [proveniência]({root}/00_nucleo/diagnosticos/clippy-expressions-2026-10-16/linter-provenance.json) preservada. {link('manifest.sha256')}.

Restam **28 avisos no inventário Clippy** e a dívida Tekt. As 11 correções internas de aridade devem ser a próxima frente; não confundir o fim desta etapa de exceções com o fim da meta geral. {link('lista-pendencias.md')}.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=28,attributes=49,targeted_tests=summaries,memory_peak_gib=peak)))
