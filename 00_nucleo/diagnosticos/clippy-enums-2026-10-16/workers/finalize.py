from pathlib import Path
import json,gzip,re,hashlib,subprocess,datetime,collections,difflib
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-enums-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');receipt=load('validation-result.json');suite=load('workspace/workspace-result.json');strict=load('strict-details.json');inventory=load('clippy-inventory.json');scope=load('scope.json');approval=load('approval-receipt.json')
assert receipt['exit']==0 and len(v['commands'])==13 and all(c['exit']==0 for c in v['commands'])
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert len(inventory)==2 and v['clippy_delta']=={'clippy::large_enum_variant':-3,'clippy::inherent_to_string':-3,'clippy::should_implement_trait':-1}
assert strict['exit']==0 and strict['warnings']=={'clippy::large_enum_variant':2}
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged']
assert load('traits-red-result.json')['exit']==101 and load('traits-green-result.json')['exit']==0
red=(out/'traits-red.log').read_text();assert red.count('error[E0277]')==5
for name in ['Decimal','Duration','Version']:assert name in red
assert 'FromIterator' in red
assert 'request=144 record=384 budget=160' in (out/'validation-size.log').read_text()
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
pkg=root/approval['package'];assert sha(pkg/'manifest.sha256')==approval['manifest_sha256']
for line in (pkg/'manifest.sha256').read_text().splitlines():
 h,p=line.split('  ',1);assert sha(pkg/p)==h
mem=receipt['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0';peak=int(mem['memory.peak'])/1024**3
counts=collections.Counter(x['code'] for x in inventory)
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:sha(p) for p in before['files']},clippy=2,rules=dict(counts),suite=suite,memory_peak_gib=peak,approval=approval)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n');(out/'remaining-rules.json').write_text(json.dumps(dict(counts),indent=2)+'\n')
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(before['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p in scope['sources']+scope['owners']))
link=lambda n:f'[{n}]({out/n})'
lines=['# Pendências após enums internos e traits aprovados\n\n',f'Inventário: **2 avisos Clippy**, medido {v["end"]} UTC, HEAD `{before["head"]}`, working tree não commitado. {link("validation-details.json")} conserva estado/hashes/diffstat.\n\n','| Ordem | Caso | Decisão pendente |\n|---|---|---|\n','| 1 | Tag | Auditar armazenamento/volume e clones; Box do payload público exige proposta e ratificação |\n','| 2 | Transformation | Auditar distribuição de Style e clones; Box pode piorar o caso Style; API exige proposta e ratificação |\n','| 3 | Tekt | 380 avisos de estrutura/contratos, com classificação por owner antes de qualquer exceção |\n']
for item in inventory:
 span=item['spans'][0];p=span['file_name'];owner=re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M)[1];lines.append(f'\n- [{p}:{span["line_start"]}]({root/p}:{span["line_start"]}) — `{item["code"]}`; L0 [{owner}]({root/owner}).\n')
(out/'lista-pendencias.md').write_text(''.join(lines))
report=f'''# Enums e traits aprovados — etapa concluída

Clippy **9 → 2 avisos**, sem novos avisos. O lote reúne um enum interno corrigido, duas exceções nominais justificadas e os quatro contratos Rust aprovados pelo humano. Nove consumers e seus nove L0 foram atualizados e resselados de forma restrita. {link('stage.diff')} · {link('code-guard.json')}.

## Contratos aprovados

Display implementado para Decimal, Duration e Version; os três métodos inerentes to_string foram retirados e as chamadas passam ao ToString padrão. Decimal delega a InnerDecimal; Duration e Version mantêm os corpos anteriores em helpers privados. Preservados escala/zeros, componentes vazios/trailing zeros, sinais, frações e a correção anterior do extremo i128::MIN. Isto não fecha a política de overflow aritmético de Duration. FromIterator<Style> implementado para Styles com o mesmo fold ordenado e último valor por campo; Styles::from_iter conserva a assinatura como adapter. A primeira validação refutou a previsão do prompt: Clippy 1.92 continua sinalizando o método inerente apesar do trait e dos testes collect aprovados. O L0 foi corrigido antes de prescrever a expectativa nominal S005 exclusivamente no adapter. Esta correção interna de metadados não modifica o contrato ratificado. O repr decimal passa diretamente ao Display para evitar a conversão intermediária sinalizada. Primeira validação preservada em first-validation/. Sem novo backing, reordenação ou clones no fold.

Ratificação **{approval['manifest_sha256']}**, proposta imutável em [review.md]({pkg}/review.md); {link('approval-receipt.json')}. Os quatro L0 propostos foram copiados e ratificados antes dos testes/código. Parsers Option e suas expectativas existentes, campos, defaults, repr Typst, PARITY_VERSION e demais APIs foram preservados. A migração dos métodos inerentes para ToString é a mudança de contrato expressamente aprovada.

## Enums: decisão após medição

O payload inteiro de OutlineEntryIndented passou para uma caixa no enum interno ContextReadRequest; fábrica pub(crate) mantém os campos privados e o replay empresta os mesmos quatro valores, com corpo preservado. Medição no ABI local: ContextReadRequest **432 → 144 bytes** e ContextRead **672 → 384 bytes**. O teste exige orçamento relativo a Value, sem hardcode do ABI. Cada registro Outline passa a alocar também o payload de 432 bytes: modelo 384+432=816, contra 672 antes, sem contar overhead do allocator. Portanto o ganho está no tamanho comum do registro, não é prova de menor RSS em todo workload. Os registros são Vec<Arc<ContextRead>>, não Vec de registros inline.

LayoutPassOutcome e MathLayoutPassOutcome permanecem por valor e recebem apenas expect(large_enum_variant) com razões E002/E003 nos respectivos L0. São retornos únicos consumidos por match; Box adicionaria alocação ao Complete e manteria o documento separado. A sonda sintética de transporte com documento vazio observou zero alocações inline contra uma por Complete boxed (20.000 por amostra). É evidência do custo da alternativa nesta fronteira, sem benchmark de layout real ou alegação de performance global. {link('measurements.json')} · {link('probe-details.json')} · {link('callers.txt')}.

Tag e Transformation continuam visíveis: boxear Tag reduz o slot mas adiciona alocação; boxear Style em Transformation pode aumentar o total desse caso. As duas APIs públicas não foram alteradas nem silenciadas. Não aplicar automaticamente a mesma decisão dos outcomes.

## Validação

RED real do orçamento interno: 432 bytes excediam o orçamento de 160. Controles anteriores passaram: 54 testes outline, 98 contexto, 31 callbacks e 8 de p1405_outline_indented. O RED dos traits produziu cinco diagnósticos E0277 por ausência dos quatro contratos; quatro testes novos passaram após a implementação. {link('baseline-details.json')} · {link('traits-red-result.json')} · {link('traits-green-result.json')}.

Suíte workspace completa: **{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados**. Build, Clippy all-targets, expectativas e lints já fechados como erro (incluindo inherent_to_string e should_implement_trait), orçamento interno, traits, formato, Rustdoc com warnings como erro e quatro índices HTML, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

Tekt: **{sum(v['architecture_warnings'].values())} avisos**, {v['architecture_warnings']}, sem erros nem aumento por regra. A meta geral de zero violations permanece aberta.

## RAM e proveniência

Serviços pesados com **12 GiB, swap zero e CARGO_BUILD_JOBS=1**. Pico da validação **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. {link('validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado; snapshot original {before['utc']}, preservado em {link('initial-before.json.gz')}. Snapshot expandido com os quatro pares aprovados e o par adicional de repr decimal, todos provados inalterados antes da inclusão pelos hashes globais; os quatro primeiros também pela base aprovada. O registro do acréscimo de repr está em traits-followup-materialization.json: {link('before.json.gz')}. Validação {v['start']}–{v['end']} UTC, fontes/L0 imutáveis durante os comandos. {link('final-state.json')} guarda hashes, status e git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150; vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/; {link('manifest.sha256')}.

Restam **2 avisos Clippy** e a dívida Tekt. {link('lista-pendencias.md')}. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=2,tests=suite['passed'],peak_gib=peak)))
