from pathlib import Path
import json,hashlib,datetime,subprocess
r=Path.cwd();p=Path(__file__).resolve().parent
audit=json.loads((p/'owner-audit-before.json').read_text());owners={x['owner']:x for x in audit['owners']}
selection=r/'00_nucleo/prompts/_nuclei/paragraph-selection.toml';s=selection.read_text();old=next(x for x in s.splitlines() if x.startswith('statement = "A seleção global certifica'))
new='statement = "A seleção global certifica texto latino sob largura finita de região constante fora de colunas, células e floats; a forma pode distinguir largura inicial e continuada para hanging-indent. RTL latino pode integrar esse caminho apenas sem BidiCursor ativo ou tracking, preservando None sem prova de identidade lógica; demais casos bidi mantêm o caminho existente. Tracking uniforme em conteúdo textual sem mudança de escopo pode ser admitido somente quando seus intervalos entre unidades são medidos e emitidos uma vez e as larguras efetivas permanecem finitas e não negativas; tracking misto permanece no percurso incremental; a combinação com hifenização uniforme só é certificada quando o tracking resolvido é zero; valores não nulos requerem informação de shaping do corte e conservam fallback. Hifenização lexical pode integrar candidatos da preparação quando a palavra possui estilo e política uniformes. Matemática, smallcaps e conteúdo recusado conservam o percurso incremental; isto nunca prova optimized completo."'
s=s.replace(old,new);selection.write_text(s)
n=r/'00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml'
claims=[
('length-and-shape','must','hanging-indent conserva Length lexical inclusive zero, negativo e abs+em. Resolver pelo tamanho compartilhado do parágrafo, não pelo primeiro ou último run. A primeira linha mantém a origem normal; continuações recuam na borda inicial da direção e usam largura reduzida pelo mesmo comprimento. Nova região continua o mesmo parágrafo.'),
('causal-boundary','must','A presença da declaração interrompe o parágrafo antes e depois do escopo, inclusive zero, repetição do mesmo valor e corpo vazio. O transporte privado de ocorrência pode diferir entre frames de StyleChain para evitar que diff_styles apague declarações iguais. O valor não é uma opção pública nem substitui o comprimento; não reavaliar argumentos ou callbacks.'),
('paragraph-vs-fragment','must','Fragmento inline de container não aplica hanging-indent genérico. Par explícito, Parbreak ou estrutura de fluxo que exige parágrafos ativa o contexto de parágrafo. Títulos e corpos simples de raw/lista/quote não herdam o recuo genérico. Subframes isolam o contexto, checkpoint e rollback o conservam.'),
('one-horizontal-authority','must','Seleção simples, optimized, hifenização, emissão, wrap incremental, justificação e alinhamento consultam a mesma forma de linha. A aplicação ao cursor ocorre uma vez por linha e não muda largura ou line_start_x globais para fingir o recuo. Reabrir admissão horizontal após mudança de região sem reiniciar o contador de linha.'),
('bibliography-default','must','Somente o plano Hanging aplica o default de 1.5em da bibliografia. Set par ambiente não substitui esse default; show-set realmente casado pode substituir por Length inclusive zero. Columns e Plain conservam a geometria própria, sem receber recuo genérico acidental.'),
('bibliography-first-line','must','Uma entrada Hanging usa inset inicial h e compensação privada de primeira linha -h, preservando corpos estruturados e continuação. Capturar a compensação na primeira unidade material, conservar a forma inicial em avanço antes do commit e em rollback. Não fabricar texto, Location, tags de ocorrência ou HSpace que force fallback greedy.'),
('progress-and-identity','must','Valores extremos mantêm progresso e todo o texto lógico, mesmo quando a tinta sai da página e o extrator PDF recorta tokens. Não saturar o recuo ao intervalo da página, apagar palavras ou certificar cópia por pdftotext recortado. BidiSpan permanece exato ou None conforme o núcleo de origem; não inferir identidade por igualdade de strings.'),
('limits','must-not','Esta alteração não certifica métricas mistas, toda seleção Unicode/RTL/objetos, reflexões públicas incompletas de Par, múltiplas bibliografias, clusters adjacentes ou a realização R3-T1 ainda não aprovada. Não alterar APIs públicas, ordem de realização, flags ou defaults externos para contornar essas pendências.')]
s='tekt = 1\nkind = "nucleus"\nid = "paragraph-hanging-indent"\ntitle = "Recuo francês e origem causal da primeira linha"\n'
for key,level,text in claims:s+='\n[[claims]]\nid = '+json.dumps(key)+'\nlevel = '+json.dumps(level)+'\nstatement = '+json.dumps(text,ensure_ascii=False)+'\n'
n.write_text(s)
def nh(f):return hashlib.sha256(f.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
pins={str(f.relative_to(r)):nh(f) for f in [selection,n]}
evidence='''
## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-hanging-2026-10-18/before-sources.json.gz
preserva HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, árvore não commitada,
status/diffstat/UTC e2099 fontes/L0s. before.json.gz contém92 pares;
extra-before.json.gz contém26; boundary-before.json.gz contém10 e
repeat-before.json.gz contém4. Oracle bd5ebec285505889e8ff1d4f68804ab147a699f3,
binário9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt inicial2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095.
Em160pt/margem8/font12, hanging10 mantém a primeira linha x8 mas move
continuações x8→18; negativo-5 exige x3. RTL mantém primeira borda direita152
mas continuações vão para142. O Tekt ignora o comprimento. MLA tem default
18pt (1.5em), set ambiente3em não o troca, show-set3em exige36pt; IEEE/Plain
não adquirem recuo de hanging por esse show-set. bibliography.where(style:"mla")
não casa na sonda vanilla: conservar a decisão do seletor, não forçar override.

Block/Box com body apenas inline ignoram hanging10; Par explícito dentro
deles e Block com Parbreak aplicam. Heading/Raw/List/Quote simples não o
aplicam. Declaração parcial, zero, repetida e escopo vazio fecham fronteiras:
FIRST/MIDDLE/LAST em5.168/27.464/49.76, contra todos5.168 no Tekt. A primeira
sonda lexical usou colchetes literais: foi refutada, corrigida para #[...] e
preservada como extra-before-initial; não é evidência de escopo Styled.
Valores extremos geram tinta recortada pelo extrator: não equiparar ausência
em pdftotext a apagamento de Content. Fonte parcial24pt não muda o em do
comprimento compartilhado12pt, como partial-font registra.

Fontes pinadas: typst-realize/src/lib.rs:1061–1085,1190–1219 distinguem
fragmento/Par e interrupção; typst-layout/src/flow/collect.rs:72–134 escolhe
fluxo, inline/mod.rs:174,216–221 e collect.rs:141–147 resolvem forma;
inline/line.rs:496–503 posiciona; rules.rs:510–523 faz inset/compensação CSL.
typst-library/src/model/bibliography.rs:112–127,367 determina o default e
precedência. Isso mede língua/geometria; não exige a estrutura Rust vanilla
nem atribui intenção histórica. Refutação por fronteira perdida, em de run,
recuo duplo, reset em página, fallback greedy imposto, cópia perdida, callbacks
reexecutados ou regressões dos controles impede fechamento.

### Classificação e limites

Correção interna/de paridade ADR-0127, fluxo contínuo: sem campo ou assinatura
pública, flag/default externo novo, fase ou ordem de realização diferente.
A nova forma inicial/continuação supersede somente a limitação correspondente
de largura constante; RTL latino certificado segue as condições do núcleo,
sem BidiCursor ativo/tracking e sem inventar BidiSpan. Os demais limites
anteriores continuam. Os valores/cópia fora da página requerem inspeção dos
frames ou SVG, não igualdade de extração recortada. Não antecipar R3-T1.
'''
responsibilities={
'compiler/layout.md':'Declarar o descendente paragraph_indent e guardar State e Option<Spec> pending_bib_escape privados, inicializados como Paragraph/sem ocorrência. Dispatcher continua estático e magro: apenas delegar hooks before_content/after_content; toda classificação e aplicação pertencem ao novo owner. Interfaces públicas permanecem.',
'compiler/layout/paragraph.md':'Após fechar fronteira anterior, instalar contexto Paragraph no corpo de Par e preparar seu escopo completo; fechar/restaurar contexto ao sair, inclusive vazios. Não mudar fórmula de espaçamento/altura nem executar body vazio novamente.',
'compiler/layout/paragraph_plan.md':'Conservar certificação inteira e recusas. Preparar a forma de escopo depois da entrada lexical válida; passar Shape inicial/continuação a seletores sem hifenização ou com candidatos. Facades uniformes continuam usadas quando a forma é uniforme. Não trocar estratégia por hanging nem emitir conjunto parcialmente recusado.',
'compiler/layout/paragraph_select.md':'Adicionar seletor privado com Shape(first, continuation), preservando fachada select com largura uniforme. Simple usa a largura da origem real do range; DP aplica largura inicial só ao predecessor0 e a continuada nos demais, inclusive poda/custo. Preservar custos, tie-break e progresso overlong.',
'compiler/layout/paragraph_hyphen.md':'Oferecer fachada privada por Shape além da uniforme. Usar largura inicial apenas no ponto origem(0,0), continuada nos demais candidatos, reavaliando-a depois de greedy avançar. Preservar penalidades, ranges UTF8, SHY, margem óptica e materialização escolhida.',
'compiler/layout/paragraph_prepare.md':'Conservar whitelist e rejeição de par.* em Styled parcial; a fronteira é tratada pelo owner de escopo. Admitir RTL latino somente sem BidiCursor ativo e sem tracking, com certificação textual atual e direção uniforme. Não consumir/sintetizar BidiSpan, tocar nas demais recusas ou forçar global para outros scripts.',
'compiler/layout/paragraph_emit.md':'Preparar contexto da linha antes de admitir/emiti-la; ajuste usa a largura efetiva inicial/continuada da mesma forma, mantendo carrier, última linha natural e admissão regional. Continuar sem selecionar de novo ou executar eval. Estado de linha do novo owner recebe commit pelo LineCommit.',
'compiler/layout/sequence.md':'Preparar escopo compartilhado completo antes de tentar seleção ou percorrer filhos; contexto já ativo é herdado por sequências parciais. Fronteiras explícitas reabrem escopo pelo canal existente, sem modificar preflight fracionário/bidi ou deduzir escopo por consenso de runs.',
'compiler/layout/text.md':'No caminho de Text, inicializar escopo ainda ausente pela chain atual antes de planejar ou emitir; escopo fornecido pelo conjunto pai tem prioridade. Preservar texto vazio, origem lógica e seleção existente.',
'compiler/layout/text_baseline.md':'Depois da admissão vertical, aplicar a origem horizontal pendente uma vez; alinhar separadores literais anteriores pelo mesmo delta, sem criar demanda vertical por eles. Nova região reabre aplicação sem reiniciar a linha lógica.',
'compiler/layout/word_layout.md':'Word/chunk usam limites inicial/final efetivos do novo owner em should_wrap/decisão, preservando progresso em origem recuada, inclusive negativa/extrema. prepared recebe admissão horizontal pela mesma autoridade após baseline. Não alterar widths dos glyphs, carriers ou políticas.',
'compiler/layout/space.md':'Manter regras de origem/duplicação e avanço; ao verificar overflow, usar a borda final efetiva do parágrafo. Não fabricar espaço para representar recuo.',
'compiler/layout/cursor.md':'Alinhamento horizontal da linha usa a borda final efetiva, conservando direção comum, prefixos literais e tracking. Não mudar regiões/line_start_x para simular hanging ou alterar admissão vertical.',
'compiler/layout/line_commit.md':'Expansão fr/repeat e justificação usam a borda final efetiva; numeração preserva margem exterior do parágrafo. Só após drenar uma linha causal, informar commit ao State, reabrindo aplicação horizontal para a próxima linha. Preservar métricas verticais, cauda fracionária e demais protocolos.',
'compiler/layout/parbreak.md':'Após fechamento causal existente, terminar o contexto de hanging; fazer a limpeza também quando a fronteira não tinha tinta, sem criar linha/altura. Fila de compensação CSL ainda não admitida permanece pendente.',
'compiler/layout/checkpoint.md':'Capturar/restaurar State e fila pending_bib_escape juntamente com os campos existentes, em todos os caminhos; não recalcular em replay nem reexecutar body.',
'compiler/layout/sub_frame.md':'Isolar State e fila pendente da região exterior. Contexto local segue fragmento inline versus Par explícito/fluxo com fronteira do body; não consumir a compensação CSL do pai dentro do frame auxiliar. Restaurar em LIFO junto das coordenadas e contexto existente, inclusive inline e isolamento de callbacks.',
'compiler/layout/block.md':'Depois de fechar o texto exterior e antes do corpo, encerrar contexto anterior e instalar contexto local do fragmento (inline puro versus fluxo com Par/Parbreak/estrutura). Restaurar depois do fechamento interno. Inset/continuação e margens existentes permanecem; não aplicar hanging ambiente a body inline. Compensação CSL é privada e pode fornecer forma inicial do body.',
'compiler/layout/boxed.md':'Preservar admissão/layout de Box; verificações horizontais do slot exterior usam os limites efetivos da linha atual, sem propagar recuo genérico para body inline isolado. Subframe governa o contexto local; Box já presente na linha recebe somente a aplicação exterior pertinente.',
'compiler/layout/bibliography.md':'Separar Hanging de Plain. Hanging resolve h do override privado realmente marcado por show-set ou default1.5em, preserva Length até tamanho corrente, usa inset inicial h e fila privada de compensação -h na primeira linha do Block. Não usar HSpace/fake Par nem tags/Location novos. Shadow par.hanging-indent com zero somente no corpo técnico, para evitar dupla aplicação ambiente; Columns/Plain conservam geometria, cache público e fallback. Restaurar chain/fila/contexto após cada entrada e antes de OUTSIDE.',
'compiler/layout/heading.md':'Isolar o corpo como fragmento inline para hanging genérico, após a fronteira já fechada; restaurar contexto após flush. Não modificar margens recém-corrigidas, escala, numeração, style bridge ou ordem de realização.',
'compiler/layout/title.md':'Isolar o corpo como fragmento inline para hanging, depois do fechamento anterior existente; restaurar contexto após flush. Manter inicialização de TextStyle e escala originais.',
'compiler/layout/list_item.md':'Instalar contexto local do body depois da admissão/marcador existentes, distinguindo inline puro de múltiplos parágrafos. O recuo próprio marker/body não se soma ao hanging genérico num body inline. Restaurar após flush e manter numeração/loose/grouping.',
'compiler/layout/enum_item.md':'Aplicar a mesma separação de contexto ao body de enum sem alterar contagem, marker/body-indent, loose ou resets. Corpo inline não recebe hanging genérico; Par explícito/fluxo segue o contexto local.',
'compiler/layout/quote.md':'Somente no modo block, isolar o contexto do body conforme fragmento/Par e restaurar depois do fechamento. Quote inline herda o parágrafo externo. Não certificar a geometria de quote inteira nem alterar aspas/atribuição.',
'compiler/layout/raw_highlight.md':'begin_block fecha o fluxo anterior e retorna um State privado de restauração, instalando fragmento inline para tokens; end_block mantém seu protocolo de margem. layout e o caller style_scope restauram o State retornado. Raw inline permanece no contexto exterior; não mudar tokenização ou corrigir whitespace nesta alteração.',
'compiler/layout/style_scope.md':'Presença de canal par.hanging-indent ou de seu marcador privado de ocorrência fecha fronteiras antes do push e depois do body, mesmo valor0/repetido/corpo vazio. Usar fechamento causal existente e limpar contexto, sem tinta/altura fictícia e sem executar body novamente. Preservar raw begin/end e capturar/restaurar o novo State privado retornado por begin_block.',
'compiler/layout/region_progress.md':'Depois de instalar a geometria real da nova região/coluna, reabrir e reaplicar a origem horizontal da linha conforme State; conservar contador inicial/continuado. Não criar página por recuo, duplicar inset ou copiar largura da região anterior.',
'compiler/eval/rules/apply.md':'Nos dois percursos de show-set já existentes, quando o nó original e a visualização ainda são Bibliography, copiar Length par.hanging-indent explicitamente casado para marcador privado bibliography.hanging-override. A presença causal é o critério, sem inferir por valor ou alterar casamento/guards, callbacks, aliases ou ordem de realização.',
'compiler/eval/rules/set.md':'No arm par, validar/avaliar hanging-indent exatamente uma vez como já prescrito, transportar Length e ocorrência local. Alternar um Bool privado eval.par.hanging-boundary por frame da chain para diff_styles não apagar set repetido; o consumidor verifica presença da chave, não seu valor. Retirar somente o warning de propriedade não suportada de hanging; demais propriedades e avisos permanecem.',
'compiler/stdlib/structural/par.md':'No constructor, transportar o Length hanging-indent já validado para Styles par.hanging-indent, preservando abs+em/sinal e precedência de erros/spans. Não reavaliar Args/body nem mudar representação pública; first-line-indent e justification-limits continuam separados.'}
for owner,row in owners.items():
 f=r/owner;s=f.read_text();newlines=[]
 if row['mode']=='functional':newlines.append('- '+str(n.relative_to(r))+' sha256:'+pins[str(n.relative_to(r))])
 for line in s.splitlines():
  if line.startswith('- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:'):line='- '+str(selection.relative_to(r))+' sha256:'+pins[str(selection.relative_to(r))]
  newlines.append(line)
 # Merge the new reference into the existing canonical block.
 if row['mode']=='functional':
  added=newlines.pop(0);s='\n'.join(newlines)+'\n'
  if 'Núcleos Tekt:\n' in s:
   a=s.index('Núcleos Tekt:\n')+len('Núcleos Tekt:\n');b=s.index('\n\n',a);refs=s[a:b].splitlines()+[added];s=s[:a]+'\n'.join(sorted(set(refs)))+s[b:]
  else:
   a=s.index('\n\n')+2;s=s[:a]+'Núcleos Tekt:\n'+added+'\n\n'+s[a:]
  spec=responsibilities[owner.removeprefix('00_nucleo/prompts/')]
  s+=evidence+'\n### Responsabilidade exclusiva deste owner\n\n'+spec+'\n\n### Aceitação\n\nRED→GREEN antes da implementação; controles fonte→layout e bilaterais dos sinais,\nfronteiras, variantes e restauração pertinentes. Não deduzir cobertura de todas\nas alternativas por executar um arm agrupado: classificações nominais fechadas\ndo contexto inline/fluxo são intencionais e exigem testes por classe/variante.\nResselo apenas após V15/V26 válidos; testes, build, Clippy/Rustdoc/fmt, reversa\ne diferencial Tekt do pin1279707, sob12GiB/High8GiB/swap0/jobs1. Sem publicação.\n'
 else:
  s='\n'.join(newlines)+'\n';s+='\n## Pin do núcleo de seleção — 2026-10-18\n\nAs medições e o refinamento de forma/RTL em\n00_nucleo/diagnosticos/bibliography-hanging-2026-10-18 precedem o novo pin.\nEste owner recebe somente o pin do núcleo compartilhado; seu código executável,\nAPI, fase e demais responsabilidades não mudam nesta etapa.\n'
 f.write_text(s)
owner=r/'00_nucleo/prompts/compiler/layout/paragraph_indent.md'
owner.write_text('''# L0 — paragraph_indent
Hash do Código: 00000000

Núcleos Tekt:
- '''+str(n.relative_to(r))+' sha256:'+pins[str(n.relative_to(r))]+'''

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_indent.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.
'''+evidence+'''
### Responsabilidade exclusiva

Possuir Spec privado Copy com offset inicial/continuado e Dir, State local de
contexto Paragraph/Inline, escopo e aplicação/commit de linha, e Shape puro com
largura inicial/continuada. Invariantes compartilhadas vêm do núcleo pinado.
Não fazer I/O, eval, callbacks ou localização e não modificar conteúdo público.

Escopo é o tronco lexical completo: fatorar somente um wrapper que cobre todo
o conjunto substantivo, atravessando Source/Located/Realized pela visualização
já pronta; vários siblings não fazem consenso de valores. Em depende do
TextStyle do tronco. Declaração par.* parcial interrompe pelo owner style_scope,
portanto não entra como fragmento tipográfico num preparo global maior.

Classificar fragmento de container por Content fechado: Par/Parbreak e objetos
de fluxo exigem contexto Paragraph; inline simples não. Recursão por Sequence,
Styled e envelopes transparentes; Box/inline math/raw são unidades inline no
exterior com corpo próprio, Raw/Equation/Quote consultam seu flag block. Não
executar Dynamic/ContextBlock/ShowRecipe para prever forma; conteúdo opaco não
certifica um parágrafo e usa a admissão explícita quando materializado. O match
nominal exaustivo conserva diferença entre todas as variantes, sem catch-all
silencioso. Agrupamentos são classes deliberadas de layout, não prova de teste
de cada alternativa.

Contexto genérico Paragraph dá Spec(0,h); Fragment dá Spec(0,0). Hanging CSL
pode fornecer uma compensação pendente com Spec(-h,0) dentro do inset h já
instalado. Ler sem consumir durante preparação/seleção; consumir quando uma
unidade material recebe sua origem. Preservar forma inicial enquanto a primeira
linha não foi commitada, mesmo se a admissão avançar região. Sem fabricar HSpace,
texto, Par, tags ou Location. Após primeira linha, limites normais do inset
mantêm todas as continuações e subestruturas do body. Subframes auxiliares
suspendem a fila exterior; checkpoint a conserva.

Aplicar ao cursor uma vez por linha usando borda inicial da direção, e mover
carriers literais iniciais pelo mesmo delta horizontal quando necessário.
Consultar limites efetivos sem reescrever line_start_x/width globais. Encaminhar
larguras inicial/continuada a ambos seletores. Quebra física consome a linha,
fronteira de parágrafo limpa escopo, nova região só reabre aplicação. Presença
sem material não cria altura. Valores negativos/extremos mantêm os offsets
reais, sem clamp que apague o contrato, e o progresso de palavras overlong.

Hooks do dispatcher são finos: aplicar origem apenas a unidades inline de
layout; finalizar escopo quando o fluxo confirmou fronteira ou há pagebreak/
colbreak explícito. Classificação, cálculo e política ficam neste módulo.
Não reposicionar origens semânticas/Location por heurística de tinta.

### Testes e aceitação

Primeiro adicionar testes no próprio módulo por APIs atuais e registrar RED
funcional, sem depender de funções ainda inexistentes como causa da falha.
Cobrir set/constructor, zero/negativo/abs+em/fontes parciais, primeira linha e
continuação, fronteiras repetidas/vazias, simple/optimized/hifenização, RTL,
containers inline/Par, páginas, checkpoint/subframe e bibliografia/defaults/
overrides. Depois testar forma/custos com entradas discriminantes, sem espelhar
somente os ramos do helper. Preservar texto lógico em frames extremos; PDF
recortado não certifica perda/cópia. Repetir corpus anterior e matriz bilateral,
declarando métricas/realização anteriores ainda pendentes, sem relaxar controles.
V15/V26 antes de resselo e implementação, resto dos gates e limite12GiB, sem
publicação. O objetivo integral e R3-T1 não são encerrados por este owner.
''')
(p/'prescription.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'owners':list(owners)+[str(owner.relative_to(r))],'new_owner':str(owner.relative_to(r)),'new_consumer':'01_core/src/compiler/layout/paragraph_indent.rs','nuclei':pins,'classification':'ADR0127 continuous internal parity; no public API/phase/default-mode change','measurement_cases':{'main':92,'contexts':26,'boundaries':10,'repeated':4}},indent=2)+'\n')
print({'existing_owners':len(owners),'new_owner':str(owner),'nuclei':pins})
