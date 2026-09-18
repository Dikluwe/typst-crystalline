# Prompt L0 — `compiler/eval/math`
Hash do Código: 9fe9690a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/math.rs`
**Vanila ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`

## Medição e contrato

### Carrier dos anexos produzidos pela sintaxe

Todo slot ausente na sintaxe produz `MathAttachSlot::Omitted`; cada sub/sup ou
prime efetivamente construído produz `Present(content)`. Este owner não
fabrica `ExplicitNone`, pois a gramática dessa forma não fornece o valor.

Merge de primes, ordem, avaliação, spans e diagnósticos permanecem vigentes.
Não mudar parser, AST, Args, Value, defaults ou fase.

Converte AST em Content matemático, resolve operadores/símbolos pelo scope,
avalia chamadas e preserva anexos, delimitadores, frações, raízes e alinhamento.
Identificador desconhecido usa hints math; deprecated avisa e resolve. Eval não
executa geometria de layout.

## P1425 — presença de `inline` na sintaxe bare (APROVADO no gate ADR-0127)

No braço sintático `limits`, além do valor booleano já validado, preservar se
o named `inline` esteve causalmente presente. Construir
`Content::MathLimitsOverride` com `inline_explicit:true` somente nessa
presença; omissão usa o default `true` com bit falso. O braço `scripts` usa bit
falso. Named desconhecido, cast, avaliação única e spans permanecem vigentes.

A sintaxe não chama o binding qualificado nem executa layout; ambos convergem
somente no carrier. Esta alteração foi confirmada pelo dono em 2026-09-16.

## P1426 — presença do índice na sintaxe bare (APROVADO no gate ADR-0127)

O braço sintático `root` continua a exigir os dois slots da sua forma: índice
e radicando. Um índice vazio é `Some(Content::Empty)` e presença verdadeira;
um valor `none` avaliado é `None` e presença verdadeira. A sintaxe `sqrt`
constrói `None` com presença falsa. Ambos usam o constructor canónico, sem
executar layout nem converter índice vazio em ausência.

Esta alteração do carrier público foi confirmada explicitamente pelo dono em
2026-09-16.

## Aceitação

Lookup, erros, warnings, chamadas e morfologia batem com a língua ratificada.

## P1291.cancel-angle-runtime — valores executáveis em argumentos math

### Medição anterior à decisão

`eval_math_arg_value` já separa escalares que devem seguir `eval_expr` dos
corpos que devem seguir `eval_math_expr`, mas não inclui `Expr::Closure` nem
`Expr::Contextual` no primeiro grupo. Como consequência, o argumento
`angle: a => 0deg` de `math.cancel` é convertido em `Value::Content` e
`angle: a => context ...` tenta avaliar `context` como identificador math. O
avaliador genérico já materializa essas duas formas como valor executável e
contextual; a falha ocorre antes da native e antes do transcript selado.

### Decisão

Em argumentos posicionais ou nomeados de chamada math, `Expr::Closure` e
`Expr::Contextual` seguem `eval_expr`, como os demais valores de código. Não
são convertidos em `Content` e não são executados nesta fase. Quando o parser
envolver o argumento num `Expr::Math` sintético, o adaptador pode reancorar os
mesmos bytes em modo Code **somente** se houver exatamente uma expressão e ela
for uma das classes de valor de código já autorizadas (inclusive closure ou
contextual); zero, duas ou mais expressões e qualquer classe de conteúdo
permanecem no caminho `eval_math_content`. O span original ancora o reparse.
Demais expressões mantêm a classificação vigente: conteúdo math continua em
`eval_math_expr`, e escalares/código continuam no ramo já existente.

Isto habilita o transporte de `MathCancelAngle::Func` até a native; execução
dependente do ângulo default e do estilo permanece exclusivamente na pipeline
L3, conforme `compiler/math/layout/callbacks.md` e `infra/pipeline.md`.

Aceitação: closure literal direta ou sob wrapper math unitário chega à native
como `Value::Func`; callback
contextual observa o snapshot de estilo selado; expressão math comum continua
`Value::Content`; nenhuma callback é executada por `eval_math_arg_value`.

## P1285 — preservar grapheme distinto de número

### Medição antes da decisão

Em `eval/math.rs:327-333`, `MathTextKind::Grapheme` e
`MathTextKind::Number` convergiam ambos para `Content::MathText`. A enum
fechada já possui `Content::MathIdent`, e os consumidores de layout/spacing
já distinguem `MathIdent` (variável/símbolo) de `MathText` (número ou texto).
No vanilla ratificado, a morfologia pública de `$ x^2 $` serializa a base
como `symbol("x")` e o expoente como texto `"2"`; a primeira candidata P1285
não consegue recuperar essa distinção depois do eval.

### Decisão

`MathTextKind::Grapheme` produz `Content::MathIdent` e
`MathTextKind::Number` produz `Content::MathText`. A decisão preserva a
informação já presente na AST e usa variantes existentes; não cria tipo,
campo, assinatura ou fase. Shorthands, símbolos resolvidos pelo scope e
identificadores desconhecidos mantêm seus caminhos atuais. O armazenamento
interno de anexos (`tr`/`br`) também permanece; a apresentação pública pode
normalizar esses nomes sem alterar layout.

Aceitação: `$ x $` preserva folha `MathIdent("x")`; `$ 2 $` preserva
`MathText("2")`; `$ x^2 $` mantém base e expoente distintos e a suíte de
layout/math permanece verde.

## P1292.vec — sintaxe e chamada convergem no constructor canónico

### Medição anterior à decisão

Em `01_core/src/compiler/eval/math.rs:1030-1062`, o braço `vec` aceita de fato
apenas `delim`, ignora named desconhecido e emite `Content::math_matrix`.
O vanilla ratificado declara `VecElem` próprio em
`lab/typst-original/crates/typst-library/src/math/matrix.rs:18-68`, com
`delim`, `align`, `gap` e filhos variádicos. Probes de 2026-08-31 confirmaram:
`align` só horizontal (`start|left|center|right|end`), `gap` é relative length,
delimiter aceita par, símbolo/string unitário inferível ou `none`, e named
desconhecido é erro.

### Decisão

O braço sintático `vec` e a função pública `math.vec` convergem no mesmo
constructor `Content::math_vec`; para a mesma lista/named produzem a mesma
identidade e morfologia, nunca `MathMatrix`. A style chain fornece defaults/set
rules; argumentos explícitos prevalecem e marcam presença. Default:
delimitadores `(` e `)`, alinhamento `center`, gap `0.2em`. Cada positional é
avaliado uma vez e preservado como um filho, inclusive zero ou um. `&` dentro
do filho permanece conteúdo de alinhamento da célula, sem criar linha extra.

Named desconhecido e casts inválidos propagam as mensagens medidas no recibo
P1292; não há descarte silencioso. `delim` aceita array, `none`, symbol ou
string e normaliza para o par tipado; `align` aceita somente alinhamento
horizontal; `gap` aceita relative length. `Start`/`End` são preservados no
payload mesmo enquanto a direção de escrita resolve geometricamente para
esquerda/direita. O percentual de `gap` é transportado integralmente; sua
resolução pertence a `P1292.vec-region-gap` no owner de layout.

Owners canónicos: `entities/elements/math_vec.md`, `entities/content.md`,
`compiler/stdlib/structural/math.md` e `compiler/math/layout/vec.md`.

## Sintaxe `attach`/`binom` e módulo público

A sintaxe matemática e `math.attach`/`math.binom` delegam aos mesmos
construtores puros. Ambos os caminhos produzem payload, morfologia e
diagnósticos equivalentes sem executar layout.

`attach` conserva base e os seis slots `t,b,tl,bl,tr,br` no carrier
triestatal: omitido, `none` explícito e conteúdo presente. `binom` conserva
upper e lower variádico na ordem e constrói `MathFrac(line:false)` entre
parênteses extensíveis.

A sintaxe pode fornecer spans mais precisos, mas não duplicar casts, defaults
ou estrutura. O dispatcher qualificado valida aridade e named no owner stdlib.

## P1307-R3 — construção coerente de Args em math

### Medição anterior à decisão

No baseline R3 SHA-256
`b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree P1306
ali capturado, `01_core/src/compiler/eval/math.rs:315-341,570-596,1330-1351`
constrói Args de AST avaliando valores math uma vez, mas retém só items/named
e span agregado. Os três loops ignoram Spread atualmente. A fonte ratificada
`lab/typst-original/crates/typst-eval/src/call.rs:412-460` conserva spans de
argumento/valor e distingue spread de Args. Não se infere que este módulo
já implemente esse spread: a omissão é dívida fora deste ajuste.

### Decisão de migração, dependente do gate Args

Cada builder AST existente deve produzir ocorrências pelo contrato público
de `entities/args.md`: positional/named em ordem lexical, com o Value já
obtido por `eval_math_arg_value`, span do argumento e span da expressão-valor.
Usar `Args::from_occurrences`; não recriar views manualmente nem invalidar
origens para adaptar um literal. Preservar o span agregado real da lista,
exceto o transporte fechado de chamada inteira dos encoders descrito abaixo.
Assim a mesma nativa recebe os dados causais disponíveis em math sem mudar
seleção de valores, retorno Content, geometria ou fase. Não acrescentar
suporte a Spread aqui, nem anunciar paridade total dessa rota como efeito
desta migração. Se caso obrigatório depender dessa dívida, reabrir escopo.

O owner único continua math.rs; não duplicar validação de encoders nem mudar
L0s/núcleos dos elementos. Testes devem preservar attach/binom/mono/script e
as rotas genérica/qualificada, incluindo avaliação única e named repetidos
transportados sem descarte. É inferência de suficiência refutada por origem
perdida em outro builder ou exigência de semântica math nova.

### P1307-R4 — transporte agregado nas chamadas genéricas

O recibo `00_nucleo/diagnosticos/p1307-r4-call-span-refinement.json`, SHA-256
`6e2335e203c2ed888e97259159e5057c904f5a808d5dddde6cf9d3f0698b8912`,
congela o estado anterior. `math.rs:551-597,1291-1352` dispõe da função
resolvida e `call.span()` até aplicar os Args. Vanilla ratificado
`typst-eval/src/call.rs:149-157` usa a chamada inteira no agregado math;
o diagnóstico missing de encoder aprovado depende desse dado.

As duas rotas genéricas, qualificada e bare resolvida no scope, chamam o
mesmo helper interno de transporte definido em
`compiler/eval/call_dispatch.md`, antes de `apply_func`. Só as três
identidades encode recebem o agregado inteiro, inclusive através de With.
Não duplicar detecção nominal ou validação neste owner. O builder específico
de attach/binom não precisa desse ajuste. Permanecem intactos o retorno
Content-only já exigido, seus erros e a dívida de Spread ignorado. É correção
interna ao diagnóstico aprovado, sem API pública ou mudança de fase.

## P1308 — reutilização do transporte fechado de panic

Medição anterior à decisão: os dois caminhos genéricos acima já chamam o
helper R4; `typst-eval/src/call.rs:149–157` passa agregado de chamada em math
também. O baseline P1308 conserva seus hashes e fontes antes da alteração.

Usar o helper renomeado `transport_native_call_span` do owner call_dispatch.
Seu conjunto fechado passa a incluir panic além dos três encoders. Não
duplicar identidade/seleção, alterar builders ou habilitar Spread. Chamadas
específicas de elementos e retorno Content-only conservam seus contratos.

## P1395 — precedência lexical do callee bare em math

### Medição anterior à decisão

No vanila ratificado, `typst-eval/src/call.rs:93-109` avalia primeiro todo
callee `MathIdent` e só depois o classifica como função ou não-função;
`typst-eval/src/math.rs:45-54` encaminha essa avaliação para
`Scopes::get_in_math`; e `typst-library/src/foundations/scope.rs:80-92`
consulta `top` e os escopos lexicais internos antes do módulo math da base.
Quando o valor lexical é função, `call.rs:147-152` aplica essa função. Quando
não é função, `call.rs:159-162,551-604` exibe o valor e preserva os argumentos
como conteúdo matemático delimitado; named e spread nesse caso produzem seus
diagnósticos próprios em vez de coerção ou chamada.

No baseline cristalino P1395,
`01_core/src/compiler/eval/math.rs:514-600` converte imediatamente o callee
`MathIdent` em nome e entra no despacho nativo/hardcoded. O lookup de função
em `math.rs:1282-1337` só ocorre no fallback posterior, portanto nomes já
interceptados como `abs` e `sqrt` nunca consultam binding lexical. Isto
contrasta com a precedência local já aplicada ao identificador standalone em
`math.rs:382-418` e prometida pelo próprio módulo em `math.rs:290-314`.

As sondas bilaterais P1393/P1395, no baseline HEAD
`dd33334ca36e2d7abf79d16b200f07691d950c59`, observaram:

- `#let abs(x) = [LOCAL-ABS]\n$abs(1)$\n`, SHA-256
  `8a2f9a49115901ac783a2764b121d04ea0442e010220a1cd26dc0a93bffcef2d`:
  vanila `LOCAL-ABS`; cristalino `|1|`;
- `#let sqrt(x) = [LOCAL-SQRT]\n$sqrt(1)$\n`, SHA-256
  `856ac0ad5ba024adcdf1cd160161550c62e0aa9d63cfb224b2caf3267e6c9b63`:
  vanila `LOCAL-SQRT`; cristalino preserva a raiz nativa de `1`.

Os quatro processos válidos terminaram com exit 0 e zero swap. Picos desta
reprodução: 31.636/39.104 KiB para vanila/cristalino em `abs` e
31.428/39.272 KiB em `sqrt`. Uma tentativa instrumental que colocou
`--color` depois de `compile` no vanila terminou com exit 2 e foi descartada;
ela não sustenta a decisão.

### Classificação e decisão

A prioridade lexical e o conteúdo/diagnóstico resultantes são semântica e
morfologia públicas da língua, não estrutura Rust, bytes de PDF ou ordem
interna de branches. A cadeia explícita `MathIdent.eval -> get_in_math` e o
tratamento intencional de `NonFunc` na fonte ratificada mostram intenção, não
apenas coincidência de saída. A inferência é que todo callee bare
`MathIdent` deve respeitar o binding lexical antes do namespace math; seria
refutada por fonte posterior no mesmo pin que introduzisse exceção nominal
para `abs`, `sqrt` ou outra native, ou por sondas bilaterais que distinguissem
esses nomes.

Antes de qualquer despacho nativo/hardcoded de um callee `MathIdent`, consultar
o binding lexical com a mesma ordem de `Scopes::get_local`. Se houver
`Value::Func`, avaliar os argumentos uma vez pelo builder math canónico e
aplicar essa função; assim closures locais sombreiam ao menos `abs` e `sqrt`.
O resultado Content mantém sua morfologia. A obrigação não amplia neste passo
o contrato histórico de retorno Content-only das chamadas funcionais math.

Se houver binding lexical não-função, não chamar nem reinterpretar o valor
como native: exibi-lo pela conversão pública já vigente e concatenar os
argumentos posicionais como conteúdo math delimitado, preservando sua ordem,
pontuação e morfologia math. Named ou spread usados com esse valor seguem o
fragmento diagnóstico medido em P1395-R6 abaixo; `Unknown`, descarte silencioso
ou coerção inventada não satisfazem o contrato.

Sem binding lexical, `abs`, `sqrt` e os demais caminhos nativos/hardcoded
permanecem inalterados. Não mudar parser, AST, `Scopes`, `Args`, assinaturas
públicas, defaults, fase eval/layout, field access, namespaces qualificados ou
layout. É correção interna de paridade em fluxo contínuo conforme ADR-0127.

Aceitação no nível da língua: os dois witnesses locais produzem exclusivamente
`LOCAL-ABS` e `LOCAL-SQRT`; um valor local não-função conserva sua forma seguida
dos argumentos entre parênteses, e named/spread conservam os diagnósticos
medidos; os controles sem binding continuam com a morfologia nativa de valor
absoluto e raiz. Comparação de bytes ou de variantes Rust não concede crédito.

### P1395-R6 — medição direta do fragmento não-função

#### Medição anterior à decisão

Depois do candidato existir, mas sem o autor deste L0 o ler, o mesmo oráculo
CLI foi executado diretamente contra o vanila ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. O recibo temporário
`/tmp/p1395-vanila-r4.json`, SHA-256
`145f6974f70b85a992ba02e43d3e36e1cc937b43304314af0e1bcbc5bf8adc22`,
terminou em 4,55 s, RSS máximo 32.288 KiB e zero swap sob 512 MiB/60 s.
Ele refutou três expected instrumentais anteriores:

- o posicional é extraído como `7(1, 𝑥)`, não `7(1, x)`; `𝑥` preserva a
  morfologia matemática do argumento;
- named tem como linhas capturáveis o primary
  `error: named-argument syntax can only be used with functions` e apenas o
  hint público de reparo
  ``hint: to render the colon as text, escape it: `foo\: 1```;
- spread tem como linhas capturáveis o primary
  `error: spread-argument syntax can only be used with functions` e apenas o
  hint público de reparo
  ``hint: to render the dots as text, add a space: `.. xs```.

Nos dois erros, o CLI vanila também apresenta uma anotação ancorada no span,
`abs is not a function`, sem o prefixo `hint:`. O adaptador
`public_diagnostics` do oráculo captura somente linhas públicas `error:` e
`hint:`. O `SourceDiagnostic` simplificado do cristalino não representa essa
anotação como categoria distinta.

#### Classificação e decisão

A forma `𝑥` é morfologia da língua e substitui o expected ASCII anterior. As
duas mensagens primary e os dois hints de reparo são o fragmento diagnóstico
público materializável neste owner. A anotação de span é observável no vanila,
mas não é representável no contrato diagnóstico simplificado atual; fica como
dívida explícita/scope-out, nunca como `MATCH` diagnóstico integral. Essa
limitação seria refutada por introdução, em owner próprio e gate público, de
carrier capaz de representar a anotação.

Assim, para P1395, o não-função posicional aceita exatamente a morfologia
`7(1, 𝑥)` no witness medido. Named e spread devem preservar o primary e o único
hint de reparo capturados acima. Não fabricar um segundo hint dizendo que
`abs` não é função; não apagar nem reivindicar equivalência da anotação de span.
As cláusulas principais de precedência das closures locais e os controles
nativos permanecem inalterados.

## P1427 — convergência da sintaxe de fração (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O branch atual de `frac(...)` coleta apenas posicionais, devolve Empty quando
faltam e ignora excesso/named. O branch `/` já recebe AST capaz de informar se
um par exterior foi removido, mas não transporta essa proveniência.

### Decisão proposta

`frac(a,b)` usa o mesmo parser de argumentos e constructor que `math.frac`,
com estilo omitido. `/` permanece associativo à esquerda e constrói o mesmo
carrier vertical, mas passa os dois bits `was_deparenthesized` do AST. Um par
exterior removido é lembrado; um par interior remanescente continua conteúdo.
Não filtrar named, não truncar posicionais e não fabricar Empty para aridade
inválida.

## P1429 — convergência das chamadas bare de acento (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

Os wrappers `hat`/`tilde`/`dot`/`dot.double` constroem hoje o recorte P296 e
descartam named. O vanilla encaminha os wrappers de símbolo para o mesmo
elemento `AccentElem`, com codepoint canônico fixo e named `size`/`dotless`.

### Decisão proposta

`accent(...)` bare usa a mesma nativa e o mesmo parser de argumentos que
`math.accent(...)`. Wrappers simbólicos exigem um base posicional, fixam o
codepoint normalizado e aceitam somente `size`/`dotless`; convergem no carrier
completo e preservam presença. Não duplicar normalização, casts ou política de
diagnósticos dentro do parser math.

## P1430 — sintaxe bare de `cases` converge no carrier público (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O braço atual avalia children, separa `MathAlignPoint` em rows e ignora falhas
de cast em named por usar apenas `if let Ok`. Isso perde a identidade causal
que o vanilla conserva como children variádicos e diverge da futura função
qualificada.

### Decisão proposta

Construir Args math canônicos, avaliando cada ocorrência uma vez, e delegar ao
mesmo constructor/validador de `math.cases`. Não pré-resolver `&`, não descartar
Spread/named/erros e não executar layout. A construção guarda defaults e
argumentos explícitos, ativando somente o bit correspondente; não assa a
cascata no carrier. Bare e qualificado devem produzir children,
delimitadores, reverse, gap e presença idênticos para entradas equivalentes.

## P1431 — sintaxe bare de `mat` converge no carrier público (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O braço atual implementa parsing próprio, aceita apenas gaps absolutos e
augment inteiro, não transporta align/presença e usa `if let Ok` em named,
silenciando falhas. Ele também mantém uma segunda regra de rows distinta da
função qualificada ausente.

### Decisão proposta

`mat(...)` bare avalia os argumentos uma vez, preserva a ordem/ocorrências e
delega ao mesmo parser/construtor de `math.mat`. O algoritmo de rows é único:
zero ou apenas escalares produzem uma linha; presença de array exige arrays em
todos os posicionais; rows menores são completadas com `Content::Empty`.
`MathAlignPoint` permanece na célula até ao consumer de layout. Nenhum cast,
named desconhecido ou duplicado pode ser descartado silenciosamente.

## P1436 — sintaxe bare de `lr` converge no carrier público (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O branch atual exige exatamente um positional, ignora named e chama
`rewrite_lr_body`, que devolve o body ou o degrada a `MathDelimited`. Isso
diverge do `LrElem` variádico e apaga `size`, identidade e nesting.

### Decisão proposta

`lr(...)` bare constrói `Args` math canônicos e delega à mesma nativa de
`math.lr`. Zero, um e muitos posicionais seguem a mesma junção por vírgula;
`size:` pode aparecer antes ou depois e valores de código como `#150%` chegam
ao cast relativo sem serem convertidos em conteúdo. Avaliar cada ocorrência
uma vez, preservar ordem, duplicados e spread, e remover a reescrita
antecipada `rewrite_lr_body` desta rota. Delimitadores sintáticos interiores
permanecem no body como seu próprio conteúdo `lr`, preservando nesting.

## P1437 — sintaxe bare de `mid` converge no carrier público (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

Hoje `mid(...)` bare cai na interpretação textual. O vanilla produz o mesmo
`MidElem` da forma qualificada, preservando identidade, nesting, aridade e
erros do builder público.

### Decisão proposta

`mid(...)` bare constrói `Args` math canônicos e delega à nativa de
`math.mid`. Exigir um positional; preservar spread, ordem e avaliação única;
rejeitar named/tipos inválidos. Não procurar cercas nem executar stretch.

## P1438 — sintaxe bare de `norm` converge no carrier público (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O braço comum atual exige um positional, ignora named e devolve
`MathDelimited('‖', body, '‖')`. O vanilla produz o mesmo `LrElem` da função
qualificada, conserva `size`, identidade e nesting e rejeita argumentos fora
da assinatura pública.

### Decisão proposta

`norm(...)` bare constrói `Args` math canônicos e delega ao mesmo
parser/construtor de `math.norm`. Avaliar cada ocorrência exatamente uma vez,
preservar ordem e spread, aceitar somente `size:` e um positional e propagar
duplicados, casts e named inválidos. O resultado é sempre `Content::MathLr`
com body `MathSequence([‖, body, ‖])`; não reescrever para `MathDelimited` nem
executar layout.

## P1439 — sintaxe bare de `overbrace` converge no carrier público (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O braço agregado atual ignora named/excedentes e baixa a chamada para
`MathUnderover`, apagando identidade e presença. O vanilla usa o mesmo
elemento público nas formas bare e qualificada.

### Decisão proposta

`overbrace(...)` bare constrói `Args` math canônicos e delega ao mesmo builder
de `math.overbrace`. Preservar ordem, spread e avaliação única; exigir body,
aceitar somente annotation posicional opcional e propagar os diagnósticos
medidos. Não montar o glifo nem executar layout na avaliação.
