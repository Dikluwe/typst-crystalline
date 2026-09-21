# Prompt L0 — `compiler/eval/rules`
Hash do Código: 13781d95

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/rules.rs`

## Medição e contrato

Avalia set/show rules, valida argumentos e aplica selectors com ordem e
terminação determinísticas. Show-set transporta Styles; transformações
preservam morfologia. Recursão termina por ponto-fixo morfológico ou teto
diagnosticado. Named desconhecido nunca é ignorado.

## Aceitação

Selectors, regras, warnings, casts, composição, recursão e spans seguem
`a51e02804`.

## P1339 — transporte da base nativa aprovada

### Medição anterior à decisão

HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto:
`rules.rs:2403-2408` delega Value::Selector ao conversor; `:476-480` e
`:829-838` reconhecem somente NodeKind::Par nas rotas de parágrafo.
As fixtures e registros `p1339-full-show-final-*` medem aplicação real de
strong/emph/text no vanilla ratificado `a51e02804`; os registros fixam binários,
working tree e UTC. A limitação CLI dos perfis cristalinos foi registrada
separadamente e não constitui evidência positiva de matching.

### Decisão de integração

Manter a delegação única de Value::Selector para selector_matching, incluindo
Element. NativeElement e Where com essa base seguem a travessia de nós; os
caminhos literais/Regex/Label continuam distintos. Não recriar filtros ou
reconhecer função pelo nome neste owner. A receita recebe o nó morfológico
correto; não texto concatenado dos descendentes nem fatia de regex.

As duas seleções especiais de regras de parágrafo devem reconhecer também
NativeElement da função par, inclusive seus filtros Where, por predicado
interno tipado do owner selector_matching. Não ampliar genericamente o
caminho de parágrafos a outros elementos ou a todas as composições antigas.
Preservar os momentos de síntese/aplicação já existentes, sem mudar fase ou
aplicar regras de texto/elementos novamente por causa do caminho especial.

Show-set, precedência, RuleId, guardas, ponto-fixo e teto mantêm seu contrato.
Testes devem distinguir transformação efetiva de simples repr não realizado,
cobrir match/miss/vazio e coexistência de bases antiga/nova. Um caso que exija
mudar fase ou informação pública adicional reabre o gate; não se resolve
por aplicação extra. A extensão de dados foi aprovada no P1339; aprovação e RED
ainda são condições anteriores ao código.

## P1286 — `#set smartquote`

### Medição anterior à decisão

O caminho vigente aceita `enabled` e apenas `quotes` string/auto/none em
`rules.rs:1881-1939`; o vanilla expõe ainda `alternative` e as formas
array/dict em `text/smartquote.rs:38-89,335-419`.

### Decisão

O set-rule aceita `enabled: bool`, `alternative: bool` e `quotes` no domínio
canônico P1286. Valida antes de inserir o delta; named desconhecido continua
nunca ignorado. `none` conserva apenas a compatibilidade cristalina vigente e
é normalizado como `auto`; não é alegado como superfície vanilla. O carrier é
o `Styles` existente, sem novo campo público ou fase.

## P1285 — show rules textuais por ocorrência

### Medição antes da decisão

O caminho literal já fatia cada `Content::Text` e chama a recipe para cada
ocorrência. O caminho regex anterior apenas testava `is_match` e entregava o
nó inteiro uma vez. No vanilla ratificado, `#show regex("f.o")` sobre
`foo fxo` chama a recipe para `foo` e `fxo`, e `regex(".")` sobre `a.b`
produz três matches; literal `"."` produz somente o ponto. Matches vazios são
rejeitados no constructor e nunca entram em loop.

### Decisão

O loop dedicado de regex usa `selector_matching::splice_regex_rule_matches`
para todas as transformações `Func`/`Content`/`Str`, entregando à recipe
somente a fatia morfológica casada. Preserva texto não casado, ordem,
revogação por `RuleId`, precedência existente e fronteira de nós. O loop
literal permanece dedicado e `is_node_rule(Text|Regex)` continua falso;
nenhum dos dois selectors passa a viajar pelo matcher de elementos.

Aceitação: literal repetido/parcial, regex repetida, distinção literal `.` vs
regex `.`, no-match e fronteira entre nós coincidem semanticamente com
`a51e02804`.

## P1406 — realização lexical de entries (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `fe3209458`, a aplicação normal de show rules termina antes de
`compiler::introspect` sintetizar `OutlineEntry`; por isso não existe nó
renderizável a transformar. No vanila `586e1bd43`, regra direta, filtro por nível
e parent contextual são observáveis, e uma regra declarada entre dois outlines
afeta somente o segundo.

### Contrato proposto

O owner fornece uma operação L1 de realização pós-introspecção que recebe a
entry gerada, o snapshot lexical causal e estilos privados do parent. Ela aplica
matching, precedência, revogação, guardas e teto uma única vez segundo o contrato
vigente. Sem substituição transformacional, aplica o show default composto por
helpers da entry e link causal. Reiterações do fixpoint podem refazer um
candidato, mas não acumulam transformações sobre o candidato anterior.

Não executar callback no layout, não usar uma pilha global final e não inserir a
entry realizada como nova ocorrência consultável. Confirmação humana explícita
recebida em 2026-09-15; a nova fronteira está autorizada pelo gate ADR-0127.

## P1412 — regras de estilo do outline

### Medição anterior à decisão

No baseline `2ff1a23d4`, `eval_set_rule` não reconhece o alvo simples
`outline` nem o alvo pontuado `outline.entry`; a realização fabrica sempre o
fill pontilhado. Além disso, uma transformação de entry que devolve conteúdo
contextual perde os estilos privados do parent antes da expansão pós-realização,
embora esses estilos estejam presentes durante a chamada da transformação.

### Decisão

`#set outline(indent:)` captura `length | function | auto` e o `bool` mantido
por compatibilidade cristalina em `outline.indent`. `#set outline.entry(fill:)`
captura `content | none` em `outline.entry.fill`. Campos desconhecidos são erro
`unexpected argument`; valores incompatíveis são erro de tipo. A chain é
léxica e argumento explícito no constructor vence o valor herdado.

Antes de capturar um outline para realização e introspecção, materializar nele
o `indent` herdado somente quando o argumento foi omitido. Antes de aplicar show
rules a uma entry construída diretamente, materializar do mesmo modo o `fill`
herdado somente quando `fill` foi omitido. Entries sintetizadas pela realização
recebem o fill efetivo já resolvido.

O resultado realizado de cada entry é embrulhado por um carrier visualmente
neutro com `outline.parent.indent` e `outline.parent.key`. Assim, qualquer
`ContextBlock` criado pela transformação e expandido posteriormente conserva o
parent causal exato. O carrier não muda matching, ordem, fase nem conteúdo
consultável.

## P1413 — síntese interceptável de `raw.line` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `e7b2ddd44`, show rules recebem `RawElem` como folha; não há
instâncias de linha. No vanila ratificado, raw inline e bloco sintetizam uma
linha por segmento, preservam vazio, índice baseado em 1, total e body, e
aplicam `#show raw.line` em ordem.

### Decisão proposta

Ao interceptar `RawElem`, detectar por matching tipado se alguma regra vigente
é aplicável a instâncias `raw.line`. Somente nesse caso, dividir o texto por
newline preservando vazios, construir os carriers dinâmicos com
`number/count/text/body`, aplicar o motor normal de show a cada instância e
recompor em ordem com fronteiras de linha. Se nenhuma regra casa, devolver o
`RawElem` intacto; não substituir highlighting/default por aproximação.

O constructor manual viaja pelo mesmo `Content::Dynamic` e motor normal.
Filtros `where` leem os campos do carrier. Guardas, precedência, teto e
revogação não ganham um segundo algoritmo.

Esta ampliação de contrato público aguarda confirmação humana explícita antes
de L1.

## P1414 — captura lexical de `#set par.line` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O target pontuado chega como `Expr::FieldAccess`; `text_str()` produz vazio e o
fallback vigente apenas avisa. O vanila aceita cinco campos e a numeração
alcança linebreaks explícitos e wraps automáticos.

### Decisão proposta

Reconhecer exclusivamente `par.line` no branch tipado de FieldAccess. Named
aceites: `numbering: string | none`, `number-align: horizontal alignment |
auto`, `number-margin: left | right | start | end`, `number-clearance: length |
auto` e `numbering-scope: "document" | "page"`. Cada valor validado entra na
StyleChain por chave `par.line.*`; omitido não altera a chain, desconhecido é
erro e nenhuma avaliação é descartada.

`numbering: function` fica explicitamente para P1415 porque callback não pode
ser executado no layout puro. P1414 não o aceita nem o converte em string. A
ampliação pública aguarda confirmação humana explícita.

## P1415 — captura de callback de `par.line` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `0a7de3dd9`, o branch de `par.line` rejeita `Value::Func` em
`rules.rs:1330-1335`. O vanila ratificado aceita função unária e só revela erro
de aridade quando a linha física é realizada. Executá-la durante o set perderia
o número pós-wrap e a página causal.

### Decisão proposta

Ampliar exclusivamente a validação de `numbering` para
`string | function | none`. Guardar o `Value::Func` intacto na StyleChain,
preservando identidade e span. Eval não chama `apply_func`, não antecipa
contador, não converte retorno e não fabrica marker. Campos e validações
restantes conservam P1414.

A execução tardia pertence ao contrato `infra/pipeline.md`; o layout puro só
produz request. A ampliação pública aguarda confirmação humana explícita.

## P1427 — propriedade settable `math.frac.style` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O vanilla declara `style` com default vertical no elemento `FracElem`; o
cristalino reconhece propriedades de frac de forma incompleta, sem transportar
um valor tipado até o layout.

### Decisão proposta

Aceitar `#set math.frac(style: ...)` somente no domínio string fechado
`vertical | skewed | horizontal`. Após validação, guardar a `Value::Str`
canônica no `StyleChain`, que não possui variante específica para este enum;
o consumer de layout faz a conversão total para `MathFracStyle`. Estilo
explícito no elemento vence o cascaded; ausência em ambos usa vertical. Named
inválido falha no set, não é ignorado. `binom` não consome esta propriedade
porque não é uma `math.frac` pública no vanilla.

## P1429 — propriedades settable de `math.accent` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O validador já reconhece `size` e `dotless`, mas os classifica como desligados
e só avisa; nenhum valor chega à construção ou ao layout.

### Decisão proposta

Ligar ambos os pares em `MATH_SET_LIGADOS`. `size` aceita relative length e
`dotless` aceita boolean, guardados em chaves canônicas da `StyleChain`.
Argumento explícito no elemento vence cascata; cascata vence defaults
`100%`/`true`. Regras não alteram os bits morfológicos de presença do elemento.

## P1430 — cascata tipada de `math.cases` (PROPOSTO; gate ADR-0127)

`#set math.cases` aceita somente `delim`, `reverse` e `gap`, com os mesmos
domínios do constructor público: par de delimitadores opcionais, booleano e
relative length. Validar antes de inserir no `StyleChain`; valor inválido ou
named desconhecido é erro atribuível, nunca warning nem fallback. A chamada
explícita prevalece sobre a cascata, que prevalece sobre os defaults
`("{", "}")`, `false`, `0.2em`. Valores cascaded não ativam bits de presença
morfológica no elemento.

## P1431 — cascata tipada de `math.mat` (PROPOSTO; gate ADR-0127)

`#set math.mat` aceita somente `delim`, `align`, `augment`, `gap`, `row-gap` e
`column-gap`, com os mesmos casts do constructor. `gap` alimenta os dois eixos;
o gap específico no mesmo nível prevalece. Argumento explícito no elemento
vence cascata, que vence defaults `()`, `center`, `none`, `0.2em`, `0.5em`.
Augment explícito dobra o stroke cascaded segundo a regra vanilla, mas preserva
os próprios offsets; `none` explícito apaga augment externo. Valores de set
rule nunca ativam bits morfológicos de presença no elemento.

No valor dict de `augment`, somente `hline`, `vline` e `stroke` são consumidos;
chaves internas desconhecidas são aceites e ignoradas, conforme o pin. Os tipos
inválidos desses três campos continuam erro. Esta tolerância interna não altera
a lista fechada de named da própria set rule.
## P1446 — propriedade settable `math.stretch.size` (PROPOSTA; gate ADR-0127)

Ligar o par já reconhecido `("stretch", "size")` ao transporte efetivo.
Validar `Rel<Length>` antes de inserir a chave `math.stretch.size` na
StyleChain; named desconhecido, duplicado e tipo inválido falham. No layout,
argumento explícito vence cascata, cascata vence `100%`. O valor cascaded não
altera `size_explicit`, fields ou repr do carrier.

## P1462 — set rule `format.svg` (MATERIALIZADA)

### Medição anterior à decisão

No baseline `7442d33b6bf48f136826d28cf71bd62f8710a432`,
`eval_set_rule` obtém `set.target().text_str()`. Um `FieldAccess` como
`format.svg` chega ao fallback com nome vazio e o cristalino apenas avisa que o
target não é suportado. O vanilla ratificado aceita `pretty: bool`, rejeita
named desconhecido com `unexpected argument: other` e tipo inteiro com
`expected boolean, found integer`.

Na calibração pré-candidata R1, o vanilla ratificado recebeu uma regra
`pretty: true` seguida de `#set format.svg()`. A saída permaneceu pretty em
`forward`, `reverse` e `forward-repeat`; portanto ausência do named não
materializa o default e não apaga a cascata. A medição e a proveniência estão
na medição histórica.

### Decisão proposta

Reconhecer estruturalmente o target pontuado exato `format.svg`, sem inferir o
nome a partir da string vazia e sem ampliar o fallback genérico. Aceitar
exatamente um named opcional `pretty`. Quando presente, validar o valor como
boolean antes de produzir mudança de estilo. Quando ausente, a set rule é
neutra: não insere `format.svg.pretty` e preserva qualquer decisão cascaded
anterior. O default público `false` só vale quando não existe decisão
documental. Argumento posicional, named duplicado/desconhecido ou tipo inválido
falha pelo canal normal de `SourceResult`, nunca warning nem descarte
silencioso.

O valor validado segue pelo carrier `StyleDelta.custom` vigente sob a chave
canônica `format.svg.pretty`. Este owner não extrai opção documental, não
decide se a regra está em top-level/container e não formata SVG. A fronteira
top-level e a mensagem de container pertencem ao owner puro
`compiler/layout/format_options`; L3 apenas orquestra sua aplicação.

Preservar todas as demais set/show rules, a ordem da cascata e a política de
named desconhecido. `format.pdf`, `format.html` e qualquer outro campo não
contratado continuam ausentes. A confirmação P1462 foi recebida em 2026-09-19
e a materialização foi validada; P1463 não reabre essa semântica.

## P1463 — set rule `format.png` (MATERIALIZADA)

### Medição anterior à decisão

No baseline `23cd47e4ade4d6deb7095cb7400478feffb1e9f9`, somente o target
pontuado `format.svg` possui braço estrutural. `format.png` cai no fallback de
target textual vazio, emite warning e não altera opções. No vanilla ratificado,
`#set format.png(ppi: 72)` e `ppi: 1.5` são aceitos; string e boolean falham com
`expected float, found <tipo>`; named desconhecido, positional e duplicado
falham com os diagnósticos normais. `#set format.png()` é neutro.

### Contrato

Reconhecer estruturalmente somente `format.png`. Aceitar zero ou um named
`ppi`; `Value::Int` é convertido para `f64` e `Value::Float` é preservado.
Quando presente, inserir `Value::Float` sob a chave canônica
`format.png.ppi`. Quando ausente, não inserir chave nem materializar 144.0,
preservando qualquer decisão cascaded anterior.

Argumento positional, named desconhecido/duplicado e variante não numérica
falham antes de alterar `StyleChain`. A mensagem de tipo usa `float` como
esperado e o nome público da variante encontrada. Valores finitos zero e
negativos não são rejeitados neste owner: o vanilla os aceita. Não há alegação
P1463 para não finitos cujo oráculo aborta.

Este owner não decide top-level, não lê CLI, não divide por 72 e não
rasteriza. A fronteira de container pertence ao collector puro; a conversão
da flag externa pertence à pipeline. Preservar integralmente `format.svg` e
as demais regras.

### Aceitação e gate

Cobrir integer/float, ausência, repetição/inversão, erros de named/tipo/
positional/duplicado e ausência do warning antigo. A confirmação humana
posterior ao commit causal P1463 foi recebida em 2026-09-19; a set rule foi
materializada e validada sem reabrir `format.svg`.

## P1465 — set rule feature-gated `format.html` (MATERIALIZADA)

### Medição anterior à decisão

No baseline `8e74134a721310cc4fb2a596a195398e16f1d5db`, os únicos targets
`format.*` reconhecidos estruturalmente são `svg` e `png`; `format.html` cai no
fallback textual e não transporta decisão. No vanilla ratificado, com a feature
ativa, a regra aceita exatamente o named opcional `pretty: bool`; regra vazia é
neutra, unknown/positional/duplicado/tipo inválido produzem os diagnósticos
públicos normais.

Uma expressão `nope: panic("arg")` é avaliada antes do erro de schema quando a
feature está ativa. Sem a feature, o gate de `format.html` acontece antes de
avaliar qualquer argumento e produz o diagnóstico de field feature-gated com
dois hints. Essa ordem é observável e impede validar o nome desconhecido antes
do gate ou descartar sua expressão depois do gate ativo.

### Contrato

Reconhecer estruturalmente somente `format.html`. Antes de percorrer os
argumentos, exigir `Feature::Html`; se ausente, devolver no field `html`:

```text
cannot access field `html` because the `html` feature is not enabled
hint: try enabling the `html` feature
hint: see https://typst.app/help/compiler-features for more details
```

Com a feature ativa, avaliar as expressões de argumento em ordem antes da
validação ordinária do respectivo schema. Aceitar zero ou um named `pretty` e
exigir `Value::Bool`. Quando presente, inserir o valor sob a chave canônica
`format.html.pretty`; quando ausente, não inserir chave e preservar decisão
cascaded anterior. Positional, named desconhecido/duplicado e tipo inválido
falham com as mensagens medidas, sem warning nem alteração parcial de styles.

Este owner não decide top-level, não extrai opções, não lê CLI e não exporta
HTML. A fronteira de containers pertence ao collector puro. Preservar
`format.svg`, `format.png`, o gate bare de `html`, todas as demais regras e a
ordem da cascata.

Aceitação cobre os quatro perfis, gate antes de panic, avaliação ordinária dos
argumentos com feature ativa, regra vazia, true/false, repetição/inversão e os
quatro negativos de schema. A confirmação humana posterior ao commit causal
`e300314e2f87e90e543c4923a8cae97f9ad978a8` foi recebida em 2026-09-20; a
regra e seus testes comportamentais foram materializados.

## P1467 — set rule `format.pdf(tagged, pretty)` (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `306ca50022104619447fae2625090b8d9c7b0a35`,
`01_core/src/compiler/eval/rules.rs:1413-1570` reconhece estruturalmente os
targets pontuados `format.html`, `format.svg` e `format.png`; não existe braço
para `format.pdf`. Medido via `./target/release/typst eval`:

```text
{ set format.pdf(tagged: true) }
=> warning: set: target '' ainda não suportado
   hint: targets suportados: heading, page, figure, text, par, table, smartquote
```

Este aviso genérico é ele próprio uma divergência do vanilla, que nunca o
produz para `format.pdf`. No vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
medido via `typst eval` em 2026-09-20:

```text
{ set format.pdf(tagged: true, pretty: true) }        -> aceito, sem diagnóstico
{ set format.pdf(nope: 1) }                            -> unexpected argument: nope
{ set format.pdf(true) }                                -> unexpected argument
{ set format.pdf(tagged: true, tagged: false) }         -> duplicate argument: tagged
{ set format.pdf(tagged: 1) }                           -> expected boolean or auto, found integer
{ set format.pdf(pretty: 1) }                           -> expected boolean, found integer
```

O diagnóstico de tipo de `tagged` difere do padrão `svg`/`png`/`html`: aceita
`auto` além de `bool`, logo a mensagem de tipo é
`expected boolean or auto, found <tipo>`, nunca a forma de dois tipos usada
pelos irmãos somente-booleanos. `format.pdf` não é feature-gated (P1464); os
dois campos existem e são aceitos nos quatro perfis, sem o gate `html`/
`a11y-extras` que `format.html` exige.

### Contrato

Reconhecer estruturalmente somente `format.pdf`. Aceitar zero, um ou dois
named dentre `tagged` e `pretty`, em qualquer ordem; positional, named
desconhecido e named duplicado falham como nos irmãos já materializados. Para
cada named presente:

- `tagged`: aceitar `Value::Auto` ou `Value::Bool`; qualquer outra variante
  falha com `expected boolean or auto, found <tipo>`. Valor validado insere
  `Value::Auto` ou `Value::Bool` sob a chave canônica `format.pdf.tagged`.
- `pretty`: aceitar somente `Value::Bool`; outra variante falha com
  `expected boolean, found <tipo>` (mesma forma de `format.svg.pretty`/
  `format.html.pretty`). Valor validado insere `Value::Bool` sob
  `format.pdf.pretty`.

Os dois campos são independentes: uma regra que só nomeia um deles não insere
chave para o outro e não apaga decisão cascaded anterior desse outro campo.
Regra vazia (`#set format.pdf()`) não insere nenhuma chave. Named
desconhecido, positional ou duplicado falham antes de qualquer inserção em
`StyleChain`, pelo canal normal de `SourceResult`, nunca warning nem descarte
silencioso — isto também substitui o aviso genérico `target '' ainda não
suportado` hoje emitido para `format.pdf`, que deixa de ser alcançado por
este target.

Este owner não extrai opção documental, não decide top-level/container e não
exporta PDF. A fronteira top-level e a mensagem de container continuam em
`compiler/layout/format_options.md`; a resolução de precedência CLI/documento
continua em `infra/pipeline.md`. Este owner não reabre a descoberta/namespace
de `format.pdf` já materializada em P1464 (`compiler/stdlib/format.md`), nem
`pdf.pages`/`pdf.standard` (P1468/P1469).

Preservar integralmente `format.svg`, `format.png`, `format.html` (incluindo
seu gate de feature) e a ordem/política de named desconhecido das demais set
rules.

### Aceitação e gate

Cobrir os quatro perfis: `tagged` com `auto`/`true`/`false`, `pretty` com
`true`/`false`, os dois juntos e cada um isolado, regra vazia neutra,
repetição/inversão lexical, os cinco negativos de schema (`nope`, positional,
`tagged` duplicado, `tagged: 1`, `pretty: 1`) e ausência do aviso genérico de
target não suportado. Este é um dos quatro L0s do gate ADR-0127 de P1467
(campo público novo e mudança de comportamento por defeito).

A confirmação humana explícita posterior ao commit causal somente-L0
`e675e8b7acc29e272bd59939e44681a7b521acd3` foi recebida em 2026-09-20.
Agente A implementou o braço `format.pdf` sem ver os testes; Agente B
escreveu 15 testes comportamentais a partir do mesmo baseline sem ver a
implementação (4/15 já passavam por coincidência mecânica — deteção
genérica de duplicado e regra vazia; os 11 restantes exigiam o braço). O
teste independente passou de RED (11/15 falhas) a GREEN (15/15) após a
integração, sem regredir `format.svg`/`format.png`/`format.html`.

## P1468 — named `pages` em `format.pdf(...)` (MATERIALIZADO; gate ADR-0127 satisfeito)

### Medição anterior à decisão

No baseline `0b322a926dd2f86b49aabfeb0d2f287ce06aaace`, o braço `format.pdf`
(P1467) só aceita `tagged`/`pretty`. No vanilla ratificado, `pages` aceita
`i64` (via `cast! { PageRange, nr: i64 => Self::at(parse_page_number(nr)?) }`,
`typst-library/src/layout/page.rs:813-821`), `Dict` com `from`/`to`
(`:822-836`), ou `Array` de qualquer mistura desses dois
(`typst-library/src/layout/page.rs:764-772`, `cast! { PageRanges, array: Array
=> ... }`), ou `none` (default). Medido via `typst eval` em 2026-09-20:

```text
{ set format.pdf(pages: 1) }                          -> aceito
{ set format.pdf(pages: (from: 1, to: 3)) }            -> aceito
{ set format.pdf(pages: (from: 2)) }                   -> aceito (aberto no fim)
{ set format.pdf(pages: (to: 2)) }                     -> aceito (aberto no início)
{ set format.pdf(pages: (1, (from: 3, to: 5))) }       -> aceito (array misto)
{ set format.pdf(pages: ()) }                          -> aceito (array vazio = nenhuma seleção extra)
{ set format.pdf(pages: (:)) }                          -> page range must not be empty
{ set format.pdf(pages: (from: 1, extra: 2)) }          -> unexpected key "extra", valid keys are "from" and "to"
{ set format.pdf(pages: (from: 5, to: 2)) }             -> start of page range cannot be larger than end
{ set format.pdf(pages: 0) }                            -> page numbers start at one
{ set format.pdf(pages: -1) }                           -> page numbers start at one
{ set format.pdf(pages: 1.5) }                          -> expected integer, dictionary, array, or none, found float
{ set format.pdf(pages: "1") }                          -> expected integer, dictionary, array, or none, found string
{ set format.pdf(pages: auto) }                          -> expected integer, dictionary, array, or none, found auto
{ set format.pdf(pages: (1, "x")) }                      -> expected integer or dictionary, found string
```

Note a mensagem de tipo de topo (`"expected integer, dictionary, array, or
none, found <tipo>"`) difere da mensagem de elemento de array
(`"expected integer or dictionary, found <tipo>"`, sem `array`/`none` porque
um elemento de array não pode ser ele próprio array/none). `pages` não é
feature-gated e coexiste nos quatro perfis.

### Contrato

Estender o braço `format.pdf` (P1467) para aceitar também o named `pages`,
independente de `tagged`/`pretty` (três campos, cada um com seu próprio
`seen_*`). Validar o valor recebido contra o domínio `PageRanges` de
`entities/document_format_options.md`, com o MESMO cast em três níveis:

- inteiro (`Value::Int`) → `PageRange` de página única, `0` ou negativo falha
  com `"page numbers start at one"`; outro tipo (float/string/bool) falha com
  a mensagem de topo;
- `Value::Dict` com `from`/`to` → `PageRange` de intervalo; dict vazia falha
  com `"page range must not be empty"`; chave extra falha com
  `"unexpected key \"{key}\", valid keys are \"from\" and \"to\""`; `from >
  to` falha com `"start of page range cannot be larger than end"`;
- `Value::Array` → cada elemento é um inteiro OU dict (mesmo cast acima,
  recursivo um nível); elemento de outro tipo falha com a mensagem de
  elemento (sem `array`/`none`); array vazio é `PageRanges` vazia (equivalente
  a nenhuma seleção adicional, mas **presente** — distinto de `none`, que
  continua a chave ausente);
- `Value::None` → regra explicitamente "todas as páginas"; insere
  `Value::None` na chave (distinto de omitir o named, que não insere nada).

Este owner faz a validação COMPLETA no momento do `set` (todos os diagnósticos
acima, incluindo `from > to`, chave extra e `0`/negativo) — os erros medidos
ocorrem ao avaliar a expressão, não só na exportação. Depois de validado,
insere `Value::Int`/`Value::Dict`/`Value::Array`/`Value::None` tal como
recebido (o `Value` nativo já suporta essas variantes; não é necessário um
tipo customizado no `Styles`) sob a chave canônica `format.pdf.pages`. A
construção do `PageRanges` tipado a partir desse `Value` já validado é feita
por `compiler/layout/format_options.md`; uma falha nessa segunda passagem
seria estado interno impossível (o valor já passou pela validação aqui) e
falha fechada, nunca um novo diagnóstico de schema.

Preservar integralmente `tagged`, `pretty`, `format.svg`, `format.png`,
`format.html` e a política de named desconhecido/duplicado. Este owner não
decide interação `pages`+`tagged` (isso é `infra/pipeline.md`), não sabe que
existe CLI e não exporta PDF.

### Aceitação e gate

Cobrir: inteiro único, dict aberto/fechado nos dois lados, array vazio/misto,
`none` explícito, os oito negativos de schema medidos acima, independência
face a `tagged`/`pretty`, e não regressão dos irmãos. Este é um dos L0s do
gate ADR-0127 de P1468.

A confirmação humana explícita posterior aos commits causais somente-L0
`9bb20b8e8`/`70d09eae2` foi recebida em 2026-09-20/21. Agente A implementou
o named `pages` sem ver os testes; Agente B escreveu 20 testes a partir do
mesmo baseline sem ver a implementação. O teste independente passou de RED
(compilava mas falhava em runtime, já que `rules.rs` não depende de tipos
ainda inexistentes noutros ficheiros) a GREEN (35/35 no módulo, incluindo os
15 pré-existentes de P1467) após a integração, sem regredir
`format.svg`/`format.png`/`format.html`.
