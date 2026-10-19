# Diagnóstico — Fase A do Passo 302 (Bug fix `sin(x)` parens)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-302.md`
**Origem**: P301 §9 — bug latente descoberto pós-implementação
auto-lookup math mode.
**Tipo declarado spec**: bug latente fixed durante materialização
dependente; reaplica sub-padrão §8.4 P288 (NBSP fix derivado P287
SmartQuote). **N=2 cumulativo**.
**Hipótese adoptada**: **HZ** (function call vanilla emite
sequência `op + delimited`) + **A.2 → (a)** + **A.3 → (β)**
preservação de args via MathSequence + MathDelimited.
**Magnitude da refutação A.0.0 N=9**: **média** — bug factual
concreto; vanilla pattern confirmado por leitura literal.

---

## A.0.0 — Verificação literal do bug (N=9 §8.7')

### A.0.0.1 — Reprodução do bug em código P301

Sítio exacto em `01_core/src/engine/eval/math.rs:305-311`:

```rust
// Outros nomes: P301 auto-lookup math (sin, cos, lim, …);
// fallback MathIdent. Args `(x)` continuam descartados (bug
// latente pré-P301 fora de scope; vanilla parser-side resolve
// `sin(x)` como `sin` + `(x)` delimited).
_ => {
    if let Some(op) = lookup_math_op(scopes, &name) {
        Ok(op)  // ← BUG: args descartados aqui
    } else {
        Ok(Content::MathIdent(name.into()))
    }
}
```

**Bug confirmado factualmente**:
- `$sin(x)$` produz `FuncCall("sin", [x])` no parser cristalino.
- `lookup_math_op("sin")` retorna `Some(MathOp{text:"sin", limits:false})`.
- Args `[x]` são **silenciosamente descartados**.
- Output: apenas `sin`, sem `(x)`.

### A.0.0.2 — Comportamento vanilla (HZ confirmado)

Vanilla typst em `lab/.../typst-eval/src/math.rs:75` tem
**`MathDelimited` separado de FuncCall**. Em math mode, `sin(x)`
é parseado como:

```
MathSequence([
    MathIdent("sin"),       // ou lookup em math scope
    MathDelimited('(', x, ')'),
])
```

**NÃO** é FuncCall em vanilla — parser distinguish `func(args)` vs
`expr(group)` em math mode.

**Divergência cristalino**: parser cristalino produz FuncCall mesmo
em math mode (limitação do parser). P302 **emula comportamento
vanilla no eval** sem refactorar parser.

### A.0.0.3 — Magnitude da refutação

| Aspecto | Veredicto |
|---|---|
| Bug factual confirmado | ✅ Sim — args descartados em fallback `_ =>` |
| Vanilla pattern claro | ✅ HZ — `MathSequence([op, MathDelimited])` |
| Magnitude da refutação | **média** — bug concreto + pattern claro |

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293 | 1 inaugural | alta |
| P294 | 2 | máxima |
| P295 | 3 | baixa |
| P296 | 4 | média |
| P297 | 5 | alta |
| P298 | 6 | alta |
| P299 | 7 | baixa |
| P300 | (retrospectivo; sem A.0.0 magnitude) | n/a |
| P301 | 8 | média-modesta |
| **P302** | **9** | **média** |

**Janela P294-P302**: máxima, baixa, média, alta, alta, baixa,
média-modesta, **média**. Continua **não-monotónico** —
flutuação saudável preservada.

### A.0.0.4 — Decisão hipótese HZ + Plano

**HZ confirmado** por inspecção vanilla. P302 procede com:

1. Construir `MathSequence([MathOp, MathDelimited])` quando lookup
   encontra operador num FuncCall com args.
2. Caso edge `$sin()$` (args vazios) — retornar só `MathOp`.
3. Múltiplos args — separar por `, ` (paridade vanilla).
4. Fallback `MathIdent` preservado quando lookup retorna None.

---

## A.0 — Reuso ADR-0098

| Verificação | Esperado pós-P302 |
|---|---|
| Hash `export.rs` | `66cb8ac3` preservado bit-exact (**19º passo consecutivo**) |
| Hash `eval/math.rs` (`@prompt-hash`) | inalterado (L0 `rules/eval.md` cobre o contrato) |
| Hash `content.rs` | preservado (sem variants novos) |
| ADR-0098 N=19 cumulativo | ✅ |

---

## A.1 — Inventário literal

### A.1.1 — `Expr::FuncCall` em math mode

Estrutura AST:
```rust
Expr::FuncCall(call) where call.callee() = Expr::MathIdent(ident)
                     and call.args() = Args { items: [Arg::Pos(expr), ...] }
```

### A.1.2 — Path P301 actual (sítio bug)

Linhas 305-311 do `eval/math.rs` pós-P301 (vide A.0.0.1).

### A.1.3 — Variants disponíveis

- `Content::MathOp { text: Box<Content>, limits: bool }` (P298).
- `Content::MathDelimited { open: char, body: Box<Content>, close: char }`.
- `Content::MathSequence(Arc<[Content]>)`.

**Todos reutilizáveis** — P302 sem variants novos.

### A.1.4 — Vanilla equivalente

Vide A.0.0.2 — vanilla parser distinguish; cristalino emula no eval.

### A.1.5 — Outros operadores afetados

`cos(x)`, `tan(x)`, `lim(x)`, `det(X)`, etc. — **todos os 42
operadores P299** sofrem o mesmo bug quando usados com parens.

### A.1.6 — Testes existentes pré-P302

Testes P301 cobrem `$sin x$` (sem parens). `$sin(x)$` específico
**não está coberto** — falha A.5 P301 (lição metodológica).

### A.1.7 — Caso `$undef(x)$`

User-defined function ou operador desconhecido. P301 fallback:
`lookup_math_op` retorna None → `Ok(Content::MathIdent("undef".into()))`.
**Args já eram descartados pré-P301** — comportamento preservado.

P302 **não conserta** este caso (fora de scope; bug latente
pré-P301 distinto).

### A.1.8 — Diagrama de fluxo P302

```
$sin(x)$
       │
       ▼
parser → FuncCall("sin", Args { items: [Pos(x)] })
       │
       ▼
eval_math_expr arm FuncCall:
  name = "sin"
  match "sin" → fallback _
    lookup_math_op(scopes, "sin") → Some(MathOp{text:"sin", limits:false})
    [P302 NEW]:
      pos_args = [x]
      body = eval_math_expr(x) = MathIdent("x") (ou MathText se single letter)
      delimited = MathDelimited { open: '(', body, close: ')' }
      result = MathSequence([MathOp{sin}, MathDelimited{(x)}])
       │
       ▼
Math Layouter consume sequence:
  - MathOp → layout_op (P298 trivial delegate)
  - MathDelimited → layout_delimited (pré-existente)
       │
       ▼
PDF "sin(x)" inline standard
```

---

## A.2 — Estrutura correcção (decisão a)

**Decisão A.2 → (a)** — preservar args via `MathSequence` +
`MathDelimited`.

Não escolhi (b) (concat manual em text) por **perder semântica**.
Não escolhi (c) (lookup só MathIdent) por **deixar gap residual**
P301.

---

## A.3 — Integração fallback (decisão β)

**Decisão A.3 → (β)** — auto-lookup com preservação de args.

Caso `$sin()$` (args vazios): retornar só `MathOp` (sem
MathDelimited vazio).

---

## A.4 — Impacto em emit

`MathSequence` + `MathDelimited` consumidos por handlers pré-existentes
(`layout_delimited` desde P55+). **Zero alteração em emit**. Hash
`export.rs` preservado bit-exact pelo **19º passo consecutivo**.

---

## A.5 — Detecção de bugs latentes adicionais

7 cenários fronteira:

| Cenário | Resultado esperado |
|---|---|
| `$sin(x)$` | `MathSequence([MathOp(sin), MathDelimited((x))])` |
| `$sin(x + y)$` | args complexos preservados em body |
| `$lim(x)$` | `MathSequence([MathOp(lim, limits=true), MathDelimited((x))])` |
| `$sin()$` (args vazios) | só `MathOp(sin)` (sem MathDelimited vazio) |
| `$undef(x)$` | fallback `MathIdent("undef")` + args descartados (pré-existente; fora scope) |
| `$sin x$` (sem parens) | continua `MathOp(sin)` directo — regressão P301 preservada |
| Múltiplos args `$sin(x, y)$` | separados por `, ` (paridade vanilla) |

### A.5.1 — Lição metodológica P301 §A.5

**P301 §A.5 falhou cobertura cross-construct**: cenários combinando
features novas (auto-lookup) com sintaxe pré-existente (parens
`FuncCall`) não foram explicitamente testados. **Lição P302**:
A.5 deve incluir matrix de combinações feature × sintaxe.

---

## A.5' — Anti-reflexão N=12 cumulativo (P291-P302)

### A.5'.1 — Comparação paradigmas P288-P302

| Passo | Paradigma |
|---|---|
| P293-P299 | 9 paradigmas materializadores |
| P300 | retrospectivo (10º) |
| P301 | parser/eval modification (11º) |
| **P302** | **bug fix derivado de materialização anterior** (12º) |

P302 reaplica **§8.4 sub-padrão "bug latente fixed durante
materialização dependente"** — N=1 P288 (NBSP fix derivado P287
SmartQuote materialização); **N=2 P302** (sin parens fix derivado
P301 auto-lookup).

### A.5'.2 — A.0.0 N=9 reaplicação — magnitude média

Bug factual concreto + vanilla pattern claro. Magnitude **média**.

### A.5'.3 — Elementos estructuralmente novos

5 elementos:

1. **§8.4 N=2 cumulativo** — sub-padrão reaplicado genuinamente.
2. **§A.5 P301 falhou cobertura cross-construct** — lição
   metodológica registada.
3. **Reutilização variants existentes** — sem variants novos
   (MathSequence + MathDelimited).
4. **Paradigma "bug fix derivado"** consolidado N=2.
5. **A.0.0 magnitude média** — categoria distinta de baixa/alta;
   bug factual concreto.

### A.5'.4 — Decisão sobre promoção ADR meta

Candidatos:
- **§8.4 sub-padrão N=2** — longe de limiar N≥3.
- **§8.7' N=9** — magnitude média; adiamento standard P300/P301.
- **§8.3 N=12** candidato adiado.

**Decisão**: **0 ADRs meta novas**. Anti-padrão P273.17 §0
honrado pela **10.ª vez consecutiva** (P293-P302).

---

## §Métricas do impacto

| Métrica | Antes | Pós-P302 |
|---|---:|---:|
| `Content` variants | 69 | 69 (inalterado) |
| Stdlib funcs | inalterado | inalterado |
| Hash `export.rs` | `66cb8ac3` | **preservado** (19º passo) |
| Hash `content.rs` | `82d3c47d` | preservado |
| Padrão §8.6 A.5' N | 11 | **12** (P291-P302) |
| Padrão §8.7' A.0.0 N | 8 | **9** (P302 reaplica) |
| Padrão §8.3 N candidato | 12 | 12 (sem aplicação P302) |
| Sub-padrão §8.4 "bug latente fixed" N | 1 (P288) | **2** (P288+P302) |
| ADRs novas | 0 | 0 |

---

## §Fecho da Fase A

Inventário literal completo + A.0.0 N=9 magnitude média + HZ
confirmado + decisão **(a) MathSequence + MathDelimited + (β)
preservação args** + A.5 cenários + A.5' N=12. **0 ADRs meta
novas** — anti-padrão honrado **10.ª vez consecutiva**.

**MARCO P302**:
- **Sub-padrão §8.4 N=2 cumulativo** (P288 + P302) — "bug latente
  fixed durante materialização dependente".
- **A.0.0 N=9 magnitude média** — bug factual concreto; vanilla
  HZ confirmado.
- **Lição metodológica P301 §A.5 falhou cross-construct**
  registada — A.5 deve incluir matrix feature × sintaxe.
- **Hash `export.rs` preservado pelo 19º passo consecutivo** —
  ADR-0098 robusta sobre 19 features distintas.
- **Reutilização variants existentes** (MathSequence + MathDelimited)
  — paridade vanilla via composição sem novos variants.

Procede-se a §3 da spec (com plano HZ + (a) + (β)).
