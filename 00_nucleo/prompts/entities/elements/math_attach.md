# Prompt L0 — `entities/elements/math_attach` — `MathAttachElem`
Hash do Código: bc010300

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_attach.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Medição P1293 anterior à decisão

No estado pré-confirmação identificado pelo manifesto SHA-256
`5c615928f69084dccdbbfe02e440363ffd128e23b86c909d9e5dcc5cc8ecfe02`, o
consumer `01_core/src/entities/elements/math_attach.rs:17-25` armazena os seis
slots como `Option<Content>`; `:27-35` projeta `sup`/`sub`; `:38-77` percorre e
reconstrói esses Options. O recibo causal
`p1293-empty-markup-carrier-measurement-receipt.md`, SHA-256
`80a9543c9450f2350a42fa48df2e42cac63109bd074ebb37c2ec424cba473d5c`,
mede que argumento omitido, `none` explícito e conteúdo vazio presente chegam
distintos até `Args`, mas os dois últimos colidem pela primeira vez no
constructor estrutural e ficam ambos `Some(Content::Empty)`. Logo
`Option<Content>` possui só dois estados e não conserva a cardinalidade
morfologicamente observável de três.

A mudança abaixo é a categoria 1 de ADR-0127: enum/campos e assinaturas
públicas. Foi confirmada pelo humano em `2026-09-02T08:06:37-03:00`, texto
`Confirmado`, sobre o input preconfirmação acima. A decisão sucede a medição
(ADR-0108); presença/none/markup vazio são morfologia de linguagem, enquanto o
enum Rust é mecânica livre (ADR-0107).

## Struct canônica

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum MathAttachSlot {
    Omitted,
    ExplicitNone,
    Present(Content),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAttachElem {
    pub base: Content,
    pub t:  MathAttachSlot,
    pub b:  MathAttachSlot,
    pub tl: MathAttachSlot,
    pub bl: MathAttachSlot,
    pub tr: MathAttachSlot,
    pub br: MathAttachSlot,
}
```

`Content::MathAttach(Arc<MathAttachElem>)` é o único payload. O constructor
canônico é `Content::math_attach(base, t, b, tl, bl, tr, br)`. `sup()` devolve
`Option<&Content>` somente de `tr: Present`, senão somente de `t: Present`;
`sub()` faz o paralelo `br`/`b`. `Omitted` e `ExplicitNone` não projetam
conteúdo e não bloqueiam o fallback. Esta é a compatibilidade confirmada, sem
campos duplicados.

`Omitted` é argumento ausente; `ExplicitNone` é o valor Typst `none`
explicitamente fornecido; `Present(content)` é conteúdo presente, inclusive
`Present(Content::Empty)` para markup `[]`. O carrier é fechado e dedicado ao
elemento. É proibido codificar qualquer estado como sentinel de `Content`,
texto, span ou heurística, bem como criar `Settable` genérico ou alterar a
álgebra global de `Content`.

## `impl Element for MathAttachElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | concat `^tl _bl base ^t _b ^tr _br` somente para `Present`; `Omitted`/`ExplicitNone` não acrescentam texto |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em `base` e no conteúdo de cada `Present`; preserva `Omitted`, `ExplicitNone` e `Present(Content::Empty)` |
| `map_text` | **terminal** (math structural; `content.rs:2622`): `Content::MathAttach(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara base e os seis carriers. Essa igualdade Rust é mecânica;
aceitação de P1293 usa morfologia/semântica da linguagem (ADR-0107).

## Critério

`plain_text` e traversal cobrem os seis slots na ordem declarada; `map_content`
preserva os três estados; `map_text` é terminal; sintaxe e `math.attach`
compartilham este payload. `repr` deve omitir `Omitted`, imprimir `none` para
`ExplicitNone` e `[]` para `Present(Content::Empty)`; layout só materializa
`Present`, portanto markup vazio reserva `SpaceAfterScript`, enquanto omissão e
`none` permanecem layout-equivalentes. Refutam o contrato qualquer colapso dos
três estados, mudança de ordem, heurística ou efeito em rotas não-MathAttach.
