# Passo 299 — Operadores math pré-definidos (P298.X)

**Frente**: `P298.X — operadores math pré-definidos` (frente
pendente P298 §8 e §10).
**Origem**:
- P298 §3.4: cristalino tem heurística `is_limit_function`
  hardcoded com **7 operadores** (`lim/max/min/sup/inf/limsup/liminf`).
- P298 §8: vanilla `op.rs` define via macro `ops!` ~36+ operadores
  pré-definidos no scope `math`; cristalino actual usa `MathIdent`
  literal + heurística — workaround viável.
- P298 §9: **P299 deve ser ortogonal por construção** (paralelo
  P293 pós-série Style). `P298.X` qualifica como ortogonal
  (refino de feature específica, não continuação cumulativa).
**Pré-requisitos**: P298 (`MathOp` variant materializado).
**Tipo declarado**: **1º passo ortogonal pós-cluster math
P296-P298** — paralelo P293 pós-série Style P288-P292.
**Magnitude**: S-M esperada, mas **scope concreto condicional
a A.0.0** (paralelo P293 §A.0.0).
**Marco**: 1ª aplicação prática de `Content::MathOp` (P298)
para registo de operadores pré-definidos.

---

## §1 — Objectivo (provisório; depende de A.0.0)

Materializar operadores math vanilla pré-definidos
(`sin`/`cos`/`tan`/`lim`/`det`/etc., ~36+) como funções stdlib no
scope `math` — usando `Content::MathOp { text, limits }` (P298)
como mecanismo de produção.

**Mas o scope concreto é ambíguo** — paralelo P293 §A.0.0. Os
36+ operadores não podem ser todos no mesmo passo (magnitude
explode). Subdivisões possíveis:

| Subdivisão | Scope | Magnitude |
|---|---|---|
| **P299.A** | Scripts-style operators (`sin`/`cos`/`tan`/`ln`/`log`/`exp`/etc., ~22 funcs) | S |
| **P299.B** | Limits-style operators completos (já cobertos parcialmente por `is_limit_function`) — `det`/`gcd`/`lcm`/`arg`/`Pr` etc. | XS-S |
| **P299.C** | Integração scope module `math.sin`/`math.lim` (acesso namespaced) | Depende de scope module existir |
| **P299.D** | Todos os 36+ em passo único agregado | M-L |

**Decisão A.0.0 obrigatória** clarifica qual subset é P299
concreto.

### §1.1 — Comparação directa com P293 (1º ortogonal pós-Style)

P293 inaugurou A.0.0 N=1 com referência ambígua na origem
("ADR-0078 sub-fase b" não cobria curve geometry). Resultou em
H6 descoberta empírica.

P299 está em **posição arquitectural simétrica**: 1º ortogonal
pós-cluster math; scope concreto ambíguo (qual subset dos 36+
operadores?). A.0.0 vai descobrir hipótese HJ/HK/HL/HM/HN
literal.

**Diferença material vs P293**: P299 tem **dependência directa
explícita** (`MathOp` materializado P298) — não é ortogonal
total. É "ortogonal categoricamente" (não cumulativo no mesmo
padrão) mas "dependente arquitecturalmente" (usa variant
pré-existente).

### §1.2 — Razões

1. **Frente pendente registada** P298 §8 — extensão directa do
   trabalho concreto P298.
2. **1ª aplicação prática `Content::MathOp`** — primeiro uso
   concreto do variant materializado em P298. Sem P299, MathOp
   é "framework sem clientes".
3. **Reset categórico** — P299 sai do cluster math
   (handlers/variants) para stdlib (registo de funções
   pré-definidas).
4. **Reaplicação ADR-0098 + ADR-0099** — N=16 + N=15
   cumulativos se hash preservado.
5. **A.0.0 N=7** — 7ª reaplicação consecutiva; testa
   persistência do template após 6 aplicações com magnitudes
   variadas.
6. **Possível qualificação de sub-padrão "cross-variant
   interaction" N=2** se P299 introduz pattern paralelo (e.g.
   stdlib func consultar `MathOp` constants).

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 8 secções)

A.0.0 + A.0-A.5 + A.5' (paralelo P298) + **A.0.0' subdivision
decision** (nova, paralelo P293 estilo).

### A.0.0 — Verificação literal estado actual operadores (N=7 cumulativo §8.7')

Inspecção literal obrigatória:

1. **`grep -rn "is_limit_function\|is_large_operator" 01_core/`** —
   confirmar P298 § referências; ver implementação literal de
   `symbols::is_limit_function` (esperado: ~7 strings hardcoded).
2. **Listar conteúdo de `01_core/src/symbols/`** (ou caminho
   equivalente) — quais operadores actualmente reconhecidos.
3. **`grep -rn "math::sin\|math\\.sin\|scope.*sin" 01_core/`** —
   verificar se cristalino tem scope `math` module com funcs
   pré-definidas.
4. **`grep -rn "native_op\|MathOp" 01_core/src/rules/stdlib/`** —
   estado pós-P298 do `native_op`.
5. **Inspeccionar `lab/typst-original/.../math/op.rs`** — macro
   `ops!` literal:
   - Lista de scripts-style operators (~22).
   - Lista de limits-style operators (~12).
   - Para cada: `text` string, `limits` flag.
6. **Inspeccionar `lab/typst-original/.../math/mod.rs`** — como
   `op` module é exposto no scope `math` vanilla.
7. **Verificar comportamento actual de `$sin x$` em cristalino**
   — funciona via `MathIdent("sin")` literal? Renderiza
   correctamente em scripts-style por default?
8. **Cross-check com `is_limit_function`** — `"lim"` está; mas
   `"det"`/`"gcd"`/`"lcm"` (também limits-style em vanilla)
   estão ou não?

**Decisão A.0.0**:

| Hipótese | Acção |
|---|---|
| **HJ** (sem operadores pré-definidos; `MathIdent` literal funciona em scripts-style por default) | P299 materializa via stdlib registo |
| **HK** (alguns operadores existem) | P299 subset complementar |
| **HL** (heurística cobre tudo via fallback) | P299 = **refino qualitativo**: registo explícito mas semantica preservada bit-exact |
| **HM** (vanilla macro vs tabela estática) | A.2 decisão arquitectural |
| **HN** (mix) | Subdivisão necessária |

**Magnitude esperada A.0.0**: **média-alta**. Cristalino tem
estrutura parcial (`is_limit_function` cobre 7 dos ~12
limits-style); scripts-style é **opaco** (funcionam via
`MathIdent` directo? regredem? não funcionam?).

### A.0.0' — Decisão de subdivisão (nova; paralelo P293 A.0.0)

Após A.0.0 resolver estado actual, decidir scope P299:

| Decisão | Critério | Acção |
|---|---|---|
| **P299 = P299.A** (scripts-style apenas) | Se HK/HL e scripts-style são gap principal | ~22 funcs novas |
| **P299 = P299.B** (limits-style complemento) | Se HK e limits-style é gap principal | ~5 funcs novas |
| **P299 = P299.A+B agregado** | Se HJ (ambos ausentes); magnitude S-M | ~27 funcs novas |
| **P299 = P299.D** (todos 36+ agregados) | Apenas se materialização é trivial via macro/tabela | M-L |
| **P299 = P299.C separado** | Scope module `math.sin` etc. | Frente independente |

**Default sugerido**: subset agregado (A+B) se A.0.0 mostra que
ambos são gap real; subset mínimo (B apenas) se A.0.0 revela
scripts-style funciona bit-exact via fallback `MathIdent`.

**Permitir interrupção honesta** se A.0.0 + A.0.0' revelam
complexidade superior ao XS-M esperado (paralelo P293 §A.0.0.3
HE redirecção).

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado | Procedimento |
|---|---|---|
| `grep "MathOp\|op_pre" 03_infra/src/export.rs` | **Zero hits** — emit agnóstico via `FrameItem::Text` | Inspecção literal |
| `native_op` produz `Content::MathOp { ... }`; layout consume via P298 paradigm | **Sim** — paradigma single source of truth | A.1.5 confirma |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo 16º passo consecutivo | Verificação final |

### A.1 — Inventário literal

8 sub-secções:

1. **A.1.1 — Operadores actuais cobertos por heurística** —
   listar literalmente `is_limit_function` strings + tipo de
   carácteres em `is_large_operator`.
2. **A.1.2 — Operadores vanilla na macro `ops!`** — lista
   completa com `(text, limits)` para cada.
3. **A.1.3 — Scope `math` cristalino actual** — existe? como
   funcs são registadas?
4. **A.1.4 — `MathIdent("sin")` actual** — renderização? Layout
   bit-exact vs scripts-style esperado?
5. **A.1.5 — `native_op` actual** — assinatura pós-P298.
6. **A.1.6 — Vanilla `math::op::sin`** — usabilidade `math.sin x`
   vs `sin x` directo.
7. **A.1.7 — Emit verification** — paralelo P298.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente.

### A.2 — Decisão arquitectural (registo dos operadores)

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** Registar cada operador individualmente como `native_*` no scope global (paralelo `native_accent` P296) | Acesso directo `sin x` sem prefixo | Polui scope global; conflitos com funções user |
| **(b)** Scope module `math` com funcs `math.sin`, `math.lim`, etc. | Paridade vanilla; isolamento | Requer scope module support |
| **(c)** Tabela estática `OPERATORS: &[(&str, bool)]` consultada por `native_op_lookup(name)` | Compromisso; sem N funcs novas | Requer lookup em runtime |
| **(d)** Macro `register_ops!` que expande em N `native_*` funcs | Paridade vanilla `ops!` macro | Complexidade Rust macro |

Default sugerido: **(b)** se A.1.3 confirma scope module
funcional; **(a)** se scope module não existe — fallback
pragmático.

**Decisão genuína condicional a A.1.3**. Sub-frente P299.C
explicitamente trata caso (b) se A.1.3 mostra scope module
ausente.

### A.3 — Integração com `MathOp` (P298)

| Opção | Mecanismo |
|---|---|
| **(α)** Cada `native_sin`/etc. retorna `Content::MathOp { text: Content::text("sin"), limits: false }` directo | Single source of truth; reusa P298 paradigm |
| **(β)** Bypass `MathOp` — retorna `MathIdent("sin")` directo com flag implícita | Preserva caminho pré-P298 |
| **(γ)** Híbrido condicional | Caso edge |

Default sugerido: **(α)** — confirma `MathOp` como ponto único
de produção de operadores math.

### A.4 — Impacto em emit (paralelo P298)

| Opção | Mecanismo |
|---|---|
| **(i)** `FrameItem::Text` agnóstico via `MathOp.text` (P298 paradigm) | Hash preserved 16º passo |
| **(ii)** Caso edge | Improvável dado P298 estabelecido |

Default sugerido: **(i)** quase certo.

### A.5 — Detecção de bugs latentes

Cenários fronteira:

- **`sin x`**: scripts-style; renderização paridade vanilla.
- **`lim_(x→0) f(x)`**: limits-style block mode (via P298
  cross-variant); **regressão potencial** se cristalino actual
  usa `is_limit_function` mas P299 substitui sem manter
  fallback.
- **Operador vanilla com Unicode** (e.g. `arg` que usa "arg"):
  paridade vanilla.
- **Operador `lim` via dois caminhos**: `lim` heurístico + `lim`
  registado via P299 — comportamento idêntico bit-exact?
- **Stdlib name collision**: `sin` definido por P299 colide com
  `sin` calc trig P283? P283 está em `calc.sin`; P299 em `math.sin`
  ou global — verificar A.1.3.

**Regressão crítica**: testes pré-P299 para `$lim_(x→0) sin(x)$`
devem produzir output bit-exact ou intencionalmente alterado;
documentar honestamente.

### A.5' — Verificação anti-reflexão (N=8 do padrão §8.6)

**7ª reaplicação A.0.0** consecutiva.

4 verificações:

1. **Comparação literal A.1.6 P288-P299** — paradigma novo?
   - P293-P298 nove paradigmas distintos identificados.
   - **P299**: paradigma novo "stdlib registo em massa de
     operadores pré-definidos via single source of truth
     (MathOp)" — primeira aplicação de variant existente para
     finalidade arquitectural nova (registo em massa).
2. **A.0 produzido empiricamente** — A.0.0 P299 magnitude.
3. **Elementos estructuralmente novos identificados**:
   - **1ª aplicação prática `Content::MathOp`** — primeiro uso
     concreto pós-materialização P298.
   - **Reset categórico** (cluster math → stdlib).
   - **Sub-padrão "operadores pré-definidos via single source of
     truth"** — paradigma novo se A.3 → (α) escolhida.
   - **A.0.0' subdivision decision** — nova secção; primeira
     spec com decisão de subdivisão explícita pós-A.0.0.
4. **Decisão sobre promoção ADR meta**:
   - **§8.7' N=7** se A.0.0 magnitude alta — primeira oportunidade
     "3 magnitudes altas consecutivas" se P297+P298+P299 todas
     altas. **Relatório P298 §9 sugeriu: "raríssimas circunstâncias
     para adiamento"** se N=7 robusto.
   - **§8.3 N=11** candidato adiado.
   - **Sub-padrão "cross-variant interaction" N=2** se P299
     usa MathOp em forma estendida (e.g. lookup table)
     — improvável.
   - **"Variant rico" N=5** — não qualifica (P299 não cria
     variants novos).
   - **Uma ADR meta por passo no máximo** (P273.17 §0).

**Critério escolha pós-P298**: §6.8 P298 honrou anti-padrão
rigorosamente (3 candidatas, todas adiadas). P299 herda o
critério: **promoção só se gatilho dispara genuinamente** com
magnitude inequívoca.

**Vigilância especial**: se A.0.0 P299 magnitude alta + 3
consecutivas → §8.7' N=7 promoção quase forçada per P298 §9.
**Honestidade**: preferir promoção robusta a adiamento
infinito; se gatilho disparar inequivocamente, **promover**
em vez de adiar.

---

## §3 — Materialização (condicional a A.0.0 + A.0.0')

Após Fase A produzir hipótese + subdivisão + decisões:

**Cenário default (HK + P299.A+B agregado + A.2 → (a) scope global + A.3 → α)**:

1. Identificar lista completa operadores vanilla via A.1.2.
2. Filtrar operadores **já cobertos** por `is_limit_function`/
   `is_large_operator` (paralelo regression test).
3. Para cada operador não-coberto, criar `native_<name>` func
   stdlib que retorna `Content::MathOp { text: Content::text(<name>), limits: <bool> }`.
4. Registar em `eval/mod.rs` scope module conforme A.2 → (a)
   (scope global) ou (b) (scope `math`).
5. Testes:
   - L1 unitário cada operador: construção correcta.
   - L1 unitário catalog: scope contém N novas funcs.
   - L3 regression: PDFs pré-P299 com `$lim_(x→0) sin(x)$` etc.
     preservam bytes (ou registar alteração intencional).
   - L3 integração: 5-10 cenários representativos verificam
     scripts-style/limits-style correctos.
6. Aplicação ADR-0098 obrigatória — A.0 documenta.
7. Promoção ADR meta condicional per A.5':
   - **§8.7' N=7 promoção** se A.0.0 magnitude alta + magnitudes
     consecutivas P297+P298+P299 todas altas.
   - Default: **sem promoção** se ambíguo.
8. Actualizar L0:
   - `rules/stdlib.md` política inalterada (não enumera funcs).
   - **Tabela A.4** — adicionar linha nova "Operadores math
     pré-definidos" ou actualizar linha existente com refª P299.
   - Possivelmente Tabela B sem alteração (P299 não adiciona
     variants).
   - Propagar hashes.
9. Diagnóstico produzido:
   `diagnostico-math-operadores-passo-299.md` com **8 secções**
   A.0.0+A.0.0'+A.0+A.1+A.2+A.3+A.4+A.5+A.5'.

**Sem caps** (per P282 §7). Estimativa de testes: 30-60
(N operadores × 1-2 testes cada + regressions).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P298: 2 852 testes.
  Esperado: ~2 880-2 910 (depende N operadores).
- `crystalline-lint` zero violations.
- Hash L0 `content.md` **preserved** (P299 não adiciona variants).
- Hash L0 `stdlib.md` **preserved** (política única).
- Hash L0 `export.rs` **preserved bit-exact** pelo **16º passo
  consecutivo** se A.4 → (i).
- **Regressão bit-exact validada** — todos os PDFs pré-P299
  com `MathIdent("sin")`/`MathIdent("lim")` etc. preservam
  bytes (caso fallback paths inalterados).
- **OU**: documentar alteração intencional para casos onde
  `native_sin` substitui `MathIdent` fallback (paridade vanilla
  exacta vs heurística aproximada).
- Tabela A.4 actualizada para reflectir P299.
- Diagnóstico A.0.0+A.0.0'+A.0+A.1+A.2+A.3+A.4+A.5+A.5'
  produzido.
- **Promoção ADR meta condicional**:
  - **§8.7' N=7 promoção** se A.0.0 magnitude alta + 3
    consecutivas P297+P298+P299.
  - Default: sem promoção (anti-padrão P273.17 §0).
- Subdivisão decision A.0.0' documentada — se P299.C separado,
  registar para futura frente.

---

## §5 — Não-objectivos

- **Não** modificar `is_limit_function`/`is_large_operator`
  heurística pré-existente. Continua a funcionar como fallback;
  P299 **estende sem substituir** (paralelo P298 cross-variant).
- **Não** materializar scope module `math` se A.1.3 mostra que
  não existe. P299.C separado.
- **Não** materializar cosméticos vanilla `OpElem`
  (`size`/etc.) em P299. ADR-0054 graded.
- **Não** alterar `Content::MathOp` semanticamente — usar
  estado pós-P298.
- **Não** materializar operadores raríssimos (e.g. obsoletos
  vanilla). Lista canónica via A.1.2.
- **Não** promover múltiplas ADRs meta. Uma por passo (P273.17
  §0).
- **Não** confundir P299 com cluster math reaplicação — é
  **categoricamente ortogonal** mesmo usando `MathOp` (variant
  como ferramenta vs criação de variant).

---

## §6 — Pendências relacionadas

Resolve (condicional):
- Frente P298 §8 — operadores math vanilla pré-definidos.
- 1ª aplicação prática `Content::MathOp` pós-materialização P298.

Não resolve (continua aberto):
- P299.C scope module `math.sin` se A.1.3 revelar ausência —
  passo dedicado.
- Cosméticos `OpElem` (`size`/etc.) — ADR-0054 graded.
- Operadores Unicode raríssimos não-vanilla — fora de scope.
- Math shaping completo — ADR-0054 perfil graded.

---

## §7 — Risco residual

Risco principal: **regressão em features math pre-P299**.
`MathIdent("sin")` actualmente funciona via fallback; P299
introduz `native_sin` que retorna `MathOp`. Se cristalino
processa `$sin x$` como `sin` parsed como identifier (não
function call), a heurística pré-P299 continua a aplicar.
Mas se algum sítio chama explicitamente o registo
stdlib, comportamento muda. Mitigação: §4 regression
bit-exact obrigatória; testes específicos para `$sin x$`,
`$lim_(x→0) f(x)$`, etc.

Risco secundário: **A.0.0' subdivision torna spec
provisória**. Mitigação: paralelo P293 §A.0.0.3; permitir
interrupção honesta + renomeamento (P299.A vs P299.D etc.).

Risco terciário: **§8.7' N=7 promoção forçada** por 3
magnitudes altas consecutivas. Mitigação: §A.5' critério —
promover só se gatilho **inequivocamente** disparar. P298 §9
sugere "raríssimas circunstâncias para adiamento" mas não
zero — preservar honestidade.

Risco quaternário: **scope global vs scope `math` conflito**.
Mitigação: A.2 decisão arquitectural; se conflito grave
(`sin` global vs `calc.sin`), abrir sub-passo P299.0 dedicado.

Risco quinário: **subdivisão A.0.0' gera múltiplos sub-passos
P299.A/B/C/D em cadeia** — risco "sequência reflexa". Mitigação:
P298 §9 explicita P299 é ortogonal; subdivisões pós-P299
**não** continuam cumulativamente — cada sub-passo precisa
justificação própria.

Risco senário: **macro/tabela vs N funcs individuais**. Mitigação:
A.2 escolha; se complexidade Rust macro for proibitiva,
fallback (c) tabela estática.

Risco septenário: **performance lookup runtime** se opção (c)
tabela estática. Mitigação: lookup string O(N) com N~36 é
trivial; preocupação prematura.

---

## §8 — Ponteiros

- Tipo a inspeccionar: `01_core/src/entities/content.rs`
  (`Content::MathOp` pós-P298).
- Função stdlib actual: `01_core/src/rules/stdlib/structural.rs`
  (`native_op` P298).
- Heurística limits-style: `01_core/src/symbols/` (ou caminho
  equivalente — `is_limit_function`/`is_large_operator`).
- Layouter math: `01_core/src/rules/math/layout/`.
- Eval scope: `01_core/src/rules/eval/mod.rs` (registo funcs).
- Vanilla: `lab/typst-original/crates/typst-library/src/math/op.rs`.
- Precedente arquitectural directo: **P298** (`Content::MathOp`).
- Precedente "1º passo ortogonal pós-série cumulativa": **P293**
  (`P-curve-geometry`; A.0.0 N=1 inaugural).
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 8 secções
  A.0.0+A.0.0'+A.0-A.5+A.5').
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- ADR scope: ADR-0054 graded.
- Padrão §8.7' A.0.0 template (N=7 reaplicação; promoção
  candidata robusta se 3 consecutivas altas).
- Padrão §8.3 refutação pragmática (N=11 candidato adiado).
- Padrão "cross-variant interaction" (N=1 P298; reaplicação
  improvável P299).
- Padrão "variant rico" N=5 — não aplicável (P299 não cria
  variants).

---

*Spec P299 produzida 2026-05-19 pós-P298 (cluster math 4/4
fechado; sub-padrões 3 candidatos adiados; A.0.0 N=6 magnitude
alta consecutiva). Frente `P298.X — operadores math pré-definidos`
— **1º passo ortogonal pós-cluster math P296-P298**, paralelo
arquitectural P293 (1º ortogonal pós-Style P288-P292). **Magnitude
incerta**: vanilla `op.rs` define ~36+ operadores via macro `ops!`;
A.0.0 + A.0.0' decidem subset concreto P299 (P299.A scripts apenas
~22 funcs; P299.B limits-style complemento ~5; P299.A+B agregado;
ou P299.D agregação total). Fase A obrigatória com **8 secções**:
A.0.0 (estado actual) + **A.0.0' (subdivision decision, nova)** +
A.0-A.5 + A.5'. **§8.7' N=7 candidato robusto**: 6ª reaplicação
consecutiva; magnitude alta esperada (HK/HL hipóteses cristalino
parcial). Se A.0.0 P299 alta + 3 magnitudes altas consecutivas
(P297+P298+P299), **promoção candidata robusta** (P298 §9
"raríssimas circunstâncias para adiamento"). **Mas honestidade
preservada**: promover apenas se gatilho **inequivocamente**
dispara. **1ª aplicação prática `Content::MathOp`** pós-P298 —
single source of truth confirmada cumulativamente. Honestidade
epistémica: spec **aceita não saber** subset concreto antes da
Fase A — paralelo P293 §A.0.0 inaugural. Sem caps LOC ou
magnitude (P282 §7).*
