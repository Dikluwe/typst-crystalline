from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,collections,re,difflib,csv,io
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16';load=lambda n:json.loads((out/n).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));validation=load('validation-details.json');receipt=load('validation-result.json');inventory=load('clippy-inventory.json');suite=load('workspace/workspace-result.json');strict=load('strict-details.json')
assert receipt['exit']==0 and len(validation['commands'])==11 and all(c['exit']==0 for c in validation['commands'])
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged'] and validation['sources_unchanged_during_validation']
assert all(sha(p)==h for p,h in validation['source_hashes'].items()) and len(inventory)==22 and validation['clippy_delta']=={'clippy::too_many_arguments':-6}
assert strict['exit']==0 and len(strict['remaining_functions'])==5
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
baseline=load('baseline-details.json');assert baseline['original_rust_sources_unchanged'] and baseline['commands'][-1]['selected_all_observed'];assert baseline['commands'][-1]['exit']==101
base_summaries=[]
for label in ['export','pdf']:
 s=(out/('baseline-'+label+'.log')).read_text();rows=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',s,re.M);assert rows
 for status,passed,failed,ignored in rows:
  assert status=='ok' and int(failed)==0;base_summaries.append(dict(label=label,passed=int(passed),failed=int(failed),ignored=int(ignored)))
base_summary_text='; '.join(f'{x["label"]}: {x["passed"]} aprovados, {x["failed"]} falhas e {x["ignored"]} ignorados' for x in base_summaries)
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
mem=receipt['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0';peak=int(mem['memory.peak'])/1024**3
rules=collections.Counter(c['code'] for c in inventory)
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:sha(p) for p in before['files']},clippy=22,rules=dict(rules),suite=suite,baseline_controls=base_summaries,memory_peak_gib=peak)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n');(out/'remaining-rules.json').write_text(json.dumps(dict(rules.most_common()),indent=2)+'\n')
(out/'cases.json').write_text(json.dumps(before['cases'],ensure_ascii=False,indent=2)+'\n');stream=io.StringIO();writer=csv.DictWriter(stream,fieldnames=['id','rule','source','line','function','prescription'],extrasaction='ignore');writer.writeheader();writer.writerows(before['cases']);(out/'cases.csv').write_text(stream.getvalue())
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(v['text'].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p,v in before['files'].items()))
lines=['# Seis aridades internas tratadas\n\n',f'HEAD `{before["head"]}`, working tree não commitado, snapshot {before["utc"]}; fontes/L0/hashes e diffstat em [before.json.gz]({out}/before.json.gz). Linhas correspondem ao estado anterior.\n\n','| ID | Função | Fonte e linha anterior | Intervenção |\n|---|---|---|---|\n']
for c in before['cases']:lines.append(f'| {c["id"]} | `{c["function"]}` | `{c["source"]}:{c["line"]}` | {c["prescription"].replace("|","/")} |\n')
(out/'lista.md').write_text(''.join(lines))
lines=['# Pendências após aridade interna\n\n',f'Inventário atual validado em {validation["end"]} UTC, HEAD `{before["head"]}`, working tree não commitado. [validation-details.json]({out}/validation-details.json) conserva fontes/hashes e diffstat. Total: **22**.\n\n','| Regra | Casos |\n|---|---:|\n']
order=['dead_code','clippy::too_many_arguments','clippy::should_implement_trait','clippy::inherent_to_string','clippy::large_enum_variant','clippy::vec_box'];assert set(order)==set(rules)
for rule in order:lines.append(f'| `{rule}` | {rules[rule]} |\n')
lines.append('\nAs cinco aridades ainda a corrigir são C014 (Grid/Table), C021 (indentação de outline) e C067/C069/C071 (sessão SVG). Continuam visíveis e não foram silenciadas. Contratos públicos novos exigem proposta concreta e ratificação ADR-0127; os defaults, flags e fases permanecem protegidos.\n')
for rule in order:
 lines.append(f'\n## {rule}\n\n')
 for item in inventory:
  if item['code']!=rule:continue
  s=item['spans'][0];lines.append(f'- [{s["file_name"]}:{s["line_start"]}]({root/s["file_name"]}:{s["line_start"]}) — {item["message"]}\n')
(out/'lista-pendencias.md').write_text(''.join(lines));link=lambda n:f'[{n}]({out/n})'
report=f'''# Aridade interna — seis casos concluídos

Tratados **seis avisos em oito consumers**, com os oito L0 atualizados antes das respectivas alterações de código e resselo restrito. Inventário cargo clippy **28 → 22 avisos**; too_many_arguments **11 → 5**, sem avisos novos. Quatro ocorrências desapareceram com agrupamentos privados; duas receberam a exceção residual prescrita somente após retirar o argumento morto. {link('lista.md')} · {link('cases.csv')} · {link('stage.diff')}.

## Mudanças e contratos

C047/C066: o argumento _text sem leitura foi retirado dos leaves raster/SVG de contornos e das chamadas. O raster deixa de extrair esse binding; o SVG continua enviando texto autoral ao writer HTML. O campo text de FrameItem::TextShaped e as APIs públicas não mudam. Os oito inputs reais restantes têm expect nominal com motivo no L0, sem elevar limiar ou suprimir módulos.

C102: PdfCompileRequest privado nomeia as nove opções de execução/exportação e é desempacotado uma vez. World, Source e Timings continuam explícitos; as três fachadas diretas preservam oracle false/false/true e todos os None/Some até resolução documental. O restante do núcleo, inclusive erros, warnings, precedência, dispatch e timings, ficou igual.

C045: um Collector local possui mapa, duas saídas e os dois contadores sincronizados. A recursão continua recebendo separadamente itens e referencial geométrico por ramo; next_id começa em first_id e counter em 1. Ordem das camadas/células/fill/stroke, dedup, atribuição de IDs e cálculos de bbox/conic não mudaram. O segundo walk de recursos e os testes ficaram intactos.

C062/C064: with_font_scenario interno recebe o enum FontScenario já existente junto aos recursos emprestados e ao modo. type1 conserva sua assinatura; os builders CIDFont/Multifont montam os mesmos campos nomeados, bitmap condicionado e LookupCache inicial por contexto. Mantidos tags Disabled, ua1/oracle false e os setters posteriores, sem clones de tabelas, novo fallback ou alegação de validação de cardinalidade.

{link('code-guard.json')} confere assinaturas externamente públicas e nomes de testes preservados; corpos de pintura intactos; núcleo PDF igual após remover o desempacotamento; corpo do walk equivalente pela projeção dos campos; enum FontScenario e defaults de PageContext preservados. Somente interfaces internas e a organização dos inputs foram alteradas. Sem API externa, default, dependência, fase, unsafe ou estado global novos.

## Verificação

Antes do código, controles existentes passaram: {base_summary_text}. A primeira medição RED parou nas duas aridades do core dependente; apenas o RED foi repetido com --no-deps, opção confirmada no help local do Clippy pinado, alcançando as seis funções deste lote. Não foram repetidos os controles já aprovados nem editado Rust para contornar o gate. {link('baseline-details.json')} · {link('baseline-harness-review.json')}.

Depois, **{suite['passed']} testes passaram, {suite['failed']} falhas, {suite['ignored']} ignorados** na suíte workspace completa. Os dados das fixtures e os esperados foram preservados; somente seus adaptadores de construção foram migrados para a interface interna. Não foram criados testes que espelhem os carriers. Build, Clippy all-targets, expectativas como erro, formato, Rustdoc com RUSTDOCFLAGS=-D warnings, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. Os cinco resíduos de aridade foram conferidos por função/arquivo, sem ocultá-los. Quatro índices HTML conferidos após a suíte. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

Tekt mantém **{sum(validation['architecture_warnings'].values())} avisos anteriores**, {validation['architecture_warnings']}, sem erros ou avisos novos. O teste continua medindo comportamento/observáveis já prescritos, sem alegar nova paridade global, ganho de performance ou conformidade de PDF a partir de contagens.

A primeira compilação all-targets detectou cinco fixtures do agregador ainda chamando os construtores retirados. A busca inicial foi incompleta. O owner foi incluído após conferir seus hashes contra o snapshot global original, e seu L0 foi atualizado antes de migrar cada chamada, mantendo todos os argumentos/esperados. A falha original, o snapshot inicial e a extensão de escopo foram preservados; os wrappers antigos não foram restaurados para esconder a migração. {link('test-caller-migration.json')} · {link('initial-before.json.gz')} · {link('first-validation/validation-details.json')}.

## RAM e proveniência

Serviços com **12 GiB**, swap zero e CARGO_BUILD_JOBS=1. Pico da validação **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. O teto cobre os processos dos serviços. {link('validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação {validation['start']}–{validation['end']} UTC, fontes/L0 imutáveis durante os comandos. {link('before.json.gz')} e {link('final-state.json')} guardam fontes, hashes, estado rastreado/não rastreado e git diff HEAD --stat. Scripts em workers/. Linter CI no pin 49c6bb0e584104a0c89270431d8402f5c6c61150; [proveniência]({root}/00_nucleo/diagnosticos/clippy-expressions-2026-10-16/linter-provenance.json). A suíte confirma o vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 e o SHA-256 do binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. {link('manifest.sha256')}.

Restam **22 avisos no inventário Clippy**, incluindo as cinco aridades acima, e a dívida Tekt. A meta geral continua aberta. {link('lista-pendencias.md')}. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report);(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=22,tests=suite['passed'],memory_peak_gib=peak)))
