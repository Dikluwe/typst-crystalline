# Prompt L0 — `entities/elements/math_cancel` — `MathCancelElem`
Hash do Código: e3993092

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_cancel.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). O contrato minimal P296 é ampliado por P1291 após autorização humana
para redigir o L0; a materialização continua proibida até o novo selo deste
prompt e dos prompts proprietários relacionados citados em §P1291.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathCancelElem {
    pub body: Content,   // era Box<Content>
}
```

`Content::MathCancel { body }` → `Content::MathCancel(Arc<MathCancelElem>)`.
Construtor ergonómico: `Content::math_cancel(body: Content)`.

## `impl Element for MathCancelElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1646`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** no `body` (`content.rs:2135`): `Content::MathCancel(Arc::new(MathCancelElem { body: self.body.map_content(f)? }))` |
| `map_text` | **terminal** (math structural; `content.rs:2632`): `Content::MathCancel(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body` (paridade `content.rs:1833`).

## Critério

`plain_text` transparente ao body; `map_content` recurse o body; `map_text`
terminal; igualdade estrutural.

## P1291.cancel-gate — payload público completo (RASCUNHO PARA SELO)

### Medição anterior à decisão

- O consumer cristalino em `01_core/src/entities/elements/math_cancel.rs:18-21`
  transporta somente `body`.
- O vanilla ratificado `a51e02804`, em
  `lab/typst-original/crates/typst-library/src/math/cancel.rs:18-115`, declara
  `body`, `length`, `inverted`, `cross`, `angle`, `stroke` e `background`;
  `angle` aceita ângulo ou função e o valor público também admite `auto`.
- Probes de 2026-08-31 registraram a ordem morfológica
  `body,length,inverted,cross,angle,stroke,background` e os domínios de erro no
  diagnóstico `00_nucleo/diagnosticos/p1291-matriz.md`. A forma de `repr` e a
  distinção entre default omitido e explícito continuam sob P1290; estes dados
  legitimam o transporte sem transferir ownership de `repr.rs`.

### Decisão pública proposta

Substituir o payload minimal por:

```rust
pub enum MathCancelAngle {
    Auto,
    Angle(Angle),
    Func(Func),
}

pub struct MathCancelElem {
    pub body: Content,
    pub length: Rel<Length>,
    pub inverted: bool,
    pub cross: bool,
    pub angle: MathCancelAngle,
    pub stroke: Option<Stroke>,
    pub background: bool,
    pub span: Span,
}
```

Defaults semânticos: `length = 100% + 0.3em`, `inverted = false`,
`cross = false`, `angle = Auto`, `stroke = None`, `background = false`.
`stroke = None` significa exclusivamente o traço MATH derivado no consumo:
espessura `0.05em` e paint do texto ativo; não significa ausência de linha.
Um `Stroke` explícito preserva todos os seus campos. `cross` prevalece sobre
`inverted`, mas ambos permanecem transportados porque são observáveis como
argumentos/set rules.

`span` preserva o local da chamada para erro tardio da callback, mas não é
campo da linguagem: igualdade e hash estruturais ignoram `span` e comparam os
sete campos públicos observáveis. O construtor compatível usa
`Span::detached()`; `native_math_cancel` usa `Args::span`.

Como `Rel<Length>`, `Angle` e `Stroke` contêm escalares de ponto flutuante, o
hash estrutural deve usar a convenção canónica já vigente para esses tipos; não
se autoriza converter grandezas em strings nem eliminar campos para obter
`derive(Hash)`.

O construtor compatível `Content::math_cancel(body)` permanece e inicializa os
defaults. Um construtor completo tipado, de ownership de
`entities/content.md`, transporta os sete valores. `plain_text` e os walkers
continuam transparentes/recursivos somente em `body`; igualdade e hash incluem
todos os campos.

### Incompletude deliberada nomeada

`MathCancelAngle::Func` é dado legítimo neste payload, mas sua execução depende
do ângulo default calculado depois do primeiro layout do corpo. O layouter atual
não possui `Engine` e não pode executar `Func`. O subpasso
`P1291.cancel-angle-runtime` usa o transcript puro definido em
`compiler/math/layout/callbacks.md` e a realização entre passagens de
`infra/pipeline.md`. É proibido executar a callback no módulo de entidade ou de
layout, ignorá-la ou tratá-la como `Auto` no documento final.

Prompts proprietários relacionados: `entities/content.md`,
`compiler/stdlib/structural/math.md`, `compiler/math/layout/cancel.md` e
`compiler/math/layout/_comum.md`.
