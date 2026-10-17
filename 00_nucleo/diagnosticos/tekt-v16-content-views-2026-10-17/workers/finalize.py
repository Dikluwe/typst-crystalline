from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();v=json.load(open(out/'validation-result.json'));assert v['complete'];hashes={p:sha(p) for p in list(b['hashes'])+scope['nuclei']};assert hashes==v['source_hashes'];assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];red=json.load(open(out/'red-result.json'));assert red['functional_red'] and not red['compiler_error'];a=json.load(open(out/'before-field-probe.json'));z=json.load(open(out/'after-field-probe.json'));assert (a['controls'],a['differences'],z['controls'],z['differences'])==(84,48,84,0);assert a['source_sha256']==z['source_sha256'];assert sha(z['rlib']['path'])==z['rlib']['sha256'];p=json.load(open(out/'after-probe-result.json'));assert p['results'][1]['codes']==['E0004']*6 and p['results'][0]['cases']==17280 and len(p['symbolic_changes'])==96;w=json.load(open(out/'workspace/workspace-result.json'));assert (w['passed'],w['failed'],w['ignored'],w['exit'],w['sources_unchanged'])==(8923,0,7,0,True);arch=json.load(open(out/'architecture-delta.json'));assert arch['after']=={'V16:error':109,'V19:note':1094,'V20:note':873};assert json.load(open(out/'integrity-result.json'))['exact_replay'];resources=[];events=collections.Counter();seen=set()
for path in sorted(out.rglob('*-resources.json')):
 r=json.load(open(path));key=json.dumps(r,sort_keys=True)
 if key in seen:continue
 seen.add(key);assert r['memory_before']['memory.max']==str(12*1024**3) and r['memory_before']['memory.high']==str(10*1024**3) and r['memory_before']['memory.swap.max']=='0';x=dict(line.split() for line in r['memory_before']['memory.events'].splitlines());y=dict(line.split() for line in r['memory_after']['memory.events'].splitlines());events.update({k:int(y[k])-int(x[k]) for k in y});resources.append(dict(path=str(path),record=r))
peak=max(int(r['record']['memory_after']['memory.peak']) for r in resources);assert peak<=12*1024**3 and events['oom']==events['oom_kill']==0
state=dict(utc=now(),head=b['head'],source_hashes=hashes,binaries={n:dict(path=p,sha256=sha(p)) for n,p in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]},rlib=z['rlib'],scope=scope,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),workspace=w,architecture=arch,memory=dict(peak_bytes=peak,peak_gib=peak/1024**3,events=dict(events),executions=resources),objective='active')
assert state['binaries']['vanilla']['sha256']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'changes.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'].get(path,'').splitlines(True),Path(path).read_text().splitlines(True),fromfile='before/'+path,tofile='after/'+path)) for path in sorted(set(scope['sources'])|set(scope['owners'].values())|set(scope['nuclei']))))
rows=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];remaining=[r for r in rows if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');(out/'inventario-v16-restante.md').write_text('# V16 restantes\n\nMedição: '+state['utc']+', HEAD '+b['head']+', working tree não commitado; fonte/UTC/status/diffstat em final-state.json.gz.\n\n'+''.join('- `'+r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')+':'+str(r['locations'][0]['physicalLocation']['region']['startLine'])+'` — '+r['message']['text'].replace('\n',' ')+'\n' for r in remaining))
(out/'follow-ups.md').write_text('''# Continuação

- Restam 109 V16 no inventario-v16-restante.md e remaining-v16.json; o gate global continua bloqueado. Continuar por domínio, com auditoria dos L0 e sem apagar complementos V19 legítimos.
- Os 12 cenários comparativos de string/hexadecimal/morfologia de include, document.format, âncoras matemáticas, igualdade de std entre arquivos e RG1/FG1/RM1/SY1 permanecem frentes próprias dos relatórios anteriores.
- RG1 aguarda aprovação; candidatos futuros precisam de rebase sobre o estado atual, preservando as correções de TS1, chamadas, imports e projeções.
- O lote não prova toda composição de Styled, todos os campos ou paridade universal de callbacks/transformações; corrige a assimetria medida de quatro carriers na identificação de Equation.
- Objetivo global ativo, sem commit/push/merge/release.
''')
(out/'relatorio.md').write_text(f'''# Relatório — projeções de Content e campos de equação

Etapa concluída. V16 **115 → 109**, V19 **1088 → 1094 notas**, V20 **873 → 873 notas**. Além dos seis complementos nominais, foi corrigida uma perda real de campos de Equation quando um carrier interno aparece dentro de Styled.

## Correção

styled_equation reconhece agora Source, Located e ShowRecipe pelo body e Realized pelo visual. As leituras de numbering, supplement, alt, number-align, body e block deixam de depender da posição do wrapper. Não executa receitas nem consulta original de Realized. O getter mantém sua precedência de estilos, incluindo fallback ao nível interno quando o externo omite a chave; None explícito continua diferente de ausência. Styled de outro elemento não ganha campos de equação.

Os seis V16 pertenciam a is_empty, styled_equation, get_field, morph_canon, Hash::hash e source_span em cinco consumers/L0. Os complementos têm respectivamente 63, 114, 94, 112, 114 e 117 variantes. Seus valores/delegações anteriores foram preservados; Styled com guard recusado permanece no complemento. String de campo continua domínio aberto. O helper causal source_body não mudou. Não houve campo, variante, assinatura pública, default ou fase nova.

O núcleo content/projection-exhaustiveness.toml está pinado nos cinco owners; source-origin conserva seu pin e obrigação anteriores. L0 foi salvo antes dos testes/código. As seis notas V19 mantêm grupos uniformes explícitos, com justificativa por owner e cobertura de cada variante no modelo. Não há supressão, registry ou despacho dinâmico novo. Não se alega ganho medido de leitura por IA.

## Provas e validação

- Medição real na rlib: **48 diferenças em 84 controles → zero em 84**. Programa, fonte, argv, UTC, hashes e saídas em before/after-field-probe.json e field-control.rs. Quatro carriers, três posições e sete nomes de campo distinguem override, body/block, None explícito e ausência. A rlib medida permaneceu idêntica até o fechamento.
- RED funcional: os dois testes novos falharam antes da implementação; reproduziram a matriz e a precedência. GREEN preserva também os negativos não-equação e Realized com original Equation/visual texto.
- Proteção estrutural: uma variante futura de Content no modelo descartável passou de zero para **seis E0004**, um por local. **17280 controles simbólicos** exercitam 120 variantes, 12 campos, dois estados de guard e seis decisões; todas as variantes de cada complemento são exercitadas. Apenas os 96 casos simbólicos do helper para os quatro carriers mudam de classe, conforme a correção real. O modelo usa payloads opacos/guard booleano: não é prova de semântica de payload nem comparação vanilla. Replay textual e testes reais completam essa limitação.
- Suíte workspace: **{w['passed']} aprovados, {w['failed']} falhas, {w['ignored']} ignorados**. Passaram build, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito/quatro índices, V1/V5/V15/V26, reverse lineage, replay e diff --check. O diferencial confirma somente as seis remoções V16 e seis notas V19 prescritas, sem V20 novo ou severidade relaxada.
- Recursos sequenciais: MemoryMax12GiB/High10GiB/swap0/Cargo jobs1. Pico **{peak/1024**3:.6f} GiB** ({peak} bytes); eventos {dict(events)}. Sem OOM/OOM kill.

## Proveniência e tentativas

HEAD `{b['head']}`, working tree não commitado; baseline `{b['utc']}`, fechamento `{state['utc']}`. before.json.gz registra textos/L0/status/diff HEAD --stat e os {len(b['hashes'])} hashes conferidos da etapa ssty; final-state.json.gz congela {len(hashes)} arquivos e binários. validation-result.json preserva argv/UTC/source freeze e recursos. Content e todos os arquivos protegidos fora dos cinco pares permaneceram inalterados. Linter 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 {b['linter']['sha256']}.

O contrato já ratificado exige transparência de carriers internos: a prova é metamórfica, contra a mesma equação sem wrapper. Não há syntax vanilla para esses carriers e não se inventa equivalência de representação Rust. O alvo de linguagem permanece o commit 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

A primeira preparação escalada não foi executada porque a revisão automática de permissão expirou. Foi confirmada a ausência dos destinos e a preparação local mais execução mínima limitada funcionaram. O scanner inicial omitiu quatro variantes struct por whitespace; foi corrigido para 120 antes de escrever L0/código. Uma importação Python ausente interrompeu a montagem da sonda antes do rustc; não foi contada como RED funcional. Registros preservados em probe-assembly-attempt.

O gate global ainda tem 109 V16; inventario-v16-restante.md lista cada local. TS1 aprovado e RG1 pendente são conferidos pela entrega. As frentes restantes continuam em follow-ups.md. Objetivo global ativo; sem commit/push/merge/release.
''');print('Closed',arch['after'],'tests',w['passed'],'peak',peak)
