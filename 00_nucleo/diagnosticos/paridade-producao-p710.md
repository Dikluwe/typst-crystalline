# Paridade Produção — P710 — `Length.to-absolute()`

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-710.md`
**Hash do commit (implementação):** `c6d1540f8`.
**HEAD base:** `0f97b5094` (fim de P709, detached HEAD).
**Estado:** ÂMBITO DE P710 FECHADO — `.to-absolute()` implementado e validado. `cetz` avança mais uma vez, novo bloqueio isolado: `Length / Length` (divisão) não implementado em `eval_binary_op`.

---

## 1. Sonda — semântica completa de `Length`, não assumida

`foundations/layout/length.rs:96-161` (vanilla): `#[scope] impl Length`
expõe `.pt()`, `.mm()`, `.cm()`, `.inches()`, `.to_absolute()` (exposto
como `to-absolute`); campos `.abs`/`.em` vêm do próprio `struct Length`
(`#[ty(scope, cast)]`), não do bloco `#[scope]`.

### Casos medidos

```
(6pt).to-absolute()                      → 6pt     (em=0, inalterado)
(6pt + 10em).to-absolute() [size: 12pt]  → 126pt   (6 + 10*12)
(6pt).pt()/.mm()/.cm()/.inches()          → 6 / 2.116.../0.211.../0.0833...
(6pt).abs / (40em+2pt).abs                → 6pt / 2pt
(3em+5pt).em / (20pt).em                  → 3 / 0
(6pt + 1em).pt()                          → Err "cannot convert a length
                                             with non-zero em units..."
(6pt).to-absolute() [sem `context`]       → Err "can only be used when
                                             context is known"
```

**Âmbito medido em `cetz`**: `grep` exaustivo confirma **só**
`.to-absolute()` (`canvas.typ:36`, `util.typ:131`). Scope-out explícito
de `.pt()`/`.mm()`/`.cm()`/`.inches()`/`.abs`/`.em` — sem consumidor
medido.

### Achado de arquitetura — sem gate de `context`

Medido directamente: blocos `context {...}` neste cristalino **já
executam eagerly** o corpo (confirmado com `#context { panic("x") }` →
erro propagado imediatamente), ao contrário do modelo do vanilla
(deferred até à posição de layout). `engine.styles` está sempre acessível
em qualquer ponto do eval — não há distinção "scripting simples" vs
"contexto resolvido" nesta arquitectura. `.to-absolute()` implementado
**sem** o gate `"can only be used when context is known"` do vanilla —
divergência de mecânica (ADR-0107), não de valor: o único caso medido
(`cetz`) sempre chama `.to-absolute()` dentro de `context {...}`
(`canvas.typ:26`, a função inteira é `context {...}`), logo o valor
produzido é idêntico nesse caso.

### Achado lateral, não corrigido — herança de estilos em `context`

Medido: `#set text(size: 12pt); #context [ #(10em).to-absolute() ]` deu
**116pt** (10×11, tamanho *default*), não **126pt**/120pt esperado
(10×12, tamanho *definido*) — a resolução de `ContextBlock` parece não
herdar o `StyleChain` activo no ponto da chamada, usando antes um
`StyleChain` "fresco"/default. **Não é causado por P710** — `.to-absolute()`
só lê `engine.styles.size()`, seja lá o que for; o problema é de onde vem
esse `engine.styles` quando uma `ContextBlockElem` é resolvida (mecanismo
não tocado por este passo). Nenhum dos meus testes/reprodução dependem
disto (nenhum documento de teste usa `#set text(size:)` antes de
`.to-absolute()`), por isso não bloqueia o fecho de P710, mas fica
registado para investigação futura caso um pacote real dependa de
`#set text(size:)` custom antes de `cetz.canvas(...)`.

---

## 2. Implementação

### Mecanismo (`01_core/src/rules/eval/closures.rs`)

Novo bloco em `eval_func_call`, mesmo padrão de
P417/P423/P504/P466/P506/P702/P707: se o callee é `FieldAccess` com campo
`"to-absolute"` e o alvo avalia para `Value::Length`, devolve
`Value::Length { abs: Abs(l.abs.to_pt() + l.em * engine.styles.size()), em: 0.0 }`
— reaproveita `StyleChain::size()` (`entities/style_chain.rs:433-443`),
já usado pelo layout de texto, sem mecanismo novo de resolução de estilo.

### Testes (3 novos, `eval/tests.rs`)

Sem `em` (inalterado), com `em` resolvido ao tamanho default (11pt), e
tipo diferente de `Length` não intercepta (cai no erro genérico normal).

---

## 3. Validação — âmbito de P710 confirmado

Reexecutado manualmente (release build): `(6pt).to-absolute()` → `6pt`;
`(6pt + 10em).to-absolute()` com `#set text(size: 12pt)` **fora** de
`context` → `126pt` — **idêntico ao vanilla**.

- `cargo test --workspace` → **3798 passed**, 0 failed (3795 de P709 + 3
  novos de P710); `typst-infra` inalterado (626/5).
- `crystalline-lint .` → 0 violations (hash realinhado, mesma L0
  partilhada `rules/eval.md`).

---

## 4. Repetição da reprodução de P700-709 — mais progresso, novo bloqueio

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Tempo de compilação: **~52.7s** — mesma ordem de grandeza de P709
(~52.8s), sem regressão.

**Novo bloqueio**: `error: cannot apply Div to length and length`.
Isolado a `canvas.typ:37-38` —
`assert(length / 1cm != 0, message: "Canvas length must be != 0!")`,
logo a seguir à chamada de `.to-absolute()` desbloqueada por este passo.
Confirmado standalone:
```
#(2cm / 1cm)
```
→ vanilla: `2`, exit 0. Cristalino: `error: cannot apply Div to length
and length` — `eval_binary_op` (`operators.rs`) não tem braço para
`(Value::Length, Value::Length)` em `BinOp::Div`.

### Próximo passo sugerido (P711, não iniciado)

1. Confirmar no vanilla a semântica completa de `Length / Length`
   (`Length::try_div`, `layout/length.rs:64-72` — só divide se `abs` de
   ambos for zero, ou `em` de ambos for zero; caso contrário `None` →
   que erro exacto?) e outras operações aritméticas entre `Length`
   (`+`/`-`/`*` já suportadas? confirmar, não assumir).
2. Implementar o(s) braço(s) confirmados em `eval_binary_op`.
3. Reexecutar a reprodução deste §4 como critério de fecho.

---

## 5. Estado da cadeia P678–710

`.to-absolute()` fechado e verificado. `cetz` mantém o mesmo tempo de
compilação (~52-53s) nos últimos 2 passos — sinal de que o bloqueio
seguinte está muito próximo na árvore de avaliação (mesma função
`canvas()`, linha seguinte). Pausada aqui, com P711 sugerido
(`Length / Length` e outras operações aritméticas de `Length`).

## 6. Critério de fecho do passo

- [x] Sonda completa, semântica de `Length` confirmada, incluindo
      tratamento de `em` em cada método/campo.
- [x] `.to-absolute()` implementado e testado, com o contexto de estilo
      necessário (`StyleChain::size()`, já existente).
- [x] Sem regressão em `cargo test --workspace` (3798 passed).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — **progresso mantido**, próximo bloqueio
      identificado (`Length / Length`), não sucesso completo.
- [x] Relatório com resultado exacto (este ficheiro), incluindo o
      achado lateral não corrigido (herança de estilos em `context`).
