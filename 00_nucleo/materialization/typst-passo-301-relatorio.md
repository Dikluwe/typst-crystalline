# Relatório — Passo 301 (`Auto-lookup math mode`)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-301.md`
**Diagnóstico Fase A**: `00_nucleo/diagnosticos/diagnostico-auto-lookup-math-passo-301.md`
**Tipo declarado spec**: 1.ª modificação parser/eval na série P283+
(paradigma genuinamente novo); magnitude M esperada.
**Hipótese adoptada**: **HP** (heurística parcial pré-existente) +
**A.0.0' P301.A** (eval-time lookup minimal) + **A.2 → (a)
eval-time** + **A.3 → (γ) híbrido SSoT-preferred com fallback
heurística**.
**Baseline P300**: 2 862 testes  →  **P301**: 2 870 testes (Δ = +8)
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**18º passo
consecutivo**: P282→P301)
**Hash `content.rs`**: `82d3c47d` inalterado
**ADRs meta novas**: 0

---

## §1 — Sumário executivo

P301 implementa **auto-lookup math mode**: quando identifier
`sin`/`lim`/etc. aparece em `$...$`, cristalino agora consulta o
scope `math` (materializado P299) e substitui por `Content::MathOp`
automaticamente — paridade vanilla typst onde `$sin x$` é
equivalente a `$math.sin x$`.

**Modificação localizada** em `01_core/src/rules/eval/math.rs`:
- Helper `lookup_math_op()` novo.
- 2 sítios estendidos: `Expr::MathIdent` arm + `FuncCall` fallback.
- Heurística P298 preservada em `attach.rs` (fallback para
  operadores Unicode literais `∑`/`∫`).

**Resultado funcional**: `$sin x$` agora produz `MathOp` semântica
operador (não `MathIdent` identifier); P298 layout handler consume
naturalmente; P298 cross-variant `is_limits` aplica-se a `$lim_(x→0) f$`.

**Resultado metodológico — categoria "confirmação esperada"
reaplicada**: A.0.0 N=8 confirmou a spec (não refutou) e
identificou sítio crítico exacto. Magnitude **média-modesta** —
entre P295 (baixa) e P296 (média). Janela P294-P301 continua
não-monotónica: máxima, baixa, média, alta, alta, baixa,
**média-modesta**.

**Integração natural P298 + P299 fechada** — gap "auto-lookup math
mode" preenchido sem alterar contrato L0:

- P298 materializou `Content::MathOp` + cross-variant em `attach.rs`.
- P299 materializou `make_math_module()` com 42 operadores SSoT.
- **P301 conecta**: `$sin x$` → scope lookup → `Content::MathOp` →
  consumido por P298 layout + cross-variant.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=8 reaplica §8.7') | Confirmação da spec; sítio crítico em `math.rs:51-59`; magnitude **média-modesta** |
| A.0.0' (P301.A escolhido) | Eval-time lookup minimal — paralelo P299 subdivision decision |
| A.0 (ADR-0098) | ✅ `export.rs` preservado bit-exact (18º passo) |
| A.1 inventário | Scope `math` injectado via `make_stdlib` + `scopes.define`; lookup O(1) hash |
| A.2 decisão | **(a) eval-time** — sem alteração parser; reusa scope existente |
| A.3 integração | **(γ) híbrido** — lookup precede heurística; sem duplicação |
| A.4 emit | **(i) `FrameItem` standard**; hash preservado |
| A.5 bugs latentes | 7 cenários verificados; regressão `MathIdent("lim")` preservada via P298 cross-variant arm |
| A.5' anti-reflexão | **N=11 cumulativo** (P291-P301); 5 elementos novos |

Detalhe completo: `00_nucleo/diagnosticos/diagnostico-auto-lookup-math-passo-301.md`.

---

## §3 — Materialização

### §3.1 — `01_core/src/rules/eval/math.rs` — helper `lookup_math_op`

```rust
use crate::entities::value::Value;

/// Lookup helper: consulta scope `math` (P299) e retorna `MathOp`
/// clone se encontrado. None caso contrário.
fn lookup_math_op(scopes: &Scopes<'_>, name: &str) -> Option<Content> {
    let Value::Dict(math_module) = scopes.get("math")? else {
        return None;
    };
    let Value::Content(c) = math_module.get(name)? else {
        return None;
    };
    if matches!(c, Content::MathOp { .. }) {
        Some(c.clone())
    } else {
        None
    }
}
```

### §3.2 — `Expr::MathIdent` arm — 3 etapas ordenadas

```rust
Expr::MathIdent(ident) => {
    let name = ident.get();
    // 1. Símbolo grego ou operador Unicode (alpha → α etc.)
    if let Some(sym) = crate::rules::math::symbols::ident_to_unicode(name) {
        return Ok(Content::MathText(sym.into()));
    }
    // 2. P301 — auto-lookup scope `math` (42 operadores P299 via SSoT MathOp).
    if let Some(op) = lookup_math_op(scopes, name) {
        return Ok(op);
    }
    // 3. Fallback: variável, função, ou identificador desconhecido
    //    — manter como MathIdent (regressão pré-P301 preservada).
    Ok(Content::MathIdent(name.into()))
}
```

### §3.3 — `FuncCall` fallback — paralelo

```rust
// Outros nomes: P301 auto-lookup math (sin, cos, lim, …);
// fallback MathIdent.
_ => {
    if let Some(op) = lookup_math_op(scopes, &name) {
        Ok(op)
    } else {
        Ok(Content::MathIdent(name.into()))
    }
}
```

### §3.4 — Zero alterações em outros sítios

| Componente | Pós-P301 |
|---|---|
| `01_core/src/entities/content.rs` | **Inalterado** — hash `82d3c47d` preservado |
| `01_core/src/rules/math/symbols.rs` | **Inalterado** — `is_limit_function`/`is_large_operator` preservados |
| `01_core/src/rules/math/layout/attach.rs` | **Inalterado** — P298 cross-variant arm continua |
| `01_core/src/rules/stdlib/structural.rs` | **Inalterado** — `make_math_module` P299 preservado |
| `03_infra/src/export.rs` | **Inalterado bit-exact** — hash `66cb8ac3` (18º passo) |
| L0 `rules/eval.md` | **Inalterado** — P301 é extensão dentro do contrato L0 |

---

## §4 — Testes

### §4.1 — `01_core/src/rules/eval/tests.rs` (+8 testes L1)

| Teste | Verifica |
|---|---|
| `p301_sin_resolve_para_mathop_scripts_style` | `$sin x$` → `MathOp { text:"sin", limits:false }` |
| `p301_lim_resolve_para_mathop_limits_style` | `$lim x$` → `MathOp { limits:true }` |
| **`p301_det_resolve_para_mathop_novo`** | `det` (não em heurística pré-P301) agora resolve via scope |
| `p301_variavel_x_continua_mathident` | `$x$` → MathText (single letter; não-operador) |
| `p301_symbol_unicode_alpha_continua_mathtext` | `$alpha$` → `MathText("α")` (path 1 não afectado) |
| `p301_funcao_user_f_continua_mathident` | `$f$` → MathText (não em scope math) |
| **`p301_regressao_mathident_lim_attach_limits_style`** | **INVARIANTE CRÍTICA**: `$lim_(n) f$` continua limits-style (via P298 cross-variant) |
| `p301_multiplos_operadores_resolvidos` | `$sin x + cos y$` resolve múltiplos |

### §4.2 — Helpers de teste estendidos

`find_mathop_in()` e `find_mathident_in()` estendidos para recursar
em `MathAttach`:

```rust
Content::MathAttach { base, sub, sup, tl, bl } => {
    find_mathop_in(base)
        .or_else(|| sub.as_deref().and_then(find_mathop_in))
        // ...
}
```

**Descoberta empírica colateral**: `$x$` em math mode produz
`MathText`, não `MathIdent` (single-letter é tratado como text
italic). Helper ajustado para aceitar ambos.

### §4.3 — Sem alterações em L3

P301 não modifica emit. Tests L3 existentes preservados.

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2366 passed; 0 failed; 0 ignored
test result: ok.  457 passed; 0 failed; 6 ignored
test result: ok.   24 passed; 0 failed; 0 ignored
test result: ok.    2 passed; 0 failed; 0 ignored
test result: ok.   21 passed; 0 failed; 0 ignored
                  -----
                  2870 passed total
```

Baseline P300 = 2 862; delta = **+8** (8 L1, 0 L3) ✓.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — `crystalline-lint --fix-hashes`

```
Nothing to fix
```

L0 `rules/eval.md` **inalterado** — P301 é **extensão dentro do
contrato L0 existente**, não modificação de interface. Hash
`@prompt-hash 19073424` no header de `eval/math.rs` continua
válido.

### §5.4 — Hashes pós-P301

| Ficheiro L0 / código | Antes P301 | Pós P301 |
|---|---|---|
| `rules/eval.md` | hash `19073424` | **inalterado** |
| `eval/math.rs` (`@prompt-hash`) | `19073424` | `19073424` inalterado |
| `entities/content.rs` (`@prompt-hash`) | `82d3c47d` | inalterado |
| `infra/export.rs` (`@prompt-hash`) | `66cb8ac3` | **`66cb8ac3` preservado bit-exact** (**18º passo consecutivo**) |

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — N=8 confirmação esperada

**Janela completa P293-P301**:

```
P293: ████████░░ alta     ← H6 não-listada
P294: ██████████ máxima   ← Spec inteira invalidada
P295: ██░░░░░░░░ baixa    ← Linha tabela admin
P296: ██████░░░░ média    ← Classificação inválida
P297: ████████░░ alta     ← Wrapper vanilla inexistente
P298: ████████░░ alta     ← Heurística limits já existia
P299: ███░░░░░░░ baixa    ← Calc module precedente
P300: (retrospectivo; sem A.0.0 magnitude)
P301: ████░░░░░░ média-modesta ← Spec confirmada; sítio identificado
```

**Trend não-monotónico saudável**. P301 reaplica **categoria
"confirmação esperada"** inaugurada P295 §10 — A.0.0 valida spec
(sítio exacto identificado) sem refutar estrutura.

**§8.7' N=8 adiado** seguindo standard P300 conservador. P273.17 §0
honrado.

### §6.2 — §8.3 "refutação pragmática" (**N=12 candidato adiado**)

P301 confirma spec (não refuta). §8.3 não dispara em P301.

### §6.3 — §8.6 "A.5' anti-reflexão" (**N=11 cumulativo**)

P291-P301. Limiar passado; anti-padrão adia.

### §6.4 — **Sub-padrão "scope lookup em eval" — N=1 inaugural**

P301 inaugura paradigma: scope `math` consultado durante eval para
substituir identifier por valor pré-definido.

**Possíveis reaplicações futuras**:
- `text` scope (caso `make_text_module` materialize).
- Outros scope modules acessados implicitamente em modes específicos.

**N=1 inaugural**. Longe de limiar tentativo N≥3.

### §6.5 — **Sub-padrão "operadores pré-definidos via SSoT" — N=2 cumulativo**

P299 inaugurou (registo); P301 reaplica (consume via lookup).
Confirma que `Content::MathOp` é genuinamente **single source of
truth** para operadores math:

- **Registo** (P299): `make_math_module()` cria 42 entries
  `Value::Content(MathOp{...})`.
- **Consumo via prefix** (P299): `math.sin` field access.
- **Consumo via lookup automático** (P301): `$sin x$` resolve para
  o mesmo MathOp via scope lookup.

**N=2 cumulativo**. Próximo passo se atingir N=3 com terceiro
caminho de consumo (e.g. show rules sobre math identifiers).

### §6.6 — ADR-0098 "single source of truth" (**N=18 cumulativo**)

Hash `export.rs` preservado bit-exact pelos **18 passos
consecutivos** P282-P301. Invariante robusta sobre 18 features
distintas. P301 é o **18.º passo** — preservação por paradigma
"modificação eval interna sem alterar emit".

### §6.7 — Anti-padrão P273.17 §0 — 9 passos consecutivos honrados

**0 ADRs meta promovidas P293-P301** (9 passos):

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293-P299 | 17 candidatos cumulativos | 0 |
| P300 retrospectivo | 8 (auditoria) | 0 |
| **P301** | **§8.7' N=8 + sub-lookup eval N=1** | **0** |

**9.ª vez consecutiva** anti-padrão honrado.

### §6.8 — Categoria "confirmação esperada" reaplicada (P295 §10)

P295 §10 inaugurou: A.0.0 pode confirmar spec sem refutar; magnitude
baixa-modesta é resultado legítimo, não ritualismo.

**P301 reaplica genuinamente**:
- Spec antecipou HP (heurística parcial); A.0.0 confirmou.
- Spec antecipou modificação parser/eval; A.0.0 confirmou sítio
  exacto.
- Spec antecipou export.rs preservado; A.0.0 confirmou via inspecção.

**Valor da inspecção A.0.0 mesmo em magnitude média-modesta**:
identificou sítio crítico exacto (`math.rs:51-59` + `:266`); evitou
implementação cega que poderia tocar mais código que necessário.

---

## §7 — Cobertura vanilla vs cristalino

P301 não altera estrutura de tabelas. **Integração natural** P298
+ P299 fechada:

| Antes P301 | Pós P301 |
|---|---|
| `$sin x$` → `MathIdent("sin")` literal; renderiza visualmente como operador "por acidente" | `$sin x$` → `MathOp { text:"sin", limits:false }`; **semanticamente operador** |
| `$lim_(x→0) f$` → `MathAttach { base: MathIdent("lim") }`; limits-style via heurística `is_limit_function` | `$lim_(x→0) f$` → `MathAttach { base: MathOp { limits:true } }`; limits-style via P298 cross-variant arm |
| `math.sin x` (prefix explícito) — único caminho user-facing | `math.sin x` **OU** `sin x` — ambos resolvem identicamente |

**Paridade vanilla atingida** para os 42 operadores P299.

---

## §8 — Frentes pendentes pós-P301

P301 fecha frente "Auto-lookup math mode" (P300 §8 prioridade #2).

**Frentes restantes** (catálogo em `frentes-pendentes-pos-p299.md`):

| Frente | Magnitude | Estado |
|---|---|---|
| **P295.1** nota corpo no rodapé | L | P300 prioridade #1 |
| P296.X toggles cancel | XS | refino cluster math |
| P297.X UnderoverKind | XS+ | cosmético |
| Cosméticos ADR-0054 graded (vários) | XS individual | agregável |
| Show rules sobre math identifiers | M+ | bloqueado por regex em L1 |
| `sin(x)` parens descartados | XS+ | bug latente fora de scope P301 |

---

## §9 — Decisão sobre P302

P301 fecha frente significativa. P302 disponível para qualquer
frente pendente:

1. **P295.1 nota corpo no rodapé** — fecha cluster footnote
   (magnitude L).
2. **Bug latente `sin(x)` parens** — refino XS+ derivado P301.
3. **Cosméticos cleanup agregado** — múltiplos XS num passo.
4. **Frente totalmente nova** — fora do escopo cumulativo P283-P301.

Decisão fica para o operador humano.

---

## §10 — Honestidade epistémica

### §10.1 — Categoria "confirmação esperada" valida-se cumulativamente

P301 demonstra que **A.0.0 magnitude média-modesta** continua a
gerar valor empírico:

- Sem A.0.0: implementação cega poderia ter tocado parser, scope
  injection, ou outros sítios desnecessariamente.
- Com A.0.0: modificação confinada a 2 sítios em `eval/math.rs` +
  1 helper.

**Lição**: A.0.0 não precisa de magnitude alta para ser útil.
"Confirmação esperada" é resultado legítimo.

### §10.2 — Sub-padrões emergentes vs promoção

P301 introduz 2 sub-padrões novos:
- "Scope lookup em eval" N=1.
- "Operadores SSoT" N=2 cumulativo.

**Decisão de não promover**: N pequenos não justificam ADR formal.
Sub-padrões preservados como **ferramentas emergentes**.

Próxima reavaliação: N=3+ com aplicações genuínas diferentes.

### §10.3 — L0 inalterado é virtude, não esquecimento

P301 não tocou `rules/eval.md` L0. Não é esquecimento — é
**confirmação de que a extensão está dentro do contrato L0
existente**.

Se P301 tivesse mudado interface (e.g. `eval_math_expr` ganhou
parâmetro novo), L0 teria que reflectir. Como apenas estendeu
comportamento interno, L0 inalterado é correcto.

---

## §11 — Fecho

P301 fechado com:

- **+8 testes** (8 L1, 0 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **0 drift** em hashes (`--fix-hashes` "Nothing to fix").
- **Hash `export.rs` preservado** bit-exact (18º passo consecutivo).
- **Hash `content.rs` inalterado**.
- **L0 `rules/eval.md` inalterado** — extensão dentro do contrato.
- **0 ADRs meta novas** — 9.ª vez consecutiva anti-padrão honrado.
- **Sub-padrão "scope lookup em eval" N=1 inaugural** registado.
- **Sub-padrão "operadores SSoT" N=2 cumulativo** confirmado.

**MARCO P301**:
- **1.ª modificação parser/eval na série P283+** — paradigma novo
  (11.º distinto na sequência cumulativa).
- **Frente "Auto-lookup math mode" resolvida** — P300 §8
  prioridade #2 fechada.
- **Integração natural P298 + P299 completa** — `$sin x$` agora
  resolve via SSoT MathOp pelo mesmo caminho que `math.sin`.
- **Categoria "confirmação esperada" P295 §10 reaplicada
  genuinamente** — A.0.0 magnitude média-modesta produz valor
  empírico (sítio crítico exacto identificado).
- **Hash `export.rs` preservado pelo 18º passo consecutivo**
  (P282→P301) — ADR-0098 robusta sobre 18 features distintas.
- **Heurística P298 preservada** em `attach.rs` como fallback —
  P301 estende sem substituir.
- **Regressão `MathIdent("lim")` paths preservada** — `$lim_(n) f$`
  continua limits-style via P298 cross-variant arm (em vez de
  `is_limit_function` direto).
- **Anti-padrão P273.17 §0 honrado pela 9.ª vez consecutiva**.

**Lição final**: P301 prova que extensões parser/eval podem ser
**localizadas e mínimas** se infraestrutura suporta (P298+P299
prepararam o terreno; P301 ligou os pontos).
