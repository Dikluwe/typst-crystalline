# Diagnóstico `pad` refino sides individualizadas — Passo P156L

Inventário pré-materialização per **ADR-0065 critério #3**
(expansão de variant existente — primeira aplicação concreta).
**Décima aplicação consecutiva** do padrão diagnóstico-primeiro
iniciado em P154A; **sexta aplicação** do critério estendido
de ADR-0065 (passos P156C/D/G/H/J + agora P156L).

Per ADR-0034 §"Conteúdo mínimo" (7 itens) + 2 itens específicos
para expansão de variant existente per ADR-0065.

---

## 1. Assinatura vanilla `PadElem`

Fonte: `lab/typst-original/crates/typst-library/src/layout/pad.rs`
(56 linhas).

```rust
#[elem(title = "Padding")]
pub struct PadElem {
    pub left:   Rel<Length>,   // #[parse(...)]
    pub top:    Rel<Length>,   // #[parse(args.named("top")?.or(y))]
    pub right:  Rel<Length>,   // #[parse(args.named("right")?.or(x))]
    pub bottom: Rel<Length>,   // #[parse(args.named("bottom")?.or(y))]
    pub x:      Rel<Length>,   // #[external] — atalho left+right
    pub y:      Rel<Length>,   // #[external] — atalho top+bottom
    pub rest:   Rel<Length>,   // #[external] — atalho 4 sides
    pub body:   Content,       // #[required]
}
```

**Tipos e defaults**:
- 4 sides: `Rel<Length>` (relative — pode ser absoluto Length OU
  percentage; cristalino só absoluto per perfil ADR-0054 graded).
- 3 atalhos `#[external]`: x, y, rest — não são fields persistidos,
  apenas resolvidos no parser.
- `body: Content` — obrigatório.

**Precedência vanilla** (lê `pad.rs:20-24`):
```rust
let all = args.named("rest")?.or(args.find()?);  // rest > positional
let x   = args.named("x")?.or(all);              // x   > rest|positional
let y   = args.named("y")?.or(all);              // y   > rest|positional
left = args.named("left")?.or(x);                // left  > x > rest|positional
right = args.named("right")?.or(x);              // right > x > rest|positional
top = args.named("top")?.or(y);                  // top    > y > rest|positional
bottom = args.named("bottom")?.or(y);            // bottom > y > rest|positional
```

**Hierarquia**: específico (left/top/right/bottom) > eixo (x/y) >
geral (rest) > positional Length (vanilla aceita; cristalino actual
não aceita per P156C — body é o único positional aceitável).

**Default por field**: cada um vanilla é `Rel<Length>::zero()`. Em
cristalino actual, defaults aplicados em `native_pad` antes de
construir variant.

---

## 2. Comportamento observável

### 2.1 Caso de uso primário

`pad(body, x: 10pt, y: 20pt)` aplica 10pt em left/right, 20pt em
top/bottom; `pad(body, left: 5pt, rest: 10pt)` aplica 5pt em left
e 10pt nos outros 3.

### 2.2 Mecânica de runtime

Em vanilla, layout consome cada side independentemente: shrink
inset esquerdo, posiciona body, shrink inset direito, etc. Em
cristalino actual (`layout/mod.rs:631`):
- `top` aplicado como `cursor_y += top`.
- `left` aplicado como `line_start_x = saved + left`.
- `bottom` aplicado como `cursor_y += bottom`.
- **`right` scope-out** per ADR-0054 graded — exige refactor
  multi-region não disponível.

Esta limitação **mantém-se em P156L** (não é objecto deste passo).

---

## 3. ADR-0064 Caso aplicável

**Caso C** — campo vanilla `T` com default não-`T::default()`
literal. Cada side vanilla tem default `Rel<Length>::zero()`;
cristalino traduz para `Option<Length>` per side onde `None ↔
zero`. Defaults resolvidos em momento de uso (layout arm).

**Esta é a segunda aplicação concreta do Caso C** (primeira foi
P156I/J spacing/gap). Reforça estabilidade do padrão (ADR-0064
§Justificação empírica passa para N=7 implícito; formalização
em ADR meta futura).

**Excepção considerada e rejeitada**: alternativa "manter
`Sides<Length>` com defaults zero em momento de construção"
(estado actual P156C). Rejeitada porque:
- Perde distinção semântica entre "lado não declarado" e "lado
  declarado a zero".
- Bloqueia futuras show rules que possam querer detectar quais
  sides foram explicitamente definidas.
- Diverge da semântica vanilla `Smart`-like (Auto vs valor explícito).

---

## 4. Variants Content existentes a estender

**Apenas `Pad`**. Não envolve criar variants novos — refactor
puro de variant existente.

Estado actual (P156C):
```rust
Pad { body: Box<Content>, padding: Sides<Length> }
```

Estado proposto (P156L):
```rust
Pad { body: Box<Content>, sides: Sides<Option<Length>> }
```

**Rename de field** `padding` → `sides` aproveita o refactor para
alinhar nomenclatura com vanilla (que usa "left/top/right/bottom"
sem prefix "padding").

---

## 5. Helpers stdlib reusáveis

### 5.1 `extract_length` — reuso N=7

`extract_length` em `stdlib/layout.rs` reusado pela **sétima vez**
consecutiva (P156C/D/G/H/I/J + agora P156L). Reforça subpadrão
emergente documentado em ADR-0064 §Implicações.

### 5.2 Helper novo `extract_sides_lengths`

Helper privado em `stdlib/layout.rs` para parse de named args
com fallback hierárquico (específico > eixo > rest). Não-genérico
neste passo (toma Length directamente; não `Sides<T>` genérico)
para evitar complexidade prematura.

```rust
fn extract_sides_lengths(args: &Args) -> SourceResult<Sides<Option<Length>>>
```

**Pré-decisão**: helper privado. Promoção a genérico/público
diferida até segundo reuso (per padrão N=2 mínimo para promoção).

### 5.3 Não-reusáveis

- `extract_dir`, `extract_parity`, `extract_weak`: irrelevantes.
- `build_spacing`: específico HSpace/VSpace.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P156L | Refino futuro |
|---------|--------------|---------------|
| `right` aplicado em layout | ✗ continua scope-out | Exige refactor multi-region |
| `Rel<Length>` (percentage) por side | ✗ scope-out | Caso A migration quando Layouter suportar Rel |
| Padding negativo | ✗ rejeitado (perfil graded) | Vanilla aceita; cristalino diverge intencionalmente |
| Side individualizada (None vs Some(zero)) | ✓ implementado | — |
| Atalhos x/y/rest com precedência vanilla | ✓ implementado (já em P156C) | — |
| body posicional (não Length) | ✓ mantido (P156C) | — |

### 6.1 Divergência da spec do passo (§"Verificação" #5)

Spec do P156L declara: "Cobertura Layout: **84%** (entrada `pad`
passa de `parcial` para `implementado puro`)".

**Inventário revela**: `pad` **já é `implementado`** (não `parcial`)
desde P156C — ver tabela A.5 de `typst-cobertura-vanilla-vs-cristalino.md`
linha 135. A entrada parcial `pad, corners, sides (inset modeling)`
no fim da tabela é **outra entrada documental** (refino PageConfig).

**Decisão arquitectural**:
- O refactor é executado integralmente (mérito próprio: precisão
  semântica + alinhamento com ADR-0064 Caso C).
- Cobertura **não muda** quantitativamente: pad continua a contar
  como implementado.
- Pad é **promovido para `implementado⁺`** (símbolo `⁺` indica
  refino além do mínimo; consistente com `figure` em ADR-0041).
  Distribuição Layout: 14/0/3/1/0=18 → 13/1/3/1/0=18.
- Cobertura `(impl + impl⁺) / total` mantém-se em **78%** (14/18 =
  13+1/18). **Não há ganho de cobertura percentual**.
- Critério "passou de parcial para puro" do spec é **factualmente
  inaplicável** — registado como divergência da spec sob ADR-0065
  critério #6 (divergência da spec via inventário).

Esta divergência segue o precedente de **P156F** (que divergiu
da spec ao detectar que `TransformMatrix` já era unificado);
inventário-primeiro per ADR-0065 cumpre exactamente este papel
de detectar suposições erradas da spec antes de execução.

---

## 7. Tests planeados

### 7.1 Regression tests P156C (críticos)

Tests pré-existentes a adaptar:
- **6 unit tests** em `entities/content.rs` (acessam `padding` field).
- **11 stdlib tests** em `stdlib/mod.rs` (acessam `padding` field).
- **2 layout E2E tests** em `layout/tests.rs` (asserções
  geométricas — não acessam variant directamente).

### 7.2 Tests novos

- Unit:
  - 4 sides independentes (cada um Some(value), outros None).
  - Atalhos x/y/rest.
  - Conflito x+left: precedência específico ganha (não erro per
    inventário §1 vanilla — "left.or(x).or(rest)").
- Stdlib:
  - Cada named arg isoladamente em variant resultante: lados
    declarados são `Some(...)`, outros `None`.
  - Combinações.
  - Defaults: pad sem named args → todos `None`.
  - Conflict scenarios (vanilla "último vence" via or chain) —
    confirmar paridade.

**Δ esperado**: +4 a +8 tests novos (mais regression que adições;
range mais estreito que P156J/I).

---

## 8. Sítios pattern-match a actualizar

Per grep `Content::Pad`:

### 8.1 Sítios não-test (estruturais)

1. `entities/content.rs:345` — declaração variant.
2. `entities/content.rs:551` — construtor `Self::pad(...)`.
3. `entities/content.rs` — `is_empty()` (`Pad { body, .. }` → não muda).
4. `entities/content.rs` — `plain_text()` (`Pad { body, .. }` → não muda).
5. `entities/content.rs` — `PartialEq` (acede `padding` → vai mudar).
6. `entities/content.rs:1053` — `map_content` (acede `padding`).
7. `entities/content.rs:1223` — `map_text` (acede `padding`).
8. `rules/introspect.rs:129` — `materialize_time` (acede `padding`).
9. `rules/introspect.rs:397` — `walk` (`Pad { body, .. }` → não muda).
10. `rules/layout/mod.rs:631` — `layout_content` (acede `padding`,
    consome `top/left/bottom`).
11. `rules/layout/mod.rs:1003` — `measure_content_constrained`
    (acede `padding`).
12. `rules/stdlib/layout.rs:288` — produção em `native_pad`.

**Total: 12 sítios estruturais** (contra esperados 9 da spec).
Diferença: spec subestima por não contar `is_empty`/`plain_text`
(que usam `Pad { body, .. }` e portanto não precisam de mudança
real, apenas verificação).

### 8.2 Sítios em tests

- `entities/content.rs:1874` — test acede `padding`.
- `rules/stdlib/mod.rs` (linhas 640, 661, 680, 698, 718, 758) —
  6 tests acedem `padding`.
- `rules/stdlib/mod.rs` (linhas 1382, 1718, 1747, 1898) — 4
  tests com `Pad { .. }` (não acedem padding — não precisam
  mudança).
- `rules/layout/tests.rs:2002+` — 2 tests E2E geométricos
  (verificação por posições, não por field padding directamente).

**Total: ~7 tests com acesso explícito a `padding`** que precisam
de adaptação directa; outros tests são compatíveis sem mudança.

---

## 9. Tests pré-existentes — válidos vs adaptação

### 9.1 Tests **directamente válidos** (sem mudança)

- 4 stdlib tests com `matches!(r, Value::Content(Content::Pad { .. }))`
  (assertion estrutural sem campos).
- 2 E2E layout tests (asserções geométricas via posições).
- 2 unit tests de `is_empty`/`plain_text` (usam `Pad { body, .. }`).

### 9.2 Tests **a adaptar** (acedem `padding` field)

- 1 unit test em `entities/content.rs` (linha 1874) — `if let
  Content::Pad { padding, .. }` precisa mudar para `sides`.
- 6 stdlib tests em `stdlib/mod.rs` que verificam `padding.left`/
  `padding.right`/etc. — precisam adaptar:
  - Para fields agora `Option<Length>`, asserções passam de
    `assert_eq!(padding.left, Length::pt(5.0))` para
    `assert_eq!(sides.left, Some(Length::pt(5.0)))` (com semântica
    explícita "lado declarado").

### 9.3 Renomeação de campo

`padding` → `sides`: renomeação coerente com vanilla naming.
Usar `replace_all` em sítios estruturais com cuidado (verificar
cada match).

---

## Resumo executivo

P156L é **refactor de variant existente** (primeira aplicação
de ADR-0065 critério #3). Mudança central:

```rust
// P156C → P156L
Pad { body, padding: Sides<Length> }
  →
Pad { body, sides: Sides<Option<Length>> }
```

Aplica **ADR-0064 Caso C** (segunda aplicação concreta —
estabilidade do padrão).

**Divergência da spec** detectada em §6.1: cobertura **não passa
para 84%** — pad já era implementado. Solução: marcar pad como
`implementado⁺` (refino) sem alterar cobertura quantitativa
(78% → 78%). Mérito do refactor é qualitativo (precisão semântica),
não quantitativo. Documentar honestamente.

**12 sítios pattern-match estruturais** + **7 tests** com acesso
directo a `padding` field. Reusos: `extract_length` N=7;
`Sides<T>` infraestrutura N=2 (segunda materialização concreta —
apenas Pad usa actualmente).

**Risco médio** (refactor, não aditivo). Mitigação por regression
tests críticos.
