# Relatório do passo P208A — Diagnóstico-primeiro `Tracked<Context>` análogo cristalino

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-208A.md`.
**Tipo**: diagnóstico-primeiro reduzido (zero código tocado).
**Magnitude planeada**: S-M (~45 min). **Magnitude real**: S.
**Marco**: M9c (Bloco IV — `here()` + `locate()`).

---

## §1 O que foi auditado

Mapeado empíricamente como cristalino vai dar suporte a
`here() -> Location` e `locate(selector) -> Location` na
stdlib. Foco em 3 dimensões: vanilla `Tracked<Context>`
pattern (referência), stdlib cristalino actual (precedentes
`native_X` + `query`), e infraestrutura disponível
(`EvalContext.introspector`, `Layouter.current_location`).
Zero código tocado; 1 output (este).

---

## §2 Auditoria A1–A6

### A1 — Vanilla `here()` + `Tracked<Context>` (CONFIRMADO)

Localização literal:

- `lab/typst-original/crates/typst-library/src/foundations/context.rs`:
  ```rust
  pub struct Context<'a> {
      pub location: Option<Location>,
      pub styles:   Option<StyleChain<'a>>,
  }

  #[comemo::track]
  impl<'a> Context<'a> {
      pub fn location(&self) -> HintedStrResult<Location> { ... }
      pub fn styles(&self) -> HintedStrResult<StyleChain<'a>> { ... }
      pub fn introspect(&self) -> HintedStrResult<()> { ... }
  }
  ```
- `lab/typst-original/crates/typst-library/src/introspection/here.rs:47`:
  ```rust
  #[func(contextual)]
  pub fn here(context: Tracked<Context>) -> HintedStrResult<Location> {
      context.location()
  }
  ```

`Tracked<Context>` é passado a stdlib funcs pelo evaluator
vanilla durante `#context { ... }` blocks. Multi-pass:
context muda entre passes; comemo::track invalida cache
quando `Context::location()` retorna diferente.

### A2 — Stdlib cristalino: assinatura `native_X` (CONFIRMADO)

Pattern uniforme em `01_core/src/rules/stdlib/*.rs`:

```rust
pub fn native_X(
    ctx:           &mut EvalContext,
    args:          &Args,
    world:         &dyn World,
    current_file:  FileId,
    figure_numbering: Option<&str>,
) -> SourceResult<Value>
```

Precedente concreto: `native_query` em `foundations.rs:426`
consulta `ctx.introspector.query(&selector)` e retorna
`Value::Array(Vec<Value::Location>)`. **Sem `Tracked<X>` em
nenhuma func cristalina** — pattern arquitectónico
estabelecido: stdlib lê `EvalContext` directamente.

### A3 — Cristalino `EvalContext` (CONFIRMADO)

`01_core/src/rules/eval/mod.rs:86` define `EvalContext`:

```rust
pub struct EvalContext {
    pub loop_iterations:     usize,
    pub max_loop_iterations: usize,
    pub next_rule_id:        RuleId,
    pub introspector:        TagIntrospector,  // P174: snapshot da
                                                // iter de fixpoint
                                                // anterior.
}
```

`introspector` é **snapshot read-only** (P174 / M7 sub-passo
1). Stdlib lê, nunca escreve. **Sem `current_location` field
actualmente.** Pattern multi-iter fixpoint: cada iteração
constrói novo introspector via walk; eval da iter N lê
introspector da iter N-1.

### A4 — Layouter `current_location` (DIVERGÊNCIA)

`01_core/src/rules/layout/mod.rs:149`:

```rust
pub(super) current_location: Option<Location>,  // P185C
```

Field `pub(super)` (módulo-privado) — Layouter avança
durante o walk de layout em locatable boundaries. Consumers
location-aware (`is_numbering_active_at`, `flat_counter_at`,
P185B) consultam via methods locais.

**Divergência crítica**: `current_location` vive no
**Layouter** (fase 3 layout), não no **EvalContext** (fase
2 eval). `here()` precisa ser evaluable durante eval (fase
2). Cristalino single-pass não tem "current_location during
eval" naturalmente — eval emite `Content` sem conhecer
ordem de aparecimento; a ordem é fixada no walk
introspect/layout subsequente.

### A5 — Cristalino `Context` (NÃO APLICÁVEL)

Grep `pub struct Context` em `01_core/src/` retorna **zero
matches**. Cristalino L1 não tem tipo `Context` análogo a
vanilla. Confirma hipótese da spec.

`Value::Location(Location)` variant existe em `Value`
(P179), pelo que Location já é first-class no value system
— qualquer mecanismo cristalino pode emitir/manipular
Location como Value.

### A6 — Comparação 3 opções de design

| Opção | Mecanismo | Custo | Adequação cristalino |
|-------|-----------|-------|----------------------|
| **1 — Espelhar vanilla `Tracked<Context>`** | Criar `Context` em L1; passar `Tracked<Context>` a todas as stdlib funcs `here()`/`locate()`. Requer refactor das ~30 assinaturas `native_X`. | **L** (~5-7h) | **Anti-pattern**. `Tracked<Context>` é específico de vanilla multi-pass; cristalino tem fixpoint cross-iter via `EvalContext.introspector` snapshot. Tracked envolvendo é redundante. |
| **2 — Especializado cristalino** | Estender `EvalContext` com `current_location: Option<Location>` (+ `Locator` se necessário); stdlib `here()` lê directamente. `locate(sel)` delega a `Introspector::query(sel).first().copied()`. | **M** (~3-5h) | **Preferido**. Reusa pattern P174 (snapshot via `EvalContext`) + P175/P179 (Selector + Value::Location). Sem novo tipo wrapper. |
| **3 — Compute-on-demand (thread-local)** | `here()` consulta thread-local `current_location` set externamente. | **S** (~1-2h) | **Rejeitado**. `static` em L1 viola V13 `MutableStateInCore`. Anti-pattern absoluto. |

**Recomendação**: Caminho 2 (especializado cristalino). Per
`P205A.div-1` — divergências arquitectónicas legítimas;
single-pass cristalino justifica forma distinta.

**Caveat empírico**: implementação de `here()` requer
mecanismo para popular `current_location` no `EvalContext`.
Opções (a refinar em P208B):

- (i) Eval walk avança um Locator em locatable boundaries
  (mirror do Layouter P185C). Custo: M.
- (ii) `here()` só funciona em contextos diferidos — emite
  placeholder eval-time, resolved layout-time. Custo: M+
  (introduz `Content::Context { body }` análogo a vanilla
  `#context` block).
- (iii) `here()` retorna `Value::Location(introspect_snapshot)`
  da iter anterior — só faz sentido cross-iter. Custo: S
  (zero refactor), mas semântica pode divergir vanilla.

P208B fixa qual sub-mecanismo. P208A não pré-fixa para
evitar inflação.

---

## §3 Decisões C1–C5

### C1 — Forma do contexto cristalino: **Caminho B (especializado)**

`EvalContext` é estendido com infraestrutura `current_location`
(mecanismo exacto fixado em P208B). Sem novo tipo `Context`;
sem `Tracked<X>` envolvendo. Reusa pattern P174 directamente.

Critério: simplicidade + reuso de patterns + custo agregado
M (vs L de Caminho A; vs anti-pattern de Caminho C).

### C2 — `locate(selector)` design: **trivial via `Introspector::query`**

Implementação literal:

```rust
pub fn native_locate(
    ctx: &mut EvalContext, args: &Args, ...,
) -> SourceResult<Value> {
    let selector = /* parse args → Selector */;
    let first = ctx.introspector.query(&selector).first().copied();
    Ok(match first {
        Some(loc) => Value::Location(loc),
        None      => Value::None,
    })
}
```

Reusa trait `Introspector::query` (P175) + `Value::Location`
(P179). Custo: trivial (~30min impl + tests).

Limitação herdada: `Selector` cristalino só tem `Kind`
variant actualmente (P175 minimal). `locate(selector(label))`
exige P209 (Selector::Label) — documentar em P208A C5 como
pré-condição cross-série. Per `P207A.div-1`: P208 e P209
seguem séries dedicadas.

### C3 — Stdlib funcs assinatura: **directa, sem Tracked**

```rust
pub fn native_here(
    ctx:              &mut EvalContext,
    args:             &Args,
    _world:           &dyn World,
    _current_file:    FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value>

pub fn native_locate(
    ctx:              &mut EvalContext,
    args:             &Args,
    _world:           &dyn World,
    _current_file:    FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value>
```

Paridade total com pattern `native_query` (P179). Zero
divergência em assinatura. Mecanismo de obter `current_location`
vive **dentro** de `EvalContext` (não na assinatura).

### C4 — Magnitude agregada P208: **M (~3-5h)**

| Sub-passo | Estimativa |
|-----------|------------|
| P208B (infra + `here()`) | S-M (~2-3h) — depende de qual sub-mecanismo (i)/(ii)/(iii) é fixado |
| P208C (`locate()`) | S (~30min-1h) — trivial após Selector parsing arg |
| P208D (encerramento + relatório) | S (~30min-1h) — documental |
| **Total série P208** | **M (~3-5h)** |

Magnitude real depende de sub-mecanismo P208B. Se (iii)
(reuso snapshot per cross-iter), magnitude desce para S.
Se (i) ou (ii) (refactor estrutural), permanece M.

### C5 — Plano P208B-D (sem ramos)

- **P208B** — Materializar infraestrutura `current_location`
  em `EvalContext` + `native_here()` stdlib. Diagnóstico
  interno (C1) fixa sub-mecanismo (i/ii/iii). Tests
  unitários + 1-2 tests E2E. Trait `Introspector` **não
  estende** — `here()` é func stdlib, não trait method
  (per spec §6 risco 4). Sub-passo `*B+` pode ser
  registado se sub-mecanismo revelar custo XL.
- **P208C** — Materializar `native_locate()` stdlib.
  Trivial após P208B + Selector argparse extension.
  Limitação `Selector::Kind` apenas — `locate(selector(label))`
  fica para P209.
- **P208D** — Encerramento série P208 documental
  (paralelo a P207E Caminho 1):
  - ADR-0076 §Plano de materialização: série P208 transita
    "PENDENTE" → "✅ MATERIALIZADO".
  - Blueprint §3.0quinquies marca (paralelo a §3.0quater
    P207E).
  - Relatório curto.
  - Critério Caminho 1 (puro) **vs** Caminho 2 (com
    captura adicional, ex: `#context` element materialização)
    fixado em P208D próprio C1.

---

## §4 Magnitude agregada P208 série

**M (~3-5h estimado)**. Sub-passos:
- P208B: S-M
- P208C: S
- P208D: S documental

Caveat: P208B sub-mecanismo (i/ii/iii) define exactidão.
Se P208B revelar XL (refactor eval walk significativo),
considerar `P208B.div-N` ou desdobrar em P208B + P208B-2.

---

## §5 Plano P208B-D (resumo executável)

| Sub-passo | Tipo | Magnitude | Output principal |
|-----------|------|-----------|------------------|
| P208B | Infra + `here()` materialização | S-M | `EvalContext.current_location` + walk advance + `native_here()` + 4-6 tests. ADR-0076 §P208 actualizado. |
| P208C | `locate()` materialização | S | `native_locate()` stdlib + 2-3 tests. Documenta limitação `Selector::Kind` apenas (P209 desbloqueia full). |
| P208D | Encerramento série P208 | S documental | ADR-0076 série anotada; blueprint §3.0quinquies marca; relatório resumo. Caminho 1 vs 2 fixado em C1 próprio. |

**Pré-condições mantidas**:
- Trait `Introspector` permanece 26 métodos (P208 não
  estende trait — `here()`/`locate()` são stdlib funcs).
- Regra empírica P207B §5 **não acionada** (zero novos
  trait methods).
- Pattern P174 + P175 + P179 reusados literal.

---

## §6 Próximo sub-passo

**P208B** — primeiro item de materialização da série.

Pré-condição cumprida: P208A diagnóstico fechado; C1
fixado em Caminho B; C2 trivial; C3 assinatura fixada; C4
magnitude reportada; C5 plano sem ramos.

P208B exige diagnóstico interno breve (sub-mecanismo i/ii/iii
para popular `current_location`). Esse diagnóstico
acontece **dentro** de P208B C1 — não é P208A.div-1.

Estado M9c: 4 séries pendentes (P208 `here`/`locate`, P209
Selector, P210 page-aware consolidação opcional, P211
encerramento M9c). ADR-0076 mantém `PROPOSTO` até P211B.
