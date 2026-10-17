# Prompt L0 — `compiler/stdlib/structural/math` — nativas de matemática
Hash do Código: a1b07b65

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml sha256:19eb838b3e56711fa151d416bd24394a0a6f5c8286bb9ab2d0dd9bc542b4aee7

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

**Alvo atual P1495/P1496:** `9f2b6e8715237cb086899a42873660fe744622e8`;
o pin `586e1bd43` acima é proveniência histórica das secções anteriores.

---

## Ownership dos slots de `attach`

A norma triestatal e o pin do Núcleo Tekt pertencem agora ao owner exclusivo
`compiler/stdlib/structural/math/attach.md`. Este parent mantém apenas o
binding estático no catálogo.

## Contexto

As nativas de matemática que vivem no stdlib estrutural — os construtores de `Content`
matemático e a montagem do módulo `math`. A família `math.vec` tem owner exclusivo
`00_nucleo/prompts/compiler/stdlib/structural/math/vector.md`; este prompt
retém somente o binding no catálogo. O **layout** destes elementos é outro eixo
inteiro (`compiler/math/layout/`); este nó só constrói o conteúdo.

**Técnica**: `make_math_module` monta um `Value::Module` com scope fechado de
funções, operadores, espaçamentos e símbolos math. A tabela é construída uma
vez; não há lookup dinâmico por registry.

## Atomização da família `math.vec`

**Medição antes da decisão:** no working tree não commitado capturado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-vector-2026-09-30-before.json.gz`,
`math.rs:724-893` contém `MathVecOptions`, casts de `delim`/`align`/`gap`,
construção e nativa; `:2401-2422` contém o teste de conversão de `gap`.
O filtro `cargo test -p typst-core math_vec --lib --offline --locked --quiet`
passa 3 testes. Esta é uma medição estrutural, não um benchmark nem nova
sonda de paridade com vanilla.

**Decisão:** mover essa unidade e seu teste para `math/vector.rs`, com Prompt
L0 exclusivo `math/vector.md`. Este owner mantém a montagem estática do módulo
`math`, incluindo o binding `math.vec`, sem copiar casts ou constructor. O
movimento preserva assinatura da nativa, ordem de argumentos, spans,
diagnósticos, bits explícitos e identidade de `Content::MathVec`; não altera
fase, default ou contrato público. A direção dos imports permanece na L1.

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

## Delegação das famílias restantes — 2026-09-30

**Medição anterior à decisão:** o working tree não commitado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz`
contém casts, builders, nativas e o catálogo estático em `math.rs:48-1685`;
a suíte core baseline passa 6.401 testes, com 2 ignorados. Isto mede
estrutura/regressão, não tempo nem nova paridade bilateral.

**Decisão:** os owners filhos `casts`, `underline`, `lr`, `mid`,
`annotations`, `primes`, `overline`, `norm`, `round`, `abs`, `stretch`,
`attach`, `binom`, `underover`, `op` e `limits` assumem os casts, builders
e nativas respectivos. Este parent conserva o catálogo estático, o espelho
`sym → math`, os imports de composição e os testes próprios. As secções
históricas abaixo preservam evidência e contrato de linguagem; o detalhe
materializável de cada família pertence ao filho indicado. `op_value`
é construído no filho `op` e apenas chamado pelo catálogo. Preservar
identidade, dispatch estático, assinaturas, diagnósticos e fases.

## Instrução

| Nativa | Assinatura | Emite |
|---|---|---|
| `accent` | `accent(base, accent)` — ambos posicionais obrigatórios | `Content::MathAccent` |
| `cancel` | `cancel(body)` | `Content::MathCancel` |
| `class` | `class(class, body)` | `Content::MathClassOverride` |
| `underover` | `underover(base, under: ?, over: ?)` | `Content::MathUnderover` |
| `op` | `op(text, limits: false)` | `Content::MathOp` |

### Delegação do cast de `math.class` — 2026-09-30

**Medição anterior à decisão:** no working tree não commitado capturado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-class-2026-09-30-before.json.gz`,
`math.rs:1069-1152` contém a nativa e o parent mantém o registro estático.
O hub reexporta a nativa via parent. Os filtros `math_class` e P825 passam
29 e 15 testes, respetivamente. Isto mede estrutura e regressão local;
não é benchmark nem nova sonda bilateral.

**Decisão:** casts, diagnósticos e construção de `math.class` passam ao owner
exclusivo `compiler/stdlib/structural/math/class.md`; este parent retém o
registro estático e o helper compartilhado `vanilla_type_name_class`.
Preservar caminho público e comportamento.

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

**Medição anterior à decisão:** o registro fresco P1292 confirma que os três
bindings são funções próprias do módulo `math`. Cancel já possui payload e
runtime P1291, mas ainda não está exposto nesse namespace; underline/vec ainda
não possuem consumers próprios. `make_math_module` deve registrar, antes do
espelho de símbolos:

| binding | assinatura de linguagem | identidade emitida |
|---|---|---|
| `math.cancel` | `cancel(body, length: 100% + 0.3em, inverted: false, cross: false, angle: auto, stroke: auto, background: false)` | `Content::MathCancel` já materializado |
| `math.underline` | `underline(body)` | `Content::MathUnderline` |
| `math.vec` | `vec(..children, delim: "(", align: center, gap: 0.2em)` | `Content::MathVec` |

**Medição estrutural 2026-09-30 anterior à decisão:** o snapshot em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-cancel-2026-09-30-before.json.gz`
mostra o corpo completo de `native_cancel` em `math.rs:395-529`, o binding
estático neste parent e a reexportação no hub; o filtro `native_cancel` passou
4 testes. A construção, os casts e a morfologia de `cancel` passam ao owner
`compiler/stdlib/structural/math/cancel.md`; aqui fica o registro estático.
A medição não é benchmark nem nova comparação bilateral com vanilla.

`underline` aceita um único body, sem named, e é identidade math
distinta do underline textual. A construção e os casts de `vec` pertencem ao
owner `compiler/stdlib/structural/math/vector.md`.

Os diagnósticos de `cancel` são propriedade do owner filho. Para
`underline`, falta de body produz `missing argument: body` e named
desconhecido produz `unexpected argument: <nome>`.

Os owners dos payloads são `entities/elements/math_cancel.md`,
`entities/elements/math_underline.md` e `entities/elements/math_vec.md`; o enum
é `entities/content.md`; syntax sugar é `compiler/eval/math.md`; geometria é
`compiler/math/layout/cancel.md`, `compiler/math/layout/underline.md` e
`compiler/math/layout/vec.md`. Este módulo somente faz cast, cascata de estilos
e construção.

O runtime de callback de cancel permanece o validado em P1291. A nativa
`math.vec` conserva a sua entrada no catálogo estático; os detalhes de cast,
construção e transporte pertencem ao seu owner. Esta unidade não duplica
layout nem assa região.

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

**Medição estrutural 2026-09-30 anterior à delegação:** o snapshot em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-root-2026-09-30-before.json.gz`
mostra `sqrt_content` e as duas nativas no parent, o bridge no hub e 12
testes P1426 verdes antes do movimento. Não é benchmark nem nova sonda
bilateral do vanilla. Casts, diagnósticos e carriers passam ao owner
`compiler/stdlib/structural/math/root.md`; aqui ficam os registros estáticos.

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

O módulo registra a nativa do owner filho que constrói `Content::math_root`. Presença do
primeiro argumento é sempre verdadeira na chamada qualificada, inclusive
quando seu valor é `none`; a nativa não executa layout. `math.sqrt` continua a
mesma função pública de um argumento e constrói presença falsa.

Esta materialização foi confirmada explicitamente pelo dono em 2026-09-16.

## Delegação de `math.equation` — 2026-09-30

**Medição anterior à decisão:** o working tree não commitado capturado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-equation-2026-09-30-before.json.gz`
mostra `equation_content`, os três casts de named e a nativa em
`math.rs:46-210`; o hub reexporta quatro helpers via parent e o filtro
P1140 passa 85 testes. Isto é medição estrutural, não benchmark nem nova
sonda bilateral.

**Decisão:** manter neste parent somente o registro estático
`math.equation`. Os contratos de linguagem documentados nas secções P1140
abaixo permanecem vigentes, com implementação e ownership exclusivos em
`compiler/stdlib/structural/math/equation.md`. Preservar bridges e
visibilidade, sem mudar contrato, default, fase ou compatibilidade.

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

## P1427 — binding público `math.frac` e delegação

### Medição anterior à decisão

A medição P1427 constatou que o registry cristalino omitia `math.frac` e que
o vanilla exige `num` e `denom` posicionais, aceita somente `style:` no
domínio `vertical | skewed | horizontal`, converte string/`none` para
conteúdo e rejeita inteiros. Essa medição histórica não é uma nova sonda do
snapshot vanilla atual.

No working tree não commitado capturado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-frac-2026-09-30-before.json.gz`,
`math.rs:291-391` contém casts, validação, constructor e nativa; o registro
estático está no mesmo parent. O filtro P1427 passou 15 testes antes do
movimento. Esta medição estrutural não é benchmark.

### Decisão de ownership

Manter aqui somente o registro estático de `math.frac`. A validação de
argumentos, casts, diagnósticos e construção de `Content::MathFrac` pertencem
ao owner exclusivo `compiler/stdlib/structural/math/frac.md`. O binding
continua a usar a nativa do filho, com a mesma identidade pública. Não
mudar contrato, default, fase ou compatibilidade.

## P1429 — binding público `math.accent` e delegação

### Medição anterior à decisão

No working tree não commitado capturado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-accent-2026-09-30-before.json.gz`,
`math.rs:393-550` contém a tabela de aliases de combining marks, casts de
base/accent/size/dotless, constructor e nativa; `:2119` registra
estaticamente `math.accent`. O filtro
`cargo test -p typst-core p1429 --lib --offline --locked --quiet` passou 9
testes. Esta é medição estrutural, não benchmark nem nova sonda bilateral.

### Decisão de ownership

Manter aqui somente o registro estático de `math.accent` antes do espelho de
símbolos. O owner exclusivo
`00_nucleo/prompts/compiler/stdlib/structural/math/accent.md` especifica
aliases, argumentos, casts, explicitude, diagnóstico e construção de
`Content::MathAccent`. Os bridges atuais do stdlib continuam com os mesmos
nomes e visibilidades por reexport direto do filho. Não mudar contrato
público, default, fase ou compatibilidade.

## P1430 — binding público `math.cases` e delegação

### Medição anterior à decisão

No working tree não commitado capturado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-cases-2026-09-30-before.json.gz`,
`math.rs:724-942` possui `MathCasesOptions`, conversão de delimitadores,
casts, filhos e nativa; `:2367` registra estaticamente `math.cases`.
`math/matrix.rs:20,131` já chama a conversão de delimitadores deste bloco.
O filtro `cargo test -p typst-core p1430 --lib --offline --locked --quiet`
passou 22 testes nesse estado. Esta é medição estrutural, não benchmark nem
nova sonda bilateral.

### Decisão de ownership

Manter aqui somente o registro estático de `math.cases` antes do espelho de
símbolos. O novo owner exclusivo
`00_nucleo/prompts/compiler/stdlib/structural/math/cases.md` especifica
argumentos, casts, conversão de delimitadores, construção do carrier e
diagnósticos. `math/matrix.rs` chama diretamente o conversor desse filho;
não cria cópia da tabela de delimitadores. O hub estrutural mantém os nomes
do bridge de eval/layout. Não mudar contrato público, defaults, fase ou
compatibilidade.

## P1431 — binding público `math.mat` e delegação

### Medição anterior à decisão

No working tree não commitado capturado em
`00_nucleo/diagnosticos/atomizacao-stdlib-math-matrix-2026-09-30-before.json.gz`,
`math.rs:725-1042` possui `MathMatrixOptions`, casts, leitura de linhas,
constructor e nativa; `:2683` registra estaticamente `math.mat`. O filtro
`cargo test -p typst-core p1431 --lib --offline --locked --quiet` passou 29
testes nesse estado. Esta medição confirma um bloco de comportamento já
materializado; não é benchmark nem nova sonda bilateral com vanilla.

### Decisão de ownership

Manter aqui somente o registro estático de `math.mat` no módulo matemático,
antes do espelho de símbolos. O novo owner exclusivo
`00_nucleo/prompts/compiler/stdlib/structural/math/matrix.md` especifica
argumentos, casts, opções, interpretação de linhas, diagnósticos e construção
de `Content::MathMatrix`. A nativa é chamada diretamente desse filho. Não
mudar contrato público, default, fase, ordem, spans ou compatibilidade.

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

## P1441 — binding público completo `math.overline` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O módulo fechado omite `overline`. No vanilla, body é o único positional
obrigatório; string e `none` convertem para conteúdo, enquanto named,
excedente e tipo incompatível são rejeitados.

### Decisão proposta

Registrar `math.overline` antes do espelho de símbolos. A nativa avalia uma
vez, exige exatamente um body convertível em conteúdo e constrói
`Content::MathOverline`. Reproduzir spread e diagnósticos medidos; não criar
regra, aplicar cramped ou executar layout aqui.

## P1442 — binding público completo `math.overparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O módulo fechado omite `overparen`. No vanilla, body é posicional obrigatório
e annotation é o único segundo positional opcional; named e excedentes são
rejeitados. Omissão e `none` explícito permanecem distintos no `repr`.

### Decisão proposta

Registrar `math.overparen` antes do espelho de símbolos. A nativa avalia uma
vez, exige body convertível em conteúdo, aceita no máximo uma annotation e
constrói `Content::MathOverparen` com bit de presença. Reproduzir casts,
spread e diagnostics medidos; não criar U+23DC nem executar layout aqui.

## P1443 — binding público completo `math.overshell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O módulo fechado omite `overshell`. No vanilla, body é posicional obrigatório
e annotation é o único segundo positional opcional; named e excedentes são
rejeitados. Omissão e `none` explícito permanecem distintos no `repr`.

### Decisão proposta

Registrar `math.overshell` antes do espelho de símbolos. A nativa avalia uma
vez, exige body convertível em conteúdo, aceita no máximo uma annotation e
constrói `Content::MathOvershell` com bit de presença. Reproduzir casts,
spread e diagnostics medidos; não criar U+23E0 nem executar layout aqui.

## P1444 — binding público `math.primes` (MATERIALIZADO)

### Medição anterior à decisão

O módulo fechado omite `primes`. No vanilla, `count` é o único positional
obrigatório, inteiro não-negativo; zero é válido, spread singleton funciona e
named/excedentes/tipos incompatíveis são rejeitados.

### Decisão proposta

Registrar `math.primes` antes do espelho de símbolos. A nativa exige
exatamente um `Value::Int` não-negativo convertível para `usize` e constrói
`Content::MathPrimes`. Reproduzir spread e mensagens primárias medidas; não
criar glyphs nem executar layout no builder.

## P1445 — binding público completo `math.round` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O módulo fechado omite `round`. No vanilla ratificado, `round` é função
pública com `body: Content` posicional obrigatório e somente
`size: Rel<Length>` nomeado. Ela retorna `LrElem` cujo body é a sequência
`⌊`, conteúdo, `⌉`; a presença de `size` é conservada, inclusive quando o
valor explícito é `100%`.

### Decisão proposta

Registrar `math.round` antes do espelho de símbolos. A nativa exige exatamente
um positional convertível a conteúdo, aceita somente `size`, detecta ocorrência
duplicada e constrói o carrier P1436 `Content::MathLr` com cercas assimétricas
`⌊` e `⌉`. Ausência, extra, `body:`, named desconhecido e cast inválido
propagam os diagnósticos canônicos; spread válido é expandido sem reavaliação.
O builder não usa `MathDelimited`, não resolve cascata e não executa layout.
`math.abs`, `floor`, `ceil` e `bar` permanecem fora deste recorte.
## P1446 — binding `math.stretch` (PROPOSTO; gate ADR-0127)

Registrar função-elemento pública `stretch`. Aceita um único body posicional
castável a `Content` e `size: Rel<Length>` named opcional. Ausência de body,
body named, named desconhecido, duplicado, tipo inválido e argumento extra
falham como no vanilla ratificado. O default é `100%`, com
`size_explicit=false`; qualquer argumento fornecido, inclusive `100%`, liga a
presença. Chamada qualificada e bare reutilizam um único construtor validado e
produzem `MathStretch`, nunca aplicação genérica textual.

## P1447 — binding público completo `math.abs` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O módulo fechado omite `abs`. No vanilla ratificado, `abs` é função pública
com `body: Content` posicional obrigatório e somente
`size: Rel<Length>` nomeado. Ela retorna o carrier público `LrElem`, cujo body
é `sequence(|, body, |)`; a presença de `size` é conservada, inclusive quando
o valor explícito é `100%`, e `.func()` do resultado permanece `lr`.

### Decisão proposta

Registrar `math.abs` antes do espelho de símbolos. A nativa exige exatamente
um positional convertível a conteúdo, aceita somente `size`, detecta
ocorrência duplicada e constrói o carrier P1436 `Content::MathLr` com duas
barras `|`. Ausência, extra, `body:`, named desconhecido e cast inválido
propagam os diagnósticos canônicos; spread válido é expandido sem reavaliação.
O builder não cria `MathAbs`, não usa `MathDelimited`, não resolve cascata e
não executa layout. As demais rotas residuais P1422 ficam fora deste recorte.

## P1448 — binding público completo `math.underbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O módulo fechado omite `underbrace`. No vanilla, body é posicional obrigatório
e annotation é o único segundo positional opcional; named e excedentes são
rejeitados. Omissão e `none` explícito permanecem distintos em repr/fields.

### Decisão proposta

Registrar `math.underbrace` antes do espelho de símbolos. A nativa avalia cada
argumento uma vez, exige body convertível em conteúdo, aceita no máximo uma
annotation posicional e constrói `Content::MathUnderbrace` com bit de presença.
Reproduzir casts, spread e diagnósticos medidos; não criar U+23DF nem executar
layout aqui. `underbracket`, `underparen` e `undershell` ficam fora do recorte.

## P1449 — binding público completo `math.underbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O módulo fechado omite `underbracket`. No vanilla, body é posicional
obrigatório e annotation é o único segundo positional opcional; named e
excedentes são rejeitados. Omissão e `none` explícito permanecem distintos.

### Decisão proposta

Registrar `math.underbracket` antes do espelho de símbolos. A nativa avalia
cada argumento uma vez, exige body convertível em conteúdo, aceita no máximo
uma annotation posicional e constrói `Content::MathUnderbracket` com bit de
presença. Reproduzir casts, spread e diagnósticos medidos; não criar U+23B5
nem executar layout aqui. `underparen` e `undershell` ficam fora do recorte.

## P1450 — construtor público `math.underparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O módulo omite `underparen`; no vanilla body é positional obrigatório e
annotation o único segundo positional opcional. Named e excedentes falham;
omissão e `none` explícito diferem.

### Decisão proposta

Registrar `math.underparen`; a nativa avalia uma vez, aplica casts e
diagnósticos medidos e constrói `Content::MathUnderparen` com presença causal.
Não criar U+23DD nem executar layout aqui; `undershell` fica fora do recorte.

## P1451 — construtor público `math.undershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O módulo omite `undershell`; no vanilla body é positional obrigatório e
annotation o único segundo positional opcional. Named e excedentes falham;
omissão e `none` explícito diferem.

### Decisão proposta

Registrar `math.undershell`; a nativa avalia uma vez, aplica casts e
diagnósticos medidos e constrói `Content::MathUndershell` com presença causal.
Não criar U+23E1 nem executar layout aqui.

## P1466 — seleção explícita do primeiro named inválido

Nos validators que devolvem erro incondicional na primeira chave named, usar
`keys().next()` em vez de um loop que nunca itera além da primeira entrada.
Ordem de diagnóstico, mensagem, span e hint permanecem idênticos; named vazio
continua sem erro e nenhum argumento passa a ser aceito. Esta é correção
mecânica de análise estática, não mudança de assinatura matemática.

## P1472 — nove rotas qualificadas `math.*` ausentes (MATERIALIZADO)

### Medição anterior à decisão

Sonda bilateral repetida em 2026-09-21 contra o vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d` e o cristalino corrente
(`target/debug/typst`, build de working tree): `math.bold`, `math.cal`,
`math.display`, `math.italic`, `math.sans`, `math.scr`, `math.sscript`,
`math.upright` e `math.text` acedem todos a
`module math does not contain <nome>` no cristalino; todos `function` no
vanilla. Confirma sem deriva a medição original de `typst-passo-1472.md`
§2 (2026-09-20T17:40:24-03:00).

`01_core/src/compiler/eval/mod.rs:8226-8238` já regista as oito nativas
de estilo como globais bare (`Func::native("bold", native_bold)`,
`Func::native("cal", native_cal)`, ..., `Func::native("italic",
native_math_italic)`, `Func::native("upright", native_upright)`) — as
mesmas nativas de `compiler/stdlib/math_style.md` (P1291/P1466), sem
nenhuma mudança de comportamento necessária. `make_math_module()`
(`compiler/stdlib/structural/math.rs:2795`) já regista `equation`,
`sqrt`, `root`, `frac`, `accent`, `cancel`, `underline` e outros membros
próprios antes do espelho `sym → math` (que nunca sobrescreve binding já
definido) — mas nenhum dos oito estilos.

### `math.text` — medição dedicada (não é `Content::MathText`)

Sonda no vanilla ratificado (`/usr/local/bin/typst eval`) confirma que
`math.text` **é literalmente a mesma função** que o global `text`
(elemento de estilo de texto documental,
`01_core/src/compiler/stdlib/text/constructor.rs::native_text`, owner
`compiler/stdlib/text/constructor.md`), não um carrier matemático novo:

```text
math.text == text                              → true
type(math.text)                                 → function
repr(math.text)                                  → "text"
math.text("abc")                                 → mesmo repr/fields que text("abc")
math.text()                                       → "missing argument: body"
math.text(1)                                      → "expected content, found integer"
math.text(none)                                   → sequence vazio
math.text(font: "Arial")[hi].fields()             → (child: [hi], styles: styles(..))
```

Não é `Content::MathText` (o carrier interno de texto/símbolo em modo
math, usado por `apply_math_style` e o parser — inalterado, sem relação
com este passo) nem `native_inline`/`MathStyleKind::Inline` (que controla
tamanho matemático, não constrói texto). `entities/text.md`/
`compiler/stdlib/text/constructor.md` não mudam — `native_text` é
reutilizada tal como está, sem novo field, cast ou diagnóstico.

**Correção descoberta na integração (papel C, 2026-09-21):** a tabela
acima mede o **vanilla**; a decisão original rotulou as quatro últimas
linhas como "idêntico"/"idêntico a text()" no cristalino **sem medir o
`native_text` cristalino** — inferência não verificada, refutada pelo
Agente B durante a escrita dos testes RED. O `native_text` cristalino
atual (`text/constructor.rs`) diverge do vanilla nesses três pontos
específicos: corpo ausente → `"text() requer body como argumento"`
(não `"missing argument: body"`); tipo errado → `"text() espera content
ou string como body, recebeu <tipo>"` (não `"expected content, found
<tipo>"`); `text(none)` **erra** (`none` não casa `Content`/`Str` no
match do body) em vez de produzir sequência vazia. Isto é uma divergência
de paridade **pré-existente e fora do escopo de P1472** — pertence a
`compiler/stdlib/text/constructor.md`, não a este owner. **O contrato
central desta secção não muda**: `math.text` continua a reutilizar
`native_text` bit-a-bit, **qualquer que seja o seu comportamento atual**
— identidade de função (`math.text == text`) é o observável que P1472
fixa, não paridade vanilla de `native_text` em si. O Agente B testou
corretamente contra o oráculo Rust real (`native_text` invocada ao vivo),
não contra as strings vanilla acima — essa é a forma certa de evitar RED
por motivo errado, e o padrão a seguir caso `native_text` mude no futuro.

### Decisão

`make_math_module()` passa a inserir, **antes** do espelho `sym → math`
(mesma posição/princípio dos membros já registados: nunca sobrescrito
pela cópia):

```text
bold, cal, display, italic, sans, scr, sscript, upright, text
```

Os oito estilos reutilizam exatamente as nativas já existentes em
`compiler/stdlib/math_style.rs`, com o mesmo nome bare já usado pelos
globais (`Func::native("bold", native_bold)`, etc. — ver
`compiler/stdlib/math_style.md` P1472 para a nota de projeção). `text`
reutiliza `native_text` com o nome bare `"text"` (não `"math.text"`) —
`Func::PartialEq` compara nativas por nome (`entities/func.rs:384-397`),
logo o nome idêntico já garante `math.text == text`/`math.bold == bold`/
etc. sem exigir partilha literal do mesmo `Arc`; a forma exata da
construção (reconstrução com nome idêntico vs. passar o `Value` já
construído por referência, no padrão `make_format_module(pdf: &Value,
html: &Value)` de P1471) é decisão de implementação, não de contrato —
ambas satisfazem o observável medido.

Nenhum `MathStyleKind` novo, nenhum wrapper, nenhum despacho por string e
nenhuma segunda implementação de `text`. `display`/`sscript` conservam os
seus defaults `cramped` diferentes das restantes (ver tabela em
`math_style.md`) — não tratá-los como funções body-only equivalentes às
demais.

### Não-objetivos (P1472 não remove nada)

Os globais bare `bold`/`cal`/.../`upright` (função `CRYSTALLINE_ONLY` no
catálogo, por existirem também fora do módulo `math` no cristalino e não
no vanilla) **não são removidos** neste passo — essa é decisão de
compatibilidade reservada a P1474 (`typst-passo-1474.md`, ainda não
executado). P1472 só acrescenta os nove membros qualificados; um RED
ausente relativo aos extras globais não pode ser confundido com quebra
deste passo.

### Gate ADR-0127

As nove adições ampliam a superfície pública do módulo `math`. Requer L0
causal (este e `math_style.md` P1472), hashes ressealados, V5/V15/V26,
commit documental e **confirmação humana explícita posterior**, antes de
qualquer código.

Confirmação humana explícita recebida em 2026-09-21, posterior ao commit
causal somente-L0 `445cf0c81`. Agente A (`cbc3edf06`) inseriu os nove
bindings em `make_math_module()` (segundo loop análogo ao P1291 para os
oito estilos, mais um `dict.insert` para `text`) sem ver os testes —
usou `crate::compiler::stdlib::native_text` (reexport público já
existente) em vez do caminho privado `text::constructor::native_text`,
decisão de implementação compatível com o contrato (identidade por nome,
não por caminho de import). Agente B (`bc6126331`) escreveu 8 testes RED
sem ver a implementação, num novo módulo inline `p1472_tests` (mesmo
padrão de `p1291_tests`/`p1283_tests` já usado neste ficheiro, sem L0
próprio). Durante a escrita dos testes, o Agente B mediu diretamente o
`native_text` cristalino (em vez de confiar nas strings vanilla desta
secção) e descobriu a imprecisão corrigida acima ("Correção descoberta
na integração") — os 5 testes RED genuínos falharam todos por
`MISSING_MEMBER` (`"module math does not contain <nome>"`), nunca por
mensagem errada, confirmando que o achado não contaminou o RED. Suíte:
8/8. Zero regressão em typst-core (6067/6067), typst-shell (83/83),
typst-infra (995/995). Clippy comparado por contagem `(ficheiro,
mensagem)` contra o baseline `445cf0c81` em worktree isolada: zero
avisos novos.

## P1495 — espelho do catálogo pinado (completar em P1496)

### Medição anterior à decisão

No vanilla `9f2b6e871`, `typst-library/src/math/mod.rs:189-190` chama
`symbols::define_math`, que injeta `codex::SYM` em math
(`typst-library/src/symbols.rs:12-35`). O `codex` git
`2e43ab90079cf9f9fa670d421c8c38233143b376` declara os novos
`chess`, `keyboard` e `ordinal`; sondas P1495 confirmaram os 21 filhos
e 19 variantes idênticos em `sym`/`math` no vanilla. O cristalino
`structural/math.rs:3123-3131` copia apenas os valores do módulo
`sym` atual e não tem os três pais. O pin, hashes e RED estão no
diagnóstico P1495.

### Decisão

Em P1496, o espelho copia o catálogo novo integral de `sym` para
`math`, inclusive submódulos e Unicode/variants, sem sobrescrever
`sqrt`, `class`, `equation`, `op` ou outra nativa math própria.
Os doze aliases legados preservados por `compiler/stdlib/sym.md`
também aparecem em `math` com os mesmos valores, nunca apenas no
fallback bare. `math.registered` continua extensão separada.
`math.spacebar` conserva valor ␣ e emite o warning do binding
depreciado; a emissão é do owner
`compiler/eval/bindings/field_access.md`, não deste montador.
Este L0 não altera assinatura, carrier ou layout matemático.

## Integração condicional dos módulos de teste — 2026-10-13

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado. A duplicação cfg(test) ocorre em `01_core/src/compiler/stdlib/structural/math/p1425_tests.rs:5` / pai :52; `01_core/src/compiler/stdlib/structural/math/p1426_tests.rs:5` / pai :55; `01_core/src/compiler/stdlib/structural/math/p1427_tests.rs:5` / pai :58.
Fontes, L0, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/cfg-test-ci-2026-10-13/before.json.gz` e
`refinement-before.json.gz`. O Clippy pinado reproduz os atributos duplicados.
A tentativa de retirar a condição interna mantém os testes Rust, mas o linter
arquitetural pinado não propaga a condição do pai e acusa V2 nos helpers.

Neste owner, retirar apenas o atributo externo cfg(test) das declarações
listadas acima. Manter os caminhos, nomes e visibilidades dos módulos e seus
atributos path. A condição interna `#![cfg(test)]` de cada arquivo listado
continua a excluir integralmente o módulo do build de produção, inclusive
seus imports e helpers. Os consumers de teste e seus L0 permanecem intactos.
Não acrescentar dependências, lógica, API, modo padrão ou fase. Validar a
mesma lista de testes, suíte core, build sem cfg(test), Clippy e arquitetura.


## Identidade matemática nativa — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado; pin `9f2b6e8715237cb086899a42873660fe744622e8`.
`00_nucleo/diagnosticos/native-math-catalogue-2026-10-17/` fixa o estado
exato em before.json.gz: UTC, diff HEAD --stat e hashes de todos os consumers,
L0 e configuração protegidos. before-measurement.json.gz é reutilização
verificada da matriz final da etapa anterior: fontes e binário Tekt têm
identidade de hash com o baseline atual (baseline-reuse.json), não uma nova
execução. A matriz cobre os oito perfis de features e 3956 casos de funções;
26 rotas math ainda são recusadas em where/kind pelo Tekt e aceitas pelo pin.
before-batch.json.gz mede 227 casos detalhados de campos, exclusões, alias
parcial e counter, com comandos/binários/saídas e seus hashes.

batch-schema.json conserva blocos e SHA-256 das fontes primárias do pin,
file:line e declarações de ABI do Tekt; typst-macros/src/elem.rs:410–421
exclui internal/external de field_id. As anotações elem destas 26 funções
contêm Mathy e não Locatable. Isto é semântica/forma pública da linguagem,
não igualdade mecânica de Func nem inferência de intenção por render.

### Responsabilidade exclusiva e decisão

Este owner conserva a montagem estática existente e acrescenta somente
reexports diretos, limitados a `pub(in crate::compiler)`, das nativas abaixo.
Nenhum wrapper/algoritmo/schema é movido ao hub, nenhum binding é adicionado.
O alias interno class_for_metadata evita tocar o reexport público condicional
de teste já existente no parent. Os nomes completos efetivos são os da lista:

- `attach::native_math_attach`
- `binom::native_math_binom`
- `cases::native_math_cases`
- `class::native_math_class` como `native_math_class_for_metadata`
- `frac::native_math_frac`
- `limits::native_math_limits`
- `lr::native_math_lr`
- `mid::native_math_mid`
- `annotations::native_math_overbrace`
- `annotations::native_math_overbracket`
- `overline::native_math_overline`
- `annotations::native_math_overparen`
- `annotations::native_math_overshell`
- `primes::native_math_primes`
- `root::native_math_root`
- `limits::native_math_scripts`
- `stretch::native_math_stretch`
- `annotations::native_math_underbrace`
- `annotations::native_math_underbracket`
- `underline::native_math_underline`
- `annotations::native_math_underparen`
- `annotations::native_math_undershell`
- `vector::native_math_vec`

Accent/cancel e mat já têm pontes próprias e não são duplicados aqui.

### Aceitação e limites

Invariantes compartilhadas: consumir o Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml`; as
responsabilidades exclusivas acima definem o segmento de cada owner.

Correção interna de tabela/composição em fluxo contínuo ADR-0127. Nenhum campo,
trait, assinatura Rust externa, compatibilidade, default ou fase muda. L0
antes dos testes/código; RED funcional por identidade ausente, GREEN dos 26
casos com filtros e exclusões; controles de With, homônimo e helpers que não
são elementos. Validar antes/depois toda a matriz, preservando observáveis
fora das identidades autorizadas e aliases efetivos (accent/cancel globais
cristalinos incluídos, sem crédito de paridade se não existem no pin).

O sucesso de where/kind não certifica getters, síntese, indexação, show,
layout ou counter. Os controles counter permanecem explícitos e a sua
admissão separada não é alterada neste lote. As quinze rotas restantes de
formatos/HTML/PDF têm investigação própria; a colisão de callback dos formatos
não é resolvida por nomes. FG1/RM1/SY1/RG1 preservam escopos e gates.

Verificar ownership/V26 antes do resselo restrito aos cinco pares; build,
suíte completa, fmt, Clippy/Rustdoc estritos, linhagem e diagnóstico global
sem nova violação, sob MemoryMax12GiB/High10GiB/swap0/jobs1 sequenciais.
