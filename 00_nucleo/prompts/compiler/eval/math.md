# Prompt L0 — `compiler/eval/math`
Hash do Código: fcd396d9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/math.rs`
**Vanilla ratificado:** `a51e02804`

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
