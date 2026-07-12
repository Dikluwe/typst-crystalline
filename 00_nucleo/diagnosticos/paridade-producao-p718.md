# Paridade Produção — P718 — Spread em literais de array/dict e em argumentos de chamada

**Data:** 2026-07-12
**Passo:** `00_nucleo/materialization/typst-passo-718.md`
**Hash do commit (implementação):** `4d7777da3`.
**HEAD base:** `a8383e1e6` (fim de P717).
**Estado:** FECHADO — spread em literal de array (qualquer posição), em
literal de dict, e em argumentos de chamada (incluindo reencaminhamento
de `Value::Args`) implementados com paridade ao vanilla, incluindo os
casos de erro e o hint de aridade.

---

## 1. Sonda

### 1.1 Comportamento completo de spread em literais (vanilla)

Binário `lab/typst-original/target/release/typst`, repositório em
`a8383e1e6`. Documento do passo (`/tmp/p718-spread.typ`):

```
(1, ..(2, 3), 4)              → (1, 2, 3, 4)          [meio, não só fim]
(a: 1, ..(b: 2, c: 3), d: 4)  → (a: 1, b: 2, c: 3, d: 4)
(..(), 1, ..())                → (1,)                  [vazio: no-op]
```

Casos adicionais (`ast::Array::eval`/`ast::Dict::eval`, `code.rs:224-306`):

| Snippet | Vanilla |
|---|---|
| `(1, ..none, 2)` | `(1, 2)` — spread de `none` é ignorado |
| `(a: 1, ..none, b: 2)` | `(a: 1, b: 2)` |
| `(..(1,2), ..(a: 1))` | `Err cannot spread dictionary into array` — **sem** hint (o spread de array antes zera `all_dict_spreads`) |
| `(..(a:1), ..(b:2))` | mesmo erro **com** hint `add a colon to create a dictionary instead: \`(: ..(a:1), ..(b:2))\`` (todos os itens são spreads de dict) |
| spread de escalar em array/dict | `cannot spread {ty} into array` / `... into dictionary` |

### 1.2 `eval_args` — mesmo gap, com consumidor pesado em `cetz`

`grep -n "fn eval_args\|Arg::Spread" 01_core/src/rules/eval/closures.rs` →
`Arg::Spread(_) => {} // fronteira deliberada` (linha 60, antes deste
passo). Medido:

```
#let f(a, b, c) = a + b + c
#f(1, ..(2, 3))
```

- **Vanilla:** `6`.
- **Cristalino (antes):** `error: cannot apply Add to int and none` —
  `b`/`c` ficavam `none` porque o spread não expandia o array em
  posicionais. **Não** é um caso "silenciosamente ignorado" inofensivo —
  produz erro de tipo em qualquer chamada real com spread.

Uso real em `cetz`: `func(..c)` (`coordinate.typ:277`), `resolve(ctx,
..c)` (`coordinate.typ:276`), `fast-line(..pts, …)`
(`mark-shapes.typ:147,160,206,246,258,301`), `matrix.transform-translate
(..vec)` (`draw/transformations.typ:157`), dezenas de outros sítios —
consumidor pesado, sem questão de scope-out.

### 1.3 Mecanismo vanilla de `Args::eval` (`call.rs:367-416`)

`None` → ignora; `Array` → cada elemento vira posicional; `Dict` → cada
par vira nomeado; **`Value::Args`** → funde posicionais e nomeados
(reencaminhamento de um sink `..rest` de outra closure — vanilla mantém
`Args` como `Vec<Arg{name,value}>` ordenado; o cristalino já separa
`items`/`named`, então a fusão é `items.extend`+`named.extend`); outro
tipo → `cannot spread {ty}` — **sem** o sufixo "into X" das mensagens de
literal (mensagem distinta, confirmada por medição):

```
f(..5)                              → Err cannot spread integer   [não "into array/call"]
f(..(1,2), ..(3,4))  (sink ..a)      → a.pos() = (1, 2, 3, 4)
f(1, ..(x:1), ..(y:2), 3)            → (a.pos(), a.named()) = ((1,3),(x:1,y:2))
g(..a)=a; f(..b)=g(..b); f(1,2,x:3)  → r.pos()=(1,2), r.named()=(x:3)
```

### 1.4 Rest params na definição (`..pts`) — fora do scope, sem regressão

`#let f(..pts) = pts.pos().len()` já funcionava em ambos antes deste
passo (P504) — mecanismo distinto (bind de parâmetro, não spread de
valor); confirmado sem regressão após a implementação.

### 1.5 Estado do cristalino antes

- Literal de array: `Expr::Array` filtrava só `ArrayItem::Pos` — spread
  silenciosamente descartado.
- Literal de dict: `DictItem::Spread(_) => {}` — no-op.
- `eval_args`: `Arg::Spread(_) => {}` — no-op, mas com efeito observável
  (aridade errada → erro de tipo a jusante).

---

## 2. Implementação

L0 actualizado primeiro: `00_nucleo/prompts/rules/eval.md` §P718 (hash
`dc496ef8` via `crystalline-lint --fix-hashes`). Testes escritos antes do
código: 15 testes `p718_*`, 11 a falhar no estado P717 (4 passavam por
coincidência — casos que não dependem de spread, ex. `none` em dict e o
teste de não-regressão de rest-param).

- **`mod.rs` `Expr::Array`**: reescrito sobre `Vec<ArrayItem>` colectado
  (permite lookahead por índice); `all_dict_spreads` seguido item a item
  (mirror exacto de `code.rs:224-271`); `remaining_are_dict_spreads`
  (nova função, mirror do `items.all(...)` de lookahead) para decidir o
  hint; `fixed` via `arr.to_untyped().clone().into_text()` +
  `replacen('(', "(: ", 1)` (mirror de `full_text().replacen(...)`).
- **`mod.rs` `Expr::Dict`**: braço `DictItem::Spread` deixa de ser no-op.
- **`closures.rs` `eval_args`**: braço `Arg::Spread` deixa de ser no-op —
  `None`/`Array`/`Dict`/`Args` + erro sem "into".
- **`long_type_name`** (P716) passa a `pub(super)` em `bindings.rs`,
  reaproveitado nos três sítios para o nome longo do tipo nas mensagens
  (ADR-0107).

---

## 3. Validação

Estado da medição: working tree com exactamente as alterações deste
passo, commitado de seguida como `4d7777da3` (o diff do commit é o
estado medido). Re-verificação pós-commit em §3.4.

### 3.1 Documentos do passo vs vanilla

- `/tmp/p718-spread.typ` → `pdftotext` **idêntico** ao vanilla:
  `(1, 2, 3, 4) (a: 1, b: 2, c: 3, d: 4) (1,)` (a única diferença é
  cosmética — presença/ausência da vírgula trailing em tuplo de 1
  elemento no render de texto, comportamento herdado, não deste passo).
- `/tmp/p718-call-spread.typ` (`f(1, ..(2,3))`) → `6`, igual ao vanilla.

### 3.2 Suites

- `cargo test --workspace` → **3906 passed, 0 failed** no `typst-core`
  (3891 em P717 + 15 novos), restantes crates verdes.
- `crystalline-lint .` → **0 violations**.

### 3.3 Reprodução `cetz` — campos fixos de progresso

Documento do passo (`/tmp/p718-cetz.typ`):

- **Exit code:** 1. **Tempo:** 51,9s (ver §3.4 para a medição pós-commit).
- **Bloqueio de P717 resolvido** — `not enough elements to destructure`
  desapareceu (o retorno `(ctx, ..result)` de `coordinate.typ:440` volta
  a ter o tamanho correcto).
- **Próximo bloqueio, com `file:line`:** `error: não é possível iterar
  sobre dictionary` — for-loop directo sobre um dict
  (`for (key, value) in dict`), sem suporte no cristalino. Consumidor
  real e directo: `styles.typ:189,322,354` (`for (key, value) in dict
  { ... }`, usado na resolução de estilos do `cetz`). Medido
  isoladamente: `#for (k, v) in (a: 1, b: 2) [#k=#v ]` → `a=1 b=2` no
  vanilla, `error: não é possível iterar sobre dictionary` no
  cristalino. Candidato a P719 (for-loop sobre dict — mecanismo
  distinto de spread, provavelmente em `control_flow.rs`).

### 3.4 Re-verificação pós-commit (proveniência)

Executada no commit `4d7777da3`, working tree limpa:

- `/tmp/p718-spread.typ` → mesmo resultado: `(1, 2, 3, 4) (a: 1, b: 2,
  c: 3, d: 4) (1,)`.
- `/tmp/p718-call-spread.typ` → mesmo resultado: `6`.
- `cetz` → mesmo bloqueio (`não é possível iterar sobre dictionary`),
  tempo real 52,7s (51,9s pré-commit — variação normal de ruído de
  máquina, sem alteração significativa; o documento continua a falhar
  na fase de eval, antes de qualquer layout).

---

## 4. Gate das ADRs (critério do passo)

Grep a `spread`/`Spread` em `00_nucleo/adr/*.md`: uma única ocorrência,
em `typst-adr-0089-gradient-conic-only.md:401` ("Spread mode repeat"),
termo de gradientes CSS-like (`spread-method`), **sem relação** com o
operador de spread da linguagem. ADR-0107/0108/0109 não mencionam o
tema. **Nada contradiz o decidido.**

---

## 5. Critério de fecho do passo

- [x] Sonda completa — comportamento de spread em literais confirmado
  (array em qualquer posição, dict, vazio, com/sem hint); `eval_args`
  confirmado como o mesmo gap, com consumidor pesado medido em `cetz`
  (não scope-out).
- [x] Implementado e testado (15 testes novos), incluindo o hint de
  aridade e a mensagem distinta de `eval_args` (sem "into").
- [x] Sem regressão — `cargo test --workspace` verde (3906/0); rest
  params (P504) confirmados intactos.
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — campos fixos registados (tempo 51,9s, exit 1,
  próximo bloqueio com `file:line`: `styles.typ:189,322,354`, for-loop
  sobre dict).
- [x] Grep às ADRs pelos termos centrais (`spread`, `Arg::Spread`) —
  nada contradiz (§4).
- [x] Relatório com hash do commit.
