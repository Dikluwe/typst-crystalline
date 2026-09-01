# Prompt L0 — `entities/elements/flush` — `FlushElem`

**Estado:** CONTRATO P1292 AGUARDA SELO ADR-0127 — sem consumer e sem
`Hash do Código` até a materialização posterior ao gate humano.

**Camada:** L1
**Alvo planejado:** `01_core/src/entities/elements/flush.rs`
**Regras comuns:** `entities/elements/_comum.md`
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

No baseline cristalino, `place` é uma função sem namespace público e não há
sentinela de flush. O vanilla medido devolve `repr(place.flush()) == "flush()"`;
zero campos, posicionais ou named são rejeitados. O controle P1292 mostrou que
o marcador força os floats anteriores antes de o conteúdo seguinte prosseguir
e não antecipa o float criado depois dele.

## Contrato público

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FlushElem;
```

`Content::Flush(Arc<FlushElem>)` é uma identidade pública distinta de
`Content::Empty`; `Content::flush()` constrói a sentinela. O elemento é
não-locatável, `plain_text()` é vazio, `is_empty()` permanece `false` para
que normalização de sequência não apague o efeito, e `map_content`/`map_text`
são terminais. Igualdade/hash refletem a identidade unitária.

`place.flush` aceita zero argumentos. Um posicional produz
`unexpected argument`; um named produz `unexpected argument: <nome>`. A nativa
construtora pertence a `compiler/stdlib/layout.md`; o namespace da função
`place` pertence a `compiler/eval.md`; a morfologia pertence a
`compiler/eval/repr.md`; o efeito pertence somente a
`compiler/layout/flush.md`.

## Aceitação linguística

`repr(place.flush())` é exatamente `flush()`. O marcador não produz texto,
item visual ou identidade de introspecção, mas não pode ser colapsado ou
confundido com vazio porque altera o ponto de realização de floats no fluxo.
