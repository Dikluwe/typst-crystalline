from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,difflib,collections,re
p=Path(__file__).resolve().parent
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
load=lambda f:json.loads(gzip.decompress((p/f).read_bytes()))
summary=json.loads((p/'complete-summary.json').read_text());freeze=load('complete-freeze.json.gz');lint=load('lint-diff.json.gz');audit=load('audit.json.gz')
assert all(sha(f)==h for f,h in freeze['sources'].items()),'source changed after validation'
assert all(x['exit']==0 for x in summary['results'] if x['label']!='architecture')
geometry=load('alias-layout.json.gz')
assert all(x['accepted'] for c in geometry['cases'] for x in c['results'].values())
base=load('stage-before.json.gz');patch=[]
for f,v in base['files'].items():patch.extend(difflib.unified_diff(v.splitlines(True),Path(f).read_text().splitlines(True),fromfile='before/'+f,tofile='after/'+f))
(p/'change.patch.gz').write_bytes(gzip.compress(''.join(patch).encode()))
resources={f.name:json.loads(f.read_text()) for f in p.glob('*-resources.json')}
peak=max(int(v['memory_after']['memory.peak']) for v in resources.values());events=collections.Counter()
for v in resources.values():
 assert v['memory_after']['memory.max']==str(12*1024**3)
 assert v['memory_after']['memory.high']==str(8*1024**3)
 assert v['memory_after']['memory.swap.max']=='0'
 events.update({a:int(b) for a,b in (line.split() for line in v['memory_after']['memory.events'].splitlines())})
assert events['oom']==events['oom_kill']==0
(p/'resources.json.gz').write_bytes(gzip.compress(json.dumps(resources,indent=2).encode()))
(p/'evidence.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'freeze':freeze,'summary':summary,'logs':{f.name:f.read_text() for f in p.glob('*.log')}},indent=2).encode()))
state={'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'sources':freeze['sources'],'stage_files':{f:sha(f) for f in base['files']},'binary_sha256':summary['binary_sha256']}
(p/'final-state.json.gz').write_bytes(gzip.compress(json.dumps(state,indent=2).encode()))
docs=sum(map(int,re.findall(r'test result: ok\. (\d+) passed;', (p/'complete-doctest.log').read_text())))
count=lambda key,status:sum(x['status']==status for x in audit[key])
counts={'pdf_total':len(audit['pdf']),'pdf_accepted':count('pdf','accepted'),'primary':count('pdf','primary-error-preserved'),'morphology':count('pdf','open-morphology'),'text_total':len(audit['text']),'text_accepted':count('text','accepted'),'targets':len(audit['targets']),'target_equal':count('targets','accepted-text-and-primary-error'),'target_scaffolding':count('targets','internal-empty-footnote-leak-not-copied'),'slots':len(audit['slots']),'emission':len(audit['emission']),'footnotes':len(audit['footnotes'])}
review=Path('00_nucleo/materialization/sequence-realization-2026-10-17')
assert sha(review/'manifest.sha256')=='46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327'
for line in (review/'manifest.sha256').read_text().splitlines():
 h,f=line.split('  ',1);assert sha(review/f)==h
text=f'''# R2/R2-S — construção, seleção e realização de Sequence

Etapa encerrada com reparos e pendências discriminados abaixo. Working tree não commitada no HEAD `{state['head']}`, branch Tekt. Freeze final: **{freeze['utc']}**. Pin vanilla **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore cac836c07e4dec2a89219370428292393a3216b5; oráculo SHA-256 `{freeze['oracle_sha256']}`. Candidato debug validado: `{summary['binary_sha256']}`. Fontes, estado exato, diffstat, comandos e horários estão em complete-freeze/final-state/evidence; o patch de R2 está separado dos reparos A1/pilha/R1 anteriores. Sem commit, push, publicação ou release.

## O que foi corrigido

- `[].func()` e Sequence devolvem o construtor real: array children posicional e requerido, zero/um/vários filhos preservados, aliases e With na chamada. Casts precedem extras e os argumentos conservam seus spans. O array de entrada é emprestado para análise. Não foi criado binding global pelo nome.
- Catálogo, selector/where e matching usam a identidade tipada; homônimos e parciais continuam negativos. Sequence permanece não localizável; query/counter rejeitam seu uso. `set sequence()` vazio funciona, argumentos são rejeitados com ordem/efeitos preservados.
- A captura preserva a Sequence inteira antes do aplanamento, com regras/estilos/guards/arquivo/origem locais. Sua receita antecede filhos que pode ocultar, incluindo Table/Cell; só filhos emitidos continuam. Regras locais em codeblock e prefixos anteriores ao show mantêm o escopo.
- Alias de `it` mantém os guards daquela ocorrência; reconstrução fresca, mesmo com children iguais, volta a realizar e atinge o limite existente. O transporte usa tipos existentes e marca privada; sem Location falsa, campos públicos novos ou mudança do tamanho de Value.
- Grupos textuais pendentes do mesmo ambiente causal são coalescidos sem executar callbacks, corrigindo `AAAAB` para `AAB`. Literal, regex, interpolação, conteúdo guardado e alias em bloco foram verificados. Fronteiras de estilos/guards/regras/arquivo distintos continuam opacas.
- A origem Source é acrescentada somente quando uma captura de Sequence realmente precisa dela. Rotas sem regra pertinente e corpos de outros tipos preservam sua forma anterior, evitando carriers e alocações inúteis. Isso conserva o escopo local de heading/figure e os prefixos/caudas após 1fr; as 24 regressões detectadas pela suíte ampla foram corrigidas sem alterar seus esperados.
- Os certificados textuais passam a inspecionar Source.body, Located.body e Realized.visual por empréstimo. O alias de Sequence conserva a reserva legada do prefixo Strong em colunas, corrigindo a página adicional detectada pelo controle geométrico. As fórmulas, profundidade tipográfica e os carriers/tags emitidos permanecem; Link e conteúdo opaco continuam recusados. Dois testes funcionais RED→GREEN cobrem o certificado e a cadeia lexical visual.
- Slots Empty internos de normalização/consumo deixam de ser tratados como ocorrências da linguagem. As dez sondas distinguem BODY, [] explícito, none, construtor vazio e filhos none/[], prefixo e grupos textuais; os arrays do domínio e Text vazio são preservados.
- Erros por falta de children apontam para a chamada inteira; recursão de `[]` aponta para o ContentBlock completo. Spans individuais, hints e limites existentes permanecem.

Os L0s proprietários foram atualizados antes de cada código. R2-S foi aprovado pelo dono nesta sessão, manifesto **46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327**; approval-r2s.json.gz registra a confirmação e a revisão original foi reconferida intacta. Há três Núcleos Tekt novos com pins efetivos: captura/realização de Sequence, grupo textual causal e transparência visual de carriers preparados. Ownership/núcleos passaram antes dos ressellos; linhagem reversa final sem erros. O ajuste interno dos spans não cria API nem nova fase. A instrumentação temporária foi removida. O percurso só pula raiz crua Empty/Sequence de pacote textual; receitas que são o único item normalizado continuam visitadas, corrigindo o alias sem EOF. A expectativa Styled Y foi retificada para a fonte com newline final medida: a sonda antiga acrescentava EOF, mas o unitário o omitia. Os seis pares exatos com/sem EOF foram preservados; sem EOF, ambos imprimem AB no controle Styled, e com EOF ambos imprimem Y.

## Validação e limites das provas

**{summary['passed']} testes passaram, {summary['failed']} falharam, {summary['ignored']} ignorados; {docs} doctests passaram.** Build workspace, Clippy all-targets com warnings/expectations não cumpridas proibidos, Rustdoc com warnings proibidos, fmt, V1/V5/V15/V26, reversa e diff-check passaram. Doctests tiveram invocação separada. Os REDs reais e os GREENs são preservados: construtor/seleção/show, aplicação textual duplicada e os dois spans. Tentativas de harness/fixtures incorretas não contam como RED funcional; o teste de texto vazio usa text("") e o de efeitos ilegais verifica o erro primário, com reconstrução fresca testada separadamente.

Corpus PDF: **{counts['pdf_total']} casos**; {counts['pdf_accepted']} aceitos por valores/texto/status/erro primário, {counts['primary']} com erro primário preservado onde o vanilla mascara a causa, {counts['morphology']} ainda divergentes de morfologia. Sondas textuais adicionais: {counts['text_accepted']}/{counts['text_total']} aceitas, com precedência local ainda aberta. Controles de target: **{counts['targets']} comparações bilaterais**, distribuídas entre PDF e HTML: {counts['target_equal']} coincidem por texto/erro primário e {counts['target_scaffolding']} expõem o vazio interno de notas do vanilla descrito abaixo. HTML usa feature explícita. Mais {counts['slots']} sondas de slots coincidem, {counts['emission']} sondas de emissão coincidem, inclusive o replay escalar nativo anterior agora corrigido e {counts['footnotes']} controles diferenciam body HTML automático e explícito. Três prefixos (simples, strong e Styled) também foram comparados com/sem show sequence identidade, dentro de cada compilador, por páginas/palavras e coordenadas PDF; os deltas e tolerância estão em alias-layout.json.gz. Esse controle não certifica igualdade geométrica entre os dois compiladores. As fontes incluem construtor guardado e inspecionado sem executar seu show, ordem do primeiro erro de filhos emitidos, filhos ocultos e regras geradas de outras famílias. Extração normalizada de texto não prova igualdade geométrica, cor, bytes PDF ou identidade de Symbol; nenhuma dessas equivalências é declarada.

RAM: MemoryMax **12 GiB**, High **8 GiB**, swap máximo **zero**, um job Cargo, debug0/incremental0 e jobs pesados sequenciais. Pico de cgroup da etapa **{peak/1024**3:.3f} GiB** ({peak} bytes); eventos acumulados `{dict(events)}`. A receita e cada janela estão em resources. Não se infere ganho de RSS do produto comparando builds/cache diferentes.

## Divergências que fazem sentido manter no Tekt

As sondas show-once, stored-value, show-panic, show-block-bad-mutation e other-parent-hides-sequence preservam no Tekt `panicked with: ...` ou a proibição de mutar variável externa, com origem. O vanilla mascara esses erros por `maximum show rule depth exceeded`. Evidência de fonte: `lab/typst-original/crates/typst-realize/src/lib.rs:394–403` atrasa o erro e substitui conteúdo por default; `typst-library/src/engine.rs:42–49` acumula o erro e retorna Default; `foundations/content/mod.rs:94–96` define default como Sequence vazia, que volta a casar com a regra. Não copiar esse defeito. A reconstrução fresca legítima continua rejeitada com o limite de show nos dois compiladores. Isso não transforma diferenças de morfologia em escolhas arquiteturais aceitas.

Também se mantém a ausência de texto artificial no HTML: quatro controles de show imprimem Y (ou LOCAL) no Tekt contra Y Y (ou LOCAL LOCAL) no vanilla. `lab/typst-original/crates/typst-html/src/document.rs:293–302` sempre realiza FootnoteContainer no body automático; `rules.rs:382–387` declara que não deve criar contêiner sem notas, mas retorna Content::empty. O visit de realize aplica show antes de expandir Sequence. O controle children:() produz BODY no Tekt contra BODY FOOTER no vanilla HTML; html.body explícito produz BODY em ambos, sem esse contêiner automático. Esta classificação é inferência de vazamento de scaffolding apoiada pela fonte e pelo controle, não alegação de que os targets devam ter saída idêntica. Seria refutada se o acréscimo viesse do autor ou persistisse no body explícito. O defeito Tekt independente EBODY dos slots foi corrigido, não aceito como divergência. Não copiar o vazamento de render; [] escrito pelo autor continua selecionável.

## Pendências de paridade e CI

1. **R3-Symbol:** o cast atual ainda produz Text; func retorna text em vez de symbol e igualdade com texto comum retorna true em vez de false. A aceitação visual de alpha não certifica morfologia. R2-S não autoriza adicionar variante pública; exige proposta/gate próprio se esse for o reparo.
2. **R3-Markup:** sem regra Sequence, o ContentBlock `[#([A]+[B]) C]` ainda aplana o filho Sequence. Deve preservar o filho imediato como no vanilla.
3. **R3-Textual:** regras literais locais distintas ainda resultam C em vez de B. O owner matching tem ordem de precedência própria a corrigir; o coalescimento deste pacote não altera essa política. O controle genérico #text("A") sob A→AA foi corrigido para AA em R2-S13. O replay anterior AAAA também foi reproduzido no release anterior da etapa de pilha (SHA 2ab1495d555ff4d61e67eb14bef67597f7f668a0e3bce3a8129623011443441c, freeze em state-stack-2026-10-17/final-state.json.gz); não é regressão criada por Sequence. Emissão de [A] e text("A") pela regra Sequence produz AA nos dois compiladores. As coletas native-before/emission-before/emission-after preservam a distinção e a correção. Receitas com callbacks de elemento aplicáveis conservam sua barreira; a otimização não certifica todos os callbacks possíveis.
4. **R3-Diagnostics:** rastros genéricos de show em filhos emitidos ainda diferem; mensagens, spans primários e hints são auditados separadamente. Os seletores lógicos OR com regex são rejeitados pelos dois compiladores, mas a mensagem ainda diverge (selector não suportado em show rule / this selector cannot be used with show), conforme logical-before/after.json.gz. OR só com elementos coincide. Casos de rastro observados: {', '.join(x['label'] for x in audit['diagnostic_details']) or 'nenhum neste corpus'}. As diferenças completas ficam no audit.json.gz.
5. O gate Tekt global continua bloqueado pelos **{lint['after'].get('V16:error','?')} V16** anteriores. SARIF final `{lint['after']}`; o diferencial por regra/nível/consumer/mensagem não adicionou erros ou warnings. Notas novas V19/V20 e as removidas estão em lint-diff.json.gz; não houve supressão. Os agrupamentos Empty/Sequence compartilham a mesma identidade/schema/transporte, e o complemento de casts rejeita os mesmos tipos, conforme seus L0s; notas não provam testar todas as alternativas.

A cláusula de divisão incompleta e os nomes dos pacotes R3 constam nos L0s. O objetivo geral de paridade e os demais lotes continuam abertos. O controle adicional styled-children em original-before/original-after.json.gz também expõem diferença de repr do filho Styled; pertence ao futuro R3-Reflection e não é prova de igualdade morfológica desse carrier. Este relatório fecha a execução e validação de R2/R2-S com essas limitações explícitas; não declara paridade universal nem lint global verde.

Evidências principais: before/expanded/controls/r2s-scope/text-before/after/text-after/target-controls/audit.json.gz, stage-before.json.gz, complete-freeze.json.gz, change.patch.gz, evidence.json.gz, resources.json.gz, lint-diff.json.gz, final-state.json.gz e manifest.sha256.
'''
(p/'relatorio.md').write_text(text)
files=sorted(f for f in p.iterdir() if f.is_file() and (f.suffix in ['.py','.md'] or f.name.endswith('.json.gz') or f.name.endswith('.patch.gz') or f.name=='.gitignore'))
(p/'manifest.sha256').write_text('\n'.join(f'{sha(f)}  {f.name}' for f in files)+'\n')
print(json.dumps({'report':str(p/'relatorio.md'),'tests':summary['passed'],'doctests':docs,'peak_gib':peak/1024**3,'manifest_sha256':sha(p/'manifest.sha256'),'corpus':counts}))
