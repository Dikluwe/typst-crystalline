# Diagnóstico — `$...$` aninhado dentro de função de layout perde processamento matemático (Fase A, Passo 993)

**Data**: 2026-08-11 · **Executor**: Kimi Code · **Padrão**: diagnóstico-primeiro (ADR-0034/ADR-0084/ADR-0065) — **sem fix neste passo**.
**Diagnóstico pai**: `00_nucleo/materialization/typst-passo-993.md`.
**Estado do código**: HEAD `e3513451e` (P992b) — binário `target/debug/typst`; vanilla de referência: `/usr/local/bin/typst` (upstream/main `a51e02804`, baseline ratificado 2026-08-11 — comportamento idêntico a 0.15.1 no corpus, ver `typst-retificacao-p990-p992-lab-sync.md`).
**Repros mínimos**: `00_nucleo/diagnosticos/p993-minimos/*.typ` (4 ficheiros, compilados nos dois motores — PDFs ao lado).

---

## 1. Tabela A — casos testados × sub-sintomas

Evidência literal de `pdftotext -bbox` (palavras extraídas) nos ficheiros de `p993-minimos/`.

| # | Fonte | Cristalino | Vanilla (referência) |
|---|-------|-----------|----------------------|
| A1 | `$ a + #text(size: 20pt)[$b$] + c $` | `𝑎+b+𝑐` — `b` **reto (ASCII), a 11pt** (tamanho 20pt **perdido**) | `𝑎+` … `𝑏` (20pt, **itálico**, y 26.1-46.1) … `+𝑐` |
| A2 | `$ #text(size: 8pt)[$x^2 + y^2$] = #text(size: 16pt)[$z^2$] $` | `x^2+y^2 = z^2` **literal** — `^` como carácter, sem sobrescrito, sem tamanhos | `𝑥`+`2`(sup, 8pt) … `𝑧`+`2`(sup, 16pt) — tudo real |
| A3 | `#let boxed(x)=box(stroke: 0.5pt, inset: 3pt)[$#x$]` `$ boxed(a)+boxed(b)=boxed(c) $` | `a+b=c` — **nenhuma caixa**, letras retas | três caixas com borda, `𝑎 𝑏 𝑐` itálicos dentro |
| A4a | `$ #align(center)[$a+b$] $` | **`center`** (palavra literal!) — o corpo `a+b` **desapareceu** | `𝑎+𝑏` |
| A4b | `$ #pad(left: 5pt)[$x^2$] $` | `x^2` literal | `𝑥`+`2`(sup) com deslocamento |
| A4c | `$ #block[$y^2$] $` | `y^2` literal | `𝑦`+`2`(sup) |

**Padrão confirmado em todas as funções testadas**: o conteúdo `$...$` aninhado no argumento sai como **texto literal** (glifo reto, `^`/`_` por interpretar) e os efeitos da função externa **perdem-se ou degradam-se** (tamanho perdido em `text()`, caixa inexistente em `box()`, nome do alinhamento a vazar em `align()`).

## 2. Localização da causa (leitura directa, `file:line`)

### 2.1 — Causa raiz dominante (única): o catch-all `plain_text()` do math layouter

`01_core/src/engine/math/layout/mod.rs:638` — o braço final de `layout_node`:

```rust
other => {
    let text: EcoString = other.plain_text().into();
    ... self.layout_text_node(&text, style)
}
```

Os braços existentes cobrem apenas variantes **matemáticas** (`MathText`, `MathIdent`, `MathSequence`, `MathFrac`, `MathRoot`, `MathAccent`, `MathUnderover`, `MathCancel`, `MathStyled`, `MathOp`, `MathLimitsOverride`, `MathAttach`, `MathDelimited`, `MathMatrix`, `MathCases`, `MathAlignPoint`, `Linebreak`, `HSpace` e — desde P990-C — `Strike`). **Qualquer outra variante de `Content` — `Styled` (resultado de `text(size:)`), `Box`, `Align`, `Pad`, `Block` — cai no catch-all** e é achatada para `plain_text()`:

- A1/A2: `Content::Styled(corpo_math, [Size(N)])` → `plain_text()` → texto reto; o `Size` e o processamento math do corpo morrem no achatamento.
- A3: `Content::Box(...)` → `plain_text()` → `a` reto; a moldura da caixa morre no achatamento.
- A4b/A4c: `Content::Pad`/`Content::Block` → idem.

**Resposta à pergunta 3.0 do passo** ("é a mesma limitação de P906/961/966?"): **NÃO — parcialmente relacionada, mecanismo distinto.** P906/961/966 eram sobre `apply_math_default` (fase de **layout**, direcção pai→filho em containers **matemáticos**) não atravessar `MathAccent`/`MathUnderover`/containers de markup de função de utilizador. Aqui o **eval produz a árvore de conteúdo correcta** (`Styled`/`Box` com conteúdo matemático dentro — `native_text` devolve `Content::Styled(body, [Size])`, `eval_math_expr` avalia o bloco aninhado) — a perda acontece **depois**, no **layout**, por ausência de caminho de layout para variantes não-matemáticas de `Content`. A família mais próxima é a **lição de P972** (catch-all silencioso em `match` sobre `Content`/`FrameItem` — "o braço `_` deixa itens sem o devido processamento"), não a família de P966.

### 2.2 — Causa secundária (só `align`, A4a): confusão de argumentos no eval

`eval_math_arg_value` (`01_core/src/engine/eval/math.rs:193-210`) avalia **todos** os argumentos posicionais como `Value::Content` — incluindo o ident de alinhamento `center` (que em modo math lex como `MathIdent`). `native_align` (`01_core/src/engine/stdlib/layout.rs:42-51`) toma como body **o primeiro `Value::Content` da lista** — que passa a ser o `center`, não o bloco `[$a+b$]`. Resultado: `Content::align(default, MathIdent("center"))` → catch-all → palavra "center" no PDF, corpo real descartado. (Em vanilla, `center` resolve como valor `Alignment` e o body é o bloco.)

### 2.3 — O mecanismo do vanilla (ADR-0123, `file:line` no lab actual)

Despacho de `resolve_into_self` (`lab/typst-original/crates/typst-library/src/math/ir/resolve.rs:155-231`):

- **`BoxElem`** → `BoxItem::create` (`resolve.rs:192-194`) → `layout_box` (`typst-layout/src/math/mod.rs:567-582`) — a caixa é layoutada pelo motor inline (`crate::inline::layout_box`), com stroke/inset, e empurrada como `FrameFragment` para o run math.
- **Tudo o resto** (`Styled`, `Align`, `Block`, `Pad`, …) → `ExternalItem::create` (`resolve.rs:228-230`) → `layout_external` (`mod.rs:585-603`) → `crate::layout_frame` — o conteúdo é layoutado pelo **motor normal** dentro do run math, com baseline ajustada a `height/2 + axis_height` quando ausente. O `$...$` aninhado dentro desse conteúdo re-entra no motor math pelo caminho normal (Equação inline é elemento de layout normal), preservando itálico, sobrescrito e tamanhos (a cadeia de estilos do `text()` aplica-se ao frame inteiro, incluindo a matemática aninhada).

Ou seja: no vanilla o "modo matemático" **não é uma propriedade do parser** — é uma propriedade da árvore de `Content` (cada `$...$` vira um `EquationElem` que o layout re-despacha), e os elementos de layout comuns têm caminho próprio dentro do run math (Box/External). O cristalino perde isso porque o seu math layouter só conhece variantes matemáticas de `Content`.

## 3. Tabela B — classificação por causa raiz

| Sub-achado | Classe (ADR-0084) | Causa | Mesma causa de P906/961/966? |
|---|---|---|---|
| Itálico perdido (A1, A2, A3, A4b, A4c) | **Bug real de linguagem** (documentos válidos) | catch-all `plain_text()` (§2.1) | **Não** — mecanismo distinto (perda estrutural em layout, não default não aplicado em eval) |
| `^`/`_` não interpretado (A2, A4b, A4c) | **Bug real** | catch-all (§2.1) | **Não** (idem) |
| Tamanho de `text(size:)` perdido (A1, A2) | **Bug real** | catch-all de `Content::Styled` (§2.1) | **Não** |
| Caixa de `box()` desaparece inteira (A3) | **Bug real** | catch-all de `Content::Box` (§2.1) — **não é causa separada** (a hipótese do passo de segunda causa para o box **não se confirmou**: com um caminho de layout "external", a caixa e o conteúdo aparecem juntos) | **Não** |
| Nome do alinhamento vaza + corpo perdido (A4a) | **Bug real** | catch-all (§2.1) **+** confusão de args no eval (§2.2) | **Não** (a §2.2 é mecanismo próprio, pequeno) |

**Agregação**: 1 causa raiz dominante (catch-all, cobre 5 dos 6 casos e todos os sub-sintomas de itálico/`^`/tamanho/caixa) + 1 causa secundária pontual (confusão de argumentos do `align`).

**Não fazer neste passo (respeitado)**: nenhum fix escrito; a forma da correcção (braços por variante vs. equivalente cristalino de `ExternalItem`/`BoxItem` — possivelmente delegando do `MathLayouter` para o `Layouter` normal e embutindo o frame resultante, ex.: via `FrameItem::Group`, `entities/layout_types.rs:430`) **só se decide depois** — se implicar variante nova no enum fechado `Content` (ADR-0026) ou mudança de assinatura dos layouters, é gate ADR-0127 com paragem.

## 4. Decisão (ADR-0084)

**B2 — passo de correcção com desenho prévio no L0**, dividido em duas frentes:

1. **Frente principal (equivalente cristalino de `ExternalItem`/`BoxItem`)**: investigar no passo de fix como o `MathLayouter` pode layoutar `Styled`/`Box`/`Align`/`Pad`/`Block` sem os achatar — comparar (α) braços por variante que despachen para os layouters existentes (texto/box/align) preservando o conteúdo, vs. (β) delegação genérica ao `Layouter` normal com embutimento do frame (o caminho do vanilla). Escolher com o L0 antes de código; gate ADR-0127 se tocar `Content` ou assinaturas públicas.
2. **Frente secundária (pontual, pode ir junto)**: `eval_math_arg_value`/`native_align` — o ident de alinhamento em modo math não pode virar `Value::Content` quando a função espera um valor não-content (o vanilla resolve `center` como `Alignment`). Provável fix pequeno no braço de chamadas math de funções de layout.

**Não recomendado**: B1 (fix directo sem desenho) — a frente principal é arquitectural (toca a fronteira entre os dois layouters), e B3 (só mais diagnóstico) é desnecessário — a causa está fechada com evidência literal e `file:line` dos dois lados.
