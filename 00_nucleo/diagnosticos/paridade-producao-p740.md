# P740 — Cinco itens pequenos: warning de return, repr de `Args`, ordem no erro extra, `rgb(ratio)`, repr de NaN

**Data:** 2026-07-13 (22:41 -03:00)
**Commit base:** `887f385a1` ("P739: preenche hash do commit no relatório")
**Commit da implementação:** (preenchido no commit seguinte)
**Estado no momento das medições E2E:** working tree não commitado; 17
ficheiros alterados (`git diff HEAD --stat`: 370 inserções, 43 remoções —
11 ficheiros de código, 3 L0, mais os headers actualizados por
`--fix-hashes`).

Vanilla de referência: `lab/typst-original/target/release/typst`.

---

## Parte A — Warning "this return unconditionally discards the content before it" — FECHADA

### Sonda

`#{ [conteúdo]; return "x" }` (com `;` — a forma sem `;` é erro de
sintaxe em ambos). Vanilla emite **duas** mensagens: o erro "cannot
return outside of function" **e** a warning + hint "try omitting the
`return` to automatically join all values". Com state update no conteúdo
(`#{ let s = state("k", 0); [txt]; s.update(1); return "x" }`), junta o
segundo hint "state/counter updates are content that must end up in the
document to have an effect". Mecanismo vanilla: `warn_for_discarded_content`
(`typst-eval/src/code.rs:413-430`) — o único consumidor do flag
`conditional` de P729; o hint extra dispara por query
`State|Counter` no conteúdo.

Cristalino antes: só o erro, sem warning.

### Implementação

- Braço `Expr::CodeBlock` de `eval/mod.rs`: após o loop de join (P728),
  se `ctx.flow == FlowEvent::Return(span, Some(_), false)` e o output é
  `Value::Content` → warning via canal tracked. A emissão precede o
  tratamento do flow, logo coexiste com o erro de return fora de função
  (paridade exacta medida).
- `content_has_state_or_counter`: travessia directa das variants
  `State`/`StateUpdate`/`CounterUpdate`/`CounterDisplay`/
  `CounterDisplayCallback`, descendo em `Sequence` e `Styled` (paridade
  do seletor `State|Counter` do vanilla). Descoberta de implementação:
  `state.update()` produz `Content::StateUpdate`, não `State` — a
  primeira versão do teste apanhou a omissão.
- `Sink::warn_note2` (bloco tracked em `world_types.rs`): dois hints,
  mesma convenção de string vazia = ausente (o comemo não aceita
  `Option<&str>`, Passo 107).

### E2E

```
/tmp/p740a-warn.typ:1:16: warning: this return unconditionally discards the content before it
  hint: try omitting the `return` to automatically join all values
/tmp/p740a-warn.typ:1:16: error: cannot return outside of function
```

Com state: os dois hints presentes. Idêntico ao vanilla em mensagem,
hints e coexistência com o erro.

---

## Parte B — Repr de `Value::Args` — FECHADA

### Sonda

Vanilla: `#let f(a, ..rest) = rest; repr(f(1, z: 2, y: 3))` →
`arguments(z: 2, y: 3)`. Sonda estendida: `repr(f(1, 2, z: 3))` →
`arguments(z: 3, 1, 2)`; `repr(f(a: 1, 5, b: 2))` →
`arguments(a: 1, b: 2, 5)` — os **nomeados vêm primeiro** (ordem de
inserção), depois os posicionais. Vazio → `arguments()`. A ordem
nomeados-primeiro casa exactamente com a estrutura cristalina
(`named: IndexMap` + `items: Vec`) — basta iterar nessa ordem.

Cristalino antes: `arguments(...)` (lossy).

### Implementação

Braço `Value::Args` de `repr.rs`: itera `named` (com `k: repr`), depois
`items`, junta com ", " — paridade de `Args::repr` +
`pretty_array_like` (`foundations/args.rs:457-461`).

### E2E

`arguments(z: 2, y: 3)`, `arguments(z: 3, 1, 2)`, `arguments()` — os
três idênticos ao vanilla.

---

## Parte C — Ordem no erro de argumento extra — SCOPE-OUT REFORÇADO (custo medido)

### Sonda

`#let f(a) = a; f(1, z: 2, 3)` → vanilla: "unexpected argument: z" (o
primeiro extra na **ordem original** da lista única `Args.items`);
cristalino: "unexpected argument" (posicional primeiro). Confirmado o
registo de P733.

### Decisão

A paridade exacta exige migrar `Args` para lista única
(`EcoVec<Arg { name, value }>`). **Custo medido** (não estimado): 335
usos de `args.items`, 676 usos de `.named` em 26 ficheiros da stdlib,
31 construções directas de `Args {}`. Desproporcional para um caso de
canto cosmético — ambos os compiladores erram, só difere **qual**
argumento é nomeado na mensagem. Scope-out reforçado registado no L0
(§P740C) e nos achados, com o custo.

---

## Parte D — `rgb(ratio, ...)` — FECHADA

### Sonda

Vanilla: `rgb(50%, 0%, 0%)` → `rgb("#800000")` (ratio × 255,
arredondado: 127.5 → 128 = 0x80); `rgb(50%, 0%, 0%, 50%)` →
`rgb("#80000080")`. Mensagens verbatim medidas (cast `Component`,
`visualize/color.rs:2680-2692`): Int fora → "number must be between 0
and 255"; Float → "expected integer or ratio, found float"; Ratio fora
→ "ratio must be between 0% and 100%".

Cristalino antes: "rgb() requer 3 ou 4 Int" / "rgb(): componente r
fora de 0–255: 300" (mensagens próprias).

### Implementação

`native_rgb` reescrito espelhando a correcção P736 de `linear-rgb`:
`as_u8` aceita Int [0,255] ou Ratio [0%,100%] (`(rel × 255).round()`),
com as três mensagens verbatim. O scope-out P703 de componentes Ratio
fica fechado.

### E2E

`rgb("#800000")` — idêntico ao vanilla.

---

## Parte E — Repr de NaN (e ±inf) — FECHADA

### Sonda

Vanilla: `repr(calc.inf - calc.inf)` → `float.nan`; a sonda estendida
mediu também `repr(calc.inf)` → `float.inf` e `repr(-calc.inf)` →
`-float.inf`. Cristalino antes: `NaN.0` / `inf.0` / `-inf.0` — o mesmo
braço (`repr_float`) cobre os três casos, pelo que a correcção inclui
inf (mesmo bug, mesmo `if`).

### Implementação

`repr_float` (repr.rs) ganha a guarda de não-finitos antes da
formatação: NaN → `float.nan`, ±inf → `±float.inf`. Finitos inalterados
(`repr(1.0)` → `1.0`).

### E2E

`float.nan`, `float.inf`, `-float.inf` — idênticos ao vanilla.

---

## Validação global

- `cargo test --workspace`: **4771 passed, 0 failed** (4764 em P739 + 7
  testes novos: 3 de A, 1 de B, 2 de D, 1 de E).
- `crystalline-lint .`: 0 violations (`--fix-hashes` aplicado).
- `cargo build --release`: limpo.
- Não-regressão cetz: `/tmp/p734-cetz.typ` a 150 dpi vs referência
  vanilla — 1535/1478 px não-brancos, diff 0.1477% (B−A=+57),
  **idêntico** a P736–P739.

## Scope-outs declarados/reforçados neste passo

- Ordem entre tipos no erro de argumento extra (parte C) — custo medido:
  335 usos de `args.items`, 676 de `.named`, 26 ficheiros, 31
  construções de `Args {}`.
