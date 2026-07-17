# Passo 301 — Auto-lookup math mode

**Frente**: `Auto-lookup math mode` (frente pendente P299 §8 +
P300 §8 prioridade #2).
**Origem**: P299 materializou `math.sin`/`math.lim`/etc. (42
operadores) acessíveis via prefixo `math.`. **Mas** em math mode
(`$sin x$`), cristalino continua a tratar `sin` como `MathIdent`
literal — sem lookup automático ao scope `math` materializado em
P299.
**Pré-requisitos**: P299 (`make_math_module` + 42 operadores
registados).
**Tipo declarado**: **1º passo a modificar parser/eval math mode**
na série P283+ — paradigma genuinamente novo. Cluster math
P296-P299 tocou variants + layout + stdlib; **P301 toca
parser/eval**.
**Marco**: 1º retorno a passo materializador pós-P300 retrospectivo.

---

## §1 — Objectivo

Implementar auto-lookup no parser/eval math mode: quando
identifier `sin`/`lim`/etc. aparece em `$...$` (ou `$ ... $`
display), consultar scope `math` (materializado P299) e
substituir por `Content::MathOp` automaticamente — paridade
vanilla typst onde `$sin x$` é equivalente a `$math.sin x$`.

### §1.1 — Estado factual pré-P301

**Cristalino actual** (pós-P299):
- **`math.sin`/`math.lim`/etc.** funcionam via prefixo explícito
  (P299 `make_math_module`).
- **`$sin x$`** continua a renderizar `sin` como `MathIdent("sin")`
  literal — paradigma pré-P298.
- **Heurística `is_limit_function`/`is_large_operator`** (P298
  §3.4) detecta `MathIdent("lim")` em attach context — fallback
  parcial. Cobre 7 limits + large operators (∑/∫/etc.).
- **Scripts-style operadores** (`sin`/`cos`/etc.) **funcionam por
  acidente** — `MathIdent("sin")` renderiza como texto math literal
  que **visualmente** parece operador; mas semanticamente é
  apenas identifier.

**Vanilla typst**:
- `$sin x$` em math mode resolve `sin` via lookup automático no
  scope `math`.
- Resultado: `sin` é `MathOp { text, limits: false }` —
  semanticamente operador, não identifier.

**Gap material**: P299 materializou os valores; P301 materializa
o **mecanismo de resolução**.

### §1.2 — Riscos arquitecturais identificados

| Risco | Descrição |
|---|---|
| **Conflito com heurística P298** | `is_limit_function` cobre `"lim"/"max"/.../liminf` (7); P301 cobre 42; sobreposição parcial |
| **`mod` collision** | `math.mod` (P299) vs operador `mod` aritmético vanilla |
| **Regressão `MathIdent` user** | Identifiers que não são operadores (variáveis: `x`, `y`, `t`, etc.) devem continuar `MathIdent` |
| **Hash `parser`/`eval`** | 1ª spec a modificar parser/eval na série P283+ — hashes mudarão |
| **Hash `export.rs`** | Esperado **preservado** (17º passo se P301 → §1.2 caso (i)) — auto-lookup é resolução, não emit |

### §1.3 — Razões

1. **Frente pendente registada** P299 §8 + P300 §8 prioridade #2.
2. **Completar integração P299** — sem P301, `math.sin` está
   disponível mas user typst tem que prefixar explicitamente
   (divergência vanilla).
3. **1º paradigma parser/eval** na série P283+ — paradigma
   genuinamente novo.
4. **Reaplicação ADR-0098 + ADR-0099** — condicional ao impacto
   em export.rs (esperado preservado pelo 18º passo consecutivo;
   verificar A.4).
5. **A.0.0 N=8 cumulativa** — 8ª reaplicação consecutiva
   (P293-P301; P300 retrospectivo incluído).
6. **Possível disparo §8.7' N=8** — se A.0.0 magnitude alta após
   P300 caminho A conservador (não promoveu), P301 alta robusta
   poderia reabrir avaliação.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 7 secções)

### A.0.0 — Verificação literal estado parser/eval math mode (N=8 cumulativo §8.7')

Inspecção literal:

1. **`grep -rn "math\|eval_math\|MathContext" 01_core/src/engine/eval/`** —
   identificar pontos onde math mode é detectado/processado.
2. **`grep -rn "MathIdent" 01_core/src/engine/eval/`** — onde
   `MathIdent` é construído (parser ou eval?).
3. **`grep -rn "scope.*get\|scope.*lookup" 01_core/src/engine/eval/`** —
   mecanismo lookup existente.
4. **Inspeccionar `01_core/src/engine/eval/markup.rs`** (ou
   caminho equivalente) — `eval_markup` vs `eval_math`.
5. **Inspeccionar `lab/typst-original/.../math/mod.rs`** — como
   vanilla typst faz auto-lookup math mode.
6. **Inspeccionar `01_core/src/engine/math/symbols.rs`** —
   `is_limit_function`/`is_large_operator` (P298 §3.4).
7. **Verificar `make_math_module` P299** — como é registado em
   `eval/mod.rs:768` (`scope.define("math", ...)`).
8. **Cross-check com `make_calc_module` (P283)** — calc tem
   auto-lookup similar? `$calc.sin(x)$` vs `#calc.sin(x)` —
   verificar paridade.

**Decisão A.0.0**:

| Hipótese | Acção |
|---|---|
| **HO** (cristalino sem auto-lookup) | P301 materializa parser/eval hook |
| **HP** (heurística cobre parcialmente) | P301 estende heurística para 42 operadores via lookup `math` |
| **HQ** (vanilla usa scope lookup parser-side) | P301 implementa paralelo paridade |
| **HR** (scope existe mas math mode bypass) | P301 conecta o bypass |
| **HS** (mix) | Subdivisão |

**Magnitude esperada A.0.0**: **média-alta**. Cristalino tem
infraestrutura parcial (P298 heurística + P299 `math` module);
gap é a **conexão** entre eles em math mode.

### A.0.0' — Decisão de scope concreto (paralelo P299 A.0.0')

P301 tem múltiplas dimensões; subdivisão possível:

| Subset | Scope | Magnitude |
|---|---|---|
| **P301.A** | Auto-lookup eval-time apenas (sem alterar parser) | M |
| **P301.B** | Auto-lookup parser-time (transforma AST) | M+ |
| **P301.C** | A+B combinado | M+ |
| **P301.D** | Mínimo: lookup apenas para os 42 P299 (sem fallback global) | S-M |
| **P301.E** | Subdivisão temporal: P301 = lookup eval; P301.1 = parser optimisation | S+S |

**Default sugerido**: **P301.A** ou **P301.D** se A.0.0 confirma
HP/HR (parcial cobertura). **P301.B/C** se HO (from-scratch).

Permitir interrupção honesta se A.0.0 + A.0.0' revelam
complexidade superior ao M-M+ esperado.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado |
|---|---|
| `grep "math_lookup\|MathIdent.*replace" 03_infra/src/export.rs` | **Zero hits** — emit consome `MathOp` agnóstico via P298 paradigm |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo 18º passo consecutivo se P301 → eval-time substitution |
| Hash `eval/mod.rs` esperado | **Muda intencionalmente** — auto-lookup é mecanismo eval; primeira modificação parser/eval na série |
| Hash `content.rs` esperado | **Preservado** — sem variants novos |

**A.0 não-trivial em sentido novo**: P301 muda parser/eval hashes
(primeira na série P283+ a fazê-lo), mas preserva `export.rs`.
ADR-0098 §"alterações justificadas" cobre — preservação do hash
é em **export**, não no resto.

### A.1 — Inventário literal parser/eval math

8 sub-secções:

1. **A.1.1 — Como math mode é detectado** — parser detecta `$...$`
   e troca eval mode?
2. **A.1.2 — `MathIdent` construção** — onde é criado: parser ou
   eval?
3. **A.1.3 — `scope` accesível em math mode** — função `eval_math`
   recebe scope completo?
4. **A.1.4 — `make_math_module` P299** — registo em scope global
   vs scope math context.
5. **A.1.5 — Vanilla `eval_math`** — algoritmo de lookup vanilla.
6. **A.1.6 — Heurística `is_limit_function` P298** — onde é
   invocada; pode ser substituída ou complementada?
7. **A.1.7 — `MathOp` consumer em emit** — confirmar P298 paradigm
   inalterado.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente.

### A.2 — Decisão arquitectural (onde inserir lookup)

| Opção | Mecanismo | Prós | Contras |
|---|---|---|---|
| **(a)** **Eval-time**: `eval_math` consulta scope `math` quando encontra `MathIdent("sin")` | Sem alteração parser; reuso scope existente | Lookup em runtime cada vez (não optimizado) |
| **(b)** **Parser-time**: AST transformação `Ident("sin")` → `MathOp(...)` durante construção | Optimizado; sem runtime lookup | Acoplamento parser ↔ scope; AST muda |
| **(c)** **Layout-time**: `layout_math` arm consulta scope antes de processar `MathIdent` | Reuso paradigm P296-P298 layout-time | Conceptualmente errado — lookup é resolução semântica, não layout |
| **(d)** **Heurística estendida**: amplia `is_limit_function` para 42 operadores hardcoded | Sem dependência scope | Duplica P299 trabalho; viola SSoT |

Default sugerido: **(a)** — eval-time lookup minimiza
acoplamento; reusa scope existente; preserva SSoT P299.
**(b)** se A.1.5 revelar vanilla parser-time. **(c)** rejeitada
— conceptualmente errado. **(d)** rejeitada — viola SSoT.

**Implicação gatilho**: nova ADR meta candidata se A.2 → (a)
introduz pattern "scope lookup em eval" como pattern arquitectural
distinto. Verificar A.5'.

### A.3 — Integração com heurística pré-P301

`is_limit_function`/`is_large_operator` (P298 §3.4) cobre 7 +
operadores Unicode. P301 cobre 42 operadores. **Sobreposição
parcial**:

| Opção | Comportamento |
|---|---|
| **(α)** P301 substitui heurística completamente — todos os operadores via lookup `math` | SSoT pura; remove código P298 |
| **(β)** P301 estende heurística — `is_limit_function` continua para fallback; lookup `math` cobre mais | Compatibilidade; redundância parcial |
| **(γ)** P301 lookup precede heurística — se `sin`/`lim` em `math` scope, usar; senão fallback heurístico | Híbrido SSoT-preferred |

Default sugerido: **(γ)** — preserva fallback de P298 sem duplicar.
**(α)** se A.5 testes confirmam paridade total bit-exact.

### A.4 — Impacto em emit (ADR-0098)

| Opção | Mecanismo | Implicação `export.rs` |
|---|---|---|
| **(i)** Auto-lookup eval-time produz `MathOp` consumido por P298 paradigm | **Preservado bit-exact** — 18º passo |
| **(ii)** Caso edge | Improvável |

Default sugerido: **(i)** quase certo se A.2 → (a)/(b).

### A.5 — Detecção de bugs latentes

Cenários fronteira **CRÍTICOS**:

- **`$sin x$`**: deve renderizar idêntico a `$math.sin x$` (paridade).
- **`$sin x$` vs `$x sin$`**: posicionamento.
- **`$lim_(x→0) f(x)$`**: limits-style block mode (via P298
  cross-variant). **Regressão crítica**: heurística pré-P301 vs
  novo lookup — comportamento bit-exact?
- **`$x$` (variável)**: continua `MathIdent` (lookup não encontra
  no scope `math`).
- **`$mod$` ambíguo**: `math.mod` (P299) operador vs `mod` aritmético?
- **`$sin(x)$`**: sin com parênteses — paridade?
- **Identifier user redefinido**: `#let sin = "custom"; $sin x$`
  — auto-lookup deve respeitar shadowing? Vanilla?
- **Regressão `MathIdent` literal**: PDFs pré-P301 com `MathIdent("sin")`
  via heurística fallback **devem produzir bytes idênticos** se A.3
  → (γ) (fallback preservado).

### A.5' — Verificação anti-reflexão (N=11 do padrão §8.6)

**8ª reaplicação A.0.0** consecutiva (P293-P301; P300 incluído).

4 verificações:

1. **Comparação literal A.1.6 P288-P301** — paradigma novo?
   - P293-P299 nove paradigmas distintos materializadores.
   - P300 retrospectivo: paradigma "consolidação sem
     materialização" (10º).
   - **P301**: paradigma **"parser/eval modification"** —
     primeira modificação parser/eval na série P283+. **11º
     paradigma distinto**.
2. **A.0 produzido empiricamente** — A.0.0 P301 magnitude.
3. **Elementos estructuralmente novos identificados**:
   - **1ª modificação parser/eval** — primeiro hash não-content
     non-export-rs a mudar na série.
   - **Pattern "scope lookup em eval"** — sub-padrão N=1 inaugural
     se A.2 → (a).
   - **Conflito potencial heurística vs SSoT** — A.3 decisão
     estrutural genuína.
4. **Decisão sobre promoção ADR meta**:
   - **§8.7' N=8** se A.0.0 magnitude alta após P300 conservador
     — reabertura legítima.
   - **§8.3 N=12** candidato adiado.
   - **Sub-padrão "scope lookup em eval" N=1** — inaugural.
   - **Sub-padrão "operadores SSoT" N=2** (P299 + P301) — se
     A.2 → (a) confirma SSoT cumulativa.
   - **Uma ADR meta por passo no máximo** (P273.17 §0).
   - **Critério P300 mantido**: promover apenas se inequívoco;
     P300 caminho A conservador estabeleceu standard.

---

## §3 — Materialização (condicional a A.0.0 + A.0.0')

**Cenário default (HP/HR + P301.A + A.2 → (a) eval-time + A.3 → γ híbrido)**:

1. Localizar `eval_math` (ou equivalente) onde `MathIdent` é
   produzido/processado.
2. Adicionar lookup pré-construção:
   ```rust
   // P301 auto-lookup: consulta scope `math` antes de construir MathIdent.
   if let Some(Value::Dict(math_module)) = scope.get("math") {
       if let Some(Value::Content(content)) = math_module.get(&name) {
           if matches!(content, Content::MathOp { .. }) {
               return Ok(content.clone());
           }
       }
   }
   // Fallback: MathIdent literal (caminho pré-P301 preservado).
   Ok(Content::MathIdent(name.into()))
   ```
3. **Heurística `is_limit_function` preservada** em `attach.rs` —
   continua a aplicar para casos não cobertos por `math` scope
   (e.g. operadores Unicode `∑`/`∫`).
4. Testes:
   - L1 unitário: `eval_math("sin")` → `Content::MathOp`, não
     `MathIdent`.
   - L1 unitário: `eval_math("x")` → `MathIdent` (não em scope).
   - L1 unitário: shadowing — `#let sin = ...; $sin$` —
     comportamento documentado.
   - L1 unitário: `mod` ambíguo — registar paridade vanilla.
   - L3 regression bit-exact: PDFs pré-P301 com `$lim_(x→0)$` via
     heurística — **bytes idênticos**.
   - L3 integração: PDFs `$sin x$` produzem output equivalente
     a `$math.sin x$`.
5. Promoção ADR meta condicional per A.5':
   - **Default**: sem promoção (anti-padrão P273.17 §0).
   - **§8.7' N=8** se A.0.0 magnitude alta inequívoca.
   - **Sub-padrão "scope lookup em eval"** registado inaugural
     se A.2 → (a).
6. Actualizar L0:
   - `rules/eval.md` (se existe) — actualizar hash.
   - Tabela A.4 — nota cruzada P301 + paridade auto-lookup.
   - Propagar hashes via `crystalline-lint --fix-hashes`.
7. Diagnóstico produzido:
   `diagnostico-auto-lookup-math-passo-301.md` com 7 secções
   A.0.0+A.0.0'+A.0-A.5+A.5'.

**Sem caps** (per P282 §7). Estimativa de testes: ~10-20.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P300: 2 862 testes.
  Esperado: ~2 872-2 882.
- `crystalline-lint` zero violations.
- Hash L0 `content.md` **preserved** (sem variants novos).
- Hash L0 `stdlib.md` **preserved** (política única).
- Hash L0 `rules/eval.md` (ou equivalente) **muda intencionalmente**
  — 1ª modificação parser/eval na série P283+; alteração
  justificada (não regressão).
- Hash L0 `export.rs` **preserved bit-exact** pelo **18º passo
  consecutivo** P282-P301 (se A.4 → (i)).
- **Regressão bit-exact validada** para PDFs pré-P301 com `MathIdent`
  via heurística fallback.
- Tabela A.4 nota cruzada P301.
- Diagnóstico produzido.
- **Promoção ADR meta condicional**:
  - Default: sem promoção (anti-padrão P273.17 §0 honrado pela
    9ª vez consecutiva).
  - §8.7' N=8 condicional a magnitude alta inequívoca.
- Frente "Auto-lookup math mode" resolvida (P300 §8 prioridade #2
  fechada).

---

## §5 — Não-objectivos

- **Não** alterar `make_math_module` P299. Permanece SSoT dos 42
  operadores.
- **Não** remover heurística `is_limit_function`/`is_large_operator`
  P298. Continua fallback para casos não cobertos por scope
  (operadores Unicode literais).
- **Não** materializar shadowing semantics adicionais. Comportamento
  paridade vanilla preservado.
- **Não** tocar parser AST se A.2 → (a). Modificação confinada a
  eval.
- **Não** estender scope module para operadores não-vanilla. Lista
  P299 é canónica.
- **Não** promover múltiplas ADRs meta. Uma por passo (P273.17
  §0). Critério P300 mantido — promover só se inequívoco.
- **Não** confundir P301 com cluster math. P301 é
  **parser/eval**, não variant/layout.

---

## §6 — Pendências relacionadas

Resolve:
- Frente "Auto-lookup math mode" — P299 §8 + P300 §8 prioridade #2.
- Integração natural P299 — `math.sin` agora também acessível
  como `sin` em math mode.

Não resolve:
- P295.1 nota rodapé.
- P296.X toggles cancel.
- P297.X UnderoverKind.
- Cosméticos ADR-0054 graded.
- Show rules sobre math identifiers — bloqueado por regex em L1.

---

## §7 — Risco residual

Risco principal: **regressão bit-exact em features math
pre-P301**. Heurística pré-P301 produzia `MathIdent("sin")` que
renderizava como texto literal. Pós-P301, mesmo input produz
`MathOp { text: "sin", limits: false }` — renderização **pode
diferir** se P296-P298 handler tratam `MathOp` diferentemente de
`MathIdent`. Mitigação: §4 testes regression obrigatórios; se
divergência bytes, A.3 → (γ) fallback preserva pre-P301 paths;
se divergência **necessária** para paridade vanilla, registar
honestamente como alteração justificada.

Risco secundário: **`mod` collision**. `math.mod` em P299 é
operador; `mod` aritmético vanilla é binary operator. Mitigação:
A.5 cenário dedicado; documentar paridade vanilla; se ambíguo,
adiar `mod` específico para sub-passo P301.0.

Risco terciário: **shadowing semantics**. `#let sin = "x"; $sin$`
— auto-lookup deve respeitar shadowing? Vanilla? Mitigação: A.5
cenário; paridade vanilla; documentar honestamente.

Risco quaternário: **A.2 → (b) parser-time** se A.1.5 mostra
vanilla parser-time. Modificação parser tem maior risco de
regressão geral; abrir P301.0 dedicado se necessário.

Risco quinário: **§8.7' N=8 promoção forçada** porque P300 caminho
A "deixou pendente". Mitigação: §A.5' critério estrito — P300 não
adiou por dúvida sobre §8.7'; adiou por preferência conservadora.
P301 só promove se evidência **adicional** dispara inequivocamente.

Risco senário: **performance lookup runtime** se A.2 → (a) faz
N lookups por equation. Mitigação: scope lookup é O(1) hash;
preocupação prematura. Profiling se necessário.

Risco septenário: **modificação parser/eval em isolamento**.
Cluster math P296-P298 estabeleceu paradigma "variant + layout";
P301 não usa este paradigma. **Não é desvio** — é frente
arquitecturalmente distinta legítima. Registar em A.5'.

---

## §8 — Ponteiros

- Eval math: `01_core/src/engine/eval/markup.rs` ou
  `01_core/src/engine/eval/math.rs` — verificar A.1.1.
- Scope `math` P299: `01_core/src/engine/stdlib/structural.rs`
  (`make_math_module`).
- Registo P299: `01_core/src/engine/eval/mod.rs:768`.
- Heurística P298: `01_core/src/engine/math/symbols.rs`
  (`is_limit_function`/`is_large_operator`).
- Vanilla: `lab/typst-original/crates/typst-eval/.../math.rs`
  (math mode eval).
- Precedente directo: **P298** (heurística) + **P299** (scope module).
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções).
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- ADR scope: ADR-0054 graded.
- Padrão §8.7' A.0.0 template (N=8 reaplicação; P300 conservador).
- Padrão §8.3 refutação pragmática (N=12 candidato adiado).
- Sub-padrão "scope lookup em eval" — N=1 inaugural candidato.
- Sub-padrão "operadores SSoT" — N=2 cumulativo (P299+P301) se
  P301 reusa scope.

---

*Spec P301 produzida 2026-05-19 pós-P300 (retrospectivo caminho
A; 0 promoções; anti-padrão honrado 8 passos). Frente
`Auto-lookup math mode` — **1º paradigma parser/eval na série
P283+**. Cluster math P296-P298 tocou variants + layout + stdlib;
P301 toca **parser/eval**. Magnitude M esperada; A.0.0 + A.0.0'
decidem subset concreto. Fase A com **7 secções**
A.0.0+A.0.0'+A.0-A.5+A.5'. **A.0 não-trivial em sentido novo**:
hash `export.rs` esperado **preservado** (18º passo consecutivo)
mas hash `eval` **muda intencionalmente** (1ª vez na série P283+).
**§8.7' N=8 candidato condicional** — P300 caminho A conservador
adiou; P301 reabre **apenas se** evidência adicional inequívoca.
**Heurística P298 preservada** como fallback (A.3 → γ); SSoT P299
preferida. Honestidade epistémica: spec **aceita não saber** se
P300 conservador foi caminho certo; P301 testa empiricamente se
§8.7' continua adiável após nova reaplicação. Sem caps LOC ou
magnitude (P282 §7).*
