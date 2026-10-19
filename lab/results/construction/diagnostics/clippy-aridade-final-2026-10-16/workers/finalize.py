from pathlib import Path
import json,gzip,re,hashlib,subprocess,datetime,collections,difflib,csv,io
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');receipt=load('validation-result.json');suite=load('workspace/workspace-result.json');strict=load('strict-details.json');inventory=load('clippy-inventory.json');baseline=load('baseline-details.json')
assert receipt['exit']==0 and len(v['commands'])==11 and all(c['exit']==0 for c in v['commands'])
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert len(inventory)==17 and v['clippy_delta']=={'clippy::too_many_arguments':-5}
assert strict['exit']==0 and not strict['warnings'].get('clippy::too_many_arguments')
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged']
assert baseline['sources_unchanged'] and all(c['exit']==(101 if c['label'].startswith('red-') else 0) for c in baseline['commands'])
assert all(c['selected_all_observed'] for c in baseline['commands'] if c['label'].startswith('red-'))
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
mem=receipt['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0';peak=int(mem['memory.peak'])/1024**3
counts=collections.Counter(x['code'] for x in inventory)
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:sha(p) for p in before['files']},clippy=17,rules=dict(counts),suite=suite,memory_peak_gib=peak)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n');(out/'remaining-rules.json').write_text(json.dumps(dict(counts.most_common()),indent=2)+'\n')
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(t.splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p,t in before['files'].items()))
link=lambda n:f'[{n}]({out/n})'
classification=json.loads((root/'00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json').read_text());rows=[]
for c in classification:
 c=dict(c);c['resultado']='correção interna concluída' if c['id'] in ['C014','C021','C045','C062','C064','C067','C069','C071','C102'] else ('argumento morto removido; exceção residual nominal' if c['id'] in ['C047','C066'] else 'exceção nominal prescrita em L0')
 c['status_na_auditoria']=c['status'];c['status']='materializado-e-validado';c['owner_sha256_atual']=sha(c['owner']);c['source_sha256_atual']=sha(c['path'])
 rows.append(c)
assert len(rows)==57
expect_count=sum(len(re.findall(r'#\[expect\(\s*clippy::too_many_arguments\b',Path(p).read_text())) for p in v['source_hashes'] if p.endswith('.rs'));assert expect_count==48,expect_count
(out/'aridade-57-resultados.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n')
s=io.StringIO();w=csv.DictWriter(s,fieldnames=['id','path','line','function','owner','decision','resultado','owner_sha256_atual'],extrasaction='ignore');w.writeheader();w.writerows(rows);(out/'aridade-57-resultados.csv').write_text(s.getvalue())
lines=['# Lista atual de pendências\n\n',f'Inventário Clippy: **17 avisos**, medido {v["end"]} UTC, HEAD `{before["head"]}`, working tree não commitado. Fontes/hashes/diffstat em {link("validation-details.json")}. Aridade: zero avisos, com 48 expectativas nominais satisfeitas e 9 refatorações nos 57 casos originais; duas das expectativas exigiram retirar antes um argumento morto.\n\n','| Regra | Quantidade |\n|---|---:|\n']
order=['dead_code','clippy::should_implement_trait','clippy::inherent_to_string','clippy::large_enum_variant','clippy::vec_box'];assert set(order)==set(counts)
for rule in order:lines.append(f'| `{rule}` | {counts[rule]} |\n')
lines.append('\nOrdem sugerida: auditar dead_code/lifetimes; decidir semântica das traits/conversões; depois representação de enums e retenção. Remoção de campo retentor, Box por tamanho e troca de to_string/FromStr não são reparos automáticos. Mudanças públicas precisam de proposta concreta e gate ADR-0127.\n')
for rule in order:
 lines.append(f'\n## {rule}\n\n')
 for x in inventory:
  if x['code']!=rule:continue
  span=x['spans'][0];lines.append(f'- [{span["file_name"]}:{span["line_start"]}]({root/span["file_name"]}:{span["line_start"]}) — {x["message"]}\n')
(out/'lista-pendencias.md').write_text(''.join(lines))
report=f'''# Aridade — cinco correções finais concluídas

Inventário Clippy **22 → 17**; `too_many_arguments` **5 → 0**, sem novos avisos. Os cinco casos deste lote receberam correções privadas, com sete L0 atualizados antes do código e resselo restrito. Os 57 casos originais estão materializados: **9 refatorações e 48 exceções nominais**, estas prescritas nos L0; duas exceções condicionadas incluíram primeiro remover argumento morto. Nenhum limiar foi elevado, nem supressão nova adicionada neste lote. Expectativas existentes conferidas com `unfulfilled_lint_expectations` como erro. {link('aridade-57-resultados.csv')} · {link('stage.diff')}.

C014: GridLayoutInput emprestado nomeia os doze campos de Grid/Table, mantendo H/V, expressões de cada chamada, None/Some e o motor visual comum. O corpo do algoritmo, tracks, gutters, spans, restauração de estado e projeção semântica de Table permanecem. Sem nova cópia profunda ou novo placement.

C021: OutlineIndentPlan nomeia os sete valores já calculados. Prefix/inner continuam explícitos; o compositor é igual após o desempacotamento. Auto/fixo/callback, níveis, parent_key, RTL, presença de prefixo e markers não mudam. As duas fixtures foram adaptadas com os mesmos valores e assertions.

C067/C069/C071: SvgRenderSession empresta fonts, glyph_defs, paint_defs, clip_next_id, destinations, glyph_fonts e modo HTML. Dispatcher, grupos, links e células reusam os mesmos caches/IDs; grupos usam Size existente para dimensões locais. Mantidos match exaustivo, despacho estático, camadas, clips, destinos/hit-area, binding de fontes e a restauração de paint_root. A célula de tiling reempresta estado com destinos vazios, HTML false e decoração temporária restaurada; links decorativos desligam HTML só durante os filhos e restauram o valor para irmãos. Sem clones de caches, global, vtable ou fase nova.

{link('code-guard.json')} e {link('materialization.json')} registram a conferência das APIs externamente públicas, replay exato das edições e da formatação, corpos Grid/outline e helpers de clip intactos e mapeamento dos argumentos para campos. A busca em todo o workspace, incluindo fixtures, está em {link('callers.txt')}; o motor homônimo de matemática ficou intacto. Não foi necessária nova confirmação ADR-0127, pois este lote muda somente interfaces privadas já prescritas.

## Validação

Antes do código passaram **1215 testes de layout (3 ignorados), 5 de outline e 575 de exportação**; o gate RED com --no-deps identificou exatamente as duas aridades core e três SVG. {link('baseline-details.json')}.

Depois passaram **{suite['passed']} testes, {suite['failed']} falhas e {suite['ignored']} ignorados** na suíte workspace completa, incluindo os controles existentes. Sem mudar fixtures, esperados ou tolerâncias e sem criar testes que espelhem carriers. Build, inventário Clippy all-targets, gate estrito de aridade e expectativas, formato, Rustdoc com warnings como erro, quatro índices HTML, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

Tekt: **{sum(v['architecture_warnings'].values())} avisos**, {v['architecture_warnings']}, sem erros nem aumentos por regra contra o baseline. A dívida anterior não está resolvida por este lote. Testes e contagens não demonstram paridade global, ganhos de performance ou conformidade PDF/UA.

## RAM e proveniência

Todos os processos pesados desta etapa rodaram em serviços com **12 GiB, swap zero e CARGO_BUILD_JOBS=1**. Pico da validação **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. {link('validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado; snapshot {before['utc']}; validação {v['start']}–{v['end']} UTC. Fontes e L0 imutáveis durante os comandos. {link('before.json.gz')} e {link('final-state.json')} conservam fontes/hashes, status e git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150 e vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 com SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/; {link('manifest.sha256')}.

Restam **17 avisos Clippy** e a dívida Tekt: {link('lista-pendencias.md')}. A meta geral continua aberta. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=17,arity_warnings=0,expectations=expect_count,tests=suite['passed'],peak_gib=peak)))
