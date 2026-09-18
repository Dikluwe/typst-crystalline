# Prompt L0 — `entities/elements/math_delimited` — `MathDelimitedElem`
Hash do Código: d4226a69

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_delimited.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathDelimitedElem {
    pub open:  char,
    pub body:  Content,   // era Box<Content>
    pub close: char,
}
```

`Content::MathDelimited { open, body, close }` →
`Content::MathDelimited(Arc<MathDelimitedElem>)`. Construtor ergonómico:
`Content::math_delimited(open: char, body: Content, close: char)`.

## `impl Element for MathDelimitedElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("{}{}{}", self.open, self.body.plain_text(), self.close)` (`content.rs:1629`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** só no `body`, `open`/`close` preservados (`content.rs:2111`): `Content::MathDelimited(Arc::new(MathDelimitedElem { open: self.open, body: self.body.map_content(f)?, close: self.close }))` |
| `map_text` | **terminal** (math structural; `content.rs:2624`): `Content::MathDelimited(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `open + body + close` (paridade `content.rs:1822`).

## Critério

`plain_text` `{open}{body}{close}`; `map_content` recurse só o body; `map_text`
terminal; igualdade estrutural.

## P1436 — carrier público `MathLrElem` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O vanilla pinado declara `LrElem { size: Rel<Length>, body: Content }`; o
carrier cristalino deste owner representa apenas o par mecânico já separado
`open/body/close`. Reutilizar `MathDelimitedElem` apagaria `lr` sem cercas,
body variádico exato, cascata e presença explícita de `size`.

### Decisão proposta

Este consumer passa a possuir também `MathLrElem`, semanticamente distinto de
`MathDelimitedElem`, com `body: Content`, `size: Rel<Length>` e
`size_explicit: bool`. Derivar `Debug`, `Clone`, `PartialEq` e `Hash` sobre os
três campos. `plain_text` delega ao body; `map_content` transforma somente o
body e reconstrói tamanho/presença byte a byte; `map_text` permanece terminal
como os demais carriers matemáticos estruturais. Cast, junção de posicionais,
cascata e stretch não pertencem a esta entidade.

`MathDelimitedElem` e seu constructor permanecem inalterados para usos
mecânicos existentes. Não inferir `MathLrElem` a partir de igualdade de
delimitadores.

### P1447 — projeção pública dos campos de `MathLrElem`

O carrier expõe `body` sempre e `size` somente quando `size_explicit=true`.
O valor de `size` é `Value::Relative(self.size)` e o body é o próprio conteúdo
armazenado. O default e a cascata geométrica não fabricam presença. Esta
projeção serve todas as construções `MathLr`, inclusive `math.abs`; não cria
campos próprios de `abs` nem altera `MathDelimitedElem`.

## P1437 — carrier público `MathMidElem` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O vanilla pinado declara `MidElem { body: Content }`: um único campo público,
obrigatório e posicional. A identidade persiste no `repr` e participa da
resolução do `lr` mais próximo; não equivale a `MathDelimitedElem` nem ao
body nu.

### Decisão proposta

Este consumer passa a possuir `MathMidElem { body: Content }`, com `Debug`,
`Clone`, `PartialEq` e `Hash`. `plain_text` delega ao body;
`map_content` transforma só o body; `map_text` permanece terminal. Cast,
classe `Relation` e stretch pertencem a avaliação e layout.
