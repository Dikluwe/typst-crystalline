# Prompt L0 — `entities/elements/math_stretch` — `MathStretchElem`
Hash do Código: 668ad11f

**Camada:** L1
**Alvo:** `01_core/src/entities/elements/math_stretch.rs`

## P1446 — carrier público `math.stretch` (PROPOSTO; gate ADR-0127)

O consumer define `MathStretchElem { pub body: Content, pub size:
Rel<Length>, pub size_explicit: bool }`. Os três campos participam de clone,
igualdade e hash; floats de `Rel` usam a normalização de zero vigente nos
carriers matemáticos.

`plain_text` delega ao body. `map_content` transforma somente o body e
reconstrói `size` e `size_explicit` sem alteração. `map_text` é terminal.
O elemento não é locatável. A presença causal separa default omitido de
`size: 100%` explícito; não é um terceiro campo público da linguagem.

Critério: identidade, body, tamanho e presença sobrevivem a clone, igualdade,
hash e reconstruções; transformação do body não muda o alvo de stretch.
