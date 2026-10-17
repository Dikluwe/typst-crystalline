from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,difflib,collections,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-enums-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');service=load('validation-result.json');suite=load('workspace/workspace-result.json');scope=load('scope.json');strict=load('strict-details.json');baseline=load('baseline-details.json');probe_before=load('probe-baseline-details.json');probe_after=load('probe-green-details.json');gate=load('scope-gate-result.json')
assert service['exit']==0 and len(v['commands'])==13 and all(c['exit']==0 for c in v['commands'])
assert not load('clippy-inventory.json') and not strict['diagnostics'] and strict['exit']==0 and v['clippy_delta']=={}
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged'] and suite['passed']==8621 and suite['ignored']==7
assert baseline['sources_unchanged'] and [c['exit'] for c in baseline['commands']]==[0,1,0]
assert 'test result: ok. 1695 passed; 0 failed; 0 ignored;' in (out/'baseline-entities.log').read_text()
assert probe_before['selected_e0004']==0 and probe_after['selected_e0004']==21 and len(probe_after['targets'])==21
assert all(t['e0004'] for t in probe_after['targets']) and not any(t['e0004'] for t in probe_before['targets'])
assert probe_before['sources_unchanged'] and probe_after['sources_unchanged'] and not probe_before['other_errors'] and not probe_after['other_errors']
assert v['architecture_warnings']=={'V16':312,'V12':19} and not load('architecture-delta.json')['added_warnings']
assert gate['exit']==0 and gate['selected_warnings_or_errors']==0 and gate['outside_warnings']==312 and gate['global_exit']!=0
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
for rec in [service,load('baseline-result.json'),load('contexts-result.json')]:
 ev=dict(s.split() for s in rec['memory_after']['memory.events'].splitlines());assert ev['oom']=='0' and ev['oom_kill']=='0' and int(rec['memory_after']['memory.max'])==12*1024**3 and int(rec['memory_after']['memory.swap.max'])==0
mem=service['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());peak=int(mem['memory.peak'])/1024**3
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],clippy=0,tekt=v['architecture_warnings'],suite=suite,memory_peak_gib=peak,additional_notes='21 V19 notes about grouped alternatives retained in new-notes.json',latent_v16='latent-v16.json retained from V17')
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n')
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(before['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p in scope['sources']+scope['owners']))
results=json.loads((out/'validation-architecture.log').read_text())['runs'][0]['results'];remaining=[]
for r in results:
 if r['level']!='warning':continue
 loc=r['locations'][0]['physicalLocation'];p=loc['artifactLocation']['uri'].removeprefix('./');owner=re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M)[1];remaining.append(dict(rule=r['ruleId'],source=p,line=loc['region']['startLine'],owner=owner,message=r['message']['text']))
(out/'remaining.json').write_text(json.dumps(remaining,ensure_ascii=False,indent=2)+'\n')
latent_path=Path('00_nucleo/diagnosticos/tekt-guards-2026-10-16/latent-v16.json');latent=json.loads(latent_path.read_text());assert all(sha(d['source'])==before['hashes'][d['source']] for d in latent);(out/'latent-v16.json').write_text(json.dumps(latent,ensure_ascii=False,indent=2)+'\n')
text=f'''# Pendências Tekt — após representação e identidade V16

**331 avisos emitidos**, Clippy zero; HEAD `{before['head']}`, working tree não commitado. Medição {v['end']} UTC; validation-details.json e final-state.json conservam hashes, status e git diff HEAD --stat. As notas ficam no SARIF; new-notes.json registra as 21 novas V19 de agrupamento nominal, sem suprimi-las.

| Ordem | Frente | Situação |
|---|---|---|
| 1 | V16 — domínio | 312 avisos e os locais sem emissão preservados abaixo |
| 2 | V12 — tipos/impl em L4 | 19 avisos: um enum produtivo e 18 declarações de fixtures |

Os 21 matches do lote atual agora têm exaustividade nominal no eixo do enum, provada por E0004 na cópia com sentinelas. Foram retirados 11 avisos e uma nota V16; nove fallbacks sem emissão também foram nominalizados. Não há fechamento global de V16: T027 e T158/T160 continuam rastreados conforme o lote V17, e T311/T313 permanecem no único aviso consolidado de metadata. Não interpretar mudança de heurística como correção desses casos.

## Locais ainda sem emissão, preservados do lote anterior

| IDs | Local | Decisão pendente |
|---|---|---|
'''
for d in latent:text+=f"| {', '.join(d['ids'])} | [{d['source']}:{d['line']}]({root/d['source']}:{d['line']}) | {d['status']} — {d['reason']} |\n"
text+='''
Diagnóstico, não L0 ou autorização de API. Ler cada owner/enum antes da prescrição. Não ocultar V16 com helper, if-let equivalente, citação ou configuração global. V12 exige distinguir composição produtiva de suporte de teste, sem mover fixtures apenas para esconder aviso.

## Avisos correntes

| Regra | Local | Owner | Evidência |
|---|---|---|---|
'''
for r in remaining:text+=f'| {r["rule"]} | [{r["source"]}:{r["line"]}]({root/r["source"]}:{r["line"]}) | [{r["owner"]}]({root/r["owner"]}) | '+r['message'].replace('|','\\|').replace('\n',' ')+' |\n'
(out/'lista-pendencias.md').write_text(text)
link=lambda p:f'[{p}]({out/p})'
report=f'''# V16 — representação e identidade: lote concluído

**11 avisos V16 corrigidos**, uma nota V16 retirada e nove fallbacks adicionais nominalizados: **21 matches em sete consumers**, com sete L0 prescritos antes do código. **Tekt 342 → 331 avisos emitidos**; Clippy permanece **zero**. Não há API, enum/campo novo, alteração de default ou fase. {link('classification.json')} · {link('stage.diff')} · {link('code-guard.json')}.

## Mudança e finalidade

Os catchalls dos enums fechados passaram a nomear as variantes atuais. Os corpos, guards, precedência e chamadas foram preservados. Uma variante futura exige decisão de compilação nesses pontos; não foi substituído match por if-let ou introduzido helper que escondesse a mesma adoção silenciosa. Char e flags de mapeamento matemático conservam seus fallbacks próprios.

| Owner | Matches | Comportamento preservado |
|---|---:|---|
| Color | 1 | Eq por espaço/campos, f32 por bits e Spot estrutural; sem conversão para comparar |
| Color adjustments | 3 | darken, rotate e saturate; mesmas rotas, falhas, fórmulas, alpha e erro interno |
| Color conversion | 4 | Oklab/Oklch, rejeição ProcessToSpot e clone/projeção processual |
| Color mixing | 4 | defaults por compatibilidade Spot e seleção de hue, pesos e ordem |
| MathStyleKind | 4 | VS, bases de letras/dígitos e exceções BMP; normalização Roundhand e char aberto |
| Func | 4 | element_name, namespace, Eq e Hash; nome nativo por kind e Arc para as demais representações |
| StateUpdate | 1 | Set por Value, Func por identidade Arc e pares cruzados distintos |

No L0 math_style, o fragmento atual agora inclui Display/Inline já existentes; a descrição Roundhand acompanha P812-C, e os três exemplos Unicode com redação de rascunho foram saneados. P964 foi qualificada sobre as notas antigas de scope-out. O L0 StateUpdate distingue o registro histórico de stub do caminho atual com Engine e do legacy, delegando execução ao owner correto sem alterar runtime. O caminho desse owner foi canonizado antes da implementação conforme ADR-0130. {link('l0-path-clarification.json')}.

## Prova de evolução — RED→GREEN estrutural

Na cópia descartável do workspace, foi acrescentada uma variante sentinela a cada um de **Color, ColorSpace, MathStyleKind, FuncRepr e StateUpdate**. As definições produtivas permaneceram idênticas, conferidas pelo guard. A sonda compilou o core real, com os mesmos manifests/lock/dependências e cargo check offline/locked; nenhuma substituição de corpos por stubs.

Antes: **{probe_before['selected_e0004']} de 21 matches selecionados** exigiam decisão, enquanto outros matches já produziam {probe_before['total_e0004']} erros E0004 na cópia. Depois: **{probe_after['selected_e0004']} de 21** exigem decisão, dentro de {probe_after['total_e0004']} E0004 totais. A seleção é por source e linha do match, sem confundir erros de outros pontos. Em ambas as rodadas não houve outro tipo de erro de compilação. As cópias foram descartadas; arquivos/hashes/UTC/comandos e diagnósticos permanecem em {link('probe-baseline-input.json.gz')}, {link('probe-baseline-details.json')}, {link('probe-green-input.json.gz')} e {link('probe-green-details.json')}.

O gate V16 por escopo tinha **11 avisos** antes e agora tem **zero** nesses sete consumers. O linter global continua retornando falha por 312 V16 externos; o wrapper registra esse resultado e não o apresenta como gate global aprovado. {link('baseline-red.log')} · {link('scope-gate-result.json')}.

Essa é prova de exaustividade futura no mecanismo Rust, não medição nova de paridade Typst, melhoria de leitura por IA ou performance. A aceitação funcional permanece nos resultados da linguagem e nas políticas de identidade já vigentes.

## Testes e validação

Quatro testes novos foram escritos antes da implementação: matriz de igualdade entre nove espaços Color, clone e política de bits; seis representações de Func com Eq/Hash/namespace/element_name, nomes nativos por kind e ausência de execução de host/ctor; StateUpdate distinguindo Arc de nativas que comparam iguais por nome; exemplos Unicode e variants de tamanho matemático sem mudança de glyph/VS. Nenhum esperado histórico foi relaxado. A baseline de entidades passou com **1.695 testes, zero falhas e zero ignorados**. {link('tests-materialization.json')} · {link('baseline-details.json')}.

Final: **{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados** na suíte workspace. Build, Clippy normal e estrito em todos os targets (-D warnings e -D unfulfilled_lint_expectations), formato, linhagem V1/V5/V15/V26 direta/reversa, Rustdoc -D warnings, quatro índices HTML e diff --check passaram. Fontes/L0/configuração ficaram congelados durante a validação. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

## Notas V19 e pendências

Os padrões OR explícitos acrescentam **21 notas V19**: elas avisam que cobertura por braço não equivale a cobertura de todas as alternativas agrupadas. Os grupos compartilham o mesmo corpo vigente; não se duplicou lógica para eliminar notas. Estão preservadas e classificadas em {link('new-notes.json')}; não foram rebaixadas, suprimidas ou contadas como avisos. Não se alega cobertura completa de caminhos apenas pelo número de testes.

Restam **312 V16 e 19 V12**, sem avisos novos por mensagem/local no SARIF. Os casos sem emissão preservados de V17 permanecem pendentes. Ordem seguinte: V16, depois V12. {link('lista-pendencias.md')} · {link('latent-v16.json')}. A meta geral continua aberta. A extração AST posterior, somente de leitura, reuniu 312 avisos e dois locais sem emissão em 314 contextos e 136 owners, sem resolver tipos por inferência: [índice de contextos](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-wildcards-2026-10-16/indice-contextos.md). contexts-result.json registra seu serviço separado sob o mesmo teto.

## Recursos e proveniência

Execuções pesadas sequenciais em cgroup **MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1**. Pico final **{peak:.3f} GiB**, incluindo cache: atingiu o teto e o kernel aplicou o limite; eventos max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. Baseline: {int(load('baseline-result.json')['memory_after']['memory.peak'])/1024**3:.3f} GiB, também sem OOM. {link('validation-result.json')} · {link('memory-samples.jsonl')}.

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação {v['start']}–{v['end']} UTC. Fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat estão em {link('before.json.gz')}, {link('baseline-details.json')} e {link('final-state.json')}. A preservação do estado externo ao lote foi conferida globalmente. Scripts em workers/; {link('manifest.sha256')}.

Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150, SHA-256 26fe38b564a0aa92e7cc49f3b09a4324d1127e3586862afa5af4480e6c394dde reconfirmado. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 e binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Nenhum commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
files=sorted(p for p in out.rglob('*') if p.is_file() and p.name!='manifest.sha256' and '__pycache__' not in p.parts)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in files))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=0,tekt=len(remaining),tests=suite['passed'],matches=21,peak_gib=peak,manifest_files=len(files))))
