from pathlib import Path
import json,gzip,re,hashlib,subprocess,datetime,collections,difflib
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-retencao-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');receipt=load('validation-result.json');suite=load('workspace/workspace-result.json');strict=load('strict-details.json');inventory=load('clippy-inventory.json');baseline=load('baseline-details.json')
assert receipt['exit']==0 and len(v['commands'])==11 and all(c['exit']==0 for c in v['commands'])
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert len(inventory)==13 and v['clippy_delta']=={'dead_code':-3,'clippy::vec_box':-1}
assert strict['exit']==0 and not any(strict['warnings'].get(k) for k in ['dead_code','clippy::vec_box','clippy::too_many_arguments'])
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged']
assert baseline['sources_unchanged'] and all(c['exit']==(101 if c['label']=='red' else 0) for c in baseline['commands'])
assert baseline['commands'][-1]['errors']=={'dead_code':3,'clippy::vec_box':1}
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
mem=receipt['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0';peak=int(mem['memory.peak'])/1024**3
counts=collections.Counter(x['code'] for x in inventory)
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:sha(p) for p in before['files']},clippy=13,rules=dict(counts),suite=suite,memory_peak_gib=peak)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n');(out/'remaining-rules.json').write_text(json.dumps(dict(counts.most_common()),indent=2)+'\n')
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(t.splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p,t in before['files'].items()))
link=lambda n:f'[{n}]({out/n})'
lines=['# Pendências após retenção e código sem uso\n\n',f'Inventário: **13 avisos**, medido {v["end"]} UTC, HEAD `{before["head"]}`, working tree não commitado; fontes/hashes/diffstat em {link("validation-details.json")}. dead_code e vec_box zerados; aridade e expectativas continuam aprovadas.\n\n','| Regra | Quantidade |\n|---|---:|\n']
order=['clippy::should_implement_trait','clippy::inherent_to_string','clippy::large_enum_variant'];assert set(order)==set(counts)
for rule in order:lines.append(f'| `{rule}` | {counts[rule]} |\n')
lines.append('\nPróxima decisão: auditar semântica dos cinco métodos sugeridos como traits e dos três formatadores; medir tamanho/uso dos enums antes de decidir Box. Contratos externos, defaults e fase continuam sujeitos à ADR-0127. Não converter automaticamente Option em Result, Location em Option<Location> ou mudar campos públicos para Box só para satisfazer Clippy.\n')
for rule in order:
 lines.append(f'\n## {rule}\n\n')
 for x in inventory:
  if x['code']!=rule:continue
  span=x['spans'][0];path=span['file_name'];owner=re.search(r'^//! @prompt (.+)$',Path(path).read_text(),re.M)[1];lines.append(f'- [{path}:{span["line_start"]}]({root/path}:{span["line_start"]}) — {x["message"]}. L0: [{owner}]({root/owner}).\n')
(out/'lista-pendencias.md').write_text(''.join(lines))
report=f'''# Retenção e código sem uso — etapa concluída

Clippy **17 → 13 avisos**, sem novos avisos: `dead_code` **3 → 0** e `vec_box` **1 → 0**. Dois consumers e seus L0 foram atualizados com resselo restrito. Nenhum allow/expect novo ou mudança de limiar. {link('stage.diff')} · {link('lista-pendencias.md')}.

Producer.parent foi removido somente do armazenamento. O argumento parent de Session.add continua herdando o target do pai, sem mudar descoberta, filhos, geração ou roteamento HTML/bundle.

Execution.final_styles tornou-se _retained_styles, com comentário sobre sua função. O mesmo styles continua retido até o drop do registro. O campo não é lido por replay, mas guarda valores/closures: apagá-lo para zerar dead_code anteciparia a liberação. A nova nomenclatura explicita essa finalidade; não cria getter ou leitura artificial.

retired usa Vec<Execution>. As três transições movem o registro inteiro para o vetor e liberam sua caixa externa, conservando ordem, ctx/observações, resultado, sink, Location e Arc de entrada. current permanece Option<Box<Execution>>. Não foram descartados históricos, refeitas closures ou deduplicadas gerações. A auditoria não encontrou dependência do endereço de Execution; identidades dos dados referidos permanecem nos Arcs. Não se afirma ganho de RSS/tempo ou redução do número total de alocações sem benchmark próprio.

O wrapper subset_font foi retirado: só era chamado por dois testes, dentro de mod subset privado e sem reexportação. Os testes agora usam subset_font_with_mapping e .data, conservando GIDs, .notdef, nomes e assertions. Todos os adapters produtivos, variações, mapping, mensagens e fallbacks ficaram preservados. O L0 anterior exigia manter um wrapper de compatibilidade sem consumer produtivo; a busca refutou essa prescrição, que foi corrigida antes do código. Embora escrito pub, o símbolo não era externamente alcançável. É alteração interna no fluxo contínuo da ADR-0127 §2, item 4; não se criou nem retirou API pública resolvida. {link('code-guard.json')} e {link('materialization.json')} registram as edições exatas e a preservação dos contratos produtivos.

## Verificação

Antes da implementação passaram **2 testes de retenção/snapshots e 9 de subset**, incluindo o novo controle. O gate RED identificou exatamente três dead_code e um vec_box. {link('baseline-details.json')}.

O novo teste usa Session.execute real e Arc/Weak para provar que um valor de estilo sobrevive na execução atual, na aposentada e após crescimento real do Vec, e é liberado ao encerrar a sessão. Confere também a mesma identidade do input. É testemunha mecânica de lifetime; não substitui a semântica de linguagem testada pelos controles existentes.

Após a mudança, **{suite['passed']} testes passaram, {suite['failed']} falhas e {suite['ignored']} ignorados** na suíte workspace completa. Build, Clippy all-targets e gate estrito de dead_code/vec_box/aridade/expectativas, formato, Rustdoc com warnings como erro, quatro índices HTML, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

Tekt mantém **{sum(v['architecture_warnings'].values())} avisos**, {v['architecture_warnings']}, sem erros nem aumentos por regra. A dívida anterior continua aberta. Contagens e testes não certificam paridade global ou conformidade PDF/UA.

A primeira abertura do serviço de validação não executou: a revisão automática de permissão expirou no prazo. A única repetição permitida foi liberada e executou estes comandos; nenhum teste ou gate foi contornado. {link('approval-retry.json')}.

## RAM e proveniência

Serviços pesados com **12 GiB, swap zero e CARGO_BUILD_JOBS=1**. Pico da validação **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. {link('validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado; snapshot {before['utc']}; validação {v['start']}–{v['end']} UTC, fontes/L0 imutáveis durante os comandos. {link('before.json.gz')} e {link('final-state.json')} guardam fontes/hashes, status e git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150; vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 com SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/; {link('manifest.sha256')}.

Restam **13 avisos Clippy** (5 sugestões de traits, 3 to_string, 5 enums grandes) e a dívida Tekt. A meta geral continua aberta. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=13,tests=suite['passed'],peak_gib=peak)))
