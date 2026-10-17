from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,difflib,collections,csv,io,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-ranges-2026-10-16';load=lambda p:json.loads((out/p).read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));v=load('validation-details.json');service=load('validation-result.json');suite=load('workspace/workspace-result.json');scope=load('scope.json');strict=load('strict-details.json');baseline=load('baseline-details.json');probe=load('probe-details.json')
assert service['exit']==0 and len(v['commands'])==12 and all(c['exit']==0 for c in v['commands'])
assert not load('clippy-inventory.json') and not strict['diagnostics'] and strict['exit']==0 and v['clippy_delta']=={}
assert v['sources_unchanged_during_validation'] and all(sha(p)==h for p,h in v['source_hashes'].items())
assert suite['exit']==0 and suite['failed']==0 and suite['sources_unchanged']
assert baseline['sources_unchanged'] and [c['exit'] for c in baseline['commands']]==[0,0,1]
assert probe['sources_unchanged'] and probe['measurements']==dict(bytes_compared=256,accepted_bytes=22,pairs_compared=65536,accepted_pairs=484,route_controls=13)
assert v['architecture_warnings']=={'V16':327,'V17':28,'V12':19}
for crate in ['typst_core','typst_shell','typst_infra','typst']:assert '<!DOCTYPE html>' in (root/'target/doc'/crate/'index.html').read_text()[:100]
subprocess.run(['python3',str(out/'workers/guard.py')],check=True,stdout=subprocess.DEVNULL)
mem=service['memory_after'];events=dict(s.split() for s in mem['memory.events'].splitlines());peak=int(mem['memory.peak'])/1024**3;assert int(mem['memory.max'])==12*1024**3 and int(mem['memory.swap.max'])==0 and events['oom']=='0' and events['oom_kill']=='0'
state=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=v['source_hashes'],clippy=0,tekt=v['architecture_warnings'],suite=suite,memory_peak_gib=peak)
(out/'final-state.json').write_text(json.dumps(state,indent=2)+'\n')
(out/'stage.diff').write_text(''.join(''.join(difflib.unified_diff(before['files'][p].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p in scope['sources']+scope['owners']))
results=json.loads((out/'validation-architecture.log').read_text())['runs'][0]['results'];remaining=[]
for r in results:
 if r['level']!='warning':continue
 loc=r['locations'][0]['physicalLocation'];p=loc['artifactLocation']['uri'].removeprefix('./');owner=re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M)[1];remaining.append(dict(rule=r['ruleId'],source=p,line=loc['region']['startLine'],owner=owner,message=r['message']['text']))
(out/'remaining.json').write_text(json.dumps(remaining,ensure_ascii=False,indent=2)+'\n')
text=f'''# Pendências Tekt — após V18

374 avisos, Clippy zero, HEAD `{before['head']}`, working tree não commitado. Medição {v['end']} UTC; validation-details.json e final-state.json registram hashes, status e git diff HEAD --stat. As notas informativas permanecem no SARIF e não entram como avisos.

| Próxima ordem | Regra | Avisos |
|---|---|---:|
| 1 | V17 — guards compostos | 28 |
| 2 | V16 — wildcards de domínio | 327 |
| 3 | V12 — tipos/impl em L4 | 19 |

Diagnóstico, não L0 ou autorização de código. Ler cada owner antes de prescrever a mudança. V12 contém um enum produtivo e 18 declarações em arquivos de teste; a distinção exige decisão, sem isentar automaticamente ou mover fixtures só para esconder aviso. Não rebaixar regras globais ou silenciar V16 por citação. Os seis V18 estão encerrados no estado medido.

| Regra | Local | Owner | Evidência |
|---|---|---|---|
'''
for r in remaining:text+=f'| {r["rule"]} | [{r["source"]}:{r["line"]}]({root/r["source"]}:{r["line"]}) | [{r["owner"]}]({root/r["owner"]}) | '+r['message'].replace('|','\\|').replace('\n',' ')+' |\n'
(out/'lista-pendencias.md').write_text(text)
link=lambda p:f'[{p}]({out/p})'
report=f'''# V18 — cardinalidade e hexadecimal: etapa concluída

Tekt **380 → 374 avisos**: os seis V18 foram corrigidos, sem supressões ou avisos novos por regra. **Clippy permanece zero**, inclusive workspace/all-targets com -D warnings e expectativas não cumpridas como erro. Dois consumers e seus dois L0 foram atualizados antes do código e resselados de modo restrito. {link('stage.diff')} · {link('code-guard.json')}.

## O que mudou

Em FloatIsNanCallSpans e nos ramos Attach/Mono/Script de P1293MathCallSpans, os quatro matches agora observam a forma de args.items.as_slice(): vazio, um item ou dois-ou-mais. Os corpos de seleção dos spans permanecem. Named/bound/spread e sua prioridade, fallback de posição ausente, erros/tipos vizinhos e as decisões Binom/HTML ficaram intactos. A forma da coleção expressa a cardinalidade diretamente, sem faixas numéricas em match de domínio.

No decoder privado do servidor de preview, somente a closure de dígito hexadecimal manual foi substituída por char::from(value).to_digit(16).map(... as u8). Permanecem lowercase, limites e combinação dos bytes, UTF-8, query, normalização, NUL/backslash/above-root, barra final e decodificação única. Sem helper/API nova, I/O adicional, mudança de fase/default ou política de rotas; HTTP/SSE/publicação não foram alterados.

## Medição e RED→GREEN

A sonda extrai a função decoder original completa e cria o candidato com uma única substituição. Todos os **256 bytes** e **65.536 pares** coincidem: **22 bytes** e **484 pares** reconhecidos como hexadecimais; 13 controles de rotas coincidem com saídas exigidas. É evidência de equivalência local, sem nova prova de paridade global, benchmark de HTTP, ganho de memória ou leitura por IA. {link('probe-details.json')}.

Antes da implementação passaram os quatro novos controles funcionais (dois de spans, dois de decoder). Eles protegem o comportamento existente; não foi criada uma falha artificial de produto. O gate **--checks v18 --fail-on warning** falhou com os seis avisos esperados. Após a correção, o mesmo gate passou sem V18. {link('baseline-details.json')} · {link('baseline-red.log')} · {link('validation-v18.log')}.

Os testes cobrem cardinalidades 0/1/2/4 com named/bound/spread, falta de posição, diferenças de precedência Attach/Mono/Script/cramped, octetos percent-encoded upper/lower, ASCII e rejeição UTF-8, e rotas com raiz, query, traversal, truncamento, percent inválido, NUL/backslash e dupla codificação. As expectativas antigas não foram alteradas. Fluxo contínuo ADR-0127: manutenção interna, com L0 primeiro, sem contrato público ou compatibilidade nova.

## Validação e recursos

Suíte workspace: **{suite['passed']} aprovados, {suite['failed']} falhas, {suite['ignored']} ignorados**. Build, Clippy normal e estrito, formato, V18, linhagem estrita V1/V5/V15/V26 e reversa, Rustdoc -D warnings com quatro índices HTML e diff --check passaram. {link('validation-details.json')} · {link('strict-details.json')} · {link('workspace/workspace-result.json')}.

Tekt restante: {v['architecture_warnings']}, sem erros. {link('lista-pendencias.md')}. A meta geral continua aberta: 374 avisos ainda exigem correção/decisão e prescrição nos owners.

Serviços pesados sequenciais: **12 GiB**, swap zero e CARGO_BUILD_JOBS=1. Pico **{peak:.3f} GiB** do cgroup, incluindo cache; max={events['max']}, OOM={events['oom']}, OOM-kill={events['oom_kill']}. {link('validation-result.json')}.

## Proveniência

HEAD `{before['head']}`, working tree não commitado. Snapshot {before['utc']}; validação {v['start']}–{v['end']} UTC, fontes/L0 e configurações congelados durante os comandos. {link('before.json.gz')} · {link('final-state.json')} guardam código/L0/config/manifests/lock, hashes, status e git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150 com hash de binário reconfirmado; vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/; {link('manifest.sha256')}. Sem commit, push ou release.
'''
(out/'relatorio.md').write_text(report)
(out/'manifest.sha256').write_text(''.join(f'{sha(p)}  {p.relative_to(out)}\n' for p in sorted(out.rglob('*')) if p.is_file() and p.name!='manifest.sha256'))
print(json.dumps(dict(report=str(out/'relatorio.md'),clippy=0,tekt=len(remaining),tests=suite['passed'],peak_gib=peak)))
