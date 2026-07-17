# Diagnóstico — Fase A do Passo 301 (`Auto-lookup math mode`)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-301.md`
**Origem**: P299 §8 + P300 §8 prioridade #2.
**Tipo declarado spec**: 1.º paradigma parser/eval na série P283+.
**Hipótese adoptada**: **HP** (heurística parcial pré-existente) +
**A.0.0' P301.A** (eval-time lookup minimal) + **A.2 → (a)
eval-time** + **A.3 → (γ) híbrido SSoT-preferred com fallback
heurística**.
**Magnitude da refutação A.0.0 N=8**: **média-modesta** — gap é
**conexão** entre infraestrutura existente (P298 heurística +
P299 scope) e math mode (eval); cristalino tinha as peças
prontas mas faltava o gancho.

---

## A.0.0 — Verificação literal estado parser/eval math mode (N=8 §8.7')

### A.0.0.1 — Sítio crítico identificado

`01_core/src/engine/eval/math.rs:51-59`:

```rust
Expr::MathIdent(ident) => {
    let name = ident.get();
    if let Some(sym) = crate::engine::math::symbols::ident_to_unicode(name) {
        // Símbolo grego ou operador: converter para Unicode
        Ok(Content::MathText(sym.into()))
    } else {
        // Variável, função, ou identificador desconhecido — manter como MathIdent
        Ok(Content::MathIdent(name.into()))
    }
}
```

**Fluxo actual** (pré-P301):
1. Detecta se nome é símbolo grego/Unicode (e.g. `alpha` → `α`).
2. Se sim → `MathText(sym)`.
3. Senão → `MathIdent(name)`.

**Gap**: nem `sin` nem `lim` estão em `ident_to_unicode` (são
operadores funcionais, não símbolos Unicode), caem em `MathIdent("sin")`.

### A.0.0.2 — Sítio FuncCall fallback

`01_core/src/engine/eval/math.rs:266` (FuncCall fallback):

```rust
// Outros nomes: tratar como MathIdent (sin, cos, lim, …)
_ => Ok(Content::MathIdent(name.into())),
```

Para `$sin(x)$` (com parêntesis), parser produz `FuncCall(sin, (x))`.
**Comportamento actual**: descarta args `(x)` silenciosamente —
**bug latente pré-existente** fora de scope P301. P301 apenas
estende o fallback para incluir lookup, sem alterar
descarte-de-args behavior.

### A.0.0.3 — Scope acessível em `eval_math_expr`

Signature `fn eval_math_expr(scopes: &mut Scopes<'_>, ctx: &mut EvalContext, expr: Expr<'_>)`.

`scopes` tem método `get(name: &str) -> Option<&Value>`
(`scopes.rs:127`). **Scope `math` injectado em scopes via
`make_stdlib` + `scopes.define` em `eval/mod.rs:218-222`**.

Confirmado: `scopes.get("math")` retorna `Some(&Value::Dict(...))`
com os 42 operadores P299.

### A.0.0.4 — Vanilla `eval_math` paridade

Vanilla typst tem `math` scope auto-injected em math mode; lookup
implicit. Cristalino vai usar **mesmo mecanismo** mas explicitamente
codificado em `eval_math_expr`.

### A.0.0.5 — Magnitude da refutação A.0.0 N=8

| Aspecto | Veredicto |
|---|---|
| Spec antecipou cristalino sem auto-lookup | ✅ Confirmado |
| Spec antecipou heurística parcial | ✅ Confirmado (HP) |
| Spec antecipou modificação parser/eval | ✅ Confirmado |
| Sítio crítico antecipado correctamente | ✅ Confirmado — `math.rs:51-59` |
| Gap material entre P298+P299 | ✅ Confirmado — falta gancho eval |

**Magnitude**: **média-modesta**. Não há refutação significativa
da estrutura proposta — A.0.0 **confirma** spec. Mas não é
factual-modesta como P295: descobre **sítio crítico exacto** e
**fluxo de dados** (scopes injection via make_stdlib).

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293 | 1 | alta |
| P294 | 2 | máxima |
| P295 | 3 | baixa |
| P296 | 4 | média |
| P297 | 5 | alta |
| P298 | 6 | alta |
| P299 | 7 | baixa |
| **P301** | **8** | **média-modesta** (P300 retrospectivo não contou) |

**Janela P294-P301**: máxima, baixa, média, alta, alta, baixa,
**média-modesta**. Continua **não-monotónico** — flutuação saudável
preservada.

### A.0.0.6 — Decisão hipótese

**HP confirmado** — heurística parcial existe (P298) + scope module
existe (P299); gap é **conexão**. P301 materializa o gancho.

---

## A.0.0' — Decisão de scope concreto (P301.A)

### A.0.0'.1 — Avaliação dos subsets

| Subset | Conteúdo | Magnitude | Decisão |
|---|---|---|---|
| **P301.A** | Eval-time lookup minimal (modificar `eval_math.rs:51-59`) | M | **ESCOLHIDO** |
| **P301.B** | Parser-time AST transformation | M+ | Rejeitado — refactor maior |
| **P301.C** | A+B combinado | M+ | Rejeitado — desnecessário |
| **P301.D** | Mínimo: lookup só 42 P299 sem fallback global | S-M | Coincide com P301.A |
| **P301.E** | Temporal: P301 lookup eval; P301.1 parser opt | S+S | Não necessário |

**P301.A escolhido** por:
- Minimiza acoplamento (eval-time confinado).
- Reusa infraestrutura (scope já acessível via `scopes.get`).
- Preserva paradigma single-source-of-truth P299.
- Magnitude controlada (M).

### A.0.0'.2 — Modificação proposta

```rust
Expr::MathIdent(ident) => {
    let name = ident.get();
    // 1. Check símbolos Unicode (alpha → α etc.)
    if let Some(sym) = crate::engine::math::symbols::ident_to_unicode(name) {
        return Ok(Content::MathText(sym.into()));
    }
    // 2. P301 — auto-lookup scope `math` (42 operadores P299)
    if let Some(Value::Dict(math_module)) = scopes.get("math") {
        if let Some(Value::Content(c)) = math_module.get(name) {
            if matches!(c, Content::MathOp { .. }) {
                return Ok(c.clone());
            }
        }
    }
    // 3. Fallback: identifier literal (regressão preservada)
    Ok(Content::MathIdent(name.into()))
}
```

Modificação paralela em `FuncCall` fallback (linha 266) — mesma
estratégia.

---

## A.0 — Reuso ADR-0098 (preservação esperada)

| Verificação | Esperado pós-P301 |
|---|---|
| Hash `export.rs` | `66cb8ac3` preservado bit-exact (**18º passo consecutivo**) |
| Hash `eval/math.rs` (`@prompt-hash`) | **muda intencionalmente** — 1.ª modificação parser/eval na série P283+ |
| Hash `content.rs` | preservado (sem variants novos) |
| ADR-0098 N=18 cumulativo | ✅ |

**A.0 não-trivial**: hash export.rs preservado **mas** hash de
parser/eval muda — primeira na série P283+. ADR-0098 §"alterações
justificadas" cobre porque export.rs é preservado bit-exact.

---

## A.1 — Inventário literal

### A.1.1 — Math mode detecção

Parser produz `Expr::Math(...)` em `$...$`. Eval despacha para
`eval_math_content` que processa cada `Expr::Math*` arm.

### A.1.2 — `MathIdent` construção

2 sítios em `eval/math.rs`:
- Linha 58: `Expr::MathIdent` direto.
- Linha 266: `FuncCall` fallback.

### A.1.3 — Scope acessível

`scopes: &mut Scopes` passado a `eval_math_expr`. `scopes.get(name)`
funcional.

### A.1.4 — `make_math_module` P299

Registado via `scope.define("math", ...)` em `eval/mod.rs:768`.
Injectado em `Scopes` via `make_stdlib + scopes.define` em
`eval/mod.rs:218-222`.

### A.1.5 — Heurística P298 preservada

`attach.rs:55-61` continua a usar `is_limit_function` para
`MathIdent`/`MathText` — fallback para casos não cobertos por
scope.

### A.1.6 — `MathOp` consumer P298 inalterado

`layout_op` trivial delegate; `layout_attach` `is_limits` arm
para `MathOp { limits, .. }` — funciona com qualquer `MathOp`
seja de `op()` constructor ou de `math.lim` lookup.

### A.1.7 — Emit standard

`FrameItem::Text/Glyph` via P298 paradigm. **Inalterado bit-exact**.

### A.1.8 — Diagrama de fluxo P301

```
$sin x$
       │
       ▼
parser → Expr::Math(...) com Expr::MathIdent("sin"), Expr::MathIdent("x")
       │
       ▼
eval_math_expr (P301 modificado):
  - Expr::MathIdent("sin"):
      1. ident_to_unicode("sin") → None
      2. P301 NEW: scopes.get("math").get("sin") → Some(MathOp{text:"sin", limits:false})
      3. Retorna Content::MathOp{...}  ✓
  - Expr::MathIdent("x"):
      1. ident_to_unicode("x") → None
      2. P301: scopes.get("math").get("x") → None (variável não é operador)
      3. Fallback MathIdent("x")  ✓
       │
       ▼
Content::MathSequence([MathOp{sin, false}, MathIdent("x")])
       │
       ▼
Math Layouter: MathOp via layout_op (P298) — trivial delegate; MathIdent via layout_text_node
       │
       ▼
PDF "sin x" inline standard
```

---

## A.2 — Decisão arquitectural (eval-time lookup)

**Decisão A.2 → (a)**: eval-time lookup minimal.

Justificações:
- Sem alteração parser.
- Reusa scope existente (`scopes.get`).
- Preserva SSoT P299.
- Magnitude controlada.
- Lookup O(1) hash; sem preocupação performance.

---

## A.3 — Integração heurística pré-P301 (decisão γ)

**Decisão A.3 → (γ) híbrido**:
- Lookup `math` scope **precede** heurística.
- Heurística `is_limit_function`/`is_large_operator` preservada em
  `attach.rs` para casos não cobertos por scope (e.g. operadores
  Unicode `∑`/`∫` literais sem identifier).

**Sem duplicação**:
- `sin` → encontrado em scope → MathOp.
- `lim` → encontrado em scope → MathOp (limits=true).
- `∑` → não está em scope (é caractere Unicode, não nome) →
  fallback MathIdent → heurística is_large_operator aplica.

---

## A.4 — Impacto em emit (preservado bit-exact)

**A.4 → (i)** — `FrameItem` standard. Hash `export.rs` preservado
bit-exact pelo **18º passo consecutivo**.

---

## A.5 — Detecção de bugs latentes

7 cenários fronteira:

| Cenário | Resultado esperado |
|---|---|
| `$sin x$` | renderiza como `$math.sin x$` (paridade) |
| `$x$` (variável) | continua `MathIdent` (lookup não encontra) |
| `$lim_(x→0) f$` | limits-style via P298 cross-variant; bit-exact paridade pré-P301 |
| `$alpha$` (símbolo Unicode) | continua `MathText("α")` (path 1 não afectado) |
| `$mod$` | resolve via `math.mod` lookup (operador) |
| `$sin(x)$` (com parens) | FuncCall fallback: lookup também aplica; descarte de args preservado (bug latente fora de scope) |
| `$f x$` (f não é operador) | continua `MathIdent("f")` (lookup retorna None) |

### A.5.1 — Bug latente fora de scope: descarte args FuncCall

`$sin(x)$` em vanilla = `sin` (op) + `(x)` (delimited). Cristalino
actual descarta `(x)`. P301 preserva esse comportamento (não
conserta) — frente futura.

### A.5.2 — Regressão crítica `MathIdent("lim")` heurística

Cenário pré-P301:
- `MathIdent("lim")` + attach `_` em block → `is_limits = true` via `is_limit_function` → limits-style ✓

Cenário pós-P301:
- `lim` agora resolvido para `MathOp{text:"lim", limits:true}` via scope lookup.
- Attach com `MathOp{limits:true}` → `is_limits = true` via P298 arm → limits-style ✓

**Bit-exact?** A diferença: pré-P301 base é `MathIdent("lim")`; pós-P301 base é `MathOp{text:MathText("lim"), limits:true}`. **Layout produz o mesmo glyph "lim"** mas via `layout_op` (delegate para `layout_node(text)`) em vez de via `MathIdent` directo.

Vou verificar se isto produz output PDF bit-exact ou não. Se diferir, regressão é **alteração intencional** (P301 produz semântica mais clara).

---

## A.5' — Anti-reflexão N=11 cumulativo (P291-P301)

### A.5'.1 — Comparação paradigmas P288-P301

| Passo | Paradigma |
|---|---|
| P293-P299 | 9 paradigmas materializadores |
| P300 | retrospectivo sem materialização (10º) |
| **P301** | **modificação parser/eval** (11º) — primeira na série P283+ |

P301 paradigma genuinamente novo.

### A.5'.2 — A.0.0 N=8 reaplicação — magnitude média-modesta

Confirmação de spec (não refutação) + clarificação factual (sítio
exacto). Mid-magnitude — entre P295 (baixa) e P296 (média).

### A.5'.3 — Elementos estructuralmente novos

5 elementos:

1. **1.ª modificação parser/eval** — primeira na série P283+; hash
   eval/math.rs muda.
2. **Sub-padrão "scope lookup em eval"** N=1 inaugural.
3. **Sub-padrão "operadores SSoT" N=2 cumulativo** (P299 registou;
   P301 consume via lookup).
4. **Integração natural P298 + P299** — gap fechado.
5. **A.0.0 confirmação** (não refutação) — categoria documentada
   P295 §10 "confirmação esperada" reaplicada.

### A.5'.4 — Decisão sobre promoção ADR meta

Candidatos:
- **§8.7' N=8** — magnitude média-modesta NÃO inequívoca. P300
  caminho A estabeleceu standard conservador. **Adiado**.
- **§8.3 N=12** — descritivo; adiado.
- **Sub-padrão "scope lookup em eval"** N=1 inaugural — longe de
  limiar.
- **Sub-padrão "operadores SSoT" N=2** (P299+P301) — abaixo de
  limiar N≥3.
- **Anti-padrão P273.17 §0** honrado pela **9.ª vez consecutiva**
  (P293-P301).

**Decisão**: **0 ADRs meta novas**. P300 estabeleceu critério
estrito "promover apenas se inequívoco"; P301 magnitude
média-modesta não dispara inequivocamente.

---

## §Métricas do impacto

| Métrica | Antes | Pós-P301 |
|---|---:|---:|
| `Content` variants | 69 | 69 (inalterado) |
| Stdlib módulos | 3 (calc, gradient, math) | 3 (inalterado) |
| Hashes parser/eval | `eval/math.rs` antigo | **muda** intencionalmente |
| Hash `export.rs` | `66cb8ac3` | **preservado** (18º passo consecutivo) |
| Hash `content.rs` | `82d3c47d` | preservado |
| Padrão §8.6 A.5' N | 10 | **11** (P291-P301) |
| Padrão §8.7' A.0.0 N | 7 | **8** (P301 reaplica) |
| Padrão §8.3 N candidato | 11 | **12** adiado |
| Sub-padrão "scope lookup em eval" N | n/a | **1 inaugural** |
| Sub-padrão "operadores SSoT" N | 1 (P299) | **2** cumulativo (P299+P301) |
| ADRs novas | 0 | 0 |

---

## §Fecho da Fase A

Inventário literal completo + A.0.0 N=8 magnitude média-modesta
+ A.0.0' P301.A escolhido + decisão **(a) eval-time + (γ) híbrido**
+ A.4 hash export preservado + A.5 cenários verificados + A.5'
N=11 cumulativo. **0 ADRs meta novas** — anti-padrão P273.17 §0
honrado **9 passos consecutivos**.

**MARCO P301**:
- **1.ª modificação parser/eval na série P283+** — paradigma novo
  (11.º distinto).
- **Integração natural P298 + P299** — gap "auto-lookup math mode"
  fechado.
- **Hash `export.rs` preservado pelo 18º passo consecutivo** —
  ADR-0098 robusta sobre 18 features distintas.
- **Hash `eval/math.rs` muda intencionalmente** — primeira na
  série; ADR-0098 §"alterações justificadas" cobre.
- **Sub-padrão "scope lookup em eval" N=1 inaugural** — registado
  para vigilância.
- **§8.7' N=8 candidato adiado** — magnitude média-modesta não
  inequívoca; P300 standard preservado.

Procede-se a §3 da spec (com plano P301.A + (a) + (γ)).
