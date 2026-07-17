# Relatório — Passo 303 (Bug fix `undef(x)` parens — lookup-miss)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-303.md`
**Tipo declarado spec**: bug latente irmão de P302 — subcategoria
ambígua §8.4 (A: bug derivado materialização vs B: bug pré-existente
descoberto durante inspecção).
**Hipótese adoptada**: **HX'** (args preservados via
`MathSequence([MathIdent, MathDelimited])`) + **A.2 → (a) refinado**
(caminho unificado via variável local `base` em vez de duplicação
literal do template) + **subcategoria B** (bug pré-existente
independente).
**Baseline P302**: 2 876 testes  →  **P303**: 2 881 testes
(Δ = **+5 net**: +6 novos L1 P303, −1 obsoleto P302).
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**20º passo
consecutivo**: P282→P303).
**Hash `content.rs`**: `82d3c47d` inalterado.
**ADRs meta novas**: 0 (**11.ª vez consecutiva** anti-padrão P273.17
§0 honrado).

---

## §1 — Sumário executivo

P303 resolve o bug irmão de P302: `$undef(x)$` em math mode (identifier
sem lookup-hit no scope `math`) descartava silenciosamente os args
`(x)` no fallback `Content::MathIdent`. Bug **pré-existente pré-P301**
— sempre existiu, mas só ficou **visível como bug** após P302 ter
fixado o caminho adjacente (lookup-hit), expondo a assimetria.

**Modificação localizada** em `01_core/src/engine/eval/math.rs` arm
`Expr::FuncCall` ramo `_`: consolidação do caminho lookup-hit (P302)
e lookup-miss (P303) via variável local `base` — refactor minor que
elimina duplicação de ~25 linhas do template sem introduzir helper
`wrap_with_args` separado. **Resultado**: ambos os ramos partilham
a lógica de preservação de args via `MathSequence([base, MathDelimited])`.

**Resultado funcional**:
- `$undef(x)$` agora produz `MathSequence([MathIdent("undef"),
  MathDelimited((x))])` — args preservados.
- `$undef()$` (args vazios) emite só `MathIdent("undef")` — paridade
  P302 `$sin()$`.
- `$undef$` (sem parens) preservado bit-exact pré-P301.
- **`$sin(x)$` P302 preservado bit-exact** — regressão verificada.

**Resultado metodológico — §8.4 sub-padrão preservado N=2**:
- N=1 P288: NBSP fix derivado P287.
- N=2 P302: sin parens fix derivado P301.
- **P303 NÃO inflaciona §8.4 para N=3** — subcategoria B genuinamente
  distinta (bug pré-existente independente, não derivado de
  materialização anterior).

**Lição metodológica aplicada primeira-vez**: P303 §A.5 aplicou
explicitamente **matrix feature × sintaxe** (lição registada em
P302 §6.7). Cobertura completa lookup-hit × lookup-miss × com/sem
parens × args vazios/simples/complexos/múltiplos.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=10 reaplica §8.7') | Bug factual confirmado; vanilla HX' confirmado; magnitude **baixa-modesta** |
| A.0 (ADR-0098) | ✅ `export.rs` preservado bit-exact (**20º passo**) |
| A.1 inventário | `MathSequence` + `MathDelimited` + `MathIdent` + `MathText` todos pré-existentes |
| A.2 decisão | **(a) refinado** — caminho unificado via `base` local (sem helper externo, sem duplicação) |
| A.3 integração | `$undef$` sem parens preservado; `$undef()$` paralelo `$sin()$` |
| A.4 emit | FrameItem standard; hash preservado |
| A.5 bugs latentes | Matrix feature × sintaxe primeira aplicação explícita; 8+ cenários verificados |
| A.5' anti-reflexão | **N=13 cumulativo** (P291-P303); subcategoria §8.4 clarificada empiricamente |

---

## §3 — Materialização

### §3.1 — `01_core/src/engine/eval/math.rs` — FuncCall ramo `_` unificado

```rust
// Outros nomes: P301 auto-lookup math (sin, cos, lim, …)
// + P302 preservação args via MathSequence + MathDelimited.
// + P303 paralelo lookup-miss: identifier desconhecido
//   também preserva args (simetria arquitectural).
//
// **P302 (HZ confirmado)**: vanilla parser distinguish
// `sin(x)` como `sin` + `(x)` delimited; cristalino parser
// produz FuncCall em math mode (divergência) mas eval
// emula comportamento vanilla retornando
// `MathSequence([MathOp, MathDelimited((x))])`.
//
// **P303 (HX')**: `undef(x)` (identifier sem lookup-hit)
// produzia `MathIdent("undef")` descartando args (bug
// pré-P301). Agora `MathSequence([MathIdent, MathDelimited])`
// — paralelo arquitectural directo P302.
_ => {
    let pos_args: Vec<Expr<'_>> = call.args().items()
        .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
        .collect();
    let base = if let Some(op) = lookup_math_op(scopes, &name) {
        op
    } else {
        Content::MathIdent(name.into())
    };
    if pos_args.is_empty() {
        // `sin()` / `undef()` — args vazios; só base sem wrapper.
        return Ok(base);
    }
    let body = if pos_args.len() == 1 {
        eval_math_expr(scopes, ctx, pos_args[0])?
    } else {
        // Múltiplos args: separados por `, ` (paridade vanilla).
        let mut items: Vec<Content> = Vec::new();
        for (i, expr) in pos_args.iter().enumerate() {
            if i > 0 {
                items.push(Content::MathText(", ".into()));
            }
            items.push(eval_math_expr(scopes, ctx, *expr)?);
        }
        Content::MathSequence(std::sync::Arc::from(items))
    };
    let delimited = Content::MathDelimited {
        open:  '(',
        body:  Box::new(body),
        close: ')',
    };
    Ok(Content::MathSequence(std::sync::Arc::from(vec![base, delimited])))
}
```

**Diferença vs template spec §3**: o template duplicava o bloco
inteiro de construção `body` + `delimited` (uma vez no ramo
lookup-hit, outra no lookup-miss). Refactor minor unifica via
variável local `base` — mantém-se dentro do espírito (a) "fix
directo composicional" sem introduzir helper externo (b)
`wrap_with_args`. **Resultado**: −25 linhas duplicadas; +clareza.

**Reutilização total** de variants existentes:
- `Content::MathOp` (P298) — caminho lookup-hit.
- `Content::MathIdent` (pré-existente) — caminho lookup-miss.
- `Content::MathDelimited` (pré-existente desde P55+).
- `Content::MathSequence` (pré-existente).
- `Content::MathText` (separador `, ` para múltiplos args).

**Zero variants novos**. P303 é puramente **composicional** —
paralelo P302.

### §3.2 — Zero alterações em outros sítios

| Componente | Pós-P303 |
|---|---|
| `lookup_math_op` helper (P301) | **Inalterado** |
| `MathIdent` arm (P301) | **Inalterado** — `$undef$` sem parens preservado |
| Heurística P298 `attach.rs` | **Inalterado** |
| `make_math_module` P299 | **Inalterado** |
| Outras arms math/eval | **Inalterados** |
| L0 `rules/eval.md` | **Inalterado** — extensão dentro do contrato |
| `03_infra/src/export.rs` | **Inalterado bit-exact** — hash `66cb8ac3` (**20º passo**) |
| `01_core/src/entities/content.rs` | **Inalterado** — hash `82d3c47d` |

---

## §4 — Testes

### §4.1 — `01_core/src/engine/eval/tests.rs` (+6 novos / −1 obsoleto = net +5)

**Removido**:

| Teste | Razão |
|---|---|
| `p302_undef_parens_continua_mathident_fallback` | Documentava o bug P303 (asseverava que args eram descartados em fallback `MathIdent`). Invalidado pela correcção P303. |

**Adicionados** (6 testes L1 P303):

| Teste | Verifica |
|---|---|
| **`p303_undef_parens_produz_mathsequence_com_delimited`** | `$undef(x)$` → `MathSequence([MathIdent("undef"), MathDelimited((x))])` — bug fix verificado |
| `p303_undef_args_complexos_preservados` | `$undef(x + y)$` — args complexos preservados |
| `p303_undef_multiplos_args_separados_por_virgula` | `$undef(x, y)$` — separação `, ` (paridade P302) |
| `p303_undef_args_vazios_so_mathident_sem_wrapper` | `$undef()$` — só MathIdent (sem MathDelimited wrapper) |
| **`p303_regressao_undef_sem_parens_preservado`** | **INVARIANTE CRÍTICA**: `$undef$` (sem parens) continua MathIdent directo |
| **`p303_regressao_sin_parens_p302_preservado`** | **INVARIANTE CRÍTICA**: `$sin(x)$` P302 fix preservado bit-exact |

### §4.2 — Sem alterações em L3

P303 não modifica emit. Tests L3 existentes preservados.

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2377 passed; 0 failed; 0 ignored   (typst-core lib)
test result: ok.  457 passed; 0 failed; 6 ignored   (typst-infra lib)
test result: ok.   24 passed; 0 failed; 0 ignored   (typst-shell lib)
test result: ok.    2 passed; 0 failed; 0 ignored   (bin)
test result: ok.   21 passed; 0 failed; 0 ignored   (bin)
                  -----
                  2881 passed total
```

Baseline P302 = 2 876; delta = **+5 net** (+6 novos P303 −1 obsoleto
P302) ✓.

Resultado dentro da janela esperada da spec (~2 881–2 884).

**Nota**: o teste pré-existente `recursao_infinita_retorna_err_sem_crash`
exibe stack overflow em modo debug independentemente de P303
(verificado em `git stash` baseline). Não relacionado.
Executado com `RUST_MIN_STACK=33554432` para validação completa.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hashes pós-P303

| Ficheiro | Pós P303 |
|---|---|
| `infra/export.rs` (`@prompt-hash`) | **`66cb8ac3` preservado bit-exact** (**20º passo consecutivo**) |
| `entities/content.rs` (`@prompt-hash`) | `82d3c47d` inalterado |
| `eval/math.rs` (`@prompt-hash`) | `19073424` inalterado |
| L0 markdown | todos preservados |

L0 `rules/eval.md` inalterado — P303 é **extensão dentro do contrato
L0 existente**, paralelo a P301 + P302.

---

## §6 — Padrões metodológicos

### §6.1 — **Sub-padrão §8.4 "bug latente fixed durante materialização dependente" — N=2 preservado**

| Aplicação | Subcategoria | Origem | Bug |
|---|---|---|---|
| N=1 P288 | A (derivado) | P287 SmartQuote materialização | NBSP eliminado por `split_whitespace()` |
| N=2 P302 | A (derivado) | P301 auto-lookup materialização | Args `(x)` descartados em FuncCall lookup-hit |
| **P303** | **B (pré-existente independente)** | Pré-P301 sempre existiu | Args `(x)` descartados em FuncCall lookup-miss |

**Distinção subcategoria A vs B**:
- **A (P288, P302)**: bug **derivado** de materialização anterior —
  passo X cria/expõe via X+1.
- **B (P303)**: bug **pré-existente** descoberto durante inspecção —
  existia desde sempre; só ficou **visível como bug** após fix
  adjacente (P302) ter criado assimetria evidente.

**Decisão**: P303 inaugura **subcategoria B** dentro de §8.4 mas
**NÃO inflaciona N=3** da subcategoria A. §8.4 sub-padrão A
preserva N=2 — longe de limiar tentativo N≥3 para promoção formal.

**Honestidade epistémica**: ambiguidade A vs B foi explicitada e
resolvida conservadoramente (subcategoria B) per P273.17 §0
anti-padrão. Não houve forçagem da categorização para inflar N.

### §6.2 — §8.7' "A.0.0 template" — N=10 magnitude baixa-modesta

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293-P302 | 1-9 | varia |
| **P303** | **10** | **baixa-modesta** |

**Janela P294-P303**: máxima, baixa, média, alta, alta, baixa,
média-modesta, média, **baixa-modesta**. Continua não-monotónico
— flutuação saudável preservada.

§8.7' N=10 **adiado** seguindo standard P300-P302 (anti-padrão
conservador).

### §6.3 — §8.3 "refutação pragmática" — N=12 sem aplicação P303

P303 **confirma** bug factual (não refuta spec). §8.3 não dispara.
N=12 candidato preservado.

### §6.4 — §8.6 "A.5' anti-reflexão" — N=13 cumulativo

P291-P303 (P300 incluído como auditoria retrospectiva). Subcategoria
§8.4 clarificada empiricamente como elemento estructuralmente novo.

### §6.5 — ADR-0098 "single source of truth" — N=20 cumulativo

Hash `export.rs 66cb8ac3` preservado bit-exact pelos **20 passos
consecutivos** P282-P303. Invariante robusta sobre 20 features
distintas. P303 é o **20.º passo** — preservação por paradigma
"modificação eval interna composicional" (idêntico P302).

### §6.6 — Anti-padrão P273.17 §0 — 11 passos consecutivos honrados

**0 ADRs meta promovidas P293-P303** (11 passos):

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293-P302 | 10 passos cumulativos | 0 |
| **P303** | **§8.4 subcat B inaugurada, §8.7' N=10, §8.6 N=13** | **0** |

**11.ª vez consecutiva** anti-padrão honrado.

### §6.7 — Lição metodológica P302 §6.7 aplicada primeira-vez

P302 §6.7 registou lição: A.5 deve incluir **matrix feature × sintaxe**.
P303 §A.5 **aplica explicitamente primeira-vez**:

```
                       Sintaxe A ($f$)  Sintaxe B ($f x$)  Sintaxe C ($f(x)$)  Sintaxe D ($f()$)
Operador conhecido    ✓ MathOp P301    ✓ MathOp P301      ✓ MathSequence P302  ✓ MathOp P302
Identifier desconhecido ✓ MathIdent    ✓ MathIdent        ✗→✓ P303 fix         ✓ MathIdent (sem wrapper)
Operador com limits   ✓ MathOp P301    ✓ MathOp P301      ✓ MathSequence P302  ✓ MathOp P302
```

Matrix completa cobre lookup-hit/miss × com/sem parens × args
vazios/presentes. Aplicação futura: A.5 de P304+ deve continuar
matrix explícita quando features tocam sintaxe pré-existente.

---

## §7 — Cobertura vanilla vs cristalino

P303 não altera tabelas de cobertura. **Paridade vanilla atingida**
para `$<ident>(<args>)$` syntax via emulação no eval (lookup-miss):

| Caso | Antes P303 | Pós P303 |
|---|---|---|
| `$undef$` | `MathIdent("undef")` | `MathIdent("undef")` ✓ preservado |
| `$undef(x)$` | `MathIdent("undef")` (bug: args descartados) | `MathSequence([MathIdent, MathDelimited((x))])` ✓ |
| `$undef(x, y)$` | `MathIdent("undef")` (bug) | `MathSequence([MathIdent, MathDelimited((x, y))])` ✓ |
| `$undef()$` | `MathIdent("undef")` ✓ degenerate | `MathIdent("undef")` ✓ preservado |
| `$sin(x)$` (regressão P302) | `MathSequence([MathOp, MathDelimited])` ✓ | **`MathSequence([MathOp, MathDelimited])` ✓ bit-exact** |

**Paridade vanilla** atingida para os caminhos lookup-hit + lookup-miss
× `(args)` syntax. Ambos os ramos do `_` arm em `FuncCall` agora
preservam args via composição idêntica.

---

## §8 — Frentes pendentes pós-P303

P303 fecha bug irmão P302 §7. **Frentes restantes** (catálogo em
`frentes-pendentes-pos-p299.md` e P300):

| Frente | Magnitude | Estado |
|---|---|---|
| **P295.1** nota corpo no rodapé | L | P300 prioridade #1 |
| P296.X toggles cancel | XS | refino cluster math |
| P297.X UnderoverKind | XS+ | cosmético |
| Cosméticos ADR-0054 graded (vários) | XS individual | agregável |

---

## §9 — Decisão sobre P304

P303 fecha bug irmão P302. P304 disponível para qualquer frente:

1. **P295.1 nota corpo no rodapé** — prioridade P300 pendente desde
   antes de P302/P303.
2. **Cosméticos cleanup agregado** — múltiplos XS.
3. **Frente totalmente nova** — fora do escopo P283-P303.

Decisão fica para o operador humano.

---

## §10 — Honestidade epistémica

### §10.1 — Subcategoria §8.4 B inaugurada conservadoramente

A ambiguidade A vs B foi resolvida **honestamente** para B
(bug pré-existente independente). Argumentos considerados:

- **Argumento A (rejeitado)**: bug derivado de P302 (sem P302,
  ambos os caminhos descartavam args simetricamente; assimetria
  pós-P302 expõe). **Refutado**: o bug pré-existia desde antes
  de P301 — P302 só tornou a assimetria evidente; não criou o bug.
- **Argumento B (aceite)**: bug pré-existente independente; só
  visibilidade mudou. Conservador per P273.17 §0.

**Resultado**: §8.4 sub-padrão A preserva N=2 (P288 + P302) sem
inflação forçada. P303 inaugura subcategoria B com N=1 — longe
de qualquer limiar de promoção.

### §10.2 — Refactor minor vs template literal

O template do spec §3 duplicava o bloco completo de construção de
args. Optei por refactor minor (variável `base` unificadora) em
vez de duplicação literal. **Justificação honesta**:
- Spec default era (a) "aceitar duplicação minor para clareza".
- A duplicação seria ~25 linhas — **não-minor**.
- Refactor minor não introduz helper externo (não é (b)) — fica
  dentro do espírito (a) mas elimina duplicação significativa.
- Resultado: **código mais claro**, lógica única para ambos os
  ramos do match, paralelo arquitectural ainda mais evidente.

Decisão conservadora: nem (a) literal nem (b) externalizado;
hybrid in-place.

### §10.3 — Matrix cross-construct primeira-vez explícita

P302 §6.7 registou a lição. P303 §A.5 aplicou genuinamente —
cobertura completa de 12 células da matrix feature × sintaxe.
**Lição vira pattern operacional**, não apenas registo.

### §10.4 — Reutilização vs criação

P303 reusa **5 variants existentes** (`MathOp`/`MathIdent`/
`MathDelimited`/`MathSequence`/`MathText`). **Zero novos variants**
— confirma novamente que cristalino tem infraestrutura
composicional rica.

**Pattern N=2 cumulativo** (P302 + P303): **bug fixes derivados
podem ser puramente composicionais** se infraestrutura suporta.
Confirmação P303 reforça observação P302.

---

## §11 — Fecho

P303 fechado com:

- **+5 testes net** (+6 novos L1, −1 obsoleto P302) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **0 drift** em hashes L0.
- **Hash `export.rs` preservado** bit-exact (**20º passo consecutivo**
  P282-P303).
- **Hash `content.rs` inalterado**.
- **L0 `rules/eval.md` inalterado** — extensão dentro do contrato.
- **0 ADRs meta novas** — **11.ª vez consecutiva** anti-padrão honrado.
- **Sub-padrão §8.4 N=2 preservado** (subcategoria A genuína);
  subcategoria B inaugurada com N=1.

**MARCO P303**:
- **Bug irmão P302 resolvido** — `$undef(x)$` produz output correcto
  via emulação vanilla paralela.
- **Subcategoria §8.4 B inaugurada** — bug pré-existente independente
  descoberto durante inspecção, distinto de A (bug derivado).
- **A.0.0 N=10 magnitude baixa-modesta** — bug factual trivial;
  verificação vanilla rotineira.
- **Reutilização total variants existentes** — paralelo P302 sem
  novos variants.
- **Hash `export.rs` preservado pelo 20.º passo consecutivo**
  (P282→P303) — ADR-0098 robusta sobre 20 features distintas.
  **Vintena cumprida**.
- **Matrix feature × sintaxe primeira aplicação explícita** —
  lição P302 §6.7 viraliza como pattern operacional.
- **Refactor minor in-place** — caminho unificado via `base` local
  elimina duplicação significativa sem introduzir helper externo;
  paralelo arquitectural lookup-hit/lookup-miss torna-se evidente
  no próprio código.

**Lição final**: P303 prova que **simetria arquitectural emerge
naturalmente** quando bug fixes derivados/irmãos partilham
infraestrutura composicional. P302 + P303 completam ambos os
ramos do `_` arm em `FuncCall` math mode — **lookup-hit e
lookup-miss agora paralelos por construção, não por coincidência**.
Refactor minor torna esse paralelo visível na própria estrutura
do código.
