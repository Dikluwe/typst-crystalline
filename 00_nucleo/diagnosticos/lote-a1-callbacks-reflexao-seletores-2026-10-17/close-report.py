from pathlib import Path
import json,gzip,hashlib,subprocess,re,datetime,collections
p=Path(__file__).resolve().parent;root=p.parents[2];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 h=hashlib.sha256()
 with Path(f).open('rb') as s:
  for b in iter(lambda:s.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def read(n):return json.loads(gzip.decompress((p/(n+'.json.gz')).read_bytes()))
def save(n,d):(p/(n+'.json.gz')).write_bytes(gzip.compress(json.dumps(d,indent=2,ensure_ascii=False).encode()))
summary=json.loads((p/'complete-summary.json').read_text());assert summary['sources_unchanged'];assert all(r['exit']==0 for r in summary['results'] if r['label']!='architecture')
freeze=read('complete-freeze');assert all(sha(root/f)==h for f,h in freeze['sources'].items())
audit=read('probe-audit');assert not audit['unexpected'];lint=read('lint-diff');assert not [d for d in lint['added'] if d['level'] in ['error','warning']]
resources={f.name:json.loads(f.read_text()) for f in p.glob('*-resources.json')};peak=max(int(d['memory_after']['memory.peak']) for d in resources.values())
assert all(d['memory_after']['memory.max']==str(12*1024**3) and d['memory_after']['memory.swap.max']=='0' for d in resources.values())
assert all(not re.search(r'^(high|max|oom|oom_kill|oom_group_kill) [1-9]',d['memory_after']['memory.events'],re.M) for d in resources.values())
save('resources',{'utc':now(),'peak_bytes':peak,'jobs':resources})
files=set(subprocess.check_output(['git','diff','HEAD','--name-only'],text=True).splitlines())|set(subprocess.check_output(['git','ls-files','--others','--exclude-standard'],text=True).splitlines())
protected={f:sha(root/f) for f in sorted(files) if f.startswith(('01_core/','02_shell/','03_infra/','04_wiring/','benches/','00_nucleo/prompts/')) and (root/f).is_file()}
owners=[]
for f,h in protected.items():
 if f.endswith('.rs'):
  s=(root/f).read_text();m=re.search(r'@prompt ([^\n]+)',s);owners.append({'consumer':f,'source_sha256':h,'prompt':m.group(1) if m else None})
state={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'linter_sha256':sha('/tmp/tekt-linter-none-target/debug/crystalline-lint'),'working_tree':'não commitado','status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'protected':protected,'owners':owners,'freeze':freeze,'candidate_sha256':sha(root/'target/debug/typst'),'baseline_sha256':sha(p/'outputs/typst-before'),'oracle_sha256':sha(root/'lab/typst-original/target/release/typst')};assert state['candidate_sha256']==summary['binary_sha256'];save('final-state',state)
logs={f.name:f.read_text() for f in p.iterdir() if f.is_file() and f.suffix in ['.log','.json']};save('evidence',{'utc':now(),'files':logs})
def totals(name):
 matches=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (p/name).read_text(),re.M)
 return tuple(sum(int(x[i]) for x in matches) for i in [1,2,3])
test=totals('complete-workspace.log');doc=totals('complete-doctest.log');assert not test[1] and not doc[1]
mem=read('runtime-memory');memrows=[]
for case in dict.fromkeys(r['case'] for r in mem['rows']):
 rs={r['backend']:r for r in mem['rows'] if r['case']==case};memrows.append('| '+case+' | '+' | '.join(f"{rs[k]['maximum_rss_kib']/1024:.2f}" for k in ['tekt-before','tekt-after','vanilla'])+' |')
msg='\n'.join(f"| {r['group']}/{r['label']} | {r['target']} | {r['tekt_message'].replace('|','/')} | {r['vanilla_message'].replace('|','/')} |" for r in audit['message_differences'])
notes='\n'.join(f"- {d['rule']} ({d['count']}): `{d['file']}` — {d['message']}" for d in lint['added'])
checks='\n'.join(f"| {r['label']} | {r['exit']} | `{ ' '.join(r['command']) }` |" for r in summary['results'])
report=f'''# Lote A1 — callbacks, reflexão e seletores

Etapa entregue no recorte validado. **State por leitor, suplementos de Heading e admissão nominal de seletores corrigidos.** Há pendências identificadas abaixo; A1 não é uma declaração de paridade global. O gate global Tekt continua aberto pelos V16 anteriores. Sem commit, push, merge ou release nesta etapa.

## Proveniência

- HEAD `{state['head']}`, branch Tekt, **working tree não commitado**. Congelamento `{freeze['utc']}`; entrega `{state['utc']}`. A lista exata de fontes, arquivos novos, status e `git diff HEAD --stat` está em [final-state.json.gz](final-state.json.gz), junto com o congelamento verificado. Fontes produtivas e L0 não mudaram durante a rodada final.
- Vanilla pin **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore `cac836c07e4dec2a89219370428292393a3216b5`, binário `{state['oracle_sha256']}`. Não se usa versão textual como prova.
- Tekt anterior arquivado `{state['baseline_sha256']}`; candidato `{state['candidate_sha256']}`. O oráculo e o anterior permaneceram intactos. Scripts, fontes de sondas, argv, UTC, saídas e logs estão nos artefatos desta pasta e no manifesto.
- Aprovações: A1-S2 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`; A1-H2 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`. Revisões preservadas em materialization. **A1-S1 foi retirada:** não se adicionou init a StateUpdateElem/ElementPayload. A contraprova do vanilla exige init no leitor.

## O que mudou

1. **State:** `state(key)` usa init none. `.update(func)` registra transformação, sem executá-la na construção ou walk. Cada objeto leitor conserva seu init, mesmo com a mesma chave; get/at/final resolvem as ações na ordem causal. Set/Func mistos, reuso de conteúdo, descarte e chaves independentes têm controles. Uma chave não lida não executa callbacks. A sequência demandada inteira pode propagar erro posterior, como no vanilla, embora o valor selecionado seja o do prefixo. Contexto do leitor não é herdado pela callback e é restaurado no sucesso/erro.
2. **Heading:** suplemento é realizado antes de query/context em PDF e HTML, com captura léxica e estilos causais. Erros propagam. A ocorrência conserva identidade Heading e Location; apenas sua pendência privada é consumida, sem repetir callback na mesma entrada. Defaults/auto/none, numeração e escopo local têm controles. Styled ordinário permanece opaco. Não se anteciparam callbacks de Counter, State ou equações.
3. **Reflexão:** campos de texto, amount/weak de H/V, dimensões de Box/Block e body de Block usam valores da linguagem e presença prescrita. O transporte específico de Heading preserva func, supplement/numbering explícitos e ordem dos campos. `assert.eq` passou a reutilizar a igualdade de linguagem existente, necessária para comprimentos relativos e valores numéricos aninhados; não redefiniu a igualdade Rust geral.
4. **Seletores:** show de text/bibliography, selector de funções nativas de elementos e counter de equation reconhecem identidade real do catálogo. Aliases são aceitos; closures, funções parciais e homônimos que não são elementos são rejeitados. A extensão de contadores textuais e os nomes canônicos Footnote/Equation foram preservados.
5. **HTML:** a projeção visual descarta apenas marcadores invisíveis State/StateUpdate/Metadata e preserva o body de Label. Não há avaliação de callbacks no encoder. Conteúdo ainda não suportado continua produzindo erro.

Os L0 proprietários foram editados antes do código. Novos helpers têm owners exclusivos; V15/V26 foram validados antes do resselo. [public-contracts.json.gz](public-contracts.json.gz) registra as cinco assinaturas State migradas e a fachada Heading aprovadas. A nova representação do snapshot é privada.

## O que faz sentido manter no Tekt

- Despacho estático e separação entre índice semântico, render e composição dos targets; nenhuma necessidade de copiar o algoritmo ou o vetor de estados intermediários do vanilla.
- Payloads Set compartilhados por Arc entre histórico, ações e clones de snapshots. O resolvedor usa empréstimos quando possível e mantém somente acumulador/resultado selecionado. Styles pendentes de Heading ficam fora do Content canônico, no cabeçalho privado do snapshot.
- APIs legadas `state_update`, `state_update_with`, `state_final`, `state_at`, `state_display`, a extensão `.display` e o helper explícito apply_state_funcs conservam seus contratos declarados. Seus controles são **compatibilidade Tekt**, não crédito de paridade com o vanilla.
- Notas V19/V20 devem ser avaliadas pela coesão e exaustividade, não removidas em massa. Os novos grupos/fluxos têm prescrição no L0; não foram adicionadas supressões genéricas.

## Validação final

**Workspace: {test[0]} testes passaram, {test[1]} falharam, {test[2]} ignorados. Doctests: {doc[0]} passaram, {doc[1]} falharam, {doc[2]} ignorados.** Os ignorados não foram introduzidos por esta etapa. A suíte não foi enfraquecida para esconder erros: o teste histórico do fixpoint que esperava callback sem leitura foi migrado para demandar a chave e preservar a mesma asserção de propagação; o caso sem leitura tem controle separado.

{audit['observations']} observações bilaterais finais nos arquivos main/extra/state/targets, incluindo {audit['extensions']} de extensões Tekt. São controles delimitados, alguns repetem a mesma fonte; não representam percentual de cobertura da linguagem. A auditoria automática de status não encontrou resultado inesperado fora das {audit['pending']} exceções nomeadas. Isso não fecha as diferenças de texto e diagnóstico registradas separadamente.

| Gate | Exit | Comando |
|---|---:|---|
{checks}

Tekt antes: `{lint['before']}`. Depois: `{lint['after']}`. O diferencial compara regra/nível/consumer/mensagem, desconsiderando deslocamentos de linha; **nenhum erro/warning novo**. V1/V5/V15/V26 estritos e linhagem reversa passaram. O exit global não foi relaxado.

Notas adicionadas no diferencial:

{notes or '- Nenhuma.'}

### Rustdoc

A primeira execução combinada da etapa apresentou E0463 em doctests. O teste p1295 invoca Cargo/Rustdoc JSON recursivamente com RUSTC_BOOTSTRAP e target compartilhado. A hipótese de invalidação de artefatos é compatível com os registros; não foi provada por isolamento causal completo. A hipótese de TMPDIR foi refutada pelos controles normal/dev-shm. A receita validada executa testes comuns e, depois, doctests em outra invocação Cargo, sem excluir o teste p1295 ou os doctests. Não se declara correção geral do problema de concorrência/artefatos do Cargo.

## RAM

Teto por execução pesada **12 GiB**, MemoryHigh **8 GiB**, swap **0**, Cargo **1 job**, tarefas pesadas sequenciais. DEBUG=0 em dev/test e incremental desativado nesta validação. Maior pico registrado nesta etapa, incluindo tentativas RED/falhas e validações intermediárias: **{peak/1024**3:.3f} GiB** ({peak} bytes). Eventos high/max/OOM/OOM-kill: zero. [resources.json.gz](resources.json.gz) conserva cada cgroup, comando, horário, HEAD/diffstat e pico.

`Value` permaneceu **144 bytes/alinhamento 16**; `IntrospectedContent`, **128/alinhamento 8**, antes/depois no host x86_64. Isso não significa ausência de custo no heap: o cabeçalho privado do mapa inclui os styles pendentes.

Máximo RSS de processos CLI, em MiB, uma amostra por caso/binário:

| Documento | Tekt anterior | Candidato | Vanilla |
|---|---:|---:|---:|
{chr(10).join(memrows)}

Os documentos acima terminaram com o mesmo texto extraído e sucesso nos três binários. Anterior debug com símbolos, candidato debug sem símbolos e vanilla release: perfis diferentes e amostra única impedem atribuir a diferença inteira ao patch ou declarar ganho geral. O caso state-set-heavy mede explicitamente o caminho com muitos payloads Set; os arquivos preservam fontes, hashes, argv e saída. Pico de cgroup de compilação e RSS de um documento são medidas distintas.

## Pendências e ordem sugerida

1. **Pilha no ensaio maior de State:** 64 updates com arrays de 2048 valores abortaram por stack overflow nos dois binários Tekt debug medidos; vanilla release passou. O caso menor da tabela não fecha esse limite. Evidência completa em pre-repair-runtime-memory.json.gz; release do Tekt não foi medido nesse ensaio. Não houve evento de OOM.
2. **Presença de Box.body:** o carrier atual não distingue corpo omitido de vazio explícito. box().fields() continua correto, mas box[A].body/fields ainda divergem. Exige proposta de transporte/contrato própria, sem inventar presença.
3. **Equation aninhada:** o caso math.equation(block:true)[$x$] duplica ocorrências no snapshot. Queries de equações diretas e contador paginado passam; isso não fecha normalização/morfologia de equações aninhadas.
4. **HTML:** Ref/âncoras e matemática continuam sem implementação suficiente. As sondas combinadas com esses nós falham mesmo com o suplemento/consulta corretos; os controles isolados de Heading e State passam. state_display legado continua pendente no HTML; sua compatibilidade PDF foi preservada.
5. **Mensagens e apresentação:** as rejeições abaixo conservam texto distinto do vanilla. Não são certificadas como igualdade de diagnóstico. O caso metadata entre espaços também conserva diferença de extração PDF já presente no anterior; HTML desse controle passou. O controle de counter-footnote conserva a apresentação anterior [1] em vez de 1; a chave do contador foi validada separadamente, sem certificar esse formato de nota.
6. **Demais lotes e arquitetura:** igualdade semântica de suplementos/Styled e demais A2/A3/A4 do inventário anterior, além dos V16 restantes, continuam abertos. Nenhuma dessas dívidas foi silenciada ou tratada como escolha Tekt correta sem nova medição.

| Controle | Target | Mensagem Tekt | Mensagem vanilla |
|---|---|---|---|
{msg}

[probe-audit.json.gz](probe-audit.json.gz) enumera cada controle, exceção, diferença de mensagem/texto e status anterior. [lint-diff.json.gz](lint-diff.json.gz), [evidence.json.gz](evidence.json.gz), [final-state.json.gz](final-state.json.gz) e [manifest.sha256](manifest.sha256) permitem conferir o fechamento. O objetivo amplo do projeto permanece aberto.
'''
report=re.sub(r'\]\(([^()]+)\)',lambda m:']('+str(p/m.group(1))+')',report)
(p/'relatorio.md').write_text(report)
manifest=[]
for f in sorted(p.iterdir()):
 if f.is_file() and (f.suffix in ['.py','.md'] or f.name.endswith('.json.gz') or f.name=='.gitignore'):
  manifest.append(f'{sha(f)}  {f.name}')
(p/'manifest.sha256').write_text('\n'.join(manifest)+'\n')
print(json.dumps({'workspace':test,'doctest':doc,'observations':audit['observations'],'pending':audit['pending'],'peak_gib':peak/1024**3,'report':str(p/'relatorio.md'),'manifest_sha256':sha(p/'manifest.sha256')},indent=2))
