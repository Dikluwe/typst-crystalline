# L0 — paragraph_indent
Hash do Código: f1d49be1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:3a97c056eff81b344f46d83567274225e3519ae141fce4817e342d573a92fb41

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_indent.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Recuo francês — aplicação física (2026-10-18)

### Medição anterior à decisão

lab/results/construction/diagnostics/bibliography-hanging-2026-10-18/before-sources.json.gz
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

### Refinamento medido após o primeiro RED→implementação

A primeira rodada funcional deixa RTL com Spec.dir LTR: cursor.rs recebe
right100, enquanto o transporte text.dir está em custom (paragraph.rs:427–436),
não em TextStyle::from isolado. Resolver Dir a partir do text.dir do tronco
lexical (Value::Dir), com fallback tipado/LTR; não tomar direção de run parcial.
O teste de checkpoint deve capturar uma fronteira confirmada com linha drenada,
como o protocolo de sufixo existente; não exigir recuperação de prefixo já
movido entre buffers por uma transação artificial. Manter prova de texto do
prefixo, exclusão da tentativa e recuo da continuação após restauração.

### Decorações: medição complementar de origem (2026-10-18)

Os12 pares SVG em bibliography-hanging-2026-10-18/deco-before.json.gz,
com a leitura corrigida de primitivas line/path em deco-inspection.json,
usam o mesmo binário/freeze da suíte inicial9026 e o oracle pinado bd5ebec.
Em overline h10, a segunda linha tem texto em x18 mas traço em x8;
vanilla começa o traço em x18. h0 já tem diferenças de y/espessura/cauda,
que não serão encobertas nem fechadas por este reparo horizontal.
Fonte local: decorations.rs:43–45,76–78 usa cursor/line_start_x globais;
line_commit.rs:615–619 e sub_frame.rs:350–354 capturam início global.
Isso é geometria da linguagem, não obrigação de copiar buffers vanilla.
Refutação: origem duplicada, consumo precoce da compensação, mudança de
baseline, fim, espessura, texto ou região fora do delta horizontal prescrito.
Correção privada de paridade ADR0127 em fluxo contínuo, sem fase/API nova.

Responsabilidade deste owner: Fornecer cursor_start privado read-only: prever somente o delta ainda não aplicado da Spec atual ou da compensação pendente, sem iniciar escopo, consumir fila ou admitir material. RTL não recebe delta físico de cursor; conservar seu alinhamento existente. O fallback geométrico sem Spec é zero. Acrescentar testes de origem das três decorações e sinais, entrada após quebra e subframe; não fechar diferenças verticais anteriores.

### Preview sem material

O preview negativo pode ser menor que o cursor real antes de qualquer
admissão; portanto end > preview sozinho não prova um segmento. Acrescentar
teste RED de decorações vazias após quebra com h negativo, incluindo body
vazio, metadata e Par vazio. Expor apenas uma consulta privada read-only de
applied no owner paragraph_indent. No owner decorations, quando o preview
difere do cursor real inicial e não houve segmento flushado, o segmento
final exige origem realmente aplicada. Não iniciar escopo/admissão para
satisfazer a decoração e não criar tinta/altura de um body vazio.
