# Prompt L0 — `compiler/stdlib/structural/math` — nativas de matemática
Hash do Código: 7521340d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/math.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanila ratificado**: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`typst-library/src/math/{accent,cancel,op,underover,style}.rs`. Co-mudança:
P290-301 e P317 movem `native_accent`, `native_cancel`, `native_op`,
`native_underover`, `op_value` e `make_math_module` como bloco.

---

## Captura triestatal dos slots de `attach`

Para cada `t,b,tl,bl,tr,br`, ausência do named produz
`MathAttachSlot::Omitted`; `Value::None` produz `ExplicitNone`; conteúdo,
string ou símbolo convertido produz `Present(content)`, inclusive conteúdo
vazio.

Spans, ordem de avaliação, casts, mensagens, hints e precedência permanecem
intactos. É proibido usar `Content::Empty` como sentinel ou inferir presença
por texto.

## Contexto

As nativas de matemática que vivem no stdlib estrutural — os construtores de `Content`
matemático e a montagem do módulo `math`. O **layout** destes elementos é outro eixo
inteiro (`compiler/math/layout/`); este nó só constrói o conteúdo.

**Técnica**: `make_math_module` monta um `Value::Module` com scope fechado de
funções, operadores, espaçamentos e símbolos math. A tabela é construída uma
vez; não há lookup dinâmico por registry.

## P1425 — binding qualificado `math.limits` (APROVADO no gate ADR-0127)

### Medição anterior à decisão

No vanilla ratificado, `math/mod.rs:43-78` registra `LimitsElem` como elemento
do módulo e `math/attach.rs` declara `limits(body, inline: true)`. Sondas
bilaterais registradas no Passo 1425 medem a forma omitida
`limits(body: [x])`, a forma explícita
`limits(body: [x], inline: false)` e os diagnósticos de missing, excesso,
posicionalidade, named desconhecido e cast booleano.

No baseline cristalino, `make_math_module` não registra `limits`, embora
`Content::MathLimitsOverride` e todo o layout P992 já existam. O carrier não
preserva a presença explícita do named `inline`; o owner da entidade P1425
acrescenta esse bit sem criar segundo elemento.

### Decisão proposta

Antes do espelho `sym → math`, registrar `math.limits` como função nativa de
nome público `limits`. Ela aceita exatamente um `Content` posicional
obrigatório e somente `inline: bool`, default `true`. Missing, excesso, body
como named, named desconhecido e tipo inválido seguem os diagnósticos medidos;
nenhum argumento é ignorado.

A nativa produz o mesmo `Content::MathLimitsOverride` da sintaxe bare, com
`limits:true`, valor `inline` e bit `inline_explicit` derivado da presença do
named. Não executa layout, não mede body e não duplica o runtime de attach.
`scripts` permanece a função já materializada com `limits:false` e presença
falsa. As outras 21 rotas residuais P1422 ficam fora de escopo.

Esta secção foi confirmada explicitamente pelo dono no gate P1425 em
2026-09-16 e autoriza somente o candidato delimitado acima.

## Instrução

| Nativa | Assinatura | Emite |
|---|---|---|
| `accent` | `accent(base, accent)` — ambos posicionais obrigatórios | `Content::MathAccent` |
| `cancel` | `cancel(body)` | `Content::MathCancel` |
| `class` | `class(class, body)` | `Content::MathClassOverride` |
| `underover` | `underover(base, under: ?, over: ?)` | `Content::MathUnderover` |
| `op` | `op(text, limits: false)` | `Content::MathOp` |

### `math.class` — as 10 classes

`class` é obrigatório e tem de ser **uma das 10 strings do cast do vanilla** (P772y,
P825a). Nome desconhecido produz a **mesma mensagem de cast** do vanilla; argumento
não-string reporta o tipo no formato longo, via `vanilla_type_name_class`
(`str → string`, `int → integer`, …). Variantes internas fora do cast são rejeitadas.

### `make_math_module()`

Constrói `math` como `Value::Module` com os **41 operadores** vanilla
pré-definidos, o elemento chamável `equation`, `op`, os cinco
espaçamentos nomeados (P895) e as funções math explicitamente registradas.
`dif`/`Dif` levam `upright` explícito — não itálico (§P962).

### P1396 — separador morfológico de `liminf` e `limsup`

**Medição anterior à decisão (2026-09-14, vanila ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`):**
`typst-library/src/math/op.rs:88-89` declara respectivamente os textos
`"lim\u{2009}inf"` e `"lim\u{2009}sup"`, com espaço fino Unicode U+2009.
Uma sonda CLI de `repr(math.liminf)` devolveu
`"op(text: [lim inf], limits: true)"` no vanila (exit 0, RSS máximo
18.260 KiB, zero swap), enquanto o cristalino no baseline
`3f081c97f7dba59549a70e89d6bfb93afcc82b1a` devolveu a variante com espaço
comum U+0020 (exit 0, RSS máximo 32.884 KiB, zero swap).

**Classificação e decisão:** o texto carregado pelo `MathOp` é morfologia
da língua (ADR-0107). A declaração nominal e explícita dos dois literais na
fonte ratificada sustenta a intenção; não se infere intenção apenas do
render. Esta conclusão seria refutada por outro literal na mesma revisão
pinada ou por uma sonda bilateral válida que produzisse a mesma sequência de
codepoints.

`make_math_module` deve registrar `liminf` como `lim\u{2009}inf` e `limsup`
como `lim\u{2009}sup`, preservando `limits: true`. Nenhum outro operador,
assinatura, layout, serializador, diagnóstico ou fase do pipeline muda. O gate
observa a representação de linguagem e os codepoints do texto; bytes de PDF
não são critério. Identidade ambígua, falha de parse/build, timeout, OOM,
swap ou saída ilegível permanecem `Unknown` e bloqueiam aceitação.

### P1283 — espelho estrutural integral `sym → math`

**Medição anterior à decisão (2026-08-30, vanilla `a51e02804`):** o módulo math
estende o scope de símbolos em `lab/typst-original/crates/typst-library/src/math/mod.rs:98-107`.
O catálogo `codex 0.3.0` pinado por P1283 contém 334 símbolos, 2 submódulos e 1.206
registos de valor/variant sob `sym`; o mesmo subgrafo público é observável sob `math`.
Estas são contagens de inventário, não percentagem de paridade.

Depois de registrar seus bindings próprios, `make_math_module` copia recursivamente o
scope completo construído pelo owner de `sym`, incluindo `gender` e `control`. A cópia
nunca sobrescreve binding math existente: `sqrt`, `class`, `equation` e `op` continuam
funções, com os contratos já definidos neste L0. Cada símbolo copiado preserva kind,
valor default, `repr`, aliases, modifiers, variants e clusters Unicode integrais.

`math.registered`, herdado de `sym.registered`, permanece extensão cristalina
individualizada, sem crédito de paridade e sem remoção automática.

### P1291 preservado — cinco bindings próprios já materializados

Antes do espelho `sym → math`, `make_math_module` registra estes cinco
bindings, que permanecem normativos em P1292:

| binding | assinatura de linguagem | identidade emitida |
|---|---|---|
| `math.bb` | `bb(body)` — um `Content`, sem named | `Content::MathStyled(DoubleStruck)` pela mesma nativa global |
| `math.frak` | `frak(body)` — um `Content`, sem named | `Content::MathStyled(Fraktur)` pela mesma nativa global |
| `math.inline` | `inline(body, cramped: false)` | `Content::MathStyled(Inline)` pela mesma nativa global |
| `math.scripts` | `scripts(body)` — um `Content`, sem named | `Content::MathLimitsOverride { limits:false, inline:true }` |
| `math.serif` | `serif(body)` — um `Content`, sem named | `Content::MathStyled(Plain)` pela mesma nativa global |

`bb`, `frak`, `inline` e `serif` reutilizam as mesmas funções já donas da
semântica no scope global; não são wrappers ou cópias. `scripts` é somente o
adapter privado do ABI para o constructor existente de
`MathLimitsOverrideElem`; uma `Sequence` comum não satisfaz sua política de
attachments.

Para os bindings body-only, ausência produz `missing argument: body`, valor
não-`Content` produz `expected content, found <tipo vanilla>`, excesso produz
`unexpected argument` e named desconhecido produz
`unexpected argument: <nome>`. `inline` aplica as mesmas regras ao body e aceita
somente `cramped: bool`; tipo inválido ou outro named é erro, nunca ignorado.

O scope continua fechado: nenhum outro binding global é copiado. Os cinco
mantêm nome público curto e são registrados antes do espelho, que nunca
sobrepõe binding math existente.

### P1292 — exposição canónica de cancel/underline/vec (GATE ADR-0127)

**Medição anterior à decisão:** o recibo fresco P1292 confirma que os três
bindings são funções próprias do módulo `math`. Cancel já possui payload e
runtime P1291, mas ainda não está exposto nesse namespace; underline/vec ainda
não possuem consumers próprios. `make_math_module` deve registrar, antes do
espelho de símbolos:

| binding | assinatura de linguagem | identidade emitida |
|---|---|---|
| `math.cancel` | `cancel(body, length: 100% + 0.3em, inverted: false, cross: false, angle: auto, stroke: auto, background: false)` | `Content::MathCancel` já materializado |
| `math.underline` | `underline(body)` | `Content::MathUnderline` |
| `math.vec` | `vec(..children, delim: "(", align: center, gap: 0.2em)` | `Content::MathVec` |

`cancel` valida `content`, `relative length`, `boolean`,
`angle|function|auto`, `stroke` e `boolean`; `none`/`auto` não são strokes
explícitos. A nativa apenas valida/cascateia e transporta para o mesmo
`MathCancelElem`; nunca mede body, executa callback ou cria segundo runtime.
Cada named presente marca seu bit morfológico, inclusive quando igual ao
default. `underline` aceita um único body, sem named, e é identidade math
distinta do underline textual. `vec` aceita filhos variádicos e os named
`delim`, `align` horizontal e `gap`; zero filhos é válido; named presentes
marcam os bits do payload.

Erros medidos e obrigatórios: falta de body em cancel/underline →
`missing argument: body`; named desconhecido →
`unexpected argument: <nome>`; `cancel.length` inteiro →
`expected relative length, found integer`; `cancel.angle` inteiro →
`expected angle, function, or auto, found integer`; `vec.gap` inteiro →
`expected relative length, found integer`; `vec.align` inteiro →
`expected alignment, found integer`; `vec.delim` inteiro →
`expected array, none, symbol, or string, found integer`. Nenhum argumento
reconhecido ou desconhecido é ignorado.

Os owners dos payloads são `entities/elements/math_cancel.md`,
`entities/elements/math_underline.md` e `entities/elements/math_vec.md`; o enum
é `entities/content.md`; syntax sugar é `compiler/eval/math.md`; geometria é
`compiler/math/layout/cancel.md`, `compiler/math/layout/underline.md` e
`compiler/math/layout/vec.md`. Este módulo somente faz cast, cascata de estilos
e construção.

O runtime de callback de cancel permanece o selado em P1291. `math.vec`
transporta `Rel<Length>` integral; a base percentual e a política
`height:auto` pertencem aos owners de layout. Esta unidade não duplica layout
nem assa região.

### `math.attach`, `math.binom`, `math.mono` e `math.script`

Antes do espelho `sym -> math`, `make_math_module` registra:

| binding | assinatura fechada | construção |
|---|---|---|
| `math.attach` | `attach(base, t:?, b:?, tl:?, bl:?, tr:?, br:?)` | carrier triestatal `MathAttach` |
| `math.binom` | `binom(upper, ..lower)`, com lower não vazio | `MathFrac(line:false)` entre parênteses |
| `math.mono` | `mono(body)` | mesma nativa global |
| `math.script` | `script(body, cramped:true)` | mesma nativa global |

`attach` exige base Content, preserva os seis slots na ordem pública e
compartilha o construtor puro com a sintaxe. `binom` preserva a ordem do lower
variádico, não desenha barra e não cria payload paralelo. `mono` e `script`
reutilizam as mesmas function pointers do owner de estilos.

Quando `base` ou `upper` é fornecido como named, o diagnóstico informa que
o argumento é posicional e inclui o hint separado `try removing <nome>:`.
Named realmente desconhecido continua `unexpected argument: <nome>`.
Missing, extra e casts permanecem fechados.

Os construtores somente validam e constroem conteúdo; não executam layout.

### P1140.3-A — `math.sqrt` função, não símbolo

**Medição anterior à decisão (2026-08-23, vanilla `a51e02804`):**
`repr(type(math.sqrt)) == "function"` e `repr(math.sqrt([x])) ==
"root(radicand: [x])"`. No cristalino anterior, `math.sqrt` era o símbolo `√`
importado de `sym`, logo não chamável; `sym.sqrt` também era um path extra que
o vanilla rejeita. O avaliador de modo math já construía corretamente
`Content::math_root(None, radicand)` por um braço sintático duplicado.

O módulo registra `math.sqrt` como `Value::Func` nativa antes da cópia dos
símbolos. A nativa aceita exatamente um `Content`, rejeita named args e emite
`Value::Content(Content::math_root(None, radicand))`. O modo math deve delegar
à mesma unidade sem duplicar o contrato de aridade. A entrada extra
`sym.sqrt` é removida da tabela cristalina; a cópia `sym → math` nunca
sobrescreve funções já registradas. `calc.sqrt` permanece numérico e separado.

`math.root` era `MISSING_MEMBER`, não `WRONG_KIND`, em P1140.3-A; P1426 abaixo
define o passo próprio que o classifica, aprovado no gate ADR-0127.

### P1426 — `math.root` qualificado (APROVADO no gate ADR-0127)

Medição no vanilla ratificado `586e1bd43` identifica `math.root` como função
com exatamente dois argumentos posicionais para chamadas bem-sucedidas. O
primeiro aceita `Content` ou `none`; o segundo exige `Content` (com coerção
vigente de `none` para conteúdo vazio). Named `radicand` é rejeitado como
posicional, named desconhecido é rejeitado e posicionais extra são erro.

O módulo registra uma nativa que constrói `Content::math_root`. Presença do
primeiro argumento é sempre verdadeira na chamada qualificada, inclusive
quando seu valor é `none`; a nativa não executa layout. `math.sqrt` continua a
mesma função pública de um argumento e constrói presença falsa.

Esta materialização foi confirmada explicitamente pelo dono em 2026-09-16.

### P1140.3-B — primeira fase pública de `math.equation`

**Medição anterior à decisão (2026-08-23, vanilla `a51e02804`):**
`repr(type(math.equation)) == "function"`; o caso mínimo representa como
`equation(body: [x])`, e `block: true` como
`equation(block: true, body: [x])`. Ausência de `body`, segundo positional e
named desconhecido são erros. O vanilla também aceita `numbering`,
`number-align`, `supplement` e `alt`.

P1140.3-B substitui o `Value::None` histórico por uma função/elemento chamável
que aceita exatamente um `Content` posicional obrigatório e `block: bool`,
default `false`, e produz `Content::Equation`. A nativa e a sintaxe `$...$`
partilham a construção dona da morfologia. Named args desconhecidos ou ainda
não materializados são rejeitados, nunca ignorados.

**Incompletude deliberada:** `numbering`, `number-align`, `supplement` e `alt`
ficam fora desta primeira fase e serão materializados pelo **P1140.4**, com
transporte, `repr`, `set`/`show`, referências e acessibilidade. O transporte já
vigente de `#set math.equation(numbering: ...)` pela style chain permanece.

### P1140.4-A — `numbering` no construtor

**Medição (2026-08-23, vanilla `a51e02804`):** o named arg aceita string,
função unária ou `none`; inteiro produz `expected string, function, or none,
found integer`. O `repr` preserva o valor. O default omitido continua sem
numeração.

`native_math_equation` aceita esse domínio e transporta o valor na style chain
da própria equação sob `equation.numbering`; não adiciona campo mecânico ao
`EquationElem`. O wrapper pertence somente ao conteúdo retornado pela chamada.
Named arg omitido não cria entrada; `none` cria entrada desativadora.

String e função são ativas somente para equações `block: true`. Funções não
são convertidas para string nem ignoradas: recebem o número inteiro depois do
fixpoint e seu retorno vira conteúdo para layout e referência. A execução
diferida reutiliza o mecanismo de callbacks pós-fixpoint; o layouter nunca
executa `Func`.

### P1140.4-B — `number-align` no construtor

**Medição (2026-08-24, vanilla `a51e02804`):** o named arg aceita um valor
`alignment`, mas o componente horizontal é restrito a `start | left | right |
end`; `center` produz `expected start, left, right, or end, found center` e
inteiro produz `expected alignment, found integer`. O componente vertical é
`top | horizon | bottom`. Um componente omitido é completado somente no
consumidor: horizontal omitido resolve para `end`, vertical omitido para
`horizon`. O default público é `end + horizon`, embora um valor explicitamente
passado apareça no `repr` na forma fornecida (`bottom`, `left + top` ou mesmo
`end + horizon`).

`native_math_equation` aceita `number-align: Align2D`, valida a restrição
horizontal e transporta o valor na style chain da própria equação sob
`equation.number-align`. O named omitido não cria entrada e herda o default ou
set-rule exterior. A entidade mecânica continua sem campo novo. O `repr`
preserva o named arg explícito, antes de `body`, e não materializa o default
quando ele foi omitido.

`#set math.equation(number-align: ...)` usa o mesmo cast e canal. Quando
`numbering` e `number-align` aparecem juntos, ambos são aplicados; o braço da
set-rule não pode retornar depois de processar apenas o primeiro campo.

## Restrições Estruturais

- L1 puro.
- As mensagens de erro de `class` são o observável (ADR-0108) — comparar verbatim, não
  por classe de erro.
- P1161: qualquer `Value::Symbol` convertido por este nó em conteúdo matemático
  usa o `EcoString` integral. `math` não exige um único scalar: a invariante é
  um grapheme cluster, já validado pelo owner `entities/symbol.md`.

## Critérios de Verificação

```
$accent(a, hat)$              → MathAccent
$cancel(x)$                   → MathCancel
$class("binary", x)$          → MathClassOverride
$class("zz", x)$              → Err (mesma mensagem de cast do vanilla)
$class(1, x)$                 → Err "found integer" (nome longo)
$op("lim", limits: true)$     → MathOp com limits
math.op existe no módulo      → true (P895)
repr(type(math.equation))     → "function"
repr(math.equation([x]))      → "equation(body: [x])"
repr(type(math.sqrt))         → "function"
repr(math.sqrt([x]))          → "root(radicand: [x])"
sym.sqrt                      → campo ausente
$dif$                         → upright, não itálico (§P962)
```

## P1132m — forma completa de `dif`/`Dif`

O binding reproduz `math/op.rs`: uma `Sequence` de `HSpace` absoluto
`Length::em(1/6)` com `weak=true`, seguido de `MathClassOverride(Unary)`
sobre o `MathStyled(italic:false)` já especificado por P962. A unidade `em`
torna o espaço proporcional ao tamanho matemático activo. O colapso nas
bordas pertence ao layouter (`compiler/math/layout/_comum.md` §P1132m).

## P1140.4-C — contrato público de `equation.supplement`

### Medição antes da decisão

No vanilla pinado `a51e02804`, `math/equation.rs:94-110` declara
`Smart<Option<Supplement>>`; `equation.rs:173-188` resolve `auto`, `none`,
conteúdo e função. Probes de 2026-08-24 confirmaram que constructor e set-rule
aceitam `content | function | none | auto`; string é castável para conteúdo.
Inteiro direto falha com `expected content, function, none, or auto, found
integer`. O cristalino rejeita o named em `structural/math.rs:92-126`.

### Decisão

`native_math_equation` aceita esse domínio e transporta o valor explícito sob
`equation.supplement` na style chain do conteúdo criado. O named omitido não
cria delta e equivale ao default `auto`; `auto` explícito continua observável
em `repr`. `#set math.equation(supplement: ...)` usa o mesmo cast/canal. Não se
adiciona campo a `EquationElem`.

A função recebe a equação referenciada como argumento único e seu resultado é
convertido para conteúdo pelo cast de display vigente, inclusive inteiro.
Erros propagam; nenhuma callback é executada no layouter.

## P1140.5-A — contrato público de `equation.alt`

### Medição antes da decisão

No vanilla pinado, `math/equation.rs:112-126` declara `Option<EcoString>`.
Probes de 2026-08-24 confirmaram domínio `string | none`, default ausente,
string vazia distinta e erros exatos `expected string or none, found content`
e `... found integer`. O cristalino rejeita `alt` como argumento inesperado.

### Decisão

`native_math_equation` aceita somente `Value::Str | Value::None` e transporta
o valor explícito em `equation.alt` na style chain local. Omitido não cria
delta. `#set math.equation(alt:)` usa o mesmo cast/canal. Não há conversão de
content para string nem descrição inferida do body.

## P1427 — binding público `math.frac` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O registry cristalino omite `math.frac`. O vanilla exige `num` e `denom`
posicionais, aceita somente `style:` nomeado no domínio `vertical | skewed |
horizontal`, converte string/`none` para conteúdo e rejeita inteiros.

### Decisão proposta

Registrar `math.frac` com exatamente dois posicionais. Ausência reporta
primeiro `num`, depois `denom`; terceiro posicional é inesperado; `num:` e
`denom:` informam que o argumento é posicional; named desconhecido e
duplicado falham. `style` pode anteceder os posicionais, conserva explicitude
inclusive para `vertical` e constrói o carrier completo P1427. Não aceitar nem
ignorar argumentos fora desse contrato.

## P1429 — binding público completo `math.accent` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

A nativa P296 existe mas não integra `make_math_module`, aceita conteúdo
arbitrário no segundo slot e rejeita todo named. O vanilla ratificado exige
dois posicionais, normaliza string/símbolo de um codepoint e aceita somente
`size: Rel<Length>` e `dotless: bool`, defaults `100%` e `true`.

### Decisão proposta

Registrar `math.accent` e fazer a fronteira construir o carrier P1429. `base`
converte string/none para conteúdo; `accent` aceita string de um codepoint ou
content de símbolo único, aplica a tabela canônica de combining marks e
rejeita qualquer outro conteúdo. Preservar presença independente de `size` e
`dotless`, inclusive quando iguais aos defaults. Aridade, positional-only,
named desconhecido, duplicado e cast inválido seguem os diagnósticos medidos;
nenhum argumento pode ser truncado ou ignorado.

## P1430 — binding público completo `math.cases` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O registry cristalino omite `math.cases`, embora sintaxe, carrier e layout
existam. O vanilla registra `CasesElem`: children `Content` variádicos, mais
`delim`, `reverse` e `gap` nomeados, com zero children válido.

### Decisão proposta

Registrar `math.cases` antes do espelho de símbolos. A nativa aceita qualquer
número de posicionais convertíveis a `Content` e somente `delim`, `reverse` e
`gap`; normaliza delimitadores opcionais, valida boolean/relative length e
preserva presença causal, ordem e duplicados. Ela constrói o carrier completo
P1430 sem executar layout ou separar `&`. A sintaxe bare delega à mesma
validação/construção. Named desconhecido ou `children:` são erros, nunca
ignorados; set rule fornece somente defaults cascaded e não marca explicitude.

## P1431 — binding público completo `math.mat` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O módulo fechado não registra `mat`. O vanilla ratificado aceita rows
posicionais variádicas e os named `delim`, `align`, `augment`, `gap`,
`row-gap`, `column-gap`; `rows:` é rejeitado. Escalares formam uma linha; se
qualquer positional for array, todos devem ser arrays e rows menores recebem
células vazias até à largura máxima.

### Decisão proposta

Registrar `math.mat` antes do espelho de símbolos e fazer a nativa construir o
carrier completo P1431. `gap` é shorthand causal para ambos os eixos e os
named específicos prevalecem. Delimitador, alinhamento horizontal, relative
length, augment inteiro/dict/none, offsets e stroke são validados sem truncar,
ignorar named ou reavaliar children. Duplicados e tipos inválidos propagam os
diagnósticos medidos. A nativa não executa layout nem separa `&`.

O dict de `augment` segue a assimetria pública do vanilla ratificado: consome
somente `hline`, `vline` e `stroke` e aceita/ignora chaves internas restantes.
Isso não autoriza tipo inválido nos três campos conhecidos e não se confunde
com named desconhecido no nível de `math.mat`, que continua erro.

## P1436 — binding público completo `math.lr` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O módulo fechado omite `lr`. O vanilla registra `LrElem` com posicionais
`Content` variádicos e somente `size: Rel<Length>` nomeado; zero posicionais
forma body vazio e múltiplos são unidos por conteúdo de vírgula. `body:` não é
campo nomeado público.

### Decisão proposta

Registrar `math.lr` antes do espelho de símbolos. A nativa avalia cada
ocorrência uma vez, converte posicionais para conteúdo, intercala a vírgula
matemática canônica e constrói `Content::MathLr`. `size` aceita relative
length, preserva presença inclusive em `100%` e participa da cascata somente
no layout. Rejeitar tipo inválido, named desconhecido, `body:` e duplicado;
spread válido é expandido pelo builder canônico, nunca ignorado. A nativa não
procura delimitadores nem executa stretch.

## P1437 — binding público completo `math.mid` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O módulo fechado omite `mid`. No vanilla, `MidElem` exige exatamente um
`body: Content` posicional; `body:`, positional extra, named desconhecido e
tipo não convertível são rejeitados. `none`, string e spread canônico seguem
cast/expansão geral para conteúdo.

### Decisão proposta

Registrar `math.mid` antes do espelho de símbolos. A nativa avalia uma vez,
exige exatamente um positional convertível em conteúdo e constrói
`Content::MathMid`. Rejeitar ausência, extra, named e tipo incompatível pelos
diagnósticos canônicos. Não classificar, resolver `lr` ou executar stretch.

## P1438 — binding público completo `math.norm` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O módulo fechado omite `norm`. No vanilla ratificado, `norm` é função pública
com `body: Content` posicional obrigatório e somente `size: Rel<Length>`
nomeado. Ela retorna `LrElem` cujo body é a sequência `‖`, conteúdo, `‖`; a
presença de `size` é conservada, inclusive quando o valor explícito é `100%`.

### Decisão proposta

Registrar `math.norm` antes do espelho de símbolos. A nativa exige exatamente
um positional convertível a conteúdo, aceita somente `size`, detecta ocorrência
duplicada e constrói o carrier P1436 `Content::MathLr` com as duas cercas no
body. Ausência, extra, `body:`, named desconhecido e cast inválido propagam os
diagnósticos canônicos; spread válido é expandido sem reavaliação. O builder
não usa `MathDelimited`, não resolve cascata e não executa layout.

## P1439 — binding público completo `math.overbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O módulo fechado omite `overbrace`. No vanilla, body é posicional obrigatório
e annotation é o único segundo positional opcional; named e excedentes são
rejeitados. Omissão e `none` explícito permanecem distintos no `repr`.

### Decisão proposta

Registrar `math.overbrace` antes do espelho de símbolos. A nativa avalia uma
vez, exige body convertível em conteúdo, aceita no máximo uma annotation e
constrói `Content::MathOverbrace` com bit de presença. Reproduzir casts,
spread e diagnósticos medidos; não criar U+23DE nem executar layout aqui.

## P1440 — binding público completo `math.overbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O módulo fechado omite `overbracket`. No vanilla, body é posicional
obrigatório e annotation é o único segundo positional opcional; named e
excedentes são rejeitados. Omissão e `none` explícito permanecem distintos no
`repr`.

### Decisão proposta

Registrar `math.overbracket` antes do espelho de símbolos. A nativa avalia
uma vez, exige body convertível em conteúdo, aceita no máximo uma annotation
e constrói `Content::MathOverbracket` com bit de presença. Reproduzir casts,
spread e diagnósticos medidos; não criar U+23B4 nem executar layout aqui.
