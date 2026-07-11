# Paridade Produção — P704 — `range()` aceita `step:`

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-704.md`
**Hash do commit (implementação):** a preencher no commit seguinte.
**HEAD base:** `104e9f614` (fim de P703, detached HEAD).
**Estado:** ÂMBITO DE P704 FECHADO — `range(..., step:)` implementado, testado, paridade vanilla confirmada em todos os casos de borda medidos (incluindo uma correcção de divergência pré-existente: `range(-n)` deixa de errar). `cetz` mantém o progresso (mesmo tempo de compilação), mas bloqueia num gap novo e não relacionado (`luma()` só aceita `Int`, não `Ratio`).

---

## 1. Proveniência das medições

- Working tree no momento da medição: ficheiros de código+L0 de P704 (ver
  §4) + este relatório. `cargo test --workspace` corrido antes (herdado de
  P703: 3756 passed) e depois (3763 passed — +7 testes novos) da
  implementação.

---

## 2. Sonda — confirmado contra o vanilla, com `file:line`

- **Algoritmo exacto**: `foundations/array.rs:384-430` do vanilla
  (`Array::range`) — `step: NonZeroI64` (default 1), `step_dir =
  0.cmp(&step)`, condição de paragem `in_bounds` distinta para
  exclusive/inclusive.
- **Casos confirmados com documento `.typ` real**:
  - `range(90, 40, step: -12)` → `(90, 78, 66, 54, 42)`.
  - `range(0, 10, step: 2)` / `step: 3` → `(0,2,4,6,8)` / `(0,3,6,9)`.
  - `range(10, 0, step: -1)` → sequência descendente completa.
  - `range(0, 10, step: -1)` (direcção incompatível) → `()`, **não erro**.
  - `range(20, step: 4)` / `range(21, step: 4)` → forma de 1 argumento
    também respeita `step` (`start` implícito `0`).
  - `range(-6, step: -2, inclusive: true)` → `(0, -2, -4, -6)` — `step` e
    `inclusive` (já existente desde P504) compõem sem conflito.
  - `step: 0` → `Err "number must not be zero"` (mensagem exacta medida).
- **Achado extra, corrigido**: `range(-5)` no vanilla devolve `()`, **não
  erro** — medido directamente. O cristalino tinha `if n < 0 { Err(...) }`
  no braço de 1 argumento, uma divergência **não documentada** e não
  intencional (não constava de nenhum ADR/scope-out). Corrigida como parte
  da generalização do algoritmo (não é código extra — é a mesma
  implementação `step`-aware a produzir o resultado certo).
- **Estado cristalino antes do fix**: `error: range() argumento nomeado
  desconhecido: 'step'` (reprodução exacta do bloqueio isolado por P703).

---

## 3. Implementação

### 3.1 `native_rgb` — não tocado; `native_range` — generalizado

`01_core/src/rules/stdlib/foundations.rs::native_range`: novo argumento
nomeado `step` (`Int` não-zero, default `1`), validado antes do braço de
argumentos posicionais. Nova função interna `stepped_range(start, end,
step, inclusive)` — replica `step_dir`/`in_bounds` do vanilla verbatim.
Os dois braços posicionais (`range(n)` e `range(start, end)`) passam a
delegar a `stepped_range`, substituindo a geração antiga baseada em
`Range`/`RangeInclusive` do Rust.

### 3.2 Remoção do `if n < 0` — correcção, não regressão

O braço de 1 argumento deixou de rejeitar negativos explicitamente — o
algoritmo `stepped_range(0, n, 1, false)` já produz `()` correctamente
quando `n < 0` (a condição `in_bounds` falha na primeira iteração), tal
como o vanilla. Um teste antigo (`native_range_directo`,
`01_core/src/rules/stdlib/mod.rs`) esperava `Err` para `range(-1)` —
**actualizado** para esperar `Value::Array(vec![])`, com comentário a
registar que isto é a correcção de P704, não uma regressão silenciosa.

### 3.3 Testes (7 novos, `foundations.rs::tests_p704_range_step`)

Sem `step` (regressão), `range(-5)` vazio (correcção), `step` positivo e
negativo, `step` com 1 argumento, direcção incompatível, `step` combinado
com `inclusive`, `step: 0` (mensagem verbatim).

---

## 4. Ficheiros tocados

- **L0**: `00_nucleo/prompts/rules/stdlib/foundations.md` — secção
  `native_range` reescrita: algoritmo completo, `inclusive:` (já existente,
  nunca documentado — aproveitado para o registar agora), `step:` novo,
  correcção do `n<0`, testes canónicos.
- **Código**: `01_core/src/rules/stdlib/foundations.rs` (`native_range` +
  `stepped_range` + 7 testes), `01_core/src/rules/stdlib/mod.rs` (1 teste
  antigo corrigido para a paridade certa).

---

## 5. Validação — âmbito de P704 confirmado

Reexecutados os 6 documentos `.typ` da sonda + casos extra (release
build): todas as sequências geradas **idênticas ao vanilla**, incluindo
`step: 0` → mesma mensagem de erro, e `range(-5)` → array vazio (não mais
erro).

- `cargo test --workspace` → **3763 passed**, 0 failed (3756 de P703 + 7
  novos de P704, incluindo a correcção do teste antigo); `typst-infra`
  inalterado (626/5).
- `crystalline-lint .` → 0 violations (hash de `foundations.rs`
  realinhado com `--fix-hashes`).

---

## 6. Repetição da reprodução de P700-703 — mesmo progresso, novo bloqueio

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Tempo de compilação: **~7.2s** — igual à ordem de grandeza de P702/P703
(sem regressão de profundidade, mas também sem sinal claro de avanço maior
— o bloqueio seguinte está, mais uma vez, na mesma zona de `palette.typ`).

**Novo bloqueio**: `error: luma() requer 1 Int, recebeu 1 args`. Isolado à
mesma linha de `palette.typ` que motivou P704:
```
range(90, 40, step: -12).map(v => luma(v * 1%))
```
`v * 1%` produz um `Value::Ratio` (percentagem), não `Value::Int` —
`native_luma` (`01_core/src/rules/stdlib/foundations.rs`) só aceita `Int`
0–255. Confirmado standalone contra o vanilla:
```
#luma(50%)
#luma(128)
```
→ vanilla: exit 0 (ambas as formas). Cristalino: `error: luma() requer 1
Int, recebeu 1 args` para a forma `Ratio`.

Mesma disciplina: registar, não forçar correcção improvisada fora do
âmbito de P704.

### Próximo passo sugerido (P705, não iniciado)

1. Confirmar no vanilla a conversão `Ratio → componente 0-255` usada por
   `luma`/`rgb` (provavelmente o mesmo tipo `Component` visto em P702/P703,
   `visualize/color.rs`, que aceita `Ratio` e `Int` uniformemente).
2. Decidir se generaliza para `rgb()` também (que actualmente só aceita
   `Int` nos componentes numéricos, per scope-out explícito de P703) —
   medir se há consumidor real (`cetz`) que precise, antes de decidir.
3. Reexecutar a reprodução deste §6 como critério de fecho.

---

## 7. Estado da cadeia P678–704

Progresso real e cumulativo: plugin WASM real (P699-P700), `cbor.encode`
(P701), `.with()` (P702), `rgb(hex)` (P703), e agora `range(step:)` (P704)
— todos fechados e testados. `cetz` ainda não renderiza; os últimos 3
bloqueios (`rgb` hex, `range` step, `luma` ratio) estão todos na mesma
`palette.typ`, o que sugere que resolver o padrão `Component` (`Int |
Ratio`) de uma vez pode esgotar vários gaps ao mesmo tempo — decisão a
medir no próximo passo, não a assumir. Pausada aqui, com P705 sugerido e a
causa exacta já isolada.

## 8. Critério de fecho do passo

- [x] Sonda completa, semântica de `step:` confirmada contra o vanilla,
      incluindo casos de borda (passo zero, direcção incompatível).
- [x] Implementado e testado.
- [x] Formas existentes (`range(n)`, `range(start,end)`, `inclusive:`) sem
      regressão — e uma divergência pré-existente (`n<0`) corrigida.
- [x] `cetz` re-testado — **progresso mantido, próximo bloqueio
      identificado e isolado** (`luma()` com `Ratio`), não sucesso completo.
- [x] Sem regressão em `cargo test --workspace` (3763 passed).
- [x] `crystalline-lint .` limpo.
- [x] Relatório com resultado exacto (este ficheiro).
