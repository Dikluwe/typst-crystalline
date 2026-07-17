# Prompt L0 — `math/layout/attach` — `MathAttach`
Hash do Código: 12c21e97

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/attach.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado (MathLayouter, despacho): ver `math/layout/_comum.md`.

---

`MathAttach` — subscripts/superscripts/primes merged via eval. Consome
`MathGlyphKern` em todos os **4 quadrantes** (top-left, bottom-left, top-right,
bottom-right) via `self.metrics.math_kern(c)` (P255 §2 item 1; geometria
correcta sem `.abs()`, kern negativo permitido — `attach.rs:49-208`).

Recebe os `MathPrimes` (resolvidos em eval; ver `_comum.md`) pelo arm
superscript regular — não há arm dedicado `MathPrimes`.

**Critério**: `attach.rs` consome `math_kern` em todos os 4 quadrantes.

---

## Empilhamento de limites (`is_limits`) — P772w

`is_limits` decide se `sub`/`sup` empilham verticalmente acima/abaixo da
base (paridade vanilla `Limits::Display`/`Limits::Always`) em vez de ficarem
como scripts laterais à direita (`Limits::Never`). Só considerado quando
`self.block` (modo bloco/display) é `true` — em modo inline nunca empilha.

Condição (`attach.rs`, braço `MathIdent`/`MathText`):

```rust
(symbols::is_large_operator(ch) && !symbols::is_integral_char(ch))
    || symbols::is_limit_function(s.as_str())
```

**P772w — exclusão de integrais**: antes desta correcção, a condição era só
`is_large_operator(ch)` — `∫`/`∬`/`∮`/etc. estavam incluídos no conjunto de
"operadores grandes" (`symbols::is_large_operator`, usado também para
spacing/classe), fazendo `∫_0^1` empilhar `0`/`1` verticalmente em modo
bloco, igual a `∑_0^1`. Paridade vanilla (`Limits::for_char_with_class`,
`math/attach.rs` no vanilla): a classe `Large` só empilha (`Display`) se
**não** for um sinal de integral — integrais têm `Limits::Never`
incondicional, scripts sempre ao lado, mesmo em display style. Corrigido
adicionando `symbols::is_integral_char` (ver `math/symbols.md`) como
exclusão explícita, sem alterar `is_large_operator` (continua a incluir
integrais para outros fins — spacing/classe — só a decisão de limites é
afectada).

`Content::MathOp { limits: true, .. }` (P298, override explícito via
`op("...", limits: true)`) não é afectado — continua a empilhar
incondicionalmente quando `self.block`, independentemente do caractere.
