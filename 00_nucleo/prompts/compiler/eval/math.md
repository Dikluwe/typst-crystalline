# Prompt L0 — `compiler/eval/math`
Hash do Código: 2ad201fa

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/math.rs`
**Vanilla ratificado:** `a51e02804`

## Medição e contrato

### P1293 — carrier dos anexos produzidos pela sintaxe

#### Medição anterior à decisão

`01_core/src/compiler/eval/math.rs:422-459` avalia base/sub/sup e chama
`Content::math_attach` com `Option<Content>`; o caminho não cria `none`
explícito, mas é um produtor do mesmo payload público e precisa selecionar o
estado canônico sem inventar sentinel. O recibo causal P1293 SHA-256
`80a9543c9450f2350a42fa48df2e42cac63109bd074ebb37c2ec424cba473d5c`
mede que a convergência sintaxe/qualificada depende de preservar a identidade
dos estados até repr/layout.

#### Decisão

Todo slot ausente na sintaxe produz `MathAttachSlot::Omitted`; cada sub/sup ou
prime efetivamente construído produz `Present(content)`. Este owner não
fabrica `ExplicitNone`, pois a gramática desta forma não fornece esse valor;
ele apenas o preserva se vier de reconstrução autorizada. Merge de primes,
ordem, avaliação, spans e diagnósticos continuam os vigentes. Não mudar
parser/AST/Args/Value, defaults ou fase. A assinatura pública foi confirmada
sob ADR-0127 categoria 1 em `2026-09-02T08:06:37-03:00`.

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

## P1293 — sintaxe `attach`/`binom` converge com o módulo público

### Medição anterior à decisão

No baseline `7dd25ff0e222b6c7c640d6bc7957b98f94227507`,
`01_core/src/compiler/eval/math.rs:822-929,963-1008` contém braços sintáticos
próprios para `attach` e `binom`; o primeiro já constrói os sete slots, enquanto
o segundo já usa `MathFrac(line:false)` e delimitadores, mas ambos repetem
extração/validação. O vanilla ratificado medido em
`math/ir/resolve.rs:402-415,470-570,714-768` resolve sintaxe e função para a
mesma morfologia. As sondas P1293 confirmaram os pares sintático/qualificado
para attach, binom, mono e script sem usar `PartialEq` ou bytes internos como
prova.

### Decisão

Os braços sintáticos `attach` e `binom` avaliam cada argumento uma vez e
delegam aos mesmos constructors puros que servem `math.attach` e `math.binom`
no owner `compiler/stdlib/structural/math.md`. Não mantêm segunda tabela de
defaults, casts, aridade ou fórmula de payload.

Para `attach`, base é o único positional e os named fechados são `t`, `b`,
`tl`, `bl`, `tr`, `br`. Named omitido permanece slot ausente; `none`
explicitamente escrito permanece morfologia presente como conteúdo vazio e
chega ao layout como caixa vazia, semanticamente equivalente à omissão no
render. Named desconhecido, extra e cast inválido conservam mensagem e span do
argumento; o erro não é agregado em texto ad hoc pelo call inteiro.

Para `binom`, upper e todos os lowers posicionais preservam ordem; é obrigatório
ao menos um lower. A sintaxe produz a mesma fração sem barra e os mesmos
parênteses esticados que o caminho qualificado. `mono` e `script` continuam a
resolver as funções do scope math, com `script.cramped` transportado; este
owner não duplica seus estilos.

Eval continua sem geometria. Novo campo, entidade ou mudança de fase não foi
necessário. `Unknown` não é aceitação para qualquer par requerido; a adição da
superfície qualificada permanece bloqueada pelo gate humano P1293.
