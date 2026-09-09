# Prompt L0 — `stdlib/calc` — subset trig/hiperbólicas/log/exp/constantes
Hash do Código: 30ef9f29

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/calc.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção de
assinatura e helpers partilhados: ver `stdlib/_comum.md`.
**Passo de origem do subset**: P283 (trig/log/exp + constantes).
**ADRs**: ADR-0018 (libm DEBT agregada), ADR-0101 (IEEE 754 — `guard_float`),
ADR-0054 (perfil graded).

---

## P1328 — diagnóstico de conteúdo em calc.abs

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1328-baseline.json`, SHA-256
`e049418db46ea039235b336254bfdbffd492253ac66c38782ac6bc56564f2ffb`,
registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado, diff/stat integral, binários e argv, com UTC inicial
`2026-09-09T11:35:18.901045+00:00`. O vanilla ratificado `a51e02804`
rejeita conteúdo em `calc.abs`, inclusive `$std.calc.abs(-1)$` e alias
bare em math, com a mensagem abaixo e o span do argumento. O baseline
P1327 usa mensagem portuguesa, span detached e, em código, trace adicional
devido à ausência de âncora. Conteúdo continua conteúdo; não vira número.

`01_core/src/compiler/stdlib/calc.rs:139-148` valida named/quantidade,
preserva Int/Float/Decimal e usa `err` detached no fallback.
`lab/typst-original/crates/typst-library/src/foundations/calc.rs:74-94`
declara o cast ToAbs; `01_core/src/compiler/eval/math.rs:329-348` já
transporta ocorrências com `value_span`. Args conserva essas origens em
`entities/args.rs:19-20,60-84`. `eval/call_dispatch.rs:1031-1044` já evita
trace redundante para erro contido na chamada; não é preciso alterá-lo.
`Value::Content` e `Value::LocatedContent` publicam o mesmo tipo content
em `entities/value.rs:348`; Symbol publica symbol e não pertence ao recorte.

### Decisão e aceitação

Somente depois dos guards existentes, para exatamente um argumento
posicional de tipo da linguagem content (Content ou LocatedContent),
retornar um erro de severidade Error, sem hints nem trace próprio, com texto:

```text
expected integer, float, length, angle, ratio, fraction, or decimal, found content
```

A âncora é `value_span` da primeira ocorrência posicional disponível em
Args. Preservar essa origem, inclusive em alias, With e spread; não usar
span agregado, nome da função, texto do conteúdo, pesquisa textual ou
origem adivinhada como substitutos. Sem ocorrência, ou com value_span
detached, manter detached no erro nativo. Não fabricar posição usando
`args.span` ou `occurrence.span`. Traces de chamadas externas continuam
responsabilidade do dispatcher vigente e são observados integralmente.

Preservar Int/Float/Decimal, salvo o overflow inteiro substituído em P1330,
o guard de named antes da aridade e erros de zero/múltiplos argumentos.
P1329, abaixo, substitui explicitamente a antiga rejeição de
Length/Angle/Ratio/Fr; os demais tipos, outras funções
calc e helpers compartilhados continuam fora desta correção de conteúdo.
As diferenças remanescentes são dívidas medidas, não paridade. A lista de
tipos no diagnóstico não implica suporte irrestrito: comprimentos mistos
têm a restrição normativa especificada em P1329.

Correção diagnóstica interna ADR-0127 em fluxo contínuo: L0-first,
testes independentes no próprio owner, RED real e GREEN dos mesmos casos.
Cobrir rotas math qualificada/bare sem mudança de fase, conteúdo markup,
alias/With/spread, origens distintas, UTF-8/linhas, wrappers de conteúdo,
argumento sintético sem origem, perfis default/html/a11y/html+a11y e controles
numéricos/de guard/outros tipos. Comparar mensagem, severidade, hints,
traces, quantidade e span resolvível, não apenas substring. CLI integral
normal/repetida/invertida e suíte completa são gates; nenhuma expectativa
histórica é alterada sem nova medição. É inferência que este owner basta;
origem ausente nas rotas reais obrigatórias, outro consumer necessário ou
mudança de fase refuta a suficiência antes de ampliar escopo. Unknown não
satisfaz aceitação. Não se afirma paridade geral de abs ou calc.

## P1329 — valores dimensionais em calc.abs

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1329-baseline.json`, SHA-256
`d0e1787fac8b6264122ca6dcf5e29e4729552e8031e591ce6f4ee725e14cbd23`,
registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado, lista exata/diff/stat/inventários, argv e binários, início
UTC `2026-09-09T12:08:08.824769+00:00`. O baseline P1328 rejeita
`calc.abs(-2pt)`, `calc.abs(-2em)`, `calc.abs(-2deg)`, `calc.abs(-2%)` e
`calc.abs(-2fr)`; o vanilla ratificado `a51e02804` devolve o módulo na
mesma espécie de valor. Unidades cm/mm/in e rad também foram medidas.

`lab/typst-original/crates/typst-library/src/foundations/calc.rs:84-94`
declara os casts dimensionais de ToAbs. `layout/length.rs:58-61` admite
somente comprimentos com componente absoluta ou em igual a zero. A
medição refuta a regra intuitiva de aceitar componentes de mesmo sinal:
`calc.abs(2pt + 3em)` e `calc.abs(-2pt - 3em)` também são rejeitados com
`cannot take absolute value of this length`, ancorado no argumento.
`layout/angle.rs:76-78`, `layout/ratio.rs:103-105` e `layout/fr.rs:49-51`
calculam módulo sem normalização angular ou resolução de contexto.

`01_core/src/compiler/stdlib/calc.rs:139-161` já concentra despacho,
guards e origem de Content; `entities/layout_types.rs:1102-1115,1162-1212,
1259-1293` e `entities/value.rs:113` já representam as grandezas requeridas
com APIs públicas suficientes. Não é necessário criar campo ou método.
`p1328-ab-tests-r2.rs:326-345`, em diagnosticos, identifica quatro controles
históricos de rejeição que deixam de ser a obrigação vigente.

As sondas com `0.0/0.0` falham antes da chamada e não medem abs/NaN.
Float multiplicado por fraction também tem lacuna anterior à chamada no
cristalino; não a corrigir neste owner. Casos nativos com valores já
construídos devem distinguir essa fronteira da operação abs. A fonte
expressa o módulo escalar; não se infere intenção geral de design de
eventuais diferenças de construção ou impressão de não finitos.

Reabertura de domínio P1329-R2, ainda antes de C:
`lab/typst-original/crates/typst-utils/src/scalar.rs:29-31` normaliza NaN
para zero; Angle, Ratio, Fr, Abs e Em usam Scalar nos construtores. Portanto
NaN dimensional já construído no cristalino não tem entrada equivalente
nesse vanilla. `p1329-nan-domain-probe-r2.json`, em diagnosticos, SHA-256
`8ca6f55fbcde70a3a4edcc521dccaa248caedd872650b6b735e113991ea84b9c`,
UTC `2026-09-09T12:31:40.046287+00:00`, guarda HEAD/diff/stat e mostra
`(calc.inf - calc.inf) * 1deg` e `* 1%` como NaN no cristalino e zero no
vanilla. Não são apenas estados sintéticos: há dívida anterior à chamada
também na linguagem. Impressão de Length não prova a identidade de suas
componentes. Inf, por outro lado, é representável no domínio vanilla.
Não copiar Scalar nem corrigir essa construção dentro de abs.

### Decisão e aceitação

Depois dos guards existentes, exatamente um argumento Angle, Ratio ou
Fraction deve produzir o módulo da magnitude na mesma espécie de valor,
sem converter em Float, limitar percentagens ou reduzir ângulos a uma
volta. Length só é aceito quando `abs == 0` ou `em == 0`, devolvendo o
módulo de suas componentes, sem resolver font-size e sem converter em Float.
Zero, incluindo zero negativo, conta como zero nessa condição. Se ambas
as componentes forem não zero, rejeitar mesmo com sinais iguais.

O erro de Length misto é Error, mensagem exata
`cannot take absolute value of this length`, sem hints ou trace nativo.
Usar somente o `value_span` da primeira ocorrência posicional; ausência
ou detached permanecem detached. Preservar origens em alias/With/spread,
inclusive fora da chamada final. Os traces externos continuam no
dispatcher; sua dívida de nome `calc.abs` versus `abs` é preservada e
deve ter expectativas integrais congeladas antes de C, não normalizadas.

Abs não usa guard_float. Para a representação cristalina já construída,
o módulo escalar propaga NaN e transforma infinitos negativos em positivos,
mantendo a espécie. Para Length já construído, a condição de componente
zero continua valendo, inclusive para não finitos. Essa obrigação local
para NaN dimensional não é evidência de paridade: o domínio de comparação
exige entradas equivalentes já construídas nos dois sistemas e exclui
esses estados divergentes. Testes nativos NaN verificam somente a regra
local, nunca uma correspondência inexistente. Não corrigir construção,
casts ou operações anteriores à chamada; registrar sua dívida explicitamente.
Preservar Content/LocatedContent P1328, Int/Float/Decimal, salvo a rejeição
de i64::MIN especificada em P1330, guards named/aridade, demais rejeições
e outras funções calc.
Este escopo substitui somente as quatro rejeições dimensionais de P1328;
não declara paridade geral de abs nem altera a política das outras funções.

Autor A/B deve migrar somente os quatro controles dimensionais antigos
para resultados positivos, mantendo todas as demais asserções de P1328,
e escrever testes independentes novos no mesmo owner. Os arquivos de
evidência e snippets antigos em diagnosticos permanecem imutáveis.
Conferir RED real antes de C e GREEN dos mesmos testes; cobrir tipo e
magnitude, sinais/zeros/unidades, mistos de todos os sinais, entrada
sintética/âncora detached, alias/With/spread, UTF-8, math sem coerção,
warnings e os quatro perfis default/html/a11y/html+a11y. NaN/Inf nativos
precisam de testes próprios, não de sondas que falham antes de abs; a
classificação local-versus-paridade acima é obrigatória nos resultados.

Aceitação primária é semântica de tipo/valor e diagnóstico na linguagem
(ADR-0107/0108), não igualdade de estrutura Rust ou bytes de render.
CLI integral normal/repetida/invertida é sentinela complementar, com
dívidas explicitamente classificadas antes de C; Unknown obrigatório
bloqueia fechamento. Executar build, workspace, fmt, lint e V5/V15/V26.
Regime ADR-0127 contínuo: correção de paridade sem mudança de contrato
Rust público, default deliberado, compatibilidade ou fase. É inferência
que este owner basta; necessidade de editar operadores/entidades/dispatcher
ou ausência de origem real obrigatória refuta a suficiência e exige
revisão de escopo antes de expandir a implementação.

## P1330 — overflow inteiro em calc.abs

### Medição anterior à decisão

`00_nucleo/diagnosticos/p1330-baseline.json`, SHA-256
`a3f732bfb2fda2f177dbf3b33caddcd84b219af6fac22f96eae9644e784b6987`,
registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado, diff/stat, inventários e argv/horários/binários. A expressão
`calc.abs(-9223372036854775807 - 1)` retorna 9223372036854775807 no
baseline P1329, enquanto vanilla ratificado a51e02804 rejeita o resultado.
O módulo do inteiro vizinho negativo e do máximo positivo cabe no tipo e
coincide. Alias, With e spread reproduzem a lacuna de overflow.

`01_core/src/compiler/stdlib/calc.rs:139-143` guarda named/quantidade e
satura Int. A fonte vanilla `foundations/calc.rs:84-94,1363-1367` faz
cast verificado para o módulo e define `the result is too large`.
Esta checagem extra da fonte confirma rejeição explícita, não apenas um
efeito acidental observado. A intenção geral de design de inteiros não
é inferida desse comportamento. O que muda é semântica de resultado/erro
da linguagem, não representação Rust nem bytes de render (ADR-0107/0108).

`entities/args.rs:19-20,60-84` conserva ocorrências e value_span; os erros
de conteúdo e Length misto deste owner já consomem essa origem.
`compiler/eval/call_dispatch.rs:1031-1044` trata os traces externos.
Vanilla localiza o overflow na expressão do argumento; With/arguments
podem acrescentar trace da chamada posterior. O nome `calc.abs` no trace
cristalino versus `abs` vanilla permanece dívida do dispatcher.

A medição separa três fronteiras: o literal direto -9223372036854775808
já diverge antes de abs (Float no cristalino, erro de parsing no vanilla);
named e quantidade continuam validados antes do valor no cristalino,
ordem distinta do vanilla. Não usar essas entradas como prova de que o
novo braço de overflow falhou, nem esconder suas diferenças.

### Decisão e aceitação

Depois dos guards vigentes, exatamente um Int cujo módulo não cabe no
inteiro da linguagem deve retornar um único Error com mensagem exata
`the result is too large`, sem hints nem trace nativo. O erro usa somente
o value_span da primeira ocorrência posicional; ocorrência ausente ou
value_span detached permanece detached. Não usar args.span, span de
ocorrência, nome, texto ou busca de origem como substituto. Preservar a
origem em alias/With/spread; traces externos continuam no dispatcher.
Nos demais inteiros retornar o módulo exato como Int, sem saturação,
wrap, promoção para Float ou panic. A fórmula é interna, sem nova API.

Esta seção substitui exclusivamente a antiga saturação de i64::MIN.
Preservar integralmente Float/Decimal, dimensionais e suas condições
P1329, conteúdo P1328, guards de named/aridade, demais rejeições, registro
de nomes e todas as outras funções. Não alterar parsing, entidades,
operadores, construção de NaN ou política guard_float.

Autor A/B migra apenas as duas expectativas antigas de saturação do mesmo
overflow (Int(MIN) nativo e expressão avaliada, em
`p1329-ab-p1328-successor.rs:209,313`) para o diagnóstico acima, com as
origens respectivas detached e do argumento, e a observação CLI de overflow nos
quatro perfis; todos os demais testes históricos ficam intactos. As
evidências em diagnosticos são imutáveis. Congelar testes novos antes de
C: limites inteiros, zero/sinais, vizinhos de MIN, erros completos e
origens distintas inclusive named antes do posicional, sem origem e
detached; controles Float/Decimal/dimensionais/conteúdo/guards.
CLI cobre alias/With/nested With/spread/arguments, UTF-8/linhas, warnings
e rotas math sem coerção; o literal inválido e precedência de guards são
dívidas explicitamente congeladas. Comparar todos os campos do diagnóstico
e resultado semântico tipo/valor, não só substring ou retorno de processo.

RED compilado antes de C e GREEN dos mesmos testes, corpus anterior mais
novos casos nos perfis default/html/a11y/html+a11y, ordens normal/repetida/
invertida; build/workspace/fmt/lint e V5/V15/V26 são gates. Unknown
obrigatório impede fechamento. Regime A/B sem atestação de isolamento.
ADR-0127: correção de paridade contínua, sem mudança deliberada de default,
API, compatibilidade ou fase. É inferência que o owner basta: necessidade
de editar outro consumer ou origem real obrigatória ausente refuta a
suficiência e exige revisão antes de ampliar. Não declarar paridade geral.

## Subset `calc` — 21 entradas (trig + hiperbólicas + exp/log + constantes)

Este prompt cobre o subset fechável do módulo `calc` identificado no P433:
as 7 funções trigonométricas, as 6 hiperbólicas, `exp`, `ln`, `log` (incluindo
a variante com base explícita), e as 4 constantes `pi`/`tau`/`e`/`inf`.

Todas as funções do subset partilham a assinatura padrão de `calc_*`:

```rust
fn calc_X(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value>
```

As funções trig/hiperbólicas/exp/ln usam o helper `unary_f64` (1 argumento
posicional, sem named args, coerção `Int|Float` → `f64`, resultado via
`guard_float`). `sin`/`cos`/`tan` usam `unary_angle` (P817-A — aceitam
`Angle` além de `Int|Float`, paridade vanilla `AngleLike`); `asin`/`acos`/
`atan` usam `angle_op` (P817-A — devolvem `Angle`). `atan2` e `log` têm
aridade própria. Ver `diagnostico-calc-passo-283.md` para a decisão libm vs
`f64::*`.

---

### `calc.sin(x)`

**Assinatura**: `sin(x: Int | Float | Angle) -> Float`

**Argumentos**:
- 1º posicional `x`: ângulo em radianos (`Int` coagido para `f64`, `Float`),
  ou valor `Angle` (convertido para radianos — P817-A).
- Sem argumentos nomeados.

**Semântica**: `f64::sin(x)`, com `guard_float` no resultado.

**Domínio**: ℝ (radianos).

**Paridade vanilla**: Equivalente a `calc.sin(x)` vanilla (`AngleLike`:
aceita `angle` — P817-A, medido `calc.sin(90deg) = 1.0` nos dois binários).

**Testes canónicos**:
```
sin(0) -> 0.0
sin(pi) -> ~0.0
sin(pi/2) -> ~1.0
sin("x") -> Err "esperava Int ou Float"
sin() -> Err "requer 1 argumento"
```

---

### `calc.cos(x)`

**Assinatura**: `cos(x: Int | Float | Angle) -> Float`

**Argumentos**:
- 1º posicional `x`: ângulo em radianos.

**Semântica**: `f64::cos(x)`; `guard_float`.

**Domínio**: ℝ (radianos).

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
cos(0) -> 1.0
cos(pi) -> -1.0
cos(pi/2) -> ~0.0
```

---

### `calc.tan(x)`

**Assinatura**: `tan(x: Int | Float | Angle) -> Float`

**Argumentos**:
- 1º posicional `x`: ângulo em radianos.

**Semântica**: `f64::tan(x)`; `guard_float`. Em `x = π/2` devolve um valor
muito grande que `guard_float` pode rejeitar como Inf.

**Domínio**: ℝ \ {π/2 + kπ}; fora do domínio o resultado tende para ±∞ → Err.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
tan(0) -> 0.0
tan(pi/4) -> ~1.0
tan(pi/2) -> Err "resultado é infinito"
```

---

### `calc.asin(x)`

**Assinatura**: `asin(x: Int | Float) -> Angle`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x ∈ [-1, 1]`; em caso contrário `Err`. De seguida
`f64::asin(x)`; resultado em `Value::Angle` (radianos internos).

**Domínio**: `[-1, 1]`.

**Paridade vanilla**: Equivalente a `calc.asin(x)`; devolve `angle`
(P817-A — medido `#repr(calc.asin(0.5))` → `30deg` nos dois binários).

**Testes canónicos**:
```
asin(0) -> 0deg
asin(1) -> 90deg
asin(-1) -> -90deg
asin(1.5) -> Err "valor deve estar entre -1 e 1"
```

---

### `calc.acos(x)`

**Assinatura**: `acos(x: Int | Float) -> Angle`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x ∈ [-1, 1]`; depois `f64::acos(x)`; `Value::Angle`.

**Domínio**: `[-1, 1]`.

**Paridade vanilla / limitações**: Idênticas a `asin` (devolve `angle`).

**Testes canónicos**:
```
acos(0) -> 90deg
acos(1) -> 0deg
acos(-1) -> 180deg
acos(2) -> Err
```

---

### `calc.atan(x)`

**Assinatura**: `atan(x: Int | Float) -> Angle`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::atan(x)`; `Value::Angle`.

**Domínio**: ℝ.

**Paridade vanilla**: Devolve `angle` (P817-A).

**Testes canónicos**:
```
atan(0) -> 0deg
atan(1) -> 45deg
```

---

### `calc.atan2(x, y)`

**Assinatura**: `atan2(x: Int | Float, y: Int | Float) -> Angle`

**Argumentos**:
- 1º posicional `x`.
- 2º posicional `y`.

**Semântica**: Paridade vanilla na **ordem dos parâmetros** (`x` antes de `y`).
Internamente chama `f64::atan2(y, x)`. Resultado em `Value::Angle`.

**Domínio**: `(x, y) ∈ ℝ²`.

**Paridade vanilla**: Equivalente a `calc.atan2(x, y)`; devolve `angle`
(P817-A — medido `#repr(calc.atan2(1, 2))` → `63.43deg` nos dois binários).

**Testes canónicos**:
```
atan2(1, 1) -> 45deg
atan2(0, 1) -> 0deg
atan2(1) -> Err "requer 2 argumentos"
```

---

### `calc.sinh(x)`

**Assinatura**: `sinh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::sinh(x)`; `guard_float`.

**Domínio**: ℝ; valores muito grandes produzem Inf → Err.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
sinh(0) -> 0.0
sinh(1) -> ~1.1752
sinh(1e10) -> Err "resultado é infinito"
```

---

### `calc.cosh(x)`

**Assinatura**: `cosh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::cosh(x)`; `guard_float`.

**Domínio**: ℝ; valores grandes → Inf → Err.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
cosh(0) -> 1.0
cosh(1) -> ~1.5431
cosh(1e10) -> Err "resultado é infinito"
```

---

### `calc.tanh(x)`

**Assinatura**: `tanh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::tanh(x)`; `guard_float`.

**Domínio**: ℝ.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
tanh(0) -> 0.0
tanh(10) -> ~1.0
```

---

### `calc.asinh(x)`

**Assinatura**: `asinh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::asinh(x)`; `guard_float`.

**Domínio**: ℝ.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
asinh(0) -> 0.0
asinh(1) -> ~0.8814
```

---

### `calc.acosh(x)`

**Assinatura**: `acosh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x >= 1`; depois `f64::acosh(x)`; `guard_float`.

**Domínio**: `[1, +∞)`.

**Paridade vanilla / limitações**: Idênticas a `asin`.

**Testes canónicos**:
```
acosh(1) -> 0.0
acosh(2) -> ~1.31696
acosh(0.5) -> Err "valor deve ser >= 1"
```

---

### `calc.atanh(x)`

**Assinatura**: `atanh(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x ∈ (-1, 1)` estrito; depois `f64::atanh(x)`;
`guard_float`.

**Domínio**: `(-1, 1)`.

**Paridade vanilla / limitações**: Idênticas a `asin`.

**Testes canónicos**:
```
atanh(0) -> 0.0
atanh(0.5) -> ~0.5493
atanh(1) -> Err "valor deve estar em (-1, 1)"
atanh(-1) -> Err
```

---

### `calc.exp(x)`

**Assinatura**: `exp(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: `f64::exp(x)`; `guard_float`.

**Domínio**: ℝ; overflow → Inf → Err.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
exp(0) -> 1.0
exp(1) -> ~2.71828
exp(1e10) -> Err "resultado é infinito"
```

---

### `calc.ln(x)`

**Assinatura**: `ln(x: Int | Float) -> Float`

**Argumentos**:
- 1º posicional `x`.

**Semântica**: Valida `x > 0`; depois `f64::ln(x)`; `guard_float`.

**Domínio**: `(0, +∞)`.

**Paridade vanilla / limitações**: Idênticas a `sin`.

**Testes canónicos**:
```
ln(1) -> 0.0
ln(e) -> 1.0
ln(0) -> Err "valor deve ser estritamente positivo"
ln(-1) -> Err
```

---

### `calc.log(x)` / `calc.log(x, base:)` / `calc.log(x, base)`

**Assinatura**: `log(x: Int | Float, base: Int | Float?) -> Float`

**Argumentos**:
- 1º posicional `x`.
- `base`: named opcional (`Int | Float`). Default `10`.
- 2º posicional `base`: forma legada, mutuamente exclusiva com `base:`.

**Semântica**: Despacho por base exacta (P817-F, paridade vanilla
`calc.rs:506-515`): base `e` → `ln(x)`; base `2` → `log2(x)`; base `10` →
`log10(x)`; outras → `ln(x) / ln(base)`. Valida `x > 0`, `base` finita,
`base > 0`, `base != 1`; `guard_float`.

**Domínio**: `x > 0`; `base ∈ (0, +∞) \ {1}`.

**Paridade vanilla**: Equivalente a `calc.log(x)` (base default 10) e a
`calc.log(x, base: b)`.

**Testes canónicos**:
```
log(1) -> 0.0
log(100) -> 2.0
log(100, base: 10) -> 2.0
log(8, 2) -> 3.0
log(8, base: 2) -> 3.0
log(0) -> Err "valor deve ser estritamente positivo"
log(10, 1) -> Err "base inválida"
log(10, base: 1) -> Err "base inválida"
log(10, 0) -> Err
log(10, inf) -> Err
log(100, 10, base: 10) -> Err (ambos especificados)
```

---

### `calc.round(x, digits:)`

**Assinatura**: `round(x: Int | Float, digits: Int?) -> Int | Float`

**Argumentos**:
- 1º posicional `x`.
- `digits`: named opcional (`Int`). Default `0`.

**Semântica**: Arredondamento half-up de `x` com `digits` casas decimais.
`digits` pode ser negativo (arredonda para dezenas, centenas, etc.).
- Se `digits == 0`, `Float` → `Int` (paridade vanilla `round(3.5) == 4`).
- Se `digits != 0`, retorna `Float`.
- `Int` com `digits == 0` → identidade; com `digits != 0` → `Float` arredondado.

**Domínio**: ℝ.

**Paridade vanilla**: Equivalente a `calc.round(x)` e `calc.round(x, digits: n)`.

**Testes canónicos**:
```
round(3.567) -> 4
round(3.567, digits: 2) -> 3.57
round(1234.0, digits: -2) -> 1200.0
round(255, digits: 2) -> 255.0
round(3.5, digits: "x") -> Err
round(3.5, foo: 1) -> Err
```

---

### `calc.pi` (constante)

**Assinatura**: `pi -> Float`

**Semântica**: `Value::Float(std::f64::consts::PI)`.

**Paridade vanilla**: Equivalente a `calc.pi`.

**Testes canónicos**:
```
calc.pi -> 3.141592653589793
```

---

### `calc.tau` (constante)

**Assinatura**: `tau -> Float`

**Semântica**: `Value::Float(std::f64::consts::TAU)`.

**Paridade vanilla**: Equivalente a `calc.tau`.

**Testes canónicos**:
```
calc.tau -> 6.283185307179586
```

---

### `calc.e` (constante)

**Assinatura**: `e -> Float`

**Semântica**: `Value::Float(std::f64::consts::E)`.

**Paridade vanilla**: Equivalente a `calc.e`.

**Testes canónicos**:
```
calc.e -> 2.718281828459045
```

---

### `calc.inf` (constante)

**Assinatura**: `inf -> Float`

**Semântica**: `Value::Float(f64::INFINITY)`.

**Paridade vanilla**: Equivalente a `calc.inf`.

**Testes canónicos**:
```
calc.inf -> inf
```

---

## Restante do módulo `calc` (fora do subset P433)

As funções abaixo continuam documentadas no módulo; o P433 não as reestrutura
em secções individuais porque são scope-out ou já cobertas por passos
anteriores. Mantêm-se as tabelas do estado actual de `calc.md`.

### Funções base (P27)

| Função | Tipos | Semântica |
|--------|-------|-----------|
| `calc_abs` | `Int`, `Float`, `Decimal`, `Length`, `Angle`, `Ratio`, `Fraction` (P1329) | Módulo na mesma espécie; overflow Int rejeitado (P1330); Length exige componente abs ou em zero; diagnóstico de conteúdo P1328 |
| `calc_pow` | `(Int,Int)`, `(Num,Num)` ou `(Decimal,Int)` (P817-C/D) | `0^0` → Err; exp Int não-i32 → Err; exp Float não-normal → Err; `(Int,Int≥0)` → `Int` (`checked_pow`, overflow → Err); `(Int,Int<0)` → `Float` (`powi`); `(Decimal,Int)` → `Decimal` (`checked_powi`); `(Decimal,Float)` → erro dedicado + hint; resto → `powf` |
| `calc_sqrt` | `Int` ou `Float` | argumento negativo → Err |
| `calc_floor` | `Int`, `Float` ou `Decimal` (P817-D) | `Int`→`Int` (identidade); `Float`→`Int`; `Decimal`→`Int` (overflow → Err) |
| `calc_ceil` | `Int`, `Float` ou `Decimal` (P817-D) | idem `floor` com `ceil` |
| `calc_round` | `Int`, `Float` ou `Decimal` (P817-D) | paridade vanilla de tipos: `Int`→`Int` (digits>0 no-op; digits<0 `round_int_com_precisao` away-from-zero); `Float`→`Float` (half away); `Decimal`→`Decimal` (`MidpointAwayFromZero`); `digits:` named opcional (default 0) |
| `calc_min` | `≥1 Num` (mistos Int/Float) | coerção Int→f64 quando misturado |
| `calc_max` | `≥1 Num` | idem `min` |
| `calc_clamp` | `(value, min, max)` | min > max → Err |

### Aritmética inteira, divisão e partes (P306)

| Função | Domínio | Retorno | Semântica |
|--------|---------|---------|-----------|
| `calc_trunc` | `Int`/`Float`/`Decimal` (P817-D) | `Int` | identidade `Int`; `f.trunc() as i64`; `Decimal`→`Int` (overflow → Err) |
| `calc_fract` | `Int`/`Float`/`Decimal` (P817-D) | `Int`/`Float`/`Decimal` | paridade vanilla: `Int` → `Int(0)`; `Float` → `f.fract()`; `Decimal` → `Decimal.fract()` |
| `calc_rem` | `(Num,Num)` | `Int` se ambos `Int`, senão `Float` | truncada (`%` Rust): sinal do dividendo. Zero → `Err` |
| `calc_rem_euclid` | `(Num,Num)` | `Int`/`Float` | Euclidiana; ≥0 para divisor>0. Zero → `Err` |
| `calc_div_euclid` | `(Num,Num)` | `Int`/`Float` | quociente Euclidiano. Zero → `Err` |
| `calc_quo` | `(Num,Num)` | `Int` | quociente **floored** (P817-B). Paridade vanilla `calc.quo(-7,2) = -4`; caminho float faz `floor` e falha fora do alcance i64 |
| `calc_even` | `Int` apenas | `Bool` | `n % 2 == 0`; `Float` → Err |
| `calc_odd` | `Int` apenas | `Bool` | `n % 2 != 0`; `Float` → Err |
| `calc_gcd` | `(Int,Int)` | `Int` | Euclides iterativo sobre `abs`. `gcd(0,0)=0` |
| `calc_lcm` | `(Int,Int)` | `Int` | `a.abs()/gcd(a,b)*b.abs()` com `checked_*`. `lcm(0,x)=0` |
| `calc_fact` | `Int n ≥ 0` | `Int` | loop `1..=n` `checked_mul`; `n<0` → Err; overflow → Err |
| `calc_perm` | `(n≥0, k≥0)` | `Int` | `n·(n-1)·…·(n-k+1)`; `k>n`→0; neg/overflow → Err |
| `calc_binom` | `(n≥0, k≥0)` | `Int` | iterativo divisão exacta; `k>n`→0; neg/overflow → Err |
| `calc_norm` | `..values: Num` + `p: Float` named | `Float` | `(Σ|x_i|^p)^(1/p)`; default `p=2.0` |
| `calc_root` | `(Num radicand, Int index)` — ordem vanilla (P817) | `Float` | `index==0` → Err; `radicand<0` index par → Err; ímpar → raiz real negativa; index negativo → `x^(1/index)` |
| `calc_erf` | `Int`/`Float` | `Float` | Aproximação Abramowitz & Stegun 7.1.26; erro máx 1.5e-7 |

---

## Política IEEE 754 — `guard_float` (ADR-0101 EM VIGOR)

`guard_float(f)` rejeita NaN e Inf no **resultado** de funções matemáticas
escalares (`pow`, `sqrt`, trig, hiperbólicas, log, exp, root, norm, atan2).
`calc.erf` (P308): NaN input → Err; ±∞ → short-circuit ±1.0; ±0 → ±0.0.

**Política transversal cristalina** (ADR-0101):
- `eval`/layout/operators: **IEEE 754 puro** (paridade vanilla).
- `stdlib` funções matemáticas que usam `guard_float` acima: **rejeita NaN+Inf** (divergência consciente). `abs` não usa esse guard; ver P1329.

---

## Critérios de Verificação (subset P433)

```
calc_sin([Float(0.0)]) -> Ok(Float(0.0))
calc_sin([Float(PI)]) -> Ok(Float(~0.0))
calc_cos([Float(0.0)]) -> Ok(Float(1.0))
calc_cos([Float(PI)]) -> Ok(Float(-1.0))
calc_tan([Float(FRAC_PI_4)]) -> Ok(Float(~1.0))
calc_asin([Float(1.0)]) -> Ok(Float(FRAC_PI_2))
calc_asin([Float(1.5)]) -> Err
calc_acos([Float(-1.0)]) -> Ok(Float(PI))
calc_atan([Float(1.0)]) -> Ok(Float(FRAC_PI_4))
calc_atan2([Float(1.0), Float(1.0)]) -> Ok(Float(FRAC_PI_4))
calc_atan2([Float(1.0)]) -> Err

calc_sinh([Float(0.0)]) -> Ok(Float(0.0))
calc_cosh([Float(0.0)]) -> Ok(Float(1.0))
calc_tanh([Float(0.0)]) -> Ok(Float(0.0))
calc_acosh([Float(1.0)]) -> Ok(Float(0.0))
calc_acosh([Float(0.5)]) -> Err
calc_atanh([Float(0.0)]) -> Ok(Float(0.0))
calc_atanh([Float(1.0)]) -> Err

calc_exp([Float(0.0)]) -> Ok(Float(1.0))
calc_exp([Int(1)]) -> Ok(Float(E))
calc_exp([Float(1e10)]) -> Err
calc_ln([Float(E)]) -> Ok(Float(1.0))
calc_ln([Float(0.0)]) -> Err
calc_log([Float(100.0)]) -> Ok(Float(2.0))
calc_log([Float(8.0), Float(2.0)]) -> Ok(Float(3.0))
calc_log([Float(100.0)], named={base: Int(10)}) -> Ok(Float(2.0))
calc_log([Float(10.0), Float(1.0)]) -> Err
calc_round([Float(3.567)], named={digits: Int(2)}) -> Ok(Float(3.57))
calc_round([Float(1234.0)], named={digits: Int(-2)}) -> Ok(Float(1200.0))

make_calc_module().pi  -> Float(PI)
make_calc_module().tau -> Float(TAU)
make_calc_module().e   -> Float(E)
make_calc_module().inf -> Float(INF)
```
