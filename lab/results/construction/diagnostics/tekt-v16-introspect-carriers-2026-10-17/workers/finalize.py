from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,collections,difflib
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();v=json.load(open(out/'validation-result.json'));assert v['complete'];hashes={p:sha(p) for p in b['hashes']};assert hashes==v['source_hashes'];assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==b['head'];red=json.load(open(out/'red-result.json'));assert red['functional_red'] and not red['compiler_error'];before=json.load(open(out/'before-citation-probe.json'));after=json.load(open(out/'after-citation-probe.json'));assert (before['differences'],after['differences'],after['controls'])==(12,0,12);assert before['source_sha256']==after['source_sha256'];assert sha(after['rlib']['path'])==after['rlib']['sha256'];p=json.load(open(out/'after-probe-result.json'));assert p['results'][1]['codes']==['E0004']*2 and p['results'][0]['cases']==290;w=json.load(open(out/'workspace/workspace-result.json'));assert (w['passed'],w['failed'],w['ignored'],w['exit'],w['sources_unchanged'])==(8926,0,7,0,True);arch=json.load(open(out/'architecture-delta.json'));assert arch['after']=={'V16:error':107,'V19:note':1096,'V20:note':873};assert json.load(open(out/'integrity-result.json'))['exact_replay'];resources=[];events=collections.Counter();seen=set()
for path in sorted(out.rglob('*-resources.json')):
 r=json.load(open(path));key=json.dumps(r,sort_keys=True)
 if key in seen:continue
 seen.add(key);assert r['memory_before']['memory.max']==str(12*1024**3) and r['memory_before']['memory.high']==str(10*1024**3) and r['memory_before']['memory.swap.max']=='0';a=dict(line.split() for line in r['memory_before']['memory.events'].splitlines());z=dict(line.split() for line in r['memory_after']['memory.events'].splitlines());events.update({k:int(z[k])-int(a[k]) for k in z});resources.append(dict(path=str(path),record=r))
peak=max(int(r['record']['memory_after']['memory.peak']) for r in resources);assert peak<=12*1024**3 and events['oom']==events['oom_kill']==0
state=dict(utc=now(),head=b['head'],source_hashes=hashes,binaries={n:dict(path=p,sha256=sha(p)) for n,p in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]},rlib=after['rlib'],scope=scope,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),workspace=w,architecture=arch,memory=dict(peak_bytes=peak,peak_gib=peak/1024**3,events=dict(events),executions=resources),objective='active');assert state['binaries']['vanilla']['sha256']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
with gzip.open(out/'final-state.json.gz','wt') as f:json.dump(state,f)
(out/'changes.diff').write_text(''.join(''.join(difflib.unified_diff(b['files'].get(path,'').splitlines(True),Path(path).read_text().splitlines(True),fromfile='before/'+path,tofile='after/'+path)) for path in sorted(set(scope['sources'])|set(scope['owners'].values()))))
rows=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];remaining=[r for r in rows if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,indent=2)+'\n');(out/'inventario-v16-restante.md').write_text('# V16 restantes\n\nMedição '+state['utc']+', HEAD '+b['head']+', working tree não commitado; fonte/status/diffstat em final-state.json.gz.\n\n'+''.join('- `'+r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')+':'+str(r['locations'][0]['physicalLocation']['region']['startLine'])+'` — '+r['message']['text'].replace('\n',' ')+'\n' for r in remaining))
(out/'follow-ups.md').write_text('''# Continuação

- Prioridade I2-bibliography-static-coverage: auditar demais containers estáticos e completar coleta/conversão conforme a vista preparada, sem callbacks Dynamic na coleta nem execução de recipes. O V16 do collector continua presente; seu wildcard não foi legitimado como terminalidade.
- Restam 107 V16 no inventário; continuar lotes por responsabilidade, com L0 antes do código e V19 uniformes justificados.
- Os 12 cenários residuais de string/hexadecimal/include, document.format, âncoras matemáticas, igualdade std entre arquivos, RG1/FG1/RM1/SY1 e demais frentes anteriores permanecem abertos. RG1 aguarda aprovação e rebase dos candidatos.
- A compatibilidade de texto dos rótulos não substitui referências numéricas autoritativas, não desmonta Styled e não fecha paridade geral de suplementos/idioma/render.
- Objetivo global ativo; sem commit/push/merge/release.
''')
(out/'relatorio.md').write_text(f'''# Relatório — introspecção I1: carriers, rótulos e ações

Etapa concluída. V16 **109 → 107**, V19 **1094 → 1096 notas**, V20 **873 → 873 notas**. Foram corrigidas duas perdas de informação causadas por Source/carriers, sem silenciar o débito da travessia bibliográfica restante.

## Alterações

A coleta de chaves e a conversão de referências visitam Source, ShowRecipe e Located pelo body e Realized pelo visual. A reconstrução modifica só esse campo, preservando span, Location, original e captura de recipe. A coleta não chama Dynamic; a conversão mantém o comportamento Dynamic já existente. Não executa recipes. O fast path sem chaves permanece idêntico.

compute_labelled reconhece Source por delegação ao mesmo helper com Introspector, Location e idioma intactos. Heading, Equation block e Figure mantêm sua projeção direta. Styled não é desmontado; outros alvos continuam sem associação inventada. Esse texto é compatibilidade e não substitui o caminho numérico autoritativo de referência.

O fallback de label tem 117 variantes nominais, incluindo Equation quando block recusa. O match de ação automática de ElementPayload tem 17 alternativas negativas explícitas (12 variantes e as recusas bool de Heading/Figure/Table/Equation). Os corpos ativos, profundidade, eventos, stores, callbacks e ordem não mudaram. As duas notas V19 permanecem visíveis e prescritas nos L0.

## Limite explícito I1/I2

A bibliografia continua **incompleta** para outros containers estáticos. I2-bibliography-static-coverage completará sua auditoria e travessia (células raw, headers/footers, Repeat/Columns e demais corpos). O wildcard de collect_bib_keys permanece V16 e o passthrough legado de convert_refs continua registrado como dívida; não foram chamados de terminais legítimos nem silenciados. O L0 inicial declara essa divisão. Este lote não redefine o objetivo global nem certifica bibliografia/referências universais.

## Evidência

- Sonda sobre tipos reais: o algoritmo original reconstruído perde a ordem nos **12 cenários**; o código vigente preserva **k2,k1 nos 12**, com verificação adicional de citation_order público. São quatro carriers em três posições: raiz, bibliografia e referências. O baseline executa os algoritmos originais extraídos do snapshot, conforme a metodologia abaixo; não é uma medição da API pública antiga na rlib atual. Os artefatos before/after-citation-probe.json conservam programa/rlib/source freeze com hashes, argv, UTC e stdout. A rlib medida permaneceu idêntica até o fechamento.
- RED funcional: dois testes falharam e um controle passou antes da implementação. Source perdia Seção 1, Equação (1) e Figura 1 no mesmo Location; as três projeções agora coincidem com o alvo direto. Os controles preservam original oculto de Realized, referência desconhecida, dados de carrier e limites de Styled.
- Os 290 controles simbólicos mantêm a classificação, salvo os dois casos Source do modelo de label. O modelo mantém nomes/formas de enums e condições bool, com payloads opacos; não é prova de seus valores ou paridade. A variante fictícia passa de zero para dois E0004. Replay exato dos corpos e regressão real completam a prova.
- Workspace: **{w['passed']} aprovados, {w['failed']} falhas, {w['ignored']} ignorados**. Build, fmt, Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito/quatro índices, V1/V5/V15/V26, reverse lineage, replay e diff --check passaram. SARIF preservou o V16 bibliográfico e todos os demais diagnósticos por fonte/regra/severidade/mensagem; sem V20 novo.
- Recursos sequenciais: MemoryMax12GiB/High10GiB/swap0/jobs1. Pico **{peak/1024**3:.6f} GiB** ({peak} bytes), eventos {dict(events)}. Sem OOM/OOM kill.

## Proveniência

HEAD `{b['head']}`, working tree não commitado. Baseline `{b['utc']}`, fechamento `{state['utc']}`. before.json.gz registra textos/L0/status/diff HEAD --stat e {len(b['hashes'])} hashes conferidos; final-state.json.gz congela o estado final. Três pares source/L0, sem campo/assinatura pública, default, fase ou dependência nova. O pin source-origin foi consumido nos dois owners pertinentes sem alterar o núcleo. L0 antes de testes/código, fluxo interno ADR-0127. Linter 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 {b['linter']['sha256']}.

A transparência de carriers é uma prova metamórfica exigida pelo contrato já ratificado; esses carriers não têm syntax vanilla. Não se inventa igualdade de representação Rust. O alvo de linguagem permanece o commit 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

O primeiro GREEN identificou uma fixture Located inválida: duas referências distintas recebiam o mesmo Location. A identidade produtiva foi preservada; o teste passou a usar um Locator por árvore com prefixo reservado. O baseline válido executa os algoritmos originais extraídos de before.json.gz sobre tipos reais da rlib, com adaptação apenas de imports; o observer estrito de Cite confere sua ordem e o GREEN também confere citation_order público. Essa reconstrução é distinguida da medição pública inicial arquivada em located-fixture-attempt.

Uma montagem inicial da sonda cortou os campos de Figure numa chave de doc comment; E0026 foi corrigido antes de contar controles, sem classificá-lo como RED funcional (probe-assembly-attempt). Três imports de fixture irrelevantes foram retirados conforme o gate Clippy, sem alterar assertivas; o replay inclui esse ajuste. Os testes não foram relaxados.

O inventário lista os 107 V16 restantes e follow-ups.md preserva I2 e as demais frentes. Pacotes TS1 aprovado e RG1 pendente conferidos pela entrega. Objetivo global ativo; sem commit/push/merge/release.
''');print('Closed',arch['after'],'tests',w['passed'],'peak',peak)
