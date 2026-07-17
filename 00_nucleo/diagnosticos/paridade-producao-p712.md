# Paridade Produção — P712 — `measure()` real (intercepção + gate de `context`)

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-712.md`
**Hash do commit (implementação):** `72a3b6ae9`.
**HEAD base:** `726fcb618` (fim de P711).
**Estado:** FECHADO — `measure()` faz layout real e isolado (via
`layout_sub_frame`, Passo 629), com gate de `context` idêntico ao
vanilla. ADR-0108/ADR-0114 em vigor (mecanismo central produzindo
valores errados sem erro).

---

## 1. Sonda

### 1.1 Mecanismo vanilla confirmado (`lab/typst-original/.../layout/measure.rs:46-105`)

`#[func(contextual)]` — a macro do vanilla exige `Tracked<Context>`;
sem `context` disponível, `context.styles()`/`context.location()`
falham com `"can only be used when context is known"`. Dentro de
`context`, invoca o motor de layout **real**:

```rust
let pod = Region::new(
    Axes::new(width.resolve(styles).unwrap_or(Abs::inf()),
              height.resolve(styles).unwrap_or(Abs::inf())),
    Axes::splat(false),
);
let frame = (engine.library.routines.layout_frame)(engine, &content, locator, styles.chain(&style), pod)?;
Ok(dict! { "width" => frame.size().x, "height" => frame.size().y })
```

Confirma: (a) região efectivamente sem limites por defeito
(`Abs::inf()`), (b) usa o `styles` real do contexto (`context.styles()`),
(c) devolve `frame.size()` de um layout genuíno, não uma aproximação.

### 1.2 Estado do cristalino, `file:line`

`01_core/src/engine/stdlib/layout.rs:1346` (`native_measure`, antes da
correcção) delegava a `measure_content` (`layout/helpers.rs:141-167`):

```rust
match content {
    Content::Shape(e) => { ... }         // resolvido
    Content::Sequence(seq) => { ... }    // recursa, soma/max
    _ => (0.0, 0.0),                     // Content::Text cai aqui!
}
```

`Content::Text` **não tem braço** — cai no fallback `(0.0, 0.0)`. Não é
um placeholder esquecido nem uma tentativa real a falhar — é uma
aproximação manual, por tipo de `Content`, que nunca cobriu texto.
Confirmado com múltiplos casos (§1.3): `measure()` de texto devolve
sempre zero, dentro e fora de `context`; `Content::Shape`/`Sequence`
já funcionavam correctamente antes deste passo (preservados, sem
regressão — ver testes migrados em §4).

### 1.3 Outros casos de teste — confirma proporcionalidade esperada

```typst
#set text(size: 12pt)
#context {
  let s1 = measure[Texto curto]
  let s2 = measure[Um texto bastante mais comprido do que o anterior]
  [Curto: #s1.width, Longo: #s2.width]
}
```

Vanilla: `Curto: 56.22pt, Longo: 252.16pt` (razão ≈ 4.48×) — confirma
que textos diferentes produzem larguras diferentes e proporcionais,
não só "não-zero".

### Critério de fecho da sonda

- [x] Mecanismo do vanilla confirmado (`layout_frame` real, região
      `Abs::inf()`, `context.styles()`).
- [x] Localização exacta do `(0, 0)`: `layout/helpers.rs:141-167`,
      `measure_content` sem braço para `Content::Text`.
- [x] Confirmado: falta o cálculo real (texto nunca medido) **e** o
      gate de `context` (cristalino aceitava fora de `context`,
      devolvendo zero silenciosamente; vanilla erra).

---

## 2. Implementação

### Duas peças, porque `NativeFn` não tem `engine`

`measure` é uma stdlib fn registada via `Func::native(name, fn)` —
assinatura genérica `(ctx, args, world, file)`, sem `engine.styles`
nem gate de `context` útil. Mesmo problema já resolvido por P702/P707/
P710 (intercepção antes do dispatch genérico em `eval_func_call`).

**`01_core/src/engine/eval/closures.rs`** — novo bloco de intercepção,
reconhece `measure(...)` (`Expr::Ident`) **e** `std.measure(...)`/
`x.measure(...)` (`Expr::FieldAccess` — forma qualificada, o caminho
real do `cetz`, `util.typ:197`: `std.measure(cnt)`). Verifica primeiro
a forma sintáctica do nome (sem side-effects), só depois avalia o
callee e compara identidade de fn-ptr (`native_fn_addr`, mesmo padrão
de `bindings::eval_element_where`) contra `native_measure` — não o
nome, para não capturar um `measure` sombreado pelo utilizador. Se a
identidade bate:

1. Gate `ctx.in_context` (mesma convenção de `counter.get()`/
   `state.get()`, `stdlib/counter.rs:144`/`stdlib/state.rs:63`): fora
   de `context`, erro `"measure() can only be used inside context"`.
2. Dentro: `extract_measure_body` (validação de argumentos) +
   `measure_content_real` (`layout/mod.rs`) → `Value::Dict { width,
   height }`.

**`01_core/src/engine/layout/mod.rs`** — nova `measure_content_real`:
constrói um `Layouter` isolado (`FixedMetrics` + `NullImageSizer` — L1
não tem métricas de fonte reais, `FallbackFontMetrics` é L3), com
`chain`/`style` = os do chamador (o tamanho medido depende do `#set
text(size:)` activo), corre `layout_sub_frame` (Passo 629 — o mesmo
mecanismo já reutilizado por `Content::Place`/`Content::Transform`/
`grid.rs` para medir células) numa região `width: f64::INFINITY,
height: None` (paridade com `Region::new(.., Abs::inf())`). Largura:
`FixedMetrics.line_content_right(&items)` sobre os itens devolvidos
(generaliza correctamente a multi-linha, já que `line_start_x`
reinicia a 0 por linha). Altura: a devolvida por `layout_sub_frame`
(cursor real avançado, não aproximação).

**`01_core/src/engine/stdlib/layout.rs`** — `native_measure` (o
`NativeFn`) passa a ser só o fallback de invocação indirecta (`measure`
como valor de primeira classe, ex. `arr.map(measure)` — sem consumidor
medido). Sem `engine.styles`, **falha sempre** em vez de devolver
`(0, 0)` silenciosamente (ADR-0108). Validação de argumentos extraída
para `extract_measure_body` (partilhada entre o fallback e a
intercepção real).

### Divergência documentada (mecânica, não língua — ADR-0107)

`FixedMetrics` (monoespaçado, 0.6×size/codepoint) — L1 não tem acesso a
métricas de fonte reais. A largura devolvida é real e proporcional ao
conteúdo dado o motor de layout usado (mesmo `layout_sub_frame` do
documento principal), mas não byte-exacta ao vanilla (shaping real via
`rustybuzz`, só disponível em L3). Sem consumidor medido que dependa do
valor exacto — `cetz` usa `measure()` para decisões geométricas
relativas (`length / 1cm != 0`, `canvas.typ:37`), não comparação com
uma constante externa.

`measure_content` (o helper antigo, `layout/helpers.rs`) **não foi
tocado** — continua a servir os seus próprios consumers internos
(`Content::Transform`, `Content::Place`), que são um problema diferente
(estimativa rápida durante o layout já em curso, não a stdlib
`measure()` do utilizador).

---

## 3. Validação

### Reprodução manual (release build)

```
#let x = measure("oi")                    → Err "measure() can only be
                                             used inside context"
#context { measure[Texto curto].width,
           measure[Um texto bastante ...].width }
                                           → Curto: 79.2, Longo: 352.8
                                             (proporção ≈ 4.45×,
                                             vanilla ≈ 4.48× — real e
                                             proporcional, valor
                                             absoluto diverge por
                                             FixedMetrics vs shaping
                                             real, divergência
                                             documentada)
#context { std.measure("teste").width }   → 33.0 (forma qualificada,
                                             caminho real do cetz,
                                             funciona identicamente)
```

O formato de exibição (`Length { abs: Abs(79.2), em: 0.0 }` em vez de
`"79.2pt"`) é o bug de `repr_value` já identificado em P711 §2 —
confirmado ainda presente, inalterado por este passo (fora de escopo).

### Testes automatizados

- `extract_measure_body`: 6 testes migrados de `native_measure`
  (content/str aceites, ausente/tipo errado/extra posicional/named arg
  rejeitados) — mesma cobertura de validação, movida para a função que
  agora a possui.
- `native_measure` fallback: 1 teste novo — confirma que falha sempre
  (não mais `(0, 0)` silencioso).
- `measure_content_real`: 5 testes novos — texto simples não é mais
  zero (a regressão que este passo fecha), texto mais longo mede mais
  largo (proporcionalidade), tamanho de fonte maior mede mais largo
  (sensibilidade a `StyleChain`), shape rect continua a funcionar (sem
  regressão do caminho antigo), `Content::Empty` continua `(0, 0)`.
- Intercepção (`eval/tests.rs`): 3 testes novos — `measure()` fora de
  `context` erra com o gate; `std.measure()` fora de `context` também
  erra (forma qualificada); `measure` sombreado pelo utilizador não é
  interceptado (identidade de fn-ptr, não nome).
- **Nota de alcance do harness**: a intercepção só resolve a medição
  real (`ctx.in_context = true`) dentro de `expand_context_blocks`
  (L3, `03_infra/pipeline.rs`) — fora do alcance do harness L1 de
  `eval/tests.rs` (`layout()` local não resolve `Content::ContextBlock`,
  `engine/layout/mod.rs:1645`). A medição real "dentro de `context`" é
  validada por reprodução manual (acima), não por teste automatizado
  de topo — mesma limitação já documentada em P711 para o mesmo
  harness.

- `cargo test --workspace` → **3802** (typst-core: 3798 − 11 migrados/
  removidos + 15 novos) + **630** (typst-infra, inalterado) passed, 0
  failed.
- `crystalline-lint .` → 0 violations; `--fix-hashes .` realinhou 22
  ficheiros (headers só — `rules/eval.md`, `engine/layout.md`,
  `rules/stdlib/layout.md` mudaram de conteúdo, os 22 ficheiros
  partilham essas L0s).

### Reprodução `cetz`

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({ import cetz.draw: *; line((0,0),(2,1)); circle((0,0)) })
```

`error: cannot apply Div to length and length` — mesmo bloqueio de
P710/P711 (não tocado por este passo), tempo de compilação **~53.2s**,
mesma ordem de grandeza, sem regressão. `measure()` já não é a causa —
o `std.measure(cnt)` interno ao `cetz` (`util.typ:197`) agora resolve
correctamente antes do bloqueio seguinte.

---

## 4. Não corrigido aqui — achados fora de escopo (registados em P711 §2)

- `repr_value` (`repr.rs:59-66`) continua a formatar `Length` (entre
  outros) com `{:?}` do Rust — confirmado inalterado, mesmo bug de
  P711.
- `Length / Length` (Div) — bloqueio actual do `cetz`, sugestão de
  passo seguinte já registada em P710/P711.

## 5. Critério de fecho do passo

- [x] Sonda completa, mecanismo e causa exacta confirmados
      (`file:line`, ambos os problemas: cálculo ausente + gate
      ausente).
- [x] `measure()` corrigido: gate de `context` idêntico ao vanilla +
      cálculo real via `layout_sub_frame`.
- [x] Testado com textos de tamanhos diferentes — larguras diferem
      correctamente (proporcionalidade real, não aproximada).
- [x] Varredura ampla do corpus existente — 0 regressões (3802 vs
      3798 antes, delta explicado por migração/adição de testes).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — mesmo bloqueio de P710/P711, sem regressão;
      `measure()` deixou de ser a causa.
- [x] Relatório com resultado exacto (este ficheiro).
