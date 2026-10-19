# Relatório — Passo 907: dois achados de espaçamento (fence `|`, `min`/`max` seguido de conteúdo)

**Data:** 2026-07-25
**Commit de partida:** `1a5538966` (P906)

---

## Resumo executivo

Dois achados registados em P825/P903 (`spacing.md`, scope-out) — gap ausente à volta do `|` como
fence/separador (`{x in RR | x > 0}`) e ausência de espaço depois de `min`/`max` (`min f(x)` →
`min𝑓(𝑥)` sem espaço) — foram diagnosticados por leitura directa da fonte vanilla e corrigidos com
duas adições pequenas e independentes em `spacing.rs`. Nenhuma das duas revelou complexidade
adicional (nenhuma precisou de desambiguação de contexto nem de passo dedicado). Ambas confirmadas
visualmente contra PDF real do vanilla, com posições de glifo a baterem dentro de 0.011pt.

## Parte A — `{x in RR | x > 0}`: gap ausente à volta de `|` como fence

### Fase A — diagnóstico

Fonte vanilla lida directamente: `math/ir/item.rs::is_spaced()` —

```rust
fn is_spaced(&self) -> bool {
    self.class() == MathClass::Fence || (self.props.spaced && matches!(self.class(), Normal | Alphabetic))
}
```

`process.rs::spacing()` tem o ramo `_ if l.is_spaced() || r.is_spaced() => return space` — um
fallback que dispara **sempre** que qualquer um dos dois lados é `Fence`, independentemente de
nenhuma regra explícita de `spacing_between_class` cobrir o par de classes envolvido.

Cristalino (`compute_gaps`, `01_core/src/engine/math/layout/spacing.rs`) já tinha esse fallback,
mas restrito a `Content::Text` — o caso corrigido em P903 (texto literal entre aspas). `|` como
fence é `Content::MathText('|')`, classificado `MathClass::Fence` por `default_math_class`, mas
como nenhuma regra explícita de `spacing_between_class` cobre `(Alphabetic, Fence)` ou
`(Fence, Alphabetic)`, o resultado caía no catch-all `None` → `0.0`, sem o fallback do vanilla
alcançá-lo (só verificava `Content::Text`, não a classe `Fence`).

`abs(x)` (`|x|` como delimitador) usa `Content::MathDelimited`, que já é tratado à parte
(`(Opening, Closing)`) — não passa por este caminho, confirmando que a correcção não lhe toca.

### Fase B — TDD

Vermelho confirmado antes da correcção: `fence_entre_identificadores_recebe_espaco_dos_dois_lados`
e `fence_dos_dois_lados_recebe_espaco_dos_dois_lados` falhavam (gap `0.0`).

Correcção em `compute_gaps` — generalização da condição de fallback:

```rust
// antes: prev_is_text || is_text
// depois:
let is_spaced = matches!(node, Content::Text(_)) || raw_l == MathClass::Fence;
```

(variável renomeada `prev_is_text`→`prev_is_spaced`, `is_text`→`is_spaced`).

4 testes novos, todos verdes após a correcção:
- `fence_entre_identificadores_recebe_espaco_dos_dois_lados`
- `fence_dos_dois_lados_recebe_espaco_dos_dois_lados`
- `fence_antes_de_punctuation_continua_sem_espaco` (regra explícita de Punctuation continua a
  vencer o fallback — ordem de prioridade preservada)
- `abs_delimitado_nao_afectado_por_fence` (regressão — `MathDelimited` inalterado)

## Parte B — `min f(x)`: sem espaço depois de `min`/`max`

### Fase A — diagnóstico

Fonte vanilla lida directamente: `math/ir/resolve.rs::resolve_op()` define
`item.set_class(MathClass::Large)` incondicionalmente para qualquer `OpElem` (`min`, `max`, `lim`,
`sin`, etc.) — a flag `limits` só controla `Limits::Display`/`Never` (se o limite aparece
sobre/sob o operador ou como subscrito lateral), **não** afecta a classe de espaçamento.

Cristalino (`base_math_class`) não tinha nenhum braço para `Content::MathOp` — caía no catch-all
`_ => MathClass::Normal`, e a regra `(Large, _) => THIN` nunca disparava.

Segundo mecanismo, para o caso com subscrito (`min_(x)`): `math/ir/item.rs::ScriptsItem::create()`
tem doc explícita "The resulting item inherits its math class from the base" —
`MathProperties::new(styles, base.raw_class(), ...)`. Cristalino (`Content::MathAttach`) também
caía no catch-all `Normal`, perdendo a classe `Large` herdada de `min`/`max` quando envolvido por
`MathAttach` (subscrito).

`AccentItem::create` tem a mesma doc/padrão de herança de classe do base, mas **não foi tocado** —
fora do achado confirmado por este passo (que cobriu especificamente `MathOp`/`MathAttach`, o caso
concreto de `min`/`max`), registado como scope-out no L0.

### Fase A, item 2 — isolar com/sem subscrito

Confirmado que o problema ocorre nos dois casos, testados separadamente:
- `min f(x)` sem subscrito → `Content::MathOp` directo, corrigido pelo braço `MathOp(_) => Large`.
- `min_(x) f(x)` com subscrito → `Content::MathAttach` envolvendo o `MathOp`, corrigido pelo braço
  `MathAttach(e) => base_math_class(&e.base)` (recursivo).

### Fase B — TDD

Vermelho confirmado antes da correcção: `math_op_e_large`, `math_op_sem_limits_tambem_e_large`,
`min_com_subscrito_recebe_thin_antes_do_conteudo_seguinte` falhavam (`base_math_class` devolvia
`Normal` em vez de `Large`).

Correcção em `base_math_class`:

```rust
Content::MathOp(_) => MathClass::Large,
Content::MathAttach(e) => base_math_class(&e.base),
```

5 testes novos, todos verdes após a correcção:
- `math_op_e_large`
- `math_op_sem_limits_tambem_e_large`
- `min_seguido_de_alphabetic_recebe_thin_sem_subscrito`
- `min_com_subscrito_recebe_thin_antes_do_conteudo_seguinte`
- `min_antes_de_parenteses_continua_sem_espaco` (regra explícita `(Large, Opening) => 0.0`
  continua a vencer — `min(x)` sem espaço antes do parêntese, comportamento correcto preservado)

## Suíte de testes

Workspace completo, discriminado por crate, ambas as partes já integradas (mesma sessão de TDD,
sem commit intermédio):

```
typst_core:  4770 passed; 0 failed; 3 ignored
typst_shell:  734 passed; 0 failed; 5 ignored
typst_infra:   41 passed; 0 failed; 0 ignored
+ crystalline_lint: 2 passed; 0 failed
+ (demais crates): 37 passed; 0 failed
```

Nenhuma regressão. `crystalline-lint .` — 0 drift (só o aviso V7 pré-existente e não relacionado,
`infra/package_version_resolution.md`, prompt órfão já registado antes deste passo).

## Confirmação visual (PDF real vs vanilla)

Recompilado `.typ/typst-math-comprehensive-test.typ` (hash
`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`, inalterado — ficheiro de teste
não tocado por este passo),
`exit=0`, sequência `(1)`...`(44)` completa (44 marcadores, todos presentes).

### Secção 7 — `{x in RR | x > 0}` (Parte A)

`pdftotext -bbox`, gaps medidos em pt entre glifos adjacentes:

| Par | Cristalino | Vanilla | Δ |
|---|---|---|---|
| `𝑥` → `∈` | 3.055 | 3.056 | 0.001 |
| `∈` → `ℝ` | 3.056 | 3.056 | 0.000 |
| `ℝ` → `\|` | **3.663** | **3.652** | 0.011 |
| `\|` → `𝑥` | **3.663** | **3.652** | 0.011 |
| `𝑥` → `>` | 3.056 | 3.056 | 0.000 |
| `>` → `0}` | 3.055 | 3.056 | 0.001 |

Gap à volta do fence (`ℝ|`/`|𝑥`) confirmado maior que os gaps normais (Alphabetic/Relation),
paridade com o vanilla dentro de 0.011pt (ruído de fonte/arredondamento, não um erro estrutural).

### Secções 29/30 — `max_(P_X) I(X;Y)`, `min_(x in RR^n) f(x)` (Parte B)

Gap `min`/`max` → conteúdo seguinte, cristalino agora **não-zero** (era `0.0` antes da correcção):

| Caso | Gap cristalino | Observação |
|---|---|---|
| `max` → `𝐼(𝑋;` (sem subscrito largo) | 1.834pt | Bate exactamente com o vanilla (1.833pt) |
| `min` → `𝑓(𝑥)` (com subscrito `x in RR^n`) | 1.833pt | THIN correcto aplicado; vanilla mede 3.146pt no mesmo par |

A diferença de 3.146pt (vanilla) vs 1.833pt (cristalino) no caso `min_(x in RR^n)` **não** é um
erro de classe de espaçamento — é um efeito de centragem horizontal do "limits" (a caixa do
subscrito, mais larga que `min`, é centrada no vanilla, empurrando o glifo `min` para a esquerda e
alargando o gap medido até `f(x)`; cristalino alinha a subscrito ao `xMin` da base, sem centrar).
Confirmado comparando `max_(P_X)` (subscrito mais estreito que `max` → gap bate exactamente,
1.834 vs 1.833) com `min_(x in RR^n)` (subscrito mais largo → só aí a discrepância aparece). Este é
o mesmo tipo de convenção de posicionamento de `layout_underover`/`MathAttach` com limites já
registado como fora de âmbito em P906 ("Gap de `layout_underover` maior que vanilla") — não
reaberto aqui, porque o item confirmado e pedido por este passo era especificamente a **ausência**
de espaço (classe), não a centragem geométrica do limite.

## Benchmark (Fase C)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Ambiente com deriva desde a medição de P906 (todos os
cenários acima da baseline registada em P906, incluindo `06-long` quase 2×) — confirmado por
medição A/B no mesmo commit (`1a5538966`, P906 HEAD) **sem** as alterações deste passo, produzindo
os mesmos tempos dentro do ruído. A deriva é ambiental, não causada por P907 (`spacing.rs` só é
exercitado por conteúdo matemático; `06-long.typ` não contém matemática).

| Cenário | P906 HEAD (mesma sessão) | Com P907 | Δ |
|---|---|---|---|
| 01-hello | 90.9ms | 90.3ms | -0.6ms |
| 02-lorem | 112.6ms | 113.7ms | +1.1ms |
| 03-images | 99.7ms | 100.1ms | +0.4ms |
| 04-math | 150.0ms | 149.7ms | -0.3ms |
| 05-tables | 111.3ms | 111.8ms | +0.5ms |
| 06-long | 365.3ms | 366.8ms | +1.5ms |
| 07-context | 131.8ms | 132.3ms | +0.5ms |

Todos os deltas dentro do ruído de `hyperfine` (σ ~1-3ms por cenário) — sem regressão atribuível a
P907.

## Resumo por item

| Item | Veredicto | Estado |
|---|---|---|
| Parte A — fence `\|` sem espaço | Diagnosticado (vanilla `is_spaced()`), corrigido, 4 testes | ✅ |
| Parte A — `abs(x)` (`MathDelimited`) não afectado | Confirmado por teste de regressão dedicado | ✅ |
| Parte B — `MathOp` sem classe `Large` | Diagnosticado (vanilla `resolve_op`), corrigido, 2 testes | ✅ |
| Parte B — `MathAttach` não herda classe do base | Diagnosticado (vanilla `ScriptsItem::create`), corrigido, 2 testes | ✅ |
| Parte B — `min(x)` antes de abertura continua sem espaço | Confirmado por teste de regressão dedicado | ✅ |
| `AccentItem`/`Cancel`/`Underover` não herdam classe do base | Confirmado no vanilla, **fora de âmbito** (não pedido por este passo) | ⚠️ Registado |
| Centragem geométrica de `min_(x in RR^n)` (gap 3.146 vanilla vs 1.833 cristalino) | Confirmado, mesma classe de achado de P906 (`layout_underover`), **fora de âmbito** | ⚠️ Registado |
