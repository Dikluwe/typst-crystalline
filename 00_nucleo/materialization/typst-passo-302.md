# Passo 302 — Bug fix `sin(x)` parens descartados

**Frente**: Bug latente `sin(x)` parens — descoberto colateral
em P301 §9.
**Origem**: P301 §3.3 — `FuncCall` fallback consultou
`lookup_math_op` mas **descartou silenciosamente** os args
`(x)` quando lookup encontra operador. P301 §9 registou como
frente pendente XS+ derivada.
**Pré-requisitos**: P301 (`lookup_math_op` helper +
`FuncCall` fallback).
**Tipo declarado**: **bug latente fixed durante materialização
dependente** — reaplica sub-padrão P288 §8.4 (N=1: NBSP fix
descoberto via SmartQuote P287→P288). P302 seria **N=2** do
sub-padrão.
**Magnitude**: XS+ esperada; A.0.0 verifica empiricamente
comportamento vanilla.

---

## §1 — Objectivo

Resolver o bug introduzido por P301 §3.3: quando `$sin(x)$` (ou
qualquer `<op>(<args>)`) em math mode é parseado como `FuncCall`,
o fallback P301 retorna apenas o `MathOp` correspondente,
**ignorando silenciosamente os args**.

### §1.1 — Bug factual exposto

**Pré-P301**: `$sin(x)$` → `FuncCall("sin", [x])` → fallback
heurístico → `MathIdent("sin")` + parens preservados via
estrutura `FuncCall` → renderização `sin(x)` literal com parens.

**Pós-P301**: `$sin(x)$` → `FuncCall("sin", [x])` → P301
`lookup_math_op("sin")` → `MathOp { text: "sin", limits: false }`
→ **args `(x)` descartados**; renderização `sin` apenas.

**Bug**: regressão visual silenciosa. `sin(x)` user input produz
output PDF sem `(x)`.

### §1.2 — Comportamento vanilla a verificar (A.0.0)

`$sin(x)$` em vanilla typst produz **o quê**? Possibilidades:

| Hipótese | Comportamento |
|---|---|
| **HX** | `sin x` (parens são grouping sintáctico; descartados) |
| **HY** | `sin(x)` com parens delimitadores visíveis |
| **HZ** | Function call: `MathSequence([MathOp(sin), MathDelimited((x))])` |
| **HW** | Renderização específica vanilla a verificar |

**A.0.0 obrigatória inspecciona vanilla literal** + cristalino
fallback pré-P301 para decidir comportamento esperado.

### §1.3 — Riscos e expectativas

- **Bug detectado pós-P301 commit**: P301 §A.5 não cobriu o
  cenário `sin(x)` directamente. Falha de A.5 cumulativa.
- **Solução tem que preservar args** sem regredir `$sin x$` (sem
  parens) que P301 estabeleceu correctamente.
- **Magnitude verdadeira condicional a hipótese**: HX é trivial
  (descartar args mesmo); HY/HZ requerem preservação de
  estrutura.
- **§8.4 P288 sub-padrão N=2** — reaplicação genuína se P302
  fixa bug emergente de feature dependente, paralelo P288
  fixing NBSP de SmartQuote P287.

### §1.4 — Razões

1. **Resolver regressão visual silenciosa** — user input `sin(x)`
   produz output PDF sem `(x)`.
2. **Reaplica sub-padrão §8.4** "bug latente fixed durante
   materialização dependente" — N=2 cumulativo se aplicação
   genuína; aguarda N≥3 para promoção candidata.
3. **Validar processo retrospectivo P300** — P300 caminho A
   conservou flexibilidade; bugs descobertos pós-P301 testam
   se a flexibilidade compensa eventual rigor formal.
4. **Magnitude controlada XS+** — primeiro passo XS+ pós-P301
   M e P300 retrospectivo; reset metodológico legítimo.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 7 secções)

### A.0.0 — Verificação literal comportamento `$<op>(<args>)$` (N=9 cumulativo §8.7')

Inspecção literal obrigatória:

1. **Reproduzir bug actual**:
   ```rust
   let result = eval_math("$sin(x)$");
   // Verificar literalmente: contém args ou só "sin"?
   ```
2. **Inspeccionar vanilla**: `lab/typst-original/.../eval/math.rs`
   ou similar — como vanilla typst processa `$sin(x)$` em math
   mode.
3. **Verificar `sin(x)` parser AST**:
   - É `FuncCall("sin", [x])` ou `MathSequence([Ident("sin"), ParenExpr("x")])`?
   - Onde paren-expr é processado em math mode?
4. **Inspeccionar `eval/math.rs` pós-P301**:
   - Arm `FuncCall` linhas exactas.
   - Como args são processados em fallback (linha 116-128 do
     relatório P301).
5. **Testes pré-P301 com `sin(x)`** — algum teste existente que
   cobre este cenário? Se sim, está a passar incorrectamente
   (validando o bug)?
6. **Cross-check com outros operadores**: `cos(x)`, `lim(x)`,
   `tan(x)` — todos sofrem o mesmo bug?

**Decisão A.0.0 sobre hipótese HX/HY/HZ/HW**:

| Hipótese | Acção |
|---|---|
| **HX** (vanilla descarta parens) | P302 confirma comportamento P301 actual; **não há bug** — apenas testes |
| **HY** (parens visíveis) | P302 fixa: emite `MathOp(sin)` seguido de parens literais |
| **HZ** (function call structural) | P302 fixa: emite `MathSequence([MathOp(sin), MathDelimited(...)])` |
| **HW** | A.0.0 documenta empiricamente |

**Magnitude esperada A.0.0**: **média**. Bug factual concreto +
verificação empírica vanilla; sem ambiguidade fundamental.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado |
|---|---|
| `grep "sin\|MathOp.*Funccall" 03_infra/src/export.rs` | Zero hits funcionais — emit consome FrameItem agnóstico |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo **19º passo consecutivo** — P302 confinado a `eval/math.rs` |
| Hash `eval/math.rs` | **Muda intencionalmente** (modificação local do `FuncCall` arm) |
| Hash `content.rs` | **Preservado** (sem variants novos) |

### A.1 — Inventário literal

8 sub-secções:

1. **A.1.1 — `Expr::FuncCall` em math mode** — estrutura AST
   exacta.
2. **A.1.2 — Path P301 actual** — sítio exacto que descarta args
   (`eval/math.rs` §3.3 relatório P301).
3. **A.1.3 — `MathDelimited` ou equivalente** — variant para
   parens em math mode.
4. **A.1.4 — Vanilla equivalente** — como vanilla AST trata
   `sin(x)`.
5. **A.1.5 — `MathSequence` construção** — para HZ caso emit
   sequence.
6. **A.1.6 — Testes existentes** — algum teste cobre `sin(x)`?
7. **A.1.7 — Outros operadores** — `cos(x)`/`tan(x)`/`lim(x)` —
   mesmo bug confirmado.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente.

### A.2 — Estrutura da correcção

Decisão arquitectural condicional a A.0.0:

**Cenário HY/HZ (most likely)**:

| Opção | Mecanismo | Prós | Contras |
|---|---|---|---|
| **(a)** Retornar `MathSequence([MathOp(<op>), MathDelimited(<args>)])` | Estrutura clara; reusa variants existentes | Muda estrutura return |
| **(b)** Concatenar args manualmente em `MathOp.text` | Sem novos variants | Perde semântica (text "sin" ≠ "sin(x)") |
| **(c)** Não substituir `MathOp` em `FuncCall`; aplicar lookup só em `MathIdent` arm | Conservador; preserva fallback FuncCall original | `$sin(x)$` continua sem auto-lookup; gap residual |

Default sugerido: **(a)** se HY/HZ confirmadas; **(c)** se HX
confirmada (lookup só `MathIdent`, não `FuncCall`).

### A.3 — Integração com fallback heurístico (paridade P301 §3.3 γ)

`FuncCall` fallback P301 actual:
```rust
_ => {
    if let Some(op) = lookup_math_op(scopes, &name) {
        Ok(op)  // Bug: args descartados
    } else {
        Ok(Content::MathIdent(name.into()))
    }
}
```

P302 modifica este arm conforme A.2.

| Opção | Comportamento |
|---|---|
| **(α)** Auto-lookup em `FuncCall` apenas se `args.is_empty()` (i.e., `sin()` sem args) | Conservador; `sin(x)` preserva path pré-P301 |
| **(β)** Auto-lookup com preservação de args via `MathSequence` | Caminho completo HY/HZ |
| **(γ)** Auto-lookup remove args (HX confirmada) | Apenas se vanilla confirma comportamento |

Default sugerido: **(β)** se A.0.0 → HY/HZ; **(γ)** se A.0.0 →
HX. **(α)** rejeitada salvo se A.0.0 não conclusiva.

### A.4 — Impacto em emit (ADR-0098)

Emit agnóstico via `FrameItem::Text/Glyph`. Hash `export.rs`
preservado bit-exact pelo **19º passo consecutivo**.

### A.5 — Detecção de bugs latentes adicionais

Cenários fronteira:

- `$sin(x)$` — bug principal.
- `$sin(x + y)$` — args complexos.
- `$lim(x)$` — limits-style + args (pode complicar).
- `$sin()$` — args vazios.
- `$undef(x)$` — operador desconhecido + args; deve manter
  estrutura `FuncCall` (fallback pre-P301 path).
- `$sin x$` (sem parens) — paridade P301 preservada.

**Regressão crítica**: testes pré-P302 para `$sin x$` (sem
parens) devem continuar a produzir bytes PDF idênticos.

### A.5' — Verificação anti-reflexão (N=12 do padrão §8.6)

**9ª reaplicação A.0.0** consecutiva (P293-P302; P300
retrospectivo inclui-se mas sem A.0.0 magnitude).

4 verificações:

1. **Comparação literal A.1.6 P288-P302** — paradigma novo?
   - P293-P299 nove paradigmas materializadores.
   - P300 retrospectivo (10º).
   - P301 parser/eval modification (11º).
   - **P302**: paradigma **"bug fix derivado de materialização
     anterior"** — reaplica P288 N=1 (NBSP fix derivado P287
     SmartQuote). **12º paradigma se reconhecido como categoria
     genuína**.
2. **A.0.0 produzido empiricamente** — magnitude do diagnóstico
   factual.
3. **Elementos estructuralmente novos identificados**:
   - **Bug latente fixed durante materialização dependente N=2**
     — reaplica P288 §8.4 (P287→P288 NBSP; P301→P302 sin parens).
   - **§A.5 de P301 falhou** — bug `sin(x)` não foi coberto na
     A.5 P301. Lição metodológica: A.5 deve incluir cenários
     cross-construct (operator + args).
   - **Magnitude esperada média** — entre baixa-modesta P301 e
     alta P298. Flutuação saudável continua.
4. **Decisão sobre promoção ADR meta**:
   - **Sub-padrão §8.4 N=2** — longe de limiar N≥3.
   - **§8.7' N=9** — adiamento standard P300 + P301.
   - **§8.3 N=12** candidato — sem aplicação P302.
   - Default: **sem promoção** (anti-padrão P273.17 §0
     honrado pela 10ª vez consecutiva).

---

## §3 — Materialização (condicional a A.0.0)

**Cenário default (HY/HZ + A.2 → (a) + A.3 → β)**:

1. Modificar arm `Expr::FuncCall` em `eval/math.rs`:
   ```rust
   _ => {
       if let Some(op) = lookup_math_op(scopes, &name) {
           // P302: preserva args via MathSequence/MathDelimited.
           let args_content = eval_args_as_math(scopes, args)?;
           Ok(Content::MathSequence(Arc::from(vec![
               op,
               args_content,
           ])))
       } else {
           Ok(Content::MathIdent(name.into()))
       }
   }
   ```
2. Helper `eval_args_as_math` (se não existir): converte args
   FuncCall em `MathDelimited` ou equivalente.
3. Testes:
   - L1 unitário `$sin(x)$` → `MathSequence([MathOp(sin), MathDelimited(x)])`.
   - L1 unitário `$sin(x + y)$` → args complexos preservados.
   - L1 unitário `$sin x$` (sem parens) → continua `MathOp(sin)`
     directo (regressão P301 preservada).
   - L1 unitário `$sin()$` → caso edge documentado.
   - L1 unitário `$undef(x)$` → fallback `MathIdent("undef")` +
     args original preservados.
   - L3 PDF: `$sin(x)$` produz output com `sin x` ou `sin(x)` per
     A.0.0 conclusão (verificar bytes esperados).
   - L3 regression bit-exact: `$sin x$` PDFs idênticos pré-P302.
4. Promoção ADR meta condicional:
   - Default: **sem promoção** (10ª vez consecutiva).
5. Actualizar L0: `rules/eval.md` (se necessário); diagnóstico
   produzido.

**Cenário alternativo (HX confirmada)**:

1. P301 actual é correcto; P302 apenas adiciona testes
   explícitos validando comportamento.
2. Magnitude trivial; documentar honestamente.

**Sem caps** (per P282 §7). Estimativa de testes: ~5-10.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P301: 2 870 testes.
  Esperado: ~2 875-2 880.
- `crystalline-lint` zero violations.
- Hash L0 `content.md` **preserved** (sem variants novos).
- Hash L0 `stdlib.md` **preserved**.
- Hash L0 `eval/math.rs` **muda intencionalmente** (modificação
  `FuncCall` arm).
- Hash L0 `export.rs` **preserved bit-exact** pelo **19º passo
  consecutivo** P282-P302.
- **Regressão bit-exact validada** — testes `$sin x$` (sem
  parens) produzem bytes idênticos.
- Bug factual resolvido — `$sin(x)$` produz output esperado per
  A.0.0 conclusão.
- Diagnóstico A.0.0+A.0-A.5+A.5' produzido.
- **Sem promoção ADR meta** — default; anti-padrão honrado 10ª
  vez consecutiva.
- Sub-padrão §8.4 "bug latente fixed durante materialização
  dependente" N=2 registado.

---

## §5 — Não-objectivos

- **Não** materializar `MathDelimited` variant novo se já existe.
  Reuso de variants pré-P302.
- **Não** estender `lookup_math_op` para outros casos (e.g. shadowing
  semantics adicionais). Apenas resolver bug factual `sin(x)`.
- **Não** modificar heurística P298 — preservada.
- **Não** alterar comportamento `$sin x$` (sem parens) — P301
  paridade preservada.
- **Não** promover ADR meta. §8.4 N=2 longe de limiar; §8.7' N=9
  segue standard P300 + P301.
- **Não** confundir P302 com P301 — P301 introduziu auto-lookup;
  P302 corrige bug latente derivado.
- **Não** assumir comportamento vanilla sem A.0.0 — bug factual
  pode revelar HX trivial.

---

## §6 — Pendências relacionadas

Resolve:
- Bug latente `sin(x)` parens — P301 §9 frente pendente.

Não resolve:
- P295.1 nota rodapé.
- P296.X toggles cancel.
- P297.X UnderoverKind.
- Cosméticos ADR-0054 graded.
- Auto-lookup math mode (resolvido P301).
- Show rules sobre math identifiers — bloqueado regex L1.

---

## §7 — Risco residual

Risco principal: **A.0.0 → HX (vanilla descarta parens)** —
significa que P301 actual é correcto e não há bug. Mitigação:
A.0.0 inspecção empírica vanilla literal antes de qualquer
materialização; se HX confirmada, P302 reduz-se a testes.

Risco secundário: **regressão `$sin x$` (sem parens)**. Mitigação:
§4 testes regression bit-exact obrigatórios.

Risco terciário: **`$undef(x)$` regression**. Função user
desconhecida deve preservar args via path FuncCall original.
Mitigação: A.5 cenário dedicado; preservar caminho `Ok(MathIdent)`
original.

Risco quaternário: **`limits-style` operadores com args** —
`$lim(x)_(x→0) f$` é caso complexo. Pode haver interacção com
P298 cross-variant. Mitigação: A.5 cenário; documentar
comportamento honesto.

Risco quinário: **A.5 P301 falhou cobertura cross-construct** —
P302 expõe falha metodológica P301. Lição: A.5 deve incluir
cenários que combinam features novas com sintaxe pré-existente.
Mitigação: registar em A.5' como elemento estructuralmente
novo + lição metodológica.

Risco senário: **magnitude esperada XS+ pode revelar-se média
ou superior** se HZ requer alterações estruturais. Mitigação:
permitir interrupção honesta + abertura P302.0 sub-passo se
necessário.

---

## §8 — Ponteiros

- Sítio bug: `01_core/src/engine/eval/math.rs:266` (linha exacta
  per P301 §3.3 relatório — FuncCall arm).
- Helper P301: `lookup_math_op` em mesmo ficheiro.
- Variants relacionados: `MathSequence`, `MathDelimited`,
  `MathOp`.
- Vanilla: `lab/typst-original/crates/typst-eval/src/math.rs`
  (FuncCall em math mode).
- Precedente §8.4 N=1: **P288** (NBSP fix derivado P287
  SmartQuote materialização).
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções).
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- Padrão §8.7' A.0.0 template (N=9 reaplicação).
- Sub-padrão §8.4 "bug latente fixed" — **N=2 cumulativo
  candidato** se P302 fix genuíno.

---

*Spec P302 produzida 2026-05-19 pós-P301 (auto-lookup math mode
materializado; bug latente `sin(x)` parens registado §9). Frente
**bug fix derivado P301** — reaplica sub-padrão §8.4 P288 (N=1
NBSP→N=2 sin parens). Magnitude XS+ esperada; A.0.0 verifica
empiricamente comportamento vanilla. Fase A com **7 secções**
A.0.0+A.0-A.5+A.5'. **4 hipóteses HX/HY/HZ/HW** sobre
comportamento vanilla — A.0.0 decide. **A.5 P301 falhou
cobertura cross-construct** — lição metodológica registada em
A.5'. **Hash `export.rs` preservado** pelo 19º passo consecutivo
P282-P302 (P302 confinado a `eval/math.rs`). **Sem promoção ADR
meta** — default; anti-padrão P273.17 §0 honrado pela 10ª vez
consecutiva. **Honestidade epistémica**: spec aceita que HX
trivializaria P302 (sem bug), mas A.0.0 obrigatória verifica
empiricamente antes de assumir. Sem caps LOC ou magnitude
(P282 §7).*
