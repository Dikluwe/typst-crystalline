# Diagnóstico `repeat` — Passo P156J (ADR-0061 Fase 3 sub-passo 1)

Inventário pré-materialização de `repeat` (vanilla → cristalino) per
ADR-0034 (padrão diagnóstico-primeiro; **nona aplicação consecutiva**
da série iniciada em P154A).

Contexto: P156I fechou Fase 2 (Stack — atinge target 72% Layout).
ADR-0061 §"Aplicações cumulativas" tem três caminhos diferidos; este
passo activa o caminho 1 (materializar Fase 3) com cadência granular
preservada (1 feature → +6%).

---

## 1. Assinatura vanilla `RepeatElem`

Fonte: `lab/typst-original/crates/typst-library/src/layout/repeat.rs`
(46 linhas; entre as menores de `layout/`).

```rust
#[elem(Tagged)]
pub struct RepeatElem {
    #[required]
    pub body: Content,

    #[default]
    pub gap: Length,

    #[default(true)]
    pub justify: bool,
}
```

**Tipos e defaults**:
- `body: Content` — **obrigatório** (sem default); construído por
  `repeat[content]` ou `repeat(content)`.
- `gap: Length` — default `Length::ZERO` (via `#[default]`).
- `justify: bool` — default **`true`** (via `#[default(true)]`).

**Atributos não-presentes** (vanilla repeat é minimalista; sem
`align`, `dir`, etc.). Total 3 fields. Confirmado por inspecção
directa do `repeat.rs` vanilla — não há fields herdados via traits
(é `#[elem(Tagged)]` mas Tagged não adiciona fields user-facing).

---

## 2. Comportamento observável (vanilla)

### 2.1 Caso de uso primário

TOC dot leaders: `#box(width: 1fr, repeat[.])` produz uma linha de
pontos que preenche o espaço disponível. Padrão ubíquo em outline,
índices, legendas direitas.

### 2.2 Mecânica de runtime (vanilla)

1. Vanilla `repeat` calcula `body_width` no momento de layout.
2. Calcula `available_width` (do contexto inline).
3. Determina `n = floor(available / (body_width + gap))`.
4. Emite `n` cópias de `body` com `gap` entre cada.
5. Se `justify == true`, distribui o espaço residual (`available - n
   * body_width - (n-1) * gap`) aumentando o gap real entre cópias.
6. **Erro vanilla**: se `available_width` é unbounded (ex: `repeat`
   fora de `box(width: 1fr, ...)`), emite "infinite content" error.

### 2.3 Limitação per ADR-0054 graded

O algoritmo de runtime layout em §2.2 vive em `typst-layout` no
vanilla. **Cristalino actual** não tem mecanismo de "available width
inline" exposto ao layouter por arm de `Content::*`. O `Layouter`
trabalha com `cursor_x`/`line_start_x`/`page_config.width` mas não
"reserva" largura para fr units mid-linha.

→ **P156J implementa apenas paridade estrutural**: variant +
stdlib + medição estática trivial + layout aproximado (emitir
o body uma vez no contexto actual). Algoritmo de repetição de
quantidade-para-preencher diferido para refino futuro (mesmo
critério aceite em P156G/H/I para containers complexos).

---

## 3. Decisões Smart<T> → Option<T>/default

Aplicação **N=6** consecutiva do padrão (P156D weak; P156E to;
P156G width/height; P156H width/height/baseline; P156I spacing).

| Vanilla field | Tipo cristalino | Default cristalino |
|---------------|-----------------|---------------------|
| `body: Content` | `Box<Content>` | obrigatório (erro se ausente) |
| `gap: Length` (default zero) | `Option<Length>` | `None` == zero |
| `justify: bool` (default true) | `bool` | **`true`** (paridade vanilla) |

Notas:
- `gap` segue padrão Smart→Option (consistente com P156I.spacing).
  `None` == zero é convenção uniforme da série.
- `justify` mantém-se `bool` (não há "auto" intermédio em vanilla;
  é genuinamente bool). Default **`true`** = paridade vanilla
  (divergência do default `bool::default() == false` é **intencional**
  e documentada no diagnóstico stdlib).

---

## 4. Variants Content existentes a estender

**Nenhuma**. `Repeat` é **variant novo**, sem encaixe em variants
existentes:
- Não é spacing (HSpace/VSpace já cobrem).
- Não é stack (Stack tem children variádicos heterogéneos; repeat
  tem body único repetido N vezes).
- Não é container estrutural simples (Block/Boxed/Pad).

→ Adicionar `Content::Repeat { body, gap, justify }` ao enum
(48 → 49? Não — vou contar: pós-P156I são 51 variants. Pós-P156J
serão **52**).

---

## 5. Helpers stdlib reusáveis

**Sexta aplicação consecutiva** de `extract_length` (P156C.pad/
P156D.h+v/P156E.pagebreak.to é parity, mas P156G/H/I.spacing reusam).

| Helper | Reuso? | Observação |
|--------|--------|------------|
| `extract_length` (em `stdlib/layout.rs`) | ✓ N=6 | para `gap` |
| `expect_no_named` (em `stdlib/mod.rs`) | ✗ | repeat tem named args |
| `extract_dir` / `extract_parity` | ✗ | irrelevante |

**Subpadrão dentro de "reuso de template containers" N=3**:
extract_length atinge sexta aplicação consecutiva em layout/* —
emergiu como **vocabulário canónico** para coerção Length em named
args de stdlib funcs. Promover a helper público em release futuro
(refactor scope-out).

---

## 6. Limitações aceites (perfil ADR-0054 graded)

P156J aceita as seguintes limitações, **alinhado** com tratamento de
P156G/H/I para containers complexos:

| Aspecto | Estado P156J | Refino futuro |
|---------|-------------|---------------|
| Algoritmo runtime "calcula N para encher" | ✗ scope-out | exige refactor inline-region |
| `justify: true` real (distribuir espaço) | ✗ scope-out | dep. de algoritmo runtime |
| Erro "infinite content" se unbounded | ✗ scope-out | dep. de detecção fr context |
| Single-render do body em contexto actual | ✓ implementado | aproximação per ADR-0054 |
| Variant + stdlib + medição estática | ✓ implementado | paridade estrutural |
| Walk + materialize_time descendentes | ✓ implementado | counters/labels resolvem em body |

**Justificação de divergência aceitável**: TOC dot leaders são uso
primário; em cristalino podem ser emulados explicitamente com
`Stack` ou múltiplas chamadas — perfil graded ADR-0054 já validou
este tipo de aproximação para Stack (BTT/RTL via reverse iter), Box
(width/baseline armazenados sem semântica completa) e Block
(width/breakable similares).

---

## 7. Tests planeados

**Unitários do variant** (em `01_core/src/entities/content.rs`,
módulo `tests`):
1. Constructor `Content::repeat(body, gap, justify)` produz
   `Content::Repeat` com fields esperados.
2. `is_empty()` proxy via body (consistente com Block/Boxed/Stack).
3. `plain_text()` recurse no body (sem multiplicar — paridade não
   visível em texto plano).
4. `PartialEq` cobre todos os fields.
5. `map_text` recurse no body, preserva gap/justify.

**Stdlib `native_repeat`** (em `01_core/src/rules/stdlib/mod.rs`,
secção tests):
6. Happy path: `#repeat[.]` → variant correcto, defaults
   (gap=None, justify=true).
7. Named args: `#repeat(gap: 5pt, justify: false)[a]` → variant
   com fields explicitos.
8. Erro hard: `#repeat()` (sem body) → SourceDiagnostic.
9. Erro hard: `#repeat(unknown: 1)[.]` → named arg desconhecido.
10. Body como string: `#repeat(".")` → `Content::text(".")` em body.
11. Erro hard: `#repeat(gap: "x")[.]` → gap não é length.

**E2E layout** (em `01_core/src/rules/layout/tests.rs`):
12. Repeat com body simples renderiza algo (não-empty page items).
13. Repeat dentro de Pad/Block descende correctamente
    (counters/labels dentro do body de repeat resolvem via walk).

**Smoke test paridade observável** (cumulativo em `stdlib/mod.rs`):
14. Cross-feature: `repeat` dentro de `stack` resolve sem panic
    (regression test — variant exhaustivo em todos os pattern-match).

Δ esperado: **+13 a +18** tests (consistente com média da série
P156C-I que adicionou 12-25 tests).

---

## Resumo executivo

`repeat` é trivial em superfície (3 fields vanilla; 1 feature) mas
o algoritmo runtime de "calcular quantidade para encher largura" é
o componente material — diferido para refino futuro per ADR-0054
graded (consistente com tratamento de containers complexos em
P156G/H/I).

P156J implementa **paridade estrutural**: variant `Content::Repeat
{ body, gap, justify }` + `native_repeat` em stdlib + medição
estática + layout single-render. Suficiente para cobertura formal
(72% → **78%**) e para que `Content::Repeat` esteja disponível
como nó AST em todo o pipeline (eval, introspect, layout, show
rules).

**Decisão arquitectural**: variant novo (sem encaixe em variants
existentes); padrão **Smart→Option N=6** (gap); `justify` bool com
default vanilla `true`. Helper `extract_length` reusado N=6 vezes
consecutivas no series (subpadrão emergente).
