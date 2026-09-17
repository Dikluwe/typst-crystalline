# Prompt L0 — `compiler/eval/rules`
Hash do Código: d7224acd

Núcleos Tekt:
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
As fixtures e recibos `p1339-full-show-final-*` medem aplicação real de
strong/emph/text no vanilla ratificado `a51e02804`; os recibos fixam binários,
working tree e UTC. A limitação CLI dos perfis cristalinos está separada em
`p1339-full-a2.md` e não constitui evidência positiva de matching.

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
por aplicação extra. A extensão de dados foi aprovada no P1339; selo e RED
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
