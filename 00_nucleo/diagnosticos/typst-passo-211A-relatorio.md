# Relatório do passo P211A — Diagnóstico-primeiro Outline configurável

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-211A.md`.
**Tipo**: diagnóstico-primeiro reduzido (zero código tocado).
**Magnitude planeada**: S-M (~45 min). **Magnitude real**: S (~30min).
**Marco**: M9c (Bloco VII — Outline configurável).

---

## §1 O que foi auditado

Mapeado empíricamente o gap entre `outline()` cristalino
actual e o target Bloco VII de `P207A.div-1` aprovado (item
55a — `outline(target, depth, indent, fill)`). Foco em 5
dimensões: outline actual, vanilla outline, divergência
outline-toc principal, consumers reais, custo empírico
sub-features. Zero código tocado; 1 output (este).

---

## §2 Auditoria A1–A5

### A1 — `outline()` cristalino actual (CONFIRMADO)

Cristalino tem **intercepção directa em eval** (não stdlib
func registada):

```rust
// 01_core/src/rules/eval/closures.rs:210-215
// Intercepção de `outline()` — produz Content::Outline (Passo 61).
if let Expr::Ident(ident) = call.callee() {
    if ident.as_str() == "outline" {
        return Ok(Value::Content(Content::Outline));
    }
}
```

`Content::Outline` é **unit variant** (P178) — sem fields.
Layout em `layout/mod.rs:565` consume e expande para TOC
auto-gerado. **Args ignorados** — `outline(depth: 2)` produz
mesma `Content::Outline` que `outline()`.

Sem `native_outline` em `stdlib/`; intercepção precede
dispatch normal.

`ElementKind::Outline` existe (per `element_kind.rs:27`) +
`ElementPayload::Outline` (per `element_payload.rs:91`).

### A2 — `outline()` vanilla (CONFIRMADO)

`lab/typst-original/.../model/outline.rs:150-244` define
`OutlineElem` com 4 params:

```rust
pub struct OutlineElem {
    pub title:  Smart<Option<Content>>,                // Auto/None/Content
    #[default(LocatableSelector(HeadingElem::ELEM.select()))]
    pub target: LocatableSelector,                     // Heading default
    pub depth:  Option<NonZeroUsize>,                  // None = all levels
    pub indent: Smart<OutlineIndent>,                  // Auto/Length/Function
}
```

`fill` é param de `OutlineEntry` (sub-elem em linha 499),
**não** do main `outline()`. Spec P211A §1 mencionou 4
features mas vanilla só tem 3 directos em `outline()` +
`fill` em `outline.entry()` show-set.

Tipos exactos:
- `target`: `LocatableSelector` (wrapper sobre Selector).
- `depth`: `Option<NonZeroUsize>`.
- `indent`: `Smart<OutlineIndent>` (enum auto/length/function).

### A3 — Divergência outline-toc principal (RE-CONFIRMADA)

Per P206C / P206D D5: cristalino auto-toc via
`Content::Outline` unit + `layout/mod.rs:565` expansão
auto-gerada; vanilla emite outline body via show-rule de
`OutlineElem` que itera matches do `target` selector.

**Implicação**: item 55a (target configurável) **não
resolve** a divergência principal. Mesmo com `target: Figure`
selector vanilla, cristalino auto-toc só gera de headings
(per P200 série). Resolver divergência exige refactor do
expand pattern em `layout/mod.rs:565` — fora do escopo
M9c.

### A4 — Consumers reais imediatos (CONFIRMADO — zero)

Grep `outline(target/depth/indent/fill/title)` em
`01_core/`, `02_shell/`, `03_infra/`, `04_wiring/`: **zero
production matches**. Tests existentes (per P200 série)
invocam `outline()` zero-arg apenas.

Pattern consistente M9c: zero consumers reais imediatos
(P207D C1.1, P208B C1.3, P209D C1.3, P210A A5, **P211A
A4**).

### A5 — Custo empírico item 55a (ESTIMADO)

Para cada sub-feature de item 55a:

| Sub-feature | Refactor cristalino | Custo |
|-------------|---------------------|-------|
| `target: ElementKind/Selector` | `Content::Outline` unit → struct com field; closures.rs:213 intercept ajustado; layout/mod.rs:565 expand filtra walk por kind/selector. | S-M (~1.5-2h) |
| `depth: usize` | Field novo; layout/mod.rs filter por level. | S (~30min-1h) |
| `indent: Length` | Field novo; layout expand aplica indent CSS-like. | S (~30min) |
| `fill: Content` | Vanilla é `outline.entry` show-set, não `outline()` direct param. Out of M9c scope. | (n/a) |

Sub-total realistic: **M (~2.5-3.5h)** para 3 sub-features
viáveis (target + depth + indent). `fill` é vanilla
diferente (show-set sobre OutlineEntry), fora escopo.

Spec P207A estimou M-L (~4-6h) — ligeiramente conservador.
Refactor `Content::Outline` unit → struct é o maior componente
(~1.5-2h sozinho).

---

## §3 Decisões C1–C5

### C1 — Forma do outline configurável

(Aplicável apenas se Caminho 2/3 fixado.)

Per A2 — 3 sub-features viáveis cristalino:

```rust
// Hipotética assinatura cristalino (se materializar):
Content::Outline {
    target: Option<Selector>,        // None = heading default
    depth:  Option<NonZeroUsize>,    // None = all levels
    indent: Option<Length>,           // None = default 1.2em/level
}
```

Backwards-compat: `outline()` zero-arg ↔ `Content::Outline {
target: None, depth: None, indent: None }`.

### C2 — Como expor `target`

(Aplicável apenas se Caminho 2/3 fixado.)

**Opção β** — `target: Selector` (não ElementKind). Reusa
trabalho P209 (5 variants Selector); paridade vanilla
`LocatableSelector` próxima. Limitação herdada P209D:
`Selector::Regex` é stub `vec![]`; `Selector::Where` adiado.
Para outline target, `Selector::Kind` + `Selector::Or` cobrem
maioria dos casos vanilla.

### C3 — **Caminho 1 puro fixado**

Justificação literal:

- **A4 zero consumers absoluto**: nenhum production caller
  de `outline(...)` com params.
- **A5 M-L cost sem consumer**: refactor `Content::Outline`
  unit → struct é refactor cross-modular (closures.rs +
  content.rs + layout/mod.rs + tests existentes); 2.5-3.5h
  estimado para 3 features.
- **A3 divergência principal não resolvida**: Bloco VII item
  55a apenas adiciona params; cristalino auto-toc vs vanilla
  outline-body show-rule continua divergente
  arquitectonicamente. Materializar params não fecha gap
  "completude vanilla".
- **Pattern M9c consolidado**: 8 aplicações anti-inflação
  consecutivas (P205D, P207E, P208B C1, P208D, P209C-vazios,
  P209D C6, P209E C1.2, P210 Caminho 3). **9ª aplicação
  cumulativa** em P211A.

**Caminho 1 puro fixado**: P211 série fecha em **1 sub-passo
apenas (P211A diagnóstico)**. Sem P211B+. M9c salta directo
para P212.

Reabertura futura: quando consumer real emergir (e.g., test
fixture com `outline(depth: 2)`), Bloco VII materializa-se
em sub-passo pós-M9c dedicado.

### C4 — Plano P211 (revisado)

**1 sub-passo apenas**: P211A (este). Sem P211B+.

Anotações em P211A relatório:
- ADR-0076 §Plano de materialização: P211 série transita
  "PENDENTE" → "✅ MATERIALIZADO 2026-05-12 (Caminho 1 puro
  — skip via anti-inflação 9ª aplicação)".
- Não há blueprint marca em P211 (apenas P211C/P211D
  encerramentos teriam marca; Caminho 1 puro não exige
  marca separada). **Alternativa**: marca §3.0octies como
  pattern formalização. Decisão deferida ao executor —
  registar como nota se útil.
- Não há ADR nova em P211.

### C5 — Magnitude agregada P211

**S (~30min)** total — apenas P211A diagnóstico.

Comparação com hipóteses:
- Caminho 1: 0 código (~30min docs) ✓ FIXADO
- Caminho 2: M-L (~4-6h) — rejeitado
- Caminho 3: S-M (~1.5-2h) — rejeitado

---

## §4 Magnitude agregada P211

**S (~30min real)** — 1 sub-passo (apenas P211A).

P211 é a 1ª série M9c a fechar em 1 sub-passo (vs P207 com 5,
P208 com 4, P209 com 5, P210 com 3). Confirma pattern
anti-inflação como mecanismo de redução real do orçamento.

---

## §5 Plano P211 final

**Sem P211B+. Série fecha aqui.**

| Sub-passo | Tipo | Magnitude | Output |
|-----------|------|-----------|--------|
| P211A | Diagnóstico-primeiro reduzido + encerramento série | S (~30min) | Auditoria A1-A5 + decisões C1-C5 + Caminho 1 puro + ADR-0076 anotada. Este relatório. |

**Anotações executor P211A**:
- Após escrever este relatório, executor anota ADR-0076 §P211
  "✅ MATERIALIZADO 2026-05-12 (Caminho 1 puro)".
- Opcionalmente: blueprint §3.0octies marca se executor
  julgar útil para consistência pattern marca-por-fecho.
  Caminho 1 puro técnicamente não tem material para marcar
  além do que ADR-0076 §P211 já documenta.

**Pré-condições mantidas**:
- Trait `Introspector` mantém 26 métodos.
- Stdlib funcs mantém ~53 (sem novas em P211).
- Tests 1939 verdes (sem código tocado).
- ADR-0076 mantém PROPOSTO (até P212).

---

## §6 Próximo sub-passo

**P212** — encerramento M9c.

Anteriores P211B-E **não acontecem** (Caminho 1 puro).
Trajectória M9c salta directo para P212:

- Auditoria 7 condições ADR-0076 §Plano de validação.
- Transição ADR-0076 PROPOSTO → ACEITE.
- Blueprint marca final M9c fechado.
- Relatório consolidado M9c (paralelo a relatórios M7/M8
  consolidados).

Magnitude P212: S documental (~1-2h estimado per P207A C11
P211B referência; mas P212 é encerramento M9c inteiro, não
série).

**Estado actual M9c**: 5 séries fechadas (P207 + P208 + P209
+ P210 + P211). Restam apenas P212 (encerramento M9c).
ADR-0076 PROPOSTO; ADR-0077 ACEITE.

Custo agregado M9c (cumulativo até P211A):
- P207A-E: ~10h.
- P208A-D: ~3h.
- P209A-E: ~4h.
- P210A-C: ~1.5h.
- P211A: ~30min.
- **Total cumulativo**: ~19h.
- + P212: ~1-2h.
- **Total esperado M9c**: ~20-21h.

Bem dentro do orçamento original aprovado (~30-50h por
`P207A.div-1` escopo amplo; reduzido empíricamente via
8+1=9 aplicações anti-inflação consecutivas).
