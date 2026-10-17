from pathlib import Path
import json,gzip,re,hashlib,subprocess,datetime,collections,difflib,csv,io
root=Path.cwd();out=root/'00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');receipt=load('validation-result.json');suite=load('workspace/workspace-result.json');strict=load('strict-details.json');inventory=load('clippy-inventory.json');baseline=load('baseline-details.json');scope=load('scope.json');proposal=load('public-proposal.json')
assert receipt['exit']==0 and len(v['commands'])==12 and all(c['exit']==0 for c in v['commands'])
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert len(inventory)==9 and v['clippy_delta']=={'clippy::should_implement_trait':-4}
assert strict['exit']==0 and strict['warnings']['clippy::should_implement_trait']==1
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged']
assert baseline['sources_unchanged'] and [c['exit'] for c in baseline['commands']]==[0,0,101,101]
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
old_probe=load('duration-before-probe.json');new_probe=load('duration-after-probe.json');assert 'Err(' in old_probe[0]['stdout'] and '--55s' in old_probe[1]['stdout'];expected=load('magnitude-calculation.json')['expected'];assert all('min\tOk("'+expected+'")' in r['stdout'] and 'Err(' not in r['stdout'] for r in new_probe)
package=root/Path(proposal['review']).parent;assert sha(package/'manifest.sha256')==proposal['manifest_sha256']
for line in (package/'manifest.sha256').read_text().splitlines():
 h,p=line.split('  ',1);assert sha(package/p)==h
mem=receipt['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0';peak=int(mem['memory.peak'])/1024**3
counts=collections.Counter(x['code'] for x in inventory)
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:sha(p) for p in before['files']},clippy=9,rules=dict(counts),suite=suite,memory_peak_gib=peak,public_proposal=proposal)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n');(out/'remaining-rules.json').write_text(json.dumps(dict(counts.most_common()),indent=2)+'\n')
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(before['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p in scope['sources']+scope['owners']))
classification=load('classificacao-13.json')
for row in classification['rows']:
 if row['status']=='exceção nominal aplicada; validação pendente':row['status']='exceção nominal prescrita e validada'
(out/'classificacao-13.json').write_text(json.dumps(classification,ensure_ascii=False,indent=2)+'\n')
s=io.StringIO();w=csv.DictWriter(s,fieldnames=list(classification['rows'][0]));w.writeheader();w.writerows(classification['rows']);(out/'classificacao-13.csv').write_text(s.getvalue())
link=lambda n:f'[{n}]({out/n})'
lines=['# Pendências após traits de domínio e Duration\n\n',f'Inventário: **9 avisos**, medido {v["end"]} UTC, HEAD `{before["head"]}`, working tree não commitado. {link("validation-details.json")} conserva estado/hashes/diffstat.\n\n','| Regra | Quantidade | Encaminhamento |\n|---|---:|---|\n','| should_implement_trait | 1 | FromIterator para Styles proposto; aguarda ratificação/materialização |\n','| inherent_to_string | 3 | Display/ToString propostos; aguarda ratificação/materialização |\n','| large_enum_variant | 5 | Medir alternativas; um enum interno e quatro contratos públicos |\n']
for item in inventory:
 span=item['spans'][0];p=span['file_name'];owner=re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M)[1];lines.append(f'\n- [{p}:{span["line_start"]}]({root/p}:{span["line_start"]}) — `{item["code"]}`; L0 [{owner}]({root/owner}).\n')
lines.append(f'\nProposta pública: [review.md]({package}/review.md), manifesto SHA-256 `{proposal["manifest_sha256"]}`. Não adicionar impls ou alterar métodos antes de ratificação. Enums não estão incluídos. A análise dos 13 casos iniciais está em {link("classificacao-13.json")}.\n')
(out/'lista-pendencias.md').write_text(''.join(lines))
report=f'''# Traits de domínio e extremo de Duration — etapa concluída

Clippy **13 → 9 avisos**, sem novos avisos: quatro nomes de domínio receberam exceções nominais prescritas nos respectivos L0; o aviso de Styles::from_iter permanece visível para correção por FromIterator. Além da manutenção, foi corrigido um defeito real de formatação de Duration no limite i128::MIN. Cinco consumers e seus L0 foram atualizados antes do código e resselados de forma restrita. {link('stage.diff')} · {link('classificacao-13.json')}.

## Decisões de nomes

Lexer.next retorna token/kind e produz End explicitamente; o parser controla modo/cursor. Locator.next aloca Location e mantém overflow explícito, sem fim normal Option::None. Decimal.from_str e Version.from_str são parsers Option; adicionar FromStr exige Result e um contrato de erro diferente. Preservados todos os corpos/assinaturas, rejeições, IDs, checkpoints, versão e defaults. Cada método recebeu somente expect(should_implement_trait) com razão nominal no código e no L0. Nenhum limiar, módulo/impl/crate ou outro lint foi silenciado; expectativas conferidas como erro. Não foi alegado ganho de leitura por IA ou paridade por esta comparação de interfaces Rust.

## Defeito corrigido

A sonda anterior com o consumer real produziu panic por overflow em debug e **--55s** em release para Duration::from_nanos(i128::MIN). MIN+1, MAX, zero e -1s foram controles. A causa era abs() em i128, incapaz de representar a magnitude positiva de MIN. Agora unsigned_abs() fornece u128 e os divisores positivos das mesmas unidades são convertidos a u128, preservando a decomposição e o único prefixo de sinal.

O resultado exato é **{expected}**. A expectativa vem de divisão inteira independente de 2^127, sem f64. Constructor, campos, accessors, Neg, Default, operações aritméticas e repr Typst não foram alterados. Esta correção não encerra a política de overflow fora da formatação. {link('duration-before-probe.json')} · {link('magnitude-calculation.json')} · {link('duration-after-probe.json')}.

## Validação

Antes da correção passaram **1682 testes de entidades e 40 de lexer**; o novo teste de extremos falhou por overflow e o gate Clippy observou os cinco métodos inicialmente sinalizados. Após a mudança, a sonda debug/release produz a string correta sem panic e todos os controles permanecem. {link('baseline-details.json')}.

Suíte workspace completa: **{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados**, incluindo os dois novos testes de limites e nanos negativos. Build, Clippy all-targets, gate estrito de expectativas/aridade/dead_code/vec_box e lints já fechados, formato, Rustdoc com warnings como erro, quatro índices HTML, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

Tekt mantém **{sum(v['architecture_warnings'].values())} avisos**, {v['architecture_warnings']}, sem erros nem aumentos por regra. A auditoria/cópia proposta dos quatro L0 públicos não foi aplicada aos owners ativos. Os cinco enums grandes permanecem pendentes; tamanhos indicados pelo Clippy não são benchmark de RSS/tempo ou aprovação para alterar campos públicos.

## Proposta pública separada

Display para Decimal/Duration/Version, retirando os três métodos inerentes to_string em favor de ToString padrão; FromIterator<Style> para Styles, conservando Styles::from_iter como adapter. Os quatro L0 completos, delta e testes T1–T5 estão em [review.md]({package}/review.md), com manifesto SHA-256 **{proposal['manifest_sha256']}**. A confirmação ADR-0127 §2.1/§2.4 foi solicitada por novos contratos públicos e migração de métodos. Até ratificação e materialização, os quatro avisos continuam abertos. Os enums não fazem parte desse pacote. Sem publicação de release.

## RAM e proveniência

Serviços pesados com **12 GiB, swap zero e CARGO_BUILD_JOBS=1**. Pico da validação **{peak:.3f} GiB**, incluindo cache; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. {link('validation-result.json')}.

HEAD `{before['head']}`, working tree não commitado; snapshot {before['utc']}; validação {v['start']}–{v['end']} UTC. Fontes/L0 imutáveis durante os comandos. {link('before.json.gz')} e {link('final-state.json')} guardam fontes/hashes, status e git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150 e vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, conferido pela suíte. Scripts em workers/; {link('manifest.sha256')}.

Restam **9 avisos Clippy** e a dívida Tekt. {link('lista-pendencias.md')}. A meta geral continua aberta. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=9,tests=suite['passed'],peak_gib=peak,proposal_manifest=proposal['manifest_sha256'])))
