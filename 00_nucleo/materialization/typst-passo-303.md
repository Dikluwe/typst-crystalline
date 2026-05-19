# Passo 303 — Bug fix `$undef(x)$` args fallback

**Frente**: Bug latente `$undef(x)$` args descartados em
fallback `MathIdent` — descoberto colateral em P302 §7
inspecção.
**Origem**: P302 §3.1 corrigiu apenas o caminho lookup-hit
(`MathSequence([MathOp, MathDelimited])`); deixou caminho
lookup-miss (`MathIdent` fallback) com **bug pré-existente
pré-P301**. P302 §7 + §8 + §9 registaram como frente XS+
pendente.
**Pré-requisitos**: P302 (`MathSequence + MathDelimited` pattern
para preservação args).
**Tipo declarado**: **bug latente irmão de P302** — subcategoria
ambígua do §8.4 (A: bug derivado materialização vs B: bug
pré-existente descoberto durante inspecção). A.0.0 clarifica
subcategoria.
**Magnitude**: XS+ esperada; reuso composição P302 directa.

---

## §1 — Objectivo

Estender fallback `MathIdent` no arm `Expr::FuncCall` para
preservar args via `MathSequence([MathIdent, MathDelimited])` —
paralelo arquitectural directo da correcção P302 mas no caminho
lookup-miss (operador desconhecido em scope `math`).

### §1.1 — Bug factual exposto

**Pré-P303** (incluindo pós-P302):
- `$undef(x)$` → `FuncCall("undef", [x])` → `lookup_math_op`
  retorna `None` → fallback `Ok(Content::MathIdent("undef"))`.
- **Args `(x)` descartados silenciosamente** — bug pré-P301.

**Visibilidade**: bug existia desde sempre (antes P301), mas só
ficou **visível como bug** depois de P302 fixar o caso adjacente
(lookup-hit). Antes de P302, ambos os caminhos descartavam args
de forma simétrica; agora P302 fixou um lado e o outro ficou
claramente assimétrico.

### §1.2 — Subcategoria §8.4: ambígua

P302 §10.1 registou §8.4 como sub-padrão "bug latente fixed
durante materialização dependente" N=2. **P303 é categoria
diferente ou mesma?**

| Subcategoria | Descrição | Exemplo |
|---|---|---|
| **A** | Bug **derivado** de materialização anterior (passo X cria/expõe via X+1) | P288 (NBSP de P287); P302 (sin parens de P301) |
| **B** | Bug **pré-existente** descoberto durante inspecção | Candidato `undef(x)` |
| **C** | Bug em feature recém-materializada descoberto em A.5 do próprio passo | (sem exemplos cumulativos ainda) |

**Crítica honesta**: `undef(x)` é **bug pré-P301** mas só ficou
**visível como bug** porque P302 fixou o caminho adjacente.
Ambiguidade:
- **Argumento A**: bug derivado da materialização P302 (sem P302,
  ambos descartavam args; assimetria pós-P302 expõe).
- **Argumento B**: bug pré-existente independente (existia antes
  P301 + P302; só agora visível).

**Decisão A.0.0**: documentar empiricamente; **não inflar N=3 do
§8.4** se subcategoria é genuinamente diferente. Honesto: §8.4
preserva N=2 (P288 + P302); P303 inaugura subcategoria distinta
ou conta-se sob mesma categoria conforme A.0.0 conclusão.

### §1.3 — Comportamento esperado vanilla

A verificar empiricamente em A.0.0:

| Hipótese | Comportamento vanilla `$undef(x)$` |
|---|---|
| **HX'** | Renderiza `undef(x)` literal — args preservados como `MathSequence([MathIdent, MathDelimited])` |
| **HY'** | Renderiza `undef x` — parens descartados (paralelo HX do P302) |
| **HZ'** | Function call error — vanilla rejeita identifier desconhecido como function |
| **HW'** | Outro comportamento empírico |

**Default esperado**: HX' (paridade com fix P302) — args
preservados via composição `MathSequence + MathDelimited`.

### §1.4 — Riscos e razões

**Riscos**:
- Subcategoria §8.4 ambígua pode inflacionar N para promoção
  forçada. Mitigação: §A.5' decide honestamente.
- Comportamento vanilla pode ser HZ' (rejeição) — cristalino
  divergir; documentar.

**Razões**:
1. **Simetria arquitectural** — P302 fixou lookup-hit; P303
   fixa lookup-miss. Caminhos gémeos.
2. **Reuso composicional total** — `MathSequence + MathDelimited`
   já estabelecido P302.
3. **Magnitude controlada XS+** — paralelo P302.
4. **Reaplicação ADR-0098** — N=20 cumulativo condicional.
5. **§8.4 subcategoria** — clarificação metodológica.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 7 secções)

### A.0.0 — Verificação literal comportamento `$<undef>(<args>)$` (N=10 cumulativo §8.7')

Inspecção literal:

1. **Reproduzir bug actual pós-P302**:
   ```rust
   let result = eval_math("$undef(x)$");
   // Esperado pós-P302: MathIdent("undef") apenas; args descartados
   ```
2. **Verificar vanilla**: `lab/typst-original/...` — como vanilla
   processa identifier desconhecido em math mode + parens.
3. **Sítio bug exacto**: `01_core/src/rules/eval/math.rs` arm
   `Expr::FuncCall` ramo `else` (last branch pós-P302).
4. **Cross-check com `MathIdent` arm pós-P301**:
   - `$undef$` (sem parens) → continua `MathIdent("undef")` ✓.
   - `$undef(x)$` (com parens) → **bug a fixar**.
5. **Subcategoria §8.4** — empiricamente verificar:
   - Bug existia pré-P301? **Sim** (parens sempre descartados
     em `MathIdent` fallback).
   - P301 introduziu? **Não** (P301 só tocou lookup-hit).
   - P302 expôs? **Indirectamente** (fixou irmão; assimetria
     evidente).

**Decisão A.0.0 sobre subcategoria**:

| Conclusão | Acção |
|---|---|
| **Subcategoria A** (bug derivado P302) | §8.4 N=3 candidato cumulativo |
| **Subcategoria B** (bug pré-existente independente) | §8.4 N=2 preservado; P303 inaugura subcategoria distinta |
| **Híbrido** | Documentar honestamente; preferir conservador (B) |

**Default sugerido**: **B** — bug pré-existente; P303 é
inauguração de subcategoria distinta dentro de §8.4 (ou padrão
novo). Conservador per P273.17 §0.

**Magnitude esperada A.0.0**: **baixa-modesta** — bug factual
trivial; verificação vanilla rotineira. Confirmação esperada
P295 §10 reaplicada genuinamente.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado |
|---|---|
| `grep "undef\|MathIdent.*FuncCall" 03_infra/src/export.rs` | Zero hits — emit consume FrameItem agnóstico |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo **20º passo consecutivo** P282-P303 |
| Hash `content.rs` | **Preservado** — sem variants novos |
| Hash `eval/math.rs` (L0) | **Inalterado** — extensão dentro contrato P301/P302 |

### A.1 — Inventário literal

8 sub-secções:

1. **A.1.1 — Arm `Expr::FuncCall` pós-P302** — linhas exactas do
   ramo `else`.
2. **A.1.2 — `MathIdent` variant** — estrutura.
3. **A.1.3 — `MathSequence + MathDelimited` composição P302** —
   reuso directo.
4. **A.1.4 — Vanilla `undef(x)` comportamento** — confirmar HX'.
5. **A.1.5 — Path `MathIdent` sem parens** — `$undef$` continua
   válido sem args (preservar).
6. **A.1.6 — Testes existentes** — `p302_undef_parens_continua_mathident_fallback`
   documenta comportamento actual; P303 vai invalidá-lo
   (esperado).
7. **A.1.7 — Outros identifiers desconhecidos** — `f(x)`, `g(x, y)`,
   user-defined names — mesmo bug.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente paralelo
   P302 §A.1.8.

### A.2 — Estrutura da correcção

**Decisão directa** — paralelo P302 §3.1 ramo `_` mas no `else`
do mesmo arm:

| Opção | Mecanismo |
|---|---|
| **(a)** Reuso integral do helper P302 — extrair body args + emitir `MathSequence([MathIdent, MathDelimited])` | Composicional; sem novos variants |
| **(b)** Helper novo `wrap_with_args` factorizando lookup-hit + lookup-miss | Refactor; reduce duplicação |
| **(c)** Manter bug pré-existente; só adicionar testes documentando | Subcategoria B se vanilla aceita HX' divergente |

Default sugerido: **(a)** se HX' confirmada — fix directo.
**(b)** se A.0.0 revelar duplicação significativa no código
P302. **(c)** se A.0.0 revela vanilla é HZ' (rejeição) e
cristalino prefere comportamento divergente actual.

### A.3 — Integração com path `MathIdent` sem parens

Verificação obrigatória: `$undef$` (sem parens) continua a
produzir `MathIdent("undef")` **sem** wrapper `MathSequence`.

P302 já estabeleceu paralelo:
- `$sin$` → `MathOp(sin)` (sem wrapper).
- `$sin()$` → `MathOp(sin)` (args vazios = sem wrapper).
- `$sin(x)$` → `MathSequence([MathOp, MathDelimited])` (args
  presentes).

P303 paralelo:
- `$undef$` → `MathIdent("undef")` (sem wrapper) ✓ preservado.
- `$undef()$` → `MathIdent("undef")` (args vazios = sem wrapper)?
  Decisão A.0.0.
- `$undef(x)$` → `MathSequence([MathIdent, MathDelimited])`
  (args presentes) — fix.

### A.4 — Impacto em emit (ADR-0098)

Emit agnóstico via `FrameItem`. Hash `export.rs` preservado bit-
exact pelo **20º passo consecutivo**.

### A.5 — Detecção de bugs latentes adicionais + matrix cross-construct

P302 §6.7 registou lição: A.5 deve incluir matrix feature ×
sintaxe. **P303 aplica primeira vez explicitamente**:

| | Sintaxe A (`$f$`) | Sintaxe B (`$f x$`) | Sintaxe C (`$f(x)$`) | Sintaxe D (`$f()$`) |
|---|---|---|---|---|
| **Operador conhecido (`sin`)** | ✓ MathOp P301 | ✓ MathOp P301 | ✓ MathSequence P302 | ✓ MathOp P302 |
| **Identifier desconhecido (`undef`)** | ✓ MathIdent pre-P301 | ✓ MathIdent pre-P301 | ✗ bug (P303 fix) | ? A.0.0 |
| **Operador com limits (`lim`)** | ✓ MathOp P301 | ✓ MathOp P301 | ✓ MathSequence P302 | ✓ MathOp P302 |

**Cenários fronteira específicos P303**:
- `$undef(x)$` — bug principal.
- `$undef(x + y)$` — args complexos.
- `$undef(x, y)$` — múltiplos args.
- `$undef()$` — args vazios; decisão A.0.0.
- `$f(x)$` (function call genuíno user) — comportamento
  expectável? Vanilla?
- `$x(y)$` (variável "called") — paridade vanilla?
- **Regressão `$undef$`** — sem parens; bytes idênticos pré-P303.
- **Regressão `$sin(x)$`** — operador conhecido; P302 fix
  preservado bit-exact.

### A.5' — Verificação anti-reflexão (N=13 do padrão §8.6)

**10ª reaplicação A.0.0** consecutiva.

4 verificações:

1. **Comparação literal A.1.6 P288-P303** — paradigma novo?
   - P293-P299 nove paradigmas materializadores.
   - P300 retrospectivo (10º).
   - P301 parser/eval (11º).
   - P302 bug latente fixed (12º).
   - **P303**: paralelo arquitectural directo P302 — paradigma
     **idêntico** mas em ramo diferente do mesmo arm. **NÃO é
     paradigma novo** — é reaplicação composicional.
2. **A.0.0 produzido empiricamente** — magnitude do bug factual.
3. **Elementos estructuralmente novos identificados**:
   - **Subcategoria §8.4 clarificada** — A vs B distinção
     primeira-vez registada empiricamente.
   - **Matrix feature × sintaxe primeira aplicação** (P302 §6.7
     lição metodológica).
   - **Simetria arquitectural lookup-hit/miss** — P302+P303
     completam ambos os ramos.
4. **Decisão sobre promoção ADR meta**:
   - **§8.4 N=2 ou N=3 conforme subcategoria** — N=3 ambíguo
     se subcategoria diferente; **NÃO promover por inflação
     ambígua**.
   - **§8.7' N=10** — adiamento standard P300-P302.
   - **§8.3 N=13** candidato.
   - Default: **sem promoção** (anti-padrão honrado 11ª vez).

**Critério estrito P273.17 §0**: subcategoria §8.4 ambígua
**não justifica promoção forçada**. Preservar §8.4 como
ferramenta emergente; aguardar N≥3 da **mesma subcategoria**
genuína.

---

## §3 — Materialização

**Cenário default (HX' + A.0.0 → subcategoria B + A.2 → (a))**:

1. Modificar arm `Expr::FuncCall` em `eval/math.rs` ramo `else`:
   ```rust
   } else {
       // P303: paralelo P302 para identifier desconhecido —
       // preservar args via MathSequence + MathDelimited.
       let pos_args: Vec<Expr<'_>> = call.args().items()
           .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
           .collect();
       if pos_args.is_empty() {
           // `undef()` — args vazios; só MathIdent.
           return Ok(Content::MathIdent(name.into()));
       }
       // Mesma estrutura body de P302.
       let body = if pos_args.len() == 1 {
           eval_math_expr(scopes, ctx, pos_args[0])?
       } else {
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
       Ok(Content::MathSequence(std::sync::Arc::from(vec![
           Content::MathIdent(name.into()),
           delimited,
       ])))
   }
   ```
2. Refactor candidato: se P302 + P303 partilham 90% código,
   extrair helper `wrap_with_args(base: Content, args: ...)`.
   A.2 → (b) trata isto. Default: aceitar duplicação minor
   para clareza.
3. Actualizar teste P302 `p302_undef_parens_continua_mathident_fallback`:
   - **Renomear/substituir** para
     `p303_undef_parens_produz_mathsequence_com_delimited`.
   - Validar nova estrutura.
4. Testes P303 (+5-8 L1):
   - L1 `$undef(x)$` → `MathSequence([MathIdent, MathDelimited])`.
   - L1 `$undef(x + y)$` → args complexos.
   - L1 `$undef(x, y)$` → múltiplos args separados por ", ".
   - L1 `$undef()$` → só MathIdent (sem wrapper).
   - L1 `$undef$` → regressão sem parens.
   - L1 `$sin(x)$` → regressão P302 preservada.
   - L1 cross-construct matrix verificação completa.
5. Sem caps. Estimativa testes: ~5-8.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P302: 2 876 testes.
  Esperado: ~2 881-2 884.
- `crystalline-lint` zero violations.
- Hash L0 inalterado em todos os ficheiros (extensão dentro
  contrato).
- Hash `export.rs` **preserved bit-exact** pelo **20º passo
  consecutivo** P282-P303.
- **Regressão bit-exact validada** — `$sin(x)$` P302 preservado;
  `$undef$` (sem parens) preservado.
- Teste P302 `p302_undef_parens_continua_mathident_fallback`
  removido/substituído (documenta comportamento que P303
  invalida).
- Bug factual resolvido — `$undef(x)$` produz output esperado.
- Diagnóstico A.0.0+A.0-A.5+A.5' produzido.
- **Sem promoção ADR meta** — 11ª vez consecutiva.
- Subcategoria §8.4 documentada honestamente (B esperado).

---

## §5 — Não-objectivos

- **Não** materializar function call semântica para identifiers
  desconhecidos. Cristalino preserva paradigma "user identifier
  desconhecido = MathIdent literal".
- **Não** estender `lookup_math_op` para scope global. Operadores
  permanecem em scope `math` apenas (P299).
- **Não** alterar comportamento `$sin(x)$` P302.
- **Não** alterar comportamento `$undef$` (sem parens).
- **Não** promover §8.4 N=3 por subcategoria ambígua. Default
  preservar N=2 (categoria A genuína).
- **Não** confundir P303 com P302 — paralelo arquitectural mas
  ramo diferente; reuso composicional.

---

## §6 — Pendências relacionadas

Resolve:
- Bug `$undef(x)$` args descartados — P302 §7 + §8 + §9 frente
  XS+ pendente.

Não resolve:
- P295.1 nota rodapé.
- Cosméticos cleanup agregado.
- P296.X/P297.X refinos cluster math.
- Outras pendências catálogo P300.

---

## §7 — Risco residual

Risco principal: **A.0.0 → HZ' (vanilla rejeita)** — cristalino
deveria divergir e rejeitar `$undef(x)$` como function call
inválida. Mitigação: A.0.0 verificação empírica; default
conservador HX' (preserva fallback "renderiza literal");
documentar divergência se HZ'.

Risco secundário: **regressão `$undef$` sem parens**. Mitigação:
§4 teste regression obrigatório.

Risco terciário: **subcategoria §8.4 ambígua usada para inflar
N=3**. Mitigação: §A.5' critério estrito; default subcategoria
B; preservar N=2 da subcategoria A.

Risco quaternário: **duplicação código P302+P303**. Mitigação:
A.2 → (b) refactor `wrap_with_args` se >90% duplicação. Default
aceitar duplicação minor.

Risco quinário: **bug `$sin(x)$` re-emerge** por interacção
inesperada com mudança P303. Mitigação: regressão bit-exact P302
obrigatória.

Risco senário: **Matrix feature × sintaxe (P302 §6.7 lição) não
aplicada em P303**. Mitigação: §A.5 P303 aplica explicitamente
primeira-vez.

---

## §8 — Ponteiros

- Sítio bug: `01_core/src/rules/eval/math.rs` arm `Expr::FuncCall`
  ramo `else` (linha ~270 pós-P302).
- Variants reusados: `MathSequence`, `MathDelimited`, `MathIdent`,
  `MathText`. Todos pré-existentes.
- Vanilla: `lab/typst-original/crates/typst-eval/src/math.rs`.
- Precedente directo: **P302** (fix `sin(x)` simétrico
  lookup-hit).
- Precedente §8.4 N=1: P288 (NBSP fix).
- Precedente §8.4 N=2: P302 (sin parens fix).
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções).
- ADR cultural: P273.17 §0 (anti-padrão; subcategoria ambígua
  **não** promove §8.4 N=3).
- Padrão §8.7' A.0.0 template (N=10 reaplicação).
- Sub-padrão §8.4 — N=2 preservado se subcategoria B; N=3
  candidato ambíguo se subcategoria A.

---

*Spec P303 produzida 2026-05-19 pós-P302 (bug `sin(x)` parens
fixed via MathSequence + MathDelimited). Frente **bug irmão
$undef(x)$** — paralelo arquitectural directo P302 mas no ramo
lookup-miss (operador desconhecido). Magnitude XS+ esperada;
reuso composicional total (zero variants novos). Fase A com
**7 secções** A.0.0+A.0-A.5+A.5'. **Subcategoria §8.4 ambígua**:
A (bug derivado P302) vs B (bug pré-existente independente);
A.0.0 decide; default **B conservador** (§8.4 N=2 preservado).
**Matrix feature × sintaxe primeira aplicação explícita** (P302
§6.7 lição metodológica). **Hash `export.rs` preservado** pelo
20º passo consecutivo P282-P303. **Sem promoção ADR meta** —
default; 11ª vez consecutiva anti-padrão P273.17 §0 honrado.
**Reuso composicional total**: confirma que infraestrutura
cristalina suporta bug fixes paralelos sem inflação arquitectural.
Sem caps LOC ou magnitude (P282 §7).*
