from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,re,collections
p=Path(__file__).resolve().parent;r=p.parents[2];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 h=hashlib.sha256()
 with Path(f).open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
def read(name):return json.loads(gzip.decompress((p/(name+'.json.gz')).read_bytes()))
def save(name,data):(p/(name+'.json.gz')).write_bytes(gzip.compress(json.dumps(data,indent=2,ensure_ascii=False).encode()))
summary=json.loads((p/'complete-summary.json').read_text());assert summary['sources_unchanged'];assert all(row['exit']==0 for row in summary['results'] if row['label']!='architecture')
freeze=read('complete-freeze');assert all(sha(r/f)==h for f,h in freeze['sources'].items())
audit=read('probe-audit');assert not audit['unexpected'];lint=read('lint-diff');assert not [x for x in lint['added'] if x['level'] in ['warning','error']]
release=read('release-result');assert release['build_exit']==0 and all(c['exit']==0 for c in release['cases'])
resources={f.name:json.loads(f.read_text()) for f in p.glob('*-resources.json')};peak=max(int(d['memory_after']['memory.peak']) for d in resources.values())
for d in resources.values():
 assert d['memory_after']['memory.max']==str(12*1024**3) and d['memory_after']['memory.swap.max']=='0'
events=collections.Counter()
for d in resources.values():
 for line in d['memory_after']['memory.events'].splitlines():
  k,v=line.split();events[k]+=int(v)
save('resources',{'utc':now(),'peak_bytes':peak,'events':dict(events),'jobs':resources})
state={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'freeze':freeze,'sources_unchanged':True,'debug_sha256':sha(r/'target/debug/typst'),'release_sha256':sha(r/'target/release/typst'),'before_sha256':sha(p/'outputs/typst-before'),'oracle_sha256':sha(r/'lab/typst-original/target/release/typst'),'linter_sha256':sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')};save('final-state',state)
assert state['debug_sha256']==summary['binary_sha256']
save('evidence',{'utc':now(),'files':{f.name:f.read_text() for f in p.iterdir() if f.is_file() and f.suffix in ['.log','.json']}})
checks='\n'.join(f"| {x['label']} | {x['exit']} | `{ ' '.join(x['command']) }` |" for x in summary['results'])
docrows=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (p/'complete-doctest.log').read_text(),re.M);docs=sum(int(x[1]) for x in docrows)
stress=read('stress-final');stressrows=[]
for c in stress['cases']:
 b=c['results']['before'];a=c['results']['candidate'];v=c['results']['vanilla'];rss=int(a['rss'].splitlines()[-1])/1024;stressrows.append(f"| {c['case']} | {b['exit']} | {a['exit']} | {v['exit']} | {rss:.2f} |")
relrows='\n'.join(f"| {x['label']} | {x['exit']} | {int(x['rss_kib'].splitlines()[-1])/1024:.2f} | {x['text']} |" for x in release['cases'])
text=f'''# Concatenação e pilha das travessias

Corrigida a concatenação que criava sequências encaixadas em blocos/for/while/+ e isolados os frames nativos das reconstruções pré-ordem e bottom-up. O ensaio original 64×2048 e Strong64 agora concluem com a pilha padrão. **O objetivo amplo permanece aberto:** paridade e os V16 anteriores ainda têm pendências. Nenhum commit, push, merge ou publicação de release nesta etapa.

## Proveniência

HEAD `{state['head']}`, working tree não commitado. Congelamento `{freeze['utc']}`, entrega `{state['utc']}`. A lista exata de fontes, scripts, novos arquivos, status e `git diff HEAD --stat` está em [final-state.json.gz](final-state.json.gz). [before-sources.json.gz](before-sources.json.gz) recupera oito arquivos anteriores com SHA-256 idêntico ao checkpoint; [change.patch.gz](change.patch.gz) separa este lote das correções A1. Fonte e L0 permaneceram congelados na validação.

Vanilla pin **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore `cac836c07e4dec2a89219370428292393a3216b5`, executável `{state['oracle_sha256']}`. Tekt anterior `{state['before_sha256']}`; candidato debug `{state['debug_sha256']}`; release `{state['release_sha256']}`. Strings de versão não são usadas como prova.

## Diagnóstico e correções

1. `join` e `+` construíam Sequence([prefixo, novo]) repetidamente. `repr` de quatro corpos [X] divergia do vanilla: três sequências encaixadas contra quatro filhos planos. A concatenação agora expande somente as sequências dos operandos imediatos. Mantém ordem, aliases, spans, filhos internos já sequenciais e wrappers opacos. Empty não vira filho; Text vazio continua filho real. Sequence de um filho conserva sua identidade na soma. `Content::sequence` público não mudou.
2. GDB encontrou SIGSEGV em `map_content_pre` durante a busca de tabelas, com 104664 bytes entre níveis consecutivos. A concatenação corrigida eliminou profundidade artificial, mas Strong64 legítimo ainda abortava. Reconstruções recursivas foram isoladas em funções genéricas estáticas FnOnce, com inline(never); a enumeração permanece exaustiva, sem vtable, thread, unsafe ou configuração nova de pilha.
3. Strong256 revelou outro aborto em `map_content_inner`, durante wrap_page_columns. O mesmo isolamento preserva a prova de mudança: None reutiliza o Arc da raiz; Some, mesmo igual, reconstrói o caminho. Source, transform_root/Located, vistas preparadas, ordem bottom-up e erros permanecem conforme os contratos existentes.

Os L0s foram atualizados antes de testes/código; houve seis REDs funcionais na concatenação e dois REDs por stack overflow nos testes isolados. Os mesmos testes passaram depois. Hashes ressellados após V15/V26 válidos. Só houve helpers internos; não mudaram campos, traits ou assinaturas Rust públicos, fases, flags ou limites de profundidade.

## Validação

Workspace: **{summary['passed']} passaram, {summary['failed']} falharam, {summary['ignored']} ignorados**. Doctests: **{docs} passaram**. Os dois testes isolados de pilha passaram com RUST_MIN_STACK=2097152; o CLI usa a pilha padrão registrada nas sondas, sem aumento. A configuração de 2 MiB existe somente no comando de teste.

| Caso CLI | Antes: exit | Debug atual | Vanilla | RSS debug atual (MiB) |
|---|---:|---:|---:|---:|
{chr(10).join(stressrows)}

Exit 134 nas execuções antigas é SIGABRT/stack overflow; não se confunde com diagnóstico de linguagem. O caso shape-for-256 usa contagens de marcadores em repr para não impor uma apresentação de linha única que o próprio vanilla rejeita. As primeiras fixtures inválidas foram preservadas como tentativas, sem crédito de paridade.

**Strong256:** o Tekt produz X; o vanilla rejeita com maximum show rule depth exceeded. Nenhum limite foi alterado para obter esse resultado. A diferença operacional está registrada e não foi contada como paridade de rejeição. Não se afirma profundidade ilimitada. A auditoria encontrou `Route` e seus guardas no código de world_types, mas seu L0 ainda incompleto; essa política precisa de auditoria própria, não de um limite novo no mapper.

Os {audit['observations']} controles A1 foram repetidos: nenhuma nova falha fora das exceções nominais anteriores. Mensagens e texto permanecem auditados separadamente do exit. Pendências anteriores não foram silenciadas.

| Gate | Exit | Comando |
|---|---:|---|
{checks}

Tekt: `{lint['before']}` → `{lint['after']}`. Nenhum erro/warning novo no diferencial por regra/consumer/mensagem, ignorando deslocamentos de linhas. O gate global continua exit 1 pelos erros V16 anteriores; não foi relaxado. Notas V19/V20 são informativas e têm política nos L0s; não houve supressão genérica.

## Build otimizado e RAM

Build release local e três controles completos passaram, sem publicar artefatos. Receita/ambiente/horários/SHA em [release-result.json.gz](release-result.json.gz).

| Caso release | Exit | RSS (MiB) | Texto |
|---|---:|---:|---|
{relrows}

Todas as execuções pesadas usaram **MemoryMax 12 GiB, MemoryHigh 8 GiB, swap zero, um job Cargo e execução sequencial**. Pico agregado da etapa: **{peak/1024**3:.3f} GiB** ({peak} bytes). Eventos acumulados: `{dict(events)}`. Os abortos RED de pilha são distintos de OOM. [resources.json.gz](resources.json.gz) conserva cada execução, inclusive falhas/tentativas.

RSS de processo e pico de cgroup de build são medidas diferentes. As amostras CLI são únicas, com perfis distintos quando comparadas ao vanilla release; não demonstram ganho universal. O caso grande de State ainda usa muito mais memória que o oráculo. Value::Array e clones de payloads merecem medição/proposta próprias; esta etapa não mudou seus contratos. Concatenação sobre Arc<[Content]> ainda copia o prefixo e não foi declarada linear.

## Pendências preservadas e próximas frentes

- Formalizar/auditar o L0 de Route/guardas em [world-types.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/world-types.md); a fonte contém contratos que a spec histórica de stubs não descreve completamente. Nenhum desses tipos/limites foi alterado.
- Reflexão de Sequence/Empty: children, fields e func ainda divergem nas sondas; são correções separadas de linguagem, não diferenças justificadas pela representação Rust.
- Morphologia de Styled observada nas sondas, e as pendências A1 de Box.body, Equation aninhada, Ref/math/StateDisplay legado no HTML e mensagens continuam abertas.
- Medir concatenação em volumes maiores e possíveis acumuladores transitórios para evitar cópia repetida do prefixo. Não trocar o carrier público ou introduzir cache global sem proposta/L0 e gate aplicável.
- Continuar os V16 restantes e os lotes do inventário de paridade. O relatório não encerra o objetivo Faça todas as correções.

Fontes principais: [join.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/operators/join.rs), [arithmetic.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/operators/arithmetic.rs), [traversal.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/content/traversal.rs). [evidence.json.gz](evidence.json.gz), [final-state.json.gz](final-state.json.gz), [lint-diff.json.gz](lint-diff.json.gz) e [manifest.sha256](manifest.sha256) preservam os registros de fechamento.
'''
text=re.sub(r'\]\(([^()]+)\)',lambda m:']('+str(p/m.group(1))+')',text);(p/'relatorio.md').write_text(text)
files=sorted(f for f in p.iterdir() if f.is_file() and (f.suffix in ['.py','.md'] or f.name.endswith('.json.gz') or f.name.endswith('.patch.gz') or f.name=='.gitignore'))
(p/'manifest.sha256').write_text('\n'.join(f'{sha(f)}  {f.name}' for f in files)+'\n')
print({'report':str(p/'relatorio.md'),'tests':summary['passed'],'doctests':docs,'peak_gib':peak/1024**3,'manifest_sha256':sha(p/'manifest.sha256')})
