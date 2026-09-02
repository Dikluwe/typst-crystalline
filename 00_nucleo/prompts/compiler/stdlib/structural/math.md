# Prompt L0 — `compiler/stdlib/structural/math` — nativas de matemática
Hash do Código: 81441281

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/math.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/math/{accent,cancel,op,underover,style}.rs`. Co-mudança: P290-301 e P317 movem `native_accent`, `native_cancel`, `native_op`, `native_underover`, `op_value` e `make_math_module` como bloco.

---

## P1293 — captura triestatal dos seis slots de `attach`

### Medição anterior à decisão

O recibo causal SHA-256
`80a9543c9450f2350a42fa48df2e42cac63109bd074ebb37c2ec424cba473d5c`
mede `Value::None` distinto de `Value::Content(Content::Empty)` em `Args`, mas
`01_core/src/compiler/stdlib/structural/math.rs:637-644,696-703` converte ambos
em `Content::Empty` antes de envolvê-los em `Some`; esse é o primeiro ponto de
colisão. Omitido já é observável separadamente pela ausência no mapa named.

### Decisão confirmada

Para cada `t,b,tl,bl,tr,br`, ausência do named produz
`MathAttachSlot::Omitted`; `Value::None` produz `ExplicitNone`; conteúdo,
string ou símbolo validamente convertido produz `Present(content)`, inclusive
`Present(Content::Empty)`. Spans de argumentos, ordem de avaliação, casts,
mensagens, hints, precedência positional/named e o constructor puro
compartilhado por sintaxe/forma qualificada permanecem intactos.

É proibido usar `Content::Empty` como sentinel, introduzir heurística nominal,
alterar `Args`, `Value`, parser/AST, defaults ou fase. Esta mudança materializa
o contrato público confirmado em `2026-09-02T08:06:37-03:00`; todo estado deve
chegar ao constructor canônico sem colapso.

## Contexto

As nativas de matemática que vivem no stdlib estrutural — os construtores de `Content`
matemático e a montagem do módulo `math`. O **layout** destes elementos é outro eixo
inteiro (`compiler/math/layout/`); este nó só constrói o conteúdo.

**Técnica**: `make_math_module` monta um `Value::Module` com scope fechado de
funções, operadores, espaçamentos e símbolos math. A tabela é construída uma
vez; não há lookup dinâmico por registry.

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

### P1293 — `math.attach`, `math.binom`, `math.mono`, `math.script` (GATE ADR-0127)

**Medição anterior à decisão:** em `2026-09-01T13:24:01-03:00`, sobre
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não commitada
registrada em `p1293-baseline-status.txt`, o recibo independente mediu no
vanilla ratificado as quatro identidades curtas e a fonte
`typst-library/src/math/mod.rs:42-92`. `math/attach.rs:19-49` usa o payload já
existente de sete slots; `math/frac.rs:133-150` exige `upper` e um `lower`
posicional variádico não vazio; `math/style.rs:134-145,208-227` define `mono`
e `script`. `math/ir/resolve.rs:402-415,470-570,714-768` confirma que attach e
binom permanecem na fase/payload vigentes. Não foi medido campo, entidade,
default de produto ou fase nova.

Antes do espelho `sym -> math`, `make_math_module` registra exatamente:

| binding | assinatura fechada | construção canônica |
|---|---|---|
| `math.attach` | `attach(base, t:?, b:?, tl:?, bl:?, tr:?, br:?)` | `Content::MathAttach` de sete slots |
| `math.binom` | `binom(upper, ..lower)` com pelo menos um lower | `MathFrac(line:false)` entre parênteses esticados |
| `math.mono` | `mono(body)` | a mesma `native_mono` global |
| `math.script` | `script(body, cramped:true)` | a mesma `native_script` global |

`attach` aceita `Content` em `base` e em cada slot. Base é obrigatório; segundo
positional e named desconhecido são erros. Omitir um slot é ausência. `none`
explicitamente fornecido é morfologia presente (`t: none` no `repr`) e deve ser
transportado como conteúdo vazio presente, não colapsado para slot omitido nem
texto; seu layout é equivalente ao da omissão. A ordem pública dos slots é
`t`, `b`, `tl`, `bl`, `tr`, `br`.

`binom` aceita `upper` como primeiro Content e um ou mais Content posicionais
em `lower`, preserva sua ordem e vírgulas na tupla morfológica, usa sempre
`MathFracElem.line = false` e delimitadores extensíveis `(`/`)`. Zero
argumentos reporta `missing argument: upper`; somente upper reporta
`missing argument: lower`; tipo inválido, positional indevido e named
desconhecido conservam mensagem e span do argumento medido. Não usar Matrix,
não desenhar barra e não criar payload `Binom` paralelo.

`mono` e `script` reutilizam diretamente as function pointers donas em
`stdlib/math_style.rs`, sem wrapper ou segunda regra. Ambos exigem body Content
posicional; `script` aceita somente `cramped: bool`, default `true`. Missing,
extra, body named, named desconhecido e cast inválido são erros fechados. A
identidade no módulo é `mono`/`script`, independentemente do reuso da nativa.

Os constructors puros de attach/binom são a fronteira comum com a sintaxe do
owner `compiler/eval/math.md`; ambos os caminhos devem produzir a mesma
morfologia e semântica, sem executar layout. `Unknown` nunca satisfaz qualquer
vetor requerido. As quatro adições são superfície pública e ficam bloqueadas
pelo gate humano P1293 antes do código.

### P1293.reopen-B-independent-RED — positional named e hints

#### Medição anterior à decisão

O julgamento independente posterior ao recibo final B SHA-256
`d677c0b1e6d8796c6680787d27b3409c100ff13653ab8ff89d7154813866720c`
confirmou para `binom(upper: ...)` a mensagem principal correta, mas rejeitou
o diagnóstico porque faltava o hint exato `try removing upper:`. O recibo
vanilla independente SHA-256
`39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7`
mede em `p1293-vanilla-measurement-receipt.md:162-186` que `attach.base` e
`binom.upper` são posicionais e que `lower` é requerido, variádico e
posicional. Este owner, em `:162-180`, fecha aridade/casts e named, mas não
fixava os hints dos campos posicionais reconhecidos.

Medido: mensagem e hint pertencem ao transcript diagnóstico observável.
Inferência: a lacuna cabe na validação de argumentos deste owner, sem tocar
constructor/payload. Refutam-na mensagem ou hint vanilla diferente para os
campos enumerados, mudança de precedência/span, ou necessidade de alterar
`Args`, call dispatch, entidade, fase ou layout.

#### Decisão estreita

Quando um campo posicional reconhecido é escrito como named:

- `math.attach(base: ...)` emite exatamente a mensagem
  `the argument base is positional` e o hint separado
  `try removing base:`;
- `math.binom(upper: ...)` emite exatamente a mensagem
  `the argument upper is positional` e o hint separado
  `try removing upper:`.

O hint não integra a mensagem principal e não substitui o erro. Slots
`t,b,tl,bl,tr,br` continuam named válidos; named realmente desconhecido
continua `unexpected argument: <nome>` sem hint posicional inventado. O lower
variádico conserva a assinatura e o comportamento medidos; esta reabertura
não infere novo diagnóstico para uma forma não enumerada.

Mensagem/hint são observáveis da linguagem (ADR-0107) e esta é correção
interna de paridade em fluxo contínuo (ADR-0127), sem novo gate humano.
Constructor, payload, casts, defaults, spans, precedência e quantidade/ordem
de avaliação permanecem idênticos. Não se altera API pública, `Args`,
entidade, compatibilidade ou fase eval/layout.

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

`math.root` é `MISSING_MEMBER`, não `WRONG_KIND`, e fica fora de P1140.3-A até
passo próprio classificá-lo.

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
